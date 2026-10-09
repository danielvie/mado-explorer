use super::*;
use egui::{Context, Event, LayerId, Pos2, RawInput, Rect, Vec2};
use std::collections::HashSet;

fn key(n: u64) -> DocumentKey {
    DocumentKey::new(root().join(format!("document-{n}-長é𐐀.pdf")), n).unwrap()
}
fn root() -> PathBuf {
    PathBuf::from(if cfg!(windows) {
        r"C:\generated"
    } else {
        "/generated"
    })
}
fn input(events: Vec<Event>) -> RawInput {
    RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 500.0))),
        events,
        ..Default::default()
    }
}
fn frame(
    ctx: &Context,
    controls: &mut PdfControls,
    events: Vec<Event>,
) -> (ControlsOutput, egui::FullOutput) {
    let mut result = ControlsOutput::default();
    let output = ctx.run_ui(input(events), |ui| {
        let current = controls.show(ui);
        if current.request.is_some() {
            assert!(result.request.is_none());
            result.request = current.request;
        }
        result.owns_keyboard |= current.owns_keyboard;
    });
    (result, output)
}
fn run(ctx: &Context, controls: &mut PdfControls, events: Vec<Event>) -> ControlsOutput {
    let (result, output) = frame(ctx, controls, events);
    output.drop_without_applying_deltas();
    result
}
fn settle(ctx: &Context, controls: &mut PdfControls) {
    for _ in 0..3 {
        assert!(run(ctx, controls, vec![]).request.is_none());
    }
}
fn open(count: u64) -> (Context, PdfControls, TaggedPageRequest) {
    let ctx = Context::default();
    let mut controls = PdfControls::new(Id::new("pane-one"));
    controls.select(Some(key(1))).unwrap();
    controls
        .supply_metadata(&key(1), 1, DocumentState::Available { page_count: count })
        .unwrap();
    let first = run(&ctx, &mut controls, vec![]).request.unwrap();
    assert_eq!(first.page_index(), 0);
    settle(&ctx, &mut controls);
    (ctx, controls, first)
}
fn press(key: Key, pressed: bool) -> Event {
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
fn click(ctx: &Context, controls: &mut PdfControls, pos: Pos2) -> ControlsOutput {
    assert!(run(ctx, controls, pointer(pos, true)).request.is_none());
    run(ctx, controls, pointer(pos, false))
}
fn button(ctx: &Context, controls: &mut PdfControls, index: usize) -> ControlsOutput {
    let pos = controls.controls.as_ref().unwrap().0[index].rect.center();
    click(ctx, controls, pos)
}
fn draft(ctx: &Context, controls: &mut PdfControls, text: &str) {
    let pos = controls.controls.as_ref().unwrap().1.rect.center();
    assert!(click(ctx, controls, pos).request.is_none());
    let id = controls.controls.as_ref().unwrap().1.id;
    assert!(ctx.memory(|m| m.has_focus(id)));
    let select_all = Event::Key {
        key: Key::A,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::COMMAND,
    };
    assert!(
        run(
            ctx,
            controls,
            vec![
                select_all,
                if text.is_empty() {
                    press(Key::Backspace, true)
                } else {
                    Event::Text(text.into())
                }
            ]
        )
        .request
        .is_none()
    );
    assert_eq!(controls.editor(), text);
}
fn submit(ctx: &Context, controls: &mut PdfControls) -> ControlsOutput {
    let result = run(ctx, controls, vec![press(Key::Enter, true)]);
    assert!(
        run(ctx, controls, vec![press(Key::Enter, false)])
            .request
            .is_none()
    );
    result
}
fn has_text(shape: &egui::Shape, wanted: &str) -> bool {
    match shape {
        egui::Shape::Text(text) => text.galley.text() == wanted,
        egui::Shape::Vec(shapes) => shapes.iter().any(|s| has_text(s, wanted)),
        _ => false,
    }
}
fn text_present(output: &egui::FullOutput, wanted: &str) -> bool {
    output.shapes.iter().any(|s| has_text(&s.shape, wanted))
}

#[test]
fn actual_previous_next_buttons_keep_requests_within_bounds() {
    let (ctx, mut controls, first) = open(3);
    assert_eq!(controls.page_count(), Some(3));
    assert_eq!(controls.requested_page(), 0);
    assert_eq!(controls.confirmed_page(), None);
    assert!(controls.pending().is_some());
    assert!(!controls.controls.as_ref().unwrap().0[0].enabled());
    assert!(button(&ctx, &mut controls, 0).request.is_none());
    let next = button(&ctx, &mut controls, 1).request.unwrap();
    assert_eq!(next.page_index(), 1);
    assert_eq!(next.target().document(), first.target().document());
    assert_eq!(
        next.target().metadata_revision(),
        first.target().metadata_revision()
    );
    assert_eq!(next.target().page_revision(), next.sequence());
    assert_ne!(
        next.target().page_revision(),
        first.target().page_revision()
    );
    assert!(next.sequence() > first.sequence());
    assert_eq!(
        button(&ctx, &mut controls, 1).request.unwrap().page_index(),
        2
    );
    run(&ctx, &mut controls, vec![]);
    assert!(!controls.controls.as_ref().unwrap().0[1].enabled());
    assert!(button(&ctx, &mut controls, 1).request.is_none());
    assert_eq!(
        button(&ctx, &mut controls, 0).request.unwrap().page_index(),
        1
    );
    assert_eq!(
        button(&ctx, &mut controls, 0).request.unwrap().page_index(),
        0
    );
    assert!(run(&ctx, &mut controls, vec![]).request.is_none());
}

#[test]
fn direct_input_go_and_enter_use_one_based_numbers_and_preserve_invalid_drafts() {
    let (ctx, mut controls, _) = open(4);
    for invalid in [
        "",
        "0",
        "5",
        "-1",
        "1.5",
        "words",
        "18446744073709551616",
        "000000000000000000000000000000029",
        " 2",
        "2 ",
    ] {
        draft(&ctx, &mut controls, invalid);
        assert!(submit(&ctx, &mut controls).request.is_none());
        assert_eq!(controls.editor(), invalid);
        assert_eq!(
            controls.editor_error(),
            Some("Enter a whole page number from 1 to 4")
        );
        assert_eq!(controls.requested_page(), 0);
        assert!(ctx.memory(|m| m.has_focus(controls.controls.as_ref().unwrap().1.id)));
        let (_, output) = frame(&ctx, &mut controls, vec![]);
        assert!(text_present(
            &output,
            "Enter a whole page number from 1 to 4"
        ));
        output.drop_without_applying_deltas();
    }
    draft(&ctx, &mut controls, "4");
    assert_eq!(submit(&ctx, &mut controls).request.unwrap().page_index(), 3);
    assert!(controls.editor_error().is_none());
    draft(&ctx, &mut controls, "2");
    assert_eq!(
        button(&ctx, &mut controls, 2).request.unwrap().page_index(),
        1
    );
    assert_eq!(controls.editor(), "2");
    draft(&ctx, &mut controls, "00");
    assert!(button(&ctx, &mut controls, 2).request.is_none());
    assert_eq!(controls.editor(), "00");
}

#[test]
fn unavailable_supplied_states_never_claim_zero_pages_or_emit_requests() {
    for (state, wanted) in [
        (DocumentState::Loading, "Loading PDF metadata"),
        (DocumentState::MissingRenderer, "PDF renderer unavailable"),
        (DocumentState::Malformed, "Malformed PDF document"),
        (
            DocumentState::PasswordRequired,
            "PDF requires a password; password entry is not supported",
        ),
        (
            DocumentState::Failure("generated failure".into()),
            "PDF preview failed: generated failure",
        ),
        (
            DocumentState::Available { page_count: 0 },
            "PDF preview failed: Invalid PDF page count supplied",
        ),
        (
            DocumentState::Available {
                page_count: u64::MAX,
            },
            "PDF preview failed: Invalid PDF page count supplied",
        ),
        (
            DocumentState::Available {
                page_count: u32::MAX as u64 + 1,
            },
            "PDF preview failed: Invalid PDF page count supplied",
        ),
    ] {
        let ctx = Context::default();
        let mut controls = PdfControls::new(Id::new("fallback"));
        controls.select(Some(key(1))).unwrap();
        controls.supply_metadata(&key(1), 1, state).unwrap();
        for _ in 0..3 {
            let (result, output) = frame(&ctx, &mut controls, vec![]);
            assert!(result.request.is_none());
            assert!(text_present(&output, wanted));
            assert!(text_present(&output, "Page count unavailable"));
            assert!(!text_present(&output, "Pages: 0"));
            assert_eq!(controls.page_count(), None);
            assert!(
                controls
                    .controls
                    .as_ref()
                    .unwrap()
                    .0
                    .iter()
                    .all(|b| !b.enabled())
            );
            assert!(!controls.controls.as_ref().unwrap().1.enabled());
            output.drop_without_applying_deltas();
        }
        assert_eq!(
            controls.request_page(&controls.target().unwrap(), PageAction::Next),
            Err(Refusal::MetadataUnavailable)
        );
    }
}

#[test]
fn u32_limit_and_sequence_exhaustion_fail_without_overflow() {
    let (ctx, mut controls, _) = open(u32::MAX as u64);
    draft(&ctx, &mut controls, &u32::MAX.to_string());
    assert_eq!(
        submit(&ctx, &mut controls).request.unwrap().page_index(),
        u32::MAX - 1
    );
    assert_eq!(controls.editor(), &u32::MAX.to_string());
    let target = controls.target().unwrap();
    assert_eq!(
        controls.request_page(&target, PageAction::Next),
        Err(Refusal::InvalidPage)
    );
    assert_eq!(
        controls.request_page(&target, PageAction::Direct(u64::MAX)),
        Err(Refusal::InvalidPage)
    );
    controls.sequence = u64::MAX;
    let exhausted_target = controls.target().unwrap();
    assert_eq!(
        controls.request_page(&exhausted_target, PageAction::Previous),
        Err(Refusal::RequestSequenceExhausted)
    );
    assert_eq!(controls.requested_page(), u32::MAX - 1);
}

#[test]
fn document_change_resets_first_page_and_same_key_does_not_reset() {
    let (ctx, mut controls, _) = open(8);
    let target = controls.target().unwrap();
    let third = controls
        .request_page(&target, PageAction::Direct(3))
        .unwrap();
    controls
        .supply_render(&third, RenderResult::Displayed)
        .unwrap();
    controls.select(Some(key(1))).unwrap();
    assert_eq!(controls.requested_page(), 2);
    assert_eq!(controls.confirmed_page(), Some(2));
    controls.select(Some(key(2))).unwrap();
    assert_eq!(controls.requested_page(), 0);
    assert_eq!(controls.confirmed_page(), None);
    assert!(run(&ctx, &mut controls, vec![]).request.is_none());
    controls
        .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 2 })
        .unwrap();
    let first = run(&ctx, &mut controls, vec![]).request.unwrap();
    assert_eq!(first.page_index(), 0);
    assert_eq!(first.target().document(), &key(2));
    assert_eq!(controls.editor(), "1");
    assert!(first.sequence() > third.sequence());
}

