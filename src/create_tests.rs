//! S019 actual App -> fixed worker -> P18 -> drawer -> listing checks.
//! Only generated, create-new owned parent/A/B fixtures. No native window, clipboard or personal inventory.
use super::*;
use create_adapter::{BeforeCallGate, CreateAdapter};
use mado_operation_drawer::{Kind, OperationId, Outcome, State};
use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Fixture {
    root: PathBuf,
    a: PathBuf,
    b: PathBuf,
    live: std::sync::Mutex<Option<Arc<std::sync::atomic::AtomicBool>>>,
}
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "mado-s019-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let parent = root.join("owned-parent");
        fs::create_dir(&parent).unwrap();
        let a = parent.join("A-é-日本-𐐀");
        let b = parent.join("B");
        fs::create_dir(&a).unwrap();
        fs::create_dir(&b).unwrap();
        fs::write(root.join("sentinel"), b"unchanged").unwrap();
        fs::write(a.join("sentinel-a"), b"a-original").unwrap();
        fs::write(b.join("sentinel-b"), b"b-original").unwrap();
        Self {
            root,
            a,
            b,
            live: std::sync::Mutex::new(None),
        }
    }
    fn app(&self, gate: Option<Arc<BeforeCallGate>>) -> Explorer {
        let mut app = tests::test_app(self.a.clone());
        app.home = self.a.clone();
        app.create = CreateAdapter::enable_owned(self.root.clone(), gate);
        *self.live.lock().unwrap() = Some(app.create.worker_live.clone());
        app.navigate(self.a.clone(), Intent::Refresh);
        wait(&mut app, settled);
        app
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if self
            .live
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(|live| live.load(Ordering::Acquire))
        {
            eprintln!(
                "S019 fixture retained because worker cleanup is not established: {}",
                self.root.display()
            );
            return;
        }
        assert_eq!(fs::read(self.root.join("sentinel")).unwrap(), b"unchanged");
        assert_eq!(fs::read(self.a.join("sentinel-a")).unwrap(), b"a-original");
        assert_eq!(fs::read(self.b.join("sentinel-b")).unwrap(), b"b-original");
        // Established create-new owner, no tests create links or uncertain rollback.
        fs::remove_dir_all(&self.root).unwrap();
    }
}
#[test]
fn s019_fixture_parallel_names_are_unique() {
    let barrier = std::sync::Barrier::new(16);
    let fixtures = thread::scope(|scope| {
        let workers: Vec<_> = (0..16)
            .map(|_| {
                scope.spawn(|| {
                    barrier.wait();
                    Fixture::new()
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    let mut suffixes = std::collections::HashSet::new();
    for fixture in &fixtures {
        let name = fixture.root.file_name().unwrap().to_str().unwrap();
        assert_eq!(name.split('-').count(), 5);
        assert!(suffixes.insert(name.rsplit('-').next().unwrap().parse::<u64>().unwrap()));
    }
}

fn tick(app: &mut Explorer) {
    eframe::App::logic(
        app,
        &egui::Context::default(),
        &mut eframe::Frame::_new_kittest(),
    );
}
fn wait(app: &mut Explorer, predicate: impl Fn(&Explorer) -> bool) {
    let until = Instant::now() + Duration::from_secs(10);
    loop {
        tick(app);
        if predicate(app) {
            return;
        }
        assert!(
            Instant::now() < until,
            "S019 timeout: {:?}, editor {:?}",
            app.create.notice,
            app.create.editor.as_ref().map(|e| (&e.error, e.preparing))
        );
        thread::sleep(Duration::from_millis(5));
    }
}
fn settled(app: &Explorer) -> bool {
    app.panes.iter().all(|p| {
        let nav = &p.tabs[p.active].navigation;
        nav.pending_location().is_none()
            && nav.projection_fresh()
            && !matches!(nav.current.status, Status::Loading)
    }) && app
        .create
        .dirty
        .iter()
        .all(|id| !app.panes.iter().any(|p| p.tabs[p.active].id == *id))
}
fn prepared(app: &Explorer) -> bool {
    app.create.editor.as_ref().is_some_and(|e| !e.preparing)
}
fn open(app: &mut Explorer, kind: Kind, name: &str) {
    app.open_create(&egui::Context::default(), kind);
    app.create.editor.as_mut().unwrap().name = name.into();
    wait(app, prepared);
}
fn commit(app: &mut Explorer) -> OperationId {
    assert!(
        app.create.commit(),
        "{:?}",
        app.create.editor.as_ref().map(|e| &e.error)
    );
    app.create.payloads.last().unwrap().request.id()
}
fn outcome(app: &Explorer, id: OperationId) -> Option<State> {
    app.create
        .operations
        .history()
        .iter()
        .find(|r| r.request().id() == id)
        .map(|r| r.state())
}
fn finish(app: &mut Explorer, id: OperationId) {
    wait(app, |a| outcome(a, id).is_some() && settled(a));
}
fn stop(app: &mut Explorer) {
    app.create.cancel_and_stop();
    wait(app, |a| {
        a.create.cleanup_acknowledged && !a.create.needs_tick()
    });
}
fn key(key: egui::Key, pressed: bool) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}
fn frame(app: &mut Explorer, ctx: &egui::Context, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1100.0, 800.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| app.persistence_ui(ui));
        },
    )
}
fn text(output: &egui::FullOutput, needle: &str) -> bool {
    output
        .shapes
        .iter()
        .any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.job.text.contains(needle)))
}

#[test]
fn s019_actual_file_folder_collision_and_fresh_selection_without_watcher() {
    let f = Fixture::new();
    let mut app = f.app(None);
    assert!(!app.watch.enabled());
    for (kind, name) in [
        (Kind::CreateFile, "new-é-𐐀.txt"),
        (Kind::CreateFolder, "new-日本-folder"),
    ] {
        open(&mut app, kind, name);
        let id = commit(&mut app);
        finish(&mut app, id);
        assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Completed)));
        let path = f.a.join(name);
        assert!(
            app.active_tab_mut()
                .navigation
                .selection
                .paths
                .contains(&path)
        );
        if kind == Kind::CreateFile {
            assert_eq!(fs::read(&path).unwrap(), b"");
        } else {
            assert!(fs::metadata(&path).unwrap().is_dir());
        }
    }
    open(&mut app, Kind::CreateFile, "sentinel-a");
    let id = commit(&mut app);
    finish(&mut app, id);
    assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Failed)));
    let report = app.create.operations.history().back().unwrap();
    assert_eq!(report.status().counts.failed, 1);
    assert_eq!(report.issues()[0].path, f.a.join("sentinel-a"));
    assert!(report.issues()[0].error.contains("os error"));
    assert_eq!(fs::read(f.a.join("sentinel-a")).unwrap(), b"a-original");
    stop(&mut app);
}

