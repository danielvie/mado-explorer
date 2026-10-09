use super::*;
use crate::tests::test_app;
use std::{
    fs, io,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mado-s011-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        fs::create_dir(path.join("a")).unwrap();
        fs::create_dir(path.join("b")).unwrap();
        Self(path)
    }
    fn settings(&self) -> PathBuf {
        self.0.join("workspace.json")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn snapshot(f: &Fixture) -> Snapshot {
    test_app(f.0.clone()).snapshot()
}
fn nested(f: &Fixture) -> Explorer {
    let mut app = test_app(f.0.clone());
    app.panes[0].tabs.push(Tab::new(TabId(2), f.0.join("a")));
    app.panes[0].active = 1;
    app.panes.push(Pane {
        id: PaneId(2),
        tabs: vec![
            Tab::new(TabId(3), f.0.join("b")),
            Tab::new(TabId(4), f.0.join("missing")),
        ],
        active: 1,
        reveal_active_tab: true,
    });
    app.panes.push(Pane {
        id: PaneId(3),
        tabs: vec![Tab::new(TabId(5), f.0.clone())],
        active: 0,
        reveal_active_tab: true,
    });
    app.focused = 1;
    app.layout = Layout::Split {
        id: PaneId(2),
        axis: Axis::LeftRight,
        ratio: 0.63,
        first: Box::new(Layout::Pane(PaneId(1))),
        second: Box::new(Layout::Split {
            id: PaneId(3),
            axis: Axis::TopBottom,
            ratio: 0.42,
            first: Box::new(Layout::Pane(PaneId(2))),
            second: Box::new(Layout::Pane(PaneId(3))),
        }),
    };
    app.locations.toggle_bookmark(f.0.join("a")).unwrap();
    app.locations.toggle_bookmark(f.0.join("missing")).unwrap();
    app.locations.visit(f.0.join("b"));
    app.locations.visit(f.0.join("a"));
    app.persistence.geometry = Geometry {
        position: Some([-100, 120]),
        size: [1200.0, 900.0],
        maximized: true,
    };
    app
}
#[test]
fn s011_settings_override_is_native_absolute_and_rejects_device_names_without_fallback() {
    let f = Fixture::new();
    assert_eq!(
        settings::override_path(&f.settings()).unwrap(),
        f.settings()
    );
    for path in [
        PathBuf::from("workspace.json"),
        f.0.clone(),
        f.0.join("NUL.json"),
    ] {
        assert!(settings::override_path(&path).is_err());
    }
    #[cfg(windows)]
    for path in [
        r"C:\NUL\workspace.json",
        r"C:\COM¹.txt\workspace.json",
        r"C:\con .txt\workspace.json",
        r"C:\folder.\workspace.json",
        r"\\.\pipe\workspace.json",
        r"C:\folder\workspace.json:stream",
    ] {
        assert!(settings::override_path(Path::new(path)).is_err(), "{path}");
    }
    assert!(!f.settings().exists());
}

#[cfg(windows)]
#[test]
fn s011_settings_override_unc_namespace_guard_is_lexical_and_lossless() {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};

    for path in [
        r"\\localhost\pipe\workspace.json",
        r"\\remote\PiPe\workspace.json",
        r"\\?\UNC\localhost\pipe\workspace.json",
        r"\\?\UNC\remote\PIPE\workspace.json",
        r"//localhost/pIpE/workspace.json",
        r"\\localhost/pipe\workspace.json",
        r"\\localhost\pipe.\workspace.json",
        r"\\localhost\pipe \workspace.json",
        r"\\?\UNC\localhost\pipe. \workspace.json",
        r"\\. \pipe\workspace.json",
        r"\\?\UNC\.\pipe\workspace.json",
        r"\\?\UNC\..\share\workspace.json",
        r"\\?\UNC\server\..\workspace.json",
        r"\\?\UNC\server/alias\share\workspace.json",
        r"\\?\UNC\server\share/alias\workspace.json",
        r"\\?\UNC\server:\share\workspace.json",
        r"\\server\share:stream\workspace.json",
        r"\\server.\share\workspace.json",
        r"\\server\share \workspace.json",
        "\\\\server\t\\share\\workspace.json",
    ] {
        assert!(
            settings::override_path(Path::new(path)).is_err(),
            "{path:?}"
        );
    }
    for path in [
        r"\\server\share\workspace.json",
        r"\\?\UNC\server\share\workspace.json",
        r"//server/share/folder/workspace.json",
        r"\\server\pipeline\workspace.json",
        r"\\?\UNC\server\pipe-data\workspace.json",
        r"\\pipe\share\workspace.json",
        r"\\con\NUL\workspace.json",
        r"\\?\UNC\con\COM1\workspace.json",
        r"\\日本語\é-𐐷\workspace.json",
        r"C:\folder\workspace.json",
        r"\\?\C:\folder\workspace.json",
    ] {
        assert_eq!(
            settings::override_path(Path::new(path))
                .unwrap()
                .as_os_str(),
            Path::new(path).as_os_str(),
            "{path:?}"
        );
    }
    for prefix in [r"\\", r"\\?\UNC\"] {
        let mut units: Vec<u16> = prefix.encode_utf16().collect();
        units.extend([0xD800, 92, 0xDC00]);
        units.extend("\\日本語\\workspace.json".encode_utf16());
        let path = PathBuf::from(OsString::from_wide(&units));
        assert_eq!(
            settings::override_path(&path).unwrap().as_os_str(),
            path.as_os_str()
        );
    }
}