#[test]
fn changed_metadata_limits_revalidate_page_and_refuse_old_target() {
    let (ctx, mut controls, _) = open(8);
    let old = controls.target().unwrap();
    controls.request_page(&old, PageAction::Direct(8)).unwrap();
    controls
        .supply_metadata(&key(1), 2, DocumentState::Available { page_count: 2 })
        .unwrap();
    assert_eq!(controls.requested_page(), 0);
    assert_eq!(
        controls.request_page(&old, PageAction::Next),
        Err(Refusal::StaleAction)
    );
    assert_eq!(
        run(&ctx, &mut controls, vec![])
            .request
            .unwrap()
            .page_index(),
        0
    );
    settle(&ctx, &mut controls);
    assert_eq!(
        button(&ctx, &mut controls, 1).request.unwrap().page_index(),
        1
    );
    controls
        .supply_metadata(&key(1), 3, DocumentState::Available { page_count: 5 })
        .unwrap();
    assert_eq!(controls.requested_page(), 1);
    assert_eq!(
        run(&ctx, &mut controls, vec![])
            .request
            .unwrap()
            .page_index(),
        1
    );
    assert_eq!(
        controls.supply_metadata(&key(1), 2, DocumentState::Available { page_count: 9 }),
        Err(Refusal::StaleMetadata)
    );
    assert_eq!(controls.page_count(), Some(5));
}

