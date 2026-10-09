use super::*;
use egui::{Event, PointerButton, Pos2, RawInput, Rect, Vec2};

const FILES: ConflictType = ConflictType {
    source: EntryKind::RegularFile,
    destination: EntryKind::RegularFile,
};
const FOLDERS: ConflictType = ConflictType {
    source: EntryKind::Directory,
    destination: EntryKind::Directory,
};

fn request(operation: u64, conflict: u64, kind: ConflictType) -> ConflictRequest {
    ConflictRequest::new(
        OperationId(operation),
        ConflictId(conflict),
        PathBuf::from(if cfg!(windows) {
            r"C:\generated\source\file.txt"
        } else {
            "/generated/source/file.txt"
        }),
        PathBuf::from(if cfg!(windows) {
            r"C:\generated\destination\file.txt"
        } else {
            "/generated/destination/file.txt"
        }),
        kind,
    )
    .unwrap()
}

fn frame(
    ctx: &Context,
    dialog: &mut ConflictDialog,
    events: Vec<Event>,
) -> (DialogOutput, egui::FullOutput) {
    let input = RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 700.0))),
        events,
        ..Default::default()
    };
    let mut result = DialogOutput::default();
    let mut output = ctx.run_ui(input, |ui| {
        let current = dialog.show(ui.ctx());
        if current.decision.is_some() || !result.owns_input {
            result = current;
        }
    });
    // Headless check has no texture backend; explicitly discard generated uploads.
    output.textures_delta.clear();
    (result, output)
}

fn open(kind: ConflictType) -> (Context, ConflictDialog) {
    let ctx = Context::default();
    let mut dialog = ConflictDialog::new(OperationId(1));
    dialog.present(request(1, 1, kind)).unwrap();
    for _ in 0..3 {
        assert!(frame(&ctx, &mut dialog, vec![]).0.decision.is_none());
    }
    (ctx, dialog)
}

fn key(key: Key) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

fn click(ctx: &Context, dialog: &mut ConflictDialog, position: Pos2) -> DialogOutput {
    let press = vec![
        Event::PointerMoved(position),
        Event::PointerButton {
            pos: position,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::NONE,
        },
    ];
    assert!(frame(ctx, dialog, press).0.decision.is_none());
    frame(
        ctx,
        dialog,
        vec![Event::PointerButton {
            pos: position,
            button: PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::NONE,
        }],
    )
    .0
}

#[test]
fn actual_buttons_return_captured_requests_once() {
    for (index, choice) in [
        Choice::Skip,
        Choice::KeepBoth,
        Choice::Replace,
        Choice::Cancel,
    ]
    .into_iter()
    .enumerate()
    {
        let (ctx, mut dialog) = open(FILES);
        let captured = dialog.pending().unwrap().clone();
        let position = dialog.controls.as_ref().unwrap().0[index].rect.center();
        let output = click(&ctx, &mut dialog, position);
        assert!(output.owns_input);
        let decision = output.decision.unwrap();
        assert_eq!(decision.choice(), choice);
        assert_eq!(decision.origin(), DecisionOrigin::User);
        assert!(decision.matches(&captured));
        assert!(!decision.apply_to_remaining());
        assert!(dialog.pending().is_none());
        let next = frame(&ctx, &mut dialog, vec![]).0;
        assert!(next.decision.is_none());
        assert!(!next.owns_input);
    }
}

#[test]
fn enter_is_skip_even_if_replace_is_focused_escape_is_cancel() {
    for action in [Key::Enter, Key::Escape] {
        let (ctx, mut dialog) = open(FILES);
        let replace = &dialog.controls.as_ref().unwrap().0[2];
        ctx.memory_mut(|memory| memory.request_focus(replace.id));
        let result = frame(&ctx, &mut dialog, vec![key(action)]).0;
        assert!(result.owns_input);
        assert_eq!(
            result.decision.unwrap().choice(),
            if action == Key::Enter {
                Choice::Skip
            } else {
                Choice::Cancel
            }
        );
    }
}

