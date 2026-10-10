use mado_core::{
    directory::{DirectoryLoader, Response, Status, Update},
    finder::{Action, Catalogue, Location, Modal, Output, Service, Target, Work},
    ids::{PaneId, TabId},
    navigation::{Intent, Navigation},
    sorting::{Column, Sorter},
};
use std::{
    fs,
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mado-core-c001-{}-{}",
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
        // Only the uniquely created generated fixture is removed.
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn wait(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !ready() {
        assert!(Instant::now() < deadline, "browsing worker timed out");
        thread::sleep(Duration::from_millis(2));
    }
}
fn settle(nav: &mut Navigation, loader: &DirectoryLoader, sorter: &Sorter) {
    let mut requested = None;
    wait(|| {
        loader.drain(|response| {
            nav.apply(response);
        });
        let key = nav.key();
        if requested != Some(key) {
            sorter.request_for(
                1,
                key,
                nav.visible().raw_entries.clone(),
                nav.filter.clone(),
            );
            requested = Some(key);
        }
        while let Some(sorted) = sorter.take() {
            nav.apply_sorted(sorted);
        }
        nav.pending_location().is_none()
            && matches!(nav.current.status, Status::Ready)
            && nav.projection_fresh()
    });
}
fn finder_response(service: &Service) -> mado_core::finder::Response {
    let mut response = None;
    wait(|| {
        response = service.take();
        response.is_some()
    });
    response.unwrap()
}

fn main() {
    let fixture = Fixture::new();
    let a = fixture.0.join("alpha");
    let b = fixture.0.join("日本語-beta");
    fs::create_dir(&a).unwrap();
    fs::create_dir(&b).unwrap();
    fs::create_dir(b.join("folder")).unwrap();
    fs::write(a.join("only-a.txt"), b"a").unwrap();
    fs::write(b.join("alpha.txt"), b"alpha").unwrap();
    fs::write(b.join("alpine.txt"), b"a").unwrap();
    fs::write(b.join("beta.txt"), b"beta").unwrap();

    let wake = Arc::new(|| {});
    let loader = DirectoryLoader::new(wake.clone()).unwrap();
    let sorter = Sorter::new(wake.clone()).unwrap();
    let mut nav = Navigation::new(a.clone());
    nav.begin(a.clone(), loader.request_for(1, a.clone()), Intent::Refresh);
    settle(&mut nav, &loader, &sorter);
    assert_eq!(nav.current.entries[0].path, a.join("only-a.txt"));

    // Hold actual A worker responses, then deliver them after requesting B.
    // This deterministically checks late transport, without relying on a slow OS call.
    let generation = loader.request_for(1, a.clone());
    nav.begin(a.clone(), generation, Intent::Refresh);
    let mut delayed: Vec<Response> = Vec::new();
    wait(|| {
        loader.drain(|response| delayed.push(response));
        delayed
            .iter()
            .any(|r| r.generation == generation && matches!(r.update, Update::Finished(_)))
    });
    nav.begin(b.clone(), loader.request_for(1, b.clone()), Intent::New);
    for response in delayed {
        assert!(!nav.apply(response));
    }
    assert_eq!(nav.pending_location(), Some(b.as_path()));
    settle(&mut nav, &loader, &sorter);
    assert_eq!(nav.current.location, b);
    assert_eq!(nav.back, [a.clone()]);
    assert!(
        nav.current
            .raw_entries
            .iter()
            .all(|e| e.path.parent() == Some(b.as_path()))
    );
    assert_eq!(nav.current.entries[0].name, "folder");
    assert_eq!(nav.current.entries[0].size, None);

    nav.selection.select_all(&nav.current.entries);
    nav.sort.toggle(Column::Size);
    nav.sort.toggle(Column::Size);
    settle(&mut nav, &loader, &sorter);
    assert_eq!(nav.selection.paths.len(), 4);
    assert_eq!(nav.current.entries[1].path, b.join("alpha.txt"));
    nav.set_filter("AL".into());
    settle(&mut nav, &loader, &sorter);
    assert_eq!(nav.current.entries.len(), 2);
    assert_eq!(
        nav.selection.paths,
        [b.join("alpha.txt"), b.join("alpine.txt")]
            .into_iter()
            .collect()
    );
    nav.set_filter(String::new());
    settle(&mut nav, &loader, &sorter);
    assert_eq!(nav.current.entries.len(), 4);
    assert_eq!(nav.selection.paths.len(), 2);

    let mut catalogue = Catalogue::default();
    catalogue.visit(a.clone());
    catalogue.toggle_bookmark(b.clone()).unwrap();
    let finder =
        Service::with_inventory(wake, Some(vec![Location::new(b.clone(), "Owned fixture")]))
            .unwrap();
    let mut modal = Modal::new(Target {
        pane: PaneId(1),
        tab: TabId(1),
        base: b.clone(),
        generation: nav.key().generation,
    });
    modal.revision = finder.request(Work::Rank {
        query: String::new(),
        session: catalogue.items(),
        children: None,
    });
    let stale = finder_response(&finder);
    modal.revision = finder.request(Work::Rank {
        query: "beta".into(),
        session: catalogue.items(),
        children: None,
    });
    modal.pending = true;
    assert!(modal.accept(stale).is_none());
    assert!(modal.pending && modal.results.is_empty());
    modal.accept(finder_response(&finder));
    assert_eq!(modal.selected_path(), Some(b.clone()));
    assert!(modal.results.iter().any(|item| !item.name_marks.is_empty()));
    modal.query = "..\\alpha".into();
    #[cfg(not(windows))]
    {
        modal.query = "../alpha".into();
    }
    modal.path_mode = true;
    let requested = modal.typed_path().unwrap();
    modal.revision = finder.request(Work::Probe(requested, Action::Navigate));
    let response = finder_response(&finder);
    assert!(matches!(
        response.output,
        Output::Probed(_, Action::Navigate)
    ));
    let (path, action) = modal.accept(response).unwrap();
    assert_eq!(action, Action::Navigate);
    assert_eq!(path, a);
    nav.begin(path.clone(), loader.request_for(1, path), Intent::Back);
    settle(&mut nav, &loader, &sorter);
    assert_eq!(nav.current.location, a);
    assert!(nav.back.is_empty());
    assert_eq!(nav.forward, [b]);
    println!(
        "Generated-fixture listing, stale navigation, filter/sort/selection and finder check passed."
    );
}