#[test]
fn captured_page_action_cannot_apply_again_after_a_newer_page_request() {
    let (_, mut controls, _) = open(4);
    let captured = controls.target().unwrap();
    let next = controls.request_page(&captured, PageAction::Next).unwrap();
    assert_eq!(next.page_index(), 1);
    assert_eq!(
        controls.request_page(&captured, PageAction::Next),
        Err(Refusal::StaleAction)
    );
    assert_eq!(controls.requested_page(), 1);
    assert_eq!(controls.pending(), Some(&next));
    let fresh = controls.target().unwrap();
    assert_eq!(
        controls
            .request_page(&fresh, PageAction::Next)
            .unwrap()
            .page_index(),
        2
    );
}

#[test]
fn stale_metadata_actions_and_selection_cannot_retarget() {
    let (ctx, mut controls, first) = open(4);
    let old = controls.target().unwrap();
    controls.select(Some(key(2))).unwrap();
    controls
        .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 3 })
        .unwrap();
    let new = run(&ctx, &mut controls, vec![]).request.unwrap();
    assert_eq!(
        controls.supply_metadata(&key(1), 99, DocumentState::Malformed),
        Err(Refusal::StaleDocument)
    );
    assert_eq!(
        controls.request_page(&old, PageAction::Direct(3)),
        Err(Refusal::StaleAction)
    );
    assert_eq!(
        controls.supply_render(&first, RenderResult::Displayed),
        Err(Refusal::StaleRender)
    );
    assert_eq!(controls.select(Some(key(1))), Err(Refusal::StaleDocument));
    assert_eq!(controls.pending(), Some(&new));
    controls.select(None).unwrap();
    assert_eq!(
        controls.supply_metadata(&key(2), 2, DocumentState::Malformed),
        Err(Refusal::StaleDocument)
    );
    assert_eq!(controls.select(Some(key(2))), Err(Refusal::StaleDocument));
    let (result, output) = frame(&ctx, &mut controls, vec![]);
    assert!(result.request.is_none());
    assert!(text_present(&output, "No PDF selected"));
    output.drop_without_applying_deltas();
}

