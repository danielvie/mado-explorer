use super::*;
use egui::{Pos2, RawInput, Rect, Vec2};
use std::collections::HashSet;

fn root() -> PathBuf {
    PathBuf::from(if cfg!(windows) {
        r"C:\generated"
    } else {
        "/generated"
    })
}
fn request(id: u64, kind: Kind, count: usize) -> Request {
    Request::new(
        RequestId(id),
        kind,
        (0..count)
            .map(|n| root().join(format!("entry-{id}-{n}.txt")))
            .collect(),
        root().join(format!("location-{id}")),
    )
}
fn key(key: Key, pressed: bool) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}
fn pointer(pos: Pos2, pressed: bool) -> Vec<Event> {
    vec![
        Event::PointerMoved(pos),
        Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        },
    ]
}
fn raw(events: Vec<Event>) -> RawInput {
    RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 700.0))),
        events,
        ..Default::default()
    }
}
fn run(ctx: &Context, dialog: &mut Confirmation, events: Vec<Event>) -> (Output, egui::FullOutput) {
    let mut result = Output::default();
    let mut output = ctx.run_ui(raw(events), |ui| {
        let current = dialog.show(ui.ctx());
        if current.answer.is_some() || !result.owns_input {
            result = current;
        }
    });
    output.textures_delta.clear();
    (result, output)
}
fn frame(ctx: &Context, dialog: &mut Confirmation, events: Vec<Event>) -> Output {
    run(ctx, dialog, events).0
}
fn settle(ctx: &Context, dialog: &mut Confirmation) {
    for _ in 0..3 {
        assert!(frame(ctx, dialog, vec![]).answer.is_none());
    }
}
fn open(kind: Kind) -> (Context, Confirmation) {
    let ctx = Context::default();
    let mut dialog = Confirmation::new();
    dialog.present(request(1, kind, 3)).unwrap();
    settle(&ctx, &mut dialog);
    (ctx, dialog)
}
fn click(ctx: &Context, dialog: &mut Confirmation, index: usize) -> Answer {
    let pos = dialog.controls.as_ref().unwrap().0[index].rect.center();
    assert!(frame(ctx, dialog, pointer(pos, true)).answer.is_none());
    frame(ctx, dialog, pointer(pos, false)).answer.unwrap()
}
fn text(shape: &egui::Shape, wanted: &str) -> bool {
    match shape {
        egui::Shape::Text(t) => t.galley.text().contains(wanted),
        egui::Shape::Vec(shapes) => shapes.iter().any(|shape| text(shape, wanted)),
        _ => false,
    }
}
fn output_text(output: &egui::FullOutput, wanted: &str) -> bool {
    output.shapes.iter().any(|s| text(&s.shape, wanted))
}

#[test]
fn actual_kind_warnings_count_location_and_control_labels() {
    for kind in [Kind::Trash, Kind::PermanentDelete] {
        let (ctx, mut dialog) = open(kind);
        let output = run(&ctx, &mut dialog, vec![]).1;
        assert!(output_text(&output, "3 captured items"));
        assert!(output_text(
            &output,
            &dialog.pending().unwrap().location().to_string_lossy()
        ));
        assert!(output_text(&output, "Keyboard focus: Cancel"));
        match kind {
            Kind::Trash => {
                assert!(output_text(&output, "Recoverable items can be restored"));
                assert!(output_text(&output, "never fall back"));
                assert!(output_text(&output, "Confirm Trash"));
                assert!(!output_text(&output, "Irreversible:"));
            }
            Kind::PermanentDelete => {
                assert!(output_text(&output, "Irreversible:"));
                assert!(output_text(&output, "NOT go to the Recycle Bin"));
                assert!(output_text(&output, "Confirm Permanent Delete"));
            }
        }
    }
}

