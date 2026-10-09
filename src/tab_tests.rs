use super::*;
use crate::tests::{filter_app, key_event};

fn finish_tab(tab: &mut Tab, entries: Arc<Vec<Arc<directory::Entry>>>) {
    let generation = tab.navigation.key().generation;
    tab.apply(directory::Response {
        generation,
        update: directory::Update::Snapshot {
            entries,
            metadata_errors: 0,
        },
    });
    let location = tab.navigation.visible().location.clone();
    assert!(tab.apply(directory::Response {
        generation,
        update: directory::Update::Finished(Ok(location)),
    }));
    project_tab(tab);
}

fn project_tab(tab: &mut Tab) {
    let nav = &mut tab.navigation;
    assert!(nav.apply_sorted(sorting::project_for_test(
        nav.key(),
        nav.current.raw_entries.clone(),
        &nav.filter
    )));
}

#[test]
fn s007_new_close_cycle_and_final_home_use_fresh_non_reused_ids() {
    let mut app = filter_app();
    app.home = PathBuf::from("injected-home");
    app.navigate(PathBuf::from("pending-destination"), Intent::New);
    app.new_tab();
    assert_eq!(app.panes[app.focused].tabs.len(), 2);
    assert_eq!(
        app.active_tab_mut().navigation.current.location,
        PathBuf::from("root")
    );
    assert!(app.active_tab_mut().navigation.back.is_empty());
    assert!(
        app.panes[app.focused].tabs[0]
            .navigation
            .pending_location()
            .is_none()
    );
    assert!(!matches!(
        app.panes[app.focused].tabs[0].navigation.current.status,
        Status::Loading
    ));
    let id = app.panes[app.focused].tabs[1].id;
    app.cycle_tab(false);
    assert_eq!(app.panes[app.focused].active, 0);
    app.cycle_tab(true);
    assert_eq!(app.panes[app.focused].active, 1);
    app.close_tab(app.panes[app.focused].active);
    assert_eq!(app.panes[app.focused].active, 0);
    app.new_tab();
    assert!(app.active_tab_mut().id.0 > id.0);
    let key = app.active_tab_mut().navigation.key();
    app.close_tab(0);
    assert_eq!(app.panes[app.focused].active, 0);
    assert_eq!(app.panes[app.focused].tabs[0].navigation.key(), key); // inactive close does not reload
    app.cycle_tab(false);
    assert_eq!(app.panes[app.focused].tabs[0].navigation.key(), key); // single-tab cycle is a no-op
    app.close_tab(0);
    assert_eq!(app.panes[app.focused].tabs.len(), 1);
    assert_eq!(
        app.panes[app.focused].tabs[0].navigation.current.location,
        app.home
    );
    assert!(app.panes[app.focused].tabs[0].id.0 > id.0);
    assert!(
        app.panes[app.focused].tabs[0].navigation.back.is_empty()
            && app.panes[app.focused].tabs[0].navigation.forward.is_empty()
    );
    assert!(
        app.panes[app.focused].tabs[0]
            .navigation
            .selection
            .paths
            .is_empty()
    );
    assert!(app.panes[app.focused].tabs[0].filter_text.is_empty());
    assert_eq!(
        app.panes[app.focused].tabs[0].navigation.sort,
        sorting::Sort::default()
    );
}