#[test]
fn requested_pending_and_confirmed_displayed_pages_are_distinct() {
    let (ctx, mut controls, first) = open(3);
    controls
        .supply_render(&first, RenderResult::Displayed)
        .unwrap();
    let next = button(&ctx, &mut controls, 1).request.unwrap();
    assert_eq!(controls.confirmed_page(), Some(0));
    assert_eq!(controls.requested_page(), 1);
    let (_, output) = frame(&ctx, &mut controls, vec![]);
    assert!(text_present(&output, "Requested page: 2 / 3"));
    assert!(text_present(&output, "Displayed page: 1"));
    assert!(text_present(&output, "Rendering pending for page 2"));
    output.drop_without_applying_deltas();
    assert_eq!(
        controls.supply_render(&first, RenderResult::Failure("late".into())),
        Err(Refusal::StaleRender)
    );
    controls
        .supply_render(
            &next,
            RenderResult::Failure("generated renderer failure".into()),
        )
        .unwrap();
    assert_eq!(controls.confirmed_page(), Some(0));
    assert!(controls.pending().is_none());
    let (_, output) = frame(&ctx, &mut controls, vec![]);
    assert!(text_present(
        &output,
        "Rendering failed: generated renderer failure"
    ));
    assert!(text_present(&output, "Pages: 3"));
    output.drop_without_applying_deltas();
    let retry = button(&ctx, &mut controls, 2).request.unwrap();
    assert_ne!(retry.sequence(), next.sequence());
    assert_eq!(
        controls.supply_render(&next, RenderResult::Displayed),
        Err(Refusal::StaleRender)
    );
    controls
        .supply_render(&retry, RenderResult::Displayed)
        .unwrap();
    assert_eq!(controls.confirmed_page(), Some(1));
    assert_eq!(
        controls.supply_render(&retry, RenderResult::Displayed),
        Err(Refusal::StaleRender)
    );
}

#[test]
fn owned_button_keyboard_and_tab_navigation_work_without_window_shortcuts() {
    let (ctx, mut controls, _) = open(3);
    let ids = controls.control_ids();
    let (buttons, editor, _) = controls.controls.as_ref().unwrap();
    assert_eq!(
        ids,
        [buttons[0].id, buttons[1].id, buttons[2].id, editor.id]
    );
    ctx.memory_mut(|m| m.request_focus(buttons[1].id));
    assert_eq!(submit(&ctx, &mut controls).request.unwrap().page_index(), 1);
    assert!(
        run(&ctx, &mut controls, vec![press(Key::Tab, true)])
            .request
            .is_none()
    );
    run(&ctx, &mut controls, vec![press(Key::Tab, false)]);
    assert!(ctx.memory(|m| m.has_focus(controls.controls.as_ref().unwrap().1.id)));
    assert!(
        run(&ctx, &mut controls, vec![press(Key::Tab, true)])
            .request
            .is_none()
    );
    run(&ctx, &mut controls, vec![press(Key::Tab, false)]);
    assert!(ctx.memory(|m| m.has_focus(controls.controls.as_ref().unwrap().0[2].id)));
    let go = run(&ctx, &mut controls, vec![press(Key::Space, true)]);
    assert_eq!(go.request.unwrap().page_index(), 1);
    assert!(go.owns_keyboard);
}

#[test]
fn unrelated_text_editor_keeps_focus_and_input_across_pdf_changes() {
    let (ctx, mut controls, _) = open(3);
    let unrelated = Id::new("unrelated-editor");
    let mut text = String::new();
    let mut run_other = |events| {
        let mut result = ControlsOutput::default();
        let output = ctx.run_ui(input(events), |ui| {
            result = controls.show(ui);
            ui.add(egui::TextEdit::singleline(&mut text).id(unrelated));
        });
        output.drop_without_applying_deltas();
        result
    };
    run_other(vec![]);
    ctx.memory_mut(|m| m.request_focus(unrelated));
    let result = run_other(vec![
        Event::Text("other text".into()),
        press(Key::Space, true),
    ]);
    assert!(!result.owns_keyboard);
    assert!(result.request.is_none());
    drop(run_other);
    assert_eq!(text, "other text");
    controls.select(Some(key(2))).unwrap();
    controls
        .supply_metadata(&key(2), 1, DocumentState::MissingRenderer)
        .unwrap();
    let output = ctx.run_ui(
        input(vec![
            Event::Text(" survives".into()),
            press(Key::Enter, true),
        ]),
        |ui| {
            let result = controls.show(ui);
            assert!(!result.owns_keyboard);
            assert!(result.request.is_none());
            ui.add(
                egui::TextEdit::singleline(&mut text)
                    .id(unrelated)
                    .return_key(None),
            );
        },
    );
    output.drop_without_applying_deltas();
    assert_eq!(text, "other text survives");
    assert!(ctx.memory(|m| m.has_focus(unrelated)));
}

