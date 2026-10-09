//! Safe generated-data demonstration. No native window or filesystem input.
use egui::{Color32, ColorImage, Context, RawInput, Rect, TextureOptions, Vec2};
use mado_raster_viewport::RasterViewport;

fn main() {
    let context = Context::default();
    let texture = context.load_texture(
        "generated",
        ColorImage::filled([8, 4], Color32::LIGHT_BLUE),
        TextureOptions::NEAREST,
    );
    let mut viewport = RasterViewport::default();
    let output = context.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(
                Default::default(),
                Vec2::new(600.0, 400.0),
            )),
            ..Default::default()
        },
        |ui| {
            let result = viewport
                .show(
                    ui,
                    "demo",
                    1,
                    &texture,
                    Vec2::new(800.0, 400.0),
                    Vec2::new(500.0, 250.0),
                )
                .expect("generated geometry is valid");
            println!(
                "Generated Fit: {:.2}% physical scale, {:?} points",
                result.physical_scale * 100.0,
                result.image_rect.size()
            );
        },
    );
    output.drop_without_applying_deltas();
}
