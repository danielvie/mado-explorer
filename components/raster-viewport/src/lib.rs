//! Read-only egui raster controls. The caller owns the texture and supplies oriented
//! full-image pixel dimensions, not the possibly downsampled texture dimensions.
use egui::{
    Color32, Event, MouseWheelUnit, PointerButton, Pos2, Rect, Response, Sense, TextureHandle, Ui,
    Vec2,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ViewMode {
    #[default]
    Fit,
    /// Physical display pixels per supplied image pixel.
    Scale(f32),
}

#[derive(Debug, Default)]
pub struct RasterViewport {
    document: Option<u64>,
    mode: ViewMode,
    // Fraction of drawn image size, preserving the center through sharper replacements.
    pan: Vec2,
    previous_pointer: Option<Pos2>,
    // Retain only a primary capture admitted at its event-time press position.
    primary_drag: Option<egui::Id>,
}

pub struct ViewportOutput {
    pub response: Response,
    /// Full image rectangle before viewport clipping, in egui points.
    pub image_rect: Rect,
    /// Physical display pixels per supplied image pixel.
    pub physical_scale: f32,
    #[cfg(test)]
    controls: [Response; 4],
}

impl RasterViewport {
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// Render controls, raster area, then current scale/dimensions. `viewport_size`
    /// is the requested raster area in points; available space may reduce it.
    /// Reserve space for controls and the status line in the parent layout.
    /// Use a stable unique `id_salt` per instance and a new `document_key` for a
    /// different document/page. Same-key sharper textures must keep full-image
    /// dimensions. No texture handle is cloned, stored, created or modified here.
    /// Invalid or unrepresentable geometry displays a refusal and returns `None`.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        id_salt: impl egui::AsIdSalt,
        document_key: u64,
        texture: &TextureHandle,
        image_pixels: Vec2,
        viewport_size: Vec2,
    ) -> Option<ViewportOutput> {
        let density = ui.ctx().pixels_per_point();
        let previous_pointer = self.previous_pointer;
        let previous_drag = self.primary_drag.take();
        self.previous_pointer = ui.input(|input| input.pointer.latest_pos());
        if !positive(image_pixels)
            || !positive(viewport_size)
            || !density.is_finite()
            || density <= 0.0
        {
            ui.label("Cannot display raster: invalid dimensions or display scale.");
            return None;
        }
        if self.document != Some(document_key) {
            self.document = Some(document_key);
            self.mode = ViewMode::Fit;
            self.pan = Vec2::ZERO;
        }
        ui.push_id(egui::Id::new(id_salt).with(document_key), |ui| {
            ui.vertical(|ui| {
                let buttons = ui
                    .horizontal_wrapped(|ui| {
                        [
                            ui.button("Fit"),
                            ui.button("100%"),
                            ui.button("Zoom out"),
                            ui.button("Zoom in"),
                        ]
                    })
                    .inner;
                let available = ui.available_size();
                let size = viewport_size.min(Vec2::new(
                    available.x,
                    (available.y
                        - ui.text_style_height(&egui::TextStyle::Body)
                        - ui.spacing().item_spacing.y)
                        .max(0.0),
                ));
                let fit = fit_scale(image_pixels, size, density);
                let Some(fit) = fit else {
                    ui.label("Cannot display raster: no usable viewport geometry.");
                    return None;
                };
                let (rect, response) = ui.allocate_exact_size(size, Sense::drag());
                if buttons[0].clicked() {
                    self.mode = ViewMode::Fit;
                    self.pan = Vec2::ZERO;
                }
                if buttons[1].clicked() {
                    self.mode = ViewMode::Scale(1.0);
                    self.pan = Vec2::ZERO;
                }
                if buttons[2].clicked() {
                    self.zoom(0.8, rect.center(), rect, image_pixels, density, fit);
                }
                if buttons[3].clicked() {
                    self.zoom(1.25, rect.center(), rect, image_pixels, density, fit);
                }
                // Replay unsmoothed events at their pointer position, not final hover.
                // Take the existing event vector to query layer ownership without nested
                // context locks; no event copy or retained event queue is needed.
                if response.enabled() {
                    let from_global = ui.ctx().layer_transform_from_global(response.layer_id);
                    let mut events = ui.input_mut(|input| std::mem::take(&mut input.events));
                    // Only a stationary batch may use the context's final position.
                    // With ordered motion, it would incorrectly retarget earlier zoom.
                    let mut pointer = if events.iter().any(|event| {
                        matches!(
                            event,
                            Event::PointerMoved(_)
                                | Event::PointerButton { .. }
                                | Event::PointerGone
                        )
                    }) {
                        previous_pointer
                    } else {
                        self.previous_pointer
                    };
                    let owns_drag = ui.ctx().dragged_id() == Some(response.id);
                    let mut dragging = previous_drag == Some(response.id)
                        && (owns_drag || response.drag_stopped_by(PointerButton::Primary));
                    events.retain(|event| {
                        match event {
                            Event::PointerMoved(pos) | Event::PointerButton { pos, .. } => {
                                if dragging && let Some(previous) = pointer {
                                    let delta = *pos - previous;
                                    let delta = from_global
                                        .map_or(delta, |transform| delta * transform.scaling);
                                    let drawn = image_pixels * (self.scale(fit) / density);
                                    if positive(drawn) && delta.is_finite() {
                                        self.pan += delta / drawn;
                                        self.clamp_pan(drawn, rect.size());
                                    }
                                }
                                pointer = Some(*pos);
                                if let Event::PointerButton {
                                    button: PointerButton::Primary,
                                    pressed,
                                    ..
                                } = event
                                {
                                    let anchor =
                                        from_global.map_or(*pos, |transform| transform * *pos);
                                    // Egui grants capture; the event-time press must also
                                    // belong to this raster, not merely end over it.
                                    dragging = *pressed
                                        && owns_drag
                                        && response.interact_rect.contains(anchor)
                                        && ui.ctx().layer_id_at(*pos) == Some(response.layer_id);
                                }
                                return true;
                            }
                            Event::PointerGone => {
                                pointer = None;
                                return true;
                            }
                            _ => {}
                        }
                        let Some(global) = pointer else {
                            return true;
                        };
                        let anchor = from_global.map_or(global, |transform| transform * global);
                        if !response.interact_rect.contains(anchor)
                            || ui.ctx().layer_id_at(global) != Some(response.layer_id)
                        {
                            return true;
                        }
                        let factor = match event {
                            Event::Zoom(factor) if factor.is_finite() && *factor > 0.0 => *factor,
                            Event::MouseWheel {
                                unit,
                                delta,
                                modifiers,
                                ..
                            } if delta.is_finite() && (modifiers.ctrl || modifiers.command) => {
                                let speed = match unit {
                                    MouseWheelUnit::Point => 0.005,
                                    MouseWheelUnit::Line => 0.2,
                                    MouseWheelUnit::Page => 1.0,
                                };
                                let amount = delta.y * speed;
                                if !amount.is_finite() {
                                    return true;
                                }
                                amount.clamp(-4.0, 4.0).exp()
                            }
                            _ => return true,
                        };
                        if factor != 1.0 {
                            self.zoom(factor, anchor, rect, image_pixels, density, fit);
                        }
                        false
                    });
                    self.primary_drag =
                        (dragging && owns_drag && ui.input(|input| input.pointer.primary_down()))
                            .then_some(response.id);
                    ui.input_mut(|input| input.events = events);
                }
                let scale = self.scale(fit);
                let drawn = image_pixels * (scale / density);
                if !positive(drawn) {
                    ui.label("Cannot display raster: unrepresentable image geometry.");
                    return None;
                }
                self.clamp_pan(drawn, size);
                let image_rect = Rect::from_center_size(rect.center() + self.pan * drawn, drawn);
                if !image_rect.is_finite() {
                    ui.label("Cannot display raster: unrepresentable image geometry.");
                    return None;
                }
                let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
                painter.rect_filled(rect, 0.0, ui.visuals().extreme_bg_color);
                painter.image(
                    texture.id(),
                    image_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
                ui.label(format!(
                    "{}  {:.2}%  |  {:.0} x {:.0} pixels",
                    if self.mode == ViewMode::Fit {
                        "Fit"
                    } else {
                        "Scale"
                    },
                    scale * 100.0,
                    image_pixels.x,
                    image_pixels.y
                ));
                Some(ViewportOutput {
                    response,
                    image_rect,
                    physical_scale: scale,
                    #[cfg(test)]
                    controls: buttons,
                })
            })
            .inner
        })
        .inner
    }

    fn scale(&self, fit: f32) -> f32 {
        match self.mode {
            ViewMode::Fit => fit,
            ViewMode::Scale(scale) => scale,
        }
    }

    fn zoom(
        &mut self,
        factor: f32,
        anchor: Pos2,
        viewport: Rect,
        pixels: Vec2,
        density: f32,
        fit: f32,
    ) {
        if !factor.is_finite() || factor <= 0.0 || !anchor.is_finite() {
            return;
        }
        let old_scale = self.scale(fit);
        // Manual zoom spans 1%-6400%, extended to include Fit for unusual geometry.
        let next = (old_scale * factor).clamp(fit.min(0.01), fit.max(64.0));
        let old_size = pixels * (old_scale / density);
        let next_size = pixels * (next / density);
        if !positive(old_size) || !positive(next_size) {
            return;
        }
        let offset = anchor - viewport.center();
        let image_point = offset / old_size - self.pan;
        self.pan = offset / next_size - image_point;
        self.mode = ViewMode::Scale(next);
        self.clamp_pan(next_size, viewport.size());
    }

    fn clamp_pan(&mut self, drawn: Vec2, viewport: Vec2) {
        let limit = ((drawn - viewport) / (drawn * 2.0)).max(Vec2::ZERO);
        self.pan = self.pan.clamp(-limit, limit);
    }
}

fn positive(size: Vec2) -> bool {
    size.is_finite() && size.x > 0.0 && size.y > 0.0
}

fn fit_scale(pixels: Vec2, viewport: Vec2, density: f32) -> Option<f32> {
    if !positive(pixels) || !positive(viewport) || !density.is_finite() || density <= 0.0 {
        return None;
    }
    let scale = (viewport.x / pixels.x).min(viewport.y / pixels.y) * density;
    let drawn = pixels * (scale / density);
    (scale.is_finite() && scale > 0.0 && positive(drawn)).then_some(scale)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "../tests/support/batched_input.rs"]
mod batched_input;
