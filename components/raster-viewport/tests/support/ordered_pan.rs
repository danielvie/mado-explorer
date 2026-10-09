use super::*;

#[test]
fn held_drag_pan_and_zoom_match_separate_frames_in_both_orders() {
    for transform in [
        egui::emath::TSTransform::IDENTITY,
        egui::emath::TSTransform::new(vec2(50.0, 30.0), 1.5),
    ] {
        for zoom in zoom_events() {
            for move_first in [true, false] {
                let mut batched = Harness::new(transform);
                let mut sequential = Harness::new(transform);
                let before = batched.hundred_percent();
                sequential.hundred_percent();
                let c = before.views[0].response.rect.center();
                for h in [&mut batched, &mut sequential] {
                    h.frame(vec![Event::PointerMoved(transform * c)]);
                    h.frame(vec![button(transform * c, true)]);
                }
                let motion = Event::PointerMoved(transform * (c + vec2(20.0, 10.0)));
                let events = if move_first {
                    vec![motion, zoom.clone()]
                } else {
                    vec![zoom.clone(), motion]
                };
                let after = batched.frame(events.clone());
                let mut reference = sequential.frame(vec![]);
                for event in events {
                    reference = sequential.frame(vec![event]);
                }
                compare_view(&after.views[0], &reference.views[0]);
                same_point(after.views[0].image_rect.center(), c + vec2(20.0, 10.0));
                let still = batched.frame(vec![]);
                compare_view(&still.views[0], &after.views[0]);
                assert_eq!(batched.views[1].mode(), ViewMode::Fit);
            }
        }
    }
}

#[test]
fn captured_pan_replays_press_release_and_clamps_each_motion() {
    for transform in [
        egui::emath::TSTransform::IDENTITY,
        egui::emath::TSTransform::new(vec2(50.0, 30.0), 1.5),
    ] {
        for held in [true, false] {
            let mut batched = Harness::new(transform);
            let mut sequential = Harness::new(transform);
            let before = batched.hundred_percent();
            sequential.hundred_percent();
            let c = before.views[0].response.rect.center();
            let start = c - vec2(40.0, 0.0);
            for h in [&mut batched, &mut sequential] {
                h.frame(vec![Event::PointerMoved(transform * start)]);
                if held {
                    h.frame(vec![button(transform * start, true)]);
                }
            }
            let mut events = vec![];
            if !held {
                events.extend([
                    Event::PointerMoved(transform * c),
                    Event::Zoom(2.0),
                    button(transform * c, true),
                ]);
            }
            events.extend([
                Event::PointerMoved(transform * (c + vec2(20.0, 0.0))),
                Event::Zoom(2.0),
                Event::PointerMoved(transform * (c + vec2(40.0, 0.0))),
            ]);
            // A captured release must include preceding motion, but none after release.
            if held {
                events.extend([
                    button(transform * (c + vec2(40.0, 0.0)), false),
                    Event::PointerMoved(transform * (c + vec2(80.0, 0.0))),
                    Event::Zoom(0.5),
                ]);
            }
            let after = batched.frame(events.clone());
            let mut reference = sequential.frame(vec![]);
            for event in events {
                reference = sequential.frame(vec![event]);
            }
            compare_view(&after.views[0], &reference.views[0]);
            let still = batched.frame(vec![]);
            compare_view(&still.views[0], &after.views[0]);
        }
        let mut batched = Harness::new(transform);
        let mut sequential = Harness::new(transform);
        let before = batched.hundred_percent();
        sequential.hundred_percent();
        let c = before.views[0].response.rect.center();
        for h in [&mut batched, &mut sequential] {
            h.frame(vec![Event::PointerMoved(transform * c)]);
            h.frame(vec![button(transform * c, true)]);
        }
        let events = vec![
            Event::PointerMoved(transform * (c + vec2(1000.0, 0.0))),
            Event::PointerMoved(transform * (c + vec2(20.0, 0.0))),
            Event::Zoom(2.0),
        ];
        let after = batched.frame(events.clone());
        let mut reference = sequential.frame(vec![]);
        for event in events {
            reference = sequential.frame(vec![event]);
        }
        compare_view(&after.views[0], &reference.views[0]);
    }
}

#[test]
fn ordered_pan_does_not_retarget_uncaptured_or_replaced_document() {
    let mut h = Harness::new(egui::emath::TSTransform::IDENTITY);
    let before = h.hundred_percent();
    let c = before.views[0].response.rect.center();
    let right = before.views[1].response.rect.center();
    for origin in [before.text.center(), right, Pos2::new(1500.0, 900.0)] {
        h.frame(vec![Event::PointerMoved(origin)]);
        h.frame(vec![button(origin, true)]);
        let after = h.frame(vec![Event::PointerMoved(c), Event::Zoom(2.0)]);
        // Zoom is pointer-targeted, but the other widget's captured motion is not pan.
        same_point(after.views[0].image_rect.center(), c);
        h.frame(vec![button(c, false)]);
        h.hundred_percent();
    }
    // A batch may end over this raster even though its press was elsewhere.
    let origin = before.text.center();
    h.frame(vec![Event::PointerMoved(origin)]);
    let after = h.frame(vec![button(origin, true), Event::PointerMoved(c)]);
    same_point(after.views[0].image_rect.center(), c);
    let after = h.frame(vec![Event::PointerMoved(c + vec2(20.0, 0.0))]);
    same_point(after.views[0].image_rect.center(), c);
    h.frame(vec![button(c, false)]);
    h.frame(vec![Event::PointerMoved(c)]);
    h.frame(vec![button(c, true)]);
    h.frame(vec![Event::PointerMoved(c + vec2(20.0, 0.0))]);
    h.keys[0] = 2;
    let reset = h.frame(vec![Event::PointerMoved(c + vec2(40.0, 0.0))]);
    assert_eq!(h.views[0].mode(), ViewMode::Fit);
    same_point(reset.views[0].image_rect.center(), c);
    let after = h.frame(vec![
        Event::Zoom(4.0),
        Event::PointerMoved(c + vec2(60.0, 0.0)),
    ]);
    let scale_only_center = c - vec2(120.0, 0.0);
    same_point(after.views[0].image_rect.center(), scale_only_center);
}