#[test]
fn s019_r01_progressive_fresh_rows_do_not_consume_creation_selection() {
    let f = Fixture::new();
    let mut app = f.app(None);
    let path = f.a.join("progressive-created.txt");
    open(&mut app, Kind::CreateFile, "progressive-created.txt");
    let id = commit(&mut app);
    // Poll actual P18 completion and schedule its refresh, without draining directory responses.
    let until = Instant::now() + Duration::from_secs(10);
    while outcome(&app, id).is_none() {
        app.create_logic();
        assert!(Instant::now() < until);
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Completed)));
    assert_eq!(fs::read(&path).unwrap(), b"");
    let generation = app.active_tab_mut().navigation.current.generation;
    assert_eq!(
        app.create.payloads[0].selection_generation,
        Some(generation)
    );
    assert!(matches!(
        app.active_tab_mut().navigation.current.status,
        Status::Loading
    ));
    assert!(app.create.dirty.is_empty());

    // Retain actual loader rows/completion; control only their progressive delivery and projection timing.
    let mut rows = None;
    let mut finished = None;
    while finished.is_none() {
        app.loader.drain(|response| {
            assert_eq!(response.generation, generation);
            match response.update {
                directory::Update::Snapshot { entries, .. } => rows = Some(entries),
                directory::Update::Finished(result) => finished = Some(result),
            }
        });
        assert!(Instant::now() < until);
        thread::sleep(Duration::from_millis(5));
    }
    let rows = rows.unwrap();
    assert!(rows.iter().any(|e| e.path == path));
    let finished = finished.unwrap();
    assert_eq!(finished.as_ref().unwrap(), &f.a);
    let partial = Arc::new(rows.iter().filter(|e| e.path != path).cloned().collect());
    for progressive in [Arc::new(Vec::new()), partial, rows.clone()] {
        let tab = app.active_tab_mut();
        tab.apply(directory::Response {
            generation,
            update: directory::Update::Snapshot {
                entries: progressive,
                metadata_errors: 0,
            },
        });
        let nav = &mut tab.navigation;
        assert!(nav.apply_sorted(sorting::project_for_test(
            nav.key(),
            nav.current.raw_entries.clone(),
            &nav.filter
        )));
        assert!(
            nav.rows_selectable(),
            "ordinary empty-filter browsing remains selectable while Loading"
        );
        app.reconcile_create();
        assert!(
            !app.create.payloads[0].selected,
            "fresh Loading projection must not consume the creation marker, even with the created path present"
        );
        assert_eq!(
            app.create.payloads[0].selection_generation,
            Some(generation)
        );
        assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
    }
    let tab = app.active_tab_mut();
    assert!(tab.apply(directory::Response {
        generation,
        update: directory::Update::Finished(finished)
    }));
    assert!(
        !tab.navigation.projection_fresh(),
        "Ready requires its own fresh projection"
    );
    app.reconcile_create();
    assert!(!app.create.payloads[0].selected);
    let nav = &mut app.active_tab_mut().navigation;
    assert!(nav.apply_sorted(sorting::project_for_test(nav.key(), rows, &nav.filter)));
    app.reconcile_create();
    assert!(app.create.payloads[0].selected);
    assert_eq!(
        app.active_tab_mut().navigation.selection.paths,
        [path].into_iter().collect()
    );
    // A consumed marker must not select again over a later user selection.
    let tab = app.active_tab_mut();
    let index = tab
        .navigation
        .current
        .entries
        .iter()
        .position(|e| e.path == f.a.join("sentinel-a"))
        .unwrap();
    tab.navigation
        .selection
        .click(&tab.navigation.current.entries, index, false, false);
    app.reconcile_create();
    assert_eq!(
        app.active_tab_mut().navigation.selection.paths,
        [f.a.join("sentinel-a")].into_iter().collect()
    );
    stop(&mut app);
}

