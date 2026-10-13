//! Read-only typed Recycle Bin pane. Shell identities stay separate from paths.
use crate::platform::{
    bin::{self, BinItem},
    windows::OleGuard,
};
use eframe::egui;
use std::{
    collections::HashSet,
    sync::mpsc::{self, Receiver, TryRecvError},
    time::Duration,
};

pub struct BinView {
    items: Vec<BinItem>,
    selected: HashSet<Vec<u16>>,
    anchor: Option<usize>,
    cursor: Option<usize>,
    result: Option<Receiver<Result<Vec<BinItem>, String>>>,
    error: Option<String>,
}
impl Default for BinView {
    fn default() -> Self {
        Self::new()
    }
}
impl BinView {
    /// Starts one read-only listing. No actions or global-bin mutations occur.
    pub fn new() -> Self {
        let mut view = Self {
            items: Vec::new(),
            selected: HashSet::new(),
            anchor: None,
            cursor: None,
            result: None,
            error: None,
        };
        view.refresh();
        view
    }
    pub fn refresh(&mut self) {
        if self.result.is_some() {
            return;
        }
        self.error = None;
        // A refresh invalidates action eligibility until the new identity snapshot.
        self.selected.clear();
        self.anchor = None;
        self.cursor = None;
        let (tx, rx) = mpsc::sync_channel(1);
        self.result = Some(rx);
        if let Err(error) = std::thread::Builder::new()
            .name("wayfarer-bin-list".into())
            .spawn(move || {
                let result = (|| {
                    let _ole = OleGuard::initialize().map_err(|e| e.to_string())?;
                    bin::list()
                })();
                let _ = tx.send(result);
            })
        {
            self.result = None;
            self.error = Some(format!("Cannot start Recycle Bin listing: {error}"));
        }
    }
    pub fn poll(&mut self) {
        let Some(rx) = &self.result else {
            return;
        };
        let result = match rx.try_recv() {
            Ok(value) => value,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                Err("Recycle Bin listing worker disconnected".into())
            }
        };
        self.result = None;
        match result {
            Ok(items) => {
                self.items = items;
                self.selected.clear();
                self.anchor = None;
                self.cursor = None;
                self.error = None;
            }
            Err(error) => {
                self.error = Some(error);
                self.selected.clear();
            }
        }
    }
    pub fn is_loading(&self) -> bool {
        self.result.is_some()
    }
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
    pub fn items(&self) -> &[BinItem] {
        &self.items
    }
    pub fn selected_identities(&self) -> Vec<Vec<u16>> {
        if self.is_loading() || self.error.is_some() {
            return Vec::new();
        }
        self.items
            .iter()
            .filter(|row| self.selected.contains(&row.identity))
            .map(|row| row.identity.clone())
            .collect()
    }
    pub fn selected_items(&self) -> Vec<BinItem> {
        if self.is_loading() || self.error.is_some() {
            return Vec::new();
        }
        self.items
            .iter()
            .filter(|row| self.selected.contains(&row.identity))
            .cloned()
            .collect()
    }
    fn select(&mut self, index: usize, extend: bool, toggle: bool) {
        if index >= self.items.len() {
            return;
        }
        self.cursor = Some(index);
        if extend {
            let start = self.anchor.unwrap_or(index).min(self.items.len() - 1);
            if !toggle {
                self.selected.clear();
            }
            for row in &self.items[start.min(index)..=start.max(index)] {
                self.selected.insert(row.identity.clone());
            }
        } else if toggle {
            let identity = self.items[index].identity.clone();
            if !self.selected.remove(&identity) {
                self.selected.insert(identity);
            }
            self.anchor = Some(index);
        } else {
            self.selected.clear();
            self.selected.insert(self.items[index].identity.clone());
            self.anchor = Some(index);
        }
    }
    /// Virtualized read-only rows. Mutation commands belong to the caller and
    /// must use selected_identities after an up-to-date successful listing.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.poll();
        ui.horizontal(|ui| {
            ui.heading("Recycle Bin");
            if ui
                .add_enabled(!self.is_loading(), egui::Button::new("Refresh"))
                .clicked()
            {
                self.refresh();
            }
            ui.label(format!(
                "{} items · {} selected",
                self.items.len(),
                self.selected.len()
            ));
        });
        if self.is_loading() {
            ui.spinner();
            ui.label("Reading Recycle Bin…");
            ui.ctx().request_repaint_after(Duration::from_millis(100));
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
            ui.label("Refresh to retry. Previous rows are read-only and cannot be acted on.");
        }
        let selectable = !self.is_loading() && self.error.is_none();
        let row_height = ui.text_style_height(&egui::TextStyle::Body).max(18.0) + 8.0;
        egui::ScrollArea::vertical()
            .id_salt("recycle-bin-rows")
            .show_rows(ui, row_height, self.items.len(), |ui, range| {
                for index in range {
                    let row = &self.items[index];
                    let selected = self.selected.contains(&row.identity);
                    let response = ui.add_enabled(
                        selectable,
                        egui::Button::selectable(selected, &row.display_name),
                    );
                    response.clone().on_hover_ui(|ui| {
                        ui.label(format!(
                            "Original folder: {}",
                            row.original_path
                                .as_ref()
                                .map(|p| p.to_string_lossy().into_owned())
                                .unwrap_or_else(|| "Unavailable".into())
                        ));
                        ui.label(format!(
                            "Size: {}",
                            row.size
                                .map(|size| format!("{size} bytes"))
                                .unwrap_or_else(|| "Unavailable".into())
                        ));
                        ui.label(format!(
                            "Deleted: {}",
                            row.deleted.as_deref().unwrap_or("Unavailable")
                        ));
                    });
                    if response.clicked() {
                        let modifiers = ui.input(|input| input.modifiers);
                        self.select(index, modifiers.shift, modifiers.command);
                    }
                    if response.has_focus() && selectable {
                        let (up, down, all, shift) = ui.input(|input| {
                            (
                                input.key_pressed(egui::Key::ArrowUp),
                                input.key_pressed(egui::Key::ArrowDown),
                                input.modifiers.command && input.key_pressed(egui::Key::A),
                                input.modifiers.shift,
                            )
                        });
                        if all {
                            self.selected =
                                self.items.iter().map(|row| row.identity.clone()).collect();
                        }
                        if up || down {
                            let current = self.cursor.unwrap_or(index);
                            let next = if up {
                                current.saturating_sub(1)
                            } else {
                                (current + 1).min(self.items.len() - 1)
                            };
                            self.select(next, shift, false);
                        }
                    }
                }
            });
        if self.items.is_empty() && !self.is_loading() && self.error.is_none() {
            ui.label("Recycle Bin is empty.");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn view() -> BinView {
        BinView {
            items: (1..=3)
                .map(|id| BinItem {
                    identity: vec![id],
                    display_name: format!("Item {id}"),
                    original_path: None,
                    size: None,
                    deleted: None,
                })
                .collect(),
            selected: HashSet::new(),
            anchor: None,
            cursor: None,
            result: None,
            error: None,
        }
    }
    #[test]
    fn bin_listing_selection_uses_shell_identity_and_ranges() {
        let mut v = view();
        v.select(0, false, false);
        v.select(2, true, false);
        assert_eq!(v.selected_identities(), vec![vec![1], vec![2], vec![3]]);
        v.select(1, false, true);
        assert_eq!(v.selected_identities(), vec![vec![1], vec![3]]);
    }
    #[test]
    fn bin_listing_error_blocks_stale_action_identity() {
        let mut v = view();
        v.select(0, false, false);
        v.error = Some("stale".into());
        assert!(v.selected_identities().is_empty());
        assert!(v.selected_items().is_empty());
    }
}