#[test]
fn tab_and_space_reach_replace_without_destructive_default() {
    let (ctx, mut dialog) = open(FILES);
    let skip_id = dialog.controls.as_ref().unwrap().0[0].id;
    assert!(ctx.memory(|memory| memory.has_focus(skip_id)));
    for _ in 0..2 {
        assert!(
            frame(&ctx, &mut dialog, vec![key(Key::Tab)])
                .0
                .decision
                .is_none()
        );
        frame(&ctx, &mut dialog, vec![]);
    }
    let replace = &dialog.controls.as_ref().unwrap().0[2];
    assert!(ctx.memory(|memory| memory.has_focus(replace.id)));
    let result = frame(&ctx, &mut dialog, vec![key(Key::Space)])
        .0
        .decision
        .unwrap();
    assert_eq!(result.choice(), Choice::Replace);
}

#[test]
fn old_request_and_operation_cannot_answer_new_request() {
    let (ctx, mut dialog) = open(FILES);
    let old = dialog.pending().unwrap().clone();
    dialog.pending.as_mut().unwrap().apply = true;
    let newer = request(1, 2, FOLDERS);
    assert!(dialog.present(newer.clone()).unwrap().is_none());
    assert!(!dialog.pending.as_ref().unwrap().apply);
    assert_eq!(
        dialog.decide(OperationId(1), ConflictId(1), Choice::Skip, true),
        Err(Refusal::StaleRequest)
    );
    assert_eq!(
        dialog.decide(OperationId(2), ConflictId(2), Choice::Skip, true),
        Err(Refusal::WrongOperation)
    );
    assert_eq!(dialog.present(old.clone()), Err(Refusal::StaleRequest));
    assert_eq!(
        dialog.present(request(2, 3, FILES)),
        Err(Refusal::WrongOperation)
    );
    // Opening-frame Enter belongs to no previously displayed current request.
    assert!(
        frame(&ctx, &mut dialog, vec![key(Key::Enter)])
            .0
            .decision
            .is_none()
    );
    frame(
        &ctx,
        &mut dialog,
        vec![Event::Key {
            key: Key::Enter,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
    );
    let decision = frame(&ctx, &mut dialog, vec![key(Key::Enter)])
        .0
        .decision
        .unwrap();
    assert!(decision.matches(&newer));
    assert!(!decision.matches(&old));
    assert!(!decision.apply_to_remaining());
    assert_eq!(dialog.present(newer), Err(Refusal::StaleRequest));
}

#[test]
fn new_request_does_not_inherit_pointer_activation_or_focus() {
    let (ctx, mut dialog) = open(FILES);
    let old_replace = dialog.controls.as_ref().unwrap().0[2].rect.center();
    ctx.memory_mut(|memory| memory.request_focus(dialog.controls.as_ref().unwrap().0[2].id));
    frame(
        &ctx,
        &mut dialog,
        vec![
            Event::PointerMoved(old_replace),
            Event::PointerButton {
                pos: old_replace,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::NONE,
            },
        ],
    );
    dialog.present(request(1, 2, FILES)).unwrap();
    let release = Event::PointerButton {
        pos: old_replace,
        button: PointerButton::Primary,
        pressed: false,
        modifiers: Modifiers::NONE,
    };
    assert!(frame(&ctx, &mut dialog, vec![release]).0.decision.is_none());
    frame(&ctx, &mut dialog, vec![]);
    assert!(ctx.memory(|memory| memory.has_focus(dialog.controls.as_ref().unwrap().0[0].id)));
    assert_eq!(
        frame(&ctx, &mut dialog, vec![key(Key::Enter)])
            .0
            .decision
            .unwrap()
            .request()
            .conflict(),
        ConflictId(2)
    );
}

#[test]
fn actual_checkbox_applies_only_to_next_compatible_conflicts() {
    for (index, choice) in [Choice::Skip, Choice::KeepBoth, Choice::Replace]
        .into_iter()
        .enumerate()
    {
        let (ctx, mut dialog) = open(FILES);
        let checkbox = dialog.controls.as_ref().unwrap().1.rect.center();
        assert!(click(&ctx, &mut dialog, checkbox).decision.is_none());
        assert!(dialog.pending.as_ref().unwrap().apply);
        let button = dialog.controls.as_ref().unwrap().0[index].rect.center();
        let decision = click(&ctx, &mut dialog, button).decision.unwrap();
        assert_eq!(decision.choice(), choice);
        assert!(decision.apply_to_remaining());
        let fresh = request(1, 2, FILES);
        let policy_response = dialog.present(fresh.clone()).unwrap().unwrap();
        assert!(policy_response.matches(&fresh));
        assert_eq!(policy_response.choice(), choice);
        assert_eq!(policy_response.origin(), DecisionOrigin::RemainingPolicy);
        assert!(dialog.present(request(1, 3, FOLDERS)).unwrap().is_none());
        assert!(!dialog.pending.as_ref().unwrap().apply);
        assert!(
            dialog
                .present(request(
                    1,
                    4,
                    ConflictType {
                        source: EntryKind::RegularFile,
                        destination: EntryKind::Directory
                    }
                ))
                .unwrap()
                .is_none()
        );
        assert!(
            dialog
                .present(request(
                    1,
                    5,
                    ConflictType {
                        source: EntryKind::Directory,
                        destination: EntryKind::RegularFile
                    }
                ))
                .unwrap()
                .is_none()
        );
    }
}

#[test]
fn directory_and_type_conflicts_refuse_replace_in_controller_and_widget() {
    for source in [
        EntryKind::RegularFile,
        EntryKind::Directory,
        EntryKind::Other,
    ] {
        for destination in [
            EntryKind::RegularFile,
            EntryKind::Directory,
            EntryKind::Other,
        ] {
            let kind = ConflictType {
                source,
                destination,
            };
            if kind == FILES {
                continue;
            }
            let (ctx, mut dialog) = open(kind);
            let replace = &dialog.controls.as_ref().unwrap().0[2];
            assert!(!replace.enabled());
            let position = replace.rect.center();
            assert!(click(&ctx, &mut dialog, position).decision.is_none());
            assert_eq!(
                dialog.decide(OperationId(1), ConflictId(1), Choice::Replace, true),
                Err(Refusal::ReplaceNotAllowed)
            );
            assert!(dialog.pending().is_some());
            let response = frame(&ctx, &mut dialog, vec![key(Key::Enter)])
                .0
                .decision
                .unwrap();
            assert_eq!(response.choice(), Choice::Skip);
        }
    }
}

#[test]
fn independent_operations_share_neither_policy_nor_checkbox() {
    let (ctx, mut first) = open(FILES);
    let checkbox = first.controls.as_ref().unwrap().1.rect.center();
    click(&ctx, &mut first, checkbox);
    let replace = first.controls.as_ref().unwrap().0[2].rect.center();
    assert_eq!(
        click(&ctx, &mut first, replace).decision.unwrap().choice(),
        Choice::Replace
    );
    assert_eq!(
        first.present(request(2, 2, FILES)),
        Err(Refusal::WrongOperation)
    );
    let mut second = ConflictDialog::new(OperationId(2));
    assert!(second.present(request(2, 1, FILES)).unwrap().is_none());
    assert!(!second.pending.as_ref().unwrap().apply);
    for _ in 0..3 {
        frame(&ctx, &mut second, vec![]);
    }
    let response = frame(&ctx, &mut second, vec![key(Key::Enter)])
        .0
        .decision
        .unwrap();
    assert_eq!(response.choice(), Choice::Skip);
    assert_eq!(response.request().operation(), OperationId(2));
    assert_eq!(
        first
            .present(request(1, 2, FILES))
            .unwrap()
            .unwrap()
            .choice(),
        Choice::Replace
    );
}

#[test]
fn cancel_and_finish_clear_policy_and_end_only_their_operation() {
    let (ctx, mut dialog) = open(FILES);
    dialog
        .decide(OperationId(1), ConflictId(1), Choice::Replace, true)
        .unwrap();
    dialog.present(request(1, 2, FOLDERS)).unwrap();
    for _ in 0..3 {
        frame(&ctx, &mut dialog, vec![]);
    }
    let checkbox = dialog.controls.as_ref().unwrap().1.rect.center();
    click(&ctx, &mut dialog, checkbox);
    let cancel = frame(&ctx, &mut dialog, vec![key(Key::Escape)])
        .0
        .decision
        .unwrap();
    assert_eq!(cancel.choice(), Choice::Cancel);
    assert!(!cancel.apply_to_remaining());
    assert!(dialog.policy.is_none());
    assert_eq!(
        dialog.present(request(1, 3, FILES)),
        Err(Refusal::OperationEnded)
    );
    let mut other = ConflictDialog::new(OperationId(2));
    other.present(request(2, 1, FILES)).unwrap();
    other.finish();
    assert!(other.pending().is_none());
    assert_eq!(
        other.present(request(2, 2, FILES)),
        Err(Refusal::OperationEnded)
    );
}

#[test]
fn manual_choice_without_apply_clears_previous_policy() {
    let mut dialog = ConflictDialog::new(OperationId(1));
    dialog.present(request(1, 1, FILES)).unwrap();
    dialog
        .decide(OperationId(1), ConflictId(1), Choice::Replace, true)
        .unwrap();
    dialog.present(request(1, 2, FOLDERS)).unwrap();
    dialog
        .decide(OperationId(1), ConflictId(2), Choice::KeepBoth, false)
        .unwrap();
    assert!(dialog.present(request(1, 3, FILES)).unwrap().is_none());
}

#[test]
fn paths_require_full_native_identity_and_truncate_only_visually() {
    let mut supplied = request(1, 1, FILES);
    let root = if cfg!(windows) {
        r"C:\generated"
    } else {
        "/generated"
    };
    supplied.source = PathBuf::from(root)
        .join("長é𐐀".repeat(180))
        .join("complete-source.txt");
    supplied.destination = PathBuf::from(root)
        .join("destination".repeat(100))
        .join("complete-destination.txt");
    let mut dialog = ConflictDialog::new(OperationId(1));
    dialog.present(supplied.clone()).unwrap();
    let ctx = Context::default();
    for _ in 0..3 {
        frame(&ctx, &mut dialog, vec![]);
    }
    let labels = &dialog.controls.as_ref().unwrap().2;
    assert!(labels.iter().all(|label| label.rect.width() <= 520.0));
    assert_eq!(dialog.pending().unwrap().source(), supplied.source());
    assert_eq!(
        dialog.pending().unwrap().destination(),
        supplied.destination()
    );
    fn contains_text(shape: &egui::Shape, wanted: &str) -> bool {
        match shape {
            egui::Shape::Text(text) => text.galley.text() == wanted,
            egui::Shape::Vec(shapes) => shapes.iter().any(|shape| contains_text(shape, wanted)),
            _ => false,
        }
    }
    for index in 0..2 {
        let position = dialog.controls.as_ref().unwrap().2[index].rect.center();
        let wanted = if index == 0 {
            supplied.source()
        } else {
            supplied.destination()
        }
        .to_string_lossy();
        let mut found = false;
        for _ in 0..80 {
            let output = frame(&ctx, &mut dialog, vec![Event::PointerMoved(position)]).1;
            found |= output
                .shapes
                .iter()
                .any(|shape| contains_text(&shape.shape, &wanted));
        }
        assert!(found, "Hover must include full supplied path {index}");
    }
    let decision = frame(&ctx, &mut dialog, vec![key(Key::Enter)])
        .0
        .decision
        .unwrap();
    assert!(decision.matches(&supplied));
    assert_eq!(
        ConflictRequest::new(
            OperationId(1),
            ConflictId(1),
            PathBuf::from("relative"),
            supplied.destination.clone(),
            FILES
        ),
        Err(Refusal::FullPathsRequired)
    );
}

#[cfg(windows)]
#[test]
fn unpaired_utf16_and_verbatim_paths_remain_native() {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    let mut wide: Vec<u16> = r"\\?\C:\generated\".encode_utf16().collect();
    wide.extend([0xD800, 0x0061]);
    let supplied = ConflictRequest::new(
        OperationId(1),
        ConflictId(1),
        PathBuf::from(OsString::from_wide(&wide)),
        PathBuf::from(r"\\?\C:\generated\destination.txt"),
        FILES,
    )
    .unwrap();
    let mut dialog = ConflictDialog::new(OperationId(1));
    dialog.present(supplied.clone()).unwrap();
    let ctx = Context::default();
    for _ in 0..3 {
        frame(&ctx, &mut dialog, vec![]);
    }
    let response = frame(&ctx, &mut dialog, vec![key(Key::Enter)])
        .0
        .decision
        .unwrap();
    assert_eq!(
        response
            .request()
            .source()
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        wide
    );
    assert!(response.matches(&supplied));
}

#[test]
fn repeated_or_modified_keys_do_not_answer_later_requests() {
    for pressed_key in [Key::Enter, Key::Space, Key::Escape] {
        let (ctx, mut dialog) = open(FILES);
        dialog.present(request(1, 2, FILES)).unwrap();
        assert!(
            frame(&ctx, &mut dialog, vec![key(pressed_key)])
                .0
                .decision
                .is_none()
        );
        for _ in 0..3 {
            frame(&ctx, &mut dialog, vec![]);
        }
        let replace_id = dialog.controls.as_ref().unwrap().0[2].id;
        ctx.memory_mut(|memory| memory.request_focus(replace_id));
        let repeated = Event::Key {
            key: pressed_key,
            physical_key: None,
            pressed: true,
            repeat: true,
            modifiers: Modifiers::NONE,
        };
        assert!(
            frame(&ctx, &mut dialog, vec![repeated])
                .0
                .decision
                .is_none(),
            "held key {pressed_key:?}"
        );
        let release = Event::Key {
            key: pressed_key,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        frame(&ctx, &mut dialog, vec![release.clone()]);
        let modified = Event::Key {
            key: pressed_key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::CTRL,
        };
        assert!(
            frame(&ctx, &mut dialog, vec![modified])
                .0
                .decision
                .is_none()
        );
        frame(&ctx, &mut dialog, vec![release]);
        assert_eq!(
            frame(&ctx, &mut dialog, vec![key(Key::Escape)])
                .0
                .decision
                .unwrap()
                .choice(),
            Choice::Cancel
        );
    }
}

#[test]
fn decision_frame_keeps_input_ownership_across_egui_passes_without_replaying_response() {
    let (ctx, mut dialog) = open(FILES);
    let mut responses = Vec::new();
    let mut passes = 0;
    let output = ctx.run_ui(
        RawInput {
            events: vec![key(Key::Enter)],
            ..Default::default()
        },
        |ui| {
            let result = dialog.show(ui.ctx());
            assert!(result.owns_input);
            if let Some(response) = result.decision {
                responses.push(response);
            }
            passes += 1;
            if ui.ctx().current_pass_index() == 0 {
                ui.ctx().request_discard("headless multipass regression");
            }
        },
    );
    output.drop_without_applying_deltas();
    assert_eq!(passes, 2);
    assert_eq!(responses.len(), 1);
    assert_eq!(responses[0].choice(), Choice::Skip);
    assert!(!frame(&ctx, &mut dialog, vec![]).0.owns_input);
}

#[test]
fn focused_checkbox_toggles_with_space() {
    let (ctx, mut dialog) = open(FILES);
    let checkbox = dialog.controls.as_ref().unwrap().1.id;
    ctx.memory_mut(|memory| memory.request_focus(checkbox));
    assert!(
        frame(&ctx, &mut dialog, vec![key(Key::Space)])
            .0
            .decision
            .is_none()
    );
    assert!(dialog.pending.as_ref().unwrap().apply);
    let decision = frame(&ctx, &mut dialog, vec![key(Key::Enter)])
        .0
        .decision
        .unwrap();
    assert!(decision.apply_to_remaining());
    assert_eq!(decision.choice(), Choice::Skip);
}

#[test]
fn outside_click_does_not_create_a_decision() {
    let (ctx, mut dialog) = open(FILES);
    assert!(
        click(&ctx, &mut dialog, Pos2::new(2.0, 2.0))
            .decision
            .is_none()
    );
    assert!(dialog.pending().is_some());
}