#[test]
fn empty_and_resource_limits_refuse_visibly_without_omitting_entries() {
    let mut cases = vec![
        (request(1, Kind::Trash, 0), Refusal::Empty),
        (
            request(2, Kind::Trash, MAX_ITEMS + 1),
            Refusal::TooManyItems,
        ),
    ];
    let mut long = request(3, Kind::Trash, 1);
    long.paths[0] = root().join("x".repeat(MAX_PATH_UNITS));
    cases.push((long, Refusal::PathTooLong));
    let mut total = request(4, Kind::Trash, 5);
    total.paths = (0..5).map(|_| root().join("x".repeat(30_000))).collect();
    cases.push((total, Refusal::TotalPathsTooLarge));
    let mut relative = request(5, Kind::Trash, 1);
    relative.paths[0] = PathBuf::from("relative");
    cases.push((relative, Refusal::FullPathsRequired));
    for (request, refusal) in cases {
        let ctx = Context::default();
        let mut dialog = Confirmation::new();
        assert_eq!(dialog.present(request), Err(refusal));
        assert!(dialog.pending().is_none());
        assert_eq!(
            dialog.choose(RequestId(1), Choice::Confirm),
            Err(Refusal::Closed)
        );
        let mut seen = false;
        for _ in 0..3 {
            let (result, output) = run(&ctx, &mut dialog, vec![]);
            assert!(result.owns_input);
            assert!(result.answer.is_none());
            seen |= output_text(&output, refusal.message());
        }
        assert!(seen);
    }
}

#[test]
fn enter_escape_and_cancel_have_no_deletion_intent() {
    for kind in [Kind::Trash, Kind::PermanentDelete] {
        for action in [Some(Key::Enter), Some(Key::Escape), None] {
            let (ctx, mut dialog) = open(kind);
            let id = dialog.controls.as_ref().unwrap().0[0].id;
            assert!(ctx.memory(|m| m.has_focus(id)));
            let answer = match action {
                Some(key_) => frame(&ctx, &mut dialog, vec![key(key_, true)])
                    .answer
                    .unwrap(),
                None => click(&ctx, &mut dialog, 0),
            };
            assert_eq!(answer.choice(), Choice::Cancel);
            assert!(answer.deletion_intent().is_none());
            assert_eq!(
                dialog.choose(RequestId(1), Choice::Confirm),
                Err(Refusal::Closed)
            );
            assert!(!frame(&ctx, &mut dialog, vec![]).owns_input);
        }
    }
}

#[test]
fn actual_confirm_returns_exact_multi_selection_snapshot_once() {
    for kind in [Kind::Trash, Kind::PermanentDelete] {
        let (ctx, mut dialog) = open(kind);
        let original = dialog.pending().unwrap().clone();
        let mut later_selection = original.paths().to_vec();
        later_selection.reverse();
        later_selection[0] = root().join("different-selection");
        assert_ne!(later_selection, original.paths());
        let answer = click(&ctx, &mut dialog, 1);
        assert_eq!(answer.request(), &original);
        assert_eq!(answer.deletion_intent(), Some(&original));
        assert_eq!(answer.request().count(), 3);
        assert!(dialog.pending().is_none());
        assert_eq!(
            dialog.choose(original.id(), Choice::Confirm),
            Err(Refusal::Closed)
        );
        assert!(frame(&ctx, &mut dialog, vec![]).answer.is_none());
    }
}

#[test]
fn tab_then_enter_or_space_explicitly_confirms_with_visible_focus() {
    for action in [Key::Space, Key::Enter] {
        let (ctx, mut dialog) = open(Kind::PermanentDelete);
        assert!(
            frame(&ctx, &mut dialog, vec![key(Key::Tab, true)])
                .answer
                .is_none()
        );
        let output = run(&ctx, &mut dialog, vec![key(Key::Tab, false)]).1;
        assert!(ctx.memory(|m| m.has_focus(dialog.controls.as_ref().unwrap().0[1].id)));
        assert!(output_text(&output, "Keyboard focus: Confirm"));
        assert_eq!(
            frame(&ctx, &mut dialog, vec![key(action, true)])
                .answer
                .unwrap()
                .choice(),
            Choice::Confirm
        );
    }
}