#[test]
fn s007_return_restores_independent_history_filter_sort_selection_and_editors() {
    let mut app = filter_app();
    let data = app.panes[app.focused].tabs[0]
        .navigation
        .current
        .raw_entries
        .clone();
    let tab = &mut app.panes[app.focused].tabs[0];
    tab.navigation.back.push(PathBuf::from("previous-a"));
    tab.navigation.forward.push(PathBuf::from("forward-a"));
    tab.filter_text = "al".into();
    tab.navigation.set_filter(tab.filter_text.clone());
    tab.navigation.sort.toggle(Column::Size);
    tab.navigation.set_show_hidden_system(true);
    tab.navigation.selection.click(&data, 0, false, false);
    tab.location_text = "unsubmitted draft".into();
    tab.location_error = Some("retained path error".into());
    tab.row_scroll_offset = 400.0;
    let selection = tab.navigation.selection.paths.clone();
    let active = tab.navigation.selection.active.clone();
    let anchor = tab.navigation.selection.anchor.clone();
    let sort = tab.navigation.sort;
    app.new_tab();
    let tab = &mut app.panes[app.focused].tabs[1];
    assert!(tab.navigation.filter.is_empty());
    assert!(!tab.navigation.show_hidden_system);
    finish_tab(tab, data.clone());
    tab.navigation.selection.click(&data, 1, false, false);
    let second_active = tab.navigation.selection.active.clone();
    tab.navigation.back.push(PathBuf::from("previous-b"));
    tab.navigation.set_filter("be".into());
    tab.filter_text = "be".into();
    tab.navigation.sort.toggle(Column::Modified);
    app.activate_tab(0);
    finish_tab(&mut app.panes[app.focused].tabs[0], data);
    let tab = &app.panes[app.focused].tabs[0];
    assert_eq!(tab.navigation.back, [PathBuf::from("previous-a")]);
    assert_eq!(tab.navigation.forward, [PathBuf::from("forward-a")]);
    assert_eq!(tab.navigation.filter, "al");
    assert_eq!(tab.filter_text, "al");
    assert_eq!(tab.navigation.sort, sort);
    assert!(tab.navigation.show_hidden_system);
    assert_eq!(tab.navigation.selection.paths, selection);
    assert_eq!(tab.navigation.selection.active, active);
    assert_eq!(tab.navigation.selection.anchor, anchor);
    assert_eq!(tab.location_text, "unsubmitted draft");
    assert_eq!(tab.location_error.as_deref(), Some("retained path error"));
    assert_eq!(tab.row_scroll_offset, 400.0);
    assert_eq!(app.panes[app.focused].tabs[1].navigation.filter, "be");
    assert_eq!(
        app.panes[app.focused].tabs[1].navigation.back,
        [PathBuf::from("previous-b")]
    );
    assert_eq!(
        app.panes[app.focused].tabs[1].navigation.selection.active,
        second_active
    );
}

#[cfg(windows)]
#[test]
fn s007_native_selection_and_late_closed_tab_responses_cannot_cross_contexts() {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    let mut app = filter_app();
    let native = PathBuf::from(OsString::from_wide(&[0x78, 0xD800]));
    let mut rows = selection::tests::rows(&["native"]);
    Arc::get_mut(&mut Arc::get_mut(&mut rows).unwrap()[0])
        .unwrap()
        .path = native.clone();
    finish_tab(&mut app.panes[app.focused].tabs[0], rows.clone());
    app.panes[app.focused].tabs[0]
        .navigation
        .selection
        .select_all(&rows);
    app.navigate(PathBuf::from("uncommitted"), Intent::New);
    let stale = app.panes[app.focused].tabs[0].navigation.key();
    app.new_tab();
    let active_key = app.panes[app.focused].tabs[1].navigation.key();
    assert_ne!(active_key.generation, stale.generation);
    let late = || directory::Response {
        generation: stale.generation,
        update: directory::Update::Finished(Ok(PathBuf::from("late"))),
    };
    assert!(!app.panes[app.focused].tabs[0].apply(late()));
    assert!(!app.panes[app.focused].tabs[1].apply(late()));
    assert!(
        !app.panes[app.focused].tabs[1]
            .navigation
            .apply_sorted(sorting::Sorted::for_test(stale, rows.clone()))
    );
    assert!(app.panes[app.focused].tabs[1].navigation.back.is_empty());
    app.activate_tab(0);
    finish_tab(&mut app.panes[app.focused].tabs[0], rows.clone());
    assert_eq!(
        app.panes[app.focused].tabs[0].navigation.selection.active,
        Some(native.clone())
    );
    assert!(
        app.panes[app.focused].tabs[0]
            .navigation
            .selection
            .paths
            .contains(&native)
    );
    let closed_key = app.panes[app.focused].tabs[0].navigation.key();
    app.close_tab(0);
    let tab = &mut app.panes[app.focused].tabs[0];
    assert!(!tab.apply(directory::Response {
        generation: closed_key.generation,
        update: directory::Update::Snapshot {
            entries: rows.clone(),
            metadata_errors: 0
        }
    }));
    assert!(
        !tab.navigation
            .apply_sorted(sorting::Sorted::for_test(closed_key, rows))
    );
    assert!(tab.navigation.back.is_empty() && tab.navigation.selection.paths.is_empty());
}

