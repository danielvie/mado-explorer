use super::*;
use crate::tests::{filter_app, key_event};

fn frame(
    ctx: &egui::Context,
    app: &mut Explorer,
    size: egui::Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
            ui.spacing_mut().button_padding = egui::vec2(6.0, 3.0);
            app.workspace(ui);
        },
    )
}

fn text_rect(output: &egui::FullOutput, label: &str) -> egui::Rect {
    output
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            egui::Shape::Text(text) if text.galley.job.text == label => {
                Some(egui::Rect::from_min_size(text.pos, text.galley.size()))
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("Missing label {label}"))
}

fn click(ctx: &egui::Context, app: &mut Explorer, size: egui::Vec2, pos: egui::Pos2) {
    for pressed in [true, false] {
        frame(
            ctx,
            app,
            size,
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        )
        .drop_without_applying_deltas();
    }
}

fn finish(tab: &mut Tab) {
    let rows = selection::tests::rows(&["alpha", "beta"]);
    let generation = tab.navigation.key().generation;
    tab.apply(directory::Response {
        generation,
        update: directory::Update::Snapshot {
            entries: rows,
            metadata_errors: 0,
        },
    });
    tab.apply(directory::Response {
        generation,
        update: directory::Update::Finished(Ok(tab.navigation.visible().location.clone())),
    });
    let nav = &mut tab.navigation;
    assert!(nav.apply_sorted(sorting::project_for_test(
        nav.key(),
        nav.current.raw_entries.clone(),
        &nav.filter
    )));
}

fn click_label(ctx: &egui::Context, app: &mut Explorer, size: egui::Vec2, label: &str) {
    let output = frame(ctx, app, size, vec![]);
    let pos = text_rect(&output, label).center();
    output.drop_without_applying_deltas();
    click(ctx, app, size, pos);
}

#[test]
fn s016_normal_header_table_footer_geometry_keeps_the_file_list_primary() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    let size = egui::vec2(1050.0, 700.0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    let output = frame(&ctx, &mut app, size, vec![]);
    let back = text_rect(&output, "Back");
    let location = text_rect(&output, "Location");
    let name = text_rect(&output, "Name ^");
    let filter = text_rect(&output, "Filter");
    let counts = text_rect(&output, "3 / 3 entries displayed");
    assert!(
        (back.center().y - location.center().y).abs() < 3.0,
        "Navigation and path share a band"
    );
    assert!(name.top() < 130.0, "Compact header: {name:?}");
    assert!(filter.top() > 620.0 && (filter.center().y - counts.center().y).abs() < 4.0);
    assert!(app.active_tab_mut().row_viewport_height > 480.0);
    assert!(text_rect(&output, "Focused").top() < back.top());
    assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.starts_with("Entries"))));
    output.drop_without_applying_deltas();
}

#[test]
fn s016_narrow_short_and_split_geometry_has_reachable_actions_and_no_band_overlap() {
    for (size, split) in [
        (egui::vec2(320.0, 300.0), false),
        (egui::vec2(480.0, 280.0), false),
        (egui::vec2(646.0, 340.0), true),
    ] {
        let ctx = egui::Context::default();
        let mut app = filter_app();
        if split {
            let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1100.0, 800.0));
            app.pane_rects = vec![(PaneId(1), bounds)];
            app.visible_workspace = bounds;
            assert!(app.split_pane(Axis::LeftRight));
            finish(&mut app.panes[1].tabs[0]);
        }
        for _ in 0..4 {
            frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
        }
        let output = frame(&ctx, &mut app, size, vec![]);
        let location = text_rect(&output, "Location");
        let back = text_rect(&output, "Back");
        let name = text_rect(&output, "Name ^");
        let filter = text_rect(&output, "Filter");
        assert!(back.bottom() <= location.top());
        assert!(location.bottom() < name.top() && name.bottom() < filter.top());
        for label in ["Layout", "+", "Go", "Refresh", "Clear", "View"] {
            let action = text_rect(&output, label);
            let pane = app
                .pane_rects
                .iter()
                .find(|(_, rect)| rect.contains(action.center()))
                .expect("Action inside its pane")
                .1;
            assert!(
                pane.contains_rect(action),
                "{label} clipped at {size:?}: {action:?}"
            );
        }
        for pane in &app.panes {
            assert!(pane.tabs[pane.active].row_viewport_height >= 60.0);
        }
        output.drop_without_applying_deltas();
    }
}

