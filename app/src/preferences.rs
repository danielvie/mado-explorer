//! Independent settings policies: exact lossless path MRU, bounded appearance,
//! and single-chord hotkeys disabled while an editor owns keyboard focus.
use serde::{Deserialize, Serialize};
pub const RECENT_LIMIT: usize = 32;
pub const BOOKMARK_LIMIT: usize = 256;
pub const COMMANDS: &[&str] = &[
    "refresh",
    "new_tab",
    "close_tab",
    "up",
    "copy",
    "cut",
    "paste",
    "rename",
    "trash",
    "select_all",
    "new_folder",
    "palette",
];
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecentMode {
    Never,
    #[default]
    Session,
    Persistent,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    #[default]
    System,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClickMode {
    #[default]
    Double,
    SingleFolders,
    SingleAll,
}
impl ClickMode {
    pub fn activates(self, is_folder: bool, double_clicked: bool, editing: bool) -> bool {
        !editing
            && (double_clicked
                || self == Self::SingleAll
                || self == Self::SingleFolders && is_folder)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bookmark {
    pub name: String,
    pub group: String,
    pub path: Vec<u16>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub command: String,
    pub chord: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub bookmarks: Vec<Bookmark>,
    pub recent_mode: RecentMode,
    pub recents: Vec<Vec<u16>>,
    pub theme: Theme,
    pub font_size: f32,
    pub monospace: bool,
    pub density: f32,
    pub click_mode: ClickMode,
    pub checkboxes: bool,
    pub hotkeys: Vec<Binding>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            bookmarks: Vec::new(),
            recent_mode: RecentMode::Session,
            recents: Vec::new(),
            theme: Theme::System,
            font_size: 14.0,
            monospace: false,
            density: 1.0,
            click_mode: ClickMode::Double,
            checkboxes: false,
            hotkeys: default_hotkeys(),
        }
    }
}
fn default_hotkeys() -> Vec<Binding> {
    [
        ("refresh", "F5"),
        ("new_tab", "Ctrl+T"),
        ("close_tab", "Ctrl+W"),
        ("up", "Alt+ArrowUp"),
        ("copy", "Ctrl+C"),
        ("cut", "Ctrl+X"),
        ("paste", "Ctrl+V"),
        ("rename", "F2"),
        ("trash", "Delete"),
        ("select_all", "Ctrl+A"),
        ("new_folder", "Ctrl+Shift+N"),
        ("palette", "Ctrl+P"),
    ]
    .into_iter()
    .map(|(command, chord)| Binding {
        command: command.into(),
        chord: chord.into(),
    })
    .collect()
}
impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if !self.font_size.is_finite()
            || !(10.0..=32.0).contains(&self.font_size)
            || !self.density.is_finite()
            || !(0.75..=2.0).contains(&self.density)
        {
            return Err("Font size must be 10–32 and density 0.75–2".into());
        }
        if self.bookmarks.len() > BOOKMARK_LIMIT || self.recents.len() > RECENT_LIMIT {
            return Err("Settings exceed bookmark/recent limits".into());
        }
        for bookmark in &self.bookmarks {
            validate_bookmark(bookmark)?;
        }
        let mut paths = std::collections::HashSet::new();
        for path in &self.recents {
            validate_path(path)?;
            if !paths.insert(path) {
                return Err("Duplicate recent path".into());
            }
        }
        let mut commands = std::collections::HashSet::new();
        let mut chords = std::collections::HashSet::new();
        for binding in &self.hotkeys {
            if !COMMANDS.contains(&binding.command.as_str()) || !commands.insert(&binding.command) {
                return Err("Unknown or duplicate hotkey command".into());
            }
            let chord = canonical_chord(&binding.chord)?;
            if !chords.insert(chord) {
                return Err("Conflicting hotkey chord".into());
            }
        }
        Ok(())
    }
    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> Result<(), String> {
        validate_bookmark(&bookmark)?;
        if self.bookmarks.len() >= BOOKMARK_LIMIT {
            return Err("Bookmark limit reached".into());
        }
        self.bookmarks.push(bookmark);
        Ok(())
    }
    pub fn remove_bookmark(&mut self, index: usize) -> bool {
        if index >= self.bookmarks.len() {
            return false;
        }
        self.bookmarks.remove(index);
        true
    }
    pub fn reorder_bookmark(&mut self, from: usize, to: usize) -> bool {
        if from >= self.bookmarks.len() || to >= self.bookmarks.len() {
            return false;
        }
        let bookmark = self.bookmarks.remove(from);
        self.bookmarks.insert(to, bookmark);
        true
    }
    pub fn set_bookmark_group(&mut self, index: usize, group: String) -> Result<(), String> {
        if group.len() > 256 {
            return Err("Group label too long".into());
        }
        self.bookmarks
            .get_mut(index)
            .ok_or("Bookmark no longer exists")?
            .group = group;
        Ok(())
    }
    pub fn visit(&mut self, path: Vec<u16>) -> Result<(), String> {
        validate_path(&path)?;
        if self.recent_mode == RecentMode::Never {
            return Ok(());
        }
        self.recents.retain(|existing| *existing != path);
        self.recents.insert(0, path);
        self.recents.truncate(RECENT_LIMIT);
        Ok(())
    }
    pub fn clear_recents(&mut self) {
        self.recents.clear();
    }
    pub fn set_recent_mode(&mut self, mode: RecentMode) {
        self.recent_mode = mode;
        if mode == RecentMode::Never {
            self.clear_recents();
        }
    }
    pub fn persistent_clone(&self) -> Self {
        let mut saved = self.clone();
        if saved.recent_mode != RecentMode::Persistent {
            saved.clear_recents();
        }
        saved
    }
    pub fn set_appearance(&mut self, font_size: f32, density: f32) {
        self.font_size = if font_size.is_finite() {
            font_size.clamp(10.0, 32.0)
        } else {
            14.0
        };
        self.density = if density.is_finite() {
            density.clamp(0.75, 2.0)
        } else {
            1.0
        };
    }
    pub fn bind(&mut self, command: &str, chord: &str) -> Result<(), String> {
        let canonical = canonical_chord(chord)?;
        let mut candidate = self.clone();
        candidate
            .hotkeys
            .retain(|binding| binding.command != command);
        candidate.hotkeys.push(Binding {
            command: command.into(),
            chord: canonical,
        });
        candidate.validate()?;
        self.hotkeys = candidate.hotkeys;
        Ok(())
    }
    pub fn reset_hotkeys(&mut self) {
        self.hotkeys = default_hotkeys();
    }
    pub fn command_for(&self, chord: &str, text_focus: bool) -> Option<&str> {
        if text_focus {
            return None;
        }
        let canonical = canonical_chord(chord).ok()?;
        self.hotkeys
            .iter()
            .find(|binding| canonical_chord(&binding.chord).ok().as_ref() == Some(&canonical))
            .map(|binding| binding.command.as_str())
    }
}
fn validate_path(path: &[u16]) -> Result<(), String> {
    crate::session::decode_path(path).map(|_| ())
}
fn validate_bookmark(bookmark: &Bookmark) -> Result<(), String> {
    if bookmark.name.trim().is_empty() || bookmark.name.len() > 256 || bookmark.group.len() > 256 {
        return Err("Bookmark labels must be nonempty and at most 256 bytes".into());
    }
    validate_path(&bookmark.path)
}
/// Grammar: optional Ctrl/Alt/Shift (each once), then A–Z, 0–9, F1–F12,
/// Enter, Escape, Delete, Backspace, Tab, Space, Home, End, or an Arrow key.
pub fn canonical_chord(input: &str) -> Result<String, String> {
    if input.is_empty() || input.len() > 48 {
        return Err("Empty or oversized hotkey".into());
    }
    let parts: Vec<_> = input.split('+').collect();
    let mut modifiers = [false; 3];
    for modifier in &parts[..parts.len() - 1] {
        let index = match *modifier {
            "Ctrl" => 0,
            "Alt" => 1,
            "Shift" => 2,
            _ => return Err("Unknown hotkey modifier".into()),
        };
        if modifiers[index] {
            return Err("Duplicate hotkey modifier".into());
        }
        modifiers[index] = true;
    }
    let key = parts[parts.len() - 1];
    let single = key.len() == 1 && key.as_bytes()[0].is_ascii_uppercase()
        || key.len() == 1 && key.as_bytes()[0].is_ascii_digit();
    let function = key
        .strip_prefix('F')
        .and_then(|n| n.parse::<u8>().ok())
        .is_some_and(|n| (1..=12).contains(&n) && key == format!("F{n}"));
    if !single
        && !function
        && ![
            "Enter",
            "Escape",
            "Delete",
            "Backspace",
            "Tab",
            "Space",
            "Home",
            "End",
            "ArrowUp",
            "ArrowDown",
            "ArrowLeft",
            "ArrowRight",
        ]
        .contains(&key)
    {
        return Err("Unsupported hotkey key".into());
    }
    let mut result = String::new();
    for (enabled, name) in modifiers.into_iter().zip(["Ctrl", "Alt", "Shift"]) {
        if enabled {
            result.push_str(name);
            result.push('+');
        }
    }
    result.push_str(key);
    if matches!(result.as_str(), "F6" | "Alt+ArrowLeft" | "Alt+ArrowRight") {
        return Err("Shortcut reserved for pane focus or navigation history".into());
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn path(number: usize) -> Vec<u16> {
        format!("C:\\owned\\{number}").encode_utf16().collect()
    }
    #[test]
    fn bookmarks_lossless_roundtrip_and_reorder() {
        let mut settings = Settings::default();
        let mut unusual = path(1);
        unusual.push(0xd800);
        for (name, path) in [("a", unusual.clone()), ("b", path(2))] {
            settings
                .add_bookmark(Bookmark {
                    name: name.into(),
                    group: "Owned".into(),
                    path,
                })
                .unwrap();
        }
        assert!(settings.reorder_bookmark(0, 1));
        assert_eq!(settings.bookmarks[1].path, unusual);
        let decoded: Settings =
            serde_json::from_slice(&serde_json::to_vec(&settings).unwrap()).unwrap();
        assert_eq!(decoded, settings);
        decoded.validate().unwrap();
    }
    #[test]
    fn recents_bounds_privacy_and_independent_clear() {
        let mut settings = Settings::default();
        for n in 0..40 {
            settings.visit(path(n)).unwrap();
        }
        settings.visit(path(20)).unwrap();
        assert_eq!(settings.recents.len(), RECENT_LIMIT);
        assert_eq!(settings.recents[0], path(20));
        assert!(settings.persistent_clone().recents.is_empty());
        settings.set_recent_mode(RecentMode::Persistent);
        assert_eq!(settings.persistent_clone().recents.len(), RECENT_LIMIT);
        settings.set_recent_mode(RecentMode::Never);
        settings.visit(path(3)).unwrap();
        assert!(settings.recents.is_empty());
    }
    #[test]
    fn appearance_settings_bounds() {
        let mut settings = Settings::default();
        settings.set_appearance(f32::NAN, 20.0);
        assert_eq!(settings.font_size, 14.0);
        assert_eq!(settings.density, 2.0);
        settings.validate().unwrap();
    }
    #[test]
    fn input_settings_editor_precedence() {
        assert!(!ClickMode::SingleAll.activates(true, true, true));
        assert!(ClickMode::SingleFolders.activates(true, false, false));
        assert!(!ClickMode::SingleFolders.activates(false, false, false));
    }
    #[test]
    fn hotkeys_conflict_is_transactional_and_text_reserved() {
        let mut settings = Settings::default();
        let before = settings.clone();
        assert!(settings.bind("refresh", "Ctrl+C").is_err());
        assert_eq!(settings, before);
        assert!(settings.bind("unimplemented", "Ctrl+Q").is_err());
        assert!(canonical_chord("Ctrl+Ctrl+Q").is_err());
        assert!(canonical_chord("Ctrl+Mouse1").is_err());
        for chord in ["F6", "Alt+ArrowLeft", "Alt+ArrowRight"] {
            assert!(settings.bind("refresh", chord).is_err());
            assert_eq!(settings, before);
        }
        settings.bind("refresh", "Shift+Ctrl+R").unwrap();
        assert_eq!(settings.command_for("Ctrl+Shift+R", false), Some("refresh"));
        assert_eq!(settings.command_for("Ctrl+C", true), None);
    }
}
