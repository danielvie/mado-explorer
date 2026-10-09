use super::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(5);
fn retry<T>(mut call: impl FnMut() -> Result<T, Failure>) -> T {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        match call() {
            Err(Failure::Busy) if Instant::now() < deadline => thread::yield_now(),
            result => return result.unwrap(),
        }
    }
}
fn response(service: &Service, target: Target) -> Response {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if let Some(response) = retry(|| service.take(target)) {
            return response;
        }
        assert!(Instant::now() < deadline, "response timeout");
        thread::yield_now();
    }
}
fn request() -> Request {
    Request {
        page_index: 0,
        width: 2,
        height: 2,
    }
}
fn page(path: &Path, request: RenderRequest) -> RenderedPage {
    let len = request.width as usize * request.height as usize * 4;
    let mut rgba = Vec::new();
    rgba.try_reserve_exact(len).unwrap();
    rgba.resize(len, 7);
    RenderedPage {
        document: pdf::DocumentMetadata {
            path: path.to_owned(),
            input_bytes: 4,
            page_count: request.page_index + 1,
            pdf_version: Some(17),
        },
        page: pdf::PageMetadata {
            index: request.page_index,
            width_points: 100.0,
            height_points: 100.0,
        },
        width: request.width,
        height: request.height,
        rgba,
    }
}
struct Controlled {
    service: Service,
    entered: Receiver<(PathBuf, RenderRequest, thread::ThreadId)>,
    release: Sender<Result<(), Failure>>,
    finished: Receiver<()>,
}
impl Controlled {
    fn new(budget: CpuBudget) -> Self {
        let (entered_tx, entered) = mpsc::channel();
        let (release, release_rx) = mpsc::channel();
        let (finished_tx, finished) = mpsc::channel();
        let service = Service::start_inner(budget, move || {
            struct Finish(Sender<()>);
            impl Drop for Finish {
                fn drop(&mut self) {
                    let _ = self.0.send(());
                }
            }
            let finish = Finish(finished_tx);
            Box::new(move |path, request| {
                std::hint::black_box(&finish);
                entered_tx
                    .send((path.to_owned(), request, thread::current().id()))
                    .unwrap();
                release_rx.recv_timeout(TIMEOUT).unwrap()?;
                Ok(page(path, request))
            })
        })
        .unwrap();
        Self {
            service,
            entered,
            release,
            finished,
        }
    }
    fn target(&self) -> Target {
        retry(|| self.service.open_target())
    }
    fn submit(&self, target: Target, path: &Path) -> Generation {
        retry(|| self.service.request(target, path, request()))
    }
    fn enter(&self) -> (PathBuf, RenderRequest, thread::ThreadId) {
        self.entered.recv_timeout(TIMEOUT).unwrap()
    }
    fn success(&self) {
        self.release.send(Ok(())).unwrap();
    }
}

#[test]
fn coalesced_fifo_is_fair_to_independent_targets() {
    let c = Controlled::new(CpuBudget::new());
    let a = c.target();
    let b = c.target();
    let d = c.target();
    c.submit(a, Path::new("active-a"));
    assert_eq!(c.enter().0, Path::new("active-a"));
    c.submit(b, Path::new("b-old"));
    c.submit(a, Path::new("a-old"));
    c.submit(d, Path::new("d"));
    let b_latest = c.submit(b, Path::new("b-latest"));
    let a_latest = c.submit(a, Path::new("a-latest"));
    c.success();
    assert_eq!(c.enter().0, Path::new("b-latest"));
    assert!(retry(|| c.service.take(a)).is_none());
    c.success();
    assert_eq!(c.enter().0, Path::new("a-latest"));
    assert_eq!(response(&c.service, b).generation, b_latest);
    c.success();
    assert_eq!(c.enter().0, Path::new("d"));
    assert_eq!(response(&c.service, a).generation, a_latest);
    c.success();
    assert!(response(&c.service, d).result.is_ok());
}

#[test]
fn completed_and_pending_results_become_stale_on_new_request() {
    let budget = CpuBudget::new();
    let c = Controlled::new(budget.clone());
    let t = c.target();
    let old = c.submit(t, Path::new("old"));
    c.enter();
    c.success();
    let held = response(&c.service, t);
    assert_eq!(budget.used(), 16);
    let next = c.submit(t, Path::new("next"));
    c.enter();
    assert_ne!(old, next);
    assert!(!retry(|| c.service.is_current(t, held.generation)));
    assert!(retry(|| c.service.is_current(t, next)));
    let newest = c.submit(t, Path::new("newest"));
    c.success();
    assert_eq!(c.enter().0, Path::new("newest"));
    assert!(retry(|| c.service.take(t)).is_none());
    c.success();
    assert_eq!(response(&c.service, t).generation, newest);
    drop(held);
    assert_eq!(budget.used(), 0);
}