#[test]
fn s019_r01_normalized_new_navigation_rejects_deferred_old_generation_selection() {
    let f = Fixture::new();
    fs::create_dir(f.a.join("child")).unwrap();
    let gate = Arc::new(BeforeCallGate::default());
    let mut app = f.app(Some(gate.clone()));
    let tab = app.active_tab_mut();
    let index = tab
        .navigation
        .current
        .entries
        .iter()
        .position(|e| e.path == f.a.join("sentinel-a"))
        .unwrap();
    tab.navigation
        .selection
        .click(&tab.navigation.current.entries, index, false, false);
    let prior_selection = tab.navigation.selection.paths.clone();
    let prior_rows = tab.navigation.current.raw_entries.clone();
    open(&mut app, Kind::CreateFile, "normalized-created.txt");
    let captured = app.create.editor.as_ref().unwrap().target.clone();
    let id = commit(&mut app);
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    let alias = f.a.join("child").join("..");
    app.navigate(alias.clone(), Intent::New);
    let pending = app.active_tab_mut().navigation.key().generation;
    assert_eq!(pending, captured.generation + 1);
    app.loader.cancel(captured.tab.0); // Control only the pending alias completion, not creation.
    gate.release.store(true, Ordering::Release);
    let until = Instant::now() + Duration::from_secs(10);
    while outcome(&app, id).is_none() {
        app.create_logic();
        assert!(Instant::now() < until);
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Completed)));
    assert_eq!(fs::read(f.a.join("normalized-created.txt")).unwrap(), b"");
    assert_eq!(app.create.payloads[0].selection_generation, Some(0));
    assert_eq!(
        app.create.payloads[0].origin_generation,
        captured.generation
    );
    assert_eq!(
        app.active_tab_mut().navigation.current.generation,
        captured.generation
    );
    assert_eq!(
        app.active_tab_mut().navigation.pending_location(),
        Some(alias.as_path())
    );
    assert!(app.create.dirty.contains(&captured.tab));
    // Existing Navigation::apply same-location alias branch commits normalized A at g2.
    let tab = app.active_tab_mut();
    tab.apply(directory::Response {
        generation: pending,
        update: directory::Update::Snapshot {
            entries: prior_rows,
            metadata_errors: 0,
        },
    });
    assert!(tab.apply(directory::Response {
        generation: pending,
        update: directory::Update::Finished(Ok(f.a.clone()))
    }));
    assert_eq!(tab.navigation.current.location, f.a);
    assert_eq!(tab.navigation.current.generation, pending);
    assert_eq!(tab.navigation.selection.paths, prior_selection);
    app.reconcile_create();
    assert_eq!(
        app.active_tab_mut().navigation.current.generation,
        pending + 1
    );
    assert!(
        app.create.dirty.is_empty(),
        "ordinary affected-tab refresh still runs"
    );
    finish(&mut app, id); // Actual g3 loader/sorter rows include the actual P18-created path.
    assert!(
        app.active_tab_mut()
            .navigation
            .current
            .entries
            .iter()
            .any(|e| e.path == f.a.join("normalized-created.txt"))
    );
    assert_eq!(
        app.active_tab_mut().navigation.selection.paths,
        prior_selection,
        "normalized explicit New navigation must not authorize old-context auto-selection"
    );
    let payload = &app.create.payloads[0];
    assert!(payload.selection_generation.is_none());
    assert_eq!(payload.origin_generation, captured.generation);
    assert_eq!(payload.target.tab, captured.tab);
    assert_eq!(payload.target.path, captured.path);
    assert_eq!(
        payload.target.generation, captured.generation,
        "captured Target remains immutable at g1"
    );
    stop(&mut app);
}

#[test]
fn s019_invalid_names_and_valid_enter_once_escape_owns_submission_frame() {
    let f = Fixture::new();
    let mut app = f.app(None);
    let ctx = egui::Context::default();
    open(&mut app, Kind::CreateFile, "");
    frame(&mut app, &ctx, vec![]).drop_without_applying_deltas();
    for name in ["", "CON.txt", "a/b", "a\\b", "x.", "x ", ".", ".."] {
        app.create.editor.as_mut().unwrap().name = name.into();
        frame(&mut app, &ctx, vec![key(egui::Key::Enter, false)]).drop_without_applying_deltas();
        frame(&mut app, &ctx, vec![key(egui::Key::Enter, true)]).drop_without_applying_deltas();
        assert!(
            app.create.editor.as_ref().unwrap().error.is_some(),
            "{name}"
        );
        assert!(app.create.operations.queued().is_empty());
        assert!(app.create.operations.history().is_empty());
    }
    app.create.editor.as_mut().unwrap().name = "exactly-once.txt".into();
    frame(&mut app, &ctx, vec![key(egui::Key::Enter, false)]).drop_without_applying_deltas();
    frame(&mut app, &ctx, vec![key(egui::Key::Enter, true)]).drop_without_applying_deltas();
    assert!(app.create.editor.is_none());
    let id = app.create.payloads[0].request.id();
    frame(&mut app, &ctx, vec![key(egui::Key::Enter, true)]).drop_without_applying_deltas();
    finish(&mut app, id);
    assert_eq!(app.create.operations.history().len(), 1);
    assert_eq!(fs::read(f.a.join("exactly-once.txt")).unwrap(), b"");
    frame(&mut app, &ctx, vec![key(egui::Key::Enter, false)]).drop_without_applying_deltas();
    open(&mut app, Kind::CreateFolder, "cancelled-folder");
    frame(&mut app, &ctx, vec![]).drop_without_applying_deltas();
    let generation = app.active_tab_mut().navigation.key().generation;
    frame(
        &mut app,
        &ctx,
        vec![
            key(egui::Key::Escape, true),
            key(egui::Key::Enter, true),
            key(egui::Key::F5, true),
        ],
    )
    .drop_without_applying_deltas();
    assert!(app.create.editor.is_none());
    assert_eq!(app.active_tab_mut().navigation.key().generation, generation);
    assert!(!f.a.join("cancelled-folder").exists());
    stop(&mut app);
}

#[test]
fn s019_editor_capture_survives_navigation_and_origin_close_drawer_survives() {
    let f = Fixture::new();
    let mut app = f.app(None);
    open(&mut app, Kind::CreateFolder, "captured-A");
    let target = app.create.editor.as_ref().unwrap().target.clone();
    app.navigate(f.b.clone(), Intent::New);
    wait(&mut app, settled);
    assert_eq!(app.create.editor.as_ref().unwrap().target.path, f.a);
    assert_eq!(app.create.editor.as_ref().unwrap().target.tab, target.tab);
    let id = commit(&mut app);
    app.insert_tab(f.b.clone());
    app.close_tab(0);
    wait(&mut app, settled);
    finish(&mut app, id);
    assert!(f.a.join("captured-A").is_dir());
    assert!(!f.b.join("captured-A").exists());
    let output = frame(&mut app, &egui::Context::default(), vec![]);
    assert!(text(&output, "Operations"));
    assert!(text(&output, "Worker finished"));
    output.drop_without_applying_deltas();
    assert!(
        !app.active_tab_mut()
            .navigation
            .selection
            .paths
            .contains(&f.a.join("captured-A"))
    );
    stop(&mut app);
}

