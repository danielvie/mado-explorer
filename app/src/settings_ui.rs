//! Quick Access and preferences UI; command execution remains with the coordinator.
use crate::{
    preferences::{Bookmark, COMMANDS, ClickMode, RecentMode, Settings, Theme},
    session,
    ui::BrowserUi,
};
use eframe::egui;
use std::path::PathBuf;
#[derive(Default)]
pub struct SettingsUi {
    open: bool,
    name: String,
    group: String,
    filter: String,
    chord: String,
    command: String,
    last_path: Option<PathBuf>,
    message: String,
}
impl SettingsUi {
    pub fn show(&mut self, ui: &mut egui::Ui, browser: &mut BrowserUi) -> Vec<String> {
        let path = browser
            .active_tab()
            .map(|tab| tab.browser.current_path().to_path_buf());
        if self.last_path != path {
            if let Some(path) = &path
                && let Err(error) = browser.settings.visit(session::encode_path(path))
            {
                self.message = error;
            }
            self.last_path = path.clone();
        }
        let mut navigation: Option<(Vec<u16>, bool)> = None;
        ui.horizontal(|ui| {
            if ui.button("Preferences").clicked() {
                self.open = true;
            }
        });
        ui.collapsing("Quick Access", |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.filter).hint_text("Filter bookmarks and recents").char_limit(256));
            let filter = self.filter.trim().to_lowercase();
            ui.horizontal(|ui| {
                ui.label("Name"); ui.text_edit_singleline(&mut self.name);
                ui.label("Group"); ui.text_edit_singleline(&mut self.group);
                if ui.button("Bookmark current folder").clicked()
                    && let Some(path) = &path {
                        let name = if self.name.trim().is_empty() { path.file_name().unwrap_or(path.as_os_str()).to_string_lossy().into_owned() } else { self.name.clone() };
                        match browser.settings.add_bookmark(Bookmark { name, group: self.group.clone(), path: session::encode_path(path) }) { Ok(()) => self.message.clear(), Err(error) => self.message = error }
                    }
            });
            let mut groups = Vec::<String>::new();
            for bookmark in &browser.settings.bookmarks { if bookmark_matches(bookmark, &filter) && !groups.contains(&bookmark.group) { groups.push(bookmark.group.clone()); } }
            let mut remove = None; let mut movement = None;
            for group in groups {
                let heading = if group.is_empty() { "Ungrouped" } else { &group };
                egui::CollapsingHeader::new(heading).id_salt(("bookmark_group", &group)).default_open(true).show(ui, |ui| {
                    let length = browser.settings.bookmarks.len();
                    for (index, bookmark) in browser.settings.bookmarks.iter_mut().enumerate().filter(|(_, bookmark)| bookmark.group == group && bookmark_matches(bookmark, &filter)) {
                        ui.push_id(index, |ui| { ui.horizontal(|ui| {
                            if ui.button(&bookmark.name).clicked() { navigation = Some((bookmark.path.clone(), false)); }
                            if ui.small_button("New tab").clicked() { navigation = Some((bookmark.path.clone(), true)); }
                            if ui.small_button("Remove").clicked() { remove = Some(index); }
                            if ui.add_enabled(index > 0, egui::Button::new("↑")).clicked() { movement = Some((index, index - 1)); }
                            if ui.add_enabled(index + 1 < length, egui::Button::new("↓")).clicked() { movement = Some((index, index + 1)); }
                            ui.label("Group"); ui.add(egui::TextEdit::singleline(&mut bookmark.group).char_limit(64).desired_width(100.0));
                        }); });
                    }
                });
            }
            if let Some(index) = remove { browser.settings.remove_bookmark(index); }
            else if let Some((from, to)) = movement { browser.settings.reorder_bookmark(from, to); }
            ui.separator();
            let mut mode = browser.settings.recent_mode;
            ui.horizontal(|ui| {
                ui.label("Recents");
                for (value, label) in [(RecentMode::Never, "Never"), (RecentMode::Session, "Session"), (RecentMode::Persistent, "Persistent")] { ui.selectable_value(&mut mode, value, label); }
                if ui.button("Clear recents").clicked() { browser.settings.clear_recents(); }
            });
            if mode != browser.settings.recent_mode { browser.settings.set_recent_mode(mode); }
            for recent in &browser.settings.recents {
                if let Ok(path) = session::decode_path(recent) {
                    if !path.to_string_lossy().to_lowercase().contains(&filter) { continue; }
                    ui.horizontal(|ui| {
                        if ui.button(path.to_string_lossy()).clicked() { navigation = Some((recent.clone(), false)); }
                        if ui.small_button("New tab").clicked() { navigation = Some((recent.clone(), true)); }
                    });
                }
            }
            ui.label("Recents: newest first, exact path deduplication, maximum 32. Unavailable locations remain listed.");
        });
        if let Some((units, new_tab)) = navigation {
            match session::decode_path(&units) {
                Ok(path) => {
                    if new_tab {
                        browser.add_tab(path);
                    } else if let Some(tab) = browser.active_any_tab() {
                        tab.navigate(path);
                    }
                }
                Err(error) => self.message = error,
            }
        }
        egui::Window::new("Preferences")
            .open(&mut self.open)
            .show(ui.ctx(), |ui| {
                let settings = &mut browser.settings;
                ui.horizontal(|ui| {
                    ui.label("Theme");
                    for (value, label) in [
                        (Theme::System, "System"),
                        (Theme::Dark, "Dark"),
                        (Theme::Light, "Light"),
                    ] {
                        ui.selectable_value(&mut settings.theme, value, label);
                    }
                });
                ui.checkbox(&mut settings.monospace, "Monospace font");
                ui.add(egui::Slider::new(&mut settings.font_size, 10.0..=32.0).text("Font size"));
                ui.add(egui::Slider::new(&mut settings.density, 0.75..=2.0).text("Density"));
                ui.label("Bundled application font; sizes and density use independent new limits.");
                ui.horizontal(|ui| {
                    ui.label("Open with");
                    for (value, label) in [
                        (ClickMode::Double, "Double click"),
                        (ClickMode::SingleFolders, "Single click folders"),
                        (ClickMode::SingleAll, "Single click all"),
                    ] {
                        ui.selectable_value(&mut settings.click_mode, value, label);
                    }
                });
                ui.checkbox(&mut settings.checkboxes, "Selection checkboxes");
                ui.separator();
                ui.label("Command hotkeys (inactive while editing text)");
                for binding in &settings.hotkeys {
                    ui.label(format!("{}: {}", binding.command, binding.chord));
                }
                if self.command.is_empty() {
                    self.command = "refresh".into();
                }
                egui::ComboBox::from_id_salt("hotkey-command")
                    .selected_text(&self.command)
                    .show_ui(ui, |ui| {
                        for &command in COMMANDS {
                            ui.selectable_value(&mut self.command, command.into(), command);
                        }
                    });
                ui.text_edit_singleline(&mut self.chord);
                ui.label("Example: Ctrl+Shift+R. Single chords only. F6 and Alt+Left/Right are reserved.");
                if ui.button("Bind").clicked() {
                    match settings.bind(&self.command, &self.chord) {
                        Ok(()) => self.message = "Binding saved".into(),
                        Err(error) => self.message = error,
                    }
                }
                if ui.button("Reset hotkeys").clicked() {
                    settings.reset_hotkeys();
                }
            });
        apply_style(ui.ctx(), &browser.settings);
        if !self.message.is_empty() {
            ui.label(&self.message);
        }
        let mut commands = Vec::new();
        if !ui.ctx().egui_wants_keyboard_input() {
            ui.input(|input| {
                for event in &input.events {
                    if let egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        modifiers,
                        ..
                    } = event
                    {
                        let mut chord = String::new();
                        if modifiers.ctrl {
                            chord.push_str("Ctrl+");
                        }
                        if modifiers.alt {
                            chord.push_str("Alt+");
                        }
                        if modifiers.shift {
                            chord.push_str("Shift+");
                        }
                        chord.push_str(match key {
                            egui::Key::ArrowUp => "ArrowUp",
                            egui::Key::ArrowDown => "ArrowDown",
                            egui::Key::ArrowLeft => "ArrowLeft",
                            egui::Key::ArrowRight => "ArrowRight",
                            _ => key.name(),
                        });
                        if let Some(command) = browser.settings.command_for(&chord, false) {
                            commands.push(command.to_owned());
                        }
                    }
                }
            });
        }
        commands
    }
}
fn apply_style(ctx: &egui::Context, settings: &Settings) {
    ctx.set_theme(match settings.theme {
        Theme::Dark => egui::ThemePreference::Dark,
        Theme::Light => egui::ThemePreference::Light,
        Theme::System => egui::ThemePreference::System,
    });
    ctx.all_styles_mut(|style| {
        for (kind, font) in &mut style.text_styles {
            font.family = if settings.monospace {
                egui::FontFamily::Monospace
            } else {
                egui::FontFamily::Proportional
            };
            font.size = match kind {
                egui::TextStyle::Heading => settings.font_size * 1.4,
                egui::TextStyle::Small => settings.font_size * 0.85,
                _ => settings.font_size,
            };
        }
        style.spacing.item_spacing = egui::vec2(8.0, 3.0) * settings.density;
        style.spacing.interact_size.y = (settings.font_size + 6.0) * settings.density;
    });
}

fn bookmark_matches(bookmark: &Bookmark, filter: &str) -> bool {
    bookmark.name.to_lowercase().contains(filter)
        || bookmark.group.to_lowercase().contains(filter)
        || String::from_utf16_lossy(&bookmark.path)
            .to_lowercase()
            .contains(filter)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bookmarks_filter_matches_labels_groups_and_paths() {
        let bookmark = Bookmark {
            name: "Work".into(),
            group: "Projects".into(),
            path: "C:\\Owned".encode_utf16().collect(),
        };
        assert!(bookmark_matches(&bookmark, "work"));
        assert!(bookmark_matches(&bookmark, "projects"));
        assert!(bookmark_matches(&bookmark, "owned"));
        assert!(!bookmark_matches(&bookmark, "missing"));
    }
}
