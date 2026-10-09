use super::*;
use egui::{
    ColorImage, Context, FullOutput, Key, Modifiers, RawInput, TextureOptions, TouchPhase,
    ViewportId, vec2,
};

struct Harness {
    ctx: Context,
    texture: TextureHandle,
    views: [RasterViewport; 2],
    keys: [u64; 2],
    pixels: Vec2,
    size: Vec2,
    text: String,
    density: f32,
}

struct Frame {
    views: [Option<ViewportOutput>; 2],
    text_rect: Rect,
    output: FullOutput,
}

impl Harness {
    fn new(density: f32) -> Self {
        let ctx = Context::default();
        let texture = ctx.load_texture(
            "generated",
            ColorImage::filled([8, 4], Color32::LIGHT_BLUE),
            TextureOptions::NEAREST,
        );
        Self {
            ctx,
            texture,
            views: Default::default(),
            keys: [1, 1],
            pixels: vec2(800.0, 400.0),
            size: vec2(300.0, 200.0),
            text: String::new(),
            density,
        }
    }

    fn frame(&mut self, events: Vec<Event>) -> Frame {
        let mut input = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1000.0, 700.0))),
            events,
            ..Default::default()
        };
        input
            .viewports
            .get_mut(&ViewportId::ROOT)
            .unwrap()
            .native_pixels_per_point = Some(self.density);
        let mut views = [None, None];
        let mut text_rect = Rect::NOTHING;
        let mut output = self.ctx.run_ui(input, |ui| {
            text_rect = ui.text_edit_singleline(&mut self.text).rect;
            ui.horizontal(|ui| {
                for (i, result) in views.iter_mut().enumerate() {
                    ui.allocate_ui(vec2(350.0, 500.0), |ui| {
                        *result = self.views[i].show(
                            ui,
                            i,
                            self.keys[i],
                            &self.texture,
                            self.pixels,
                            self.size,
                        );
                    });
                }
            });
        });
        // Headless tests inspect geometry/shapes; no renderer applies texture uploads.
        output.textures_delta.clear();
        Frame {
            views,
            text_rect,
            output,
        }
    }

    fn click(&mut self, pos: Pos2) -> Frame {
        self.frame(vec![Event::PointerMoved(pos)]);
        self.frame(vec![button(pos, true)]);
        self.frame(vec![button(pos, false)])
    }

    fn control(&mut self, view: usize, control: usize) -> Frame {
        let frame = self.frame(vec![]);
        self.click(
            frame.views[view].as_ref().unwrap().controls[control]
                .rect
                .center(),
        )
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
    assert!((a - b).abs() <= 0.002 * b.abs().max(1.0), "{a} != {b}");
}

#[test]
fn fit_and_hundred_percent_use_physical_pixels_and_keep_aspect_at_three_densities() {
    for density in [1.0, 1.5, 2.0] {
        let mut h = Harness::new(density);
        h.frame(vec![]);
        let fit = h.frame(vec![]);
        let fit = fit.views[0].as_ref().unwrap();
        near(h.ctx.pixels_per_point(), density);
        near(fit.image_rect.width(), 300.0);
        near(fit.image_rect.height(), 150.0);
        near(fit.physical_scale, 300.0 / 800.0 * density);
        assert!(fit.response.rect.contains_rect(fit.image_rect));
        let full = h.control(0, 1);
        let full = full.views[0].as_ref().unwrap();
        near(full.image_rect.width() * density, 800.0);
        near(full.image_rect.height() * density, 400.0);
        near(full.physical_scale, 1.0);
        assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
        let fit = h.control(0, 0);
        near(fit.views[0].as_ref().unwrap().image_rect.width(), 300.0);
        assert_eq!(h.views[0].mode(), ViewMode::Fit);
    }
}

