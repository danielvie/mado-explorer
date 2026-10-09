mod directory;
mod font_fallback;
mod native;
mod navigation;
mod open;
mod selection;
mod sorting;

use directory::{DirectoryLoader, EntryKind, Status};
use eframe::egui;
use navigation::{Intent, Navigation};
use open::Launcher;
use selection::Movement;
use sorting::{Column, Key, Sorter};
use std::{path::PathBuf, sync::Arc};

fn main() -> eframe::Result {
    let location = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let location = std::path::absolute(&location).unwrap_or(location);
    let fonts = font_fallback::load_installed();
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
        Box::new(move |cc| Ok(Box::new(Explorer::new(cc, location, fonts)?))),
    )
}

struct Explorer {
    loader: DirectoryLoader,
    navigation: Navigation,
    sorter: Sorter,
    launcher: Launcher,
    requested_sort: Option<Key>,
    location_text: String,
    location_editor_focused: bool,
    location_error: Option<String>,
    font_warning: Option<String>,
    filter_text: String,
    row_scroll_offset: f32,
    row_viewport_height: f32,
    scroll_to_active: bool,
}

impl Explorer {
    fn new(
        cc: &eframe::CreationContext<'_>,
        location: PathBuf,
        fonts: font_fallback::InstalledFonts,
    ) -> std::io::Result<Self> {
        let mut style = egui::Style::default();
        style.visuals = egui::Visuals::dark();
        style.spacing.item_spacing = egui::vec2(8.0, 3.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.visuals.selection.bg_fill = egui::Color32::from_rgb(18, 93, 111);
        style.visuals.selection.stroke =
            egui::Stroke::new(1.0, egui::Color32::from_rgb(89, 215, 235));
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        cc.egui_ctx.set_global_style(style);
        cc.egui_ctx.set_fonts(fonts.definitions);
        let context = cc.egui_ctx.clone();
        let repaint: Arc<dyn Fn() + Send + Sync> = Arc::new(move || context.request_repaint());
        let loader = DirectoryLoader::new(repaint.clone())?;
        let sorter = Sorter::new(repaint)?;
        let mut app = Self {
            loader,
            navigation: Navigation::new(location.clone()),
            sorter,
            launcher: Launcher::default(),
            requested_sort: None,
            location_text: native::display(location.as_os_str()),
            location_editor_focused: false,
            location_error: None,
            font_warning: fonts.warning,
            filter_text: String::new(),
            row_scroll_offset: 0.0,
            row_viewport_height: 0.0,
            scroll_to_active: false,
        };
        app.navigate(location, Intent::Refresh);
        Ok(app)
    }

    fn navigate(&mut self, location: PathBuf, intent: Intent) {
        self.location_error = None;
        let generation = self.loader.request(location.clone());
        self.navigation.begin(location, generation, intent);
    }

    fn request_sort(&mut self) {
        let key = self.navigation.key();
        if self.requested_sort != Some(key) {
            self.sorter.request(
                key,
                self.navigation.visible().raw_entries.clone(),
                self.navigation.filter.clone(),
            );
            self.requested_sort = Some(key);
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) -> bool {
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
                match self.navigation.resolve(&self.location_text) {
                    Ok(path) => command = Some((path, Intent::New)),
                    Err(error) => self.location_error = Some(error.into()),
                }
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
                            .button(native::display(label))
                            .on_hover_text(native::display(path.as_os_str()))
                            .clicked()
                        {
                            command = Some((path.clone(), Intent::New));
                        }
                    }
                });
            });
        ui.horizontal(|ui| {
            ui.strong("Filter");
            let width = (ui.available_width() - 65.0).max(100.0);
            let input = ui.add_sized(
                [width, 24.0],
                egui::TextEdit::singleline(&mut self.filter_text)
                    .hint_text("Fuzzy name filter")
                    .id(egui::Id::new("filter")),
            );
            editing |= input.has_focus() || input.lost_focus();
            if input.has_focus()
                && ui.input_mut(|input| {
                    consume_exact_key(input, egui::Modifiers::NONE, egui::Key::Escape)
                })
            {
                input.surrender_focus();
            }
            let clear = ui
                .add_enabled(!self.filter_text.is_empty(), egui::Button::new("Clear"))
                .on_hover_text("Restore all entries without navigation")
                .clicked();
            if clear {
                self.filter_text.clear();
            }
            if input.changed() || clear {
                self.navigation.set_filter(self.filter_text.clone());
            }
        });
        ui.horizontal(|ui| {
            let mut show = self.navigation.show_hidden_system;
            if ui.checkbox(&mut show, "Show hidden/system entries")
                .on_hover_text("Hidden and system entries are hidden by default. This affects immediate entries only; no links are followed.")
                .changed()
            {
                self.navigation.set_show_hidden_system(show);
            }
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
        editing
    }

    fn status(&self, ui: &mut egui::Ui) {
        if let Some(warning) = &self.font_warning {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 130), "Installed font fallback limited; some Unicode characters may appear as boxes. Hover for details.")
                .on_hover_text(warning);
        }
        if let Some(error) = &self.location_error {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 130), error);
            ui.label("Location, history and selection retained. Edit the path and Go to retry.");
        }
        if let Some(path) = self.launcher.busy_path() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(format!("Requesting default application for {}… (one launch at a time; browsing remains available)", native::display(path.as_os_str())));
            });
        }
        if let Some(path) = &self.launcher.refused {
            ui.colored_label(egui::Color32::from_rgb(255, 180, 130), format!("Did not launch {}: another launch was busy when requested. This request is not queued; retry explicitly when the launch slot is free.", native::display(path.as_os_str())));
        }
        if let Some(report) = &self.launcher.report {
            match &report.result {
                Ok(()) => {
                    ui.label(format!("Windows accepted the launch of {}. External application completion is not monitored.", native::display(report.path.as_os_str())));
                }
                Err(error) => {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 130), format!("Could not launch {}: {}. Browsing location, history and selection retained.", native::display(report.path.as_os_str()), error));
                }
            }
        }
        if let Some(path) = self.navigation.pending_location() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(format!(
                    "Opening {}… {} entries so far; previous location retained until success.",
                    native::display(path.as_os_str()),
                    self.navigation.pending_count()
                ));
            });
        }
        if let Some((path, failure)) = &self.navigation.error {
            ui.colored_label(
                egui::Color32::from_rgb(255, 180, 130),
                format!(
                    "Could not open {}: {} — {}",
                    native::display(path.as_os_str()),
                    failure.kind.label(),
                    failure.detail
                ),
            );
            ui.label("Previous location and history retained. Edit the path and Go to retry, or navigate elsewhere.");
        }
        if !self.navigation.projection_fresh() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Updating filter/sort/visibility… previous rows are not interactive.");
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
            Status::Ready => {}
            Status::Failed(failure) => {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 130), failure.kind.label());
                ui.label(format!(
                    "Read {}: {}",
                    native::display(directory.location.as_os_str()),
                    failure.detail
                ));
                if !directory.raw_entries.is_empty() {
                    ui.label("Listing incomplete: some entries could not be read.");
                }
                ui.label("Refresh to retry, use Up, or enter another folder path.");
            }
        }
        if self.navigation.projection_fresh() {
            ui.label(format!(
                "{} / {} entries displayed{}",
                directory.entries.len(),
                directory.raw_entries.len(),
                if matches!(directory.status, Status::Loading | Status::Failed(_)) {
                    " (incomplete)"
                } else {
                    ""
                }
            ));
            if matches!(directory.status, Status::Ready)
                && self.navigation.filter.is_empty()
                && !directory.raw_entries.is_empty()
                && directory.entries.is_empty()
            {
                ui.label("All entries are hidden/system. Enable Show hidden/system entries to view them.");
            }
            if matches!(directory.status, Status::Ready)
                && !self.navigation.filter.is_empty()
                && !directory.raw_entries.is_empty()
                && directory.entries.is_empty()
            {
                ui.label("No names match the current filter/visibility settings. Clear or edit the filter, or show hidden/system entries.");
            }
        }
        if directory.metadata_errors > 0 {
            ui.label(format!(
                "Metadata unavailable for {} entries; hover a name for the error.",
                directory.metadata_errors
            ));
        }
    }

    fn table(&mut self, ui: &mut egui::Ui, editing: bool) -> Option<Activation> {
        let mut destination = None;
        let selectable = self.navigation.rows_selectable();
        let table_focus = ui.add(
            egui::Label::new(format!(
                "Entries — {} selected{}",
                self.navigation.selection.paths.len(),
                if selectable {
                    ""
                } else if self.navigation.pending_location().is_some() {
                    " (selection paused until destination opens)"
                } else if !self.navigation.projection_fresh() {
                    " (selection paused while filter/sort/visibility updates)"
                } else {
                    " (selection paused: filtered listing is loading or incomplete; refresh to retry)"
                }
            ))
            .sense(egui::Sense::click()),
        );
        if table_focus.clicked() && selectable {
            table_focus.request_focus();
        }
        if table_focus.has_focus() {
            ui.painter().rect_stroke(
                table_focus.rect.expand(2.0),
                0.0,
                ui.visuals().selection.stroke,
                egui::StrokeKind::Inside,
            );
        }
        let width = ui.available_width().max(720.0);
        let columns = [width - 440.0, 150.0, 180.0, 86.0];
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_min_width(width);
            ui.horizontal(|ui| {
                for ((heading, column), width) in [
                    ("Name", Column::Name), ("Type", Column::Type),
                    ("Modified (UTC)", Column::Modified), ("Size (bytes)", Column::Size),
                ].into_iter().zip(columns) {
                    let arrow = if self.navigation.sort.column == column {
                        if self.navigation.sort.descending { " v" } else { " ^" }
                    } else { "" };
                    if table_cell(ui, width, egui::Button::new(format!("{heading}{arrow}")).frame(false).truncate())
                        .on_hover_text("Toggle sort direction. Folders first; unknown values last in each group.").clicked() {
                        self.navigation.sort.toggle(column);
                    }
                }
            });
            ui.separator();
            // A heading click may have invalidated the projection in this frame.
            let selectable = self.navigation.rows_selectable();
            let entries = self.navigation.visible().entries.clone();
            let pitch = 24.0 + ui.spacing().item_spacing.y;
            let available_height = ui.available_height().max(24.0);
            let viewport_height = if self.row_viewport_height > 0.0 {
                self.row_viewport_height.min(available_height).max(24.0)
            } else { available_height };
            if selectable {
                let enter_allowed = ui.memory(|memory| memory.focused().is_none() || memory.has_focus(table_focus.id));
                for command in selection_shortcuts(ui.ctx(), editing, enter_allowed) {
                    match command {
                        SelectionShortcut::Move(movement, shift) => {
                            self.navigation.selection.move_active(&entries, movement, (viewport_height / pitch).floor() as usize, shift);
                            table_focus.request_focus();
                            self.scroll_to_active = true;
                        }
                        SelectionShortcut::All => {
                            self.navigation.selection.select_all(&entries);
                            table_focus.request_focus();
                        }
                        SelectionShortcut::Open => { if destination.is_none() { destination = active_activation(&self.navigation, editing); } }
                    }
                }
            }
            let directory = self.navigation.visible();
            let mut scroll = egui::ScrollArea::vertical()
                .id_salt(("directory", &directory.location))
                .content_margin(egui::Margin::ZERO).auto_shrink([false, false]);
            if self.scroll_to_active && selectable {
                if let Some(index) = self.navigation.selection.active_index(&entries) {
                    scroll = scroll.vertical_scroll_offset(row_scroll_offset(self.row_scroll_offset, viewport_height, pitch, index));
                }
                self.scroll_to_active = false;
            }
            let output = scroll.show_rows(ui, 24.0, entries.len(), |ui, rows| {
                for index in rows {
                    let entry = &entries[index];
                    let selected = selectable && self.navigation.selection.paths.contains(&entry.path);
                    let active = selectable && self.navigation.selection.active.as_ref() == Some(&entry.path);
                    ui.push_id(&entry.path, |ui| {
                        let response = entry_row(ui, entry, columns, width, selectable, selected, active, editing);
                        if selectable && response.clicked() {
                            let modifiers = ui.input(|input| input.modifiers);
                            if !modifiers.alt && !modifiers.mac_cmd {
                                self.navigation.selection.click(&entries, index, modifiers.ctrl, modifiers.shift);
                                table_focus.request_focus();
                            }
                        }
                        if response.double_clicked() && destination.is_none() {
                            destination = capture_activation(&self.navigation, &entry.path, editing);
                        }
                        response.on_hover_ui(|ui| {
                            ui.label(native::display(entry.path.as_os_str()));
                            if entry.hidden { ui.label("Hidden attribute"); }
                            if entry.system { ui.label("System attribute"); }
                            if entry.kind == EntryKind::Folder { ui.label("Double-click or make active and press Enter to open folder."); }
                            if entry.kind == EntryKind::File { ui.label("Double-click or make active and press Enter to open in the Windows default application. External apps may edit files."); }
                            if let Some(error) = &entry.metadata_error { ui.label(error); }
                        });
                    });
                }
            });
            self.row_scroll_offset = output.state.offset.y;
            self.row_viewport_height = output.inner_rect.height();
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
            if consume_exact_key(input, modifiers, key) {
                return Some(shortcut);
            }
        }
        None
    })
}

