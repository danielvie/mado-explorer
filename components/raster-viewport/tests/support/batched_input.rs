use super::*;
use egui::{ColorImage, Context, Modifiers, RawInput, TextureOptions, TouchPhase, vec2};

struct Harness {
    ctx: Context,
    texture: TextureHandle,
    views: [RasterViewport; 2],
    transform: egui::emath::TSTransform,
}

struct Frame {
    views: [ViewportOutput; 2],
    text: Rect,
    remaining_after_left: Vec<Event>,
}

impl Harness {
    fn new(transform: egui::emath::TSTransform) -> Self {
        let ctx = Context::default();
        let texture = ctx.load_texture(
            "generated-batched-input",
            ColorImage::filled([8, 4], Color32::LIGHT_BLUE),
            TextureOptions::NEAREST,
        );
        Self {
            ctx,
            texture,
            views: Default::default(),
            transform,
        }
    }

    fn frame(&mut self, events: Vec<Event>) -> Frame {
        let mut views = [None, None];
        let mut text = Rect::NOTHING;
        let mut remaining_after_left = Vec::new();
        let output = self.ctx.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1600.0, 1000.0))),
                events,
                ..Default::default()
            },
            |ui| {
                ui.ctx().set_transform_layer(ui.layer_id(), self.transform);
                text = ui.text_edit_singleline(&mut String::new()).rect;
                ui.horizontal(|ui| {
                    for (i, result) in views.iter_mut().enumerate() {
                        ui.allocate_ui(vec2(350.0, 500.0), |ui| {
                            *result = self.views[i].show(
                                ui,
                                i,
                                1,
                                &self.texture,
                                vec2(800.0, 400.0),
                                vec2(300.0, 200.0),
                            );
                        });
                        if i == 0 {
                            remaining_after_left = ui.input(|input| input.events.clone());
                        }
                    }
                });
            },
        );
        output.drop_without_applying_deltas();
        Frame {
            views: views.map(Option::unwrap),
            text,
            remaining_after_left,
        }
    }

    fn hundred_percent(&mut self) -> Frame {
        self.frame(vec![]);
        let frame = self.frame(vec![]);
        let pos = self.transform * frame.views[0].controls[1].rect.center();
        self.frame(vec![Event::PointerMoved(pos)]);
        self.frame(vec![button(pos, true)]);
        let frame = self.frame(vec![button(pos, false)]);
        near(frame.views[0].physical_scale, 1.0);
        frame
    }
}

fn button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

fn near(a: f32, b: f32) {
    assert!((a - b).abs() < 0.002, "{a} != {b}");
}

fn same_point(a: Pos2, b: Pos2) {
    near(a.x, b.x);
    near(a.y, b.y);
}

fn zoom_events() -> [Event; 2] {
    [
        Event::Zoom(2.0),
        Event::MouseWheel {
            unit: MouseWheelUnit::Line,
            delta: vec2(0.0, 2.0_f32.ln() / 0.2),
            modifiers: Modifiers::CTRL,
            phase: TouchPhase::Move,
        },
    ]
}

#[test]
fn batched_drag_start_excludes_pre_press_travel() {
    for transform in [
        egui::emath::TSTransform::IDENTITY,
        egui::emath::TSTransform::new(vec2(50.0, 30.0), 1.5),
    ] {
        let mut h = Harness::new(transform);
        let before = h.hundred_percent();
        let center = before.views[0].response.rect.center();
        h.frame(vec![Event::PointerMoved(
            transform * (center - vec2(80.0, 0.0)),
        )]);
        let pressed = h.frame(vec![
            Event::PointerMoved(transform * center),
            button(transform * center, true),
        ]);
        assert!(
            pressed.views[0]
                .response
                .drag_started_by(PointerButton::Primary)
        );
        same_point(pressed.views[0].image_rect.center(), center);
    }
}

#[test]
fn batched_drag_counts_post_press_motion_then_only_incremental_motion() {
    for transform in [
        egui::emath::TSTransform::IDENTITY,
        egui::emath::TSTransform::new(vec2(50.0, 30.0), 1.5),
    ] {
        let mut h = Harness::new(transform);
        let before = h.hundred_percent();
        let center = before.views[0].response.rect.center();
        h.frame(vec![Event::PointerMoved(
            transform * (center - vec2(80.0, 0.0)),
        )]);
        let pressed = h.frame(vec![
            Event::PointerMoved(transform * center),
            button(transform * center, true),
            Event::PointerMoved(transform * (center + vec2(20.0, 10.0))),
        ]);
        assert!(
            pressed.views[0]
                .response
                .drag_started_by(PointerButton::Primary)
        );
        same_point(
            pressed.views[0].image_rect.center(),
            center + vec2(20.0, 10.0),
        );
        let moved = h.frame(vec![Event::PointerMoved(
            transform * (center + vec2(35.0, 15.0)),
        )]);
        same_point(
            moved.views[0].image_rect.center(),
            center + vec2(35.0, 15.0),
        );
        let still = h.frame(vec![]);
        same_point(
            still.views[0].image_rect.center(),
            moved.views[0].image_rect.center(),
        );
    }
}

