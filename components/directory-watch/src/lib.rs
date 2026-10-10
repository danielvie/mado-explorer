//! Window-owned, nonrecursive Windows directory invalidations. No entry parsing or I/O
//! occurs in the UI methods. Reports are reconciliation requests, never directory listings.
#![cfg(windows)]

use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    os::windows::ffi::OsStrExt,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

mod native;
mod path;

pub const MAX_TARGETS: usize = 32;
/// Includes retiring requests. One additional short-lived metadata handle is allowed.
pub const MAX_REQUESTS: usize = 64;
const DEBOUNCE: Duration = Duration::from_millis(200);
const MAX_DELAY: Duration = Duration::from_millis(800);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Target {
    id: u64,
    generation: u64,
    path: PathBuf,
}
impl Target {
    pub fn new(id: u64, generation: u64, path: PathBuf) -> Self {
        Self {
            id,
            generation,
            path,
        }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathRejection {
    Nul,
    Length,
    NotAbsoluteFilesystem,
    UnsupportedSpelling,
    DeviceAlias,
    PipeShare,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Failure {
    Rejected(PathRejection),
    Native {
        action: &'static str,
        code: u32,
    },
    /// A terminal access-denied completion followed by an independently observed missing name.
    Disappeared {
        completion_code: u32,
        observation_code: u32,
    },
    Unsupported(&'static str),
    Capacity,
    DriverPanicked,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Refusal {
    Busy,
    Stopping,
    Capacity,
    DuplicateId,
    StaleRevision,
    StaleTarget,
    PathTooLong,
}
#[derive(Clone, Debug)]
pub struct TargetReport {
    pub revision: u64,
    pub target: Target,
    pub reconcile: bool,
    pub full_reconcile: bool,
    pub overflow: bool,
    pub ready: bool,
    /// Directory and parent failure are independent of dirty/reconciliation flags.
    pub directory_error: Option<Failure>,
    pub parent_error: Option<Failure>,
}
#[derive(Debug)]
pub struct Report {
    pub revision: u64,
    pub targets: Vec<TargetReport>,
    /// Sticky, deduplicated driver failures. At most eight distinct errors are retained.
    pub driver_errors: Vec<Failure>,
    pub omitted_driver_errors: u64,
    pub stopped: bool,
}
struct Mailbox {
    revision: u64,
    targets: Vec<Target>,
    changed: bool,
    retries: BTreeSet<u64>,
    reports: BTreeMap<u64, TargetReport>,
    driver_errors: Vec<Failure>,
    omitted_driver_errors: u64,
    port: Option<Arc<native::Port>>,
}
struct Shared {
    mailbox: Mutex<Mailbox>,
    stop: AtomicBool,
    stopped: AtomicBool,
    wake_error: AtomicU32,
    wake_pending: AtomicBool,
}
impl Shared {
    fn lock(&self) -> std::sync::MutexGuard<'_, Mailbox> {
        // A poisoned control lock cannot free native allocations. Preserve its bounded data.
        self.mailbox.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn failure(&self, error: Failure) {
        let mut m = self.lock();
        if m.driver_errors.contains(&error) {
            return;
        }
        if m.driver_errors.len() < 8 {
            m.driver_errors.push(error);
        } else {
            m.omitted_driver_errors = m.omitted_driver_errors.saturating_add(1);
        }
    }
    fn publish(&self, report: TargetReport) {
        let mut m = self.lock();
        if m.revision != report.revision || !m.targets.contains(&report.target) {
            return;
        }
        if let Some(old) = m.reports.get_mut(&report.target.id) {
            let dirty = old.reconcile | report.reconcile;
            let full = old.full_reconcile | report.full_reconcile;
            *old = report;
            old.reconcile = dirty;
            old.full_reconcile = full;
        } else {
            m.reports.insert(report.target.id, report);
        }
    }
    fn wake(&self, port: Option<Arc<native::Port>>) {
        if let Some(port) = port {
            if !self.wake_pending.swap(true, Ordering::AcqRel) {
                if let Err(code) = port.wake() {
                    self.wake_error.store(code, Ordering::Release);
                    self.wake_pending.store(false, Ordering::Release);
                }
            }
        }
    }
}

/// One fixed driver. Drop requests stop and detaches; it NEVER waits for provider cleanup.
/// Keep the service until stopped() and join_after_cleanup() acknowledge completion.
pub struct Service {
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
}
impl Service {
    pub fn new() -> io::Result<Self> {
        let shared = Arc::new(Shared {
            mailbox: Mutex::new(Mailbox {
                revision: 0,
                targets: Vec::new(),
                changed: false,
                retries: BTreeSet::new(),
                reports: BTreeMap::new(),
                driver_errors: Vec::new(),
                omitted_driver_errors: 0,
                port: None,
            }),
            stop: AtomicBool::new(false),
            stopped: AtomicBool::new(false),
            wake_error: AtomicU32::new(0),
            wake_pending: AtomicBool::new(false),
        });
        let driver_shared = shared.clone();
        let thread = thread::Builder::new()
            .name("mado-directory-iocp".into())
            .spawn(move || native::drive(driver_shared))?;
        Ok(Self {
            shared,
            thread: Some(thread),
        })
    }
    /// Coalesces the latest snapshot. All refusals are synchronous and visible; no partial admission.
    /// Revisions must increase. Even unchanged IDs are tied to the supplied immutable generation/path.
    pub fn replace_targets(&self, revision: u64, targets: Vec<Target>) -> Result<(), Refusal> {
        if targets.len() > MAX_TARGETS {
            return Err(Refusal::Capacity);
        }
        let mut ids = BTreeSet::new();
        for target in &targets {
            if target.path.as_os_str().encode_wide().take(32767).count() > 32766 {
                return Err(Refusal::PathTooLong);
            }
            if !ids.insert(target.id) {
                return Err(Refusal::DuplicateId);
            }
        }
        let port = {
            let mut m = self.shared.mailbox.try_lock().map_err(|_| Refusal::Busy)?;
            if self.shared.stop.load(Ordering::Acquire) || self.stopped() {
                return Err(Refusal::Stopping);
            }
            if revision <= m.revision {
                return Err(Refusal::StaleRevision);
            }
            m.revision = revision;
            // Never emit old-revision records, including identical paths/generations.
            m.reports.clear();
            m.targets = targets;
            m.changed = true;
            m.retries.clear();
            m.port.clone()
        };
        self.shared.wake(port);
        Ok(())
    }
    pub fn retry(&self, revision: u64, id: u64, generation: u64) -> Result<(), Refusal> {
        let port = {
            let mut m = self.shared.mailbox.try_lock().map_err(|_| Refusal::Busy)?;
            if self.shared.stop.load(Ordering::Acquire) || self.stopped() {
                return Err(Refusal::Stopping);
            }
            if m.revision != revision
                || !m
                    .targets
                    .iter()
                    .any(|t| t.id == id && t.generation == generation)
            {
                return Err(Refusal::StaleTarget);
            }
            m.retries.insert(id);
            m.port.clone()
        };
        self.shared.wake(port);
        Ok(())
    }
    pub fn take_report(&self) -> Result<Report, Refusal> {
        let mut m = self.shared.mailbox.try_lock().map_err(|_| Refusal::Busy)?;
        Ok(Report {
            revision: m.revision,
            targets: std::mem::take(&mut m.reports).into_values().collect(),
            driver_errors: m.driver_errors.clone(),
            omitted_driver_errors: m.omitted_driver_errors,
            stopped: self.stopped(),
        })
    }
    pub fn request_stop(&self) {
        self.shared.stop.store(true, Ordering::Release);
        let port = self
            .shared
            .mailbox
            .try_lock()
            .ok()
            .and_then(|m| m.port.clone());
        self.shared.wake(port);
    }
    pub fn stopped(&self) -> bool {
        self.shared.stopped.load(Ordering::Acquire)
    }
    /// No provider waits. False means cleanup or thread exit has not completed yet.
    pub fn join_after_cleanup(&mut self) -> io::Result<bool> {
        if !self.stopped() || self.thread.as_ref().is_some_and(|t| !t.is_finished()) {
            return Ok(false);
        }
        if let Some(t) = self.thread.take() {
            t.join()
                .map_err(|_| io::Error::other("directory driver panicked"))?;
        }
        Ok(true)
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        self.request_stop();
    }
}

#[derive(Default)]
struct Dirty {
    first: Option<Instant>,
    last: Option<Instant>,
    full: bool,
}
impl Dirty {
    fn mark(&mut self, now: Instant, full: bool) {
        self.first.get_or_insert(now);
        self.last = Some(now);
        self.full |= full;
    }
    fn due(&self, now: Instant) -> bool {
        self.first
            .is_some_and(|first| now.duration_since(first) >= MAX_DELAY)
            || self
                .last
                .is_some_and(|last| now.duration_since(last) >= DEBOUNCE)
    }
}
#[cfg(test)]
mod tests;
