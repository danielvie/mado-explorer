//! Read-only notification routing. Native I/O belongs to the accepted fixed driver.
use crate::{native, panes::PaneId, tabs::TabId};
use mado_native_directory_watch::{
    Failure, MAX_TARGETS, Refusal, Report, Service, Target, TargetReport,
};
use std::path::PathBuf;

/// SPEC checkpoint-2 native capability probes/authority are still pending.
/// No settings, environment, CLI or UI override. Supervisor alone may clear this gate.
const NATIVE_CAPABILITY_GATE_CLEARED: bool = false;
pub(super) const MANUAL_STATUS: &str = "Notifications disabled; refresh manually (F5)";

#[derive(Clone, Debug)]
pub(super) struct Context {
    pub pane: PaneId,
    pub tab: TabId,
    pub path: PathBuf,
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.pane == other.pane
            && self.tab == other.tab
            && self.path.as_os_str() == other.path.as_os_str()
    }
}
impl Eq for Context {}

pub(super) struct Binding {
    pub context: Context,
    pub target: Target,
    pub status: Option<TargetReport>,
    pub dirty: bool,
}

pub(super) struct WatchAdapter {
    service: Option<Service>,
    pub bindings: Vec<Binding>,
    pub revision: u64,
    next_generation: u64,
    admitted: bool,
    enabled: bool,
    capacity_refused: bool,
    pub refusal: Option<Refusal>,
    report_refusal: Option<Refusal>,
    pub unavailable: Option<String>,
    driver_errors: Vec<Failure>,
    omitted_errors: u64,
    stopping: bool,
    pub cleanup_acknowledged: bool,
    #[cfg(test)]
    owned_root: Option<PathBuf>,
}

impl WatchAdapter {
    pub fn production() -> Self {
        let mut adapter = Self {
            service: None,
            bindings: Vec::new(),
            revision: 0,
            next_generation: 1,
            admitted: false,
            enabled: NATIVE_CAPABILITY_GATE_CLEARED,
            capacity_refused: false,
            refusal: None,
            report_refusal: None,
            unavailable: None,
            driver_errors: Vec::new(),
            omitted_errors: 0,
            stopping: false,
            cleanup_acknowledged: false,
            #[cfg(test)]
            owned_root: None,
        };
        if adapter.enabled {
            adapter.start();
        }
        adapter
    }

    fn start(&mut self) {
        match Service::new() {
            Ok(service) => {
                self.service = Some(service);
                self.unavailable = None;
            }
            Err(error) => {
                self.unavailable = Some(format!("Start directory notifications: {error}"))
            }
        }
    }

    // Called with at most MAX_TARGETS+1 active contexts. Excess means refuse ALL,
    // never silently select an arbitrary subset. No filesystem reads occur here.
    pub fn sync(&mut self, contexts: Vec<Context>) {
        if !self.enabled || self.stopping {
            return;
        }
        #[cfg(test)]
        if let Some(root) = &self.owned_root {
            assert!(contexts.iter().all(|c| {
                c.path.is_absolute()
                    && c.path.starts_with(root)
                    && c.path.parent().is_some_and(|p| p.starts_with(root))
            }));
        }
        let capacity = contexts.len() > MAX_TARGETS;
        let changed = capacity != self.capacity_refused
            || (!capacity
                && (contexts.len() != self.bindings.len()
                    || contexts
                        .iter()
                        .zip(&self.bindings)
                        .any(|(c, b)| *c != b.context)));
        if changed {
            self.revision = self
                .revision
                .checked_add(1)
                .expect("Watch revision exhausted");
            self.admitted = false;
            let mut old = std::mem::take(&mut self.bindings);
            if !capacity {
                for context in contexts {
                    if let Some(index) = old.iter().position(|b| b.context == context) {
                        self.bindings.push(old.remove(index));
                    } else {
                        let generation = self.next_generation;
                        self.next_generation = generation
                            .checked_add(1)
                            .expect("Watch generation exhausted");
                        self.bindings.push(Binding {
                            target: Target::new(context.tab.0, generation, context.path.clone()),
                            context,
                            status: None,
                            dirty: false,
                        });
                    }
                }
            }
            self.capacity_refused = capacity;
        }
        if !self.admitted && self.revision != 0 {
            if let Some(service) = &self.service {
                match service.replace_targets(
                    self.revision,
                    self.bindings.iter().map(|b| b.target.clone()).collect(),
                ) {
                    Ok(()) => {
                        self.admitted = true;
                        self.refusal = None;
                    }
                    Err(refusal) => self.refusal = Some(refusal),
                }
            }
        }
    }

    pub fn drain(&mut self) {
        if self.stopping {
            // No UI-thread join, even after acknowledgement. Drop detaches the finished
            // or still-retiring driver; its Arc-owned native storage outlives the app.
            self.cleanup_acknowledged = self.service.as_ref().is_none_or(Service::stopped);
            return;
        }
        let Some(service) = &self.service else { return };
        match service.take_report() {
            Ok(report) => {
                self.report_refusal = None;
                self.accept(report);
            }
            Err(refusal) => self.report_refusal = Some(refusal),
        }
    }

