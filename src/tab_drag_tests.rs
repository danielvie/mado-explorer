use super::*;
use crate::{
    tab_drag::{Edge, TabDrag, TabDrop, TabStripGeometry},
    tests::{filter_app, key_event, test_app},
};

fn geometry(app: &mut Explorer, size: egui::Vec2) {
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, size.max(app.layout.minimum()));
    app.pane_rects.clear();
    app.layout
        .rectangles(bounds, &mut app.pane_rects, &mut Vec::new());
    app.visible_workspace = bounds;
    app.tab_strips = app
        .pane_rects
        .iter()
        .map(|(id, rect)| TabStripGeometry {
            pane: *id,
            rect: egui::Rect::from_min_size(
                rect.min + egui::vec2(5.0, 35.0),
                egui::vec2(rect.width() - 50.0, 26.0),
            ),
            tabs: Vec::new(),
        })
        .collect();
}

fn drag(app: &Explorer, pane: usize, tab: usize) -> TabDrag {
    TabDrag {
        source: app.panes[pane].id,
        tab: app.panes[pane].tabs[tab].id,
    }
}

fn render(ctx: &egui::Context, app: &mut Explorer, events: Vec<egui::Event>) {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1400.0, 1000.0),
            )),
            events,
            ..Default::default()
        },
        |ui| app.workspace(ui),
    )
    .drop_without_applying_deltas();
}

fn pointer(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn start_drag(ctx: &egui::Context, app: &mut Explorer, origin: egui::Pos2, target: egui::Pos2) {
    render(
        ctx,
        app,
        vec![egui::Event::PointerMoved(origin), pointer(origin, true)],
    );
    render(ctx, app, vec![egui::Event::PointerMoved(target)]);
    assert!(egui::DragAndDrop::has_payload_of_type::<TabDrag>(ctx));
}

fn tab_point(app: &Explorer, pane: PaneId, tab: usize) -> egui::Pos2 {
    app.tab_strips.iter().find(|s| s.pane == pane).unwrap().tabs[tab]
        .1
        .min
        + egui::vec2(30.0, 12.0)
}

fn assert_unique(app: &Explorer) {
    let mut ids = std::collections::HashSet::new();
    for p in &app.panes {
        assert!(!p.tabs.is_empty());
        assert!(p.active < p.tabs.len());
        assert!(app.layout.contains(p.id));
        for tab in &p.tabs {
            assert!(ids.insert(tab.id));
        }
    }
}

#[test]
fn s009_actual_pointer_strip_reorder_preserves_active_identity_and_click_distinction() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    app.new_tab();
    app.new_tab();
    let active = app.active_tab_mut().id;
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    let pane = app.panes[0].id;
    let first = app.panes[0].tabs[0].id;
    let origin = tab_point(&app, pane, 0);
    let end = app.tab_strips[0].tabs[2].1.max - egui::vec2(2.0, 12.0);
    start_drag(&ctx, &mut app, origin, end);
    render(&ctx, &mut app, vec![pointer(end, false)]);
    assert_eq!(
        app.panes[0].tabs.iter().map(|t| t.id).collect::<Vec<_>>(),
        [TabId(2), active, first]
    );
    assert_eq!(app.active_tab_mut().id, active);
    assert_eq!(app.panes[0].id, pane);
    assert!(!egui::DragAndDrop::has_any_payload(&ctx));
    // Plain click activates without reordering.
    render(&ctx, &mut app, vec![]);
    let click = tab_point(&app, pane, 2);
    render(
        &ctx,
        &mut app,
        vec![egui::Event::PointerMoved(click), pointer(click, true)],
    );
    render(&ctx, &mut app, vec![pointer(click, false)]);
    assert_eq!(app.active_tab_mut().id, first);
    assert_eq!(app.panes[0].tabs[2].id, first);
    assert_unique(&app);
}

