use super::*;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub(super) struct Fixture {
    pub root: PathBuf,
    pub parent: PathBuf,
    pub target: PathBuf,
}
impl Fixture {
    pub fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("mado-p17-{}-{nonce}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let parent = root.join("owned-parent");
        fs::create_dir(&parent).unwrap();
        let target = parent.join("données-日本-𐐀");
        fs::create_dir(&target).unwrap();
        fs::write(root.join("source-sentinel"), b"unchanged source").unwrap();
        Self {
            root,
            parent,
            target,
        }
    }
    pub fn sentinel(&self) {
        assert_eq!(
            fs::read(self.root.join("source-sentinel")).unwrap(),
            b"unchanged source"
        );
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.sentinel();
        fs::remove_dir_all(&self.root).unwrap();
    }
}
pub(super) fn shared() -> Arc<Shared> {
    Arc::new(Shared {
        mailbox: Mutex::new(Mailbox {
            revision: 0,
            targets: vec![],
            changed: false,
            retries: BTreeSet::new(),
            reports: BTreeMap::new(),
            driver_errors: Vec::new(),
            omitted_driver_errors: 0,
            port: None,
        }),
        stop: AtomicBool::new(false),
        stopped: AtomicBool::new(false),
        wake_error: AtomicU32::new(0),
        wake_pending: AtomicBool::new(false),
    })
}
fn replace(service: &Service, revision: u64, targets: Vec<Target>) {
    let until = Instant::now() + Duration::from_secs(5);
    loop {
        match service.replace_targets(revision, targets.clone()) {
            Ok(()) => return,
            Err(Refusal::Busy) if Instant::now() < until => thread::yield_now(),
            e => panic!("replace: {e:?}"),
        }
    }
}
fn wait(service: &Service, predicate: impl Fn(&TargetReport) -> bool) -> TargetReport {
    let until = Instant::now() + Duration::from_secs(8);
    loop {
        if let Ok(report) = service.take_report() {
            assert!(
                report.driver_errors.is_empty() && report.omitted_driver_errors == 0,
                "driver failure: {:?}",
                report.driver_errors
            );
            if let Some(r) = report.targets.into_iter().find(&predicate) {
                return r;
            }
        }
        assert!(Instant::now() < until, "missing expected service report");
        thread::sleep(Duration::from_millis(10));
    }
}
fn stop(service: &mut Service) {
    let start = Instant::now();
    service.request_stop();
    assert!(start.elapsed() < Duration::from_millis(100));
    let until = Instant::now() + Duration::from_secs(8);
    while !service.join_after_cleanup().unwrap() {
        assert!(
            Instant::now() < until,
            "owned local provider cleanup did not finish"
        );
        thread::sleep(Duration::from_millis(10));
    }
    assert!(service.stopped());
}
#[test]
fn native_service_create_rename_delete_root_recreation_and_stop() {
    let fixture = Fixture::new();
    let mut service = Service::new().unwrap();
    replace(
        &service,
        1,
        vec![
            Target::new(7, 11, fixture.target.clone()),
            Target::new(8, 9, fixture.target.clone()),
        ],
    );
    wait(&service, |r| {
        r.target.id == 7 && r.ready && r.full_reconcile
    });
    let file = fixture.target.join("épreuve-𐐀.txt");
    fs::write(&file, b"owned contents").unwrap();
    wait(&service, |r| r.target.id == 7 && r.reconcile);
    let renamed = fixture.target.join("renamed");
    fs::rename(&file, &renamed).unwrap();
    wait(&service, |r| r.target.id == 8 && r.reconcile);
    fs::remove_file(&renamed).unwrap();
    wait(&service, |r| r.target.id == 7 && r.reconcile);
    let moved = fixture.parent.join("moved");
    fs::rename(&fixture.target, &moved).unwrap();
    let missing = wait(&service, |r| {
        r.target.id == 7 && !r.ready && r.directory_error.is_some()
    });
    assert!(missing.full_reconcile);
    fs::remove_dir(&moved).unwrap();
    fs::create_dir(&fixture.target).unwrap();
    wait(&service, |r| {
        r.target.id == 7 && r.ready && r.full_reconcile
    });
    fs::write(fixture.target.join("after-recreation"), b"preserved").unwrap();
    wait(&service, |r| r.target.id == 7 && r.reconcile);
    stop(&mut service);
    fixture.sentinel();
}
#[test]
fn bounded_coalesced_controls_stale_reports_and_nonblocking_lock_refusal() {
    let shared = shared();
    let service = Service {
        shared: shared.clone(),
        thread: None,
    };
    let targets: Vec<_> = (0..MAX_TARGETS)
        .map(|id| Target::new(id as u64, 1, PathBuf::from("C:\\owned\\child")))
        .collect();
    service.replace_targets(1, targets.clone()).unwrap();
    assert_eq!(
        service.replace_targets(
            2,
            (0..=MAX_TARGETS)
                .map(|id| Target::new(id as u64, 1, PathBuf::from("C:\\owned")))
                .collect()
        ),
        Err(Refusal::Capacity)
    );
    for id in 0..MAX_TARGETS {
        for _ in 0..100 {
            service.retry(1, id as u64, 1).unwrap();
        }
    }
    assert_eq!(shared.lock().retries.len(), MAX_TARGETS);
    let old = TargetReport {
        revision: 1,
        target: targets[0].clone(),
        reconcile: true,
        full_reconcile: true,
        overflow: true,
        ready: false,
        directory_error: Some(Failure::Capacity),
        parent_error: None,
    };
    shared.publish(old.clone());
    let mut fresh = targets.clone();
    fresh[0] = Target::new(0, 2, PathBuf::from("C:\\owned\\other"));
    service.replace_targets(2, fresh.clone()).unwrap();
    shared.publish(old);
    assert!(service.take_report().unwrap().targets.is_empty());
    assert_eq!(service.retry(2, 0, 1), Err(Refusal::StaleTarget));
    assert_eq!(
        service.replace_targets(2, vec![]),
        Err(Refusal::StaleRevision)
    );
    for revision in 3..200 {
        service.replace_targets(revision, fresh.clone()).unwrap();
    }
    assert_eq!(shared.lock().targets.len(), MAX_TARGETS);
    {
        let _guard = shared.lock();
        assert_eq!(service.replace_targets(200, vec![]), Err(Refusal::Busy));
        assert!(matches!(service.take_report(), Err(Refusal::Busy)));
        service.request_stop();
    }
    assert!(shared.stop.load(Ordering::Acquire));
    assert_eq!(service.replace_targets(201, vec![]), Err(Refusal::Stopping));
}
#[test]
fn report_dirty_merge_and_driver_failure_capacity_are_visible_and_sticky() {
    let shared = shared();
    let service = Service {
        shared: shared.clone(),
        thread: None,
    };
    let target = Target::new(1, 2, PathBuf::from("C:\\owned"));
    service.replace_targets(1, vec![target.clone()]).unwrap();
    let mut report = TargetReport {
        revision: 1,
        target,
        reconcile: true,
        full_reconcile: true,
        overflow: true,
        ready: true,
        directory_error: None,
        parent_error: None,
    };
    shared.publish(report.clone());
    report.reconcile = false;
    report.full_reconcile = false;
    report.ready = false;
    report.directory_error = Some(Failure::Native {
        action: "rearm",
        code: 5,
    });
    shared.publish(report);
    let output = service.take_report().unwrap().targets.remove(0);
    assert!(output.reconcile && output.full_reconcile && output.overflow && !output.ready);
    for code in 1..=10 {
        shared.failure(Failure::Native {
            action: "injected driver failure",
            code,
        });
    }
    let output = service.take_report().unwrap();
    assert_eq!(output.driver_errors.len(), 8);
    assert_eq!(output.omitted_driver_errors, 2);
    assert_eq!(
        service.take_report().unwrap().driver_errors,
        output.driver_errors
    );
}
#[test]
fn dirty_debounce_has_maximum_delay_and_merges_full_flag() {
    let start = Instant::now();
    let mut dirty = Dirty::default();
    dirty.mark(start, false);
    for ms in [100, 190, 300, 450, 600, 750] {
        dirty.mark(start + Duration::from_millis(ms), ms == 300);
        assert!(!dirty.due(start + Duration::from_millis(ms)));
    }
    assert!(dirty.due(start + MAX_DELAY));
    assert!(dirty.full);
    let mut quiet = Dirty::default();
    quiet.mark(start, false);
    assert!(!quiet.due(start + Duration::from_millis(199)));
    assert!(quiet.due(start + DEBOUNCE));
}
#[test]
fn namespace_admission_is_lossless_and_rejects_endpoints_without_provider_calls() {
    use std::{
        ffi::OsString,
        os::windows::ffi::{OsStrExt, OsStringExt},
    };
    for path in [
        "NUL",
        "C:\\NUL.txt",
        "\\\\.\\pipe\\name",
        "\\\\?\\GLOBALROOT\\Device\\disk",
        "\\\\host\\pipe\\name",
        "C:\\a\\..\\b",
        "C:\\a ",
        "C:\\a\0b",
    ] {
        assert!(
            path::admitted_name(std::path::Path::new(path)).is_err(),
            "{path:?}"
        );
    }
    let mut units: Vec<u16> = "C:\\owned\\".encode_utf16().collect();
    units.extend([0xd800, 0x0061]);
    let native = PathBuf::from(OsString::from_wide(&units));
    let admitted = path::admitted_name(&native).unwrap();
    assert_eq!(&admitted[4..admitted.len() - 1], &units);
    assert_eq!(native.as_os_str().encode_wide().collect::<Vec<_>>(), units);
    assert_eq!(
        path::admitted_name(std::path::Path::new("C:/owned/child")).unwrap(),
        path::admitted_name(std::path::Path::new("C:\\owned\\child")).unwrap()
    );
}
