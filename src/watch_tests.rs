//! S018 uses only create-new owned containers. No application startup, settings,
//! common-place inventory, personal clipboard, drive enumeration or GUI launch.
use super::*;
use mado_native_directory_watch::{Failure, MAX_TARGETS, Refusal, Report, Target, TargetReport};
use std::{
    fs, thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Fixture {
    root: PathBuf,
    parent: PathBuf,
    a: PathBuf,
    b: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("mado-s018-{}-{nonce}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let parent = root.join("owned-parent");
        fs::create_dir(&parent).unwrap();
        let a = parent.join("données-日本-𐐀");
        let b = parent.join("b");
        fs::create_dir(&a).unwrap();
        fs::create_dir(&b).unwrap();
        fs::write(root.join("sentinel"), b"unchanged").unwrap();
        Self { root, parent, a, b }
    }
    fn app(&self) -> Explorer {
        let mut app = tests::test_app(self.a.clone());
        app.home = self.a.clone();
        app.watch = watch_adapter::WatchAdapter::enable_owned(&self.root, &[app.watch_context(0)]);
        app.navigate(self.a.clone(), Intent::Refresh);
        wait(&mut app, settled);
        app
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        assert_eq!(fs::read(self.root.join("sentinel")).unwrap(), b"unchanged");
        fs::remove_dir_all(&self.root).unwrap();
    }
}
fn settled(app: &Explorer) -> bool {
    app.panes.iter().all(|p| {
        let nav = &p.tabs[p.active].navigation;
        nav.pending_location().is_none()
            && !matches!(nav.current.status, Status::Loading)
            && nav.projection_fresh()
    }) && app
        .watch
        .bindings
        .iter()
        .all(|b| b.status.is_some() && !b.dirty)
}
fn wait(app: &mut Explorer, predicate: impl Fn(&Explorer) -> bool) {
    let until = Instant::now() + Duration::from_secs(10);
    let ctx = egui::Context::default();
    let mut frame = eframe::Frame::_new_kittest();
    loop {
        eframe::App::logic(app, &ctx, &mut frame);
        if predicate(app) {
            return;
        }
        assert!(
            Instant::now() < until,
            "S018 timeout: {}",
            app.watch.message(&app.watch_context(0))
        );
        thread::sleep(Duration::from_millis(10));
    }
}
fn rows(app: &Explorer) -> Vec<PathBuf> {
    app.panes[0].tabs[app.panes[0].active]
        .navigation
        .current
        .entries
        .iter()
        .map(|r| r.path.clone())
        .collect()
}
fn ready(app: &Explorer) -> bool {
    app.watch
        .bindings
        .iter()
        .all(|b| b.status.as_ref().is_some_and(|s| s.ready))
}
fn stop(app: &mut Explorer) {
    let before = Instant::now();
    app.watch.stop();
    assert!(
        before.elapsed() < Duration::from_millis(100),
        "stop request blocked"
    );
    assert!(app.watch.bindings.is_empty());
    let until = Instant::now() + Duration::from_secs(10);
    while !app.watch.join_owned_cleanup() {
        assert!(
            Instant::now() < until,
            "owned local cleanup did not acknowledge"
        );
        thread::sleep(Duration::from_millis(10));
    }
    app.watch.drain();
    assert!(app.watch.cleanup_acknowledged);
}
fn status(app: &Explorer, reconcile: bool) -> TargetReport {
    TargetReport {
        revision: app.watch.revision,
        target: app.watch.bindings[0].target.clone(),
        reconcile,
        full_reconcile: reconcile,
        overflow: false,
        ready: true,
        directory_error: None,
        parent_error: None,
    }
}
fn inject(app: &mut Explorer, status: TargetReport) {
    app.watch.accept(Report {
        revision: app.watch.revision,
        targets: vec![status],
        driver_errors: vec![],
        omitted_driver_errors: 0,
        stopped: false,
    });
}
fn text(output: &egui::FullOutput, value: &str) -> bool {
    output
        .shapes
        .iter()
        .any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.job.text.contains(value)))
}