#[test]
fn s009_actual_cross_strip_transfer_keeps_full_context_and_source_active_id() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    let rows = app.active_tab_mut().navigation.current.raw_entries.clone();
    let tab = app.active_tab_mut();
    tab.navigation.back.push(PathBuf::from("previous"));
    tab.navigation.forward.push(PathBuf::from("future"));
    tab.navigation.selection.click(&rows, 1, false, false);
    tab.navigation.set_filter("be".into());
    tab.filter_text = "be".into();
    tab.navigation.sort.toggle(Column::Size);
    tab.navigation.set_show_hidden_system(true);
    tab.location_text = "unsubmitted draft".into();
    tab.location_error = Some("draft error".into());
    tab.row_scroll_offset = 450.0;
    let id = tab.id;
    let selection = tab.navigation.selection.paths.clone();
    let active = tab.navigation.selection.active.clone();
    let anchor = tab.navigation.selection.anchor.clone();
    let sort = tab.navigation.sort;
    app.new_tab();
    app.panes[0].tabs[0].navigation.error = Some((
        PathBuf::from("failed-destination"),
        directory::Failure {
            kind: directory::FailureKind::Other,
            detail: "retained navigation error".into(),
        },
    ));
    let source_active = app.active_tab_mut().id;
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    let origin = tab_point(&app, PaneId(1), 0);
    let destination = app.panes[1].id;
    let target = app
        .tab_strips
        .iter()
        .find(|s| s.pane == destination)
        .unwrap()
        .rect
        .center();
    start_drag(&ctx, &mut app, origin, target);
    render(&ctx, &mut app, vec![pointer(target, false)]);
    assert_eq!(app.panes[0].tabs[app.panes[0].active].id, source_active);
    assert_eq!(app.panes[app.focused].id, destination);
    let tab = app.active_tab_mut();
    assert_eq!(tab.id, id);
    assert_eq!(tab.navigation.current.location, PathBuf::from("root"));
    assert_eq!(tab.navigation.back, [PathBuf::from("previous")]);
    assert_eq!(tab.navigation.forward, [PathBuf::from("future")]);
    assert_eq!(tab.navigation.selection.paths, selection);
    assert_eq!(tab.navigation.selection.active, active);
    assert_eq!(tab.navigation.selection.anchor, anchor);
    assert_eq!(tab.navigation.filter, "be");
    assert_eq!(tab.filter_text, "be");
    assert_eq!(tab.navigation.sort, sort);
    assert!(tab.navigation.show_hidden_system);
    assert_eq!(tab.location_text, "unsubmitted draft");
    assert_eq!(tab.location_error.as_deref(), Some("draft error"));
    assert_eq!(
        tab.navigation.error.as_ref().unwrap().1.detail,
        "retained navigation error"
    );
    assert_eq!(tab.row_scroll_offset, 450.0);
    assert!(tab.restore_scroll);
    assert_unique(&app);
}

#[test]
fn s009_source_last_collapse_resolves_destination_after_index_shift_and_preserves_live_target() {
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    assert!(app.split_pane(Axis::TopBottom));
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    let unrelated = app.panes[1].tabs[0].navigation.key();
    app.focus_pane(0);
    app.navigate(PathBuf::from("still-opening"), Intent::New);
    let moved = drag(&app, 0, 0);
    let generation = app.active_tab_mut().navigation.key();
    let dest = app.panes[2].id;
    assert!(app.drop_tab(
        moved,
        TabDrop::Strip {
            pane: dest,
            before: None
        }
    ));
    assert_eq!(app.panes.len(), 2);
    assert_eq!(app.panes[app.focused].id, dest);
    assert_eq!(app.active_tab_mut().id, moved.tab);
    assert_eq!(app.active_tab_mut().navigation.key(), generation);
    assert_eq!(
        app.active_tab_mut().navigation.pending_location(),
        Some(std::path::Path::new("still-opening"))
    );
    assert_eq!(app.panes[0].tabs[0].navigation.key(), unrelated);
    assert_unique(&app);
}

