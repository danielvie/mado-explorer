use crate::{
    platform::{
        clipboard::{self, ClipboardFiles},
        operations::{ItemStatus, Operation, OperationRequest, OperationResult, OperationWorker},
    },
    preview::{Preview, PreviewResult},
    ui::BrowserUi,
};
use eframe::egui;
use std::{ffi::OsString, path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Copy,
    Cut,
    Paste,
    CopyPath,
    CopyTo,
    MoveTo,
    Trash,
    NewFile,
    NewFolder,
    Rename,
    Refresh,
    Inspector,
}
impl Action {
    pub fn eligible(self, filesystem: bool, selected: usize) -> bool {
        if self == Self::Refresh {
            return true;
        }
        if !filesystem {
            return false;
        }
        match self {
            Self::Copy | Self::Cut | Self::CopyPath | Self::CopyTo | Self::MoveTo | Self::Trash => {
                selected > 0
            }
            Self::Rename => selected == 1,
            _ => true,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Copy => "Copy",
            Self::Cut => "Cut",
            Self::Paste => "Paste",
            Self::CopyPath => "Copy paths",
            Self::CopyTo => "Copy to…",
            Self::MoveTo => "Move to…",
            Self::Trash => "Trash",
            Self::NewFile => "New file",
            Self::NewFolder => "New folder",
            Self::Rename => "Rename",
            Self::Refresh => "Refresh",
            Self::Inspector => "Inspector",
        }
    }
}
const ACTIONS: [Action; 12] = [
    Action::Copy,
    Action::Cut,
    Action::Paste,
    Action::CopyPath,
    Action::CopyTo,
    Action::MoveTo,
    Action::Trash,
    Action::NewFile,
    Action::NewFolder,
    Action::Rename,
    Action::Refresh,
    Action::Inspector,
];
struct Dialog {
    action: Action,
    paths: Vec<PathBuf>,
    directory: PathBuf,
    value: String,
}
pub struct Controls {
    pub worker: OperationWorker,
    next: u64,
    captured: Option<ClipboardFiles>,
    pub ledger: Option<OperationResult>,
    dialog: Option<Dialog>,
    palette: bool,
    query: String,
    preview: Preview,
    preview_path: Option<PathBuf>,
    texture: Option<egui::TextureHandle>,
    close_requested: bool,
    pub external_busy: bool,
    shutting_down: bool,
}
impl Default for Controls {
    fn default() -> Self {
        Self {
            worker: OperationWorker::new(),
            next: 1,
            captured: None,
            ledger: None,
            dialog: None,
            palette: false,
            query: String::new(),
            preview: Preview::new(),
            preview_path: None,
            texture: None,
            close_requested: false,
            external_busy: false,
            shutting_down: false,
        }
    }
}
impl Controls {
    pub fn open_palette(&mut self) {
        self.palette = true;
    }
    pub fn keep_open(&mut self) {
        self.shutting_down = false;
        self.close_requested = false;
    }
    pub fn submit(&mut self, items: Vec<Operation>) -> Result<(), String> {
        if self.external_busy || self.shutting_down {
            return Err("Another operation or shutdown is active".into());
        }
        if self.worker.is_finished() {
            self.worker = OperationWorker::new();
        }
        self.worker.submit(OperationRequest {
            id: self.next,
            items,
        })?;
        self.next += 1;
        Ok(())
    }
    pub fn action(&mut self, action: Action, browser: &mut BrowserUi) -> Result<(), String> {
        let (filesystem, selected) = browser
            .active_tab()
            .map(|t| (true, t.selection.paths().count()))
            .unwrap_or((false, 0));
        if !action.eligible(filesystem, selected) {
            return Err("Command is unavailable for this location or selection".into());
        }
        if self.external_busy || self.shutting_down || self.worker.is_busy() {
            return Err("Wait for the active operation to finish".into());
        }
        if action == Action::Refresh {
            browser.refresh();
            return Ok(());
        }
        if action == Action::Inspector {
            let tab = browser.active_tab().ok_or("No active tab")?;
            tab.inspector_visible = !tab.inspector_visible;
            if !tab.inspector_visible {
                self.preview.clear();
                self.texture = None;
                self.preview_path = None;
            }
            return Ok(());
        }
        let tab = browser.active_tab().ok_or("No active tab")?;
        let paths: Vec<_> = tab.selection.paths().cloned().collect();
        let directory = tab.browser.current_path().to_owned();
        match action {
            Action::Copy | Action::Cut => {
                clipboard::write_files(&paths, action == Action::Cut)?;
            }
            Action::CopyPath => {
                let text = paths
                    .iter()
                    .map(|p| format!("\"{}\"", p.to_string_lossy()))
                    .collect::<Vec<_>>()
                    .join("\r\n");
                clipboard::write_text(&text)?;
            }
            Action::Paste => {
                let captured = clipboard::read_files()?;
                let items = captured
                    .paths
                    .iter()
                    .map(|source| {
                        if captured.cut {
                            Operation::Move {
                                source: source.clone(),
                                destination: directory.clone(),
                            }
                        } else {
                            Operation::Copy {
                                source: source.clone(),
                                destination: directory.clone(),
                            }
                        }
                    })
                    .collect();
                self.submit(items)?;
                self.captured = Some(captured);
            }
            Action::Trash => {
                self.dialog = Some(Dialog {
                    action,
                    paths,
                    directory,
                    value: String::new(),
                });
            }
            Action::Rename => {
                if paths.len() != 1 {
                    return Err("Select exactly one item to rename".into());
                }
                let name = paths[0]
                    .file_name()
                    .ok_or("Cannot rename root")?
                    .to_str().ok_or("This name contains non-Unicode Windows characters; text rename is unavailable")?.to_owned();
                self.dialog = Some(Dialog {
                    action,
                    paths,
                    directory,
                    value: name,
                });
            }
            Action::CopyTo | Action::MoveTo | Action::NewFile | Action::NewFolder => {
                self.dialog = Some(Dialog {
                    action,
                    paths,
                    directory,
                    value: String::new(),
                });
            }
            _ => {}
        }
        Ok(())
    }
    pub fn show(&mut self, ui: &mut egui::Ui, browser: &mut BrowserUi) {
        if let Some(result) = self.worker.try_result() {
            if let Some(captured) = self.captured.take() {
                let completed: Vec<_> = result
                    .items
                    .iter()
                    .filter_map(|r| {
                        if r.status == ItemStatus::Completed {
                            if let Operation::Move { source, .. } = &r.operation {
                                Some(source.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                if captured.cut
                    && let Err(error) = clipboard::remove_completed(&captured, &completed)
                {
                    browser.status = error;
                }
            }
            let completed_paths = result
                .items
                .iter()
                .filter(|item| item.status == ItemStatus::Completed)
                .filter_map(|item| match item.operation {
                    Operation::Create { .. } | Operation::Rename { .. } => item.final_path.clone(),
                    _ => None,
                })
                .collect();
            browser.refresh_with_selection(completed_paths);
            self.ledger = Some(result);
        }
        let ctx = ui.ctx().clone();
        if !self.external_busy
            && ctx.input(|i| i.viewport().close_requested())
            && (self.worker.is_busy() || !self.worker.is_finished())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            if self.worker.is_busy() {
                self.close_requested = true;
            } else {
                self.worker.begin_shutdown();
                self.shutting_down = true;
            }
        }
        if self.shutting_down && self.worker.is_finished() && !self.external_busy {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if self.close_requested {
            egui::Window::new("Operation in progress").collapsible(false).show(&ctx,|ui|{
                ui.label("Keep the application open until the operation finishes, or request cancellation and wait.");
                if ui.button("Cancel operation and close when safe").clicked(){self.worker.begin_shutdown();self.shutting_down=true;self.close_requested=false;}
                if ui.button("Keep open").clicked(){self.close_requested=false;}
                if ui.button("Wait, then close").clicked(){self.close_requested=false;self.shutting_down=true;}
            });
        }
        if self.shutting_down && !self.worker.is_busy() {
            self.worker.begin_shutdown();
        }
        let (filesystem, selected) = browser
            .active_tab()
            .map(|t| (true, t.selection.paths().count()))
            .unwrap_or((false, 0));
        let mut requested = None;
        ui.horizontal_wrapped(|ui| {
            if ui.button("Commands").clicked() {
                self.palette = true;
            }
            for action in [
                Action::Copy,
                Action::Cut,
                Action::Paste,
                Action::CopyTo,
                Action::Trash,
                Action::NewFolder,
                Action::NewFile,
                Action::Rename,
                Action::Inspector,
            ] {
                if ui
                    .add_enabled(
                        !self.worker.is_busy()
                            && !self.shutting_down
                            && !self.external_busy
                            && action.eligible(filesystem, selected),
                        egui::Button::new(action.label()),
                    )
                    .clicked()
                {
                    requested = Some(action);
                }
            }
            if self.worker.is_busy() {
                ui.spinner();
                ui.label("Operation running");
                if ui.button("Request cancel").clicked() {
                    self.worker.cancel();
                }
            }
        });
        if self.palette {
            let mut open = true;
            egui::Window::new("Commands")
                .open(&mut open)
                .show(&ctx, |ui| {
                    ui.text_edit_singleline(&mut self.query);
                    for action in ACTIONS {
                        if action
                            .label()
                            .to_lowercase()
                            .contains(&self.query.to_lowercase())
                            && ui
                                .add_enabled(
                                    !self.worker.is_busy()
                                        && !self.shutting_down
                                        && !self.external_busy
                                        && action.eligible(filesystem, selected),
                                    egui::Button::new(action.label()),
                                )
                                .clicked()
                        {
                            requested = Some(action);
                        }
                    }
                });
            self.palette = open && requested.is_none();
        }
        if let Some(action) = requested
            && let Err(error) = self.action(action, browser)
        {
            browser.status = error;
        }
        let mut accept = false;
        let mut cancel = false;
        if let Some(dialog) = &mut self.dialog {
            egui::Window::new(dialog.action.label()).collapsible(false).show(&ctx,|ui|{
            ui.label(format!("{} selected • {}",dialog.paths.len(),dialog.directory.display()));
            if dialog.action==Action::Trash{ui.label("Move these selected items to the Recycle Bin. Permanent deletion is never a fallback.");for path in &dialog.paths{ui.label(path.to_string_lossy());}}else{ui.label(if matches!(dialog.action,Action::CopyTo|Action::MoveTo){"Destination directory"}else{"Complete name, including extension"});ui.text_edit_singleline(&mut dialog.value);}
            ui.horizontal(|ui|{accept=ui.button("Apply").clicked();cancel=ui.button("Cancel").clicked();});
        });
        }
        if cancel {
            self.dialog = None;
        }
        if accept && let Some(dialog) = self.dialog.take() {
            let items = match dialog.action {
                Action::Trash => dialog
                    .paths
                    .into_iter()
                    .map(|source| Operation::Trash { source })
                    .collect(),
                Action::Rename => dialog
                    .paths
                    .into_iter()
                    .map(|source| Operation::Rename {
                        source,
                        name: OsString::from(&dialog.value),
                        expected: None,
                    })
                    .collect(),
                Action::NewFile | Action::NewFolder => vec![Operation::Create {
                    directory: dialog.directory,
                    name: OsString::from(dialog.value),
                    folder: dialog.action == Action::NewFolder,
                }],
                Action::CopyTo | Action::MoveTo => {
                    let entered = PathBuf::from(dialog.value);
                    let destination = if entered.is_absolute() {
                        entered
                    } else {
                        dialog.directory.join(entered)
                    };
                    dialog
                        .paths
                        .into_iter()
                        .map(|source| {
                            if dialog.action == Action::CopyTo {
                                Operation::Copy {
                                    source,
                                    destination: destination.clone(),
                                }
                            } else {
                                Operation::Move {
                                    source,
                                    destination: destination.clone(),
                                }
                            }
                        })
                        .collect()
                }
                _ => Vec::new(),
            };
            if let Err(error) = self.submit(items) {
                browser.status = error;
            }
        }
        if let Some(ledger) = &self.ledger {
            egui::CollapsingHeader::new(format!("Operation {} results", ledger.id)).show(
                ui,
                |ui| {
                    if let Some(error) = &ledger.error {
                        ui.colored_label(egui::Color32::LIGHT_RED, error);
                    }
                    ui.label(format!("Aborted: {}", ledger.aborted));
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            for item in &ledger.items {
                                ui.label(format!(
                                    "{:?}: {:?} → {:?}",
                                    item.operation, item.status, item.final_path
                                ));
                            }
                        });
                },
            );
        }
        let Some(tab) = browser.active_tab() else {
            return;
        };
        if !tab.inspector_visible {
            self.preview.clear();
            self.preview_path = None;
            self.texture = None;
            return;
        }
        let path = tab.selection.cursor().map(|path| path.to_owned());
        // Metadata comes from the owned directory snapshot, never synchronous UI IO.
        let metadata = path
            .as_ref()
            .and_then(|path| {
                tab.browser
                    .snapshot()
                    .and_then(|snapshot| snapshot.entries.iter().find(|entry| &entry.path == path))
            })
            .map(|entry| {
                let size = entry
                    .size
                    .map(|size| format!("{size} bytes"))
                    .unwrap_or_else(|| "Size unavailable".into());
                let modified = entry
                    .modified
                    .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|time| format!("Modified: {} seconds since Unix epoch", time.as_secs()))
                    .unwrap_or_else(|| "Modified time unavailable".into());
                format!("{size} · {modified}")
            });
        if path != self.preview_path {
            self.texture = None;
            self.preview_path = path.clone();
            if let Some(path) = path {
                self.preview.request(path);
            } else {
                self.preview.clear();
            }
        }
        if self.preview.poll() {
            self.texture = None;
            if let Some(PreviewResult::Image {
                width,
                height,
                pixels,
            }) = self.preview.result()
            {
                let image = egui::ColorImage::from_rgba_premultiplied(
                    [*width as usize, *height as usize],
                    pixels,
                );
                self.texture =
                    Some(ctx.load_texture("selected-preview", image, egui::TextureOptions::LINEAR));
            }
        }
        egui::Panel::bottom("inspector")
            .resizable(true)
            .default_size(180.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("Inspector");
                    if let Some(path) = &self.preview_path {
                        ui.label(path.to_string_lossy());
                    }
                });
                if let Some(metadata) = &metadata {
                    ui.label(metadata);
                }
                if let Some(texture) = &self.texture {
                    let available = ui.available_size();
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} × {} pixels",
                            texture.size()[0],
                            texture.size()[1]
                        ));
                        if ui.button("Fit").clicked() {
                            let size = texture.size_vec2();
                            tab.preview_zoom = (available.x / size.x)
                                .min((available.y - 32.0).max(1.0) / size.y)
                                .clamp(0.01, 4.0);
                        }
                        if ui.button("Original scale").clicked() {
                            tab.preview_zoom = 1.0;
                        }
                        ui.add(
                            egui::Slider::new(&mut tab.preview_zoom, 0.01..=4.0)
                                .logarithmic(true)
                                .text("Zoom"),
                        );
                    });
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.image((texture.id(), texture.size_vec2() * tab.preview_zoom));
                    });
                } else {
                    match self.preview.result() {
                        Some(PreviewResult::Text(text)) => {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut tab.word_wrap, "Word wrap");
                                ui.checkbox(&mut tab.line_numbers, "Line numbers");
                            });
                            let numbered;
                            let displayed = if tab.line_numbers {
                                numbered = text
                                    .split('\n')
                                    .enumerate()
                                    .map(|(index, line)| format!("{:>6}  {line}", index + 1))
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                numbered.as_str()
                            } else {
                                text.as_str()
                            };
                            let scroll = if tab.word_wrap {
                                egui::ScrollArea::vertical()
                            } else {
                                egui::ScrollArea::both()
                            };
                            scroll.show(ui, |ui| {
                                ui.add(
                                    egui::Label::new(egui::RichText::new(displayed).monospace())
                                        .selectable(true)
                                        .wrap_mode(if tab.word_wrap {
                                            egui::TextWrapMode::Wrap
                                        } else {
                                            egui::TextWrapMode::Extend
                                        }),
                                );
                            });
                        }
                        Some(PreviewResult::Error(error)) => {
                            ui.label(error);
                        }
                        _ => {
                            ui.label("Select an item for a bounded preview.");
                        }
                    }
                }
            });
    }
}

#[cfg(test)]
mod command_tests {
    use super::Action;
    #[test]
    fn command_palette_context_eligibility() {
        assert!(!Action::Rename.eligible(true, 0));
        assert!(!Action::Rename.eligible(true, 2));
        assert!(Action::Rename.eligible(true, 1));
        assert!(!Action::Trash.eligible(false, 1));
        assert!(!Action::Copy.eligible(true, 0));
        assert!(Action::NewFolder.eligible(true, 0));
        assert!(Action::Refresh.eligible(false, 0));
    }
}