#[test]
fn s018_native_create_rename_delete_reconcile_visible_rows_and_path_selection() {
    let f = Fixture::new();
    let mut app = f.app();
    wait(&mut app, ready);
    let keep = f.a.join("keep-é-𐐀.txt");
    fs::write(&keep, b"owned").unwrap();
    wait(&mut app, |app| rows(app).contains(&keep) && settled(app));
    let nav = &mut app.active_tab_mut().navigation;
    nav.selection.select_all(&nav.current.entries);
    let fresh = f.a.join("fresh.txt");
    fs::write(&fresh, b"new").unwrap();
    wait(&mut app, |app| rows(app).contains(&fresh) && settled(app));
    assert_eq!(app.active_tab_mut().navigation.selection.paths.len(), 1);
    assert!(
        app.active_tab_mut()
            .navigation
            .selection
            .paths
            .contains(&keep)
    );
    let renamed = f.a.join("renamed.txt");
    fs::rename(&fresh, &renamed).unwrap();
    wait(&mut app, |app| {
        rows(app).contains(&renamed) && !rows(app).contains(&fresh) && settled(app)
    });
    fs::remove_file(&keep).unwrap();
    wait(&mut app, |app| !rows(app).contains(&keep) && settled(app));
    assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
    assert!(app.active_tab_mut().navigation.back.is_empty());
    stop(&mut app);
}

#[test]
fn s018_native_root_delete_recreate_missing_retry_and_usable_up() {
    let f = Fixture::new();
    let mut app = f.app();
    wait(&mut app, ready);
    fs::remove_dir(&f.a).unwrap();
    wait(
        &mut app,
        |app| matches!(&app.panes[0].tabs[0].navigation.current.status, Status::Failed(e) if e.kind == directory::FailureKind::Missing),
    );
    assert_eq!(app.active_tab_mut().navigation.up().unwrap(), f.parent);
    assert!(
        app.watch.bindings[0]
            .status
            .as_ref()
            .unwrap()
            .directory_error
            .is_some()
    );
    assert!(
        app.watch
            .message(&app.watch_context(0))
            .contains("CreateFileW")
            || app
                .watch
                .message(&app.watch_context(0))
                .contains("Disappeared")
    );
    let context = app.watch_context(0);
    app.watch.retry(&context); // actual retry of the immutable missing target
    fs::create_dir(&f.a).unwrap();
    fs::write(f.a.join("after-recreation"), b"owned").unwrap();
    wait(&mut app, |app| {
        ready(app) && settled(app) && rows(app).contains(&f.a.join("after-recreation"))
    });
    assert!(
        app.watch.bindings[0]
            .status
            .as_ref()
            .unwrap()
            .directory_error
            .is_none()
    );
    let up = app.active_tab_mut().navigation.up().unwrap();
    app.navigate(up, Intent::New);
    wait(&mut app, |app| {
        app.panes[0].tabs[0].navigation.current.location == f.parent && settled(app)
    });
    assert!(rows(&app).contains(&f.a));
    stop(&mut app);
}