#[test]
fn s019_affected_active_inactive_tabs_filter_and_alias_reconciliation() {
    let f = Fixture::new();
    let mut app = f.app(None);
    app.insert_tab(f.a.clone());
    wait(&mut app, settled);
    let origin = app.active_tab_mut().id;
    app.active_tab_mut()
        .navigation
        .set_filter("sentinel".into());
    app.active_tab_mut().filter_text = "sentinel".into();
    wait(&mut app, settled);
    let other = app.fresh_tab(PathBuf::from(f.a.to_str().unwrap().replace('\\', "/")));
    let pane = PaneId(2);
    app.panes.push(Pane {
        id: pane,
        tabs: vec![other],
        active: 0,
        reveal_active_tab: true,
    });
    app.layout = Layout::Split {
        id: PaneId(1),
        axis: Axis::LeftRight,
        ratio: 0.5,
        first: Box::new(Layout::Pane(PaneId(1))),
        second: Box::new(Layout::Pane(pane)),
    };
    app.refresh_tab_in(1);
    wait(&mut app, settled);
    open(&mut app, Kind::CreateFile, "hidden-created.txt");
    let id = commit(&mut app);
    finish(&mut app, id);
    let tab = app.panes[0].tabs.iter().find(|t| t.id == origin).unwrap();
    assert_eq!(tab.navigation.filter, "sentinel");
    assert!(
        !tab.navigation
            .selection
            .paths
            .contains(&f.a.join("hidden-created.txt"))
    );
    assert!(
        app.create
            .payloads
            .iter()
            .find(|p| p.request.id() == id)
            .unwrap()
            .filtered
    );
    assert!(
        app.panes[1].tabs[0]
            .navigation
            .current
            .entries
            .iter()
            .any(|e| e.path.file_name().unwrap() == "hidden-created.txt")
    );
    assert!(app.create.dirty.contains(&app.panes[0].tabs[0].id));
    app.activate_tab(0);
    wait(&mut app, settled);
    assert!(
        app.active_tab_mut()
            .navigation
            .current
            .entries
            .iter()
            .any(|e| e.path == f.a.join("hidden-created.txt"))
    );
    stop(&mut app);
}

#[test]
fn s019_bounded_fifo_queued_cancel_and_active_acknowledgement() {
    let f = Fixture::new();
    let gate = Arc::new(BeforeCallGate::default());
    let mut app = f.app(Some(gate.clone()));
    // Controlled scheduling gate only holds Start delivery. All preparations use actual P18.
    app.create.hold_start = true;
    let mut ids = Vec::new();
    for i in 0..16 {
        open(&mut app, Kind::CreateFile, &format!("fifo-{i:02}"));
        ids.push(commit(&mut app));
    }
    open(&mut app, Kind::CreateFile, "refused-17");
    assert!(!app.create.commit());
    assert!(
        app.create
            .editor
            .as_ref()
            .unwrap()
            .error
            .as_ref()
            .unwrap()
            .contains("Queue full")
    );
    assert_eq!(app.create.editor.as_ref().unwrap().name, "refused-17");
    app.create.dismiss();
    app.create.hold_start = false;
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    assert_eq!(
        app.create.operations.active().unwrap().request().id(),
        ids[0]
    );
    assert_eq!(app.create.operations.queued().len(), 15);
    tick(&mut app);
    assert_eq!(
        app.create
            .operations
            .active()
            .unwrap()
            .status()
            .current_item
            .as_deref(),
        Some(f.a.join("fifo-00").as_path())
    );
    assert_eq!(
        app.create
            .operations
            .active()
            .unwrap()
            .status()
            .counts
            .completed,
        0
    );
    assert!(app.create.operations.cancel(ids[1]));
    assert!(app.create.operations.cancel(ids[0]));
    tick(&mut app);
    assert_eq!(
        app.create.operations.active().unwrap().state(),
        State::Cancelling
    );
    assert!(outcome(&app, ids[0]).is_none());
    assert!(!f.a.join("fifo-00").exists());
    assert!(!f.a.join("fifo-01").exists());
    gate.release.store(true, Ordering::Release);
    wait(&mut app, |a| {
        a.create.operations.history().len() == 16 && settled(a)
    });
    assert_eq!(
        outcome(&app, ids[0]),
        Some(State::Stopped(Outcome::Cancelled))
    );
    assert_eq!(outcome(&app, ids[1]), Some(State::CancelledBeforeStart));
    let completed: Vec<_> = app
        .create
        .operations
        .history()
        .iter()
        .filter(|r| r.state() == State::Stopped(Outcome::Completed))
        .map(|r| r.request().id())
        .collect();
    assert_eq!(completed, ids[2..]);
    for i in 2..16 {
        assert_eq!(fs::read(f.a.join(format!("fifo-{i:02}"))).unwrap(), b"");
    }
    assert!(!f.a.join("refused-17").exists());
    assert_eq!(
        app.active_tab_mut().navigation.selection.paths,
        [f.a.join("fifo-15")].into_iter().collect(),
        "last completed queued creation selects its intended fresh entry in the unchanged origin"
    );
    stop(&mut app);
}

