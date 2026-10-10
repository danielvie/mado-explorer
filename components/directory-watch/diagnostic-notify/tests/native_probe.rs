#![cfg(windows)]

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("mado-p04-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // notify's Drop queues stop rather than joining its Windows driver.
        // Cleanup is limited to this fixture, never a caller-supplied location.
        for _ in 0..100 {
            match fs::remove_dir_all(&self.0) {
                Ok(()) => return,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
                Err(_) => std::thread::sleep(Duration::from_millis(20)),
            }
        }
        eprintln!("owned fixture cleanup pending: {:?}", self.0);
    }
}

fn wait_event(
    rx: &mpsc::Receiver<notify::Result<Event>>,
    path: &Path,
    kind: fn(&EventKind) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let event = rx
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .unwrap()
            .unwrap();
        assert!(
            !event.need_rescan(),
            "unexpected rescan during ordinary event probe"
        );
        if event.paths.iter().any(|p| p == path) && kind(&event.kind) {
            return;
        }
    }
}

#[test]
fn native_create_metadata_rename_delete_and_pending_drop() {
    let fixture = Fixture::new();
    let watched = fixture.0.join("données-日本-𐐀");
    fs::create_dir(&watched).unwrap();
    let sentinel = fixture.0.join("sentinel");
    fs::write(&sentinel, b"unchanged").unwrap();
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
    watcher
        .watch(&watched, RecursiveMode::NonRecursive)
        .unwrap();
    watcher
        .watch(&fixture.0, RecursiveMode::NonRecursive)
        .unwrap();

    let file = watched.join("épreuve-𐐀.txt");
    fs::write(&file, b"one").unwrap();
    wait_event(&rx, &file, EventKind::is_create);
    fs::write(&file, b"two changed bytes").unwrap();
    wait_event(&rx, &file, EventKind::is_modify);
    let renamed = watched.join("renamed.txt");
    fs::rename(&file, &renamed).unwrap();
    wait_event(&rx, &renamed, EventKind::is_modify);
    fs::remove_file(&renamed).unwrap();
    wait_event(&rx, &renamed, EventKind::is_remove);

    let moved = fixture.0.join("moved");
    fs::rename(&watched, &moved).unwrap();
    wait_event(&rx, &watched, EventKind::is_modify);
    fs::remove_dir(&moved).unwrap();
    fs::create_dir(&watched).unwrap();
    wait_event(&rx, &watched, EventKind::is_create);
    // Parent notification does not re-register the recreated folder automatically.
    watcher.unwatch(&watched).unwrap();
    watcher
        .watch(&watched, RecursiveMode::NonRecursive)
        .unwrap();
    let reappeared = watched.join("after-recreation");
    fs::write(&reappeared, b"sentinel contents").unwrap();
    wait_event(&rx, &reappeared, EventKind::is_create);
    assert_eq!(fs::read(&sentinel).unwrap(), b"unchanged");
    assert_eq!(fs::read(&reappeared).unwrap(), b"sentinel contents");

    // Exercise a pending async read. Passing is a smoke check, not proof of UAF freedom.
    let started = Instant::now();
    drop(watcher);
    assert!(started.elapsed() < Duration::from_secs(1));
    println!("native ordinary events, parent root lifecycle and pending Drop smoke passed");
}

#[test]
fn windows_overflow_must_report_rescan() {
    let fixture = Fixture::new();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let creates = Arc::new(AtomicUsize::new(0));
    let rescan_or_error = Arc::new(AtomicBool::new(false));
    let seen_creates = creates.clone();
    let seen_failure = rescan_or_error.clone();
    let mut gate = Some(release_rx);
    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<Event>| {
            match result {
                Ok(event) => {
                    if event.kind.is_create() {
                        seen_creates.fetch_add(1, Ordering::Relaxed);
                    }
                    if event.need_rescan() {
                        seen_failure.store(true, Ordering::Relaxed);
                    }
                }
                Err(_) => seen_failure.store(true, Ordering::Relaxed),
            }
            if let Some(gate) = gate.take() {
                entered_tx.send(()).unwrap();
                // The dependency already armed the next native read before invoking us.
                // Hold this first callback while its 16 KiB buffer receives a burst.
                let _ = gate.recv_timeout(Duration::from_secs(15));
            }
        },
        notify::Config::default(),
    )
    .unwrap();
    watcher
        .watch(&fixture.0, RecursiveMode::NonRecursive)
        .unwrap();
    fs::write(fixture.0.join("trigger"), b"").unwrap();
    entered_rx.recv_timeout(Duration::from_secs(5)).unwrap();

    const COUNT: usize = 800;
    let suffix = "x".repeat(120);
    for i in 0..COUNT {
        fs::write(fixture.0.join(format!("{i:04}-{suffix}")), b"").unwrap();
    }
    release_tx.send(()).unwrap();
    // This bounded probe waits for the callback queue to drain, not for application polling.
    std::thread::sleep(Duration::from_secs(2));
    let observed = creates.load(Ordering::Relaxed);
    let reported = rescan_or_error.load(Ordering::Relaxed);
    drop(watcher);
    println!(
        "overflow fixture: generated={}, observed creates={observed}, rescan/error={reported}",
        COUNT + 1
    );
    assert!(
        observed < COUNT + 1,
        "probe did not induce notification loss; evidence inconclusive"
    );
    assert!(
        reported,
        "native events were lost without a rescan/error diagnostic"
    );
}