#[test]
fn s016_footer_and_path_editors_block_shortcuts_including_the_first_click_frame() {
    for editor in ["filter", "location"] {
        let ctx = egui::Context::default();
        let mut app = filter_app();
        let size = egui::vec2(1050.0, 700.0);
        for _ in 0..3 {
            frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
        }
        let id = app.active_tab_mut().widget_id(editor);
        let rect = ctx.read_response(id).expect("Live editor").rect;
        let before = app.active_tab_mut().navigation.key();
        let pos = rect.center();
        frame(
            &ctx,
            &mut app,
            size,
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                key_event(egui::Key::T, egui::Modifiers::CTRL),
                key_event(egui::Key::F5, egui::Modifiers::NONE),
                key_event(egui::Key::A, egui::Modifiers::CTRL),
            ],
        )
        .drop_without_applying_deltas();
        assert_eq!(
            app.panes[0].tabs.len(),
            1,
            "{editor}: no opening-frame tab shortcut"
        );
        assert_eq!(app.active_tab_mut().navigation.key(), before);
        assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
        assert!(ctx.memory(|m| m.has_focus(id)));
        frame(
            &ctx,
            &mut app,
            size,
            vec![
                key_event(egui::Key::ArrowDown, egui::Modifiers::NONE),
                key_event(egui::Key::W, egui::Modifiers::CTRL),
            ],
        )
        .drop_without_applying_deltas();
        assert_eq!(app.active_tab_mut().id, TabId(1));
        assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
    }
}

#[test]
fn s016_footer_click_and_f6_route_only_the_focused_pane() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    let size = egui::vec2(1400.0, 700.0);
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
    app.pane_rects = vec![(PaneId(1), bounds)];
    app.visible_workspace = bounds;
    assert!(app.split_pane(Axis::LeftRight));
    finish(&mut app.panes[1].tabs[0]);
    app.focus_pane(0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    let id = app.panes[1].tabs[0].widget_id("filter");
    let pos = ctx.read_response(id).unwrap().rect.center();
    click(&ctx, &mut app, size, pos);
    frame(&ctx, &mut app, size, vec![egui::Event::Text("be".into())])
        .drop_without_applying_deltas();
    assert_eq!(app.focused, 1);
    assert_eq!(app.panes[1].tabs[0].filter_text, "be");
    assert!(app.panes[0].tabs[0].filter_text.is_empty());
    frame(
        &ctx,
        &mut app,
        size,
        vec![
            key_event(egui::Key::F6, egui::Modifiers::NONE),
            key_event(egui::Key::A, egui::Modifiers::CTRL),
        ],
    )
    .drop_without_applying_deltas();
    assert_eq!(app.focused, 0);
    assert_eq!(app.panes[0].tabs[0].navigation.selection.paths.len(), 3);
    assert!(app.panes[1].tabs[0].navigation.selection.paths.is_empty());
}

