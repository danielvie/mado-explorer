use super::*;
use crate::tests::{Fixture, shared};
use std::fs;

struct Harness(Driver);
impl Harness {
    fn new(targets: Vec<Target>) -> Self {
        let shared = shared();
        {
            let mut m = shared.lock();
            m.revision = 1;
            m.targets = targets;
            m.changed = true;
        }
        let port = Arc::new(Port::new().unwrap());
        Self(Driver::new(shared, port))
    }
    fn ready(&mut self) {
        self.0.controls();
        assert!(self.0.requests.values().all(|r| r.pending && !r.retiring));
    }
    fn until(&mut self, predicate: impl Fn(&Driver) -> bool) {
        let until = Instant::now() + Duration::from_secs(5);
        while !predicate(&self.0) {
            assert!(
                Instant::now() < until,
                "native harness timeout; attachments={:?}; requests={:?}",
                self.0
                    .attachments
                    .values()
                    .map(|a| (&a.request, &a.error))
                    .collect::<Vec<_>>(),
                self.0
                    .requests
                    .iter()
                    .map(|(id, r)| (id, r.pending, r.retiring))
                    .collect::<Vec<_>>()
            );
            self.0.dequeue(25);
        }
    }
}
impl Drop for Harness {
    fn drop(&mut self) {
        self.0.cleanup();
        assert!(self.0.requests.is_empty());
    }
}

