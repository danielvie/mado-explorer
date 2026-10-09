mod directory;

use directory::{DirectoryLoader, DirectoryState, EntryKind, Status};
use eframe::egui;
use std::{path::PathBuf, sync::Arc};

fn main() -> eframe::Result {
    let location = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1050.0, 700.0])
            .with_min_inner_size([480.0, 280.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Mado Explorer",
        options,
        Box::new(move |cc| Ok(Box::new(Explorer::new(cc, location)?))),
    )
}

struct Explorer {
    loader: DirectoryLoader,
    directory: DirectoryState,
    location_text: String,
}

impl Explorer {
    fn new(cc: &eframe::CreationContext<'_>, location: PathBuf) -> std::io::Result<Self> {
        let mut style = egui::Style::default();
        style.visuals = egui::Visuals::dark();
        style.spacing.item_spacing = egui::vec2(8.0, 3.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.visuals.selection.bg_fill = egui::Color32::from_rgb(18, 93, 111);
        style.visuals.selection.stroke =
            egui::Stroke::new(1.0, egui::Color32::from_rgb(89, 215, 235));
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        cc.egui_ctx.set_global_style(style);
        let context = cc.egui_ctx.clone();
        let loader = DirectoryLoader::new(Arc::new(move || context.request_repaint()))?;
        let mut app = Self {
            loader,
            directory: DirectoryState::new(location.clone()),
            location_text: String::new(),
        };
        app.navigate(location);
        Ok(app)
    }

    fn navigate(&mut self, location: PathBuf) {
        self.location_text = location.to_string_lossy().into_owned();
        let generation = self.loader.request(location.clone());
        self.directory.begin(location, generation);
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        let mut destination = None;
        ui.horizontal(|ui| {
            ui.strong("Location");
            let width = (ui.available_width() - 140.0).max(120.0);
            let input = ui.add_sized(
                [width, 26.0],
                egui::TextEdit::singleline(&mut self.location_text)
                    .hint_text("Folder path")
                    .id_salt("location"),
            );
            let submitted =
                input.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
            if ui.button("Go").clicked() || submitted {
                let path = PathBuf::from(&self.location_text);
                if !self.location_text.is_empty() {
                    destination = Some(if path.is_absolute() {
                        path
                    } else {
                        self.directory.location.join(path)
                    });
                }
            }
            if ui.button("Refresh").clicked() {
                destination = Some(self.directory.location.clone());
            }
        });
        if let Some(destination) = destination {
            self.navigate(destination);
        }
    }

    fn status(&self, ui: &mut egui::Ui) {
        match &self.directory.status {
            Status::Loading => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(format!(
                        "Loading folder… {} entries so far",
                        self.directory.entries.len()
                    ));
                });
            }
            Status::Ready if self.directory.entries.is_empty() => {
                ui.label("Empty folder");
            }
            Status::Ready => {
                ui.label(format!("{} entries", self.directory.entries.len()));
            }
            Status::Failed(failure) => {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 130), failure.kind.label());
                ui.label(format!(
                    "Read {}: {}",
                    self.directory.location.display(),
                    failure.detail
                ));
                if !self.directory.entries.is_empty() {
                    ui.label("Listing incomplete: some entries could not be read.");
                }
                ui.label("Refresh to retry, or enter another folder path.");
            }
        }
        if self.directory.metadata_errors > 0 {
            ui.label(format!(
                "Metadata unavailable for {} entries; hover a name for the error.",
                self.directory.metadata_errors
            ));
        }
    }

    fn table(&self, ui: &mut egui::Ui) -> Option<PathBuf> {
        let mut destination = None;
        let width = ui.available_width().max(720.0);
        let columns = [width - 440.0, 150.0, 180.0, 86.0];
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(width);
            ui.horizontal(|ui| {
                for (heading, width) in ["Name", "Type", "Modified (UTC)", "Size (bytes)"]
                    .into_iter()
                    .zip(columns)
                {
                    table_cell(
                        ui,
                        width,
                        egui::Label::new(egui::RichText::new(heading).strong()),
                    );
                }
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .id_salt(("directory", self.directory.generation))
                .auto_shrink([false, false])
                .show_rows(ui, 24.0, self.directory.entries.len(), |ui, rows| {
                    for index in rows {
                        let entry = &self.directory.entries[index];
                        ui.horizontal(|ui| {
                            let response = if entry.kind == EntryKind::Folder {
                                table_cell(
                                    ui,
                                    columns[0],
                                    egui::Button::new(&entry.name).frame(false).truncate(),
                                )
                            } else {
                                table_cell(ui, columns[0], egui::Label::new(&entry.name).truncate())
                            };
                            if entry.kind == EntryKind::Folder
                                && (response.double_clicked()
                                    || (response.has_focus()
                                        && ui.input(|input| input.key_pressed(egui::Key::Enter))))
                            {
                                destination = Some(entry.path.clone());
                            }
                            response.on_hover_ui(|ui| {
                                ui.label(entry.path.to_string_lossy());
                                if entry.kind == EntryKind::Folder {
                                    ui.label(
                                        "Double-click or focus and press Enter to open folder.",
                                    );
                                }
                                if let Some(error) = &entry.metadata_error {
                                    ui.label(error);
                                }
                            });
                            table_cell(
                                ui,
                                columns[1],
                                egui::Label::new(entry.kind.label()).truncate(),
                            );
                            table_cell(
                                ui,
                                columns[2],
                                egui::Label::new(&entry.modified).truncate(),
                            );
                            let size = entry
                                .size
                                .map_or_else(|| "—".into(), |size| size.to_string());
                            table_cell(ui, columns[3], egui::Label::new(size).truncate());
                        });
                    }
                });
        });
        destination
    }
}

fn table_cell(ui: &mut egui::Ui, width: f32, widget: impl egui::Widget) -> egui::Response {
    ui.allocate_ui_with_layout(
        egui::vec2(width, 24.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_min_size(egui::vec2(width, 24.0));
            ui.add(widget)
        },
    )
    .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_cell_keeps_columns_aligned_for_short_and_truncated_names() {
        let context = egui::Context::default();
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
                for name in ["a", "a long folder name ".repeat(40).as_str()] {
                    ui.horizontal(|ui| {
                        let name =
                            table_cell(ui, 280.0, egui::Button::new(name).frame(false).truncate());
                        let kind = table_cell(ui, 150.0, egui::Label::new("Folder"));
                        assert!((kind.rect.left() - name.rect.left() - 288.0).abs() < 0.1);
                        assert!(name.rect.width() <= 280.0);
                        assert!(name.rect.height() <= 24.0);
                    });
                }
            },
        );
        output.drop_without_applying_deltas();
    }
}

impl eframe::App for Explorer {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.loader.drain_into(&mut self.directory) {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.toolbar(ui);
            ui.separator();
            self.status(ui);
            ui.separator();
            if let Some(location) = self.table(ui) {
                self.navigate(location);
            }
        });
    }
}
