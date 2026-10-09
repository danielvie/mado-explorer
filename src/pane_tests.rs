use super::*;
use crate::tests::{filter_app, key_event, test_app};
use std::{
    fs,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

fn geometry(app: &mut Explorer, size: egui::Vec2) {
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, size.max(app.layout.minimum()));
    app.pane_rects.clear();
    app.layout
        .rectangles(rect, &mut app.pane_rects, &mut Vec::new());
    app.visible_workspace = rect;
}

fn finish(tab: &mut Tab, rows: Arc<Vec<Arc<directory::Entry>>>) {
    let generation = tab.navigation.key().generation;
    tab.apply(directory::Response {
        generation,
        update: directory::Update::Snapshot {
            entries: rows,
            metadata_errors: 0,
        },
    });
    let location = tab.navigation.visible().location.clone();
    tab.apply(directory::Response {
        generation,
        update: directory::Update::Finished(Ok(location)),
    });
    let nav = &mut tab.navigation;
    assert!(nav.apply_sorted(sorting::project_for_test(
        nav.key(),
        nav.current.raw_entries.clone(),
        &nav.filter
    )));
}

#[test]
fn s008_both_axes_nested_minimums_divider_clamps_and_collapse() {
    let mut tree = Layout::Pane(PaneId(1));
    assert!(!tree.split(PaneId(1), PaneId(2), Axis::LeftRight, panes::MIN_PANE));
    assert!(tree.split(
        PaneId(1),
        PaneId(2),
        Axis::LeftRight,
        egui::vec2(1000.0, 700.0)
    ));
    assert!(tree.split(
        PaneId(2),
        PaneId(3),
        Axis::TopBottom,
        egui::vec2(497.0, 700.0)
    ));
    assert_eq!(tree.minimum(), egui::vec2(646.0, 606.0));
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1000.0, 700.0));
    let mut leaves = Vec::new();
    let mut dividers = Vec::new();
    tree.rectangles(bounds, &mut leaves, &mut dividers);
    assert_eq!(
        leaves.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        [PaneId(1), PaneId(2), PaneId(3)]
    );
    for divider in &dividers {
        tree.resize(divider, egui::pos2(-1000.0, -1000.0));
        tree.resize(divider, egui::pos2(9000.0, 9000.0));
    }
    leaves.clear();
    tree.rectangles(bounds, &mut leaves, &mut Vec::new());
    for (index, (_, rect)) in leaves.iter().enumerate() {
        assert!(rect.width() >= panes::MIN_PANE.x && rect.height() >= panes::MIN_PANE.y);
        assert!(bounds.contains_rect(*rect));
        for (_, other) in &leaves[index + 1..] {
            assert!(!rect.intersects(*other));
        }
    }
    assert!(tree.remove(PaneId(2)));
    assert_eq!(tree.minimum(), egui::vec2(646.0, 300.0));
    assert!(tree.remove(PaneId(1)));
    assert!(matches!(tree, Layout::Pane(PaneId(3))));
    assert!(!tree.remove(PaneId(3)));
}

#[test]
fn s008_committed_source_focus_tab_commands_close_and_final_home_are_isolated() {
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    let rows = app.active_tab_mut().navigation.current.raw_entries.clone();
    app.active_tab_mut().navigation.selection.select_all(&rows);
    app.active_tab_mut().navigation.set_filter("al".into());
    app.active_tab_mut().location_text = "retained draft".into();
    app.active_tab_mut().row_scroll_offset = 500.0;
    app.navigate(PathBuf::from("uncommitted"), Intent::New);
    assert!(app.split_pane(Axis::LeftRight));
    assert_eq!(app.focused, 1);
    assert_eq!(
        app.active_tab_mut().navigation.current.location,
        PathBuf::from("root")
    );
    assert!(app.active_tab_mut().navigation.filter.is_empty());
    assert!(app.panes[0].tabs[0].navigation.pending_location().is_some());
    assert_eq!(app.panes[0].tabs[0].location_text, "retained draft");
    assert_eq!(app.panes[0].tabs[0].row_scroll_offset, 500.0);
    let selection = app.panes[0].tabs[0].navigation.selection.paths.clone();
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    app.cycle_pane();
    assert_eq!(app.focused, 0);
    app.cycle_pane();
    assert_eq!(app.focused, 1);
    app.apply_tab_shortcut(TabShortcut::New);
    assert_eq!(app.panes[1].tabs.len(), 2);
    assert_eq!(app.panes[0].tabs.len(), 1);
    assert_eq!(app.panes[0].tabs[0].navigation.selection.paths, selection);
    assert!(app.split_pane(Axis::TopBottom));
    let removed_id = app.panes[2].id;
    let removed_tab = app.panes[2].tabs[0].id;
    app.close_tab(0);
    assert_eq!(app.panes.len(), 2);
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(!app.pane_rects.iter().any(|(id, _)| *id == removed_id));
    assert!(app.split_pane(Axis::TopBottom));
    assert!(app.panes[2].id.0 > removed_id.0);
    assert!(app.panes[2].tabs[0].id.0 > removed_tab.0);
    app.close_pane();
    app.focus_pane(1);
    app.close_pane();
    assert_eq!(app.panes.len(), 1);
    app.home = PathBuf::from("fixture-home");
    app.close_tab(0);
    assert_eq!(
        app.active_tab_mut().navigation.current.location,
        PathBuf::from("fixture-home")
    );
    assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
    assert!(app.active_tab_mut().navigation.filter.is_empty());
}