// Loader completion is controlled here because local enumeration cannot reliably
// stay pending long enough. The dirty notification itself is an actual native event.
fn pending_native_dirty(success: bool) {
    let f = Fixture::new();
    fs::write(f.a.join("keep"), b"owned").unwrap();
    let mut app = f.app();
    wait(&mut app, ready);
    app.active_tab_mut().navigation.set_filter("keep".into());
    app.active_tab_mut().filter_text = "keep".into();
    app.active_tab_mut().navigation.sort.toggle(Column::Size);
    wait(&mut app, settled);
    let nav = &mut app.active_tab_mut().navigation;
    nav.selection.select_all(&nav.current.entries);
    let selection = nav.selection.paths.clone();
    let sort = nav.sort;
    app.navigate(f.b.clone(), Intent::New);
    let pending_generation = app.active_tab_mut().navigation.key().generation;
    app.loader.cancel(1);
    fs::write(f.a.join("keep-new"), b"event during B").unwrap();
    let until = Instant::now() + Duration::from_secs(10);
    loop {
        app.sync_watch();
        app.watch.drain();
        app.reconcile_watch(); // don't drain the controlled directory result yet
        if app.watch.bindings[0].dirty {
            break;
        }
        assert!(Instant::now() < until, "native dirty A missing");
        thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(
        app.active_tab_mut().navigation.pending_location(),
        Some(f.b.as_path())
    );
    assert_eq!(
        app.active_tab_mut().navigation.key().generation,
        pending_generation
    );
    let a_watch_generation = app.watch.bindings[0].target.generation();
    app.active_tab_mut().apply(directory::Response {
        generation: pending_generation,
        update: directory::Update::Finished(if success {
            Ok(f.b.clone())
        } else {
            Err(directory::Failure {
                kind: directory::FailureKind::AccessDenied,
                detail: "controlled pending B failure".into(),
            })
        }),
    });
    app.reconcile_watch();
    if success {
        assert_eq!(app.active_tab_mut().navigation.current.location, f.b);
        assert_eq!(app.active_tab_mut().navigation.back, vec![f.a.clone()]);
        assert!(app.watch.bindings[0].target.generation() > a_watch_generation);
        assert!(!app.watch.bindings[0].dirty);
        wait(&mut app, |app| settled(app) && ready(app));
        assert!(rows(&app).is_empty());
    } else {
        assert_eq!(app.active_tab_mut().navigation.current.location, f.a);
        assert!(matches!(
            app.active_tab_mut().navigation.current.status,
            Status::Loading
        ));
        assert_eq!(
            app.watch.bindings[0].target.generation(),
            a_watch_generation
        );
        wait(&mut app, |app| {
            rows(app).contains(&f.a.join("keep-new")) && settled(app)
        });
        assert!(app.active_tab_mut().navigation.back.is_empty());
        assert_eq!(
            app.active_tab_mut().navigation.error.as_ref().unwrap().0,
            f.b
        );
        assert_eq!(app.active_tab_mut().navigation.selection.paths, selection);
    }
    assert_eq!(app.active_tab_mut().navigation.sort, sort);
    assert_eq!(app.active_tab_mut().navigation.filter, "keep");
    stop(&mut app);
}
#[test]
fn s018_native_dirty_a_does_not_cancel_pending_b_success() {
    pending_native_dirty(true);
}
#[test]
fn s018_native_dirty_a_recovers_after_controlled_pending_b_failure() {
    pending_native_dirty(false);
}

#[test]
fn s018_refresh_filter_sort_do_not_reregister_or_repeat_cancel_inflight_load() {
    let f = Fixture::new();
    let mut app = f.app();
    let revision = app.watch.revision;
    let generation = app.watch.bindings[0].target.generation();
    app.navigate(f.a.clone(), Intent::Refresh);
    let loading = app.active_tab_mut().navigation.key().generation;
    for _ in 0..20 {
        let report = status(&app, true);
        inject(&mut app, report);
        app.reconcile_watch();
    }
    assert_eq!(app.active_tab_mut().navigation.key().generation, loading);
    wait(&mut app, settled);
    app.active_tab_mut().navigation.set_filter("needle".into());
    app.active_tab_mut()
        .navigation
        .sort
        .toggle(Column::Modified);
    wait(&mut app, settled);
    for _ in 0..20 {
        app.poll_directory();
    }
    assert_eq!(app.watch.revision, revision);
    assert_eq!(app.watch.bindings[0].target.generation(), generation);
    stop(&mut app);
}

#[test]
fn s018_switch_close_stale_revision_generation_and_complete_native_path() {
    let f = Fixture::new();
    let mut app = f.app();
    let old = status(&app, true);
    app.insert_tab(f.b.clone());
    app.sync_watch();
    let second = app.active_tab_mut().id;
    inject(&mut app, old.clone());
    assert!(!app.watch.bindings[0].dirty);
    app.activate_tab(0);
    app.sync_watch();
    let mut stale_gen = status(&app, true);
    stale_gen.target = old.target.clone();
    inject(&mut app, stale_gen);
    assert!(!app.watch.bindings[0].dirty);
    let mut wrong_path = status(&app, true);
    wrong_path.target = Target::new(1, app.watch.bindings[0].target.generation(), f.b.clone());
    inject(&mut app, wrong_path);
    assert!(!app.watch.bindings[0].dirty);
    let mut alternate_spelling = status(&app, true);
    alternate_spelling.target = Target::new(
        1,
        app.watch.bindings[0].target.generation(),
        PathBuf::from(f.a.as_os_str().to_str().unwrap().replace('\\', "/")),
    );
    inject(&mut app, alternate_spelling);
    assert!(
        !app.watch.bindings[0].dirty,
        "complete native spelling must match"
    );
    app.close_tab(1); // inactive close keeps target
    app.sync_watch();
    assert_ne!(app.watch.bindings[0].target.id(), second.0);
    let closed = status(&app, true);
    app.close_tab(0);
    app.sync_watch();
    inject(&mut app, closed);
    assert!(!app.watch.bindings[0].dirty);
    wait(&mut app, settled);
    stop(&mut app);
}

#[test]
fn s018_actual_tab_move_and_pane_close_reject_old_context_keep_loader_and_other_target() {
    use tab_drag::{Edge, TabDrag, TabDrop, TabStripGeometry};
    let f = Fixture::new();
    let mut app = f.app();
    app.insert_tab(f.b.clone());
    app.activate_tab(0);
    wait(&mut app, settled);
    let old = status(&app, true);
    let source = app.panes[0].id;
    let tab_id = app.active_tab_mut().id;
    app.navigate(f.b.clone(), Intent::New);
    let loading = app.active_tab_mut().navigation.key().generation;
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1400.0, 900.0));
    app.visible_workspace = bounds;
    app.pane_rects = vec![(source, bounds)];
    app.tab_strips = vec![TabStripGeometry {
        pane: source,
        rect: bounds,
        tabs: vec![],
    }];
    assert!(app.drop_tab(
        TabDrag {
            source,
            tab: tab_id
        },
        TabDrop::Edge {
            pane: source,
            edge: Edge::Right
        }
    ));
    assert_eq!(app.panes.len(), 2);
    assert_eq!(app.active_tab_mut().id, tab_id);
    assert_eq!(app.active_tab_mut().navigation.key().generation, loading);
    assert_eq!(
        app.active_tab_mut().navigation.pending_location(),
        Some(f.b.as_path())
    );
    app.sync_watch();
    let moved = app
        .watch
        .bindings
        .iter()
        .find(|b| b.target.id() == tab_id.0)
        .unwrap();
    assert_ne!(moved.context.pane, source);
    assert!(moved.target.generation() > old.target.generation());
    let mut old_context = old;
    old_context.revision = app.watch.revision;
    inject(&mut app, old_context.clone());
    assert!(app.watch.bindings.iter().all(|b| !b.dirty));
    app.close_pane();
    app.sync_watch();
    inject(&mut app, old_context);
    assert_eq!(app.watch.bindings.len(), 1);
    assert_ne!(app.watch.bindings[0].target.id(), tab_id.0);
    wait(&mut app, settled);
    let live = f.b.join("remaining-pane");
    fs::write(&live, b"owned").unwrap();
    wait(&mut app, |app| rows(app).contains(&live) && settled(app));
    stop(&mut app);
}