#[test]
fn s007_saved_manual_scroll_waits_for_complete_fresh_projection_not_active_row() {
    let context = egui::Context::default();
    let mut app = filter_app();
    let names: Vec<_> = (0..200).map(|i| format!("row-{i:03}")).collect();
    let data = selection::tests::rows(&names.iter().map(String::as_str).collect::<Vec<_>>());
    finish_tab(&mut app.panes[app.focused].tabs[0], data.clone());
    app.panes[app.focused].tabs[0]
        .navigation
        .selection
        .click(&data, 0, false, false);
    app.panes[app.focused].tabs[0].row_scroll_offset = 2200.0;
    let render = |app: &mut Explorer| {
        context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(800.0, 400.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    ui.spacing_mut().item_spacing.y = 3.0;
                    app.table(ui, false);
                },
            )
            .drop_without_applying_deltas();
    };
    render(&mut app);
    // A user scroll, not a selected-row target, is the position to retain.
    context
        .run_ui(
            egui::RawInput {
                events: vec![egui::Event::PointerMoved(egui::pos2(100.0, 200.0))],
                ..Default::default()
            },
            |ui| {
                ui.spacing_mut().item_spacing.y = 3.0;
                ui.input_mut(|input| input.smooth_scroll_delta = egui::vec2(0.0, -400.0));
                app.table(ui, false);
            },
        )
        .drop_without_applying_deltas();
    let saved = app.panes[app.focused].tabs[0].row_scroll_offset;
    assert!(saved > 2200.0);
    app.new_tab();
    finish_tab(&mut app.panes[app.focused].tabs[1], data.clone());
    app.panes[app.focused].tabs[1].row_scroll_offset = 800.0;
    render(&mut app);
    assert!((app.panes[app.focused].tabs[1].row_scroll_offset - 800.0).abs() < 0.1);
    app.activate_tab(0);
    render(&mut app);
    assert_eq!(app.panes[app.focused].tabs[0].row_scroll_offset, saved);
    let generation = app.panes[app.focused].tabs[0].navigation.key().generation;
    app.panes[app.focused].tabs[0].apply(directory::Response {
        generation,
        update: directory::Update::Snapshot {
            entries: Arc::new(vec![data[0].clone()]),
            metadata_errors: 0,
        },
    });
    project_tab(&mut app.panes[app.focused].tabs[0]);
    render(&mut app);
    assert_eq!(app.panes[app.focused].tabs[0].row_scroll_offset, saved);
    finish_tab(&mut app.panes[app.focused].tabs[0], data.clone());
    render(&mut app);
    assert!((app.panes[app.focused].tabs[0].row_scroll_offset - saved).abs() < 0.1);
    assert!(!app.panes[app.focused].tabs[0].restore_scroll);
    context
        .run_ui(
            egui::RawInput {
                events: vec![key_event(egui::Key::ArrowDown, egui::Modifiers::NONE)],
                ..Default::default()
            },
            |ui| {
                app.table(ui, false);
            },
        )
        .drop_without_applying_deltas();
    assert!(
        app.panes[app.focused].tabs[0].row_scroll_offset < 100.0,
        "explicit keyboard movement still reveals the active row"
    );
}