#[test]
fn arbitrary_cached_focus_without_new_navigation_cannot_confirm() {
    let (ctx, mut dialog) = open(Kind::PermanentDelete);
    ctx.memory_mut(|m| m.request_focus(dialog.controls.as_ref().unwrap().0[1].id));
    assert_eq!(
        frame(&ctx, &mut dialog, vec![key(Key::Enter, true)])
            .answer
            .unwrap()
            .choice(),
        Choice::Cancel
    );
}

#[test]
fn wrong_old_answered_closed_ids_cannot_respond_and_fresh_requests_have_no_policy() {
    let (ctx, mut dialog) = open(Kind::Trash);
    let old = dialog.pending().unwrap().clone();
    let newer = request(2, Kind::PermanentDelete, 5);
    dialog.present(newer.clone()).unwrap();
    assert_eq!(
        dialog.choose(old.id(), Choice::Confirm),
        Err(Refusal::WrongRequest)
    );
    assert_eq!(
        dialog.choose(RequestId(99), Choice::Confirm),
        Err(Refusal::WrongRequest)
    );
    assert_eq!(dialog.present(old), Err(Refusal::StaleRequest));
    settle(&ctx, &mut dialog);
    assert_eq!(click(&ctx, &mut dialog, 1).request(), &newer);
    assert_eq!(dialog.present(newer), Err(Refusal::StaleRequest));
    dialog.present(request(3, Kind::Trash, 1)).unwrap();
    settle(&ctx, &mut dialog);
    assert!(dialog.pending().is_some());
    assert!(ctx.memory(|m| m.has_focus(dialog.controls.as_ref().unwrap().0[0].id)));
    assert_eq!(dialog.close(RequestId(2)), Err(Refusal::WrongRequest));
    dialog.close(RequestId(3)).unwrap();
    assert_eq!(
        dialog.choose(RequestId(3), Choice::Confirm),
        Err(Refusal::Closed)
    );
    assert_eq!(dialog.close(RequestId(3)), Err(Refusal::Closed));
    assert!(frame(&ctx, &mut dialog, vec![]).answer.is_none());
}

#[test]
fn old_held_pointer_and_focus_do_not_cross_request_or_controller_epoch() {
    for recreate in [false, true] {
        let (ctx, mut dialog) = open(Kind::Trash);
        let pos = dialog.controls.as_ref().unwrap().0[1].rect.center();
        assert!(
            frame(&ctx, &mut dialog, pointer(pos, true))
                .answer
                .is_none()
        );
        if recreate {
            dialog = Confirmation::new();
        }
        dialog.present(request(2, Kind::Trash, 3)).unwrap();
        settle(&ctx, &mut dialog);
        assert!(
            frame(&ctx, &mut dialog, pointer(pos, false))
                .answer
                .is_none()
        );
        assert!(ctx.memory(|m| m.has_focus(dialog.controls.as_ref().unwrap().0[0].id)));
        assert_eq!(click(&ctx, &mut dialog, 1).choice(), Choice::Confirm);
    }
}

#[test]
fn opening_keys_and_navigation_do_not_restore_confirm_focus() {
    for opening in [Key::Enter, Key::Space, Key::Escape, Key::Tab] {
        let (ctx, mut dialog) = open(Kind::Trash);
        ctx.memory_mut(|m| m.request_focus(dialog.controls.as_ref().unwrap().0[1].id));
        dialog
            .present(request(2, Kind::PermanentDelete, 3))
            .unwrap();
        assert!(
            frame(&ctx, &mut dialog, vec![key(opening, true)])
                .answer
                .is_none()
        );
        settle(&ctx, &mut dialog);
        assert!(ctx.memory(|m| m.has_focus(dialog.controls.as_ref().unwrap().0[0].id)));
        frame(&ctx, &mut dialog, vec![key(opening, false)]);
        assert_eq!(
            frame(&ctx, &mut dialog, vec![key(Key::Enter, true)])
                .answer
                .unwrap()
                .choice(),
            Choice::Cancel
        );
    }
}

