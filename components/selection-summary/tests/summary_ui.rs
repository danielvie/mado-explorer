use egui::{Context, Event, FullOutput, Modifiers, PointerButton, RawInput, Response, Shape};
use mado_selection_summary::{
    EntryKind, RequestTag, SelectedEntry, SelectionSnapshot, SelectionSummary, SnapshotState,
};
use std::{ffi::OsStr, path::Path};

const TAG: RequestTag = RequestTag {
    context: 10,
    revision: 1,
};

fn entry(name: &'static str, kind: EntryKind, size_bytes: Option<u64>) -> SelectedEntry<'static> {
    SelectedEntry {
        name: OsStr::new(name),
        path: Path::new(name),
        kind,
        size_bytes,
        modified_utc: Some("2026-01-02 03:04:05"),
        metadata_error: None,
    }
}

fn rendered_text(output: &FullOutput) -> Vec<String> {
    fn collect(shape: &Shape, text: &mut Vec<String>) {
        match shape {
            Shape::Text(shape) => text.push(shape.galley.job.text.clone()),
            Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let mut text = Vec::new();
    for shape in &output.shapes {
        collect(&shape.shape, &mut text);
    }
    text
}

fn frame(
    ctx: &Context,
    panel: &mut SelectionSummary,
    active: RequestTag,
    snapshot: Option<&SelectionSnapshot<'_>>,
    events: Vec<Event>,
) -> (FullOutput, Option<Response>) {
    let mut response = None;
    let mut output = ctx.run_ui(
        RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(500.0, 650.0),
            )),
            events,
            ..Default::default()
        },
        |ui| response = panel.show(ui, active, snapshot),
    );
    // Headless renderer consumes shapes only, intentionally discards font uploads.
    output.textures_delta.clear();
    (output, response)
}

fn contains(output: &FullOutput, expected: &str) {
    let text = rendered_text(output);
    assert!(
        text.iter().any(|text| text == expected),
        "{expected:?} missing from {text:?}"
    );
}

#[test]
fn p06_selection_changes_render_empty_multiple_and_single_fallback() {
    let ctx = Context::default();
    let mut panel = SelectionSummary::new(1);
    let empty = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&[]),
    };
    contains(
        &frame(&ctx, &mut panel, TAG, Some(&empty), vec![]).0,
        "No selection.",
    );
    let entries = [
        entry("one.bin", EntryKind::RegularFile, Some(12)),
        entry("two.bin", EntryKind::RegularFile, Some(30)),
        entry("folder", EntryKind::Folder, Some(999)),
        entry("missing-size", EntryKind::RegularFile, None),
        entry("link", EntryKind::Link, Some(999)),
    ];
    let multiple = SelectionSnapshot {
        tag: RequestTag { revision: 2, ..TAG },
        state: SnapshotState::Ready(&entries),
    };
    let (output, response) = frame(&ctx, &mut panel, multiple.tag, Some(&multiple), vec![]);
    contains(&output, "Selected visible entries: 5");
    contains(&output, "Known regular-file bytes: 42");
    contains(&output, "Regular files with known size: 2");
    contains(&output, "Entries with unknown size: 3");
    assert!(response.is_none());
    assert!(
        !rendered_text(&output)
            .iter()
            .any(|text| text.contains("one.bin"))
    );
    let single = SelectionSnapshot {
        tag: RequestTag { revision: 3, ..TAG },
        state: SnapshotState::Ready(&entries[..1]),
    };
    let (output, response) = frame(&ctx, &mut panel, single.tag, Some(&single), vec![]);
    contains(&output, "one.bin");
    contains(&output, "Size: 12 bytes");
    contains(
        &output,
        "Content preview unavailable. Basic information only.",
    );
    assert!(response.is_some());
    assert!(
        !rendered_text(&output)
            .iter()
            .any(|text| text.starts_with("Selected visible entries:"))
    );
    let empty = SelectionSnapshot {
        tag: RequestTag { revision: 4, ..TAG },
        state: SnapshotState::Ready(&[]),
    };
    contains(
        &frame(&ctx, &mut panel, empty.tag, Some(&empty), vec![]).0,
        "No selection.",
    );
}