#[test]
fn s016_layout_and_visibility_menus_perform_real_actions_at_minimum_width() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    let size = egui::vec2(480.0, 400.0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    let output = frame(&ctx, &mut app, size, vec![]);
    let pos = text_rect(&output, "View").center();
    output.drop_without_applying_deltas();
    click(&ctx, &mut app, size, pos);
    let output = frame(&ctx, &mut app, size, vec![]);
    let pos = text_rect(&output, "Show hidden/system entries").center();
    assert!(egui::Rect::from_min_size(egui::Pos2::ZERO, size).contains(pos));
    output.drop_without_applying_deltas();
    click(&ctx, &mut app, size, pos);
    assert!(app.active_tab_mut().navigation.show_hidden_system);
    frame(
        &ctx,
        &mut app,
        size,
        vec![key_event(egui::Key::Escape, egui::Modifiers::NONE)],
    )
    .drop_without_applying_deltas();
    let output = frame(&ctx, &mut app, size, vec![]);
    let pos = text_rect(&output, "Layout").center();
    output.drop_without_applying_deltas();
    click(&ctx, &mut app, size, pos);
    let output = frame(&ctx, &mut app, size, vec![]);
    let pos = text_rect(&output, "Split left/right").center();
    output.drop_without_applying_deltas();
    click(&ctx, &mut app, size, pos);
    assert_eq!(app.panes.len(), 1);
    assert!(
        app.layout_error
            .as_deref()
            .unwrap()
            .contains("Cannot split")
    );
}

#[test]
fn s016_menu_keyboard_ownership_split_and_close_route_to_the_invoking_pane() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    let size = egui::vec2(1100.0, 800.0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    for menu in ["View", "Layout"] {
        click_label(&ctx, &mut app, size, menu);
        let key = app.active_tab_mut().navigation.key();
        frame(
            &ctx,
            &mut app,
            size,
            vec![
                key_event(egui::Key::F5, egui::Modifiers::NONE),
                key_event(egui::Key::T, egui::Modifiers::CTRL),
                key_event(egui::Key::ArrowDown, egui::Modifiers::NONE),
            ],
        )
        .drop_without_applying_deltas();
        assert_eq!(app.active_tab_mut().navigation.key(), key);
        assert_eq!(app.panes[0].tabs.len(), 1);
        assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
        frame(
            &ctx,
            &mut app,
            size,
            vec![
                key_event(egui::Key::Escape, egui::Modifiers::NONE),
                key_event(egui::Key::A, egui::Modifiers::CTRL),
            ],
        )
        .drop_without_applying_deltas();
        assert!(
            app.active_tab_mut().navigation.selection.paths.is_empty(),
            "Menu exit frame owns keys"
        );
    }
    click_label(&ctx, &mut app, size, "Layout");
    click_label(&ctx, &mut app, size, "Split left/right");
    assert_eq!(app.panes.len(), 2);
    assert_eq!(app.focused, 1);
    let left = app.panes[0].id;
    // The last drawn Layout belongs to the newly focused right pane.
    let output = frame(&ctx, &mut app, size, vec![]);
    let pos = output
        .shapes
        .iter()
        .rev()
        .find_map(|shape| match &shape.shape {
            egui::Shape::Text(text) if text.galley.job.text == "Layout" => {
                Some(text.pos + text.galley.size() / 2.0)
            }
            _ => None,
        })
        .unwrap();
    output.drop_without_applying_deltas();
    click(&ctx, &mut app, size, pos);
    click_label(&ctx, &mut app, size, "Close pane");
    assert_eq!(app.panes.len(), 1);
    assert_eq!(app.panes[0].id, left);
}

#[test]
fn s016_footer_clear_does_not_leave_a_stale_no_match_warning_in_the_same_frame() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    app.active_tab_mut().filter_text = "zz".into();
    app.active_tab_mut().navigation.set_filter("zz".into());
    let nav = &mut app.active_tab_mut().navigation;
    assert!(nav.apply_sorted(sorting::project_for_test(
        nav.key(),
        nav.current.raw_entries.clone(),
        &nav.filter
    )));
    let size = egui::vec2(1050.0, 700.0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    let output = frame(&ctx, &mut app, size, vec![]);
    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.starts_with("No names match"))));
    let pos = text_rect(&output, "Clear").center();
    output.drop_without_applying_deltas();
    frame(
        &ctx,
        &mut app,
        size,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    )
    .drop_without_applying_deltas();
    let output = frame(
        &ctx,
        &mut app,
        size,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert!(app.active_tab_mut().filter_text.is_empty());
    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.starts_with("Updating filter/sort/visibility"))));
    assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.starts_with("No names match"))));
    output.drop_without_applying_deltas();
}