#[test]
fn ready_mailbox_is_released_before_replacement() {
    let budget = CpuBudget::new();
    let c = Controlled::new(budget.clone());
    let t = c.target();
    c.submit(t, Path::new("first"));
    c.enter();
    c.success();
    let deadline = Instant::now() + TIMEOUT;
    loop {
        let state = c.service.shared.state.lock().unwrap();
        if state.slots[0].ready.is_some() {
            break;
        }
        drop(state);
        assert!(Instant::now() < deadline);
        thread::yield_now();
    }
    assert_eq!(budget.used(), 16);
    c.submit(t, Path::new("second"));
    c.enter();
    assert_eq!(budget.used(), SOURCE_RESERVATION + 16);
    c.success();
    drop(response(&c.service, t));
    assert_eq!(budget.used(), 0);
}

#[test]
fn stale_error_does_not_replace_current_error_or_other_target() {
    let budget = CpuBudget::new();
    let c = Controlled::new(budget.clone());
    let a = c.target();
    let b = c.target();
    c.submit(a, Path::new("bad-old"));
    c.enter();
    let good = c.submit(a, Path::new("good"));
    let bad = c.submit(b, Path::new("bad-current"));
    c.release
        .send(Err(Failure::Backend {
            state: BackendState::MalformedDocument,
            message: "old error".into(),
        }))
        .unwrap();
    assert_eq!(c.enter().0, Path::new("good"));
    assert!(retry(|| c.service.take(a)).is_none());
    c.success();
    assert_eq!(c.enter().0, Path::new("bad-current"));
    let frame = response(&c.service, a);
    assert_eq!(frame.generation, good);
    c.release
        .send(Err(Failure::Backend {
            state: BackendState::PasswordRequired,
            message: "password".into(),
        }))
        .unwrap();
    let error = response(&c.service, b);
    assert_eq!(error.generation, bad);
    assert!(matches!(
        error.result,
        Err(Failure::Backend {
            state: BackendState::PasswordRequired,
            ..
        })
    ));
    assert_eq!(budget.used(), 16);
    drop(frame);
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancel_pending_active_and_closed_targets_do_not_retarget() {
    let budget = CpuBudget::new();
    let c = Controlled::new(budget.clone());
    let a = c.target();
    let b = c.target();
    let d = c.target();
    let old = c.submit(a, Path::new("active"));
    c.enter();
    c.submit(b, Path::new("pending-cancel"));
    c.submit(d, Path::new("pending-close"));
    retry(|| c.service.cancel(b));
    retry(|| c.service.close_target(d));
    retry(|| c.service.cancel(a));
    assert!(!retry(|| c.service.is_current(a, old)));
    let replacement = c.target();
    assert_ne!(replacement, d);
    c.submit(replacement, Path::new("replacement"));
    c.success();
    assert_eq!(c.enter().0, Path::new("replacement"));
    assert!(retry(|| c.service.take(a)).is_none());
    assert!(retry(|| c.service.take(b)).is_none());
    assert_eq!(c.service.take(d).unwrap_err(), Failure::UnknownTarget);
    c.success();
    drop(response(&c.service, replacement));
    assert_eq!(budget.used(), 0);
}

#[test]
fn closed_active_result_is_freed_while_other_target_survives() {
    let budget = CpuBudget::new();
    let c = Controlled::new(budget.clone());
    let a = c.target();
    let b = c.target();
    c.submit(a, Path::new("closed"));
    c.enter();
    c.submit(b, Path::new("survivor"));
    retry(|| c.service.close_target(a));
    c.success();
    assert_eq!(c.enter().0, Path::new("survivor"));
    assert_eq!(budget.used(), SOURCE_RESERVATION + 16);
    c.success();
    drop(response(&c.service, b));
    assert_eq!(budget.used(), 0);
}

#[test]
fn native_path_capture_and_worker_only_factory_and_calls() {
    let ui_thread = thread::current().id();
    let (init_tx, init_rx) = mpsc::channel();
    let (call_tx, call_rx) = mpsc::channel();
    let mut service = Service::start_inner(CpuBudget::new(), move || {
        init_tx.send(thread::current().id()).unwrap();
        Box::new(move |path, request| {
            call_tx.send(thread::current().id()).unwrap();
            Ok(page(path, request))
        })
    })
    .unwrap();
    let worker_thread = init_rx.recv_timeout(TIMEOUT).unwrap();
    assert_ne!(worker_thread, ui_thread);
    #[cfg(windows)]
    let mut path = {
        use std::os::windows::ffi::OsStringExt;
        PathBuf::from(std::ffi::OsString::from_wide(&[
            67, 58, 92, 0xD800, 0x00E9, 92, 112, 46, 112, 100, 102,
        ]))
    };
    #[cfg(not(windows))]
    let mut path = PathBuf::from("C:/é/𐐀.pdf");
    let captured = path.clone();
    let t = retry(|| service.open_target());
    let generation = retry(|| service.request(t, &path, request()));
    path.set_file_name("changed.pdf");
    let result = response(&service, t);
    assert_eq!(result.path, captured);
    let frame = result.result.unwrap();
    assert_eq!(frame.document.path, captured);
    assert_eq!(result.generation, generation);
    assert_eq!(call_rx.recv_timeout(TIMEOUT).unwrap(), worker_thread);
    service.join().unwrap();
}

#[test]
fn ui_methods_never_wait_for_worker_or_locked_state() {
    let c = Controlled::new(CpuBudget::new());
    let t = c.target();
    c.submit(t, Path::new("blocked-native"));
    c.enter();
    // These complete while renderer is deliberately blocked.
    assert!(retry(|| c.service.take(t)).is_none());
    retry(|| c.service.cancel(t));
    let guard = c.service.shared.state.lock().unwrap();
    assert_eq!(c.service.open_target().unwrap_err(), Failure::Busy);
    assert_eq!(
        c.service.request(t, Path::new("x"), request()).unwrap_err(),
        Failure::Busy
    );
    assert_eq!(c.service.take(t).unwrap_err(), Failure::Busy);
    assert_eq!(c.service.cancel(t).unwrap_err(), Failure::Busy);
    assert_eq!(c.service.close_target(t).unwrap_err(), Failure::Busy);
    // Stop cannot wait for even this state lock.
    c.service.request_stop();
    drop(guard);
    c.success();
}

#[test]
fn stop_retains_handle_until_actual_finish_and_drops_every_reservation() {
    let budget = CpuBudget::new();
    let mut c = Controlled::new(budget.clone());
    let a = c.target();
    let b = c.target();
    c.submit(a, Path::new("active"));
    c.enter();
    c.submit(b, Path::new("never-run"));
    c.service.request_stop();
    assert!(c.service.try_join().is_none());
    assert!(c.service.worker.is_some());
    assert_eq!(c.service.take(a).unwrap_err(), Failure::Stopped);
    assert_eq!(budget.used(), SOURCE_RESERVATION + 16);
    c.success();
    c.finished.recv_timeout(TIMEOUT).unwrap();
    c.service.join().unwrap();
    assert!(c.service.worker.is_none());
    assert_eq!(budget.used(), 0);
    assert!(c.entered.try_recv().is_err());
}

#[test]
fn held_frames_plus_inflight_charge_and_budget_refusal_reuse() {
    let budget = CpuBudget::new();
    let c = Controlled::new(budget.clone());
    let a = c.target();
    let b = c.target();
    c.submit(a, Path::new("held"));
    c.enter();
    c.success();
    let held = response(&c.service, a);
    let charge = held.result.as_ref().unwrap().charged_bytes();
    assert_eq!(charge, 16);
    c.submit(b, Path::new("inflight"));
    c.enter();
    assert_eq!(budget.used(), charge + SOURCE_RESERVATION + 16);
    // Reserve logical space without allocating a huge fixture. The service cannot
    // enter the renderer when the source/output admission would exceed 256 MiB.
    let filler = budget.reserve(CPU_BUFFER_LIMIT - budget.used()).unwrap();
    assert_eq!(budget.used(), CPU_BUFFER_LIMIT);
    c.success();
    let second = response(&c.service, b);
    let remaining = CPU_BUFFER_LIMIT - SOURCE_RESERVATION;
    assert_eq!(budget.used(), remaining);
    let next = c.submit(b, Path::new("refused"));
    let refusal = response(&c.service, b);
    assert_eq!(refusal.generation, next);
    assert_eq!(refusal.result.unwrap_err(), Failure::BudgetExceeded);
    assert!(c.entered.try_recv().is_err());
    drop(filler);
    drop(second);
    drop(held);
    assert_eq!(budget.used(), 0);
    c.submit(a, Path::new("reused"));
    c.enter();
    c.success();
    drop(response(&c.service, a));
    assert_eq!(budget.used(), 0);
}

#[test]
fn concrete_caller_copy_and_aggregate_lease_drop_are_exact() {
    let budget = CpuBudget::new();
    let mut copy = budget.allocate(64).unwrap();
    copy.bytes_mut()[0] = 9;
    assert_eq!(copy.bytes()[0], 9);
    assert_eq!(copy.capacity(), 64);
    assert_eq!(budget.used(), 64);
    let all = budget.reserve(CPU_BUFFER_LIMIT - 64).unwrap();
    assert_eq!(budget.allocate(1).unwrap_err(), Failure::BudgetExceeded);
    assert_eq!(
        budget.reserve(usize::MAX).unwrap_err(),
        Failure::BudgetExceeded
    );
    drop(copy);
    assert_eq!(budget.used(), CPU_BUFFER_LIMIT - 64);
    let again = budget.allocate(64).unwrap();
    assert_eq!(budget.used(), CPU_BUFFER_LIMIT);
    drop(all);
    drop(again);
    assert_eq!(budget.used(), 0);
    for _ in 0..100 {
        let a = budget.allocate(5).unwrap();
        let b = budget.allocate(7).unwrap();
        assert_eq!(budget.used(), 12);
        drop(a);
        drop(b);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn metadata_request_and_target_limits_refuse_without_new_current() {
    let c = Controlled::new(CpuBudget::new());
    let t = c.target();
    let current = c.submit(t, Path::new("valid"));
    c.enter();
    for invalid in [
        Request {
            width: 0,
            ..request()
        },
        Request {
            height: 8193,
            ..request()
        },
        Request {
            page_index: u32::MAX,
            ..request()
        },
    ] {
        assert_eq!(
            c.service.request(t, Path::new("x"), invalid).unwrap_err(),
            Failure::InvalidRequest
        );
    }
    assert_eq!(
        c.service
            .request(
                t,
                Path::new("x"),
                Request {
                    width: 8192,
                    height: 8192,
                    ..request()
                }
            )
            .unwrap_err(),
        Failure::OutputTooLarge
    );
    let huge_path = PathBuf::from("x".repeat(MAX_PATH_BYTES + 1));
    assert_eq!(
        c.service.request(t, &huge_path, request()).unwrap_err(),
        Failure::PathTooLong
    );
    assert!(retry(|| c.service.is_current(t, current)));
    for _ in 1..MAX_TARGETS {
        c.target();
    }
    assert_eq!(
        retry_error(|| c.service.open_target()),
        Failure::TargetLimit
    );
    assert!(c.service.shared.state.lock().unwrap().slots.len() <= MAX_TARGETS);
    c.success();
    drop(response(&c.service, t));
}
fn retry_error<T: fmt::Debug>(mut call: impl FnMut() -> Result<T, Failure>) -> Failure {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        match call() {
            Err(Failure::Busy) if Instant::now() < deadline => thread::yield_now(),
            result => return result.unwrap_err(),
        }
    }
}

#[test]
fn error_metadata_is_bounded_and_states_remain_distinct() {
    for (error, expected) in [
        (pdf::Error::PasswordRequired, BackendState::PasswordRequired),
        (
            pdf::Error::MalformedDocument,
            BackendState::MalformedDocument,
        ),
        (pdf::Error::InputTooLarge, BackendState::ResourceLimit),
        (
            pdf::Error::RendererUnavailable {
                library: PathBuf::from("trusted"),
                reason: "é".repeat(10_000),
            },
            BackendState::RendererUnavailable,
        ),
    ] {
        let Failure::Backend { state, message } = backend_error(error) else {
            panic!()
        };
        assert_eq!(state, expected);
        assert!(message.len() <= MAX_ERROR_BYTES);
        assert!(message.capacity() <= MAX_ERROR_BYTES);
    }
}

#[test]
fn backend_buffer_contract_failure_releases_admission() {
    let budget = CpuBudget::new();
    let mut service = Service::start_inner(budget.clone(), || {
        Box::new(|path, request| {
            let mut rendered = page(path, request);
            rendered.rgba.pop();
            Ok(rendered)
        })
    })
    .unwrap();
    let t = retry(|| service.open_target());
    retry(|| service.request(t, Path::new("generated"), request()));
    assert_eq!(
        response(&service, t).result.unwrap_err(),
        Failure::BufferContract
    );
    assert_eq!(budget.used(), 0);
    service.join().unwrap();
}

#[test]
fn capacity_not_length_is_charged_until_frame_drop() {
    let budget = CpuBudget::new();
    let mut service = Service::start_inner(budget.clone(), || {
        Box::new(|path, request| {
            let mut rendered = page(path, request);
            rendered.width = 1;
            rendered.height = 1;
            rendered.rgba.truncate(4);
            Ok(rendered)
        })
    })
    .unwrap();
    let target = retry(|| service.open_target());
    retry(|| service.request(target, Path::new("owned"), request()));
    let frame = response(&service, target).result.unwrap();
    assert_eq!(frame.pixels().len(), 4);
    assert_eq!(frame.charged_bytes(), 16);
    assert_eq!(budget.used(), 16);
    service.join().unwrap();
    assert_eq!(budget.used(), 16);
    drop(frame);
    assert_eq!(budget.used(), 0);
}

#[test]
fn queues_mailboxes_and_recreated_service_ids_stay_bounded() {
    let budget = CpuBudget::new();
    let mut c = Controlled::new(budget.clone());
    let a = c.target();
    c.submit(a, Path::new("active"));
    c.enter();
    let mut targets = vec![a];
    for i in 1..MAX_TARGETS {
        let t = c.target();
        c.submit(t, Path::new(&format!("target-{i}")));
        targets.push(t);
    }
    let mut last = None;
    for _ in 0..100 {
        let current = c.submit(a, Path::new("latest-a"));
        assert_ne!(last, Some(current));
        last = Some(current);
        let state = c.service.shared.state.lock().unwrap();
        assert_eq!(state.queue.len(), MAX_TARGETS);
        assert_eq!(state.slots.len(), MAX_TARGETS);
    }
    for target in targets {
        retry(|| c.service.cancel(target));
    }
    assert!(c.service.shared.state.lock().unwrap().queue.is_empty());
    c.service.request_stop();
    c.success();
    c.service.join().unwrap();
    let mut replacement = Service::start_inner(budget.clone(), || {
        Box::new(|path, request| Ok(page(path, request)))
    })
    .unwrap();
    assert_eq!(
        retry_error(|| replacement.request(a, Path::new("foreign"), request())),
        Failure::UnknownTarget
    );
    let b = retry(|| replacement.open_target());
    assert_ne!(a, b);
    let generation = retry(|| replacement.request(b, Path::new("fresh"), request()));
    assert_ne!(last, Some(generation));
    drop(response(&replacement, b));
    replacement.join().unwrap();
    assert_eq!(budget.used(), 0);
}

#[test]
fn idle_stop_joins_without_a_request_or_detach() {
    let mut service =
        Service::start_inner(CpuBudget::new(), || Box::new(|_, _| panic!("no request"))).unwrap();
    service.request_stop();
    service.join().unwrap();
    assert!(service.worker.is_none());
}

#[test]
fn production_renderer_path_reports_missing_library_without_ui_io() {
    // Own generated directory only. No native DLL, personal path or download.
    let owned = std::env::temp_dir().join(format!(
        "mado-p11-{}-{}",
        std::process::id(),
        fresh().unwrap()
    ));
    std::fs::create_dir(&owned).unwrap();
    struct Fixture(PathBuf);
    impl Drop for Fixture {
        fn drop(&mut self) {
            std::fs::remove_dir(&self.0).unwrap();
        }
    }
    let fixture = Fixture(owned);
    let budget = CpuBudget::new();
    let mut service =
        Service::start(&fixture.0.join("missing-pdfium.dll"), budget.clone()).unwrap();
    let t = retry(|| service.open_target());
    retry(|| service.request(t, &fixture.0.join("owned.pdf"), request()));
    assert!(matches!(
        response(&service, t).result,
        Err(Failure::Backend {
            state: BackendState::RendererUnavailable,
            ..
        })
    ));
    assert_eq!(budget.used(), 0);
    service.join().unwrap();
}
