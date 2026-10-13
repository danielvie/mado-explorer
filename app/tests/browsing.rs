use eframe::egui;
use std::{
    path::PathBuf,
    time::{Duration, Instant, SystemTime},
};
use wayfarer_files::{
    directory::{Browser, Entry, Snapshot},
    ui::{BrowserUi, ListingView},
};

fn synthetic(count: usize) -> Snapshot {
    let path = PathBuf::from(r"C:\wayfarer-synthetic-headless");
    let entries = (0..count)
        .map(|index| {
            let name = format!("item-{index:06}.txt");
            Entry {
                path: path.join(&name),
                name: name.into(),
                is_dir: false,
                size: Some(42),
                modified: None,
                hidden: false,
                system: false,
            }
        })
        .collect();
    Snapshot {
        path,
        entries,
        error: None,
    }
}
fn rendered_rows(count: usize, view: ListingView) -> usize {
    let started = Instant::now();
    let snapshot = synthetic(count);
    let mut browser = BrowserUi::new(snapshot.path.clone(), snapshot.path.clone());
    let tab = browser.active_tab().unwrap();
    tab.visible = snapshot
        .entries
        .iter()
        .map(|entry| entry.path.clone())
        .collect();
    tab.browser = Browser::from_snapshot(snapshot);
    tab.view = view;
    let context = egui::Context::default();
    let mut measured = 0;
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1024.0, 768.0),
            )),
            ..Default::default()
        };
        let mut output = context.run_ui(input, |ui| {
            assert!(browser.show(ui).is_empty());
        });
        // This headless test deliberately has no renderer consuming atlas changes.
        output.textures_delta.clear();
        assert_eq!(browser.pane_targets.len(), 1);
        assert!(browser.pane_targets[0].0.is_positive());
        let current = browser
            .active_tab()
            .unwrap()
            .browser
            .current_path()
            .to_owned();
        assert_eq!(browser.pane_targets[0].1, current);
        measured = measured.max(browser.active_tab().unwrap().rows_rendered);
    }
    eprintln!(
        "Browsing baseline: {count} {view:?} entries; construction+three frames {:?}; {measured} visible rows; entry-header storage {} bytes (excludes path/name buffers, allocator and renderer)",
        started.elapsed(),
        count * std::mem::size_of::<Entry>()
    );
    measured
}
#[test]
fn browsing_virtualization_ten_and_hundred_thousand_rows() {
    let smaller = rendered_rows(10_000, ListingView::Details);
    let larger = rendered_rows(100_000, ListingView::Details);
    // 768/24 visible rows plus four rows of documented allowance for toolkit overscan.
    let bound = 768 / 24 + 4;
    assert!(smaller > 0 && larger > 0);
    assert!(smaller <= bound, "10k constructed {smaller} rows");
    assert!(larger <= bound, "100k constructed {larger} rows");
    assert_eq!(
        smaller, larger,
        "same viewport must have size-independent construction"
    );
}
#[test]
fn icon_view_virtualizes_hundred_thousand_items() {
    let smaller = rendered_rows(10_000, ListingView::Icons);
    let larger = rendered_rows(100_000, ListingView::Icons);
    // At 96px zoom, row height is 158px; toolkit overscan allowance is four rows.
    assert!(smaller > 0 && larger > 0);
    assert!(larger <= 768 / 158 + 4, "constructed {larger} icon rows");
    assert_eq!(smaller, larger);
}
fn wait_listing(browser: &mut Browser) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while browser.is_loading() {
        browser.poll();
        assert!(Instant::now() < deadline, "directory worker timed out");
        std::thread::sleep(Duration::from_millis(5));
    }
}
#[test]
fn browsing_owned_async_filter_generation_and_navigation() {
    let root = std::env::temp_dir().join(format!(
        "wayfarer-browsing-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    std::fs::write(root.join("alpha.txt"), b"owned fixture").unwrap();
    std::fs::write(root.join("beta.txt"), b"owned fixture").unwrap();
    let mut browser = Browser::new();
    browser.navigate(root.clone());
    wait_listing(&mut browser);
    assert_eq!(browser.snapshot().unwrap().entries.len(), 2);
    let initial_generation = browser.generation();
    let mut options = browser.options().clone();
    options.filter = "ALPHA".into();
    browser.set_options(options);
    assert!(browser.generation() > initial_generation);
    wait_listing(&mut browser);
    assert_eq!(browser.snapshot().unwrap().entries.len(), 1);
    assert_eq!(
        browser.snapshot().unwrap().entries[0].path,
        root.join("alpha.txt")
    );
    browser.navigate(root.join("missing"));
    wait_listing(&mut browser);
    assert!(browser.snapshot().unwrap().error.is_some());
    drop(browser);
    std::fs::remove_file(root.join("alpha.txt")).unwrap();
    std::fs::remove_file(root.join("beta.txt")).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match std::fs::remove_dir(&root) {
            Ok(()) => break,
            Err(error) if Instant::now() >= deadline => panic!("fixture cleanup: {error}"),
            Err(_) => std::thread::sleep(Duration::from_millis(10)),
        }
    }
}
#[test]
fn browsing_close_reconciles_active_and_retains_home_tab() {
    let home = std::env::temp_dir();
    let mut browser = BrowserUi::new(home.clone(), home.clone());
    browser.add_tab(home.join("wayfarer-missing-test-location"));
    let second = browser.active;
    browser.close_active();
    assert_ne!(browser.active, second);
    assert!(browser.active_tab().is_some());
    browser.close_active();
    assert_eq!(browser.dock.iter_all_tabs().count(), 1);
    assert_eq!(browser.active_tab().unwrap().browser.current_path(), home);
}

#[test]
fn browsing_duplicate_and_reopen_preserve_independent_owned_state() {
    let home = std::env::temp_dir();
    let mut browser = BrowserUi::new(home.clone(), home.clone());
    {
        let tab = browser.active_tab().unwrap();
        tab.view = ListingView::Icons;
        tab.icon_size = 144.0;
        tab.word_wrap = false;
        tab.history.push(home.join("historical"));
        tab.history_index = 0;
        let mut options = tab.browser.options().clone();
        options.filter = "source".into();
        tab.browser.set_options(options);
    }
    browser.duplicate_active();
    assert_eq!(browser.dock.iter_all_tabs().count(), 2);
    let duplicated_id = browser.active;
    {
        let tab = browser.active_tab().unwrap();
        assert_eq!(tab.view, ListingView::Icons);
        assert_eq!(tab.icon_size, 144.0);
        assert!(!tab.word_wrap);
        assert_eq!(tab.history.len(), 2);
        assert_eq!(tab.browser.options().filter, "source");
        let mut options = tab.browser.options().clone();
        options.filter = "independent".into();
        tab.browser.set_options(options);
    }
    assert_eq!(
        browser
            .dock
            .iter_all_tabs()
            .find(|(_, tab)| tab.id == 1)
            .unwrap()
            .1
            .browser
            .options()
            .filter,
        "source"
    );
    browser.close_active();
    browser.reopen_closed();
    assert_ne!(browser.active, duplicated_id);
    let tab = browser.active_tab().unwrap();
    assert_eq!(tab.browser.options().filter, "independent");
    assert_eq!(tab.history.len(), 2);
    assert_eq!(tab.icon_size, 144.0);
}
#[test]
fn browsing_pane_switch_and_equalize_preserve_locations() {
    let home = std::env::temp_dir();
    let mut browser = BrowserUi::new(home.clone(), home.clone());
    browser.dock.main_surface_mut().split_right(
        egui_dock::NodeIndex::root(),
        0.2,
        vec![wayfarer_files::ui::Tab::new(2, home.join("second"))],
    );
    browser.active = 1;
    browser.next_pane();
    assert_eq!(browser.active, 2);
    browser.next_pane();
    assert_eq!(browser.active, 1);
    browser.equalize_panes();
    match &browser.dock.main_surface()[egui_dock::NodeIndex::root()] {
        egui_dock::Node::Horizontal(split) | egui_dock::Node::Vertical(split) => {
            assert_eq!(split.fraction, 0.5)
        }
        _ => panic!("expected split"),
    }
    assert_eq!(browser.active_tab().unwrap().browser.current_path(), home);
}

#[test]
fn bin_location_never_exposes_filesystem_actions_or_drop_targets() {
    let home = std::env::temp_dir();
    let mut browser = BrowserUi::new(home.clone(), home.clone());
    // Synthetic virtual location: exercise routing without reading the real Recycle Bin.
    browser.active_any_tab().unwrap().kind = wayfarer_files::ui::LocationKind::RecycleBin;
    assert!(browser.active_tab().is_none());
    let context = egui::Context::default();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        assert!(browser.show(ui).is_empty());
    });
    output.textures_delta.clear();
    assert!(browser.pane_targets.is_empty());
    browser.close_active();
    assert!(browser.active_tab().is_some());
    assert_eq!(browser.active_tab().unwrap().browser.current_path(), home);
}