#[test]
fn batched_zoom_uses_event_time_anchor_before_and_after_pointer_move() {
    for zoom in zoom_events() {
        for move_first in [false, true] {
            for transform in [
                egui::emath::TSTransform::IDENTITY,
                egui::emath::TSTransform::new(vec2(50.0, 30.0), 1.5),
            ] {
                let mut h = Harness::new(transform);
                let before = h.hundred_percent();
                let a = before.views[0].response.rect.center();
                let b = a + vec2(80.0, 0.0);
                let anchor = if move_first { b } else { a };
                let image_point =
                    (anchor - before.views[0].image_rect.min) / before.views[0].image_rect.size();
                h.frame(vec![Event::PointerMoved(transform * a)]);
                let movement = Event::PointerMoved(transform * b);
                let events = if move_first {
                    vec![movement, zoom.clone()]
                } else {
                    vec![zoom.clone(), movement]
                };
                let after = h.frame(events);
                near(after.views[0].physical_scale, 2.0);
                let next =
                    (anchor - after.views[0].image_rect.min) / after.views[0].image_rect.size();
                near(image_point.x, next.x);
                near(image_point.y, next.y);
                assert!(
                    !after
                        .remaining_after_left
                        .iter()
                        .any(|event| matches!(event, Event::Zoom(_) | Event::MouseWheel { .. }))
                );
                assert_eq!(h.views[1].mode(), ViewMode::Fit);
            }
        }
    }
}

#[test]
fn neutral_targeted_zoom_keeps_fit_responsive() {
    let mut h = Harness::new(egui::emath::TSTransform::IDENTITY);
    h.frame(vec![]);
    let before = h.frame(vec![]);
    h.frame(vec![Event::PointerMoved(
        before.views[0].response.rect.center(),
    )]);
    let after = h.frame(vec![
        Event::Zoom(1.0),
        Event::MouseWheel {
            unit: MouseWheelUnit::Line,
            delta: Vec2::ZERO,
            modifiers: Modifiers::CTRL,
            phase: TouchPhase::Move,
        },
    ]);
    assert_eq!(h.views[0].mode(), ViewMode::Fit);
    assert_eq!(after.views[0].image_rect, before.views[0].image_rect);
    assert!(after.remaining_after_left.is_empty());
}

#[test]
fn batched_zoom_preserves_interleaved_anchors_and_scale_limits() {
    let mut batched = Harness::new(egui::emath::TSTransform::IDENTITY);
    let mut sequential = Harness::new(egui::emath::TSTransform::IDENTITY);
    let before = batched.hundred_percent();
    sequential.hundred_percent();
    let a = before.views[0].response.rect.center();
    let b = a + vec2(80.0, 20.0);
    let events = vec![
        Event::PointerMoved(a),
        Event::Zoom(1e20),
        Event::PointerMoved(b),
        Event::Zoom(0.5),
    ];
    let after = batched.frame(events.clone());
    let mut reference = sequential.frame(vec![]);
    for event in events {
        reference = sequential.frame(vec![event]);
    }
    near(after.views[0].physical_scale, 32.0);
    assert_eq!(after.views[0].image_rect, reference.views[0].image_rect);
}

#[test]
fn batched_zoom_leaves_text_sibling_outside_and_invalid_events_unconsumed() {
    for zoom in zoom_events() {
        let mut h = Harness::new(egui::emath::TSTransform::IDENTITY);
        let before = h.hundred_percent();
        let left = before.views[0].response.rect.center();
        let right = before.views[1].response.rect.center();
        for elsewhere in [before.text.center(), right, Pos2::new(1500.0, 900.0)] {
            h.frame(vec![Event::PointerMoved(elsewhere)]);
            let after = h.frame(vec![zoom.clone(), Event::PointerMoved(left)]);
            assert!(after.remaining_after_left.contains(&zoom));
            near(after.views[0].physical_scale, 1.0);
            // The left event is handled even if the frame ends elsewhere.
            let after = h.frame(vec![zoom.clone(), Event::PointerMoved(elsewhere)]);
            near(after.views[0].physical_scale, 2.0);
            assert!(!after.remaining_after_left.contains(&zoom));
            h.hundred_percent();
        }
        h.frame(vec![Event::PointerMoved(left)]);
        let invalid = vec![
            Event::Zoom(f32::INFINITY),
            Event::Zoom(-1.0),
            Event::MouseWheel {
                unit: MouseWheelUnit::Point,
                delta: vec2(0.0, f32::INFINITY),
                modifiers: Modifiers::CTRL,
                phase: TouchPhase::Move,
            },
        ];
        let after = h.frame(invalid.clone());
        assert_eq!(after.remaining_after_left, invalid);
        near(after.views[0].physical_scale, 1.0);
    }
}