fn two_frame(
    ctx: &Context,
    first: &mut PdfControls,
    second: &mut PdfControls,
    events: Vec<Event>,
) -> [ControlsOutput; 2] {
    let mut result = [ControlsOutput::default(), ControlsOutput::default()];
    let output = ctx.run_ui(input(events), |ui| {
        ui.columns(2, |panes| {
            result[0] = first.show(&mut panes[0]);
            result[1] = second.show(&mut panes[1]);
        });
    });
    output.drop_without_applying_deltas();
    result
}

#[test]
fn two_panes_share_neither_requests_edits_focus_nor_reset() {
    let ctx = Context::default();
    let mut first = PdfControls::new(Id::new("first"));
    let mut second = PdfControls::new(Id::new("second"));
    for controls in [&mut first, &mut second] {
        controls.select(Some(key(1))).unwrap();
        controls
            .supply_metadata(&key(1), 1, DocumentState::Available { page_count: 4 })
            .unwrap();
    }
    let initial = two_frame(&ctx, &mut first, &mut second, vec![]);
    assert!(initial.iter().all(|r| r.request.is_some()));
    let other_request = initial[1].request.as_ref().unwrap();
    assert_eq!(
        first.supply_render(other_request, RenderResult::Displayed),
        Err(Refusal::StaleRender)
    );
    assert_eq!(
        first.request_page(other_request.target(), PageAction::Next),
        Err(Refusal::StaleAction)
    );
    for _ in 0..3 {
        two_frame(&ctx, &mut first, &mut second, vec![]);
    }
    let first_ids = first.control_ids();
    let second_ids = second.control_ids();
    assert!(first_ids.iter().all(|id| !second_ids.contains(id)));
    ctx.memory_mut(|m| m.request_focus(second.controls.as_ref().unwrap().0[1].id));
    let outputs = two_frame(&ctx, &mut first, &mut second, vec![press(Key::Enter, true)]);
    assert!(outputs[0].request.is_none());
    assert_eq!(outputs[1].request.as_ref().unwrap().page_index(), 1);
    assert!(!outputs[0].owns_keyboard);
    assert!(outputs[1].owns_keyboard);
    first.select(Some(key(2))).unwrap();
    first
        .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 2 })
        .unwrap();
    let outputs = two_frame(
        &ctx,
        &mut first,
        &mut second,
        vec![press(Key::Enter, false)],
    );
    assert_eq!(outputs[0].request.as_ref().unwrap().page_index(), 0);
    assert!(outputs[1].request.is_none());
    assert!(ctx.memory(|m| m.has_focus(second.controls.as_ref().unwrap().0[1].id)));
    assert_eq!(second.requested_page(), 1);
    let outputs = two_frame(&ctx, &mut first, &mut second, vec![press(Key::Enter, true)]);
    assert!(outputs[0].request.is_none());
    assert_eq!(outputs[1].request.as_ref().unwrap().page_index(), 2);
}

#[test]
fn disabled_or_modal_blocked_pdf_controls_do_not_consume_activation_input() {
    for modal in [false, true] {
        let (ctx, mut controls, _) = open(3);
        let modal_id = Id::new("other-modal");
        if modal {
            for _ in 0..3 {
                ctx.run_ui(input(vec![]), |ui| {
                    controls.show(ui);
                    egui::Modal::new(modal_id).show(ui.ctx(), |ui| {
                        ui.label("Other owner");
                    });
                })
                .drop_without_applying_deltas();
            }
        }
        ctx.memory_mut(|m| m.request_focus(controls.control_ids()[1]));
        let mut observed = false;
        let output = ctx.run_ui(input(vec![press(Key::Enter, true)]), |ui| {
            let result = if modal {
                controls.show(ui)
            } else {
                ui.add_enabled_ui(false, |ui| controls.show(ui)).inner
            };
            assert!(result.request.is_none());
            assert!(!result.owns_keyboard);
            if modal {
                egui::Modal::new(modal_id).show(ui.ctx(), |ui| {
                    observed |= ui.input(|i| i.key_pressed(Key::Enter));
                    ui.label("Other owner");
                });
            } else {
                observed |= ui.input(|i| i.key_pressed(Key::Enter));
            }
        });
        output.drop_without_applying_deltas();
        assert!(observed, "another owner must receive its activation key");
        assert_eq!(controls.requested_page(), 0);
    }
}