#[test]
fn p06_folder_size_is_unknown_even_if_caller_supplies_a_size() {
    let ctx = Context::default();
    let mut panel = SelectionSummary::new(1);
    let entries = [entry("folder", EntryKind::Folder, Some(0))];
    let snapshot = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let (output, response) = frame(&ctx, &mut panel, TAG, Some(&snapshot), vec![]);
    contains(&output, "Type: Folder");
    contains(
        &output,
        "Size: Unknown. Folder contents have not been scanned.",
    );
    assert!(
        !rendered_text(&output)
            .iter()
            .any(|text| text == "Size: 0 bytes")
    );
    let pos = response.unwrap().rect.center();
    for pressed in [true, false] {
        let (output, _) = frame(
            &ctx,
            &mut panel,
            TAG,
            Some(&snapshot),
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
        if !pressed {
            contains(&output, "Path:");
            contains(&output, "Modified (UTC): 2026-01-02 03:04:05");
        }
    }
}

#[test]
fn p06_unavailable_and_partial_metadata_remain_explicit() {
    let ctx = Context::default();
    let mut panel = SelectionSummary::new(1);
    let failed = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::MetadataError("Listing access denied"),
    };
    let (output, response) = frame(&ctx, &mut panel, TAG, Some(&failed), vec![]);
    contains(&output, "Selection metadata unavailable.");
    contains(&output, "Listing access denied");
    assert!(response.is_none());
    let mut entries = [entry("missing", EntryKind::Unknown, None)];
    entries[0].modified_utc = None;
    entries[0].metadata_error = Some("Read metadata: file disappeared");
    let snapshot = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let (output, response) = frame(&ctx, &mut panel, TAG, Some(&snapshot), vec![]);
    contains(&output, "Type: Unknown");
    contains(&output, "Size: Unknown.");
    contains(&output, "Read metadata: file disappeared");
    response.unwrap().request_focus();
    let (output, _) = frame(
        &ctx,
        &mut panel,
        TAG,
        Some(&snapshot),
        vec![Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
    );
    contains(&output, "Modified (UTC): Unknown");
    let entries = [
        SelectedEntry {
            metadata_error: Some("Modified time unavailable"),
            ..entry("known", EntryKind::RegularFile, Some(7))
        },
        entries[0],
    ];
    let snapshot = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let (output, _) = frame(&ctx, &mut panel, TAG, Some(&snapshot), vec![]);
    contains(&output, "Known regular-file bytes: 7");
    contains(&output, "Entries with unknown size: 1");
    contains(&output, "Metadata errors: 2 entries");
}

#[test]
fn p06_stale_revisions_and_contexts_never_render_old_metadata() {
    let ctx = Context::default();
    let mut panel = SelectionSummary::new(1);
    contains(
        &frame(&ctx, &mut panel, TAG, None, vec![]).0,
        "Waiting for selection metadata.",
    );
    let loading = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Loading,
    };
    contains(
        &frame(&ctx, &mut panel, TAG, Some(&loading), vec![]).0,
        "Loading selection metadata.",
    );
    let entries = [entry("obsolete.bin", EntryKind::RegularFile, Some(123))];
    let old = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let (_, response) = frame(&ctx, &mut panel, TAG, Some(&old), vec![]);
    response.unwrap().request_focus();
    for active in [
        RequestTag { revision: 2, ..TAG },
        RequestTag { context: 11, ..TAG },
    ] {
        let (output, response) = frame(
            &ctx,
            &mut panel,
            active,
            Some(&old),
            vec![Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
            }],
        );
        contains(
            &output,
            "Stale selection metadata. Waiting for the current selection.",
        );
        assert!(response.is_none());
        assert!(
            !rendered_text(&output)
                .iter()
                .any(|text| text.contains("obsolete") || text.contains("123"))
        );
    }
    let current = SelectionSnapshot {
        tag: RequestTag { revision: 3, ..TAG },
        state: SnapshotState::Ready(&[]),
    };
    contains(
        &frame(&ctx, &mut panel, current.tag, Some(&current), vec![]).0,
        "No selection.",
    );
    let late_error = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::MetadataError("obsolete error"),
    };
    let (output, _) = frame(&ctx, &mut panel, current.tag, Some(&late_error), vec![]);
    assert!(
        !rendered_text(&output)
            .iter()
            .any(|text| text.contains("obsolete error"))
    );
}