#[test]
fn actual_zoom_buttons_pointer_anchor_drag_and_bounds() {
    let mut h = Harness::new(1.0);
    let hundred = h.control(0, 1);
    let old = hundred.views[0].as_ref().unwrap();
    let anchor = old.response.rect.center() + vec2(40.0, 30.0);
    let image_point = (anchor - old.image_rect.min) / old.image_rect.size();
    h.frame(vec![Event::PointerMoved(anchor)]);
    let zoom = h.frame(vec![Event::Zoom(2.0)]);
    let zoom = zoom.views[0].as_ref().unwrap();
    near(zoom.physical_scale, 2.0);
    let next_point = (anchor - zoom.image_rect.min) / zoom.image_rect.size();
    near(image_point.x, next_point.x);
    near(image_point.y, next_point.y);
    let minus = h.control(0, 2);
    near(minus.views[0].as_ref().unwrap().physical_scale, 1.6);
    let plus = h.control(0, 3);
    near(plus.views[0].as_ref().unwrap().physical_scale, 2.0);

    let center = plus.views[0].as_ref().unwrap().response.rect.center();
    h.frame(vec![Event::PointerMoved(center)]);
    h.frame(vec![button(center, true)]);
    let panned = h.frame(vec![Event::PointerMoved(center + vec2(10000.0, 10000.0))]);
    let panned = panned.views[0].as_ref().unwrap();
    near(panned.image_rect.min.x, panned.response.rect.min.x);
    near(panned.image_rect.min.y, panned.response.rect.min.y);
    h.frame(vec![button(center + vec2(10000.0, 10000.0), false)]);
    h.frame(vec![Event::PointerMoved(center)]);
    let max = h.frame(vec![Event::Zoom(1e20)]);
    near(max.views[0].as_ref().unwrap().physical_scale, 64.0);
    let min = h.frame(vec![Event::Zoom(1e-20)]);
    near(min.views[0].as_ref().unwrap().physical_scale, 0.01);
    near(h.views[0].pan.x, 0.0);
    near(h.views[0].pan.y, 0.0);
}

#[test]
fn fit_resize_density_change_and_same_document_replacement_preserve_manual_view() {
    let mut h = Harness::new(1.0);
    h.frame(vec![]);
    h.size = vec2(200.0, 100.0);
    let resized = h.frame(vec![]);
    near(resized.views[0].as_ref().unwrap().image_rect.width(), 200.0);
    near(
        resized.views[0].as_ref().unwrap().image_rect.height(),
        100.0,
    );
    h.control(0, 1);
    let center = h.frame(vec![]).views[0]
        .as_ref()
        .unwrap()
        .response
        .rect
        .center();
    h.frame(vec![Event::PointerMoved(center)]);
    h.frame(vec![button(center, true)]);
    let panned = h.frame(vec![Event::PointerMoved(center + vec2(50.0, 20.0))]);
    h.frame(vec![button(center + vec2(50.0, 20.0), false)]);
    let pan = h.views[0].pan;
    assert!(pan.x > 0.0 && pan.y > 0.0);
    let rect = panned.views[0].as_ref().unwrap().image_rect;
    let old_id = h.texture.id();
    h.texture = h.ctx.load_texture(
        "sharper",
        ColorImage::filled([16, 8], Color32::LIGHT_RED),
        TextureOptions::NEAREST,
    );
    assert_ne!(old_id, h.texture.id());
    let replaced = h.frame(vec![]);
    assert_eq!(replaced.views[0].as_ref().unwrap().image_rect, rect);
    assert_eq!(h.views[0].pan, pan);
    h.density = 2.0;
    h.frame(vec![]);
    let density = h.frame(vec![]);
    near(density.views[0].as_ref().unwrap().image_rect.width(), 400.0);
    near(density.views[0].as_ref().unwrap().physical_scale, 1.0);
    assert_eq!(h.views[0].pan, pan);
    h.keys[0] = 2;
    let reset = h.frame(vec![]);
    assert_eq!(h.views[0].mode(), ViewMode::Fit);
    assert_eq!(h.views[0].pan, Vec2::ZERO);
    near(reset.views[0].as_ref().unwrap().image_rect.width(), 200.0);
}

#[test]
fn neighboring_viewports_and_text_do_not_receive_each_others_pointer_input() {
    let mut h = Harness::new(1.0);
    let frame = h.control(0, 1);
    assert_eq!(h.views[1].mode(), ViewMode::Fit);
    let left = frame.views[0].as_ref().unwrap().response.rect.center();
    let right = frame.views[1].as_ref().unwrap().response.rect.center();
    h.frame(vec![Event::PointerMoved(right)]);
    h.frame(vec![Event::MouseWheel {
        unit: MouseWheelUnit::Line,
        delta: vec2(0.0, 2.0),
        modifiers: Modifiers::CTRL,
        phase: TouchPhase::Move,
    }]);
    assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
    assert_ne!(h.views[1].mode(), ViewMode::Fit);
    let right_mode = h.views[1].mode();
    h.click(frame.text_rect.center());
    h.frame(vec![Event::Text("query".into()), Event::Zoom(2.0)]);
    assert_eq!(h.text, "query");
    assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
    assert_eq!(h.views[1].mode(), right_mode);
    // A zoom over text followed by movement into a viewport in the same frame
    // still belongs to the text position, not the final frame position.
    h.frame(vec![Event::Zoom(2.0), Event::PointerMoved(left)]);
    assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
    // Moving over left without a new wheel event must not import smoothed wheel zoom.
    h.frame(vec![Event::PointerMoved(left)]);
    assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
    h.frame(vec![button(left, true)]);
    h.frame(vec![Event::PointerMoved(right)]);
    assert_eq!(h.views[1].mode(), right_mode);
    assert_eq!(h.views[1].pan, Vec2::ZERO);
    h.frame(vec![button(right, false)]);
}