#[test]
fn s007_shortcuts_obey_focused_editors_submission_and_exact_modifiers() {
    for (key, modifiers, command) in [
        (egui::Key::T, egui::Modifiers::CTRL, TabShortcut::New),
        (egui::Key::W, egui::Modifiers::CTRL, TabShortcut::Close),
        (egui::Key::Tab, egui::Modifiers::CTRL, TabShortcut::Next),
        (
            egui::Key::Tab,
            egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
            TabShortcut::Previous,
        ),
    ] {
        for editor in ["filter", "location"] {
            let context = egui::Context::default();
            let mut app = filter_app();
            context
                .run_ui(egui::RawInput::default(), |ui| {
                    app.toolbar(ui);
                    ui.memory_mut(|m| {
                        m.request_focus(app.panes[app.focused].tabs[0].widget_id(editor))
                    });
                })
                .drop_without_applying_deltas();
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![key_event(key, modifiers)],
                        ..Default::default()
                    },
                    |ui| {
                        let editing = app.toolbar(ui);
                        assert!(editing);
                        assert_eq!(tab_shortcut(ui.ctx(), editing), None);
                    },
                )
                .drop_without_applying_deltas();
        }
        let context = egui::Context::default();
        let mut app = filter_app();
        context
            .run_ui(
                egui::RawInput {
                    events: vec![key_event(key, modifiers)],
                    ..Default::default()
                },
                |ui| {
                    app.tab_strip(ui);
                    let editing = app.toolbar(ui);
                    assert!(!editing, "tab shortcut must not focus an editor first");
                    let shortcut = tab_shortcut(ui.ctx(), editing);
                    assert_eq!(shortcut, Some(command));
                    app.apply_tab_shortcut(shortcut.unwrap());
                },
            )
            .drop_without_applying_deltas();
        for extra in [egui::Modifiers::ALT, egui::Modifiers::MAC_CMD] {
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![key_event(key, modifiers | extra)],
                        ..Default::default()
                    },
                    |ui| {
                        assert_eq!(tab_shortcut(ui.ctx(), false), None);
                    },
                )
                .drop_without_applying_deltas();
        }
        context
            .run_ui(
                egui::RawInput {
                    events: vec![key_event(key, modifiers)],
                    ..Default::default()
                },
                |ui| {
                    assert_eq!(tab_shortcut(ui.ctx(), true), None);
                },
            )
            .drop_without_applying_deltas();
    }
}

#[test]
fn s007_same_location_ids_and_overflow_keep_active_tab_and_plus_reachable() {
    let context = egui::Context::default();
    let mut app = filter_app();
    for _ in 0..19 {
        app.new_tab();
    }
    for name in ["location", "filter", "directory", "table-horizontal"] {
        assert_ne!(
            app.panes[app.focused].tabs[0].widget_id(name),
            app.panes[app.focused].tabs[1].widget_id(name)
        );
    }
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(480.0, 280.0));
    for backwards in [false, true] {
        app.cycle_tab(backwards);
        app.active_tab_mut().navigation.current.location = PathBuf::from("active-overflow-tab");
        app.panes[app.focused].reveal_active_tab = true;
        let mut active_rect = egui::Rect::NOTHING;
        let mut plus = egui::Pos2::ZERO;
        for _ in 0..6 {
            let output = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                },
                |ui| app.tab_strip(ui),
            );
            for shape in &output.shapes {
                if let egui::Shape::Text(text) = &shape.shape {
                    if text.galley.job.text == "+" {
                        plus = text.pos + text.galley.size() / 2.0;
                    }
                    if text.galley.job.text == "active-overflow-tab" {
                        active_rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                    }
                }
            }
            output.drop_without_applying_deltas();
        }
        assert!(
            screen.contains(plus),
            "plus must stay outside horizontal overflow"
        );
        assert!(
            screen.contains_rect(active_rect),
            "active label must be scrolled into view: {active_rect:?}"
        );
        app.active_tab_mut().navigation.current.location = PathBuf::from("root");
    }
}

