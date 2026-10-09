use super::*;

fn modal_frame(h: &mut Harness, events: Vec<Event>, text: &mut String) -> (Frame, Vec<Event>) {
    let mut available = Vec::new();
    let frame = h.frame_with_after(events, |ui| {
        egui::Modal::new(egui::Id::new("capture-blocking-modal")).show(ui.ctx(), |ui| {
            available = ui.input(|input| input.events.clone());
            ui.text_edit_singleline(text).request_focus();
        });
    });
    (frame, available)
}

#[test]
fn established_modal_blocks_captured_pan_and_zoom_without_resuming_old_capture() {
    for zoom in zoom_events() {
        let mut h = Harness::new(egui::emath::TSTransform::IDENTITY);
        let before = h.hundred_percent();
        let c = before.views[0].response.rect.center();
        h.frame(vec![Event::PointerMoved(c)]);
        let pressed = h.frame(vec![button(c, true)]);
        assert!(
            pressed.views[0]
                .response
                .drag_started_by(PointerButton::Primary)
        );
        assert_eq!(h.views[0].primary_drag, Some(pressed.views[0].response.id));
        let captured = h.frame(vec![Event::PointerMoved(c + vec2(10.0, 0.0))]);
        same_point(captured.views[0].image_rect.center(), c + vec2(10.0, 0.0));

        // Establish the modal on a stationary held-button frame before motion.
        // The background Ui is deliberately enabled, not disabled by an adapter.
        let mut text = String::new();
        let (stationary, _) = modal_frame(&mut h, vec![], &mut text);
        assert!(stationary.views[0].response.enabled());
        assert!(
            !h.ctx
                .memory(|memory| memory.allows_interaction(stationary.views[0].response.layer_id))
        );
        compare_view(&stationary.views[0], &captured.views[0]);
        let events = vec![
            Event::PointerMoved(c + vec2(30.0, 10.0)),
            zoom.clone(),
            Event::Text("modal text".into()),
        ];
        let (blocked, available) = modal_frame(&mut h, events.clone(), &mut text);
        assert!(blocked.views[0].response.enabled());
        assert!(blocked.views[0].response.dragged_by(PointerButton::Primary));
        compare_view(&blocked.views[0], &captured.views[0]);
        assert_eq!(h.views[0].primary_drag, None);
        assert_eq!(blocked.remaining_after_left, events);
        assert_eq!(available, events);
        assert_eq!(text, "modal text");

        // Modal permission settles after a no-modal frame. The old egui capture
        // still exists, but it must not re-admit this component's cancelled drag.
        let dismissed = h.frame(vec![]);
        compare_view(&dismissed.views[0], &captured.views[0]);
        let old_held = h.frame(vec![Event::PointerMoved(c + vec2(40.0, 15.0))]);
        assert!(
            h.ctx
                .memory(|memory| memory.allows_interaction(old_held.views[0].response.layer_id))
        );
        assert!(
            old_held.views[0]
                .response
                .dragged_by(PointerButton::Primary)
        );
        compare_view(&old_held.views[0], &captured.views[0]);
        assert_eq!(h.views[0].primary_drag, None);
        let released = h.frame(vec![button(c + vec2(40.0, 15.0), false)]);
        compare_view(&released.views[0], &captured.views[0]);

        h.frame(vec![Event::PointerMoved(c)]);
        let pressed = h.frame(vec![button(c, true)]);
        assert!(
            pressed.views[0]
                .response
                .drag_started_by(PointerButton::Primary)
        );
        assert_eq!(h.views[0].primary_drag, Some(pressed.views[0].response.id));
        let fresh = h.frame(vec![Event::PointerMoved(c + vec2(20.0, 5.0))]);
        same_point(fresh.views[0].image_rect.center(), c + vec2(30.0, 5.0));
        near(fresh.views[0].physical_scale, 1.0);
        h.frame(vec![button(c + vec2(20.0, 5.0), false)]);
    }
}