#[test]
fn old_pointer_release_cannot_change_page_or_restore_owned_focus() {
    for recreate in [false, true] {
        for control in 0..4 {
            let (ctx, mut controls, _) = open(4);
            let pos = if control < 3 {
                controls.controls.as_ref().unwrap().0[control].rect.center()
            } else {
                controls.controls.as_ref().unwrap().1.rect.center()
            };
            run(&ctx, &mut controls, pointer(pos, true));
            if recreate {
                controls = PdfControls::new(Id::new("pane-one"));
            }
            controls.select(Some(key(2))).unwrap();
            controls
                .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 4 })
                .unwrap();
            assert_eq!(
                run(&ctx, &mut controls, vec![])
                    .request
                    .unwrap()
                    .page_index(),
                0
            );
            settle(&ctx, &mut controls);
            assert!(
                run(&ctx, &mut controls, pointer(pos, false))
                    .request
                    .is_none()
            );
            assert_eq!(controls.requested_page(), 0);
            assert!(
                !ctx.memory(|m| m.focused())
                    .is_some_and(|id| controls.control_ids().contains(&id))
            );
            assert_eq!(
                button(&ctx, &mut controls, 1).request.unwrap().page_index(),
                1
            );
        }
    }
}

#[test]
fn opening_keys_focus_and_held_enter_do_not_change_new_document() {
    for control in 0..4 {
        let (ctx, mut controls, _) = open(4);
        ctx.memory_mut(|m| m.request_focus(controls.control_ids()[control]));
        run(&ctx, &mut controls, vec![press(Key::Enter, true)]);
        controls.select(Some(key(2))).unwrap();
        controls
            .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 4 })
            .unwrap();
        let result = run(
            &ctx,
            &mut controls,
            vec![
                press(Key::Tab, true),
                Event::Text("4".into()),
                press(Key::Enter, true),
            ],
        );
        assert_eq!(result.request.unwrap().page_index(), 0);
        settle(&ctx, &mut controls);
        assert!(
            !ctx.memory(|m| m.focused())
                .is_some_and(|id| controls.control_ids().contains(&id))
        );
        ctx.memory_mut(|m| m.request_focus(controls.controls.as_ref().unwrap().0[1].id));
        let mut repeat = press(Key::Enter, true);
        if let Event::Key { repeat, .. } = &mut repeat {
            *repeat = true;
        }
        assert!(run(&ctx, &mut controls, vec![repeat]).request.is_none());
        assert!(
            run(&ctx, &mut controls, vec![press(Key::Enter, true)])
                .request
                .is_none()
        );
        assert_eq!(controls.requested_page(), 0);
        run(&ctx, &mut controls, vec![press(Key::Enter, false)]);
        assert_eq!(submit(&ctx, &mut controls).request.unwrap().page_index(), 1);
    }
}

#[test]
fn opening_release_press_and_held_activation_keys_remain_disarmed() {
    for activation in [Key::Enter, Key::Space] {
        let (ctx, mut controls, _) = open(4);
        controls.select(Some(key(2))).unwrap();
        controls
            .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 4 })
            .unwrap();
        let first = run(
            &ctx,
            &mut controls,
            vec![press(activation, false), press(activation, true)],
        )
        .request
        .unwrap();
        assert_eq!(first.page_index(), 0);
        settle(&ctx, &mut controls);
        ctx.memory_mut(|m| m.request_focus(controls.control_ids()[1]));
        assert!(
            run(&ctx, &mut controls, vec![press(activation, true)])
                .request
                .is_none()
        );
        run(&ctx, &mut controls, vec![press(activation, false)]);
        assert_eq!(
            run(&ctx, &mut controls, vec![press(activation, true)])
                .request
                .unwrap()
                .page_index(),
            1
        );
    }
}

#[test]
fn invalid_draft_survives_metadata_limit_change_and_automatic_request() {
    let (ctx, mut controls, _) = open(8);
    draft(&ctx, &mut controls, "9");
    assert!(submit(&ctx, &mut controls).request.is_none());
    controls
        .supply_metadata(&key(1), 2, DocumentState::Available { page_count: 2 })
        .unwrap();
    assert_eq!(
        run(&ctx, &mut controls, vec![])
            .request
            .unwrap()
            .page_index(),
        0
    );
    assert_eq!(controls.editor(), "9");
    assert_eq!(
        controls.editor_error(),
        Some("Enter a whole page number from 1 to 2")
    );
    settle(&ctx, &mut controls);
    draft(&ctx, &mut controls, "2");
    assert_eq!(submit(&ctx, &mut controls).request.unwrap().page_index(), 1);
}