fn pending_navigation(success: bool) {
    let f = Fixture::new();
    let gate = Arc::new(BeforeCallGate::default());
    let mut app = f.app(Some(gate.clone()));
    open(&mut app, Kind::CreateFile, "late-A.txt");
    let id = commit(&mut app);
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    app.navigate(f.b.clone(), Intent::New);
    let pending = app.active_tab_mut().navigation.key().generation;
    app.loader.cancel(1); // Keep the actual A->B intent, control only B's eventual completion.
    gate.release.store(true, Ordering::Release);
    let until = Instant::now() + Duration::from_secs(10);
    while outcome(&app, id).is_none() {
        app.create_logic();
        assert!(Instant::now() < until);
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(
        app.active_tab_mut().navigation.pending_location(),
        Some(f.b.as_path())
    );
    assert_eq!(app.active_tab_mut().navigation.key().generation, pending);
    app.active_tab_mut().apply(directory::Response {
        generation: pending,
        update: directory::Update::Finished(if success {
            Ok(f.b.clone())
        } else {
            Err(directory::Failure {
                kind: directory::FailureKind::AccessDenied,
                detail: "controlled B failure".into(),
            })
        }),
    });
    app.reconcile_create();
    wait(&mut app, settled);
    if success {
        assert_eq!(app.active_tab_mut().navigation.current.location, f.b);
        assert!(
            !app.active_tab_mut()
                .navigation
                .current
                .entries
                .iter()
                .any(|e| e.path == f.a.join("late-A.txt"))
        );
        assert!(
            !app.active_tab_mut()
                .navigation
                .selection
                .paths
                .contains(&f.a.join("late-A.txt"))
        );
    } else {
        assert_eq!(app.active_tab_mut().navigation.current.location, f.a);
        assert!(
            app.active_tab_mut()
                .navigation
                .current
                .entries
                .iter()
                .any(|e| e.path == f.a.join("late-A.txt"))
        );
        assert!(
            app.active_tab_mut()
                .navigation
                .selection
                .paths
                .contains(&f.a.join("late-A.txt"))
        );
        assert_eq!(
            app.active_tab_mut().navigation.error.as_ref().unwrap().0,
            f.b
        );
    }
    let generation = app.active_tab_mut().navigation.current.generation;
    app.active_tab_mut().apply(directory::Response {
        generation: pending - 1,
        update: directory::Update::Finished(Ok(f.a.clone())),
    });
    assert_eq!(
        app.active_tab_mut().navigation.current.generation,
        generation
    );
    stop(&mut app);
}
#[test]
fn s019_pending_b_success_defers_a_refresh_and_suppresses_stale_selection() {
    pending_navigation(true);
}
#[test]
fn s019_pending_b_failure_reconciles_a_and_selects_captured_entry() {
    pending_navigation(false);
}

#[test]
fn s019_failed_only_explicit_retry_retains_identity_changed_and_missing_target_refuse() {
    let f = Fixture::new();
    let mut app = f.app(None);
    open(&mut app, Kind::CreateFile, "sentinel-a");
    let failed = commit(&mut app);
    finish(&mut app, failed);
    app.create.retry(failed);
    wait(&mut app, prepared);
    assert_eq!(app.create.editor.as_ref().unwrap().name, "sentinel-a");
    assert_eq!(app.create.editor.as_ref().unwrap().target.path, f.a);
    app.create.editor.as_mut().unwrap().name = "resolved.txt".into();
    let succeeded = commit(&mut app);
    finish(&mut app, succeeded);
    app.create.retry(succeeded);
    assert!(app.create.editor.is_none());
    app.create.retry(failed);
    wait(&mut app, prepared);
    app.create.dismiss();
    // Drain retirement, then native exclusive rename proves worker pins really released.
    for _ in 0..20 {
        tick(&mut app);
        thread::sleep(Duration::from_millis(5));
    }
    let old = f.a.with_file_name("A-retained-original");
    fs::rename(&f.a, &old).unwrap();
    fs::create_dir(&f.a).unwrap();
    app.create.retry(failed);
    wait(&mut app, prepared);
    assert!(
        app.create
            .editor
            .as_ref()
            .unwrap()
            .error
            .as_ref()
            .unwrap()
            .contains("identity changed")
    );
    assert_eq!(app.create.editor.as_ref().unwrap().name, "sentinel-a");
    assert!(!app.create.commit());
    app.create.dismiss();
    fs::remove_dir(&f.a).unwrap();
    app.create.retry(failed);
    wait(&mut app, prepared);
    assert!(app.create.editor.as_ref().unwrap().error.is_some());
    assert_eq!(app.create.editor.as_ref().unwrap().target.path, f.a);
    app.create.dismiss();
    fs::rename(&old, &f.a).unwrap();
    stop(&mut app);
}

#[test]
fn s019_preparation_failure_keeps_draft_target_retry_cancel_no_mutation() {
    let f = Fixture::new();
    let mut app = f.app(None);
    let missing = f.a.join("missing-owned-target");
    app.active_tab_mut().navigation.current.location = missing.clone();
    open(&mut app, Kind::CreateFile, "kept-draft.txt");
    assert!(app.create.editor.as_ref().unwrap().error.is_some());
    assert!(!app.create.commit());
    assert_eq!(app.create.editor.as_ref().unwrap().name, "kept-draft.txt");
    fs::create_dir(&missing).unwrap();
    app.create.prepare();
    wait(&mut app, prepared);
    let id = commit(&mut app);
    finish(&mut app, id);
    assert_eq!(fs::read(missing.join("kept-draft.txt")).unwrap(), b"");
    stop(&mut app);
}

#[test]
fn s019_close_wait_counts_actual_success_after_cancel_and_preserves_settings_flow() {
    let f = Fixture::new();
    let gate = Arc::new(BeforeCallGate {
        after_call: true,
        ..Default::default()
    });
    let mut app = f.app(Some(gate.clone()));
    open(&mut app, Kind::CreateFile, "finished-before-cancel.txt");
    let captured = app.create.editor.as_ref().unwrap().target.clone();
    let id = commit(&mut app);
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    assert_eq!(
        fs::read(f.a.join("finished-before-cancel.txt")).unwrap(),
        b""
    );
    assert!(!app.create.cleanup_acknowledged);
    assert!(!app.create_close_requested(true));
    assert!(app.create.close_prompt);
    app.create.close_prompt = false; // Keep running choice.
    assert!(!app.create_close_requested(false));
    assert!(!app.create_close_requested(true));
    app.create.cancel_and_stop();
    assert!(!app.create_close_requested(false));
    let ctx = egui::Context::default();
    tick(&mut app);
    assert!(!app.persistence.is_closing());
    gate.release.store(true, Ordering::Release);
    wait(&mut app, |a| {
        a.create.cleanup_acknowledged && !a.create.needs_tick()
    });
    assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Completed)));
    assert_eq!(
        app.create
            .operations
            .history()
            .back()
            .unwrap()
            .status()
            .counts
            .completed,
        1
    );
    assert!(
        !app.create.waiting_close,
        "cleanup continuation is consumed exactly once"
    );
    assert!(!app.create_close_requested(false));
    app.persistence_logic(&ctx, &eframe::Frame::_new_kittest());
    assert!(app.persistence.is_closing());
    let output = ctx.run_ui(egui::RawInput::default(), |ui| app.persistence_status(ui));
    assert!(
        text(&output, "Close without saving workspace changes?"),
        "existing unavailable-settings prompt remains in control"
    );
    output.drop_without_applying_deltas();
    let payload = app
        .create
        .payloads
        .iter()
        .find(|p| p.request.id() == id)
        .unwrap();
    assert_eq!(payload.target.tab, captured.tab);
    assert_eq!(payload.target.path, captured.path);
    assert_eq!(payload.target.generation, captured.generation);
    assert_eq!(
        payload.request.destination(),
        Some(f.a.join("finished-before-cancel.txt").as_path())
    );
    let tab = app
        .panes
        .iter()
        .flat_map(|p| &p.tabs)
        .find(|t| t.id == captured.tab)
        .unwrap();
    assert_eq!(tab.navigation.current.location, captured.path);
    assert_eq!(tab.navigation.current.generation, captured.generation + 1);
    assert_eq!(payload.selection_generation, Some(captured.generation + 1));
    assert_eq!(
        fs::read(f.a.join("finished-before-cancel.txt")).unwrap(),
        b""
    );
    assert!(app.create.cleanup_acknowledged);
    assert!(!app.create.needs_tick());
    stop(&mut app);
}