#[test]
fn held_repeated_modified_keys_cannot_confirm_and_new_intent_works() {
    for recreate in [false, true] {
        let (ctx, mut dialog) = open(Kind::Trash);
        frame(&ctx, &mut dialog, vec![key(Key::Tab, true)]);
        frame(&ctx, &mut dialog, vec![key(Key::Tab, false)]);
        // A modified press is held across a new request, including controller recreation.
        let mut held = key(Key::Space, true);
        if let Event::Key { modifiers, .. } = &mut held {
            *modifiers = Modifiers::CTRL;
        }
        assert!(frame(&ctx, &mut dialog, vec![held]).answer.is_none());
        if recreate {
            dialog = Confirmation::new();
        }
        dialog.present(request(2, Kind::Trash, 3)).unwrap();
        settle(&ctx, &mut dialog);
        frame(&ctx, &mut dialog, vec![key(Key::Tab, true)]);
        frame(&ctx, &mut dialog, vec![key(Key::Tab, false)]);
        for repeat in [true, false] {
            let mut held = key(Key::Space, true);
            if let Event::Key { repeat: r, .. } = &mut held {
                *r = repeat;
            }
            assert!(frame(&ctx, &mut dialog, vec![held]).answer.is_none());
        }
        frame(&ctx, &mut dialog, vec![key(Key::Space, false)]);
        assert_eq!(
            frame(&ctx, &mut dialog, vec![key(Key::Space, true)])
                .answer
                .unwrap()
                .choice(),
            Choice::Confirm
        );
    }
    for key_ in [Key::Escape, Key::Enter, Key::Space] {
        let (ctx, mut dialog) = open(Kind::Trash);
        let mut modified = key(key_, true);
        if let Event::Key { modifiers, .. } = &mut modified {
            *modifiers = Modifiers::SHIFT;
        }
        assert!(frame(&ctx, &mut dialog, vec![modified]).answer.is_none());
    }
}

#[test]
fn answer_frame_and_layout_retries_own_input_and_emit_once() {
    let (ctx, mut dialog) = open(Kind::Trash);
    let pos = dialog.controls.as_ref().unwrap().0[1].rect.center();
    frame(&ctx, &mut dialog, pointer(pos, true));
    let mut passes = 0;
    let mut answers = Vec::new();
    ctx.run_ui(raw(pointer(pos, false)), |ui| {
        let output = dialog.show(ui.ctx());
        assert!(output.owns_input);
        if let Some(answer) = output.answer {
            answers.push(answer);
        }
        let again = dialog.show(ui.ctx());
        assert!(again.owns_input);
        assert!(again.answer.is_none());
        passes += 1;
        if ui.ctx().current_pass_index() == 0 {
            ui.ctx().request_discard("confirmation retry");
        }
    })
    .drop_without_applying_deltas();
    assert_eq!(passes, 2);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].choice(), Choice::Confirm);
    assert!(!frame(&ctx, &mut dialog, vec![]).owns_input);
}

#[test]
fn higher_modal_and_unrelated_input_keep_their_ownership() {
    let (ctx, mut dialog) = open(Kind::Trash);
    let other = Id::new("generated-other-modal");
    for _ in 0..3 {
        ctx.run_ui(raw(vec![]), |ui| {
            assert!(dialog.show(ui.ctx()).answer.is_none());
            egui::Modal::new(other).show(ui.ctx(), |ui| {
                ui.label("Other modal");
            });
        })
        .drop_without_applying_deltas();
    }
    ctx.run_ui(
        raw(vec![
            key(Key::Enter, true),
            Event::Text("owned text".into()),
        ]),
        |ui| {
            assert!(dialog.show(ui.ctx()).answer.is_none());
            assert!(ui.input(|i| i.events.iter().any(|e| matches!(
                e,
                Event::Key {
                    key: Key::Enter,
                    ..
                }
            ))));
            assert!(ui.input(|i| {
                i.events
                    .iter()
                    .any(|e| matches!(e, Event::Text(t) if t == "owned text"))
            }));
            egui::Modal::new(other).show(ui.ctx(), |ui| {
                ui.label("Other modal");
            });
        },
    )
    .drop_without_applying_deltas();
    frame(&ctx, &mut dialog, vec![key(Key::Enter, false)]);
    settle(&ctx, &mut dialog);
    let mut retained_text = false;
    ctx.run_ui(raw(vec![Event::Text("unrelated".into())]), |ui| {
        assert!(dialog.show(ui.ctx()).answer.is_none());
        retained_text = ui.input(|i| {
            i.events
                .iter()
                .any(|e| matches!(e, Event::Text(t) if t == "unrelated"))
        });
    })
    .drop_without_applying_deltas();
    assert!(retained_text);
    assert_eq!(click(&ctx, &mut dialog, 1).choice(), Choice::Confirm);
}

