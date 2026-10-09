//! Read-only selection summaries. All metadata comes from the caller's fresh listing.

use std::{ffi::OsStr, path::Path};

/// Caller-owned freshness tag. Never reuse a revision within a context.
/// Change context on active-tab/activation changes; advance revision on selection,
/// listing, projection, or loading/error changes, even when returning to old values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestTag {
    pub context: u64,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    Folder,
    RegularFile,
    Link,
    Other,
    Unknown,
}

/// Immutable native identity and already-loaded metadata for one visible selection.
/// Display strings are never returned as command paths. Size is honored only for
/// regular files; folder/link sizes never imply recursive or target information.
#[derive(Clone, Copy, Debug)]
pub struct SelectedEntry<'a> {
    pub name: &'a OsStr,
    pub path: &'a Path,
    pub kind: EntryKind,
    pub size_bytes: Option<u64>,
    pub modified_utc: Option<&'a str>,
    /// May describe partially unavailable metadata, such as only the modified time.
    pub metadata_error: Option<&'a str>,
}

#[derive(Clone, Copy, Debug)]
pub enum SnapshotState<'a> {
    Loading,
    MetadataError(&'a str),
    /// Exactly the distinct visible selections, not retained hidden selection paths.
    Ready(&'a [SelectedEntry<'a>]),
}

#[derive(Clone, Copy, Debug)]
pub struct SelectionSnapshot<'a> {
    pub tag: RequestTag,
    pub state: SnapshotState<'a>,
}

/// One instance per pane, with a unique stable pane ID in the egui context.
/// Holds one details toggle and no metadata, paths, asynchronous work, or cache.
pub struct SelectionSummary {
    pane_id: u64,
    details_open: bool,
}

impl SelectionSummary {
    pub fn new(pane_id: u64) -> Self {
        Self {
            pane_id,
            details_open: false,
        }
    }

    /// Render into the pane's preview region. Mismatched tags reveal no metadata.
    /// Returns the single-entry details button, when present, for focus routing.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        active: RequestTag,
        snapshot: Option<&SelectionSnapshot<'_>>,
    ) -> Option<egui::Response> {
        ui.push_id(("selection-summary", self.pane_id), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("summary-scroll")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.heading("Selection summary");
                    let Some(snapshot) = snapshot else {
                        ui.label("Waiting for selection metadata.");
                        return None;
                    };
                    if snapshot.tag != active {
                        ui.label("Stale selection metadata. Waiting for the current selection.");
                        return None;
                    }
                    match snapshot.state {
                        SnapshotState::Loading => {
                            ui.label("Loading selection metadata.");
                            None
                        }
                        SnapshotState::MetadataError(error) => {
                            ui.label("Selection metadata unavailable.");
                            ui.add(egui::Label::new(error).wrap());
                            None
                        }
                        SnapshotState::Ready([]) => {
                            ui.label("No selection.");
                            None
                        }
                        SnapshotState::Ready([entry]) => Some(self.single(ui, entry)),
                        SnapshotState::Ready(entries) => {
                            multiple(ui, entries);
                            None
                        }
                    }
                })
                .inner
        })
        .inner
    }

    fn single(&mut self, ui: &mut egui::Ui, entry: &SelectedEntry<'_>) -> egui::Response {
        // Lossy text is display-only. Native references in the DTO remain unchanged.
        ui.add(egui::Label::new(entry.name.to_string_lossy()).wrap());
        ui.label(match entry.kind {
            EntryKind::Folder => "Type: Folder",
            EntryKind::RegularFile => "Type: Regular file",
            EntryKind::Link => "Type: Link / reparse point",
            EntryKind::Other => "Type: Other",
            EntryKind::Unknown => "Type: Unknown",
        });
        match (entry.kind, entry.size_bytes) {
            (EntryKind::RegularFile, Some(size)) => {
                ui.label(format!("Size: {size} bytes"));
            }
            (EntryKind::Folder, _) => {
                ui.label("Size: Unknown. Folder contents have not been scanned.");
            }
            _ => {
                ui.label("Size: Unknown.");
            }
        }
        if entry.kind != EntryKind::Folder {
            ui.label("Content preview unavailable. Basic information only.");
        }
        if let Some(error) = entry.metadata_error {
            ui.label("Metadata error:");
            ui.add(egui::Label::new(error).wrap());
        }
        let response = ui
            .push_id("details-toggle", |ui| {
                ui.add(egui::Button::new("Basic details").selected(self.details_open))
            })
            .inner;
        if response.clicked() {
            self.details_open = !self.details_open;
        }
        if response.has_focus() {
            ui.painter().rect_stroke(
                response.rect.expand(1.0),
                0.0,
                egui::Stroke::new(2.0, ui.visuals().text_color()),
                egui::StrokeKind::Outside,
            );
        }
        if self.details_open {
            ui.label("Path:");
            ui.add(egui::Label::new(entry.path.to_string_lossy()).wrap());
            ui.label(format!(
                "Modified (UTC): {}",
                entry.modified_utc.unwrap_or("Unknown")
            ));
        }
        response
    }
}

fn multiple(ui: &mut egui::Ui, entries: &[SelectedEntry<'_>]) {
    let mut known_bytes = 0_u128;
    let mut known_files = 0_usize;
    let mut errors = 0_usize;
    // ponytail: O(selected entries), constant retained state and aggregate-only UI.
    // If large-selection measurements require it, preaggregate on the listing worker.
    for entry in entries {
        if let (EntryKind::RegularFile, Some(size)) = (entry.kind, entry.size_bytes) {
            known_bytes += u128::from(size);
            known_files += 1;
        }
        errors += usize::from(entry.metadata_error.is_some());
    }
    ui.label(format!("Selected visible entries: {}", entries.len()));
    ui.label(format!("Known regular-file bytes: {known_bytes}"));
    ui.label(format!("Regular files with known size: {known_files}"));
    ui.label(format!(
        "Entries with unknown size: {}",
        entries.len() - known_files
    ));
    ui.label("Folder contents and link targets have not been scanned.");
    if errors > 0 {
        ui.label(format!("Metadata errors: {errors} entries"));
    }
}