#[test]
fn s009_all_four_pointer_edge_splits_replenish_lone_own_source() {
    for edge in [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom] {
        let ctx = egui::Context::default();
        let mut app = filter_app();
        let original = drag(&app, 0, 0);
        render(&ctx, &mut app, vec![]);
        render(&ctx, &mut app, vec![]);
        let rect = app.pane_rects[0].1;
        let target = match edge {
            Edge::Left => egui::pos2(rect.left() + 2.0, rect.center().y),
            Edge::Right => egui::pos2(rect.right() - 2.0, rect.center().y),
            Edge::Top => egui::pos2(rect.center().x, rect.top() + 2.0),
            Edge::Bottom => egui::pos2(rect.center().x, rect.bottom() - 2.0),
        };
        let origin = tab_point(&app, original.source, 0);
        start_drag(&ctx, &mut app, origin, target);
        render(&ctx, &mut app, vec![pointer(target, false)]);
        assert_eq!(app.panes.len(), 2, "{edge:?}");
        assert_eq!(app.active_tab_mut().id, original.tab);
        assert_eq!(app.panes[0].id, original.source);
        assert_ne!(app.panes[0].tabs[0].id, original.tab);
        assert_eq!(
            app.panes[0].tabs[0].navigation.current.location,
            PathBuf::from("root")
        );
        assert!(app.panes[0].tabs[0].navigation.back.is_empty());
        geometry(&mut app, egui::vec2(1400.0, 1000.0));
        let a = app
            .pane_rects
            .iter()
            .find(|(id, _)| *id == original.source)
            .unwrap()
            .1;
        let b = app
            .pane_rects
            .iter()
            .find(|(id, _)| *id == app.panes[app.focused].id)
            .unwrap()
            .1;
        match edge {
            Edge::Left => assert!(b.right() < a.left()),
            Edge::Right => assert!(a.right() < b.left()),
            Edge::Top => assert!(b.bottom() < a.top()),
            Edge::Bottom => assert!(a.bottom() < b.top()),
        }
        assert_unique(&app);
    }
}

#[test]
fn s009_edge_transfer_from_other_last_source_collapses_only_source() {
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    let moved = drag(&app, 0, 0);
    let dest = app.panes[1].id;
    assert!(app.drop_tab(
        moved,
        TabDrop::Edge {
            pane: dest,
            edge: Edge::Top
        }
    ));
    assert_eq!(app.panes.len(), 2);
    assert!(!app.layout.contains(moved.source));
    assert!(app.layout.contains(dest));
    assert_eq!(app.active_tab_mut().id, moved.tab);
    assert_unique(&app);
}

#[test]
fn s009_minimum_offscreen_stale_closed_drops_are_atomic() {
    let mut app = filter_app();
    for edge in [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom] {
        geometry(&mut app, panes::MIN_PANE);
        let moved = drag(&app, 0, 0);
        let key = app.active_tab_mut().navigation.key();
        let ids = (app.next_tab_id, app.next_pane_id);
        assert!(!app.drop_tab(
            moved,
            TabDrop::Edge {
                pane: moved.source,
                edge
            }
        ));
        assert!(app.layout_error.as_ref().unwrap().contains("minimum"));
        assert_eq!(app.active_tab_mut().navigation.key(), key);
        assert_eq!((app.next_tab_id, app.next_pane_id), ids);
        assert_eq!(app.panes.len(), 1);
    }
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    let moved = drag(&app, 0, 0);
    app.visible_workspace.max.x -= 10.0;
    assert!(!app.drop_tab(
        moved,
        TabDrop::Edge {
            pane: moved.source,
            edge: Edge::Right
        }
    ));
    assert!(app.layout_error.as_ref().unwrap().contains("offscreen"));
    app.pane_rects.clear();
    assert!(!app.drop_tab(
        moved,
        TabDrop::Edge {
            pane: moved.source,
            edge: Edge::Right
        }
    ));
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    let closed = app.panes[1].id;
    app.close_pane();
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(!app.drop_tab(
        moved,
        TabDrop::Strip {
            pane: closed,
            before: None
        }
    ));
    assert!(!app.drop_tab(
        moved,
        TabDrop::Strip {
            pane: moved.source,
            before: Some(TabId(999))
        }
    ));
    app.close_tab(0);
    assert!(!app.drop_tab(
        moved,
        TabDrop::Strip {
            pane: moved.source,
            before: None
        }
    ));
    assert_unique(&app);
}