fn consume_exact_key(
    input: &mut egui::InputState,
    wanted: egui::Modifiers,
    key: egui::Key,
) -> bool {
    let index = input.events.iter().position(|event| {
        matches!(event,
            egui::Event::Key { key: pressed_key, modifiers, pressed: true, .. }
                if *pressed_key == key && modifiers.ctrl == wanted.ctrl
                    && modifiers.alt == wanted.alt && modifiers.shift == wanted.shift
                    && modifiers.mac_cmd == wanted.mac_cmd
        )
    });
    if let Some(index) = index {
        input.events.remove(index);
        true
    } else {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelectionShortcut {
    Move(Movement, bool),
    All,
    Open,
}

fn selection_shortcuts(
    ctx: &egui::Context,
    editing: bool,
    enter_allowed: bool,
) -> Vec<SelectionShortcut> {
    if editing {
        return Vec::new();
    }
    ctx.input_mut(|input| {
        let mut commands = Vec::new();
        input.events.retain(|event| {
            let egui::Event::Key {
                key,
                modifiers,
                pressed: true,
                repeat,
                ..
            } = event
            else {
                return true;
            };
            if modifiers.alt || modifiers.mac_cmd {
                return true;
            }
            let command = if modifiers.ctrl {
                if *key == egui::Key::A && !modifiers.shift {
                    Some(SelectionShortcut::All)
                } else {
                    None
                }
            } else {
                let movement = match key {
                    egui::Key::ArrowUp => Some(Movement::Up),
                    egui::Key::ArrowDown => Some(Movement::Down),
                    egui::Key::Home => Some(Movement::Home),
                    egui::Key::End => Some(Movement::End),
                    egui::Key::PageUp => Some(Movement::PageUp),
                    egui::Key::PageDown => Some(Movement::PageDown),
                    _ => None,
                };
                movement
                    .map(|movement| SelectionShortcut::Move(movement, modifiers.shift))
                    .or_else(|| {
                        (*key == egui::Key::Enter && !modifiers.shift && !repeat && enter_allowed)
                            .then_some(SelectionShortcut::Open)
                    })
            };
            if let Some(command) = command {
                commands.push(command);
                false
            } else {
                true
            }
        });
        commands
    })
}

#[derive(Debug, PartialEq, Eq)]
enum Activation {
    Folder(PathBuf),
    File(PathBuf),
}

fn capture_activation(
    navigation: &Navigation,
    path: &std::path::Path,
    editing: bool,
) -> Option<Activation> {
    if editing || !navigation.rows_selectable() {
        return None;
    }
    let entry = navigation
        .current
        .entries
        .iter()
        .find(|entry| entry.path == path)?;
    match entry.kind {
        EntryKind::Folder => Some(Activation::Folder(entry.path.clone())),
        EntryKind::File => Some(Activation::File(entry.path.clone())),
        _ => None,
    }
}

fn active_activation(navigation: &Navigation, editing: bool) -> Option<Activation> {
    capture_activation(navigation, navigation.selection.active.as_deref()?, editing)
}

fn row_scroll_offset(offset: f32, viewport_height: f32, pitch: f32, index: usize) -> f32 {
    let top = index as f32 * pitch;
    let bottom = top + 24.0;
    if top < offset {
        top
    } else if bottom > offset + viewport_height {
        (bottom - viewport_height).max(0.0)
    } else {
        offset
    }
}

fn entry_row(
    ui: &mut egui::Ui,
    entry: &directory::Entry,
    columns: [f32; 4],
    width: f32,
    selectable: bool,
    selected: bool,
    active: bool,
    editing: bool,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, 24.0),
        if selectable {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, ui.visuals().selection.bg_fill);
    } else if selectable && response.hovered() {
        ui.painter()
            .rect_filled(rect, 0.0, ui.visuals().widgets.hovered.bg_fill);
    }
    if active {
        ui.painter().rect_stroke(
            rect.shrink(0.5),
            0.0,
            if editing {
                ui.visuals().widgets.inactive.fg_stroke
            } else {
                ui.visuals().selection.stroke
            },
            egui::StrokeKind::Inside,
        );
    }
    let size = entry
        .size
        .map_or_else(|| "—".into(), |size| size.to_string());
    let mut x = rect.left();
    for (text, cell_width) in [
        entry.name.as_str(),
        entry.kind.label(),
        &entry.modified,
        &size,
    ]
    .into_iter()
    .zip(columns)
    {
        let cell =
            egui::Rect::from_min_size(egui::pos2(x, rect.top()), egui::vec2(cell_width, 24.0));
        // A manually placed, hover-only label must neither advance the row
        // cursor nor consume clicks intended for the whole row behind it.
        ui.new_child(
            egui::UiBuilder::new()
                .max_rect(cell)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        )
        .add(egui::Label::new(text).truncate().selectable(false));
        x += cell_width + ui.spacing().item_spacing.x;
    }
    response
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

    fn key_event(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    #[test]
    fn selection_shortcuts_respect_editor_ownership_and_exact_modifiers() {
        for key in [
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::Home,
            egui::Key::End,
            egui::Key::PageUp,
            egui::Key::PageDown,
            egui::Key::Enter,
            egui::Key::A,
        ] {
            for modifiers in [
                egui::Modifiers::NONE,
                egui::Modifiers::SHIFT,
                egui::Modifiers::CTRL,
            ] {
                let context = egui::Context::default();
                let mut text = "folder".to_owned();
                context
                    .run_ui(egui::RawInput::default(), |ui| {
                        ui.add(egui::TextEdit::singleline(&mut text).id_salt("selection-path"))
                            .request_focus();
                    })
                    .drop_without_applying_deltas();
                context.run_ui(egui::RawInput { events: vec![key_event(key, modifiers)], ..Default::default() }, |ui| {
                    // Check before TextEdit gets a chance to consume the event.
                    assert!(selection_shortcuts(ui.ctx(), true, true).is_empty());
                    assert!(ui.input(|input| input.events.iter().any(|event| matches!(event, egui::Event::Key { key: k, .. } if *k == key))));
                    let response = ui.add(egui::TextEdit::singleline(&mut text).id_salt("selection-path"));
                    assert!(response.has_focus() || response.lost_focus());
                    assert!(selection_shortcuts(ui.ctx(), response.has_focus() || response.lost_focus(), true).is_empty());
                }).drop_without_applying_deltas();
            }
        }
        let context = egui::Context::default();
        let events = vec![
            key_event(egui::Key::ArrowUp, egui::Modifiers::ALT),
            key_event(egui::Key::Home, egui::Modifiers::CTRL),
            key_event(egui::Key::A, egui::Modifiers::CTRL | egui::Modifiers::SHIFT),
            key_event(egui::Key::Enter, egui::Modifiers::SHIFT),
            key_event(egui::Key::ArrowDown, egui::Modifiers::NONE),
            key_event(egui::Key::End, egui::Modifiers::SHIFT),
            key_event(egui::Key::A, egui::Modifiers::CTRL),
            egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: false,
                repeat: false,
                modifiers: egui::Modifiers::SHIFT,
            },
            key_event(egui::Key::Enter, egui::Modifiers::NONE),
        ];
        context
            .run_ui(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ui| {
                    assert_eq!(
                        selection_shortcuts(ui.ctx(), false, true),
                        vec![
                            SelectionShortcut::Move(Movement::Down, false),
                            SelectionShortcut::Move(Movement::End, true),
                            SelectionShortcut::All,
                            SelectionShortcut::Open,
                        ]
                    );
                    // Four unhandled key presses plus the Enter release remain.
                    assert_eq!(ui.input(|input| input.events.len()), 5);
                    assert_eq!(navigation_shortcut(ui.ctx(), false), Some(Shortcut::Up));
                    assert_eq!(ui.input(|input| input.events.len()), 4);
                },
            )
            .drop_without_applying_deltas();
        for modifiers in [
            egui::Modifiers::ALT | egui::Modifiers::SHIFT,
            egui::Modifiers::ALT | egui::Modifiers::CTRL,
        ] {
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![key_event(egui::Key::ArrowUp, modifiers)],
                        ..Default::default()
                    },
                    |ui| {
                        assert_eq!(navigation_shortcut(ui.ctx(), false), None);
                        assert!(selection_shortcuts(ui.ctx(), false, true).is_empty());
                    },
                )
                .drop_without_applying_deltas();
        }
        context
            .run_ui(
                egui::RawInput {
                    events: vec![key_event(egui::Key::Enter, egui::Modifiers::NONE)],
                    ..Default::default()
                },
                |ui| {
                    assert!(selection_shortcuts(ui.ctx(), false, false).is_empty());
                    assert_eq!(ui.input(|input| input.events.len()), 1);
                },
            )
            .drop_without_applying_deltas();
    }

    #[test]
    fn open_activation_requires_a_fresh_visible_file_or_folder() {
        let mut navigation = Navigation::new(PathBuf::from("root"));
        let rows = selection::tests::rows(&["root/a", "root/b"]);
        navigation.current.entries = rows.clone();
        navigation.sorted_key = Some(navigation.key());
        assert_eq!(active_activation(&navigation, false), None);
        navigation.selection.click(&rows, 1, false, false);
        assert_eq!(
            active_activation(&navigation, false),
            Some(Activation::Folder(PathBuf::from("root/b")))
        );
        navigation.current.entries = Arc::new(vec![rows[0].clone()]);
        assert_eq!(active_activation(&navigation, false), None); // missing active row is not row zero
        navigation.current.entries = rows.clone();
        navigation.begin(PathBuf::from("destination"), 1, Intent::New);
        assert_eq!(active_activation(&navigation, false), None);
        let mut file = rows[1].as_ref().path.clone();
        file.push("file");
        let file = Arc::new(directory::Entry {
            path: file.clone(),
            name: "file".into(),
            kind: EntryKind::File,
            modified: "—".into(),
            modified_time: None,
            size: Some(0),
            metadata_error: None,
            hidden: false,
            system: false,
        });
        let mut navigation = Navigation::new(PathBuf::from("root"));
        navigation.current.entries = Arc::new(vec![file]);
        navigation.sorted_key = Some(navigation.key());
        navigation
            .selection
            .click(&navigation.current.entries, 0, false, false);
        assert_eq!(
            active_activation(&navigation, false),
            Some(Activation::File(navigation.current.entries[0].path.clone()))
        );
        assert_eq!(active_activation(&navigation, true), None);
    }

    #[test]
    fn open_file_enter_obeys_editor_focus_repeat_and_pending_projection_guards() {
        let context = egui::Context::default();
        let mut app = filter_app();
        let file = Arc::new(directory::Entry {
            path: PathBuf::from(r"C:\fixture\captured & file.txt"),
            name: "not the command path".into(),
            kind: EntryKind::File,
            modified: "—".into(),
            modified_time: None,
            size: Some(0),
            metadata_error: None,
            hidden: false,
            system: false,
        });
        app.navigation.current.entries = Arc::new(vec![file.clone()]);
        app.navigation
            .selection
            .select_all(&app.navigation.current.entries);
        let run = |app: &mut Explorer, editing: bool, focused_control: bool, repeat: bool| {
            let mut result = None;
            let mut events = Vec::new();
            // egui recomputes repeat from held keys; model physical release only
            // for fresh presses, and leave Enter held for the repeat scenario.
            if !repeat {
                events.push(egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: None,
                    pressed: false,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                });
            }
            events.push(key_event(egui::Key::Enter, egui::Modifiers::NONE));
            context
                .run_ui(
                    egui::RawInput {
                        events,
                        ..Default::default()
                    },
                    |ui| {
                        if focused_control {
                            ui.button("Other control").request_focus();
                        } else {
                            ui.memory_mut(|memory| {
                                if let Some(id) = memory.focused() {
                                    memory.surrender_focus(id);
                                }
                            });
                        }
                        result = app.table(ui, editing);
                    },
                )
                .drop_without_applying_deltas();
            result
        };
        assert_eq!(
            run(&mut app, false, false, false),
            Some(Activation::File(file.path.clone()))
        );
        assert_eq!(run(&mut app, true, false, false), None); // editor and submission frame
        assert_eq!(run(&mut app, false, true, false), None);
        assert_eq!(run(&mut app, false, false, true), None); // holding Enter is not repeated launch
        app.navigation.set_filter("captured".into());
        assert_eq!(run(&mut app, false, false, false), None); // stale projection
        app.navigation.sorted_key = Some(app.navigation.key());
        app.navigation.current.status = Status::Loading;
        assert_eq!(run(&mut app, false, false, false), None); // incomplete filtered listing
        app.navigation.current.status = Status::Ready;
        app.navigation
            .begin(PathBuf::from(r"C:\destination"), 7, Intent::New);
        assert_eq!(run(&mut app, false, false, false), None);
    }

    #[test]
    fn open_double_click_routes_visible_file_and_folder_without_launching() {
        for (kind, editing, expected) in [
            (
                EntryKind::File,
                false,
                Some(Activation::File(PathBuf::from("alpha"))),
            ),
            (
                EntryKind::Folder,
                false,
                Some(Activation::Folder(PathBuf::from("alpha"))),
            ),
            (EntryKind::File, true, None),
            (EntryKind::Link, false, None),
        ] {
            let context = egui::Context::default();
            let mut app = filter_app();
            let mut rows = selection::tests::rows(&["alpha"]);
            let row = Arc::get_mut(&mut Arc::get_mut(&mut rows).unwrap()[0]).unwrap();
            row.kind = kind;
            row.name = "alpha".into();
            app.navigation.current.entries = rows;
            let mut pos = egui::Pos2::ZERO;
            let output = context.run_ui(egui::RawInput::default(), |ui| {
                app.table(ui, editing);
            });
            for shape in &output.shapes {
                if let egui::Shape::Text(text) = &shape.shape {
                    if text.galley.job.text == "alpha" {
                        pos = text.pos + text.galley.size() / 2.0;
                    }
                }
            }
            output.drop_without_applying_deltas();
            assert_ne!(pos, egui::Pos2::ZERO);
            for (index, pressed) in [true, false, true, false].into_iter().enumerate() {
                let mut activation = None;
                context
                    .run_ui(
                        egui::RawInput {
                            time: Some(index as f64 * 0.05 + 1.0),
                            events: vec![
                                egui::Event::PointerMoved(pos),
                                egui::Event::PointerButton {
                                    pos,
                                    button: egui::PointerButton::Primary,
                                    pressed,
                                    modifiers: egui::Modifiers::NONE,
                                },
                            ],
                            ..Default::default()
                        },
                        |ui| {
                            activation = app.table(ui, editing);
                        },
                    )
                    .drop_without_applying_deltas();
                assert_eq!(
                    activation,
                    if index == 3 {
                        expected.as_ref().map(|a| match a {
                            Activation::File(path) => Activation::File(path.clone()),
                            Activation::Folder(path) => Activation::Folder(path.clone()),
                        })
                    } else {
                        None
                    }
                );
            }
            assert!(app.launcher.busy_path().is_none()); // routing tests never call the shell
        }
    }

    #[cfg(windows)]
    #[test]
    fn open_captured_native_file_survives_selection_navigation_and_failure_reporting() {
        use std::{
            ffi::OsString,
            os::windows::ffi::OsStringExt,
            sync::mpsc,
            time::{Duration, Instant},
        };
        let mut app = filter_app();
        let mut units: Vec<_> = r"C:\fixture\".encode_utf16().collect();
        units.extend([0xD800, b'.' as u16, b't' as u16, b'x' as u16, b't' as u16]);
        let original = PathBuf::from(OsString::from_wide(&units));
        let row = Arc::new(directory::Entry {
            path: original.clone(),
            name: "lossy display is not a path".into(),
            kind: EntryKind::File,
            modified: "—".into(),
            modified_time: None,
            size: Some(0),
            metadata_error: None,
            hidden: false,
            system: false,
        });
        app.navigation.current.entries = Arc::new(vec![row]);
        app.navigation
            .selection
            .select_all(&app.navigation.current.entries);
        let Some(Activation::File(captured)) = active_activation(&app.navigation, false) else {
            panic!("capture file");
        };
        let (release_tx, release_rx) = mpsc::channel();
        let expected = original.clone();
        app.launcher
            .request_with(captured, Arc::new(|| {}), move |path| {
                release_rx.recv().unwrap();
                assert_eq!(path, expected);
                Err(std::io::Error::from_raw_os_error(5))
            });
        app.navigation.selection.click(
            &selection::tests::rows(&["later-selection"]),
            0,
            false,
            false,
        );
        app.navigation
            .begin(PathBuf::from("later-location"), 9, Intent::New);
        app.navigation.apply(directory::Response {
            generation: 9,
            update: directory::Update::Finished(Ok(PathBuf::from("later-location"))),
        });
        let history = (
            app.navigation.current.location.clone(),
            app.navigation.back.clone(),
            app.navigation.forward.clone(),
        );
        let selection = app.navigation.selection.paths.clone();
        release_tx.send(()).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while app.launcher.busy_path().is_some() {
            assert!(Instant::now() < deadline);
            app.launcher.poll();
            std::thread::sleep(Duration::from_millis(1));
        }
        let report = app.launcher.report.as_ref().unwrap();
        assert_eq!(report.path, original);
        assert_eq!(report.result.as_ref().unwrap_err().raw_os_error(), Some(5));
        assert_eq!(
            history,
            (
                app.navigation.current.location.clone(),
                app.navigation.back.clone(),
                app.navigation.forward.clone()
            )
        );
        assert_eq!(app.navigation.selection.paths, selection);
        let context = egui::Context::default();
        let output = context.run_ui(egui::RawInput::default(), |ui| app.status(ui));
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text)
            if text.galley.job.text.contains("Could not launch") && text.galley.job.text.contains("os error 5")
                && text.galley.job.text.contains("fixture") && !text.galley.job.text.contains("later-location"))));
        output.drop_without_applying_deltas();
    }

    #[test]
    fn whole_row_metadata_and_gaps_receive_clicks_without_changing_virtual_pitch() {
        for x in [10.0, 203.0, 250.0, 410.0, 540.0, 695.0] {
            let context = egui::Context::default();
            let rows = selection::tests::rows(&["a", "b"]);
            let mut first = egui::Rect::NOTHING;
            let mut draw = |ui: &mut egui::Ui, clicked: bool| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
                let a = entry_row(
                    ui,
                    &rows[0],
                    [200.0, 150.0, 180.0, 146.0],
                    700.0,
                    true,
                    false,
                    false,
                    false,
                );
                first = a.rect;
                assert_eq!(a.clicked(), clicked);
                let b = entry_row(
                    ui,
                    &rows[1],
                    [200.0, 150.0, 180.0, 146.0],
                    700.0,
                    true,
                    false,
                    false,
                    false,
                );
                assert!((b.rect.top() - a.rect.top() - 27.0).abs() < 0.1);
            };
            context
                .run_ui(egui::RawInput::default(), |ui| draw(ui, false))
                .drop_without_applying_deltas();
            let pos = egui::pos2(first.left() + x, first.center().y);
            for pressed in [true, false] {
                context
                    .run_ui(
                        egui::RawInput {
                            events: vec![
                                egui::Event::PointerMoved(pos),
                                egui::Event::PointerButton {
                                    pos,
                                    button: egui::PointerButton::Primary,
                                    pressed,
                                    modifiers: egui::Modifiers::NONE,
                                },
                            ],
                            ..Default::default()
                        },
                        |ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
                            let a = entry_row(
                                ui,
                                &rows[0],
                                [200.0, 150.0, 180.0, 146.0],
                                700.0,
                                true,
                                false,
                                false,
                                false,
                            );
                            assert_eq!(a.clicked(), !pressed, "x={x}");
                        },
                    )
                    .drop_without_applying_deltas();
            }
        }
    }

    #[test]
    fn virtualized_table_keyboard_keeps_active_visible_and_pauses_pending_rows() {
        for size in [egui::vec2(480.0, 280.0), egui::vec2(1050.0, 700.0)] {
            let context = egui::Context::default();
            let repaint: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {});
            let mut app = Explorer {
                loader: DirectoryLoader::new(repaint.clone()).unwrap(),
                navigation: Navigation::new(PathBuf::from("root")),
                sorter: Sorter::new(repaint).unwrap(),
                launcher: Launcher::default(),
                requested_sort: None,
                location_text: "root".into(),
                location_editor_focused: false,
                location_error: None,
                font_warning: None,
                filter_text: String::new(),
                row_scroll_offset: 0.0,
                row_viewport_height: 0.0,
                scroll_to_active: false,
            };
            let names: Vec<_> = (0..200).map(|i| format!("row-{i}")).collect();
            app.navigation.current.entries =
                selection::tests::rows(&names.iter().map(String::as_str).collect::<Vec<_>>());
            app.navigation.sorted_key = Some(app.navigation.key());
            for key in [
                None,
                Some(egui::Key::End),
                Some(egui::Key::Home),
                Some(egui::Key::PageDown),
                Some(egui::Key::PageUp),
            ] {
                context
                    .run_ui(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                            events: key
                                .map(|key| vec![key_event(key, egui::Modifiers::NONE)])
                                .unwrap_or_default(),
                            ..Default::default()
                        },
                        |ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
                            assert_eq!(app.table(ui, false), None);
                        },
                    )
                    .drop_without_applying_deltas();
                if key.is_some() {
                    let index = app
                        .navigation
                        .selection
                        .active_index(&app.navigation.current.entries)
                        .unwrap();
                    let top = index as f32 * 27.0;
                    assert!(
                        app.row_scroll_offset <= top + 0.1,
                        "{size:?}: active top clipped"
                    );
                    assert!(
                        top + 24.0 <= app.row_scroll_offset + app.row_viewport_height + 0.1,
                        "{size:?}: active bottom clipped"
                    );
                }
            }
            let selected = app.navigation.selection.paths.clone();
            app.navigation
                .begin(PathBuf::from("pending"), 1, Intent::New);
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![key_event(egui::Key::A, egui::Modifiers::CTRL)],
                        ..Default::default()
                    },
                    |ui| {
                        assert_eq!(app.table(ui, false), None);
                    },
                )
                .drop_without_applying_deltas();
            assert_eq!(app.navigation.selection.paths, selected);
        }
    }

    fn filter_app() -> Explorer {
        let repaint: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {});
        let mut app = Explorer {
            loader: DirectoryLoader::new(repaint.clone()).unwrap(),
            navigation: Navigation::new(PathBuf::from("root")),
            sorter: Sorter::new(repaint).unwrap(),
            launcher: Launcher::default(),
            requested_sort: None,
            location_text: "root".into(),
            location_editor_focused: false,
            location_error: None,
            font_warning: None,
            filter_text: String::new(),
            row_scroll_offset: 0.0,
            row_viewport_height: 0.0,
            scroll_to_active: false,
        };
        let mut rows = selection::tests::rows(&["alpha", "beta", "alpine"]);
        for row in Arc::get_mut(&mut rows).unwrap() {
            let row = Arc::get_mut(row).unwrap();
            row.name = native::display(row.path.as_os_str());
        }
        app.navigation.current.raw_entries = rows;
        app.navigation.current.status = Status::Ready;
        accept_app_filter(&mut app);
        app
    }

    fn accept_app_filter(app: &mut Explorer) {
        assert!(app.navigation.apply_sorted(sorting::project_for_test(
            app.navigation.key(),
            app.navigation.visible().raw_entries.clone(),
            &app.navigation.filter
        )));
    }

    #[test]
    fn s006_visibility_control_updates_freshness_without_navigation_at_narrow_viewport() {
        let context = egui::Context::default();
        let mut app = filter_app();
        let location = app.navigation.current.location.clone();
        let before = app.navigation.key();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(480.0, 280.0));
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |ui| {
                app.toolbar(ui);
            },
        );
        let toggle = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == "Show hidden/system entries" => {
                    Some(text.pos + text.galley.size() / 2.0)
                }
                _ => None,
            })
            .expect("explicit visibility control");
        assert!(screen.contains(toggle));
        output.drop_without_applying_deltas();
        for pressed in [true, false] {
            context
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(screen),
                        events: vec![
                            egui::Event::PointerMoved(toggle),
                            egui::Event::PointerButton {
                                pos: toggle,
                                button: egui::PointerButton::Primary,
                                pressed,
                                modifiers: egui::Modifiers::NONE,
                            },
                        ],
                        ..Default::default()
                    },
                    |ui| {
                        app.toolbar(ui);
                    },
                )
                .drop_without_applying_deltas();
        }
        assert!(app.navigation.show_hidden_system);
        assert_ne!(app.navigation.key(), before);
        assert!(!app.navigation.rows_selectable());
        accept_app_filter(&mut app);
        assert!(app.navigation.rows_selectable());
        assert_eq!(app.navigation.current.location, location);
        assert!(app.navigation.back.is_empty() && app.navigation.forward.is_empty());
        let mut hidden = selection::tests::rows(&["hidden"]);
        Arc::get_mut(&mut Arc::get_mut(&mut hidden).unwrap()[0])
            .unwrap()
            .hidden = true;
        app.navigation.current.raw_entries = hidden;
        app.navigation.set_show_hidden_system(false);
        accept_app_filter(&mut app);
        let output = context.run_ui(egui::RawInput::default(), |ui| app.status(ui));
        let text = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            text.contains("All entries are hidden/system")
                && text.contains("0 / 1 entries displayed")
        );
        assert!(!text.contains("Empty folder"));
        output.drop_without_applying_deltas();
    }

    #[cfg(windows)]
    #[test]
    fn s006_path_submission_reports_ambiguous_drive_relative_without_navigation() {
        let context = egui::Context::default();
        let mut app = filter_app();
        app.location_text = "C:relative".into();
        let before = app.navigation.key();
        app.navigation
            .selection
            .select_all(&app.navigation.current.entries);
        let selected = app.navigation.selection.paths.clone();
        let output = context.run_ui(egui::RawInput::default(), |ui| {
            app.toolbar(ui);
        });
        let go = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == "Go" => {
                    Some(text.pos + text.galley.size() / 2.0)
                }
                _ => None,
            })
            .unwrap();
        output.drop_without_applying_deltas();
        for pressed in [true, false] {
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![
                            egui::Event::PointerMoved(go),
                            egui::Event::PointerButton {
                                pos: go,
                                button: egui::PointerButton::Primary,
                                pressed,
                                modifiers: egui::Modifiers::NONE,
                            },
                        ],
                        ..Default::default()
                    },
                    |ui| {
                        app.toolbar(ui);
                    },
                )
                .drop_without_applying_deltas();
        }
        assert!(
            app.location_error
                .as_ref()
                .unwrap()
                .contains("Drive-relative")
        );
        assert_eq!(app.navigation.key(), before);
        assert_eq!(app.navigation.selection.paths, selected);
        assert!(app.navigation.pending_location().is_none());
        let output = context.run_ui(egui::RawInput::default(), |ui| app.status(ui));
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("Edit the path and Go to retry"))));
        output.drop_without_applying_deltas();
        app.navigate(app.navigation.current.location.clone(), Intent::Refresh);
        assert!(app.location_error.is_none());
    }

    #[test]
    fn filter_toolbar_owns_editing_keys_and_submission_frame() {
        for (key, modifiers) in [
            (
                egui::Key::A,
                egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
            ),
            (egui::Key::ArrowDown, egui::Modifiers::NONE),
            (egui::Key::ArrowLeft, egui::Modifiers::NONE),
            (egui::Key::Enter, egui::Modifiers::NONE),
            (egui::Key::ArrowUp, egui::Modifiers::ALT),
            (egui::Key::F5, egui::Modifiers::NONE),
            (egui::Key::Escape, egui::Modifiers::NONE),
        ] {
            let context = egui::Context::default();
            let mut app = filter_app();
            app.filter_text = "al".into();
            app.navigation.set_filter("al".into());
            accept_app_filter(&mut app);
            app.navigation
                .selection
                .click(&app.navigation.current.entries, 0, false, false);
            let selected = app.navigation.selection.paths.clone();
            let key_before = app.navigation.key();
            context
                .run_ui(egui::RawInput::default(), |ui| {
                    app.toolbar(ui);
                    ui.memory_mut(|memory| memory.request_focus(egui::Id::new("filter")));
                })
                .drop_without_applying_deltas();
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![key_event(key, modifiers)],
                        ..Default::default()
                    },
                    |ui| {
                        let editing = app.toolbar(ui);
                        assert!(editing, "{key:?} must own its submission/exit frame");
                        assert_eq!(app.table(ui, editing), None);
                    },
                )
                .drop_without_applying_deltas();
            assert_eq!(
                app.navigation.key(),
                key_before,
                "editing must not refresh/navigate"
            );
            assert_eq!(app.navigation.selection.paths, selected);
            if key == egui::Key::A {
                context
                    .run_ui(
                        egui::RawInput {
                            events: vec![egui::Event::Text("be".into())],
                            ..Default::default()
                        },
                        |ui| {
                            assert!(app.toolbar(ui));
                        },
                    )
                    .drop_without_applying_deltas();
                assert_eq!(
                    app.filter_text, "be",
                    "Ctrl+A selected input text, not rows"
                );
                assert_eq!(app.navigation.filter, "be");
                assert!(!app.navigation.rows_selectable());
            }
        }
    }

    #[test]
    fn filter_table_commands_use_only_fresh_complete_visible_rows() {
        let context = egui::Context::default();
        let mut app = filter_app();
        app.navigation
            .selection
            .click(&app.navigation.current.entries, 2, false, false); // beta
        let selected = app.navigation.selection.paths.clone();
        app.navigation.set_filter("al".into());
        let run = |app: &mut Explorer, keys: Vec<egui::Event>| {
            let mut destination = None;
            context
                .run_ui(
                    egui::RawInput {
                        // Each call represents a new physical key press, not holding Enter.
                        events: keys
                            .into_iter()
                            .flat_map(|event| {
                                let mut release = event.clone();
                                if let egui::Event::Key { pressed, .. } = &mut release {
                                    *pressed = false;
                                }
                                [release, event]
                            })
                            .collect(),
                        ..Default::default()
                    },
                    |ui| {
                        destination = app.table(ui, false);
                    },
                )
                .drop_without_applying_deltas();
            destination
        };
        assert_eq!(active_activation(&app.navigation, false), None);
        assert_eq!(
            run(
                &mut app,
                vec![
                    key_event(egui::Key::A, egui::Modifiers::CTRL),
                    key_event(egui::Key::Enter, egui::Modifiers::NONE)
                ]
            ),
            None
        );
        assert_eq!(app.navigation.selection.paths, selected);
        accept_app_filter(&mut app);
        assert!(app.navigation.selection.paths.is_empty());
        run(
            &mut app,
            vec![key_event(egui::Key::A, egui::Modifiers::CTRL)],
        );
        assert_eq!(
            app.navigation.selection.paths,
            [PathBuf::from("alpha"), PathBuf::from("alpine")]
                .into_iter()
                .collect()
        );
        assert_eq!(
            run(
                &mut app,
                vec![key_event(egui::Key::Enter, egui::Modifiers::NONE)]
            ),
            Some(Activation::Folder(PathBuf::from("alpha")))
        );
        let selected = app.navigation.selection.paths.clone();
        app.navigation.current.status = Status::Loading;
        assert_eq!(active_activation(&app.navigation, false), None);
        assert_eq!(
            run(
                &mut app,
                vec![
                    key_event(egui::Key::End, egui::Modifiers::NONE),
                    key_event(egui::Key::A, egui::Modifiers::CTRL),
                    key_event(egui::Key::Enter, egui::Modifiers::NONE)
                ]
            ),
            None
        );
        assert_eq!(app.navigation.selection.paths, selected);
        app.navigation.current.status = Status::Ready;
        app.navigation.set_filter(String::new());
        accept_app_filter(&mut app);
        assert_eq!(app.navigation.selection.paths, selected);
        run(
            &mut app,
            vec![key_event(egui::Key::A, egui::Modifiers::CTRL)],
        );
        assert_eq!(app.navigation.selection.paths.len(), 3);
    }

    #[test]
    fn filter_clear_control_restores_rows_without_navigation_at_narrow_viewport() {
        let context = egui::Context::default();
        let mut app = filter_app();
        app.filter_text = "al".into();
        app.navigation.set_filter("al".into());
        accept_app_filter(&mut app);
        app.navigation
            .selection
            .select_all(&app.navigation.current.entries);
        let selected = app.navigation.selection.paths.clone();
        let location = app.navigation.current.location.clone();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(480.0, 280.0));
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |ui| {
                let editing = app.toolbar(ui);
                app.status(ui);
                app.table(ui, editing);
            },
        );
        let clear = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == "Clear" => {
                    Some(text.pos + text.galley.size() / 2.0)
                }
                _ => None,
            })
            .expect("clear control must be visible");
        assert!(screen.contains(clear));
        output.drop_without_applying_deltas();
        for pressed in [true, false] {
            context
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(screen),
                        events: vec![
                            egui::Event::PointerMoved(clear),
                            egui::Event::PointerButton {
                                pos: clear,
                                button: egui::PointerButton::Primary,
                                pressed,
                                modifiers: egui::Modifiers::NONE,
                            },
                        ],
                        ..Default::default()
                    },
                    |ui| {
                        let editing = app.toolbar(ui);
                        app.status(ui);
                        app.table(ui, editing);
                    },
                )
                .drop_without_applying_deltas();
        }
        assert!(app.filter_text.is_empty() && app.navigation.filter.is_empty());
        assert!(!app.navigation.rows_selectable());
        accept_app_filter(&mut app);
        assert_eq!(app.navigation.current.entries.len(), 3);
        assert_eq!(app.navigation.selection.paths, selected);
        assert_eq!(app.navigation.current.location, location);
        assert!(app.navigation.back.is_empty() && app.navigation.forward.is_empty());
    }

    #[test]
    fn filter_status_counts_and_no_match_are_never_from_an_old_projection() {
        let context = egui::Context::default();
        let mut app = filter_app();
        let status_text = |app: &Explorer| {
            let output = context.run_ui(egui::RawInput::default(), |ui| app.status(ui));
            let text = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            output.drop_without_applying_deltas();
            text
        };
        assert!(status_text(&app).contains("3 / 3 entries displayed"));
        app.navigation.set_filter("zz".into());
        let pending = status_text(&app);
        assert!(
            pending.contains("Updating filter/sort/visibility"),
            "pending text: {pending:?}"
        );
        assert!(!pending.contains("No names match") && !pending.contains("entries displayed"));
        accept_app_filter(&mut app);
        let fresh = status_text(&app);
        assert!(fresh.contains("0 / 3 entries displayed") && fresh.contains("No names match"));
        app.navigation.current.status = Status::Loading;
        assert!(!status_text(&app).contains("No names match"));
        app.navigation.current.status = Status::Ready;
        app.navigation.set_filter(String::new());
        accept_app_filter(&mut app);
        assert!(status_text(&app).contains("3 / 3 entries displayed"));
    }

    #[test]
    fn s006a_sort_direction_feedback_uses_builtin_ascii_glyphs() {
        let context = egui::Context::default();
        let mut app = filter_app();
        for expected in ["Name ^", "Name v"] {
            let output = context.run_ui(egui::RawInput::default(), |ui| {
                app.table(ui, false);
                assert!(ui.fonts_mut(|fonts| {
                    fonts.has_glyphs(&egui::FontId::proportional(14.0), expected)
                }));
            });
            assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text == expected)));
            output.drop_without_applying_deltas();
            app.navigation.sort.toggle(Column::Name);
        }
    }

    #[test]
    fn row_scroll_offset_keeps_first_last_and_page_targets_inside_viewport() {
        assert_eq!(row_scroll_offset(400.0, 270.0, 27.0, 0), 0.0);
        assert_eq!(row_scroll_offset(0.0, 270.0, 27.0, 9), 0.0);
        assert_eq!(row_scroll_offset(0.0, 270.0, 27.0, 10), 24.0);
        for index in [0, 1, 10, 19999] {
            let offset = row_scroll_offset(100.0, 270.0, 27.0, index);
            let top = index as f32 * 27.0;
            assert!(offset <= top && top + 24.0 <= offset + 270.0);
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
        self.launcher.poll();
        // The completion repaint can arrive just before the thread fully exits.
        if self.launcher.busy_path().is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
        let mut committed = false;
        let more = self.loader.drain(|response| {
            committed |= self.navigation.apply(response);
        });
        if committed && !self.location_editor_focused {
            self.location_text = native::display(self.navigation.current.location.as_os_str());
        }
        self.request_sort();
        if let Some(sorted) = self.sorter.take() {
            if self.navigation.apply_sorted(sorted) {
                self.scroll_to_active = true;
            }
        }
        if more {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let editing = self.toolbar(ui);
            ui.separator();
            self.status(ui);
            ui.separator();
            if let Some(activation) = self.table(ui, editing) {
                match activation {
                    Activation::Folder(location) => self.navigate(location, Intent::New),
                    Activation::File(path) => {
                        let context = ui.ctx().clone();
                        self.launcher
                            .request(path, Arc::new(move || context.request_repaint()));
                    }
                }
            }
            self.request_sort();
        });
    }
}