#[test]
fn s018_headless_actual_retry_button_routes_native_retry_and_preserves_generation() {
    let f = Fixture::new();
    let mut app = f.app();
    wait(&mut app, ready);
    let generation = app.watch.bindings[0].target.generation();
    let mut warning = status(&app, false);
    warning.ready = false;
    warning.overflow = true;
    warning.directory_error = Some(Failure::Native {
        action: "controlled UI warning",
        code: 5,
    });
    inject(&mut app, warning);
    let ctx = egui::Context::default();
    let frame = |app: &mut Explorer, events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 900.0),
                )),
                events,
                ..Default::default()
            },
            |ui| app.workspace(ui),
        )
    };
    for _ in 0..2 {
        frame(&mut app, vec![]).drop_without_applying_deltas();
    }
    let output = frame(&mut app, vec![]);
    assert!(text(&output, "controlled UI warning"));
    assert!(text(&output, &native::display(f.a.as_os_str())));
    let pos = output
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::Shape::Text(t) if t.galley.job.text == "Retry notifications" => {
                Some(t.pos + t.galley.size() * 0.5)
            }
            _ => None,
        })
        .expect("working retry control");
    output.drop_without_applying_deltas();
    for pressed in [true, false] {
        frame(
            &mut app,
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
    // No filesystem mutation here. Only the actual Retry can replace the injected
    // warning with a real service report and schedule initial reconciliation.
    wait(&mut app, |app| {
        ready(app) && settled(app) && !app.watch.bindings[0].status.as_ref().unwrap().overflow
    });
    assert_eq!(app.watch.bindings[0].target.generation(), generation);
    stop(&mut app);
}

#[test]
fn s018_inactive_location_has_no_target_and_refreshes_on_activation() {
    let f = Fixture::new();
    let mut app = f.app();
    app.insert_tab(f.b.clone());
    wait(&mut app, settled);
    assert_eq!(app.watch.bindings.len(), 1);
    assert_eq!(app.watch.bindings[0].target.id(), app.active_tab_mut().id.0);
    fs::write(f.a.join("inactive-create"), b"owned").unwrap();
    app.activate_tab(0);
    wait(&mut app, |app| {
        rows(app).contains(&f.a.join("inactive-create")) && settled(app)
    });
    stop(&mut app);
}

#[test]
fn s018_capacity_refuses_all_contexts_and_recovers_without_arbitrary_subset() {
    let f = Fixture::new();
    let mut app = f.app();
    for id in 2..=MAX_TARGETS as u64 + 1 {
        app.panes.push(Pane {
            id: PaneId(id),
            tabs: vec![Tab::new(TabId(id), f.a.clone())],
            active: 0,
            reveal_active_tab: false,
        });
    }
    app.sync_watch();
    assert!(app.watch.bindings.is_empty());
    assert!(
        app.watch
            .message(&app.watch_context(0))
            .contains("none watched")
    );
    let revision = app.watch.revision;
    for _ in 0..20 {
        app.sync_watch();
    }
    assert_eq!(app.watch.revision, revision);
    app.panes.truncate(1);
    app.sync_watch();
    wait(&mut app, settled);
    assert_eq!(app.watch.bindings.len(), 1);
    assert!(app.watch.revision > revision);
    stop(&mut app);
}

#[test]
fn s018_injected_overflow_failure_busy_warning_survives_listing_refresh_until_retry_report() {
    let f = Fixture::new();
    let mut app = f.app();
    let mut warning = status(&app, true);
    warning.ready = false;
    warning.overflow = true;
    warning.directory_error = Some(Failure::Native {
        action: "injected rearm",
        code: 5,
    });
    warning.parent_error = Some(Failure::Capacity);
    inject(&mut app, warning.clone());
    app.watch.refusal = Some(Refusal::Busy); // component try-lock refusal is separately tested upstream
    app.reconcile_watch();
    app.watch.drain();
    assert!(
        app.watch.message(&app.watch_context(0)).contains("Busy"),
        "a successful report drain must not hide a refused control"
    );
    // The listing's successful finish does not clear a service warning.
    let tab = app.active_tab_mut();
    let generation = tab.navigation.key().generation;
    tab.apply(directory::Response {
        generation,
        update: directory::Update::Finished(Ok(f.a.clone())),
    });
    assert!(app.watch.bindings[0].status.as_ref().unwrap().overflow);
    assert!(
        app.watch
            .message(&app.watch_context(0))
            .contains("injected rearm")
    );
    let context = app.watch_context(0);
    app.watch.retry(&context);
    assert!(
        app.watch.bindings[0].status.as_ref().unwrap().overflow,
        "request is not recovery"
    );
    // Controlled retry-result status, not proof of an actual native rearm failure.
    let recovered = status(&app, false);
    inject(&mut app, recovered);
    assert!(!app.watch.bindings[0].status.as_ref().unwrap().overflow);
    stop(&mut app);
}

#[test]
fn s018_production_gate_truthful_footer_zero_service_targets_ticks_and_manual_f5() {
    let f = Fixture::new();
    let mut app = tests::test_app(f.a.clone());
    assert!(!app.watch.enabled());
    assert!(!app.watch.needs_tick());
    assert_eq!(
        app.watch.message(&app.watch_context(0)),
        watch_adapter::MANUAL_STATUS
    );
    for _ in 0..20 {
        app.sync_watch();
        app.watch.drain();
    }
    assert!(app.watch.bindings.is_empty());
    assert_eq!(app.watch.revision, 0);
    let ctx = egui::Context::default();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1400.0, 700.0),
            )),
            ..Default::default()
        },
        |ui| app.workspace(ui),
    );
    assert!(text(&output, "Notifications off; F5"));
    assert!(!text(&output, "Retry notifications"));
    output.drop_without_applying_deltas();
    let before = app.active_tab_mut().navigation.key().generation;
    ctx.run_ui(
        egui::RawInput {
            events: vec![tests::key_event(egui::Key::F5, egui::Modifiers::NONE)],
            ..Default::default()
        },
        |ui| {
            app.toolbar(ui);
        },
    )
    .drop_without_applying_deltas();
    assert!(app.active_tab_mut().navigation.key().generation > before);
    wait(&mut app, |app| {
        matches!(
            app.panes[0].tabs[0].navigation.current.status,
            Status::Ready
        ) && app.panes[0].tabs[0].navigation.projection_fresh()
    });
    assert_eq!(app.watch.revision, 0);
    assert!(!app.watch.needs_tick());
}