    pub(super) fn accept(&mut self, report: Report) {
        if !self.enabled || self.service.is_none() || self.stopping {
            return;
        }
        // Service-wide failures also precede admission or survive target revisions.
        // The current Service supplies a sticky snapshot bounded to eight errors;
        // retain it across empty reports, without growing a separate error log.
        if !report.driver_errors.is_empty() {
            self.driver_errors = report.driver_errors;
        }
        self.omitted_errors = self.omitted_errors.max(report.omitted_driver_errors);
        if report.stopped {
            self.unavailable =
                Some("Directory notification driver stopped; use manual Refresh/F5.".into());
        }
        if !self.admitted || report.revision != self.revision {
            return;
        }
        for status in report.targets {
            if status.revision != self.revision {
                continue;
            }
            if let Some(binding) = self.bindings.iter_mut().find(|b| {
                b.target.id() == status.target.id()
                    && b.target.generation() == status.target.generation()
                    && b.target.path().as_os_str() == status.target.path().as_os_str()
            }) {
                binding.dirty |= status.reconcile || status.full_reconcile;
                binding.status = Some(status);
            }
        }
    }

    pub fn retry(&mut self, context: &Context) {
        if !self.enabled || self.stopping {
            return;
        }
        if self.service.is_none() {
            self.start();
            self.admitted = false;
            return;
        }
        let result = if !self.admitted {
            Err(Refusal::StaleTarget)
        } else if let Some(binding) = self.bindings.iter().find(|b| b.context == *context) {
            self.service.as_ref().unwrap().retry(
                self.revision,
                binding.target.id(),
                binding.target.generation(),
            )
        } else if self.capacity_refused {
            Err(Refusal::Capacity)
        } else {
            Err(Refusal::StaleTarget)
        };
        match result {
            Ok(()) => self.refusal = None,
            Err(refusal) => self.refusal = Some(refusal),
        }
        // Do not clear warnings on button press or listing refresh. The driver must
        // report the result of retry independently of the loader's result.
    }

    pub fn message(&self, context: &Context) -> String {
        if !self.enabled {
            return MANUAL_STATUS.into();
        }
        let path = native::display(context.path.as_os_str());
        let mut message = format!("Notifications for {path}: ");
        if self.capacity_refused {
            message.push_str("refused: more than 32 active locations; none watched. Close panes. ");
        } else if let Some(binding) = self.bindings.iter().find(|b| b.context == *context) {
            match &binding.status {
                None => message.push_str("registration pending. "),
                Some(status) => {
                    message.push_str(if status.ready {
                        "registered. "
                    } else {
                        "unavailable. "
                    });
                    if status.overflow {
                        message.push_str("Overflow: notifications lost. ");
                    }
                    for (action, error) in [
                        ("directory", &status.directory_error),
                        ("parent", &status.parent_error),
                    ] {
                        if let Some(error) = error {
                            let error_path = if action == "parent" {
                                context.path.parent().unwrap_or(&context.path)
                            } else {
                                &context.path
                            };
                            message.push_str(&format!(
                                "{action} {}: {error:?}. ",
                                native::display(error_path.as_os_str())
                            ));
                        }
                    }
                }
            }
            if binding.dirty {
                message.push_str("Reconciliation pending. ");
            }
        } else {
            message.push_str("current context not registered. ");
        }
        if let Some(refusal) = self.refusal {
            message.push_str(&format!("Control refused: {refusal:?}. "));
        }
        if let Some(refusal) = self.report_refusal {
            message.push_str(&format!("Report drain refused: {refusal:?}. "));
        }
        if let Some(error) = &self.unavailable {
            message.push_str(error);
        }
        if !self.driver_errors.is_empty() || self.omitted_errors != 0 {
            message.push_str(&format!(
                " Driver errors: {:?}; omitted: {}.",
                self.driver_errors, self.omitted_errors
            ));
        }
        message.push_str(" Listing may be out of date. Retry notifications or Refresh/F5; registration is not listing freshness.");
        message
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn needs_tick(&self) -> bool {
        self.service.is_some() && !self.cleanup_acknowledged
    }
    pub fn stop(&mut self) {
        if !self.stopping {
            self.stopping = true;
            self.bindings.clear();
            if let Some(service) = &self.service {
                service.request_stop();
            }
        }
    }

    #[cfg(test)]
    pub(super) fn enable_owned(root: &std::path::Path, contexts: &[Context]) -> Self {
        // Explicit internal fixture enablement only. Callers create-new this container
        // and keep watched target AND parent below it. Synthetic root paths stay gated.
        assert!(
            root.is_absolute()
                && root
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("mado-s018-")
        );
        assert!(contexts.iter().all(|c| {
            c.path.is_absolute()
                && c.path.starts_with(root)
                && c.path.parent().is_some_and(|p| p.starts_with(root))
        }));
        let mut adapter = Self::production();
        adapter.enabled = true;
        adapter.owned_root = Some(root.to_owned());
        if adapter.service.is_none() {
            adapter.start();
        }
        adapter.sync(contexts.to_vec());
        adapter
    }

    #[cfg(test)]
    pub(super) fn refuse_owned_admission(&mut self, refusal: Refusal) {
        // Controlled admission refusal, not a native startup-failure probe.
        assert!(self.owned_root.is_some() && self.service.is_some());
        self.admitted = false;
        self.refusal = Some(refusal);
    }

    #[cfg(test)]
    pub(super) fn join_owned_cleanup(&mut self) -> bool {
        self.service
            .as_mut()
            .is_none_or(|s| s.join_after_cleanup().unwrap())
    }
}