fn fresh_stationary_zoom(zoom: Event) {
    for transform in [
        egui::emath::TSTransform::IDENTITY,
        egui::emath::TSTransform::new(vec2(50.0, 30.0), 1.5),
    ] {
        let mut h = Harness::new(transform);
        h.frame(vec![]);
        let before = h.frame(vec![]);
        let c = before.views[0].response.rect.center();
        h.frame(vec![Event::PointerMoved(transform * c)]);
        h.views[0] = RasterViewport::default();
        let after = h.frame(vec![zoom.clone()]);
        near(
            after.views[0].physical_scale,
            before.views[0].physical_scale * 2.0,
        );
        assert!(!after.remaining_after_left.contains(&zoom));
        assert_eq!(h.views[1].mode(), ViewMode::Fit);
        same_point(after.views[0].image_rect.center(), c);

        for elsewhere in [
            before.text.center(),
            before.views[1].response.rect.center(),
            Pos2::new(900.0, 500.0),
        ] {
            h.frame(vec![Event::PointerMoved(transform * elsewhere)]);
            h.views[0] = RasterViewport::default();
            let after = h.frame(vec![zoom.clone()]);
            assert_eq!(h.views[0].mode(), ViewMode::Fit);
            assert!(after.remaining_after_left.contains(&zoom));
        }
        // Keep the old instance, hide it while the context pointer moves, then return
        // with no position-changing event. The old instance pointer must not win.
        h.frame(vec![Event::PointerMoved(transform * c)]);
        let output = h.ctx.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1600.0, 1000.0))),
                events: vec![Event::PointerMoved(transform * before.text.center())],
                ..Default::default()
            },
            |_| {},
        );
        output.drop_without_applying_deltas();
        let after = h.frame(vec![zoom.clone()]);
        assert_eq!(h.views[0].mode(), ViewMode::Fit);
        assert!(after.remaining_after_left.contains(&zoom));
        assert_eq!(
            h.ctx.input(|input| input.pointer.latest_pos()),
            Some(transform * before.text.center())
        );
        let output = h.ctx.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1600.0, 1000.0))),
                events: vec![Event::PointerMoved(transform * c)],
                ..Default::default()
            },
            |_| {},
        );
        output.drop_without_applying_deltas();
        let after = h.frame(vec![zoom.clone()]);
        near(
            after.views[0].physical_scale,
            before.views[0].physical_scale * 2.0,
        );
        assert!(!after.remaining_after_left.contains(&zoom));
    }
}

#[test]
fn fresh_instance_does_not_backfill_final_position_across_ordered_events() {
    for zoom in zoom_events() {
        let mut h = Harness::new(egui::emath::TSTransform::IDENTITY);
        h.frame(vec![]);
        let before = h.frame(vec![]);
        let c = before.views[0].response.rect.center();
        for events in [
            vec![zoom.clone(), Event::PointerMoved(c)],
            vec![zoom.clone(), button(c, true)],
            vec![Event::PointerGone, zoom.clone(), Event::PointerMoved(c)],
        ] {
            h.frame(vec![Event::PointerMoved(before.text.center())]);
            h.views[0] = RasterViewport::default();
            let after = h.frame(events);
            assert_eq!(h.views[0].mode(), ViewMode::Fit);
            assert!(after.remaining_after_left.contains(&zoom));
            h.frame(vec![button(c, false)]);
        }
        h.views[0] = RasterViewport::default();
        let after = h.frame(vec![
            Event::PointerMoved(c),
            zoom.clone(),
            Event::PointerGone,
        ]);
        near(
            after.views[0].physical_scale,
            before.views[0].physical_scale * 2.0,
        );
        assert!(!after.remaining_after_left.contains(&zoom));
        // PointerGone also prevents fallback to an old instance position in a
        // later stationary batch.
        let after = h.frame(vec![zoom.clone()]);
        near(
            after.views[0].physical_scale,
            before.views[0].physical_scale * 2.0,
        );
        assert!(after.remaining_after_left.contains(&zoom));
    }
}

#[test]
fn fresh_instance_native_zoom_uses_known_stationary_context_pointer() {
    fresh_stationary_zoom(zoom_events()[0].clone());
}

#[test]
fn fresh_instance_ctrl_wheel_uses_known_stationary_context_pointer() {
    fresh_stationary_zoom(zoom_events()[1].clone());
}