use crate::tests::test_app;
use std::{
    fs,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mado-s007-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        Self(root)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn settle(app: &mut Explorer) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        app.poll_directory();
        let nav = &app.active_tab_mut().navigation;
        if nav.pending_location().is_none()
            && matches!(nav.current.status, Status::Ready)
            && nav.projection_fresh()
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "tab did not finish enumeration/projection"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn s007_shared_background_refresh_reconciles_only_active_tab_and_final_fixture_home() {
    let fixture = Fixture::new();
    let home = fixture.0.join("injected-home");
    fs::create_dir(&home).unwrap();
    fs::write(fixture.0.join("alpha.txt"), b"alpha").unwrap();
    fs::write(fixture.0.join("beta.txt"), b"beta").unwrap();
    let mut app = test_app(fixture.0.clone());
    app.home = home.clone();
    app.navigate(fixture.0.clone(), Intent::Refresh);
    settle(&mut app);
    app.panes[app.focused].tabs[0]
        .navigation
        .set_filter("alpha".into());
    settle(&mut app);
    let tab = &mut app.panes[app.focused].tabs[0];
    tab.filter_text = "alpha".into();
    tab.navigation
        .selection
        .select_all(&tab.navigation.current.entries);
    let selected_a = tab.navigation.selection.paths.clone();
    app.new_tab();
    settle(&mut app);
    app.panes[app.focused].tabs[1]
        .navigation
        .set_filter("beta".into());
    settle(&mut app);
    let tab = &mut app.panes[app.focused].tabs[1];
    tab.filter_text = "beta".into();
    tab.navigation
        .selection
        .select_all(&tab.navigation.current.entries);
    assert_eq!(tab.navigation.selection.paths.len(), 1);
    fs::remove_file(fixture.0.join("beta.txt")).unwrap();
    fs::write(fixture.0.join("beta-new.txt"), b"new").unwrap();
    app.activate_tab(0);
    settle(&mut app);
    assert_eq!(
        app.panes[app.focused].tabs[0].navigation.selection.paths,
        selected_a
    );
    assert_eq!(app.panes[app.focused].tabs[0].navigation.filter, "alpha");
    assert!(
        app.panes[app.focused].tabs[1]
            .navigation
            .selection
            .paths
            .contains(&fixture.0.join("beta.txt")),
        "inactive selection is not reconciled by another tab's refresh"
    );
    app.activate_tab(1);
    settle(&mut app);
    assert!(
        app.panes[app.focused].tabs[1]
            .navigation
            .selection
            .paths
            .is_empty()
    );
    assert_eq!(
        app.panes[app.focused].tabs[1].navigation.current.entries[0].path,
        fixture.0.join("beta-new.txt")
    );
    app.navigate(home.clone(), Intent::New);
    settle(&mut app);
    assert_eq!(
        app.panes[app.focused].tabs[1].navigation.back,
        [fixture.0.clone()]
    );
    app.activate_tab(0);
    settle(&mut app);
    assert!(app.panes[app.focused].tabs[0].navigation.back.is_empty());
    app.close_tab(1);
    app.close_tab(0);
    settle(&mut app);
    assert_eq!(
        app.panes[app.focused].tabs[0].navigation.current.location,
        home
    );
    assert!(
        app.panes[app.focused].tabs[0]
            .navigation
            .current
            .raw_entries
            .is_empty()
    );
    assert!(app.panes[app.focused].tabs[0].navigation.filter.is_empty());
}

#[test]
fn s007_plus_and_close_controls_create_tabs_and_replace_the_final_tab() {
    let context = egui::Context::default();
    let mut app = filter_app();
    let click_label = |app: &mut Explorer, label: &str| {
        let output = context.run_ui(egui::RawInput::default(), |ui| app.tab_strip(ui));
        let pos = output
            .shapes
            .iter()
            .rev()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == label => {
                    Some(text.pos + text.galley.size() / 2.0)
                }
                _ => None,
            })
            .expect("tab control must be drawn");
        output.drop_without_applying_deltas();
        for pressed in [true, false] {
            context
                .run_ui(
                    egui::RawInput {
                        events: vec![
                            egui::Event::PointerMoved(pos),
                            egui::Event::PointerButton {
                                pos,
                                button: egui::PointerButton::Primary,
                                pressed,
                                modifiers: egui::Modifiers::NONE,
                            },
                        ],
                        ..Default::default()
                    },
                    |ui| app.tab_strip(ui),
                )
                .drop_without_applying_deltas();
        }
    };
    click_label(&mut app, "+");
    assert_eq!(app.panes[app.focused].tabs.len(), 2);
    assert_eq!(app.panes[app.focused].active, 1);
    click_label(&mut app, "x");
    assert_eq!(app.panes[app.focused].tabs.len(), 1);
    assert_eq!(app.panes[app.focused].active, 0);
    click_label(&mut app, "x");
    assert_eq!(app.panes[app.focused].tabs.len(), 1);
    assert_eq!(
        app.panes[app.focused].tabs[0].navigation.current.location,
        app.home
    );
}