#[test]
fn s009_escape_outside_release_closed_source_and_foreign_payload_do_not_move_tabs() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    app.new_tab();
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    let original = app.panes[0].tabs.iter().map(|t| t.id).collect::<Vec<_>>();
    let origin = tab_point(&app, PaneId(1), 0);
    let target = app.tab_strips[0].rect.center();
    start_drag(&ctx, &mut app, origin, target);
    render(
        &ctx,
        &mut app,
        vec![key_event(egui::Key::Escape, egui::Modifiers::NONE)],
    );
    assert!(!egui::DragAndDrop::has_any_payload(&ctx));
    render(&ctx, &mut app, vec![pointer(target, false)]);
    assert_eq!(
        app.panes[0].tabs.iter().map(|t| t.id).collect::<Vec<_>>(),
        original
    );
    start_drag(&ctx, &mut app, origin, egui::pos2(-10.0, -10.0));
    render(
        &ctx,
        &mut app,
        vec![pointer(egui::pos2(-10.0, -10.0), false)],
    );
    assert!(app.layout_error.as_ref().unwrap().contains("cancelled"));
    assert_eq!(app.panes[0].tabs.len(), 2);
    // Visible cancellation feedback shifts the workspace; use fresh coordinates.
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    let origin = tab_point(&app, PaneId(1), 0);
    let target = app.tab_strips[0].rect.center();
    start_drag(&ctx, &mut app, origin, target);
    app.close_tab(0);
    let survivor = app.active_tab_mut().id;
    render(&ctx, &mut app, vec![pointer(target, false)]);
    assert_eq!(app.active_tab_mut().id, survivor);
    assert!(!egui::DragAndDrop::has_any_payload(&ctx));
    egui::DragAndDrop::set_payload(&ctx, PathBuf::from("future-file-payload"));
    render(&ctx, &mut app, vec![pointer(target, false)]);
    assert_eq!(app.active_tab_mut().id, survivor);
    assert_unique(&app);
}

#[test]
fn s009_closed_hover_destination_cannot_retarget_release_to_expanded_sibling() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    let original = drag(&app, 0, 0);
    let dest = app.panes[1].id;
    let target = app
        .tab_strips
        .iter()
        .find(|s| s.pane == dest)
        .unwrap()
        .rect
        .center();
    let origin = tab_point(&app, original.source, 0);
    start_drag(&ctx, &mut app, origin, target);
    app.focus_pane(1);
    app.close_pane();
    render(&ctx, &mut app, vec![pointer(target, false)]);
    assert_eq!(app.panes.len(), 1);
    assert_eq!(app.active_tab_mut().id, original.tab);
    assert!(app.layout_error.as_ref().unwrap().contains("destination"));
    assert!(!egui::DragAndDrop::has_any_payload(&ctx));
    assert_unique(&app);
}

#[test]
fn s009_active_source_replacement_and_reorder_of_inactive_tab_do_not_retarget_neighbors() {
    let mut app = filter_app();
    app.new_tab();
    app.new_tab();
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    let moved = drag(&app, 0, 2);
    let old_generation = app.panes[0].tabs[1].navigation.key();
    assert!(app.split_pane(Axis::LeftRight));
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    let dest = app.panes[1].id;
    assert!(app.drop_tab(
        moved,
        TabDrop::Strip {
            pane: dest,
            before: None
        }
    ));
    assert_eq!(app.panes[0].active, 1);
    assert_ne!(app.panes[0].tabs[1].navigation.key(), old_generation);
    let source_active = app.panes[0].tabs[1].id;
    let source_key = app.panes[0].tabs[1].navigation.key();
    let dest_key = app.active_tab_mut().navigation.key();
    geometry(&mut app, egui::vec2(2000.0, 1000.0));
    let inactive = drag(&app, 0, 0);
    assert!(app.drop_tab(
        inactive,
        TabDrop::Strip {
            pane: inactive.source,
            before: None
        }
    ));
    assert_eq!(app.panes[0].tabs[app.panes[0].active].id, source_active);
    assert_eq!(
        app.panes[0].tabs[app.panes[0].active].navigation.key(),
        source_key
    );
    assert_eq!(
        app.panes[1].tabs[app.panes[1].active].navigation.key(),
        dest_key
    );
    assert_unique(&app);
}

