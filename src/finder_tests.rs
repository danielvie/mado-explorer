use super::*;
use crate::tests::{filter_app, key_event, test_app};
use finder::{Action, Location, Output, Response, Work};
use std::{
    fs,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mado-s010-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        fs::create_dir(path.join("mado-explorer")).unwrap();
        fs::create_dir(path.join("日本語-é-\u{10437}")).unwrap();
        fs::write(path.join("file.txt"), b"fixture").unwrap();
        Self(path)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn settle(app: &mut Explorer) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while app.finder.as_ref().is_some_and(|modal| modal.pending) {
        app.poll_finder();
        assert!(Instant::now() < deadline, "finder worker timed out");
        std::thread::sleep(Duration::from_millis(2));
    }
}
fn render(ctx: &egui::Context, app: &mut Explorer, events: Vec<egui::Event>) {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1100.0, 850.0),
            )),
            events,
            ..Default::default()
        },
        |ui| app.finder_workspace(ui),
    )
    .drop_without_applying_deltas();
}
fn geometry(app: &mut Explorer, size: egui::Vec2) {
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
    app.pane_rects.clear();
    app.layout
        .rectangles(rect, &mut app.pane_rects, &mut Vec::new());
    app.visible_workspace = rect;
}
fn typed(app: &mut Explorer, query: &str) {
    let modal = app.finder.as_mut().unwrap();
    modal.query = query.into();
    modal.path_mode = true;
    modal.pending = false;
}
#[test]
fn s010_fuzzy_terms_ranking_marks_no_match_and_empty_categories() {
    let items = vec![
        Location::new(
            PathBuf::from("root/many-distant-other-experiment"),
            "Recent",
        ),
        Location::new(PathBuf::from("root/mado-explorer"), "Bookmark"),
        Location::new(PathBuf::from("/"), "Storage root"),
        Location {
            path: PathBuf::from("home/documents"),
            name: "Documents".into(),
            category: "Common place",
        },
    ];
    let empty = finder::rank("  ", items.clone(), &|| false);
    assert_eq!(
        empty
            .iter()
            .map(|item| item.location.category)
            .collect::<Vec<_>>(),
        ["Recent", "Bookmark", "Storage root", "Common place"]
    );
    let ranked = finder::rank("MADO exp", items.clone(), &|| false);
    assert_eq!(ranked[0].location.path, PathBuf::from("root/mado-explorer"));
    assert!(!ranked[0].name_marks.is_empty());
    assert_eq!(
        finder::rank("root", items.clone(), &|| false)[0].path_marks,
        vec![0, 1, 2, 3]
    );
    assert!(finder::rank("zzzz", items.clone(), &|| false).is_empty());
    assert!(finder::rank("", items, &|| true).is_empty());
    let a = Location::new(PathBuf::from("b/same"), "Recent");
    let b = Location::new(PathBuf::from("a/same"), "Bookmark");
    assert_eq!(
        finder::rank("same", vec![a, b], &|| false)[0].location.path,
        PathBuf::from("a/same")
    );
}
#[test]
fn s010_open_dismiss_freezes_committed_native_base_during_pending_navigation() {
    let fixture = Fixture::new();
    let mut app = test_app(fixture.0.clone());
    let ctx = egui::Context::default();
    app.navigate(fixture.0.join("mado-explorer"), Intent::New);
    let history = app.active_tab_mut().navigation.back.clone();
    app.open_finder(&ctx);
    settle(&mut app);
    typed(&mut app, "mado-explorer");
    let modal = app.finder.as_ref().unwrap();
    assert_eq!(modal.target.base, fixture.0);
    assert_eq!(modal.typed_path().unwrap(), fixture.0.join("mado-explorer"));
    typed(&mut app, fixture.0.to_str().unwrap());
    assert_eq!(
        app.finder.as_ref().unwrap().typed_path().unwrap(),
        fixture.0
    );
    #[cfg(windows)]
    {
        typed(&mut app, "C:ambiguous");
        assert!(app.finder.as_ref().unwrap().typed_path().is_err());
        typed(&mut app, r"\\.\device");
        assert!(app.finder.as_ref().unwrap().typed_path().is_err());
        typed(&mut app, r"\\?\UNC\host\share\folder");
        assert_eq!(
            app.finder
                .as_ref()
                .unwrap()
                .typed_path()
                .unwrap()
                .as_os_str(),
            std::ffi::OsStr::new(r"\\?\UNC\host\share\folder")
        );
    }
    let generation = app.active_tab_mut().navigation.key().generation;
    app.active_tab_mut().apply(directory::Response {
        generation,
        update: directory::Update::Finished(Ok(fixture.0.join("mado-explorer"))),
    });
    assert_eq!(app.finder.as_ref().unwrap().target.base, fixture.0);
    assert!(app.finder_target().is_err());
    assert!(
        app.commit_finder(fixture.0.clone(), Action::Navigate)
            .is_err()
    );
    app.dismiss_finder(&ctx);
    assert!(app.finder.is_none());
    assert_eq!(
        app.active_tab_mut().navigation.back.len(),
        history.len() + 1
    ); // only the original pending browse committed
}
#[test]
fn s010_worker_expansion_never_navigates_and_failed_path_recovers() {
    let fixture = Fixture::new();
    let mut app = test_app(fixture.0.clone());
    let ctx = egui::Context::default();
    app.open_finder(&ctx);
    settle(&mut app);
    let key = app.active_tab_mut().navigation.key();
    for query in ["missing", "file.txt"] {
        typed(&mut app, query);
        app.request_finder_action(Some(Action::Navigate));
        settle(&mut app);
        assert!(
            app.finder
                .as_ref()
                .unwrap()
                .error
                .as_ref()
                .unwrap()
                .contains("Cannot open folder")
        );
        assert_eq!(app.active_tab_mut().navigation.key(), key);
        assert!(app.active_tab_mut().navigation.back.is_empty());
    }
    typed(&mut app, ".");
    app.request_finder_action(None);
    settle(&mut app);
    let modal = app.finder.as_ref().unwrap();
    assert!(modal.error.is_none());
    assert!(!modal.path_mode);
    assert_eq!(modal.results.len(), 2);
    assert!(
        modal
            .results
            .iter()
            .any(|item| item.location.path == fixture.0.join("日本語-é-\u{10437}"))
    );
    assert_eq!(app.active_tab_mut().navigation.current.location, fixture.0);
    assert_eq!(app.active_tab_mut().navigation.key(), key);
    assert!(app.active_tab_mut().navigation.back.is_empty());
    app.request_finder_action(Some(Action::Navigate));
    settle(&mut app);
    assert!(app.finder.is_none());
    assert!(app.active_tab_mut().navigation.pending_location().is_some());
    assert!(app.active_tab_mut().navigation.back.is_empty()); // enumeration has not committed yet
}
#[test]
fn s010_query_catalogue_probe_modal_revisions_reject_late_responses() {
    let mut app = test_app(PathBuf::from("root"));
    let ctx = egui::Context::default();
    app.open_finder(&ctx);
    settle(&mut app);
    let old = app.finder.as_ref().unwrap().revision;
    app.locations.visit(PathBuf::from("root/new"));
    app.reconcile_finder_catalogue();
    let modal = app.finder.as_mut().unwrap();
    assert!(modal.revision > old);
    assert!(
        modal
            .accept(Response {
                revision: old,
                output: Output::Probed(PathBuf::from("wrong"), Action::NewTab)
            })
            .is_none()
    );
    assert!(modal.pending);
    settle(&mut app);
    assert_eq!(
        app.finder.as_ref().unwrap().results[0].location.path,
        PathBuf::from("root/new")
    );
    app.dismiss_finder(&ctx);
    app.open_finder(&ctx);
    let modal = app.finder.as_mut().unwrap();
    assert!(
        modal
            .accept(Response {
                revision: old,
                output: Output::Ranked(Vec::new(), vec!["wrong".into()])
            })
            .is_none()
    );
    assert!(modal.notices.is_empty());
    settle(&mut app);
    let service = finder::Service::with_inventory(Arc::new(|| {}), Some(Vec::new())).unwrap();
    let mut revision = 0;
    for n in 0..200 {
        revision = service.request(Work::Rank {
            query: String::new(),
            session: vec![Location::new(PathBuf::from(n.to_string()), "Recent")],
            children: None,
        });
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(response) = service.take() {
            assert_eq!(response.revision, revision);
            let Output::Ranked(items, _) = response.output else {
                panic!("not ranked")
            };
            assert_eq!(items[0].location.path, PathBuf::from("199"));
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    service.cancel();
    assert!(service.take().is_none());
}
#[test]
fn s010_stale_moved_closed_changed_active_targets_and_split_refusal_do_not_retarget() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    app.open_finder(&ctx);
    settle(&mut app);
    geometry(&mut app, panes::MIN_PANE);
    let ids = (app.next_tab_id, app.next_pane_id);
    assert!(
        app.commit_finder(PathBuf::from("chosen"), Action::Split)
            .unwrap_err()
            .contains("320")
    );
    assert_eq!(app.panes.len(), 1);
    assert_eq!(app.panes[0].tabs.len(), 1);
    assert_eq!((app.next_tab_id, app.next_pane_id), ids);
    assert_eq!(
        app.active_tab_mut().navigation.current.location,
        PathBuf::from("root")
    );
    app.new_tab();
    assert!(
        app.commit_finder(PathBuf::from("wrong"), Action::Navigate)
            .is_err()
    );
    app.dismiss_finder(&ctx);
    app.open_finder(&ctx);
    settle(&mut app);
    geometry(&mut app, egui::vec2(1100.0, 850.0));
    assert!(
        app.commit_finder(PathBuf::from("chosen"), Action::Split)
            .is_ok()
    );
    assert_eq!(app.panes.len(), 2);
    assert_eq!(
        app.panes[1].tabs[0].navigation.current.location,
        PathBuf::from("chosen")
    );
    app.open_finder(&ctx);
    settle(&mut app);
    let captured = app.panes[1].tabs.remove(0); // simulate a moved context
    app.panes[0].tabs.push(captured);
    app.panes[1]
        .tabs
        .push(Tab::new(TabId(99), PathBuf::from("other")));
    assert!(app.finder_target().is_err());
    assert!(
        app.commit_finder(PathBuf::from("wrong"), Action::NewTab)
            .is_err()
    );
    app.close_pane();
    assert!(app.finder_target().unwrap_err().contains("gone"));
}
#[test]
fn s010_modal_headless_keyboard_text_clipboard_background_isolation_and_escape() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    render(&ctx, &mut app, vec![]);
    let original_generation = app.active_tab_mut().navigation.key().generation;
    let original_filter = app.active_tab_mut().filter_text.clone();
    let selection = app.active_tab_mut().navigation.selection.paths.clone();
    render(
        &ctx,
        &mut app,
        vec![key_event(egui::Key::O, egui::Modifiers::CTRL)],
    );
    settle(&mut app);
    render(&ctx, &mut app, vec![]); // focus transfer to input
    assert_eq!(
        ctx.memory(|m| m.focused()),
        Some(egui::Id::new("finder-input"))
    );
    let events = vec![
        key_event(egui::Key::T, egui::Modifiers::CTRL),
        key_event(egui::Key::W, egui::Modifiers::CTRL),
        key_event(egui::Key::Tab, egui::Modifiers::CTRL),
        key_event(egui::Key::F6, egui::Modifiers::NONE),
        key_event(egui::Key::F5, egui::Modifiers::NONE),
        key_event(egui::Key::ArrowLeft, egui::Modifiers::ALT),
        key_event(egui::Key::A, egui::Modifiers::CTRL),
        egui::Event::Text("mado".into()),
        egui::Event::Paste(" exp".into()),
    ];
    render(&ctx, &mut app, events);
    settle(&mut app);
    assert_eq!(app.finder.as_ref().unwrap().query, "mado exp");
    assert_eq!(app.panes[0].tabs.len(), 1);
    assert_eq!(app.panes.len(), 1);
    assert_eq!(app.active_tab_mut().filter_text, original_filter);
    assert_eq!(app.active_tab_mut().navigation.selection.paths, selection);
    assert_eq!(
        app.active_tab_mut().navigation.key().generation,
        original_generation
    );
    render(
        &ctx,
        &mut app,
        vec![
            key_event(egui::Key::Escape, egui::Modifiers::NONE),
            key_event(egui::Key::T, egui::Modifiers::CTRL),
        ],
    );
    assert!(app.finder.is_none());
    assert_eq!(app.panes[0].tabs.len(), 1);
    assert_eq!(ctx.memory(|m| m.focused()), None);
    render(
        &ctx,
        &mut app,
        vec![key_event(egui::Key::T, egui::Modifiers::CTRL)],
    );
    assert_eq!(app.panes[0].tabs.len(), 2); // old behavior restored
}
#[test]
fn s010_modal_headless_all_key_actions_and_selection() {
    let fixture = Fixture::new();
    for (modifiers, action) in [
        (egui::Modifiers::NONE, Action::Navigate),
        (egui::Modifiers::SHIFT, Action::NewTab),
        (egui::Modifiers::CTRL, Action::Split),
    ] {
        let ctx = egui::Context::default();
        let mut app = test_app(fixture.0.clone());
        app.locations.visit(fixture.0.join("mado-explorer"));
        app.locations.visit(fixture.0.clone());
        app.open_finder(&ctx);
        settle(&mut app);
        render(&ctx, &mut app, vec![]);
        render(
            &ctx,
            &mut app,
            vec![key_event(egui::Key::ArrowDown, egui::Modifiers::NONE)],
        );
        assert_eq!(app.finder.as_ref().unwrap().selected, 1);
        render(
            &ctx,
            &mut app,
            vec![key_event(egui::Key::ArrowUp, egui::Modifiers::NONE)],
        );
        assert_eq!(app.finder.as_ref().unwrap().selected, 0);
        typed(&mut app, "mado-explorer");
        render(
            &ctx,
            &mut app,
            vec![key_event(egui::Key::Tab, egui::Modifiers::NONE)],
        );
        settle(&mut app);
        assert!(app.finder.is_some());
        assert_eq!(app.active_tab_mut().navigation.current.location, fixture.0);
        typed(&mut app, "mado-explorer");
        render(&ctx, &mut app, vec![key_event(egui::Key::Enter, modifiers)]);
        settle(&mut app);
        assert!(app.finder.is_none());
        match action {
            Action::Navigate => assert_eq!(
                app.active_tab_mut().navigation.pending_location(),
                Some(fixture.0.join("mado-explorer").as_path())
            ),
            Action::NewTab => {
                assert_eq!(app.panes[0].tabs.len(), 2);
                assert_eq!(
                    app.panes[0].tabs[1].navigation.current.location,
                    fixture.0.join("mado-explorer")
                );
            }
            Action::Split => {
                assert_eq!(app.panes.len(), 2);
                assert_eq!(
                    app.panes[1].tabs[0].navigation.current.location,
                    fixture.0.join("mado-explorer")
                );
            }
        }
    }
}
#[test]
fn s010_session_bookmark_recent_management_only_successful_commits_are_visits() {
    let fixture = Fixture::new();
    let mut app = test_app(fixture.0.clone());
    app.navigate(fixture.0.join("missing"), Intent::New);
    let deadline = Instant::now() + Duration::from_secs(5);
    while app.active_tab_mut().navigation.pending_location().is_some() {
        app.poll_directory();
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(app.locations.recents.is_empty());
    app.navigate(fixture.0.join("mado-explorer"), Intent::New);
    while app.active_tab_mut().navigation.pending_location().is_some() {
        app.poll_directory();
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(
        app.locations.recents[0].path,
        fixture.0.join("mado-explorer")
    );
    let path = app.locations.recents[0].path.clone();
    app.locations.toggle_bookmark(path.clone()).unwrap();
    assert_eq!(app.locations.bookmarks[0].path, path);
    app.locations.forget(&path);
    assert!(app.locations.recents.is_empty());
    assert_eq!(app.locations.bookmarks.len(), 1);
    app.locations.toggle_bookmark(path).unwrap();
    assert!(app.locations.bookmarks.is_empty());
    for n in 0..200 {
        app.locations.visit(PathBuf::from(n.to_string()));
    }
    assert_eq!(app.locations.recents.len(), finder::SESSION_LIMIT);
    for n in 0..finder::SESSION_LIMIT {
        app.locations
            .toggle_bookmark(PathBuf::from(n.to_string()))
            .unwrap();
    }
    assert!(
        app.locations
            .toggle_bookmark(PathBuf::from("overflow"))
            .is_err()
    );
}

#[test]
fn s010_review_expansion_keeps_input_focus_for_keyboard_narrowing() {
    let fixture = Fixture::new();
    let ctx = egui::Context::default();
    let mut app = test_app(fixture.0.clone());
    app.locations.visit(fixture.0.clone());
    app.open_finder(&ctx);
    settle(&mut app);
    render(&ctx, &mut app, vec![]);
    render(&ctx, &mut app, vec![]);
    render(
        &ctx,
        &mut app,
        vec![key_event(egui::Key::Tab, egui::Modifiers::NONE)],
    );
    settle(&mut app);
    render(&ctx, &mut app, vec![]);
    assert_eq!(
        ctx.memory(|m| m.focused()),
        Some(egui::Id::new("finder-input"))
    );
    render(&ctx, &mut app, vec![egui::Event::Text("explorer".into())]);
    settle(&mut app);
    let modal = app.finder.as_ref().unwrap();
    assert_eq!(modal.query, "explorer");
    assert_eq!(modal.results.len(), 1);
    assert_eq!(
        modal.results[0].location.path,
        fixture.0.join("mado-explorer")
    );
    assert_eq!(app.active_tab_mut().navigation.current.location, fixture.0);
}

#[test]
fn s010_review_pasted_paths_are_never_silently_truncated() {
    let fixture = Fixture::new();
    let ctx = egui::Context::default();
    let mut app = test_app(fixture.0.clone());
    app.open_finder(&ctx);
    settle(&mut app);
    typed(&mut app, "");
    render(&ctx, &mut app, vec![]);
    let long = format!(r"\\?\C:\{}", "a\\".repeat(3000));
    render(&ctx, &mut app, vec![egui::Event::Paste(long.clone())]);
    settle(&mut app);
    let modal = app.finder.as_ref().unwrap();
    assert_eq!(modal.query, long);
    assert_eq!(modal.typed_path().unwrap(), PathBuf::from(&long));
    render(
        &ctx,
        &mut app,
        vec![key_event(
            egui::Key::A,
            egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
        )],
    );
    let oversized = format!(r"\\?\C:\{}", "a\\".repeat(17000));
    render(&ctx, &mut app, vec![egui::Event::Paste(oversized.clone())]);
    assert_eq!(app.finder.as_ref().unwrap().query, oversized);
    assert!(app.finder.as_ref().unwrap().error.is_some());
    app.request_finder_action(Some(Action::Navigate));
    assert!(!app.finder.as_ref().unwrap().pending);
    assert!(app.finder.as_ref().unwrap().typed_path().is_err());
    assert_eq!(app.active_tab_mut().navigation.current.location, fixture.0);
}

#[test]
fn s010_review_finder_emits_focused_accessibility_input() {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    let mut app = filter_app();
    app.open_finder(&ctx);
    settle(&mut app);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1100.0, 850.0),
            )),
            ..Default::default()
        },
        |ui| app.finder_workspace(ui),
    );
    let tree = output.platform_output.accesskit_update.as_ref().unwrap();
    let focused = tree.nodes.iter().find(|(id, _)| *id == tree.focus).unwrap();
    assert_eq!(focused.1.role(), egui::accesskit::Role::TextInput);
    output.drop_without_applying_deltas();
    render(
        &ctx,
        &mut app,
        vec![key_event(egui::Key::Escape, egui::Modifiers::NONE)],
    );
    assert!(app.finder.is_none());
    render(
        &ctx,
        &mut app,
        vec![key_event(egui::Key::T, egui::Modifiers::CTRL)],
    );
    assert_eq!(app.panes[0].tabs.len(), 2);
}

