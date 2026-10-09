use super::*;
use egui::{Event, LayerId, PointerButton, Pos2, RawInput, Rect, Vec2};
use std::collections::HashSet;

fn frame(ctx: &Context, dialog: &mut ConflictDialog, events: Vec<Event>) -> DialogOutput {
    run(ctx, dialog, events).0
}

fn run(
    ctx: &Context,
    dialog: &mut ConflictDialog,
    events: Vec<Event>,
) -> (DialogOutput, egui::FullOutput) {
    let mut result = DialogOutput::default();
    let mut output = ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 700.0))),
            events,
            ..Default::default()
        },
        |ui| {
            let current = dialog.show(ui.ctx());
            if current.decision.is_some() || !result.owns_input {
                result = current;
            }
        },
    );
    output.textures_delta.clear();
    (result, output)
}

fn request(operation: u64, conflict: u64, folders: bool) -> ConflictRequest {
    let root = if cfg!(windows) {
        r"C:\generated"
    } else {
        "/generated"
    };
    ConflictRequest::new(
        OperationId(operation),
        ConflictId(conflict),
        PathBuf::from(root).join(format!(
            "source-{}-{operation}-{conflict}.txt",
            "長é𐐀".repeat(100)
        )),
        PathBuf::from(root).join(format!(
            "destination-{}-{operation}-{conflict}.txt",
            "complete".repeat(100)
        )),
        ConflictType {
            source: if folders {
                EntryKind::Directory
            } else {
                EntryKind::RegularFile
            },
            destination: if folders {
                EntryKind::Directory
            } else {
                EntryKind::RegularFile
            },
        },
    )
    .unwrap()
}

fn retained(ctx: &Context) -> (HashSet<LayerId>, HashSet<LayerId>) {
    // Read retained order/map after hiding, not just visible layers.
    let layers: HashSet<_> = ctx.memory(|memory| memory.layer_ids().collect());
    let areas = layers
        .iter()
        .copied()
        .filter(|layer| egui::AreaState::load(ctx, layer.id).is_some())
        .collect();
    (layers, areas)
}

fn contains_text(shape: &egui::Shape, wanted: &str) -> bool {
    match shape {
        egui::Shape::Text(text) => text.galley.text() == wanted,
        egui::Shape::Vec(shapes) => shapes.iter().any(|shape| contains_text(shape, wanted)),
        _ => false,
    }
}

fn settle(ctx: &Context, dialog: &mut ConflictDialog) {
    for _ in 0..3 {
        assert!(frame(ctx, dialog, vec![]).decision.is_none());
    }
}