fn pointer_click(app: &mut Explorer, ctx: &egui::Context, label: &str) {
    let output = frame(app, ctx, vec![]);
    let position = output
        .shapes
        .iter()
        .find_map(|s| match &s.shape {
            egui::Shape::Text(t) if t.galley.job.text == label => {
                Some(t.pos + t.galley.size() / 2.0)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("Missing UI command {label}"));
    output.drop_without_applying_deltas();
    for pressed in [true, false] {
        frame(
            app,
            ctx,
            vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        )
        .drop_without_applying_deltas();
    }
}

#[test]
fn s019_actual_new_menu_and_focused_confirm_enter_reach_worker_once() {
    let f = Fixture::new();
    let mut app = f.app(None);
    let ctx = egui::Context::default();
    for (command, kind, name) in [
        ("New folder", Kind::CreateFolder, "menu-folder"),
        ("New empty file", Kind::CreateFile, "menu-file"),
    ] {
        pointer_click(&mut app, &ctx, "New");
        pointer_click(&mut app, &ctx, command);
        assert_eq!(app.create.editor.as_ref().unwrap().kind, kind);
        assert_eq!(app.create.editor.as_ref().unwrap().target.path, f.a);
        wait(&mut app, prepared);
        app.create.editor.as_mut().unwrap().name = name.into();
        frame(&mut app, &ctx, vec![]).drop_without_applying_deltas();
        let id = egui::Id::new(egui::Id::new("create-confirm").value().wrapping_add(1));
        ctx.memory_mut(|m| m.request_focus(id));
        frame(&mut app, &ctx, vec![key(egui::Key::Enter, false)]).drop_without_applying_deltas();
        frame(&mut app, &ctx, vec![key(egui::Key::Enter, true)]).drop_without_applying_deltas();
        assert!(app.create.editor.is_none());
        let operation = app.create.payloads.last().unwrap().request.id();
        finish(&mut app, operation);
        assert_eq!(
            outcome(&app, operation),
            Some(State::Stopped(Outcome::Completed))
        );
        frame(&mut app, &ctx, vec![key(egui::Key::Enter, false)]).drop_without_applying_deltas();
    }
    assert!(f.a.join("menu-folder").is_dir());
    assert_eq!(fs::read(f.a.join("menu-file")).unwrap(), b"");
    assert_eq!(app.create.operations.history().len(), 2);
    stop(&mut app);
}

#[test]
fn s019_drawer_layout_retry_does_not_leak_enter_but_global_finder_stays_usable() {
    let f = Fixture::new();
    fs::create_dir(f.a.join("must-not-open")).unwrap();
    let mut app = f.app(None);
    let ctx = egui::Context::default();
    let tab = app.active_tab_mut();
    let index = tab
        .navigation
        .current
        .entries
        .iter()
        .position(|e| e.path == f.a.join("must-not-open"))
        .unwrap();
    tab.navigation
        .selection
        .click(&tab.navigation.current.entries, index, false, false);
    let generation = tab.navigation.current.generation;
    frame(&mut app, &ctx, vec![]).drop_without_applying_deltas();
    let mut toggle = None;
    ctx.run_ui(egui::RawInput::default(), |ui| {
        toggle = Some(app.create.operations.show(ui).toggle.id);
    })
    .drop_without_applying_deltas();
    ctx.memory_mut(|m| m.request_focus(toggle.unwrap()));
    frame(&mut app, &ctx, vec![]).drop_without_applying_deltas();
    let mut passes = 0;
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1100.0, 800.0),
            )),
            events: vec![key(egui::Key::Enter, true)],
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| app.persistence_ui(ui));
            passes += 1;
            if ctx.current_pass_index() == 0 {
                ctx.request_discard("S019 input-owner layout retry");
            }
        },
    )
    .drop_without_applying_deltas();
    assert_eq!(passes, 2);
    assert!(!app.create.operations.is_open());
    assert_eq!(
        app.active_tab_mut().navigation.current.generation,
        generation
    );
    assert!(app.active_tab_mut().navigation.pending_location().is_none());
    frame(&mut app, &ctx, vec![key(egui::Key::Enter, false)]).drop_without_applying_deltas();
    let mut event = key(egui::Key::O, true);
    if let egui::Event::Key { modifiers, .. } = &mut event {
        *modifiers = egui::Modifiers::CTRL;
    }
    frame(&mut app, &ctx, vec![event]).drop_without_applying_deltas();
    assert!(
        app.finder.is_some(),
        "drawer focus does not disable the global folder finder"
    );
    frame(&mut app, &ctx, vec![key(egui::Key::Escape, true)]).drop_without_applying_deltas();
    assert!(app.finder.is_none());
    stop(&mut app);
}

#[test]
fn s019_explicit_refresh_during_create_does_not_reauthorize_old_selection_marker() {
    let f = Fixture::new();
    let gate = Arc::new(BeforeCallGate {
        after_call: true,
        ..Default::default()
    });
    let mut app = f.app(Some(gate.clone()));
    open(&mut app, Kind::CreateFile, "refreshed-before-completion");
    let id = commit(&mut app);
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    app.navigate(f.a.clone(), Intent::Refresh);
    wait(&mut app, settled);
    gate.release.store(true, Ordering::Release);
    finish(&mut app, id);
    assert!(
        app.active_tab_mut()
            .navigation
            .current
            .entries
            .iter()
            .any(|e| e.path == f.a.join("refreshed-before-completion"))
    );
    assert!(app.active_tab_mut().navigation.selection.paths.is_empty());
    assert!(
        app.create
            .payloads
            .iter()
            .find(|p| p.request.id() == id)
            .unwrap()
            .selection_generation
            .is_none()
    );
    stop(&mut app);
}