#[test]
fn s010_review_opening_pointer_focus_precedes_capture() {
    let ctx = egui::Context::default();
    let mut app = filter_app();
    geometry(&mut app, egui::vec2(1100.0, 850.0));
    assert!(app.split_pane(Axis::LeftRight));
    app.focus_pane(0);
    render(&ctx, &mut app, vec![]);
    let rect = app
        .pane_rects
        .iter()
        .find(|(id, _)| *id == app.panes[1].id)
        .unwrap()
        .1;
    let click = egui::pos2(rect.right() - 15.0, rect.top() + 12.0);
    render(
        &ctx,
        &mut app,
        vec![
            egui::Event::PointerMoved(click),
            egui::Event::PointerButton {
                pos: click,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            key_event(egui::Key::O, egui::Modifiers::CTRL),
            key_event(egui::Key::T, egui::Modifiers::CTRL),
        ],
    );
    assert_eq!(app.focused, 1);
    assert_eq!(app.finder.as_ref().unwrap().target.pane, app.panes[1].id);
    assert_eq!(
        app.finder.as_ref().unwrap().target.tab,
        app.panes[1].tabs[0].id
    );
    assert_eq!(app.panes[0].tabs.len(), 1);
    assert_eq!(app.panes[1].tabs.len(), 1);
}

#[test]
fn s010_ready_probe_cannot_beat_same_frame_query_edit_or_dismissal() {
    let fixture = Fixture::new();
    for cancel_by_escape in [false, true] {
        let ctx = egui::Context::default();
        let mut app = test_app(fixture.0.clone());
        let (sender, receiver) = std::sync::mpsc::channel();
        app.finder_service = finder::Service::with_inventory(
            Arc::new(move || {
                let _ = sender.send(());
            }),
            Some(Vec::new()),
        )
        .unwrap();
        app.open_finder(&ctx);
        settle(&mut app);
        render(&ctx, &mut app, vec![]);
        while receiver.try_recv().is_ok() {}
        typed(&mut app, "mado-explorer");
        app.request_finder_action(Some(Action::Navigate));
        receiver.recv_timeout(Duration::from_secs(5)).unwrap(); // probe result is now ready, but unaccepted
        let events = if cancel_by_escape {
            vec![key_event(egui::Key::Escape, egui::Modifiers::NONE)]
        } else {
            vec![egui::Event::Text("changed".into())]
        };
        render(&ctx, &mut app, events);
        if !cancel_by_escape {
            settle(&mut app);
            assert!(app.finder.is_some());
        }
        assert_eq!(app.active_tab_mut().navigation.current.location, fixture.0);
        assert!(app.active_tab_mut().navigation.pending_location().is_none());
        assert!(app.active_tab_mut().navigation.back.is_empty());
    }
}