fn render(
    context: &egui::Context,
    app: &mut Explorer,
    size: egui::Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 3.0);
            app.workspace(ui);
        },
    )
}

#[test]
fn s008_pointer_focus_precedes_shortcuts_f6_and_navigation_target_only_one_pane() {
    let context = egui::Context::default();
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(1100.0, 800.0));
    assert!(app.split_pane(Axis::LeftRight));
    let rows = selection::tests::rows(&["alpha", "beta"]);
    finish(&mut app.panes[0].tabs[0], rows.clone());
    finish(&mut app.panes[1].tabs[0], rows.clone());
    app.focus_pane(0);
    render(&context, &mut app, egui::vec2(1100.0, 800.0), vec![]).drop_without_applying_deltas();
    let right = app.pane_rects[1].1;
    let click = egui::pos2(right.right() - 15.0, right.top() + 12.0);
    render(
        &context,
        &mut app,
        egui::vec2(1100.0, 800.0),
        vec![
            egui::Event::PointerMoved(click),
            egui::Event::PointerButton {
                pos: click,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            key_event(egui::Key::T, egui::Modifiers::CTRL),
        ],
    )
    .drop_without_applying_deltas();
    assert_eq!(app.focused, 1);
    assert_eq!(app.panes[0].tabs.len(), 1);
    assert_eq!(app.panes[1].tabs.len(), 2);
    render(
        &context,
        &mut app,
        egui::vec2(1100.0, 800.0),
        vec![
            egui::Event::PointerButton {
                pos: click,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
            key_event(egui::Key::F6, egui::Modifiers::NONE),
            key_event(egui::Key::A, egui::Modifiers::CTRL),
        ],
    )
    .drop_without_applying_deltas();
    assert_eq!(app.focused, 0);
    assert_eq!(app.panes[0].tabs[0].navigation.selection.paths.len(), 2);
    assert!(app.panes[1].tabs[0].navigation.selection.paths.is_empty());
    let generation = app.panes[1].tabs[1].navigation.key().generation;
    render(
        &context,
        &mut app,
        egui::vec2(1100.0, 800.0),
        vec![key_event(egui::Key::F5, egui::Modifiers::NONE)],
    )
    .drop_without_applying_deltas();
    assert_ne!(app.panes[0].tabs[0].navigation.key().generation, 0);
    assert_eq!(app.panes[1].tabs[1].navigation.key().generation, generation);
}

#[test]
fn s008_narrow_name_first_layout_and_shrink_keep_panes_accessible() {
    for width in [200.0, 284.0, 310.0, 480.0, 720.0, 1000.0] {
        let columns = table_columns(width, 8.0);
        assert!(columns[0] >= 200.0);
        let count = columns.iter().filter(|w| **w > 0.0).count();
        assert!((columns.iter().sum::<f32>() + (count - 1) as f32 * 8.0 - width).abs() < 0.1);
        if width < 480.0 {
            assert_eq!(columns[1], 0.0);
            assert_eq!(columns[2], 0.0);
        }
    }
    let context = egui::Context::default();
    let mut app = filter_app();
    let mut marked_rows = selection::tests::rows(&["folder", "file"]);
    for (index, row) in Arc::get_mut(&mut marked_rows)
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        let row = Arc::get_mut(row).unwrap();
        row.name = native::display(row.path.as_os_str());
        row.kind = if index == 0 {
            EntryKind::Folder
        } else {
            EntryKind::File
        };
    }
    finish(app.active_tab_mut(), marked_rows);
    let output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 400.0),
            )),
            ..Default::default()
        },
        |ui| {
            app.table(ui, false);
        },
    );
    let text: Vec<_> = output
        .shapes
        .iter()
        .filter_map(|shape| match &shape.shape {
            egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
            _ => None,
        })
        .collect();
    output.drop_without_applying_deltas();
    assert!(text.iter().any(|text| text.starts_with("[D] folder")));
    assert!(text.iter().any(|text| text.starts_with("[F] file")));
    assert!(
        !text
            .iter()
            .any(|text| text.starts_with("Type") || text.starts_with("Modified (UTC)"))
    );
    geometry(&mut app, egui::vec2(1300.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    geometry(&mut app, egui::vec2(1300.0, 1000.0));
    assert!(app.split_pane(Axis::TopBottom));
    let rows = selection::tests::rows(&["folder", "file"]);
    for p in &mut app.panes {
        finish(&mut p.tabs[0], rows.clone());
    }
    // Existing layout scrolls at its recursive minimum instead of compressing controls.
    for _ in 0..5 {
        render(&context, &mut app, egui::vec2(480.0, 280.0), vec![]).drop_without_applying_deltas();
    }
    assert_eq!(app.pane_rects.len(), 3);
    for (_, rect) in &app.pane_rects {
        assert!(rect.width() >= 320.0 && rect.height() >= 300.0);
    }
    assert!(!app.split_pane(Axis::LeftRight));
    assert!(app.layout_error.is_some());
    for _ in 0..3 {
        render(
            &context,
            &mut app,
            egui::vec2(480.0, 280.0),
            vec![key_event(egui::Key::F6, egui::Modifiers::NONE)],
        )
        .drop_without_applying_deltas();
        for _ in 0..10 {
            render(&context, &mut app, egui::vec2(480.0, 280.0), vec![])
                .drop_without_applying_deltas();
        }
        let id = app.panes[app.focused].id;
        let rect = app
            .pane_rects
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .unwrap()
            .1;
        let indicator =
            egui::Rect::from_min_size(rect.min + egui::vec2(5.0, 5.0), egui::vec2(240.0, 28.0));
        assert!(
            app.visible_workspace.contains_rect(indicator),
            "F6 must reveal the focused pane indicator: {indicator:?} in {:?}",
            app.visible_workspace
        );
    }
}

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mado-s008-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn settle_all(app: &mut Explorer) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        app.poll_directory();
        if app.panes.iter().all(|p| {
            let nav = &p.tabs[p.active].navigation;
            nav.pending_location().is_none()
                && matches!(nav.current.status, Status::Ready)
                && nav.projection_fresh()
        }) {
            return;
        }
        assert!(Instant::now() < deadline, "concurrent panes did not settle");
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn s008_concurrent_load_projection_switch_close_and_stale_rejection() {
    let fixture = Fixture::new();
    for dir in ["left", "right"] {
        fs::create_dir(fixture.0.join(dir)).unwrap();
        for name in ["alpha.txt", "beta.txt", "日本語.txt"] {
            fs::write(fixture.0.join(dir).join(name), b"fixture").unwrap();
        }
    }
    let mut app = test_app(fixture.0.clone());
    geometry(&mut app, egui::vec2(1200.0, 800.0));
    app.navigate(fixture.0.join("left"), Intent::New);
    assert!(app.split_pane(Axis::LeftRight));
    app.navigate(fixture.0.join("right"), Intent::New);
    // Both destinations start before any polling. Independent targets must complete.
    settle_all(&mut app);
    for p in &app.panes {
        assert_eq!(p.tabs[0].navigation.current.entries.len(), 3);
    }
    assert_eq!(
        app.panes[0].tabs[0].navigation.current.location,
        fixture.0.join("left")
    );
    assert_eq!(
        app.panes[1].tabs[0].navigation.current.location,
        fixture.0.join("right")
    );
    for revision in 0..30 {
        app.panes[0].tabs[0]
            .navigation
            .set_filter(if revision % 2 == 0 { "beta" } else { "alpha" }.into());
        app.panes[1].tabs[0]
            .navigation
            .set_filter(if revision % 2 == 0 { "alpha" } else { "beta" }.into());
        app.request_sort();
    }
    settle_all(&mut app);
    assert_eq!(
        app.panes[0].tabs[0].navigation.current.entries[0].name,
        "alpha.txt"
    );
    assert_eq!(
        app.panes[1].tabs[0].navigation.current.entries[0].name,
        "beta.txt"
    );
    app.focus_pane(0);
    let old_key = app.panes[0].tabs[0].navigation.key();
    let old_rows = app.panes[0].tabs[0].navigation.current.raw_entries.clone();
    app.new_tab();
    app.navigate(fixture.0.join("left"), Intent::Refresh);
    let neighbor_key = app.panes[1].tabs[0].navigation.key();
    settle_all(&mut app);
    assert_eq!(app.panes[1].tabs[0].navigation.key(), neighbor_key);
    assert!(!app.active_tab_mut().apply(directory::Response {
        generation: old_key.generation,
        update: directory::Update::Finished(Ok(PathBuf::from("stale")))
    }));
    assert!(
        !app.active_tab_mut()
            .navigation
            .apply_sorted(sorting::Sorted::for_test(old_key, old_rows.clone()))
    );
    let closed_key = app.active_tab_mut().navigation.key();
    app.close_pane();
    settle_all(&mut app);
    assert_eq!(app.panes.len(), 1);
    assert_eq!(app.active_tab_mut().navigation.key(), neighbor_key);
    assert!(
        !app.active_tab_mut()
            .navigation
            .apply_sorted(sorting::Sorted::for_test(closed_key, old_rows))
    );
    assert_eq!(
        app.active_tab_mut().navigation.current.entries[0].name,
        "beta.txt"
    );
}

#[test]
fn s008_captured_launcher_survives_source_pane_close_without_retargeting() {
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(1200.0, 800.0));
    assert!(app.split_pane(Axis::LeftRight));
    let captured = PathBuf::from("captured-file");
    let (send, receive) = std::sync::mpsc::channel();
    app.launcher
        .request_with(captured.clone(), Arc::new(|| {}), move |path| {
            receive.recv().unwrap();
            assert_eq!(path, PathBuf::from("captured-file"));
            Err(std::io::Error::from_raw_os_error(5))
        });
    app.close_pane();
    let key = app.active_tab_mut().navigation.key();
    let location = app.active_tab_mut().navigation.current.location.clone();
    send.send(()).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while app.launcher.busy_path().is_some() {
        app.launcher.poll();
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(app.launcher.report.as_ref().unwrap().path, captured);
    assert_eq!(app.active_tab_mut().navigation.key(), key);
    assert_eq!(app.active_tab_mut().navigation.current.location, location);
}

#[test]
fn s008_first_click_in_inactive_pane_selects_its_row_without_a_second_click() {
    let context = egui::Context::default();
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(1100.0, 800.0));
    assert!(app.split_pane(Axis::LeftRight));
    let mut rows = selection::tests::rows(&["right-row"]);
    Arc::get_mut(&mut Arc::get_mut(&mut rows).unwrap()[0])
        .unwrap()
        .name = "right-row".into();
    finish(&mut app.panes[1].tabs[0], rows);
    app.focus_pane(0);
    let mut row = None;
    for _ in 0..3 {
        let output = render(&context, &mut app, egui::vec2(1100.0, 800.0), vec![]);
        for shape in &output.shapes {
            if let egui::Shape::Text(text) = &shape.shape {
                if text.galley.job.text == "right-row" {
                    row = Some(text.pos + text.galley.size() / 2.0);
                }
            }
        }
        output.drop_without_applying_deltas();
    }
    let row = row.expect("right pane row is visible");
    for pressed in [true, false] {
        render(
            &context,
            &mut app,
            egui::vec2(1100.0, 800.0),
            vec![
                egui::Event::PointerMoved(row),
                egui::Event::PointerButton {
                    pos: row,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        )
        .drop_without_applying_deltas();
    }
    assert_eq!(app.focused, 1);
    assert_eq!(
        app.panes[1].tabs[0].navigation.selection.paths,
        [PathBuf::from("right-row")].into_iter().collect()
    );
    assert!(app.panes[0].tabs[0].navigation.selection.paths.is_empty());
}