#[test]
fn retained_areas_and_layers_are_bounded_across_requests_and_controllers() {
    let ctx = Context::default();
    ctx.global_style_mut(|style| {
        style.interaction.tooltip_delay = 0.0;
        style.interaction.show_tooltips_only_when_still = false;
    });
    let mut baseline = None;
    for operation in 1..=12 {
        let mut dialog = ConflictDialog::new(OperationId(operation));
        for conflict in 1..=32 {
            let supplied = request(operation, conflict, conflict % 2 == 0);
            assert!(dialog.present(supplied.clone()).unwrap().is_none());
            settle(&ctx, &mut dialog);
            for index in 0..2 {
                let position = dialog.controls.as_ref().unwrap().2[index].rect.center();
                let wanted = if index == 0 {
                    supplied.source()
                } else {
                    supplied.destination()
                }
                .to_string_lossy();
                let mut found = false;
                for _ in 0..4 {
                    let output = run(&ctx, &mut dialog, vec![Event::PointerMoved(position)]).1;
                    found |= output
                        .shapes
                        .iter()
                        .any(|shape| contains_text(&shape.shape, &wanted));
                }
                assert!(
                    found,
                    "actual full-path tooltip must render: {operation}/{conflict}/{index}"
                );
            }
            dialog
                .decide(
                    OperationId(operation),
                    ConflictId(conflict),
                    Choice::Skip,
                    false,
                )
                .unwrap();
            frame(&ctx, &mut dialog, vec![Event::PointerGone]);
            frame(&ctx, &mut dialog, vec![]);
            let state = retained(&ctx);
            let expected = baseline.get_or_insert_with(|| state.clone());
            assert_eq!(
                expected
                    .0
                    .iter()
                    .filter(|layer| layer.order == egui::Order::Tooltip)
                    .count(),
                2
            );
            assert_eq!(
                expected
                    .1
                    .iter()
                    .filter(|layer| layer.order == egui::Order::Tooltip)
                    .count(),
                2,
                "both hidden tooltips must have retained AreaState"
            );
            assert!(
                expected
                    .1
                    .iter()
                    .any(|layer| layer.id == Id::new("mado-conflict-presenter"))
            );
            assert_eq!(
                &state, expected,
                "retained state grew at operation {operation}, conflict {conflict}"
            );
        }
        dialog.finish();
        frame(&ctx, &mut dialog, vec![]);
        assert_eq!(retained(&ctx), *baseline.as_ref().unwrap());
    }
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

#[test]
fn focusable_ids_are_bounded_without_cardinal_navigation() {
    let ctx = Context::default();
    let mut ids = HashSet::new();
    for operation in 1..=12 {
        let mut dialog = ConflictDialog::new(OperationId(operation));
        for conflict in 1..=32 {
            dialog
                .present(request(operation, conflict * 2 - 1, conflict % 2 == 0))
                .unwrap();
            settle(&ctx, &mut dialog);
            let (buttons, checkbox, _) = dialog.controls.as_ref().unwrap();
            ids.extend(buttons.iter().map(|button| button.id));
            ids.insert(checkbox.id);
            // These are the five widgets that register interest in egui's focus cache.
            // Only real pointer and Enter inputs, never cardinal navigation/cache pruning.
            let pos = buttons[0].rect.center();
            assert!(
                frame(&ctx, &mut dialog, pointer(pos, true))
                    .decision
                    .is_none()
            );
            assert_eq!(
                frame(&ctx, &mut dialog, pointer(pos, false))
                    .decision
                    .unwrap()
                    .choice(),
                Choice::Skip
            );
            dialog
                .present(request(operation, conflict * 2, false))
                .unwrap();
            settle(&ctx, &mut dialog);
            let (buttons, checkbox, _) = dialog.controls.as_ref().unwrap();
            ids.extend(buttons.iter().map(|button| button.id));
            ids.insert(checkbox.id);
            assert_eq!(
                frame(&ctx, &mut dialog, vec![key(Key::Enter, true)])
                    .decision
                    .unwrap()
                    .choice(),
                Choice::Skip
            );
            frame(&ctx, &mut dialog, vec![key(Key::Enter, false)]);
            assert_eq!(
                ids.len(),
                5,
                "presentation IDs must not depend on request or type"
            );
        }
    }
}

#[test]
fn delayed_old_pointer_release_cannot_answer_toggle_or_restore_replace_focus() {
    for replace_controller in [false, true] {
        for target in [0, 1, 2, 3, 4] {
            let ctx = Context::default();
            let mut dialog = ConflictDialog::new(OperationId(1));
            dialog.present(request(1, 1, false)).unwrap();
            settle(&ctx, &mut dialog);
            let pos = if target == 4 {
                dialog.controls.as_ref().unwrap().1.rect.center()
            } else {
                dialog.controls.as_ref().unwrap().0[target].rect.center()
            };
            let replace_id = dialog.controls.as_ref().unwrap().0[2].id;
            ctx.memory_mut(|memory| memory.request_focus(replace_id));
            assert!(
                frame(&ctx, &mut dialog, pointer(pos, true))
                    .decision
                    .is_none()
            );
            let operation = if replace_controller { 2 } else { 1 };
            if replace_controller {
                dialog = ConflictDialog::new(OperationId(operation));
            }
            let supplied = request(operation, 2, false);
            dialog.present(supplied.clone()).unwrap();
            settle(&ctx, &mut dialog); // release deliberately later than the fresh frame
            assert!(
                frame(&ctx, &mut dialog, pointer(pos, false))
                    .decision
                    .is_none()
            );
            assert!(!dialog.pending.as_ref().unwrap().apply);
            let skip_id = dialog.controls.as_ref().unwrap().0[0].id;
            assert!(
                ctx.memory(|memory| memory.has_focus(skip_id)),
                "old release must not focus Replace/checkbox"
            );
            assert!(
                frame(&ctx, &mut dialog, pointer(pos, true))
                    .decision
                    .is_none()
            );
            let result = frame(&ctx, &mut dialog, pointer(pos, false));
            if target == 4 {
                assert!(result.decision.is_none());
                assert!(dialog.pending.as_ref().unwrap().apply);
                let replace_id = dialog.controls.as_ref().unwrap().0[2].id;
                ctx.memory_mut(|memory| memory.request_focus(replace_id));
                let decision = frame(&ctx, &mut dialog, vec![key(Key::Space, true)])
                    .decision
                    .unwrap();
                assert_eq!(decision.choice(), Choice::Replace);
                assert!(decision.matches(&supplied));
            } else {
                let decision = result.decision.unwrap();
                assert_eq!(
                    decision.choice(),
                    [
                        Choice::Skip,
                        Choice::KeepBoth,
                        Choice::Replace,
                        Choice::Cancel
                    ][target]
                );
                assert!(decision.matches(&supplied));
            }
        }
    }
}

#[test]
fn checkbox_pointer_toggle_is_not_replayed_on_layout_retry() {
    let ctx = Context::default();
    let mut dialog = ConflictDialog::new(OperationId(1));
    dialog.present(request(1, 1, false)).unwrap();
    settle(&ctx, &mut dialog);
    let pos = dialog.controls.as_ref().unwrap().1.rect.center();
    assert!(
        frame(&ctx, &mut dialog, pointer(pos, true))
            .decision
            .is_none()
    );
    let mut passes = 0;
    ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 700.0))),
            events: pointer(pos, false),
            ..Default::default()
        },
        |ui| {
            assert!(dialog.show(ui.ctx()).decision.is_none());
            passes += 1;
            if ui.ctx().current_pass_index() == 0 {
                ui.ctx().request_discard("checkbox pointer retry");
            }
        },
    )
    .drop_without_applying_deltas();
    assert_eq!(passes, 2);
    assert!(
        dialog.pending.as_ref().unwrap().apply,
        "one click must toggle once, not once per pass"
    );
    assert!(
        frame(&ctx, &mut dialog, vec![key(Key::Enter, true)])
            .decision
            .unwrap()
            .apply_to_remaining()
    );
}