#[test]
fn invalid_geometry_and_zoom_are_refused_without_a_texture_draw() {
    let mut h = Harness::new(1.0);
    for pixels in [
        vec2(0.0, 40.0),
        vec2(-1.0, 40.0),
        vec2(f32::NAN, 40.0),
        vec2(40.0, f32::INFINITY),
        vec2(f32::MIN_POSITIVE, f32::MIN_POSITIVE),
    ] {
        h.pixels = pixels;
        let frame = h.frame(vec![]);
        assert!(frame.views.iter().all(Option::is_none));
        assert!(!frame.output.shapes.iter().any(|shape| matches!(&shape.shape, egui::epaint::Shape::Mesh(mesh) if mesh.texture_id == h.texture.id())));
    }
    h.pixels = vec2(800.0, 400.0);
    for size in [
        Vec2::ZERO,
        vec2(f32::NAN, 200.0),
        vec2(300.0, f32::INFINITY),
    ] {
        h.size = size;
        assert!(h.frame(vec![]).views.iter().all(Option::is_none));
    }
    assert!(fit_scale(h.pixels, vec2(300.0, 200.0), 0.0).is_none());
    assert!(fit_scale(h.pixels, vec2(300.0, 200.0), f32::NAN).is_none());
    h.size = vec2(300.0, 200.0);
    let frame = h.control(0, 1);
    let center = frame.views[0].as_ref().unwrap().response.rect.center();
    h.frame(vec![Event::PointerMoved(center)]);
    for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        h.frame(vec![Event::Zoom(factor)]);
        assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
    }
}

#[test]
fn portrait_fit_and_document_switch_during_drag_remain_centered() {
    let mut h = Harness::new(2.0);
    h.pixels = vec2(400.0, 800.0);
    let fit = h.frame(vec![]);
    let fit = fit.views[0].as_ref().unwrap();
    near(fit.image_rect.width(), 100.0);
    near(fit.image_rect.height(), 200.0);
    h.control(0, 1);
    let center = h.frame(vec![]).views[0]
        .as_ref()
        .unwrap()
        .response
        .rect
        .center();
    h.frame(vec![Event::PointerMoved(center)]);
    h.frame(vec![button(center, true)]);
    h.frame(vec![Event::PointerMoved(center + vec2(0.0, 40.0))]);
    assert!(h.views[0].pan.y > 0.0);
    h.keys[0] = 3;
    h.frame(vec![Event::PointerMoved(center + vec2(0.0, 60.0))]);
    assert_eq!(h.views[0].mode(), ViewMode::Fit);
    assert_eq!(h.views[0].pan, Vec2::ZERO);
    h.frame(vec![Event::PointerMoved(center + vec2(0.0, 80.0))]);
    assert_eq!(h.views[0].pan, Vec2::ZERO);
    h.frame(vec![button(center + vec2(0.0, 80.0), false)]);
}

#[test]
fn texture_mesh_is_clipped_and_scale_is_visible_and_keyboard_buttons_work() {
    let mut h = Harness::new(1.5);
    let frame = h.control(0, 1);
    let result = frame.views[0].as_ref().unwrap();
    let mesh = frame.output.shapes.iter().find(|shape| matches!(&shape.shape, egui::epaint::Shape::Mesh(mesh) if mesh.texture_id == h.texture.id())).unwrap();
    assert_eq!(mesh.clip_rect, result.response.rect);
    assert!(frame.output.shapes.iter().any(|shape| matches!(&shape.shape, egui::epaint::Shape::Text(text) if text.galley.job.text.contains("100.00%") && text.galley.job.text.contains("800 x 400"))));
    // Inject focus, then use egui's real keyboard button activation path.
    h.ctx
        .memory_mut(|memory| memory.request_focus(result.controls[1].id));
    h.views[0].mode = ViewMode::Scale(2.0);
    h.frame(vec![Event::Key {
        key: Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);
    assert_eq!(h.views[0].mode(), ViewMode::Scale(1.0));
}