#[test]
fn s016_long_warning_remains_truthful_and_short_body_scroll_reaches_footer() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    app.active_tab_mut().location_error = Some(format!(
        "Could not open generated fixture: {}",
        "detail ".repeat(100)
    ));
    let size = egui::vec2(320.0, 300.0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    let output = frame(&ctx, &mut app, size, vec![]);
    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("Could not open generated fixture"))));
    output.drop_without_applying_deltas();
    for _ in 0..12 {
        frame(
            &ctx,
            &mut app,
            size,
            vec![
                egui::Event::PointerMoved(egui::pos2(150.0, 150.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    phase: egui::TouchPhase::Move,
                    delta: egui::vec2(0.0, -1000.0),
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        )
        .drop_without_applying_deltas();
    }
    let output = frame(&ctx, &mut app, size, vec![]);
    let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
    for label in ["Filter", "Clear", "View"] {
        let rect = text_rect(&output, label);
        assert!(
            viewport.contains_rect(rect),
            "Body scroll must reach {label}: {rect:?}"
        );
    }
    assert_eq!(
        app.active_tab_mut().navigation.current.location,
        PathBuf::from("root")
    );
    assert!(app.active_tab_mut().location_error.is_some());
    output.drop_without_applying_deltas();
}

#[test]
fn s016_long_unicode_paths_keep_editor_and_breadcrumb_native_targets() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    let path = PathBuf::from(format!(r"C:\{}\日本語\é", "long-folder".repeat(20)));
    app.active_tab_mut().navigation.current.location = path.clone();
    app.active_tab_mut().location_text = native::display(path.as_os_str());
    let size = egui::vec2(320.0, 400.0);
    for _ in 0..3 {
        frame(&ctx, &mut app, size, vec![]).drop_without_applying_deltas();
    }
    assert_eq!(
        app.active_tab_mut().location_text,
        native::display(path.as_os_str())
    );
    let rect = ctx
        .read_response(app.active_tab_mut().widget_id("location"))
        .unwrap()
        .rect;
    assert!(app.pane_rects[0].1.contains_rect(rect));
    assert!(rect.width() >= 100.0);
    // Inspect the actual breadcrumb command without submitting a drive-root
    // enumeration. Native paths here are synthetic, never personal-file probes.
    let ctx = egui::Context::default();
    let id = app.active_tab_mut().id;
    let before = app.active_tab_mut().navigation.key();
    let mut command = None;
    let mut toolbar = |ui: &mut egui::Ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
            ui.spacing_mut().button_padding = egui::vec2(6.0, 3.0);
            command = ui
                .push_id(id, |ui| app.active_tab_mut().toolbar(ui, false))
                .inner
                .1;
        });
    };
    for _ in 0..3 {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            &mut toolbar,
        )
        .drop_without_applying_deltas();
    }
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        &mut toolbar,
    );
    let root = text_rect(&output, r"C:\").center();
    output.drop_without_applying_deltas();
    for pressed in [true, false] {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                events: vec![
                    egui::Event::PointerMoved(root),
                    egui::Event::PointerButton {
                        pos: root,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                ..Default::default()
            },
            &mut toolbar,
        )
        .drop_without_applying_deltas();
    }
    assert!(matches!(command, Some((target, Intent::New)) if target == PathBuf::from(r"C:\")));
    assert_eq!(app.active_tab_mut().navigation.current.location, path);
    assert_eq!(app.active_tab_mut().navigation.key(), before);
    assert!(app.active_tab_mut().navigation.pending_location().is_none());
}
