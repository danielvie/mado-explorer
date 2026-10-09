use super::*;
use egui::{LayerId, Pos2, RawInput, Rect, Vec2};

fn frame(ctx: &Context, dialog: &mut ConflictDialog) {
    ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 700.0))),
            ..Default::default()
        },
        |ui| {
            assert!(dialog.show(ui.ctx()).decision.is_none());
        },
    )
    .drop_without_applying_deltas();
}

fn retained(ctx: &Context) -> (Vec<LayerId>, Vec<LayerId>) {
    // layer_ids reads retained Areas.order, not just currently visible layers.
    let layers = ctx.memory(|memory| memory.layer_ids().collect::<Vec<_>>());
    // AreaState::load reads the retained Areas.areas map, even after hiding.
    let areas = layers
        .iter()
        .copied()
        .filter(|layer| egui::AreaState::load(ctx, layer.id).is_some())
        .collect();
    (layers, areas)
}

#[test]
fn retained_areas_and_layers_are_bounded_across_requests_and_controllers() {
    let ctx = Context::default();
    let mut baseline = None;
    let mut previous_widgets = None;
    for operation in 1..=12 {
        let mut dialog = ConflictDialog::new(OperationId(operation));
        for conflict in 1..=32 {
            let request = ConflictRequest::new(
                OperationId(operation),
                ConflictId(conflict),
                PathBuf::from(if cfg!(windows) {
                    r"C:\generated\source.txt"
                } else {
                    "/generated/source.txt"
                }),
                PathBuf::from(if cfg!(windows) {
                    r"C:\generated\destination.txt"
                } else {
                    "/generated/destination.txt"
                }),
                ConflictType {
                    source: EntryKind::RegularFile,
                    destination: EntryKind::RegularFile,
                },
            )
            .unwrap();
            assert!(dialog.present(request).unwrap().is_none());
            for _ in 0..3 {
                frame(&ctx, &mut dialog);
            }
            let (buttons, checkbox, _) = dialog.controls.as_ref().unwrap();
            let widgets = (buttons.each_ref().map(|button| button.id), checkbox.id);
            if let Some((previous_buttons, previous_checkbox)) = previous_widgets {
                for (button, previous) in widgets.0.into_iter().zip(previous_buttons) {
                    assert_ne!(button, previous, "action ID must be request-specific");
                }
                assert_ne!(
                    widgets.1, previous_checkbox,
                    "checkbox ID must be request-specific"
                );
            }
            previous_widgets = Some(widgets);
            dialog
                .decide(
                    OperationId(operation),
                    ConflictId(conflict),
                    Choice::Skip,
                    false,
                )
                .unwrap();
            // Hide it for two frames so a visible-layer count cannot hide the leak.
            frame(&ctx, &mut dialog);
            frame(&ctx, &mut dialog);
            let state = retained(&ctx);
            let expected = baseline.get_or_insert_with(|| state.clone());
            assert!(
                !expected.1.is_empty(),
                "must inspect actual retained area state"
            );
            assert_eq!(
                &state, expected,
                "retained state grew at operation {operation}, conflict {conflict}"
            );
        }
        dialog.finish();
        frame(&ctx, &mut dialog);
        assert_eq!(retained(&ctx), *baseline.as_ref().unwrap());
        // Dropping and recreating the controller must not allocate another area.
    }
}