#[test]
fn s018_actual_app_exit_requests_nonblocking_stop_and_retains_cleanup_ownership() {
    let f = Fixture::new();
    let mut app = f.app();
    wait(&mut app, ready);
    assert!(!app.watch.cleanup_acknowledged);
    let start = Instant::now();
    eframe::App::on_exit(&mut app);
    assert!(start.elapsed() < Duration::from_millis(100));
    assert!(app.watch.bindings.is_empty());
    assert!(
        !app.watch.cleanup_acknowledged,
        "request is not cleanup acknowledgement"
    );
    // Test, not UI, waits for the actual worker cleanup/thread completion.
    stop(&mut app);
}

#[test]
fn s018_actual_initial_missing_target_error_and_retry_are_not_mock_registration() {
    let f = Fixture::new();
    let missing = f.parent.join("initially-missing");
    let mut app = tests::test_app(missing.clone());
    app.watch = watch_adapter::WatchAdapter::enable_owned(&f.root, &[app.watch_context(0)]);
    app.navigate(missing.clone(), Intent::Refresh);
    wait(&mut app, |app| {
        app.watch.bindings[0]
            .status
            .as_ref()
            .is_some_and(|s| s.directory_error.is_some())
            && settled(app)
    });
    assert!(matches!(
        app.active_tab_mut().navigation.current.status,
        Status::Failed(_)
    ));
    assert!(
        app.watch
            .message(&app.watch_context(0))
            .contains("CreateFileW")
    );
    fs::create_dir(&missing).unwrap();
    fs::write(missing.join("recovered"), b"owned").unwrap();
    let context = app.watch_context(0);
    app.watch.retry(&context);
    wait(&mut app, |app| {
        ready(app) && settled(app) && rows(app).contains(&missing.join("recovered"))
    });
    stop(&mut app);
}