#[test]
fn p06_independent_panes_and_keyboard_focus_control_actual_details() {
    let ctx = Context::default();
    let mut left = SelectionSummary::new(100);
    let mut right = SelectionSummary::new(200);
    let entries = [entry("fallback.bin", EntryKind::RegularFile, Some(4))];
    let snapshot = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let mut responses = Vec::new();
    let mut render = |events: Vec<Event>| {
        responses.clear();
        let mut output = ctx.run_ui(
            RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 600.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                ui.columns(2, |columns| {
                    responses.push(left.show(&mut columns[0], TAG, Some(&snapshot)).unwrap());
                    responses.push(right.show(&mut columns[1], TAG, Some(&snapshot)).unwrap());
                });
            },
        );
        output.textures_delta.clear();
        (output, responses.clone())
    };
    let (output, buttons) = render(vec![]);
    assert_ne!(buttons[0].id, buttons[1].id);
    assert!(!rendered_text(&output).iter().any(|text| text == "Path:"));
    // Start with no focus: actual Tab traversal must reach the left details button.
    render(vec![Event::Key {
        key: egui::Key::Tab,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);
    let (output, focused) = render(vec![]);
    assert!(focused[0].has_focus());
    assert!(!focused[1].has_focus());
    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape,
        Shape::Rect(rect) if rect.stroke.width == 2.0 && rect.rect == focused[0].rect.expand(1.0)
    )), "keyboard focus border missing");
    let (output, _) = render(vec![Event::Key {
        key: egui::Key::Space,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);
    assert_eq!(
        rendered_text(&output)
            .iter()
            .filter(|text| *text == "Path:")
            .count(),
        1
    );
    // A second Tab moves focus independently; right's closed toggle must stay closed.
    render(vec![Event::Key {
        key: egui::Key::Tab,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);
    let (_, focused) = render(vec![]);
    assert!(focused[1].has_focus());
    let (output, _) = render(vec![Event::Key {
        key: egui::Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);
    assert_eq!(
        rendered_text(&output)
            .iter()
            .filter(|text| *text == "Path:")
            .count(),
        2
    );
    let newer = RequestTag { revision: 2, ..TAG };
    let mut output = ctx.run_ui(RawInput::default(), |ui| {
        ui.columns(2, |columns| {
            left.show(&mut columns[0], newer, Some(&snapshot));
            right.show(&mut columns[1], TAG, Some(&snapshot));
        });
    });
    output.textures_delta.clear();
    contains(
        &output,
        "Stale selection metadata. Waiting for the current selection.",
    );
    assert_eq!(
        rendered_text(&output)
            .iter()
            .filter(|text| *text == "Path:")
            .count(),
        1
    );
}

#[test]
fn p06_known_bytes_do_not_overflow_u64_or_count_unknown_as_zero() {
    let ctx = Context::default();
    let mut panel = SelectionSummary::new(1);
    let entries = [
        entry("a", EntryKind::RegularFile, Some(u64::MAX)),
        entry("b", EntryKind::RegularFile, Some(u64::MAX)),
        entry("zero", EntryKind::RegularFile, Some(0)),
        entry("unknown", EntryKind::RegularFile, None),
    ];
    let snapshot = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let (output, _) = frame(&ctx, &mut panel, TAG, Some(&snapshot), vec![]);
    contains(&output, "Known regular-file bytes: 36893488147419103230");
    contains(&output, "Regular files with known size: 3");
    contains(&output, "Entries with unknown size: 1");
}

#[cfg(windows)]
#[test]
fn p06_native_non_unicode_identity_survives_display_collisions() {
    use std::{
        ffi::OsString,
        os::windows::ffi::{OsStrExt, OsStringExt},
        path::PathBuf,
    };
    let names = [
        OsString::from_wide(&[0x78, 0xD800]),
        OsString::from_wide(&[0x78, 0xD801]),
    ];
    let paths = [PathBuf::from(&names[0]), PathBuf::from(&names[1])];
    assert_eq!(names[0].to_string_lossy(), names[1].to_string_lossy());
    let entries = [
        SelectedEntry {
            name: &names[0],
            path: &paths[0],
            ..entry("unused", EntryKind::RegularFile, Some(3))
        },
        SelectedEntry {
            name: &names[1],
            path: &paths[1],
            ..entry("unused", EntryKind::RegularFile, Some(5))
        },
    ];
    let ctx = Context::default();
    let mut panel = SelectionSummary::new(1);
    let snapshot = SelectionSnapshot {
        tag: TAG,
        state: SnapshotState::Ready(&entries),
    };
    let (output, _) = frame(&ctx, &mut panel, TAG, Some(&snapshot), vec![]);
    contains(&output, "Selected visible entries: 2");
    contains(&output, "Known regular-file bytes: 8");
    assert_ne!(entries[0].path, entries[1].path);
    assert_eq!(
        entries[0].name.encode_wide().collect::<Vec<_>>(),
        [0x78, 0xD800]
    );
    assert_eq!(
        entries[1]
            .path
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        [0x78, 0xD801]
    );
}