#[test]
fn s019_failed_retry_ui_retains_original_target_after_origin_tab_closes() {
    let f = Fixture::new();
    let mut app = f.app(None);
    let ctx = egui::Context::default();
    open(&mut app, Kind::CreateFile, "sentinel-a");
    let id = commit(&mut app);
    finish(&mut app, id);
    app.insert_tab(f.b.clone());
    app.close_tab(0);
    wait(&mut app, settled);
    pointer_click(&mut app, &ctx, "Operations"); // Keep the retry command inside the bounded outer viewport.
    pointer_click(
        &mut app,
        &ctx,
        &format!("Retry failed creation #{}", id.sequence),
    );
    wait(&mut app, prepared);
    assert_eq!(app.create.editor.as_ref().unwrap().target.path, f.a);
    assert_eq!(app.create.editor.as_ref().unwrap().name, "sentinel-a");
    assert!(app.create.editor.as_ref().unwrap().expected.is_some());
    app.create.editor.as_mut().unwrap().name = "explicitly-retried.txt".into();
    frame(&mut app, &ctx, vec![]).drop_without_applying_deltas();
    frame(&mut app, &ctx, vec![key(egui::Key::Enter, true)]).drop_without_applying_deltas();
    let retry = app.create.payloads.last().unwrap().request.id();
    assert_ne!(retry, id);
    finish(&mut app, retry);
    assert_eq!(fs::read(f.a.join("explicitly-retried.txt")).unwrap(), b"");
    assert!(!f.b.join("explicitly-retried.txt").exists());
    stop(&mut app);
}

#[test]
fn s019_worker_refuses_injected_drawer_payload_mismatch_before_any_mutation() {
    let f = Fixture::new();
    let mut app = f.app(None);
    app.create.hold_start = true;
    open(&mut app, Kind::CreateFile, "captured-command.txt");
    let id = commit(&mut app);
    app.create
        .inject_mismatched_start(f.b.join("altered-command.txt"));
    finish(&mut app, id);
    assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Failed)));
    assert!(
        app.create.operations.history().back().unwrap().issues()[0]
            .error
            .contains("does not match")
    );
    assert!(!f.a.join("captured-command.txt").exists());
    assert!(!f.b.join("altered-command.txt").exists());
    stop(&mut app);
}

#[test]
fn s019_editor_owner_rejects_inherited_enter_space_and_pointer_release() {
    let ctx = egui::Context::default();
    let mut owner = create_adapter::InputOwner::default();
    let id = egui::Id::new("owned-check");
    let pos = egui::pos2(30.0, 30.0);
    let held = vec![
        key(egui::Key::Enter, true),
        key(egui::Key::Space, true),
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    ctx.run_ui(
        egui::RawInput {
            events: held,
            ..Default::default()
        },
        |_| {
            assert!(!owner.prepare(&ctx, id));
        },
    )
    .drop_without_applying_deltas();
    let release = vec![
        key(egui::Key::Enter, false),
        key(egui::Key::Space, false),
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    ctx.run_ui(
        egui::RawInput {
            events: release,
            ..Default::default()
        },
        |_| {
            assert!(
                !owner.prepare(&ctx, id),
                "inherited release is not a new action"
            );
            assert!(
                !owner.prepare(&ctx, id),
                "layout retry cannot rearm same frame"
            );
        },
    )
    .drop_without_applying_deltas();
    ctx.run_ui(egui::RawInput::default(), |_| {
        assert!(owner.prepare(&ctx, id));
    })
    .drop_without_applying_deltas();
}

#[test]
fn s019_origin_pane_close_does_not_cancel_active_creation_or_lose_drawer() {
    let f = Fixture::new();
    let gate = Arc::new(BeforeCallGate::default());
    let mut app = f.app(Some(gate.clone()));
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1400.0, 900.0));
    app.visible_workspace = bounds;
    app.pane_rects = vec![(PaneId(1), bounds)];
    assert!(app.split_pane(Axis::LeftRight));
    app.navigate(f.b.clone(), Intent::New);
    wait(&mut app, settled);
    let origin = app.panes[app.focused].id;
    open(&mut app, Kind::CreateFile, "pane-closed-file");
    let id = commit(&mut app);
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    app.close_pane();
    assert!(app.panes.iter().all(|p| p.id != origin));
    assert_eq!(app.create.operations.active().unwrap().request().id(), id);
    gate.release.store(true, Ordering::Release);
    finish(&mut app, id);
    assert_eq!(fs::read(f.b.join("pane-closed-file")).unwrap(), b"");
    assert!(!f.a.join("pane-closed-file").exists());
    let output = frame(&mut app, &egui::Context::default(), vec![]);
    assert!(text(&output, "Worker finished"));
    output.drop_without_applying_deltas();
    stop(&mut app);
}

#[test]
fn s019_history_bound_refusal_keeps_editor_and_acknowledgement_frees_metadata() {
    let f = Fixture::new();
    let mut app = f.app(None);
    app.create.hold_start = true;
    for batch in 0..2 {
        let mut ids = Vec::new();
        for i in 0..16 {
            open(
                &mut app,
                Kind::CreateFile,
                &format!("never-created-{batch}-{i}"),
            );
            ids.push(commit(&mut app));
        }
        for id in ids {
            assert!(app.create.operations.cancel(id));
        }
        tick(&mut app);
    }
    assert_eq!(app.create.operations.history().len(), 32);
    open(&mut app, Kind::CreateFile, "retained-refusal");
    assert!(!app.create.commit());
    assert!(
        app.create
            .editor
            .as_ref()
            .unwrap()
            .error
            .as_ref()
            .unwrap()
            .contains("Report capacity")
    );
    assert_eq!(app.create.editor.as_ref().unwrap().name, "retained-refusal");
    assert_eq!(app.create.payloads.len(), 32);
    let dismissed = app.create.operations.history()[0].request().id();
    app.create.operations.acknowledge(dismissed);
    tick(&mut app);
    assert_eq!(app.create.payloads.len(), 31);
    let id = commit(&mut app);
    app.create.operations.cancel(id);
    tick(&mut app);
    assert_eq!(app.create.operations.history().len(), 32);
    assert_eq!(app.create.payloads.len(), 32);
    assert_eq!(
        fs::read_dir(&f.a).unwrap().count(),
        1,
        "queued cancellations and capacity refusal create nothing"
    );
    stop(&mut app);
}