#[cfg(windows)]
#[test]
fn native_unpaired_utf16_and_long_paths_survive_response_exactly() {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    let mut supplied = request(1, Kind::PermanentDelete, 2);
    let mut native = root().as_os_str().encode_wide().collect::<Vec<_>>();
    native.extend([92, 0xd800, 0x00e9, 0xd801, 0xdc00]);
    native.extend(vec![0x61; 600]);
    supplied.paths[0] = PathBuf::from(OsString::from_wide(&native));
    let ctx = Context::default();
    let mut dialog = Confirmation::new();
    dialog.present(supplied.clone()).unwrap();
    settle(&ctx, &mut dialog);
    let answer = click(&ctx, &mut dialog, 1);
    assert_eq!(answer.request(), &supplied);
    assert_eq!(
        answer.request().paths()[0]
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        native
    );
}

#[test]
fn accessibility_buttons_have_labels_and_cancel_is_focused() {
    for kind in [Kind::Trash, Kind::PermanentDelete] {
        let (ctx, mut dialog) = open(kind);
        ctx.enable_accesskit();
        let output = run(&ctx, &mut dialog, vec![]).1;
        let tree = output.platform_output.accesskit_update.as_ref().unwrap();
        let buttons: Vec<_> = tree
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
            .collect();
        assert!(
            buttons
                .iter()
                .any(|(_, node)| node.label() == Some("Cancel"))
        );
        assert!(buttons.iter().any(|(_, node)| node.label()
            == Some(match kind {
                Kind::Trash => "Confirm Trash",
                Kind::PermanentDelete => "Confirm Permanent Delete",
            })));
        let focused = tree.nodes.iter().find(|(id, _)| *id == tree.focus).unwrap();
        assert_eq!(focused.1.role(), egui::accesskit::Role::Button);
        assert_eq!(focused.1.label(), Some("Cancel"));
    }
}

#[test]
fn all_maximum_captured_entries_are_present_with_fixed_slot_ids() {
    let (ctx, mut dialog) = open(Kind::Trash);
    let mut ids = HashSet::new();
    for id in 2..=5 {
        let supplied = request(id, Kind::Trash, MAX_ITEMS);
        dialog.present(supplied.clone()).unwrap();
        settle(&ctx, &mut dialog);
        let (_, location, paths) = dialog.controls.as_ref().unwrap();
        assert_eq!(paths.len(), MAX_ITEMS);
        ids.insert(location.id);
        ids.extend(paths.iter().map(|path| path.id));
        assert_eq!(ids.len(), MAX_ITEMS + 1);
        assert!(output_text(
            &run(&ctx, &mut dialog, vec![]).1,
            "64 captured items"
        ));
        assert_eq!(click(&ctx, &mut dialog, 1).request(), &supplied);
    }
}

#[test]
fn opening_pointer_press_with_key_cannot_carry_into_current_request() {
    let (ctx, mut dialog) = open(Kind::Trash);
    let pos = dialog.controls.as_ref().unwrap().0[1].rect.center();
    dialog.present(request(2, Kind::Trash, 3)).unwrap();
    let mut events = pointer(pos, true);
    events.push(key(Key::Space, true));
    events.push(key(Key::Tab, true));
    assert!(frame(&ctx, &mut dialog, events).answer.is_none());
    settle(&ctx, &mut dialog);
    assert!(
        frame(&ctx, &mut dialog, pointer(pos, false))
            .answer
            .is_none()
    );
    assert!(ctx.memory(|m| m.has_focus(dialog.controls.as_ref().unwrap().0[0].id)));
    frame(
        &ctx,
        &mut dialog,
        vec![key(Key::Space, false), key(Key::Tab, false)],
    );
    assert_eq!(click(&ctx, &mut dialog, 1).choice(), Choice::Confirm);
}

