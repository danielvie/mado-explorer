pub mod actions;
pub mod advanced;
pub mod directory;
pub mod persistence;
pub mod platform;
pub mod preferences;
pub mod preview;
pub mod rename;
pub mod selection;
pub mod session;
pub mod settings_ui;
pub mod statistics;
pub mod ui;
use eframe::egui;
use platform::windows::{NativeHost, OleGuard};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
pub const APP_NAME: &str = "Wayfarer Files";
type AppResult = Result<(), Box<dyn std::error::Error>>;
pub fn run() -> AppResult {
    run_window(None)
}
fn run_window(smoke_frames: Option<Arc<AtomicUsize>>) -> AppResult {
    let _ole = OleGuard::initialize()?;
    let native = Rc::new(RefCell::new(None::<NativeHost>));
    let owner = native.clone();
    eframe::run_native(
        APP_NAME,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1200.0, 820.0])
                .with_min_inner_size([640.0, 480.0])
                .with_drag_and_drop(false),
            ..Default::default()
        },
        Box::new(move |cc| {
            use raw_window_handle::HasWindowHandle;
            let raw_window_handle::RawWindowHandle::Win32(handle) = cc.window_handle()?.as_raw()
            else {
                return Err("Expected HWND".into());
            };
            let hwnd = windows::Win32::Foundation::HWND(handle.hwnd.get() as *mut _);
            let home = platform::shell::home_directory().map_err(std::io::Error::other)?;
            let start = std::env::args_os()
                .nth(1)
                .filter(|a| a != "--window-boot-probe")
                .map(PathBuf::from)
                .map(|p| {
                    if p.is_absolute() {
                        p
                    } else {
                        std::env::current_dir()
                            .unwrap_or_else(|_| home.clone())
                            .join(p)
                    }
                })
                .unwrap_or_else(|| home.clone());
            let host = NativeHost::attach(cc)?;
            host.probe()?;
            *owner.borrow_mut() = Some(host);
            let mut browser_ui = ui::BrowserUi::new(start, home.clone());
            let mut advanced = advanced::Advanced::default();
            let mut store = None;
            let mut geometry = None;
            if smoke_frames.is_none() {
                if let Some(local) = std::env::var_os("LOCALAPPDATA") {
                    match session::Store::open(PathBuf::from(local).join("WayfarerFiles")) {
                        Ok(opened) => match opened.load() {
                            Ok(Some(saved)) => {
                                geometry = saved.geometry;
                                for preset in &saved.presets {
                                    advanced.presets.insert(
                                        preset.name.clone(),
                                        session::Session {
                                            version: session::VERSION,
                                            tabs: preset.tabs.clone(),
                                            active: preset.active,
                                            layout: preset.layout.clone(),
                                            settings: saved.settings.clone(),
                                            presets: Vec::new(),
                                            geometry: None,
                                        },
                                    );
                                }
                                match persistence::restore(saved, home) {
                                    Ok(restored) => {
                                        browser_ui = restored;
                                        store = Some(opened);
                                    }
                                    Err(e) => {
                                        browser_ui.status =
                                            format!("State preserved; saving disabled: {e}")
                                    }
                                }
                            }
                            Ok(None) => store = Some(opened),
                            Err(e) => {
                                browser_ui.status = format!("State preserved; saving disabled: {e}")
                            }
                        },
                        Err(e) => browser_ui.status = format!("State unavailable/read-only: {e}"),
                    }
                } else {
                    browser_ui.status = "LocalAppData unavailable; persistence disabled".into();
                }
            }
            if let Some(saved) = &geometry {
                match platform::windows::work_area(hwnd, cc.egui_ctx.pixels_per_point())
                    .and_then(|(origin, size)| saved.clamped(origin, size))
                {
                    Ok(restored) => {
                        cc.egui_ctx
                            .send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                                restored.position.into(),
                            ));
                        cc.egui_ctx
                            .send_viewport_cmd(egui::ViewportCommand::InnerSize(
                                restored.size.into(),
                            ));
                        cc.egui_ctx
                            .send_viewport_cmd(egui::ViewportCommand::Maximized(
                                restored.maximized,
                            ));
                        geometry = Some(restored);
                    }
                    Err(error) => browser_ui.status = format!("Window geometry reset: {error}"),
                }
            }
            let drop_target =
                platform::drag_drop::DropRegistration::new(hwnd).map_err(std::io::Error::other)?;
            drop_target.set_read_only_probe(smoke_frames.is_some());
            Ok(Box::new(Window {
                smoke_frames,
                native: owner,
                browser_ui,
                controls: actions::Controls::default(),
                advanced,
                settings: settings_ui::SettingsUi::default(),
                hwnd,
                drop_target: Some(drop_target),
                store,
                search: None,
                search_query: String::new(),
                save_error: None,
                geometry,
            }))
        }),
    )?;
    let mut native = native.borrow_mut();
    let host = native.as_mut().ok_or("Native host was never created")?;
    if host.is_attached() {
        host.detach()?;
        return Err("Native callback was not detached before shutdown".into());
    }
    Ok(())
}
struct Window {
    smoke_frames: Option<Arc<AtomicUsize>>,
    native: Rc<RefCell<Option<NativeHost>>>,
    browser_ui: ui::BrowserUi,
    controls: actions::Controls,
    advanced: advanced::Advanced,
    settings: settings_ui::SettingsUi,
    hwnd: windows::Win32::Foundation::HWND,
    drop_target: Option<platform::drag_drop::DropRegistration>,
    store: Option<session::Store>,
    search: Option<platform::shell_search::MenuSession>,
    search_query: String,
    save_error: Option<String>,
    geometry: Option<session::WindowGeometry>,
}
impl Window {
    fn save_state(&self) -> Result<(), String> {
        if let Some(store) = &self.store {
            let mut saved = persistence::snapshot(&self.browser_ui);
            saved.geometry = self.geometry;
            saved.presets = self
                .advanced
                .presets
                .iter()
                .map(|(name, s)| session::NamedLayout {
                    name: name.clone(),
                    tabs: s.tabs.clone(),
                    active: s.active,
                    layout: s.layout.clone(),
                })
                .collect();
            store.save(&saved)?;
        }
        Ok(())
    }
    fn command(&mut self, command: &str) {
        use actions::Action;
        let action = match command {
            "copy" => Some(Action::Copy),
            "cut" => Some(Action::Cut),
            "paste" => Some(Action::Paste),
            "rename" => Some(Action::Rename),
            "trash" => Some(Action::Trash),
            "new_folder" => Some(Action::NewFolder),
            "refresh" => Some(Action::Refresh),
            _ => None,
        };
        if let Some(action) = action {
            if let Err(e) = self.controls.action(action, &mut self.browser_ui) {
                self.browser_ui.status = e;
            }
            return;
        }
        match command {
            "new_tab" => {
                if let Some(path) = self
                    .browser_ui
                    .active_tab()
                    .map(|t| t.browser.current_path().to_owned())
                {
                    self.browser_ui.add_tab(path);
                }
            }
            "close_tab" => self.browser_ui.close_active(),
            "up" => {
                if let Some(t) = self.browser_ui.active_tab()
                    && let Some(p) = t.browser.current_path().parent()
                {
                    t.navigate(p.to_owned());
                }
            }
            "select_all" => {
                if let Some(t) = self.browser_ui.active_tab() {
                    t.selection.select_all(&t.visible);
                }
            }
            "palette" => self.controls.open_palette(),
            _ => {}
        }
    }
}
impl eframe::App for Window {
    fn on_exit(&mut self) {
        self.search = None;
        self.drop_target = None;
        if let Err(e) = self.save_state() {
            eprintln!("State save failed; prior state preserved: {e}");
        }
        if let Some(host) = self.native.borrow_mut().as_mut()
            && let Err(e) = host.detach()
        {
            eprintln!("Native teardown failed: {e}");
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let viewport = ui.ctx().input(|i| i.viewport().clone());
        if viewport.minimized != Some(true) {
            if viewport.maximized == Some(true) {
                if let Some(saved) = &mut self.geometry {
                    saved.maximized = true;
                }
            } else if let (Some(outer), Some(inner)) = (viewport.outer_rect, viewport.inner_rect) {
                let candidate = session::WindowGeometry {
                    position: [outer.min.x, outer.min.y],
                    size: [inner.width(), inner.height()],
                    maximized: false,
                };
                if candidate.validate().is_ok() {
                    self.geometry = Some(candidate);
                }
            }
        }
        if let Some(outcome) = self
            .drop_target
            .as_ref()
            .and_then(|target| target.take_outcome())
        {
            self.browser_ui.status = match outcome.result {
                Ok(()) if outcome.delegated => format!(
                    "Drop returned from Windows: effect {}, destination {:?}; refreshing (per-item results unavailable)",
                    outcome.effect, outcome.destination
                ),
                Ok(()) => "Drop rejected or read-only probe; no transfer delegated".into(),
                Err(error) => format!("Drop failed: {error}"),
            };
            self.browser_ui.refresh();
        }
        self.advanced.poll(&mut self.browser_ui);
        self.controls.external_busy = self.advanced.is_busy();
        if self.advanced.is_busy() && ui.ctx().input(|i| i.viewport().close_requested()) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.browser_ui.status =
                "Recycle Bin action is active; wait for its result before closing".into();
        }
        let mut intents = Vec::new();
        let mut commands = Vec::new();
        let mut search_requested = false;
        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            commands = self.settings.show(ui, &mut self.browser_ui);
            self.controls.show(ui, &mut self.browser_ui);
            ui.collapsing("More tools", |ui| {
                self.advanced.show(
                    ui,
                    &mut self.browser_ui,
                    &mut self.controls,
                    self.hwnd,
                    self.store.as_ref(),
                );
                search_requested = ui.button("Search Shell menu").clicked();
            });
            intents = self.browser_ui.show(ui);
        });
        self.controls.external_busy = self.advanced.is_busy();
        for command in commands {
            self.command(&command);
        }
        if let Some(target) = &self.drop_target {
            target.set_read_only_probe(
                self.smoke_frames.is_some()
                    || self.controls.worker.is_busy()
                    || self.advanced.is_busy(),
            );
            let mut origin = windows::Win32::Foundation::POINT::default();
            unsafe {
                let _ = windows::Win32::Graphics::Gdi::ClientToScreen(self.hwnd, &mut origin);
            }
            let scale = ui.ctx().pixels_per_point();
            target.set_targets(
                self.browser_ui
                    .pane_targets
                    .iter()
                    .map(|(rect, path)| {
                        (
                            windows::Win32::Foundation::RECT {
                                left: origin.x + (rect.min.x * scale) as i32,
                                top: origin.y + (rect.min.y * scale) as i32,
                                right: origin.x + (rect.max.x * scale) as i32,
                                bottom: origin.y + (rect.max.y * scale) as i32,
                            },
                            path.clone(),
                        )
                    })
                    .collect(),
            );
        }
        if let Some(error) = ui::execute(intents, self.hwnd) {
            self.browser_ui.status = error;
        }
        if search_requested {
            let paths = self
                .browser_ui
                .active_tab()
                .map(|t| t.selection.paths().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            match platform::shell_search::MenuSession::new(self.hwnd, &paths) {
                Ok(menu) => self.search = Some(menu),
                Err(e) => {
                    self.browser_ui.status =
                        format!("Search unavailable; use native context menu: {e}")
                }
            }
        }
        let mut choice = None;
        let mut open = true;
        if let Some(menu) = &mut self.search {
            egui::Window::new("Search Shell menu")
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    ui.label("Type to load and search Shell commands, including submenus. Use the native menu for unsupported items.");
                    ui.text_edit_singleline(&mut self.search_query);
                    egui::ScrollArea::vertical()
                        .max_height(350.0)
                        .show(ui, |ui| {
                            let items = match menu.query(&self.search_query) { Ok(items) => items, Err(error) => { ui.colored_label(egui::Color32::LIGHT_RED, error); Vec::new() } };
                            for item in items {
                                if ui
                                    .add_enabled(item.enabled, egui::Button::new(&item.label))
                                    .clicked()
                                {
                                    choice = Some(item.id);
                                }
                            }
                        });
                });
        }
        if let Some(id) = choice {
            if let Some(mut menu) = self.search.take()
                && let Err(e) = menu.invoke(id)
            {
                self.browser_ui.status = e;
            }
            self.browser_ui.refresh();
        }
        if !open {
            self.search = None;
        }
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(80));
        if ui.ctx().input(|i| i.viewport().close_requested())
            && !self.controls.worker.is_busy()
            && !self.advanced.is_busy()
            && let Err(error) = self.save_state()
        {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.controls.keep_open();
            self.save_error = Some(error);
        }
        if let Some(error) = self.save_error.clone() {
            egui::Window::new("Session could not be saved")
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.label(error);
                    ui.label("The previous state has been preserved.");
                    if ui.button("Retry saving").clicked() {
                        match self.save_state() {
                            Ok(()) => self.save_error = None,
                            Err(e) => self.save_error = Some(e),
                        }
                    }
                    if ui.button("Close without saving").clicked() {
                        self.store = None;
                        self.save_error = None;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Keep open").clicked() {
                        self.save_error = None;
                    }
                });
        }
        if let Some(frames) = &self.smoke_frames {
            if frames.fetch_add(1, Ordering::Relaxed) >= 2 {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ui.ctx().request_repaint();
        }
    }
}
pub fn window_boot_probe() -> AppResult {
    let frames = Arc::new(AtomicUsize::new(0));
    run_window(Some(frames.clone()))?;
    if frames.load(Ordering::Relaxed) < 3 {
        return Err("Window closed before three frames".into());
    }
    println!(
        "window_boot: {} frames; HWND/OLE lifecycle passed",
        frames.load(Ordering::Relaxed)
    );
    Ok(())
}

pub mod bin_view;