#[test]
fn opening_press_delayed_release_and_button_layout_retries_are_guarded() {
    let (ctx, mut controls, _) = open(4);
    let pos = controls.controls.as_ref().unwrap().0[1].rect.center();
    controls.select(Some(key(2))).unwrap();
    controls
        .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 4 })
        .unwrap();
    assert_eq!(
        run(&ctx, &mut controls, pointer(pos, true))
            .request
            .unwrap()
            .page_index(),
        0
    );
    settle(&ctx, &mut controls);
    assert!(
        run(&ctx, &mut controls, pointer(pos, false))
            .request
            .is_none()
    );
    assert_eq!(controls.requested_page(), 0);
    let pos = controls.controls.as_ref().unwrap().0[1].rect.center();
    run(&ctx, &mut controls, pointer(pos, true));
    let mut requests = Vec::new();
    let mut passes = 0;
    let output = ctx.run_ui(input(pointer(pos, false)), |ui| {
        if let Some(request) = controls.show(ui).request {
            requests.push(request);
        }
        passes += 1;
        if ui.ctx().current_pass_index() == 0 {
            ui.ctx().request_discard("button request retry");
        }
    });
    output.drop_without_applying_deltas();
    assert_eq!(passes, 2);
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].page_index(), 1);
}

#[test]
fn metadata_epoch_invalidates_old_press_and_stale_page_result() {
    let (ctx, mut controls, first) = open(4);
    let pos = controls.controls.as_ref().unwrap().0[1].rect.center();
    run(&ctx, &mut controls, pointer(pos, true));
    controls
        .supply_metadata(&key(1), 2, DocumentState::Available { page_count: 2 })
        .unwrap();
    let new = run(&ctx, &mut controls, vec![]).request.unwrap();
    settle(&ctx, &mut controls);
    assert!(
        run(&ctx, &mut controls, pointer(pos, false))
            .request
            .is_none()
    );
    assert_eq!(controls.requested_page(), 0);
    assert_eq!(
        controls.supply_render(&first, RenderResult::Displayed),
        Err(Refusal::StaleRender)
    );
    controls
        .supply_render(&new, RenderResult::Displayed)
        .unwrap();
}

#[test]
fn stale_release_cannot_restore_editor_or_button_focus_on_layout_retry() {
    for target in [1, 3] {
        let (ctx, mut controls, _) = open(4);
        let pos = if target == 3 {
            controls.controls.as_ref().unwrap().1.rect.center()
        } else {
            controls.controls.as_ref().unwrap().0[target].rect.center()
        };
        run(&ctx, &mut controls, pointer(pos, true));
        controls.select(Some(key(2))).unwrap();
        controls
            .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 4 })
            .unwrap();
        assert_eq!(
            run(&ctx, &mut controls, vec![])
                .request
                .unwrap()
                .page_index(),
            0
        );
        settle(&ctx, &mut controls);
        let output = ctx.run_ui(input(pointer(pos, false)), |ui| {
            assert!(controls.show(ui).request.is_none());
            if ui.ctx().current_pass_index() == 0 {
                ui.ctx().request_discard("stale release retry");
            }
        });
        output.drop_without_applying_deltas();
        assert!(
            !ctx.memory(|m| m.focused())
                .is_some_and(|id| controls.control_ids().contains(&id)),
            "no owned focus from stale release, target {target}"
        );
        assert_eq!(controls.requested_page(), 0);
    }
}

#[test]
fn layout_retries_emit_one_request_and_edit_text_once() {
    let (ctx, mut controls, _) = open(4);
    draft(&ctx, &mut controls, "3");
    let mut requests = Vec::new();
    let mut passes = 0;
    let output = ctx.run_ui(input(vec![press(Key::Enter, true)]), |ui| {
        if let Some(request) = controls.show(ui).request {
            requests.push(request);
        }
        passes += 1;
        if ui.ctx().current_pass_index() == 0 {
            ui.ctx().request_discard("page request retry");
        }
    });
    output.drop_without_applying_deltas();
    assert_eq!(passes, 2);
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].page_index(), 2);
    assert!(ctx.memory(|m| m.has_focus(controls.controls.as_ref().unwrap().1.id)));
    let output = ctx.run_ui(input(vec![Event::Text("1".into())]), |ui| {
        assert!(controls.show(ui).request.is_none());
        if ui.ctx().current_pass_index() == 0 {
            ui.ctx().request_discard("text retry");
        }
    });
    output.drop_without_applying_deltas();
    assert_eq!(controls.editor(), "31");
    controls.select(Some(key(2))).unwrap();
    controls
        .supply_metadata(&key(2), 1, DocumentState::Available { page_count: 5 })
        .unwrap();
    let mut first_requests = Vec::new();
    let output = ctx.run_ui(input(vec![]), |ui| {
        if let Some(request) = controls.show(ui).request {
            first_requests.push(request);
        }
        if ui.ctx().current_pass_index() == 0 {
            ui.ctx().request_discard("initial request retry");
        }
    });
    output.drop_without_applying_deltas();
    assert_eq!(first_requests.len(), 1);
    assert_eq!(first_requests[0].page_index(), 0);
}