#[test]
fn s011_native_path_roundtrip_including_malformed_and_long_prefix() {
    let f = Fixture::new();
    let mut paths = vec![f.0.join("日本語-é-\u{10437}")];
    #[cfg(windows)]
    {
        use std::{
            ffi::OsString,
            os::windows::ffi::{OsStrExt, OsStringExt},
        };
        let mut units: Vec<u16> = f.0.as_os_str().encode_wide().collect();
        units.extend([92, 0xD800, 0x0061, 0xDC00]);
        paths.push(PathBuf::from(OsString::from_wide(&units)));
        paths.push(PathBuf::from(format!(
            "\\\\?\\{}\\{}",
            f.0.display(),
            "x".repeat(300)
        )));
        paths.push(PathBuf::from("\\\\server\\share\\folder"));
        paths.push(PathBuf::from("\\\\?\\UNC\\server\\share\\folder"));
    }
    #[cfg(not(windows))]
    {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        let mut bytes = f.0.as_os_str().as_encoded_bytes().to_vec();
        bytes.extend([b'/', 255]);
        paths.push(PathBuf::from(OsString::from_vec(bytes)));
    }
    for path in paths {
        let encoded = serde_json::to_vec(&NativePath::capture(&path)).unwrap();
        let restored: NativePath = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored.restore().unwrap().as_os_str(), path.as_os_str());
    }
    assert!(NativePath::Windows(vec![0]).restore().is_err());
}
#[test]
fn s011_nested_restore_missing_location_up_and_explicit_folder_override() {
    let f = Fixture::new();
    let mut source = nested(&f);
    let tab = &mut source.panes[0].tabs[1];
    tab.navigation
        .begin(f.0.join("not-committed"), 71, Intent::New);
    tab.navigation.back.push(f.0.clone());
    tab.navigation.set_filter("draft-filter".into());
    tab.location_text = "editor draft".into();
    tab.row_scroll_offset = 100.0;
    tab.navigation.selection.paths.insert(f.0.join("a/file"));
    let saved = source.snapshot();
    let json = serde_json::to_string(&saved).unwrap();
    for transient in [
        "not-committed",
        "editor draft",
        "draft-filter",
        "selection",
        "operations",
        "clipboard",
        "history",
    ] {
        assert!(!json.contains(transient));
    }
    let mut restored = test_app(f.0.join("b"));
    restored
        .restore_snapshot(serde_json::from_str(&json).unwrap(), None)
        .unwrap();
    assert_eq!(restored.snapshot(), saved);
    assert_eq!(
        restored.panes[0].tabs[1].navigation.current.location,
        f.0.join("a")
    );
    assert!(
        restored.panes[0].tabs[1]
            .navigation
            .selection
            .paths
            .is_empty()
    );
    assert!(restored.panes[0].tabs[1].navigation.back.is_empty());
    assert_eq!(restored.panes[0].tabs[1].row_scroll_offset, 0.0);
    for p in 0..restored.panes.len() {
        restored.refresh_tab_in(p);
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    while matches!(
        restored.panes[1].tabs[1].navigation.current.status,
        Status::Loading
    ) {
        restored.poll_directory();
        assert!(Instant::now() < deadline, "missing path load timed out");
        std::thread::sleep(Duration::from_millis(2));
    }
    let nav = &restored.panes[1].tabs[1].navigation;
    assert!(
        matches!(&nav.current.status, Status::Failed(e) if e.kind == directory::FailureKind::Missing)
    );
    assert_eq!(nav.up(), Some(f.0.clone()));
    let mut overridden = test_app(f.0.clone());
    overridden
        .restore_snapshot(saved.clone(), Some(f.0.join("b")))
        .unwrap();
    let mut expected = saved;
    expected.panes[expected.focused].tabs[1] = NativePath::capture(&f.0.join("b"));
    assert_eq!(overridden.snapshot(), expected);
    assert!(overridden.launcher.busy_path().is_none());
    assert!(overridden.finder.is_none());
}
#[test]
fn s011_invalid_schema_tree_dimensions_indices_counts_and_encodings_are_rejected() {
    let f = Fixture::new();
    let valid = nested(&f).snapshot();
    let invalid = |mutate: fn(&mut Snapshot)| {
        let mut s = valid.clone();
        mutate(&mut s);
        assert!(s.validate().is_err());
    };
    invalid(|s| s.version = 500);
    invalid(|s| s.geometry.size[0] = f32::NAN);
    invalid(|s| s.geometry.size[1] = 0.0);
    invalid(|s| s.geometry.size[0] = 20000.0);
    invalid(|s| s.geometry.position = Some([i32::MIN, 0]));
    invalid(|s| s.focused = 3);
    invalid(|s| s.panes[0].active = 2);
    invalid(|s| s.panes[0].tabs.clear());
    invalid(|s| s.panes = vec![s.panes[0].clone(); settings::MAX_PANES + 1]);
    invalid(|s| s.panes[0].tabs = vec![s.panes[0].tabs[0].clone(); settings::MAX_TABS + 1]);
    invalid(|s| s.bookmarks = vec![s.bookmarks[0].clone(); finder::SESSION_LIMIT + 1]);
    invalid(|s| s.recents.push(s.recents[0].clone()));
    invalid(|s| s.layout = SavedLayout::Pane { pane: 0 });
    invalid(|s| s.layout = SavedLayout::Pane { pane: 800 });
    invalid(|s| {
        if let SavedLayout::Split { ratio, .. } = &mut s.layout {
            *ratio = 1.0;
        }
    });
    invalid(|s| {
        if let SavedLayout::Split { first, second, .. } = &mut s.layout {
            *second = first.clone();
        }
    });
    invalid(|s| {
        for _ in 0..18 {
            s.layout = SavedLayout::Split {
                vertical: false,
                ratio: 0.5,
                first: Box::new(s.layout.clone()),
                second: Box::new(SavedLayout::Pane { pane: 0 }),
            };
        }
    });
    let mut schema = serde_json::to_value(&valid).unwrap();
    schema["operation"] = serde_json::json!("copy");
    assert!(serde_json::from_value::<Snapshot>(schema).is_err());
    let path = NativePath::capture(&f.0);
    let mut path_schema = serde_json::to_value(path).unwrap();
    path_schema["draft"] = serde_json::json!(true);
    assert!(serde_json::from_value::<NativePath>(path_schema).is_err());
    #[cfg(windows)]
    assert!(
        NativePath::Windows(vec![65; finder::PATH_LIMIT + 1])
            .restore()
            .is_err()
    );
    assert!(
        NativePath::capture(PathBuf::from("relative").as_path())
            .restore()
            .is_err()
    );
}
#[test]
fn s011_corrupt_oversized_version_invalid_and_unreadable_settings_preserved_and_disable_adapter() {
    let f = Fixture::new();
    let mut version = snapshot(&f);
    version.version += 1;
    let mut bad_index = snapshot(&f);
    bad_index.focused = 1;
    for bytes in [
        b"not json".to_vec(),
        serde_json::to_vec(&version).unwrap(),
        serde_json::to_vec(&bad_index).unwrap(),
        vec![b'x'; settings::MAX_BYTES + 1],
    ] {
        fs::write(f.settings(), &bytes).unwrap();
        let (mut adapter, restore) = Persistence::at_path(f.settings(), true);
        assert!(restore.is_none());
        assert!(adapter.warning.is_some());
        assert!(adapter.disabled.is_some());
        adapter.start_worker(Arc::new(|| {}));
        assert!(adapter.worker.is_none());
        assert_eq!(fs::read(f.settings()).unwrap(), bytes);
    }
    fs::remove_file(f.settings()).unwrap();
    fs::create_dir(f.settings()).unwrap();
    let loaded = settings::load(&f.settings());
    assert!(!loaded.writable);
    assert!(loaded.warning.is_some());
    assert!(f.settings().is_dir());
}
#[test]
fn s011_atomic_success_replaces_old_bytes_and_all_precommit_failures_preserve_old_bytes() {
    let f = Fixture::new();
    let saved = snapshot(&f);
    for stage in [
        settings::Stage::Write,
        settings::Stage::Flush,
        settings::Stage::Sync,
        settings::Stage::Close,
        settings::Stage::Commit,
    ] {
        fs::write(f.settings(), b"previous bytes").unwrap();
        let result = settings::save_with(&f.settings(), &saved, |point| {
            if point == stage {
                Err(io::Error::other("injected failure"))
            } else {
                Ok(())
            }
        });
        let error = result.unwrap_err();
        assert!(error.leftover.is_none());
        assert_eq!(fs::read(f.settings()).unwrap(), b"previous bytes");
        assert_eq!(fs::read_dir(&f.0).unwrap().count(), 3);
    }
    settings::save(&f.settings(), &saved).unwrap();
    assert_eq!(settings::load(&f.settings()).snapshot.unwrap(), saved);
    let replacement = nested(&f).snapshot();
    settings::save(&f.settings(), &replacement).unwrap();
    assert_eq!(settings::load(&f.settings()).snapshot.unwrap(), replacement);
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 3);
}
#[test]
fn s011_native_commit_failure_and_cleanup_failure_report_owned_temp_without_deleting_others() {
    let f = Fixture::new();
    let unrelated = f.0.join(".mado-settings-unrelated.tmp");
    fs::write(&unrelated, b"unrelated").unwrap();
    fs::create_dir(f.settings()).unwrap();
    fs::write(f.settings().join("sentinel"), b"original").unwrap();
    let error = settings::save(&f.settings(), &snapshot(&f)).unwrap_err();
    assert!(error.leftover.is_none());
    assert_eq!(
        fs::read(f.settings().join("sentinel")).unwrap(),
        b"original"
    );
    fs::remove_dir_all(f.settings()).unwrap();
    fs::write(f.settings(), b"previous").unwrap();
    let error = settings::save_with(&f.settings(), &snapshot(&f), |stage| {
        if matches!(stage, settings::Stage::Commit | settings::Stage::Cleanup) {
            Err(io::Error::other("injected"))
        } else {
            Ok(())
        }
    })
    .unwrap_err();
    assert!(error.detail.contains("cleanup failed"));
    assert!(error.leftover.unwrap().exists());
    assert_eq!(fs::read(f.settings()).unwrap(), b"previous");
    assert_eq!(fs::read(unrelated).unwrap(), b"unrelated");
}
#[test]
fn s011_bounded_serialization_preserves_old_file_without_temp_creation() {
    let f = Fixture::new();
    fs::write(f.settings(), b"previous").unwrap();
    let mut s = snapshot(&f);
    let long = f.0.join("x".repeat(18000));
    s.panes[0].tabs = vec![NativePath::capture(&long); settings::MAX_TABS];
    assert!(s.validate().is_ok());
    assert!(
        settings::save(&f.settings(), &s)
            .unwrap_err()
            .detail
            .contains("2 MiB")
    );
    assert_eq!(fs::read(f.settings()).unwrap(), b"previous");
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 3);
}
#[test]
fn s011_coalescing_worker_shutdown_saves_latest_and_join_reports_failure() {
    let f = Fixture::new();
    let mut worker = settings::Worker::new(f.settings(), Arc::new(|| {})).unwrap();
    let mut latest = snapshot(&f);
    for n in 0..200 {
        latest.geometry.position = Some([n, 20]);
        worker.request(latest.clone());
    }
    worker.finish().unwrap();
    assert_eq!(settings::load(&f.settings()).snapshot.unwrap(), latest);
    fs::remove_file(f.settings()).unwrap();
    fs::create_dir(f.settings()).unwrap();
    let mut failed = settings::Worker::new(f.settings(), Arc::new(|| {})).unwrap();
    failed.request(latest);
    assert!(failed.finish().is_err());
    assert!(f.settings().is_dir());
}
fn logic(app: &mut Explorer, ctx: &egui::Context, close: bool) -> Vec<egui::ViewportCommand> {
    let mut input = egui::RawInput::default();
    if close {
        input
            .viewports
            .get_mut(&egui::ViewportId::ROOT)
            .unwrap()
            .events
            .push(egui::ViewportEvent::Close);
    }
    let output = ctx.run_ui(input, |ui| {
        app.persistence_logic(ctx, &eframe::Frame::_new_kittest());
        app.persistence_status(ui);
    });
    let commands = output.viewport_output[&egui::ViewportId::ROOT]
        .commands
        .clone();
    output.drop_without_applying_deltas();
    commands
}
#[test]
fn s011_close_waits_for_latest_snapshot_or_explicit_failure_gate_prompt() {
    let f = Fixture::new();
    assert!(PRODUCTION_SETTINGS_WRITES_ENABLED);
    let mut app = nested(&f);
    let (p, _) = Persistence::at_path(f.settings(), false);
    app.persistence = p;
    app.persistence.start_worker(Arc::new(|| {}));
    let ctx = egui::Context::default();
    assert!(logic(&mut app, &ctx, true).contains(&egui::ViewportCommand::CancelClose));
    assert!(app.persistence.close_prompt);
    assert!(!f.settings().exists());
    let (p, _) = Persistence::at_path(f.settings(), PRODUCTION_SETTINGS_WRITES_ENABLED);
    app.persistence = p;
    app.persistence.start_worker(Arc::new(|| {}));
    let expected = app.snapshot();
    app.open_finder(&ctx);
    let commands = logic(&mut app, &ctx, true);
    assert!(commands.contains(&egui::ViewportCommand::CancelClose));
    assert!(app.finder.is_none());
    ctx.run_ui(
        egui::RawInput {
            events: vec![
                crate::tests::key_event(egui::Key::T, egui::Modifiers::CTRL),
                crate::tests::key_event(egui::Key::O, egui::Modifiers::CTRL),
                crate::tests::key_event(egui::Key::F6, egui::Modifiers::NONE),
            ],
            ..Default::default()
        },
        |ui| app.persistence_ui(ui),
    )
    .drop_without_applying_deltas();
    assert_eq!(app.snapshot(), expected);
    assert!(app.finder.is_none());
    let deadline = Instant::now() + Duration::from_secs(5);
    while !app.persistence.allow_close {
        logic(&mut app, &ctx, false);
        assert!(Instant::now() < deadline, "close save timed out");
        std::thread::sleep(Duration::from_millis(2));
    }
    app.persistence_exit();
    assert_eq!(settings::load(&f.settings()).snapshot.unwrap(), expected);
    fs::remove_file(f.settings()).unwrap();
    fs::create_dir(f.settings()).unwrap();
    // Known failure after startup: the destination became a directory, so native replacement fails.
    let mut failed = test_app(f.0.clone());
    failed.persistence = Persistence {
        path: Some(f.settings()),
        ..Default::default()
    };
    failed.persistence.start_worker(Arc::new(|| {}));
    logic(&mut failed, &ctx, true);
    while failed.persistence.error.is_none() {
        logic(&mut failed, &ctx, false);
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(!failed.persistence.allow_close);
    assert!(failed.persistence.close_prompt);
    failed.persistence.allow_close = true; // Represents explicit Close without saving; no MessageBox in tests.
    failed.persistence_exit();
    assert!(f.settings().is_dir());
}
#[test]
fn s011_geometry_offscreen_recovery_and_normal_bounds_survive_maximize_minimize() {
    let monitors = [[-1920, 0, 0, 1080], [0, 0, 1920, 1080]];
    assert_eq!(
        settings::recover_position([-1800, 100], [1000, 700], &monitors),
        Some([-1800, 100])
    );
    assert_eq!(
        settings::recover_position([9000, 9000], [1000, 700], &monitors),
        Some([-1000, 380])
    );
    assert_eq!(
        settings::recover_position([1800, 10], [1000, 700], &monitors),
        Some([920, 10])
    );
    assert_eq!(settings::recover_position([0, 0], [1000, 700], &[]), None);
    let f = Fixture::new();
    let mut app = test_app(f.0.clone());
    app.persistence.geometry = Geometry {
        position: Some([10, 20]),
        size: [1000.0, 700.0],
        maximized: true,
    };
    // Headless frame provides no native bounds; it cannot overwrite the last normal geometry.
    logic(&mut app, &egui::Context::default(), false);
    assert_eq!(app.persistence.geometry.position, Some([10, 20]));
    assert_eq!(app.persistence.geometry.size, [1000.0, 700.0]);
}

#[test]
fn s011_worker_error_cannot_be_hidden_by_coalescing_or_consuming_report() {
    let f = Fixture::new();
    fs::create_dir(f.settings()).unwrap();
    let mut worker = settings::Worker::new(f.settings(), Arc::new(|| {})).unwrap();
    worker.request(snapshot(&f));
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(report) = worker.take() {
            assert!(report.result.is_err());
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    fs::remove_dir(f.settings()).unwrap();
    for _ in 0..20 {
        worker.request(snapshot(&f));
    }
    // Queuing newer snapshots cannot auto-retry or erase the consumed error on close.
    assert!(worker.finish().is_err());
    assert!(!f.settings().exists());

    fs::create_dir(f.settings()).unwrap();
    let mut worker = settings::Worker::new(f.settings(), Arc::new(|| {})).unwrap();
    worker.request(snapshot(&f));
    loop {
        if let Some(report) = worker.take() {
            assert!(report.result.is_err());
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    fs::remove_dir(f.settings()).unwrap();
    let latest = nested(&f).snapshot();
    worker.retry(latest.clone());
    worker.finish().unwrap();
    assert_eq!(settings::load(&f.settings()).snapshot.unwrap(), latest);
}

#[test]
fn s011_gate_corruption_and_failed_close_controls_are_visible_at_minimum_window_size() {
    let f = Fixture::new();
    let mut app = test_app(f.0.clone());
    fs::write(f.settings(), b"corrupt").unwrap();
    app.persistence = Persistence::at_path(f.settings(), false).0;
    app.persistence.close_prompt = true;
    let ctx = egui::Context::default();
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(480.0, 280.0));
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(screen),
            ..Default::default()
        },
        |ui| app.persistence_status(ui),
    );
    for required in [
        "Settings restore failed.",
        "Settings writes disabled:",
        "Keep browsing",
        "Close without saving",
    ] {
        assert!(output.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.job.text.contains(required) && screen.contains_rect(egui::Rect::from_min_size(t.pos, t.galley.size())))), "Missing/clipped {required}");
    }
    output.drop_without_applying_deltas();
    app.persistence = Persistence {
        error: Some(settings::SaveError {
            detail: "fixture failure".into(),
            leftover: Some(f.0.join("owned.tmp")),
        }),
        close_prompt: true,
        ..Default::default()
    };
    app.persistence.worker = Some(settings::Worker::new(f.settings(), Arc::new(|| {})).unwrap());
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(screen),
            ..Default::default()
        },
        |ui| app.persistence_status(ui),
    );
    for required in [
        "Settings save failed.",
        "Settings temp cleanup failed.",
        "Retry settings save",
        "Retry save and close",
        "Close without saving",
    ] {
        assert!(output.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.job.text.contains(required) && screen.contains_rect(egui::Rect::from_min_size(t.pos, t.galley.size())))), "Missing/clipped {required}");
    }
    output.drop_without_applying_deltas();
}