#[test]
fn s019_capture_capacity_includes_unacknowledged_retiring_pins_and_retry_keeps_draft() {
    let f = Fixture::new();
    let mut app = f.app(None);
    app.create.hold_start = true;
    let mut ids = Vec::new();
    for i in 0..16 {
        open(&mut app, Kind::CreateFile, &format!("cancelled-pin-{i}"));
        ids.push(commit(&mut app));
    }
    open(&mut app, Kind::CreateFile, "ready-admitted-after-cancel");
    assert_eq!(app.create.capture_calls.load(Ordering::Acquire), 17);
    for id in ids {
        app.create.operations.cancel(id);
    }
    let first = commit(&mut app);
    open(&mut app, Kind::CreateFile, "capacity-retry-draft");
    assert!(
        app.create
            .editor
            .as_ref()
            .unwrap()
            .error
            .as_ref()
            .unwrap()
            .contains("capture capacity")
    );
    assert_eq!(
        app.create.capture_calls.load(Ordering::Acquire),
        17,
        "capacity refuses before an eighteenth capture/provider call"
    );
    assert_eq!(
        app.create.editor.as_ref().unwrap().name,
        "capacity-retry-draft"
    );
    app.create.prepare();
    wait(&mut app, prepared);
    assert!(app.create.editor.as_ref().unwrap().error.is_none());
    let second = commit(&mut app);
    app.create.hold_start = false;
    finish(&mut app, first);
    finish(&mut app, second);
    assert_eq!(
        fs::read(f.a.join("ready-admitted-after-cancel")).unwrap(),
        b""
    );
    assert_eq!(fs::read(f.a.join("capacity-retry-draft")).unwrap(), b"");
    assert_eq!(
        app.create
            .operations
            .history()
            .iter()
            .filter(|r| r.state() == State::CancelledBeforeStart)
            .count(),
        16
    );
    stop(&mut app);
}

#[test]
fn s019_native_close_request_routes_prompt_buttons_and_waits_before_settings() {
    let f = Fixture::new();
    let gate = Arc::new(BeforeCallGate::default());
    let mut app = f.app(Some(gate.clone()));
    let ctx = egui::Context::default();
    open(&mut app, Kind::CreateFile, "cancel-before-call");
    let id = commit(&mut app);
    wait(&mut app, |_| gate.entered.load(Ordering::Acquire));
    assert_eq!(app.create.capture_calls.load(Ordering::Acquire), 1);
    app.open_create(&ctx, Kind::CreateFolder);
    app.create.editor.as_mut().unwrap().name = "pending-preparation-draft".into();
    tick(&mut app); // Queue preparation behind the controlled active call, without provider entry.
    assert!(app.create.editor.as_ref().unwrap().preparing);
    let mut input = egui::RawInput::default();
    input
        .viewports
        .get_mut(&egui::ViewportId::ROOT)
        .unwrap()
        .events
        .push(egui::ViewportEvent::Close);
    let output = ctx.run_ui(input, |ui| {
        eframe::App::logic(&mut app, &ctx, &mut eframe::Frame::_new_kittest());
        egui::CentralPanel::default().show(ui, |ui| app.persistence_ui(ui));
    });
    assert!(
        output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .contains(&egui::ViewportCommand::CancelClose)
    );
    assert!(!app.persistence.is_closing());
    assert!(app.create.close_prompt);
    output.drop_without_applying_deltas();
    pointer_click(&mut app, &ctx, "Keep running");
    assert!(!app.create.close_prompt);
    assert!(!app.persistence.is_closing());
    assert_eq!(
        app.create.editor.as_ref().unwrap().name,
        "pending-preparation-draft"
    );
    assert!(!app.create_close_requested(true));
    pointer_click(&mut app, &ctx, "Cancel and wait");
    assert!(app.create.waiting_close);
    assert!(!app.create.cleanup_acknowledged);
    assert!(app.create.operations.active().is_some());
    assert!(!app.persistence.is_closing());
    gate.release.store(true, Ordering::Release);
    wait(&mut app, |a| {
        a.create.cleanup_acknowledged && !a.create.needs_tick()
    });
    assert_eq!(outcome(&app, id), Some(State::Stopped(Outcome::Cancelled)));
    assert!(!f.a.join("cancel-before-call").exists());
    assert!(!f.a.join("pending-preparation-draft").exists());
    assert_eq!(
        app.create.capture_calls.load(Ordering::Acquire),
        1,
        "stop prevents queued preparation provider calls"
    );
    assert!(app.persistence.is_closing());
}

#[test]
fn s019_production_gate_false_and_explicit_ui_commands_explain_unavailable() {
    let f = Fixture::new();
    let mut app = tests::test_app(f.a.clone());
    app.home = f.a.clone();
    assert!(app.create.production_disabled());
    assert!(!watch_adapter::NATIVE_CAPABILITY_GATE_CLEARED);
    let ctx = egui::Context::default();
    for command in ["New folder", "New empty file"] {
        pointer_click(&mut app, &ctx, "New");
        pointer_click(&mut app, &ctx, command);
        assert!(app.create.editor.is_none());
        assert!(app.create.production_disabled());
    }
    assert!(app.create.editor.is_none());
    assert!(app.create.operations.queued().is_empty());
    let output = frame(&mut app, &egui::Context::default(), vec![]);
    assert!(text(&output, "Creation unavailable"));
    output.drop_without_applying_deltas();
    assert!(app.create.production_disabled());
}