#[test]
fn opening_focus_navigation_cannot_schedule_inherited_focus() {
    for modifiers in [Modifiers::NONE, Modifiers::SHIFT] {
        let ctx = Context::default();
        let mut dialog = ConflictDialog::new(OperationId(1));
        dialog.present(request(1, 1, false)).unwrap();
        settle(&ctx, &mut dialog);
        ctx.memory_mut(|memory| memory.request_focus(dialog.controls.as_ref().unwrap().0[2].id));
        dialog.present(request(1, 2, false)).unwrap();
        assert!(
            frame(
                &ctx,
                &mut dialog,
                vec![Event::Key {
                    key: Key::Tab,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers,
                }]
            )
            .decision
            .is_none()
        );
        settle(&ctx, &mut dialog);
        let skip = dialog.controls.as_ref().unwrap().0[0].id;
        assert!(
            ctx.memory(|memory| memory.has_focus(skip)),
            "opening navigation must not schedule old focus"
        );
        assert_eq!(
            frame(&ctx, &mut dialog, vec![key(Key::Space, true)])
                .decision
                .unwrap()
                .choice(),
            Choice::Skip
        );
    }
}

#[test]
fn opening_press_and_space_cannot_carry_into_current_request() {
    for target in [2, 4] {
        let ctx = Context::default();
        let mut dialog = ConflictDialog::new(OperationId(1));
        dialog.present(request(1, 1, false)).unwrap();
        settle(&ctx, &mut dialog);
        let pos = if target == 4 {
            dialog.controls.as_ref().unwrap().1.rect.center()
        } else {
            dialog.controls.as_ref().unwrap().0[target].rect.center()
        };
        ctx.memory_mut(|memory| memory.request_focus(dialog.controls.as_ref().unwrap().0[2].id));
        dialog.present(request(1, 2, false)).unwrap();
        let mut events = pointer(pos, true);
        events.push(key(Key::Space, true));
        assert!(frame(&ctx, &mut dialog, events).decision.is_none());
        settle(&ctx, &mut dialog);
        assert!(
            frame(&ctx, &mut dialog, pointer(pos, false))
                .decision
                .is_none()
        );
        assert!(!dialog.pending.as_ref().unwrap().apply);
        frame(&ctx, &mut dialog, vec![key(Key::Space, false)]);
        let decision = frame(&ctx, &mut dialog, vec![key(Key::Space, true)])
            .decision
            .unwrap();
        assert_eq!(decision.choice(), Choice::Skip);
        assert_eq!(decision.request().conflict(), ConflictId(2));
    }
}
