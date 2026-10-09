mod directory;
mod navigation;
mod sorting;

use directory::{DirectoryLoader, EntryKind, Status};
use eframe::egui;
use navigation::{Intent, Navigation};
use sorting::{Column, Key, Sorter};
use std::{path::PathBuf, sync::Arc};

fn main() -> eframe::Result {
    let location = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let location = std::path::absolute(&location).unwrap_or(location);
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
    navigation: Navigation,
    sorter: Sorter,
    requested_sort: Option<Key>,
    location_text: String,
    location_editor_focused: bool,
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
        let repaint: Arc<dyn Fn() + Send + Sync> = Arc::new(move || context.request_repaint());
        let loader = DirectoryLoader::new(repaint.clone())?;
        let sorter = Sorter::new(repaint)?;
        let mut app = Self {
            loader,
            navigation: Navigation::new(location.clone()),
            sorter,
            requested_sort: None,
            location_text: location.to_string_lossy().into_owned(),
            location_editor_focused: false,
        };
        app.navigate(location, Intent::Refresh);
        Ok(app)
    }

    fn navigate(&mut self, location: PathBuf, intent: Intent) {
        let generation = self.loader.request(location.clone());
        self.navigation.begin(location, generation, intent);
    }

    fn request_sort(&mut self) {
        let key = self.navigation.key();
        if self.requested_sort != Some(key) {
            self.sorter
                .request(key, self.navigation.visible().raw_entries.clone());
            self.requested_sort = Some(key);
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        let mut command = None;
        let mut editing = false;
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.navigation.back.is_empty(), egui::Button::new("Back"))
                .on_hover_text("Back (Alt+Left)")
                .clicked()
            {
                command = self
                    .navigation
                    .back
                    .last()
                    .cloned()
                    .map(|p| (p, Intent::Back));
            }
            if ui
                .add_enabled(
                    !self.navigation.forward.is_empty(),
                    egui::Button::new("Forward"),
                )
                .on_hover_text("Forward (Alt+Right)")
                .clicked()
            {
                command = self
                    .navigation
                    .forward
                    .last()
                    .cloned()
                    .map(|p| (p, Intent::Forward));
            }
            let parent = self.navigation.up();
            if ui
                .add_enabled(parent.is_some(), egui::Button::new("Up"))
                .on_hover_text("Parent folder (Alt+Up)")
                .clicked()
            {
                command = parent.map(|p| (p, Intent::New));
            }
            if ui.button("Refresh").on_hover_text("Refresh (F5)").clicked() {
                command = Some((self.navigation.current.location.clone(), Intent::Refresh));
            }
        });
        ui.horizontal(|ui| {
            ui.strong("Location");
            let width = (ui.available_width() - 55.0).max(120.0);
            let input = ui.add_sized(
                [width, 26.0],
                egui::TextEdit::singleline(&mut self.location_text)
                    .hint_text("Folder path")
                    .id_salt("location"),
            );
            self.location_editor_focused = input.has_focus();
            editing = input.has_focus() || input.lost_focus();
            let submitted =
                input.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
            if (ui.button("Go").clicked() || submitted) && !self.location_text.is_empty() {
                command = Some((self.navigation.resolve(&self.location_text), Intent::New));
            }
        });
        egui::ScrollArea::horizontal()
            .id_salt("breadcrumbs")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut path = PathBuf::new();
                    for component in self.navigation.current.location.components() {
                        path.push(component.as_os_str());
                        if matches!(component, std::path::Component::Prefix(_)) {
                            continue;
                        }
                        let label = if matches!(component, std::path::Component::RootDir) {
                            path.as_os_str()
                        } else {
                            component.as_os_str()
                        };
                        if ui
                            .button(label.to_string_lossy())
                            .on_hover_text(path.to_string_lossy())
                            .clicked()
                        {
                            command = Some((path.clone(), Intent::New));
                        }
                    }
                });
            });
        if command.is_none() {
            command = match navigation_shortcut(ui.ctx(), editing) {
                Some(Shortcut::Back) => self
                    .navigation
                    .back
                    .last()
                    .cloned()
                    .map(|p| (p, Intent::Back)),
                Some(Shortcut::Forward) => self
                    .navigation
                    .forward
                    .last()
                    .cloned()
                    .map(|p| (p, Intent::Forward)),
                Some(Shortcut::Up) => self.navigation.up().map(|p| (p, Intent::New)),
                Some(Shortcut::Refresh) => {
                    Some((self.navigation.current.location.clone(), Intent::Refresh))
                }
                None => None,
            };
        }
        if let Some((location, intent)) = command {
            self.navigate(location, intent);
        }
    }

    fn status(&self, ui: &mut egui::Ui) {
        if let Some(path) = self.navigation.pending_location() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(format!(
                    "Opening {}… {} entries so far; previous location retained until success.",
                    path.display(),
                    self.navigation.pending_count()
                ));
            });
        }
        if let Some((path, failure)) = &self.navigation.error {
            ui.colored_label(
                egui::Color32::from_rgb(255, 180, 130),
                format!(
                    "Could not open {}: {} — {}",
                    path.display(),
                    failure.kind.label(),
                    failure.detail
                ),
            );
            ui.label("Previous location and history retained. Edit the path and Go to retry, or navigate elsewhere.");
        }
        if self.navigation.sorted_key != Some(self.navigation.key()) {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Sorting…");
            });
        }
        let directory = self.navigation.visible();
        match &directory.status {
            Status::Loading => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(format!(
                        "Loading folder… {} entries so far",
                        directory.raw_entries.len()
                    ));
                });
            }
            Status::Ready if directory.raw_entries.is_empty() => {
                ui.label("Empty folder");
            }
            Status::Ready => {
                ui.label(format!("{} entries", directory.raw_entries.len()));
            }
            Status::Failed(failure) => {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 130), failure.kind.label());
                ui.label(format!(
                    "Read {}: {}",
                    directory.location.display(),
                    failure.detail
                ));
                if !directory.raw_entries.is_empty() {
                    ui.label("Listing incomplete: some entries could not be read.");
                }
                ui.label("Refresh to retry, use Up, or enter another folder path.");
            }
        }
        if directory.metadata_errors > 0 {
            ui.label(format!(
                "Metadata unavailable for {} entries; hover a name for the error.",
                directory.metadata_errors
            ));
        }
    }

    fn table(&mut self, ui: &mut egui::Ui) -> Option<PathBuf> {
        let mut destination = None;
        let width = ui.available_width().max(720.0);
        let columns = [width - 440.0, 150.0, 180.0, 86.0];
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(width);
            ui.horizontal(|ui| {
                for ((heading, column), width) in [
                    ("Name", Column::Name),
                    ("Type", Column::Type),
                    ("Modified (UTC)", Column::Modified),
                    ("Size (bytes)", Column::Size),
                ]
                .into_iter()
                .zip(columns)
                {
                    let arrow = if self.navigation.sort.column == column {
                        if self.navigation.sort.descending {
                            " ▼"
                        } else {
                            " ▲"
                        }
                    } else {
                        ""
                    };
                    if table_cell(
                        ui,
                        width,
                        egui::Button::new(format!("{heading}{arrow}"))
                            .frame(false)
                            .truncate(),
                    )
                    .on_hover_text(
                        "Toggle sort direction. Folders first; unknown values last in each group.",
                    )
                    .clicked()
                    {
                        self.navigation.sort.toggle(column);
                    }
                }
            });
            ui.separator();
            let directory = self.navigation.visible();
            egui::ScrollArea::vertical()
                .id_salt(("directory", directory.generation))
                .auto_shrink([false, false])
                .show_rows(ui, 24.0, directory.entries.len(), |ui, rows| {
                    for index in rows {
                        let entry = &directory.entries[index];
                        ui.push_id(&entry.path, |ui| {
                            ui.horizontal(|ui| {
                                let response = if entry.kind == EntryKind::Folder {
                                    table_cell(
                                        ui,
                                        columns[0],
                                        egui::Button::new(&entry.name).frame(false).truncate(),
                                    )
                                } else {
                                    table_cell(
                                        ui,
                                        columns[0],
                                        egui::Label::new(&entry.name).truncate(),
                                    )
                                };
                                if entry.kind == EntryKind::Folder
                                    && (response.double_clicked()
                                        || (response.has_focus()
                                            && ui.input(|input| {
                                                input.key_pressed(egui::Key::Enter)
                                            })))
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
                        });
                    }
                });
        });
        destination
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shortcut {
    Back,
    Forward,
    Up,
    Refresh,
}

fn navigation_shortcut(ctx: &egui::Context, editing: bool) -> Option<Shortcut> {
    if editing {
        return None;
    }
    ctx.input_mut(|input| {
        for (modifiers, key, shortcut) in [
            (egui::Modifiers::ALT, egui::Key::ArrowLeft, Shortcut::Back),
            (
                egui::Modifiers::ALT,
                egui::Key::ArrowRight,
                Shortcut::Forward,
            ),
            (egui::Modifiers::ALT, egui::Key::ArrowUp, Shortcut::Up),
            (egui::Modifiers::NONE, egui::Key::F5, Shortcut::Refresh),
        ] {
            if input.consume_key(modifiers, key) {
                return Some(shortcut);
            }
        }
        None
    })
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
    fn navigation_shortcuts_are_suspended_for_focused_path_editing() {
        for (modifiers, key, expected) in [
            (egui::Modifiers::ALT, egui::Key::ArrowLeft, Shortcut::Back),
            (
                egui::Modifiers::ALT,
                egui::Key::ArrowRight,
                Shortcut::Forward,
            ),
            (egui::Modifiers::ALT, egui::Key::ArrowUp, Shortcut::Up),
            (egui::Modifiers::NONE, egui::Key::F5, Shortcut::Refresh),
        ] {
            let context = egui::Context::default();
            let mut text = String::from("folder");
            context
                .run_ui(egui::RawInput::default(), |ui| {
                    ui.add(egui::TextEdit::singleline(&mut text).id_salt("path-test"))
                        .request_focus();
                })
                .drop_without_applying_deltas();
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![egui::Event::Key {
                            key,
                            physical_key: None,
                            pressed: true,
                            repeat: false,
                            modifiers,
                        }],
                        ..Default::default()
                    },
                    |ui| {
                        let response =
                            ui.add(egui::TextEdit::singleline(&mut text).id_salt("path-test"));
                        assert!(response.has_focus());
                        assert_eq!(navigation_shortcut(ui.ctx(), response.has_focus()), None);
                    },
                )
                .drop_without_applying_deltas();
            // Without an editor, these same shortcuts remain available to browsing.
            let context = egui::Context::default();
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![egui::Event::Key {
                            key,
                            physical_key: None,
                            pressed: true,
                            repeat: false,
                            modifiers,
                        }],
                        ..Default::default()
                    },
                    |ui| {
                        assert_eq!(navigation_shortcut(ui.ctx(), false), Some(expected));
                    },
                )
                .drop_without_applying_deltas();
        }
    }

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
        let mut committed = false;
        let more = self.loader.drain(|response| {
            committed |= self.navigation.apply(response);
        });
        if committed && !self.location_editor_focused {
            self.location_text = self
                .navigation
                .current
                .location
                .to_string_lossy()
                .into_owned();
        }
        self.request_sort();
        if let Some(sorted) = self.sorter.take() {
            self.navigation.apply_sorted(sorted);
        }
        if more {
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
                self.navigate(location, Intent::New);
            }
            self.request_sort();
        });
    }
}