#[test]
fn injected_completion_contract_zero_enum_unknown_abort_and_retired_success() {
    assert_eq!(
        classify(false, 0, 64),
        Completion::Dirty { overflow: false }
    );
    assert_eq!(classify(false, 0, 0), Completion::Dirty { overflow: true });
    assert_eq!(
        classify(false, ERROR_NOTIFY_ENUM_DIR, 0),
        Completion::Dirty { overflow: true }
    );
    assert_eq!(classify(false, 9999, 64), Completion::Failed(9999));
    assert_eq!(
        classify(false, ERROR_OPERATION_ABORTED, 0),
        Completion::Failed(ERROR_OPERATION_ABORTED)
    );
    assert_eq!(classify(true, 0, 64), Completion::Retired);
    assert_eq!(
        classify(true, ERROR_OPERATION_ABORTED, 0),
        Completion::Retired
    );
}
#[test]
fn injected_initial_read_failure_is_not_ready_and_retry_registers() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.0.faults.insert("initial read", ERROR_FILE_NOT_FOUND);
    h.0.controls();
    let reports: Vec<_> = h.0.shared.lock().reports.values().cloned().collect();
    assert!(!reports[0].ready);
    assert!(reports[0].directory_error.is_some() || reports[0].parent_error.is_some());
    h.0.revalidate(false);
    h.0.publish_all(true);
    assert!(
        !h.0.shared.lock().reports[&1].ready,
        "initial read errors are sticky even with a missing-name code"
    );
    h.0.shared.lock().retries.insert(1);
    h.0.controls();
    assert!(h.0.shared.lock().reports[&1].ready);
    assert_eq!(h.0.requests.len(), 2);
}
#[test]
fn injected_initial_enum_error_keeps_warning_full_reconcile_and_unavailable_independent() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.0.faults.insert("initial read", ERROR_NOTIFY_ENUM_DIR);
    h.0.controls();
    let report = h.0.shared.lock().reports[&1].clone();
    assert!(report.overflow && report.reconcile && report.full_reconcile && !report.ready);
    assert!(report.directory_error.is_some() || report.parent_error.is_some());
}
#[test]
fn injected_rearm_failure_keeps_dirty_data_and_is_sticky_until_retry() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.ready();
    h.0.shared.lock().reports.clear();
    h.0.faults.insert("rearm", ERROR_FILE_NOT_FOUND);
    fs::write(fixture.target.join("trigger"), b"").unwrap();
    h.until(|d| d.attachments.values().any(|a| a.error.is_some()));
    let report = h.0.shared.lock().reports[&1].clone();
    assert!(report.reconcile && report.full_reconcile && !report.ready);
    assert!(matches!(
        report.directory_error,
        Some(Failure::Native {
            code: ERROR_FILE_NOT_FOUND,
            ..
        })
    ));
    h.0.revalidate(false);
    assert!(h.0.attachments.values().any(|a| matches!(
        a.error,
        Some(Failure::Native {
            code: ERROR_FILE_NOT_FOUND,
            ..
        })
    )));
    h.0.shared.lock().retries.insert(1);
    h.0.controls();
    assert!(h.0.shared.lock().reports[&1].ready);
}
#[test]
fn injected_zero_enum_unknown_and_abort_reach_sticky_reports_after_real_drain() {
    for code in [
        0,
        ERROR_NOTIFY_ENUM_DIR,
        9999,
        ERROR_OPERATION_ABORTED,
        ERROR_ACCESS_DENIED,
    ] {
        let fixture = Fixture::new();
        let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
        h.ready();
        h.0.shared.lock().reports.clear();
        h.0.faults.insert("completion code", code);
        h.0.faults.insert("completion bytes", 0);
        if code == ERROR_NOTIFY_ENUM_DIR {
            h.0.faults.insert("rearm", 4567);
        }
        fs::write(fixture.target.join("real-completion"), b"").unwrap();
        h.until(|d| !d.faults.contains_key("completion code"));
        h.0.publish_all(true);
        let report = h.0.shared.lock().reports[&1].clone();
        assert!(report.reconcile && report.full_reconcile);
        if code == 0 || code == ERROR_NOTIFY_ENUM_DIR {
            assert!(report.overflow);
            if code == ERROR_NOTIFY_ENUM_DIR {
                assert!(!report.ready);
                assert!(matches!(
                    report.directory_error,
                    Some(Failure::Native { code: 4567, .. })
                ));
            } else {
                assert!(report.ready);
            }
        } else {
            assert!(!report.ready);
            assert_eq!(
                report.directory_error,
                Some(Failure::Native {
                    action: "notification completion",
                    code
                })
            );
            h.0.revalidate(false);
            h.0.publish_all(true);
            assert!(!h.0.shared.lock().reports[&1].ready);
            h.0.shared.lock().retries.insert(1);
            h.0.controls();
            assert!(h.0.shared.lock().reports[&1].ready);
        }
        if report.overflow {
            {
                let mut m = h.0.shared.lock();
                m.revision = 2;
                m.changed = true;
            }
            h.0.controls();
            assert!(
                h.0.shared.lock().reports[&1].overflow,
                "revision changes must not silently discard warnings"
            );
        }
    }
}
#[test]
fn queued_success_after_cancel_never_rearms_and_retirement_owns_buffers_until_drain() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.ready();
    let key = path::admitted_name(&fixture.target).unwrap();
    let id = h.0.attachments[&key].request.unwrap();
    let address = h.0.requests[&id].storage.get() as usize;
    fs::write(fixture.target.join("queued-success"), b"").unwrap();
    thread::sleep(Duration::from_millis(50));
    h.0.attachments.get_mut(&key).unwrap().request = None;
    h.0.retire(id);
    assert!(h.0.requests[&id].pending && h.0.requests[&id].retiring);
    assert_eq!(h.0.requests[&id].storage.get() as usize, address);
    h.until(|d| !d.requests.contains_key(&id));
    let (completed_id, code, bytes) = h.0.last_native_completion.unwrap();
    assert_eq!((completed_id, code), (id, 0));
    assert!(bytes > 0, "expected actual queued native success");
    assert!(!h.0.attachments[&key].request.is_some());
}
#[test]
fn repeated_controls_queue_only_one_native_wake_packet() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.ready();
    h.0.shared.lock().port = Some(h.0.port.clone());
    let service = Service {
        shared: h.0.shared.clone(),
        thread: None,
    };
    for revision in 2..1000 {
        service
            .replace_targets(revision, vec![Target::new(1, 1, fixture.target.clone())])
            .unwrap();
    }
    assert!(h.0.shared.wake_pending.load(Ordering::Acquire));
    h.0.dequeue(0);
    assert!(!h.0.shared.wake_pending.load(Ordering::Acquire));
    let mut bytes = 0;
    let mut key = 0;
    let mut ov = ptr::null_mut();
    let ok = unsafe {
        GetQueuedCompletionStatus(h.0.port.0.as_raw_handle(), &mut bytes, &mut key, &mut ov, 0)
    };
    assert_eq!(ok, 0);
    assert_eq!(unsafe { GetLastError() }, WAIT_TIMEOUT);
    assert!(ov.is_null());
}
#[test]
fn unwinding_preserves_pending_storage_until_explicit_cleanup_drains_it() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.ready();
    let addresses: Vec<_> =
        h.0.requests
            .iter()
            .map(|(id, r)| (*id, r.storage.get() as usize))
            .collect();
    let result = catch_unwind(AssertUnwindSafe(|| {
        panic!("injected driver unwind");
    }));
    assert!(result.is_err());
    for (id, address) in addresses {
        assert!(h.0.requests[&id].pending);
        assert_eq!(h.0.requests[&id].storage.get() as usize, address);
    }
    h.0.cleanup();
    assert!(h.0.requests.is_empty());
}
#[test]
fn delayed_retirement_capacity_counts_all_requests_and_deduplicates_paths() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![
        Target::new(1, 1, fixture.target.clone()),
        Target::new(2, 1, fixture.target.clone()),
    ]);
    h.ready();
    assert_eq!(h.0.requests.len(), 2);
    // Hold every completion in the port. Replacements accumulate retirees, not new threads.
    for rev in 2..=80 {
        let path = fixture.parent.join(format!("child-{rev}"));
        fs::create_dir(&path).unwrap();
        {
            let mut m = h.0.shared.lock();
            m.revision = rev;
            m.targets = vec![Target::new(1, rev, path)];
            m.changed = true;
        }
        h.0.controls();
        assert!(h.0.requests.len() <= MAX_REQUESTS);
        assert!(h.0.attachments.len() <= 2);
    }
    assert_eq!(h.0.requests.len(), MAX_REQUESTS);
    let report = h.0.shared.lock().reports[&1].clone();
    assert!(!report.ready && report.directory_error == Some(Failure::Capacity));
    assert!(h.0.requests.values().any(|r| r.retiring && r.pending));
    h.until(|d| d.requests.values().all(|r| !r.retiring));
    // Pressure is not silently cleared after drain. Explicit retry is required.
    assert_eq!(
        h.0.shared.lock().reports[&1].directory_error,
        Some(Failure::Capacity)
    );
    h.0.shared.lock().retries.insert(1);
    h.0.controls();
    assert!(h.0.shared.lock().reports[&1].ready);
}
#[test]
fn actual_native_overflow_exposes_warning_and_full_reconcile() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.ready();
    h.0.shared.lock().reports.clear();
    // Driver is intentionally not draining here. The pending 16 KiB read completes and
    // the provider's bounded native change buffer overflows before the next rearm.
    for i in 0..800 {
        fs::write(
            fixture.target.join(format!("{i:04}-{}", "x".repeat(120))),
            b"",
        )
        .unwrap();
    }
    h.until(|d| d.current[&1].overflow);
    h.0.publish_all(true);
    let report = h.0.shared.lock().reports[&1].clone();
    assert!(report.overflow && report.reconcile && report.full_reconcile);
    assert!(report.ready && report.directory_error.is_none());
    let (_, code, bytes) = h.0.last_native_completion.unwrap();
    assert!(code == ERROR_NOTIFY_ENUM_DIR || (code == 0 && bytes == 0));
    println!(
        "actual native overflow code={code}, bytes={bytes}; warning and full reconciliation preserved"
    );
}
#[test]
fn native_long_unpaired_names_and_equivalent_spelling_dedup_preserve_original_paths() {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    let fixture = Fixture::new();
    let mut long = fixture.parent.clone();
    for i in 0..12 {
        long = long.join(format!("{i:02}-{}", "a".repeat(24)));
        fs::create_dir(&long).unwrap();
    }
    let name = OsString::from_wide(&[0x006e, 0xd800, 0x0061]);
    long = long.join(name);
    fs::create_dir(&long).unwrap();
    let admitted = path::admitted_name(&long).unwrap();
    let verbatim = PathBuf::from(OsString::from_wide(&admitted[..admitted.len() - 1]));
    let mut h = Harness::new(vec![
        Target::new(1, 1, long.clone()),
        Target::new(2, 1, verbatim.clone()),
    ]);
    h.ready();
    assert_eq!(h.0.requests.len(), 2);
    assert!(h.0.shared.lock().reports[&1].ready && h.0.shared.lock().reports[&2].ready);
    assert_eq!(h.0.shared.lock().reports[&1].target.path(), &long);
    assert_eq!(h.0.shared.lock().reports[&2].target.path(), &verbatim);
    fs::write(long.join("new-file"), b"owned").unwrap();
    h.until(|d| d.current[&1].dirty.first.is_some() && d.current[&2].dirty.first.is_some());
    h.0.publish_all(true);
    assert!(h.0.shared.lock().reports[&1].reconcile && h.0.shared.lock().reports[&2].reconcile);
}
#[test]
fn initially_absent_target_and_direct_root_delete_recreate_reattach_from_parent() {
    let fixture = Fixture::new();
    fs::remove_dir(&fixture.target).unwrap();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.0.controls();
    let key = path::admitted_name(&fixture.target).unwrap();
    assert!(!h.0.shared.lock().reports[&1].ready);
    assert!(h.0.shared.lock().reports[&1].directory_error.is_some());
    assert!(h.0.shared.lock().reports[&1].parent_error.is_none());
    fs::create_dir(&fixture.target).unwrap();
    h.until(|d| d.attachments[&key].request.is_some());
    fs::remove_dir(&fixture.target).unwrap();
    h.until(|d| d.attachments[&key].request.is_none());
    h.until(|d| d.requests.values().all(|r| r.key != key));
    h.0.publish_all(true);
    assert!(!h.0.shared.lock().reports[&1].ready);
    fs::create_dir(&fixture.target).unwrap();
    h.until(|d| d.attachments[&key].request.is_some());
    h.0.publish_all(true);
    assert!(h.0.shared.lock().reports[&1].ready);
    assert!(h.0.shared.lock().reports[&1].full_reconcile);
}
#[test]
fn lost_parent_is_unavailable_and_manual_retry_recovers() {
    let fixture = Fixture::new();
    let mut h = Harness::new(vec![Target::new(1, 1, fixture.target.clone())]);
    h.ready();
    fs::remove_dir(&fixture.target).unwrap();
    let target_key = path::admitted_name(&fixture.target).unwrap();
    h.until(|d| d.attachments[&target_key].request.is_none());
    h.until(|d| d.requests.values().all(|r| r.key != target_key));
    fs::remove_dir(&fixture.parent).unwrap();
    let parent_key = path::admitted_name(&fixture.parent).unwrap();
    h.until(|d| d.attachments[&parent_key].error.is_some());
    h.0.publish_all(true);
    assert!(!h.0.shared.lock().reports[&1].ready);
    assert!(h.0.shared.lock().reports[&1].parent_error.is_some());
    h.until(|d| d.requests.is_empty());
    fs::create_dir(&fixture.parent).unwrap();
    fs::create_dir(&fixture.target).unwrap();
    h.0.shared.lock().retries.insert(1);
    h.0.controls();
    assert!(h.0.shared.lock().reports[&1].ready);
}