#[test]
fn invalid_new_request_removes_previous_positive_action() {
    let (ctx, mut dialog) = open(Kind::Trash);
    assert_eq!(
        dialog.present(request(2, Kind::PermanentDelete, 0)),
        Err(Refusal::Empty)
    );
    assert_eq!(
        dialog.choose(RequestId(1), Choice::Confirm),
        Err(Refusal::Closed)
    );
    assert!(
        frame(&ctx, &mut dialog, vec![key(Key::Enter, true)])
            .answer
            .is_none()
    );
    dialog.present(request(3, Kind::Trash, 1)).unwrap();
    settle(&ctx, &mut dialog);
    assert!(dialog.refusal().is_none());
    assert_eq!(click(&ctx, &mut dialog, 1).request().id(), RequestId(3));
}

fn retained(ctx: &Context) -> (HashSet<egui::LayerId>, HashSet<egui::LayerId>) {
    let layers: HashSet<_> = ctx.memory(|m| m.layer_ids().collect());
    let areas = layers
        .iter()
        .copied()
        .filter(|layer| egui::AreaState::load(ctx, layer.id).is_some())
        .collect();
    (layers, areas)
}
#[test]
fn many_request_controller_cycles_bound_actual_focus_ids_and_hover_areas() {
    let ctx = Context::default();
    ctx.global_style_mut(|s| {
        s.interaction.tooltip_delay = 0.0;
        s.interaction.show_tooltips_only_when_still = false;
    });
    let mut ids = HashSet::new();
    let mut path_ids = HashSet::new();
    let mut baseline = None;
    for controller in 0..8 {
        let mut dialog = Confirmation::new();
        for n in 1..=32 {
            let mut supplied = request(
                controller * 32 + n,
                if n % 2 == 0 {
                    Kind::Trash
                } else {
                    Kind::PermanentDelete
                },
                3,
            );
            supplied.location = root().join("long-location".repeat(50));
            supplied.paths = (0..3)
                .map(|index| root().join(format!("{index}-{}-{n}", "é長𐐀".repeat(100))))
                .collect();
            dialog.present(supplied.clone()).unwrap();
            settle(&ctx, &mut dialog);
            let (buttons, location, paths) = dialog.controls.as_ref().unwrap();
            ids.extend(buttons.iter().map(|r| r.id));
            let mut positions = vec![(
                location.rect.center(),
                supplied.location().to_string_lossy().into_owned(),
            )];
            positions.extend(
                paths
                    .iter()
                    .zip(supplied.paths())
                    .map(|(r, p)| (r.rect.center(), p.to_string_lossy().into_owned())),
            );
            path_ids.insert(location.id);
            path_ids.extend(paths.iter().map(|r| r.id));
            for (pos, wanted) in positions {
                let mut seen = false;
                for _ in 0..4 {
                    seen |= output_text(
                        &run(&ctx, &mut dialog, vec![Event::PointerMoved(pos)]).1,
                        &wanted,
                    );
                }
                assert!(seen, "actual full-path hover missing");
            }
            assert_eq!(click(&ctx, &mut dialog, 0).choice(), Choice::Cancel);
            frame(&ctx, &mut dialog, vec![Event::PointerGone]);
            frame(&ctx, &mut dialog, vec![]);
            let state = retained(&ctx);
            let expected = baseline.get_or_insert_with(|| state.clone());
            assert_eq!(
                expected
                    .1
                    .iter()
                    .filter(|layer| layer.order == egui::Order::Tooltip)
                    .count(),
                4
            );
            assert_eq!(&state, expected, "retained Areas/layers grew");
            assert_eq!(
                ids.len(),
                2,
                "actual focus-widget ID set grew without cardinal navigation"
            );
            assert_eq!(path_ids.len(), 4, "full-path slot IDs grew");
        }
    }
}
