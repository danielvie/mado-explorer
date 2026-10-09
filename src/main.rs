mod directory;
mod navigation;
mod selection;
mod sorting;

use directory::{DirectoryLoader, EntryKind, Status};
use eframe::egui;
use navigation::{Intent, Navigation};
use selection::Movement;
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
    row_scroll_offset: f32,
    row_viewport_height: f32,
    scroll_to_active: bool,
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
            row_scroll_offset: 0.0,
            row_viewport_height: 0.0,
            scroll_to_active: false,
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
        editing
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

    fn table(&mut self, ui: &mut egui::Ui, editing: bool) -> Option<PathBuf> {
        let mut destination = None;
        let selectable = self.navigation.pending_location().is_none();
        let table_focus = ui.add(
            egui::Label::new(format!(
                "Entries — {} selected{}",
                self.navigation.selection.paths.len(),
                if selectable {
                    ""
                } else {
                    " (selection paused until destination opens)"
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
                        if self.navigation.sort.descending { " ▼" } else { " ▲" }
                    } else { "" };
                    if table_cell(ui, width, egui::Button::new(format!("{heading}{arrow}")).frame(false).truncate())
                        .on_hover_text("Toggle sort direction. Folders first; unknown values last in each group.").clicked() {
                        self.navigation.sort.toggle(column);
                    }
                }
            });
            ui.separator();
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
                        SelectionShortcut::Open => { destination = active_folder(&self.navigation); }
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
                        if response.clicked() {
                            let modifiers = ui.input(|input| input.modifiers);
                            if !modifiers.alt && !modifiers.mac_cmd {
                                self.navigation.selection.click(&entries, index, modifiers.ctrl, modifiers.shift);
                                table_focus.request_focus();
                            }
                        }
                        if response.double_clicked() && entry.kind == EntryKind::Folder { destination = Some(entry.path.clone()); }
                        response.on_hover_ui(|ui| {
                            ui.label(entry.path.to_string_lossy());
                            if entry.kind == EntryKind::Folder { ui.label("Double-click or select the active row and press Enter to open folder."); }
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
                        (*key == egui::Key::Enter && !modifiers.shift && enter_allowed)
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

fn active_folder(navigation: &Navigation) -> Option<PathBuf> {
    if navigation.pending_location().is_some() {
        return None;
    }
    let index = navigation
        .selection
        .active_index(&navigation.current.entries)?;
    let entry = &navigation.current.entries[index];
    (entry.kind == EntryKind::Folder).then(|| entry.path.clone())
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
                    assert_eq!(ui.input(|input| input.events.len()), 4);
                    assert_eq!(navigation_shortcut(ui.ctx(), false), Some(Shortcut::Up));
                    assert_eq!(ui.input(|input| input.events.len()), 3);
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
    fn active_folder_requires_a_current_displayed_native_folder() {
        let mut navigation = Navigation::new(PathBuf::from("root"));
        let rows = selection::tests::rows(&["root/a", "root/b"]);
        navigation.current.entries = rows.clone();
        assert_eq!(active_folder(&navigation), None);
        navigation.selection.click(&rows, 1, false, false);
        assert_eq!(active_folder(&navigation), Some(PathBuf::from("root/b")));
        navigation.current.entries = Arc::new(vec![rows[0].clone()]);
        assert_eq!(active_folder(&navigation), None); // missing active row is not row zero
        navigation.current.entries = rows.clone();
        navigation.begin(PathBuf::from("destination"), 1, Intent::New);
        assert_eq!(active_folder(&navigation), None);
        let mut file = rows[1].as_ref().path.clone();
        file.push("file");
        let file = Arc::new(directory::Entry {
            path: file,
            name: "file".into(),
            kind: EntryKind::File,
            modified: "—".into(),
            modified_time: None,
            size: Some(0),
            metadata_error: None,
        });
        let mut navigation = Navigation::new(PathBuf::from("root"));
        navigation.current.entries = Arc::new(vec![file]);
        navigation
            .selection
            .click(&navigation.current.entries, 0, false, false);
        assert_eq!(active_folder(&navigation), None);
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
                requested_sort: None,
                location_text: "root".into(),
                location_editor_focused: false,
                row_scroll_offset: 0.0,
                row_viewport_height: 0.0,
                scroll_to_active: false,
            };
            let names: Vec<_> = (0..200).map(|i| format!("row-{i}")).collect();
            app.navigation.current.entries =
                selection::tests::rows(&names.iter().map(String::as_str).collect::<Vec<_>>());
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
            if let Some(location) = self.table(ui, editing) {
                self.navigate(location, Intent::New);
            }
            self.request_sort();
        });
    }
}
