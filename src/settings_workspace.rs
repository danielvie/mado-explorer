use super::*;
use settings::{Geometry, NativePath, SavedLayout, SavedPane, Snapshot};
use std::{
    path::Path,
    time::{Duration, Instant},
};

// Supervisor enabled after independent settings-write and override-guard safety reviews.
pub const PRODUCTION_SETTINGS_WRITES_ENABLED: bool = true;

#[derive(Default)]
pub(super) struct Persistence {
    pub path: Option<PathBuf>,
    pub warning: Option<String>,
    pub disabled: Option<String>,
    pub geometry: Geometry,
    worker: Option<settings::Worker>,
    last: Option<Snapshot>,
    revision: u64,
    saved_revision: u64,
    error: Option<settings::SaveError>,
    due: Option<Instant>,
    close_prompt: bool,
    closing: bool,
    allow_close: bool,
}
impl Persistence {
    pub fn is_closing(&self) -> bool {
        self.closing || self.allow_close || self.close_prompt
    }
    pub fn startup() -> (Self, Option<Snapshot>) {
        let path = match std::env::var_os("MADO_SETTINGS_PATH") {
            Some(value) => settings::override_path(Path::new(&value)),
            None => settings::default_path(),
        };
        match path {
            Ok(path) => Self::at_path(path, PRODUCTION_SETTINGS_WRITES_ENABLED),
            Err(e) => (
                Self {
                    disabled: Some(
                        "Settings writes disabled: settings location unavailable.".into(),
                    ),
                    warning: Some(settings::bounded(&e.to_string())),
                    ..Default::default()
                },
                None,
            ),
        }
    }
    fn at_path(path: PathBuf, writes_reviewed: bool) -> (Self, Option<Snapshot>) {
        let loaded = settings::load(&path);
        let geometry = loaded
            .snapshot
            .as_ref()
            .map_or_else(Geometry::default, |s| s.geometry.clone());
        let disabled = if !loaded.writable {
            Some("Settings writes disabled: repair unreadable/invalid settings and restart.".into())
        } else if !writes_reviewed {
            Some("Settings writes disabled pending independent safety review. Workspace and bookmark/recent changes will not survive restart.".into())
        } else {
            None
        };
        (
            Self {
                path: Some(path),
                warning: loaded.warning,
                disabled,
                geometry,
                ..Default::default()
            },
            loaded.snapshot,
        )
    }
    pub fn start_worker(&mut self, repaint: Arc<dyn Fn() + Send + Sync>) {
        if self.disabled.is_none() {
            if let Some(path) = &self.path {
                match settings::Worker::new(path.clone(), repaint) {
                    Ok(worker) => self.worker = Some(worker),
                    Err(e) => {
                        self.disabled = Some(format!(
                            "Settings writes disabled: cannot start worker: {e}"
                        ))
                    }
                }
            }
        }
    }
}
impl Explorer {
    pub(super) fn snapshot(&self) -> Snapshot {
        fn tree(layout: &Layout, panes: &[Pane]) -> SavedLayout {
            match layout {
                Layout::Pane(id) => SavedLayout::Pane {
                    pane: panes.iter().position(|p| p.id == *id).unwrap(),
                },
                Layout::Split {
                    axis,
                    ratio,
                    first,
                    second,
                    ..
                } => SavedLayout::Split {
                    vertical: *axis == Axis::TopBottom,
                    ratio: *ratio,
                    first: Box::new(tree(first, panes)),
                    second: Box::new(tree(second, panes)),
                },
            }
        }
        Snapshot::new(
            self.persistence.geometry.clone(),
            self.panes
                .iter()
                .map(|p| SavedPane {
                    tabs: p
                        .tabs
                        .iter()
                        .map(|t| NativePath::capture(&t.navigation.current.location))
                        .collect(),
                    active: p.active,
                })
                .collect(),
            self.focused,
            tree(&self.layout, &self.panes),
            self.locations
                .bookmarks
                .iter()
                .map(|l| NativePath::capture(&l.path))
                .collect(),
            self.locations
                .recents
                .iter()
                .map(|l| NativePath::capture(&l.path))
                .collect(),
        )
    }
    pub(super) fn restore_snapshot(
        &mut self,
        snapshot: Snapshot,
        override_location: Option<PathBuf>,
    ) -> Result<(), String> {
        snapshot.validate()?;
        // Validation completes before replacing any workspace state. Locations are not probed here.
        let mut next_tab = 1;
        let panes: Vec<Pane> = snapshot
            .panes
            .iter()
            .enumerate()
            .map(|(i, pane)| {
                Ok(Pane {
                    id: PaneId(i as u64 + 1),
                    active: pane.active,
                    reveal_active_tab: true,
                    tabs: pane
                        .tabs
                        .iter()
                        .map(|path| {
                            let tab = Tab::new(TabId(next_tab), path.restore()?);
                            next_tab += 1;
                            Ok(tab)
                        })
                        .collect::<Result<_, String>>()?,
                })
            })
            .collect::<Result<_, String>>()?;
        fn tree(saved: SavedLayout, next: &mut u64) -> Layout {
            match saved {
                SavedLayout::Pane { pane } => Layout::Pane(PaneId(pane as u64 + 1)),
                SavedLayout::Split {
                    vertical,
                    ratio,
                    first,
                    second,
                } => {
                    let id = PaneId(*next);
                    *next += 1;
                    Layout::Split {
                        id,
                        axis: if vertical {
                            Axis::TopBottom
                        } else {
                            Axis::LeftRight
                        },
                        ratio,
                        first: Box::new(tree(*first, next)),
                        second: Box::new(tree(*second, next)),
                    }
                }
            }
        }
        let mut next_pane = panes.len() as u64 + 1;
        let layout = tree(snapshot.layout, &mut next_pane);
        let bookmarks = snapshot
            .bookmarks
            .iter()
            .map(|p| Ok(finder::Location::new(p.restore()?, "Bookmark")))
            .collect::<Result<_, String>>()?;
        let recents = snapshot
            .recents
            .iter()
            .map(|p| Ok(finder::Location::new(p.restore()?, "Recent")))
            .collect::<Result<_, String>>()?;
        for pane in &self.panes {
            for tab in &pane.tabs {
                self.loader.cancel(tab.id.0);
                self.sorter.cancel(tab.id.0);
            }
        }
        self.panes = panes;
        self.layout = layout;
        self.focused = snapshot.focused;
        self.next_tab_id = next_tab;
        self.next_pane_id = next_pane;
        self.persistence.geometry = snapshot.geometry;
        self.locations = finder::Catalogue {
            bookmarks,
            recents,
            revision: 0,
        };
        if let Some(location) = override_location {
            let p = &mut self.panes[self.focused];
            p.tabs[p.active] = Tab::new(p.tabs[p.active].id, location);
        }
        self.reveal_focused_pane = true;
        Ok(())
    }
    pub(super) fn persistence_logic(&mut self, ctx: &egui::Context, frame: &eframe::Frame) {
        if let Some(window) = frame.winit_window() {
            if !self.persistence.is_closing() && window.is_minimized() != Some(true) {
                self.persistence.geometry.maximized = window.is_maximized();
                if !window.is_maximized() {
                    if let Ok(position) = window.outer_position() {
                        self.persistence.geometry.position = Some([position.x, position.y]);
                    }
                    let size = window.inner_size().to_logical::<f32>(window.scale_factor());
                    let candidate = Geometry {
                        size: [size.width, size.height],
                        ..self.persistence.geometry.clone()
                    };
                    if candidate.validate().is_ok() {
                        self.persistence.geometry = candidate;
                    }
                }
            }
        }
        if let Some(report) = self.persistence.worker.as_ref().and_then(|w| w.take()) {
            match report.result {
                Ok(()) if report.revision == self.persistence.revision => {
                    self.persistence.saved_revision = report.revision;
                    self.persistence.error = None;
                }
                Err(e) => {
                    self.persistence.error = Some(e);
                    self.persistence.close_prompt = self.persistence.closing;
                    self.persistence.closing = false;
                    self.persistence.due = None;
                }
                Ok(()) => {}
            }
        }
        let requested = ctx.input(|i| i.viewport().close_requested());
        if self.persistence.allow_close {
            return;
        }
        if requested {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.finder = None;
            self.finder_service.cancel();
            workspace::surrender_focus(ctx);
            if self.persistence.worker.is_some() && self.persistence.error.is_none() {
                self.persistence.closing = true;
                self.queue_snapshot(true);
            } else {
                self.persistence.close_prompt = true;
            }
        }
        if self.persistence.closing
            && self.persistence.saved_revision == self.persistence.revision
            && self.persistence.revision != 0
        {
            self.persistence.allow_close = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if let Some(due) = self.persistence.due {
            if Instant::now() >= due {
                self.queue_snapshot(true);
            } else {
                ctx.request_repaint_after(due.saturating_duration_since(Instant::now()));
            }
        }
        if self.persistence.closing {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
    }
    fn queue_snapshot(&mut self, force: bool) {
        if self.persistence.worker.is_none() || (!force && self.persistence.error.is_some()) {
            return;
        }
        if self.panes.len() > settings::MAX_PANES
            || self.panes.iter().map(|p| p.tabs.len()).sum::<usize>() > settings::MAX_TABS
        {
            self.persistence.error = Some(settings::SaveError { detail: "Settings limit reached: at most 64 panes and 256 total tabs can be saved. Close excess contexts and retry.".into(), leftover: None });
            self.persistence.close_prompt |= self.persistence.closing;
            self.persistence.due = None;
            self.persistence.closing = false;
            return;
        }
        let snapshot = self.snapshot();
        if let Err(e) = snapshot.validate() {
            self.persistence.error = Some(settings::SaveError {
                detail: e,
                leftover: None,
            });
            self.persistence.close_prompt |= self.persistence.closing;
            self.persistence.due = None;
            self.persistence.closing = false;
            return;
        }
        let changed = self.persistence.last.as_ref() != Some(&snapshot);
        if !force {
            if changed && self.persistence.due.is_none() {
                self.persistence.due = Some(Instant::now() + Duration::from_millis(500));
            }
            return;
        }
        self.persistence.due = None;
        self.persistence.last = Some(snapshot.clone());
        let worker = self.persistence.worker.as_mut().unwrap();
        self.persistence.revision = if self.persistence.error.take().is_some() {
            worker.retry(snapshot)
        } else {
            worker.request(snapshot)
        };
    }
    pub(super) fn persistence_status(&mut self, ui: &mut egui::Ui) {
        if let Some(warning) = &self.persistence.warning {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 130), "Settings restore failed. Defaults loaded; original preserved. Repair settings and restart.").on_hover_text(warning);
        }
        if let Some(reason) = &self.persistence.disabled {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 130), reason)
                .on_hover_text(
                    self.persistence
                        .path
                        .as_ref()
                        .map_or_else(String::new, |p| native::display(p.as_os_str())),
                );
        }
        if let Some(error) = &self.persistence.error {
            ui.label("Settings save failed. Previous settings retained; hover for details.")
                .on_hover_text(&error.detail);
            if let Some(temp) = &error.leftover {
                ui.label("Settings temp cleanup failed. Hover for the full recovery path.")
                    .on_hover_text(native::display(temp.as_os_str()));
            }
            if ui.button("Retry settings save").clicked() {
                self.queue_snapshot(true);
            }
        }
        if self.persistence.closing {
            ui.label("Saving last workspace before closing. Waiting for settings worker...");
        }
        if self.persistence.close_prompt {
            ui.separator();
            ui.label(
                "Close without saving workspace changes? Settings saving is unavailable or failed.",
            );
            ui.horizontal(|ui| {
                if ui.button("Keep browsing").clicked() {
                    self.persistence.close_prompt = false;
                }
                if self.persistence.worker.is_some() && ui.button("Retry save and close").clicked()
                {
                    self.persistence.close_prompt = false;
                    self.persistence.closing = true;
                    self.queue_snapshot(true);
                }
                if ui.button("Close without saving").clicked() {
                    self.persistence.allow_close = true;
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        }
    }
    pub(super) fn persistence_ui(&mut self, ui: &mut egui::Ui) {
        self.persistence_status(ui);
        // Do not call workspace/finder custom keyboard or completion handlers during close.
        if !self.persistence.is_closing() {
            self.finder_workspace(ui);
            self.queue_snapshot(false);
        }
        if let Some(due) = self.persistence.due {
            ui.ctx()
                .request_repaint_after(due.saturating_duration_since(Instant::now()));
        }
    }
    pub(super) fn persistence_exit(&mut self) {
        if self.persistence.worker.is_some() {
            if !self.persistence.allow_close && self.persistence.error.is_none() {
                self.queue_snapshot(true);
            }
            let result = self.persistence.worker.as_mut().unwrap().finish();
            let error = result.err().or_else(|| self.persistence.error.clone());
            if let Some(error) =
                error.filter(|_| !self.persistence.allow_close || self.persistence.error.is_none())
            {
                exit_warning(&format!(
                    "Workspace settings could not be saved: {}\n{}",
                    error.detail,
                    error
                        .leftover
                        .as_ref()
                        .map_or_else(String::new, |p| format!(
                            "Unremoved temp: {}",
                            native::display(p.as_os_str())
                        ))
                ));
            }
        }
    }
}
#[cfg(windows)]
fn exit_warning(message: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONWARNING, MB_OK, MessageBoxW};
    let text: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = "Mado Explorer settings"
        .encode_utf16()
        .chain(Some(0))
        .collect();
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONWARNING,
        );
    }
}
#[cfg(not(windows))]
fn exit_warning(message: &str) {
    eprintln!("{message}");
}

#[cfg(test)]
#[path = "settings_tests.rs"]
mod tests;

pub(super) fn restore_geometry(cc: &eframe::CreationContext<'_>, geometry: &Geometry) {
    if let Some(window) = cc.winit_window() {
        if let (Some(position), Ok(mut native)) = (geometry.position, window.outer_position()) {
            let size = window.outer_size();
            let monitors: Vec<_> = window
                .available_monitors()
                .map(|m| {
                    let p = m.position();
                    let s = m.size();
                    [
                        p.x,
                        p.y,
                        p.x.saturating_add(s.width as i32),
                        p.y.saturating_add(s.height as i32),
                    ]
                })
                .collect();
            if let Some(p) = settings::recover_position(
                position,
                [size.width as i32, size.height as i32],
                &monitors,
            ) {
                native.x = p[0];
                native.y = p[1];
                window.set_outer_position(native);
            }
        }
        window.set_maximized(geometry.maximized);
    }
}
