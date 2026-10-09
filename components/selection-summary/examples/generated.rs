//! Safe headless demonstration. Names and metadata are generated, never read from disk.
use mado_selection_summary::{
    EntryKind, RequestTag, SelectedEntry, SelectionSnapshot, SelectionSummary, SnapshotState,
};
use std::{ffi::OsStr, path::Path};

fn main() {
    let ctx = egui::Context::default();
    let mut summary = SelectionSummary::new(1);
    let tag = RequestTag {
        context: 1,
        revision: 1,
    };
    let entries = [
        SelectedEntry {
            name: OsStr::new("generated.bin"),
            path: Path::new("generated/generated.bin"),
            kind: EntryKind::RegularFile,
            size_bytes: Some(128),
            modified_utc: Some("2026-01-02 03:04:05"),
            metadata_error: None,
        },
        SelectedEntry {
            name: OsStr::new("generated folder"),
            path: Path::new("generated/folder"),
            kind: EntryKind::Folder,
            size_bytes: None,
            modified_utc: None,
            metadata_error: None,
        },
    ];
    for (revision, state) in [
        SnapshotState::Loading,
        SnapshotState::Ready(&[]),
        SnapshotState::Ready(&entries),
        SnapshotState::Ready(&entries[1..]),
        SnapshotState::Ready(&entries[..1]),
        SnapshotState::MetadataError("Generated unavailable metadata"),
    ]
    .into_iter()
    .enumerate()
    {
        let tag = RequestTag {
            revision: revision as u64 + 1,
            ..tag
        };
        let snapshot = SelectionSnapshot { tag, state };
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            summary.show(ui, tag, Some(&snapshot));
        });
        // No GPU in this demo; discard only egui's font-upload commands.
        output.textures_delta.clear();
        println!("--- Generated egui frame ---");
        for shape in output.shapes {
            if let egui::Shape::Text(text) = shape.shape {
                println!("{}", text.galley.job.text);
            }
        }
    }
    let obsolete = SelectionSnapshot {
        tag,
        state: SnapshotState::Ready(&entries),
    };
    let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
        summary.show(ui, RequestTag { revision: 7, ..tag }, Some(&obsolete));
    });
    output.textures_delta.clear();
    println!("--- Stale generated egui frame ---");
    for shape in output.shapes {
        if let egui::Shape::Text(text) = shape.shape {
            println!("{}", text.galley.job.text);
        }
    }
}