#[test]
fn repeated_documents_and_hover_have_bounded_presenter_and_focus_ids() {
    let ctx = Context::default();
    ctx.global_style_mut(|style| {
        style.interaction.tooltip_delay = 0.0;
        style.interaction.show_tooltips_only_when_still = false;
    });
    let mut controls = PdfControls::new(Id::new("bounded-pane"));
    let mut ids = HashSet::new();
    let mut layers = None;
    for n in 1..=128 {
        // Recreating with the same pane salt must also remain bounded.
        if n % 16 == 0 {
            controls = PdfControls::new(Id::new("bounded-pane"));
        }
        let document =
            DocumentKey::new(root().join(format!("{}-{n}.pdf", "長é𐐀".repeat(120))), n).unwrap();
        controls.select(Some(document.clone())).unwrap();
        controls
            .supply_metadata(&document, 1, DocumentState::Available { page_count: 4 })
            .unwrap();
        assert_eq!(
            run(&ctx, &mut controls, vec![])
                .request
                .unwrap()
                .page_index(),
            0
        );
        settle(&ctx, &mut controls);
        let (buttons, editor, _) = controls.controls.as_ref().unwrap();
        ids.extend(buttons.iter().map(|b| b.id));
        ids.insert(editor.id);
        assert_eq!(
            ids.len(),
            4,
            "only four focus registration IDs, no cardinal cache pruning"
        );
        assert_eq!(
            controls.control_ids(),
            [buttons[0].id, buttons[1].id, buttons[2].id, editor.id]
        );
        for index in 0..2 {
            let pos = controls.controls.as_ref().unwrap().2[index].rect.center();
            let wanted = if index == 0 {
                document.path().file_name().unwrap().to_string_lossy()
            } else {
                document.path().to_string_lossy()
            };
            let mut found = false;
            for _ in 0..4 {
                let (_, output) = frame(&ctx, &mut controls, vec![Event::PointerMoved(pos)]);
                found |= text_present(&output, &wanted);
                output.drop_without_applying_deltas();
            }
            assert!(found, "real full identity tooltip {n}/{index}");
        }
        assert_eq!(
            button(&ctx, &mut controls, 1).request.unwrap().page_index(),
            1
        );
        let returned = controls.pending().unwrap();
        assert_eq!(returned.target().document().path(), document.path());
        controls.select(None).unwrap();
        run(&ctx, &mut controls, vec![Event::PointerGone]);
        run(&ctx, &mut controls, vec![]);
        let current: HashSet<LayerId> = ctx.memory(|m| m.layer_ids().collect());
        let areas: HashSet<LayerId> = current
            .iter()
            .copied()
            .filter(|l| egui::AreaState::load(&ctx, l.id).is_some())
            .collect();
        assert_eq!(
            areas
                .iter()
                .filter(|l| l.order == egui::Order::Tooltip)
                .count(),
            2
        );
        assert_eq!(
            *layers.get_or_insert_with(|| (current.clone(), areas.clone())),
            (current, areas)
        );
    }
}

#[test]
fn full_paths_required_and_native_identity_never_uses_display_text() {
    assert_eq!(
        DocumentKey::new(PathBuf::from("relative.pdf"), 1),
        Err(Refusal::FullPathRequired)
    );
    let path = root().join("長é𐐀".repeat(200)).join("complete.pdf");
    let document = DocumentKey::new(path.clone(), 1).unwrap();
    let ctx = Context::default();
    let mut controls = PdfControls::new(Id::new("native"));
    controls.select(Some(document.clone())).unwrap();
    controls
        .supply_metadata(&document, 1, DocumentState::Available { page_count: 1 })
        .unwrap();
    let request = run(&ctx, &mut controls, vec![]).request.unwrap();
    assert_eq!(request.target().document().path(), path);
    assert_eq!(controls.document().unwrap().path(), path);
    assert!(
        controls
            .controls
            .as_ref()
            .unwrap()
            .2
            .iter()
            .all(|r| r.rect.width() <= 800.0)
    );
}

#[cfg(windows)]
#[test]
fn unpaired_utf16_and_verbatim_path_stay_native_in_metadata_and_requests() {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    let mut units: Vec<u16> = r"\\?\C:\generated\".encode_utf16().collect();
    units.extend([0xD800, 0x0061, 0x002E, 0x0070, 0x0064, 0x0066]);
    let path = PathBuf::from(OsString::from_wide(&units));
    let document = DocumentKey::new(path, 1).unwrap();
    let ctx = Context::default();
    let mut controls = PdfControls::new(Id::new("native-surrogate"));
    controls.select(Some(document.clone())).unwrap();
    controls
        .supply_metadata(&document, 1, DocumentState::Available { page_count: 2 })
        .unwrap();
    let request = run(&ctx, &mut controls, vec![]).request.unwrap();
    assert_eq!(
        request
            .target()
            .document()
            .path()
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        units
    );
}