#[test]
fn s009_real_worker_generation_routing_and_manual_scroll_restoration_after_move() {
    use std::{
        fs,
        time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    };
    let root = std::env::temp_dir().join(format!(
        "mado-s009-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&root).unwrap();
    for i in 0..150 {
        fs::write(root.join(format!("entry-{i:03}.txt")), b"fixture").unwrap();
    }
    let mut app = test_app(root.clone());
    app.navigate(root.clone(), Intent::Refresh);
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(app.split_pane(Axis::LeftRight));
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        app.poll_directory();
        if app
            .panes
            .iter()
            .all(|p| p.tabs[p.active].scroll_restore_ready())
        {
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    app.focus_pane(0);
    app.active_tab_mut().row_scroll_offset = 600.0;
    let rows = app.active_tab_mut().navigation.current.entries.clone();
    app.active_tab_mut()
        .navigation
        .selection
        .click(&rows, 0, false, false);
    app.new_tab();
    let stale = app.panes[0].tabs[0].navigation.key();
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    let moved = drag(&app, 0, 0);
    let dest = app.panes[1].id;
    assert!(app.drop_tab(
        moved,
        TabDrop::Strip {
            pane: dest,
            before: None
        }
    ));
    assert_ne!(
        app.active_tab_mut().navigation.key().generation,
        stale.generation
    );
    assert!(!app.active_tab_mut().apply(directory::Response {
        generation: stale.generation,
        update: directory::Update::Finished(Ok(PathBuf::from("stale-location")))
    }));
    assert!(
        !app.active_tab_mut()
            .navigation
            .apply_sorted(sorting::project_for_test(stale, rows, ""))
    );
    let ctx = egui::Context::default();
    loop {
        app.poll_directory();
        if app
            .panes
            .iter()
            .all(|p| p.tabs[p.active].scroll_restore_ready())
        {
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    assert_eq!(app.active_tab_mut().id, moved.tab);
    assert!((app.active_tab_mut().row_scroll_offset - 600.0).abs() < 1.0);
    assert_eq!(app.active_tab_mut().navigation.selection.paths.len(), 1);
    assert_eq!(app.active_tab_mut().navigation.current.location, root);
    assert!(
        app.panes
            .iter()
            .all(|p| p.tabs[p.active].navigation.current.entries.len() == 150)
    );
    // Move an active refresh while its real worker response is outstanding.
    // Generation routing follows the original context, not the old pane index.
    app.navigate(root.clone(), Intent::Refresh);
    let live_key = app.active_tab_mut().navigation.key();
    let source = app.focused;
    let active = app.panes[source].active;
    let live_drag = drag(&app, source, active);
    let other = app
        .panes
        .iter()
        .find(|p| p.id != live_drag.source)
        .unwrap()
        .id;
    geometry(&mut app, egui::vec2(1400.0, 1000.0));
    assert!(app.drop_tab(
        live_drag,
        TabDrop::Strip {
            pane: other,
            before: None
        }
    ));
    assert_eq!(app.active_tab_mut().navigation.key(), live_key);
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        app.poll_directory();
        if app
            .panes
            .iter()
            .all(|p| p.tabs[p.active].scroll_restore_ready())
        {
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(app.active_tab_mut().id, live_drag.tab);
    assert_eq!(app.active_tab_mut().navigation.current.location, root);
    assert_eq!(app.active_tab_mut().navigation.current.entries.len(), 150);
    assert_unique(&app);
    drop(app);
    fs::remove_dir_all(root).unwrap();
}
