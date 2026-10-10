// Native tests serialize the single-window backend; fixtures remain in target/p21.
use super::*;
use image::{
    ExtendedColorType, ImageEncoder,
    codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
};
use std::{
    ffi::{OsStr, OsString},
    fs,
    hash::{Hash, Hasher},
    os::windows::ffi::OsStringExt,
    sync::atomic::AtomicUsize,
    time::{Duration, Instant},
};
pub(super) static SERIAL: Mutex<()> = Mutex::new(());
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/p21/fixtures")
            .join(format!(
                "owned-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                id().unwrap()
            ));
        fs::create_dir_all(root.parent().unwrap()).unwrap();
        fs::create_dir(&root).unwrap(); // Never adopt an existing fixture directory.
        // Generated path construction contains ../..; canonicalize only this
        // owned fixture so the strict source namespace guard stays unchanged.
        Self(fs::canonicalize(root).unwrap())
    }
    fn write(&self, name: impl AsRef<OsStr>, bytes: &[u8]) -> PathBuf {
        let p = self.0.join(Path::new(name.as_ref()));
        fs::write(&p, bytes).unwrap();
        p
    }
}
fn hash(bytes: &[u8]) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut h);
    h.finish()
}
fn exif(o: u8) -> Vec<u8> {
    vec![
        73, 73, 42, 0, 8, 0, 0, 0, 1, 0, 0x12, 1, 3, 0, 1, 0, 0, 0, o, 0, 0, 0, 0, 0, 0, 0,
    ]
}
fn encode(format: Format, orientation: Option<u8>) -> Vec<u8> {
    let rgb: Vec<u8> = [20, 50, 80, 110, 140, 170]
        .into_iter()
        .flat_map(|n| [n, n, n])
        .collect();
    let rgba: Vec<u8> = rgb
        .chunks_exact(3)
        .enumerate()
        .flat_map(|(i, p)| [p[0], p[1], p[2], 20 + i as u8 * 40])
        .collect();
    let mut b = Vec::new();
    match format {
        Format::Png => {
            let mut e = PngEncoder::new(&mut b);
            if let Some(o) = orientation {
                e.set_exif_metadata(exif(o)).unwrap();
            }
            e.write_image(&rgba, 2, 3, ExtendedColorType::Rgba8)
                .unwrap();
        }
        Format::Jpeg => {
            let mut e = JpegEncoder::new_with_quality(&mut b, 100);
            if let Some(o) = orientation {
                e.set_exif_metadata(exif(o)).unwrap();
            }
            e.write_image(&rgb, 2, 3, ExtendedColorType::Rgb8).unwrap();
        }
        Format::WebP => {
            let mut e = WebPEncoder::new_lossless(&mut b);
            if let Some(o) = orientation {
                e.set_exif_metadata(exif(o)).unwrap();
            }
            e.write_image(&rgba, 2, 3, ExtendedColorType::Rgba8)
                .unwrap();
        }
    }
    b
}
fn retry<T>(mut call: impl FnMut() -> Result<T, Failure>) -> T {
    let start = Instant::now();
    loop {
        match call() {
            Ok(t) => return t,
            Err(Failure::Busy) => {
                assert!(start.elapsed() < Duration::from_secs(20));
                thread::yield_now();
            }
            Err(e) => panic!("unexpected refusal: {e:?}"),
        }
    }
}
fn refused<T>(mut call: impl FnMut() -> Result<T, Failure>) -> Failure {
    retry(|| match call() {
        Err(Failure::Busy) => Err(Failure::Busy),
        Err(e) => Ok(e),
        Ok(_) => panic!("expected refusal"),
    })
}
fn ready(service: &Service, target: Target) -> Response {
    let start = Instant::now();
    loop {
        if let Some(r) = retry(|| service.take_current(target)) {
            return r;
        }
        assert!(
            start.elapsed() < Duration::from_secs(20),
            "native request did not complete"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn stop(service: &mut Service) {
    service.stop();
    let start = Instant::now();
    loop {
        if let Some(r) = service.poll_join() {
            r.unwrap();
            return;
        }
        assert!(
            start.elapsed() < Duration::from_secs(20),
            "native cleanup still pending, not acknowledged"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn result(service: &Service, target: Target, path: &Path) -> Result<Frame, Failure> {
    let generation = retry(|| service.request(target, path));
    let response = ready(service, target);
    assert_eq!(response.generation, generation);
    response.result
}
fn executable() -> PathBuf {
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    p.pop();
    p.join("mado-image-helper.exe")
}
fn child(budget: &CpuBudget) -> native::Child {
    native::Child::launch(
        &executable(),
        budget,
        || true,
        |c, _| {
            assert!(c.before_resume_verified);
            Ok(())
        },
    )
    .unwrap()
}
fn probe(c: &native::Child, wake: &native::Handle, mode: u32, aux: u64) -> Request {
    let r = Request {
        target: Target(id().unwrap()),
        generation: Generation(id().unwrap()),
        path: Vec::new(),
        probe_mode: 0,
    };
    let mut wire = protocol::request(r.target.0, r.generation.0, &[], mode, aux);
    c.exact(&mut wire, true, wake, || true).unwrap();
    r
}

#[test]
fn native_three_formats_all_orientations_rgba_metadata_and_source_preservation() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let budget = CpuBudget::new();
    let mut service = Service::start(budget.clone()).unwrap();
    let target = retry(|| service.open_target());
    let orders = [
        [0, 1, 2, 3, 4, 5],
        [1, 0, 3, 2, 5, 4],
        [5, 4, 3, 2, 1, 0],
        [4, 5, 2, 3, 0, 1],
        [0, 2, 4, 1, 3, 5],
        [4, 2, 0, 5, 3, 1],
        [5, 3, 1, 4, 2, 0],
        [1, 3, 5, 0, 2, 4],
    ];
    for format in [Format::Png, Format::Jpeg, Format::WebP] {
        let path = fixture.write("base", &encode(format, None));
        let base = result(&service, target, &path).unwrap();
        assert_eq!(
            base.pixels()
                .chunks_exact(4)
                .map(|p| p[3])
                .collect::<Vec<_>>(),
            if format == Format::Jpeg {
                vec![255; 6]
            } else {
                vec![20, 60, 100, 140, 180, 220]
            }
        );
        for o in 1..=8 {
            let bytes = encode(format, Some(o));
            let path = fixture.write("日本語-é-𐐀.txt", &bytes);
            let before = fs::metadata(&path).unwrap().modified().unwrap();
            let checksum = hash(&bytes);
            let frame = result(&service, target, &path).unwrap();
            assert_eq!(frame.format, format);
            assert_eq!(frame.orientation, o);
            assert_eq!((frame.encoded_width, frame.encoded_height), (2, 3));
            assert_eq!(
                (frame.width, frame.height),
                if o >= 5 { (3, 2) } else { (2, 3) }
            );
            assert_eq!(frame.file_bytes, bytes.len() as u64);
            assert_eq!(frame.charged_bytes(), 24);
            let expected: Vec<u8> = orders[o as usize - 1]
                .iter()
                .flat_map(|i| base.pixels()[i * 4..i * 4 + 4].iter().copied())
                .collect();
            assert_eq!(frame.pixels(), expected);
            let after = fs::read(&path).unwrap();
            assert_eq!(hash(&after), checksum);
            assert_eq!(after, bytes);
            assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), before);
            assert_eq!(budget.used(), CONTROL_CAP + 48); // both actual externally held frames
        }
    }
    stop(&mut service);
    drop(service);
    assert_eq!(budget.used(), 0);
}
#[test]
fn native_bad_sources_oversized_dimensions_input_and_unpaired_paths() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let budget = CpuBudget::new();
    let mut service = Service::start(budget.clone()).unwrap();
    let target = retry(|| service.open_target());
    for f in [Format::Png, Format::Jpeg, Format::WebP] {
        let mut b = encode(f, None);
        b.truncate(b.len() / 2);
        let path = fixture.write("truncated", &b);
        assert_eq!(
            result(&service, target, &path).unwrap_err(),
            Failure::MalformedImage
        );
        assert_eq!(fs::read(path).unwrap(), b);
    }
    let mut huge = encode(Format::Jpeg, None);
    let at = huge.windows(2).position(|p| p == [255, 192]).unwrap();
    huge[at + 5..at + 9].copy_from_slice(&[255; 4]);
    let path = fixture.write("huge.jpg", &huge);
    assert_eq!(
        result(&service, target, &path).unwrap_err(),
        Failure::Oversize
    );
    assert_eq!(fs::read(path).unwrap(), huge);
    // Both dimensions fit 8192, but checked RGBA area exceeds the 32MiB ceiling.
    huge[at + 5..at + 7].copy_from_slice(&4000u16.to_be_bytes());
    huge[at + 7..at + 9].copy_from_slice(&4000u16.to_be_bytes());
    let path = fixture.write("oversized-area.jpg", &huge);
    assert_eq!(
        result(&service, target, &path).unwrap_err(),
        Failure::Oversize
    );
    assert_eq!(fs::read(path).unwrap(), huge);
    let old = retry(|| service.request(target, &fixture.0.join("missing.png")));
    for bad in [
        PathBuf::from("C:\\NUL.png"),
        PathBuf::from("\\\\.\\pipe\\x"),
        PathBuf::from("relative.png"),
    ] {
        assert_eq!(
            refused(|| service.request(target, &bad)),
            Failure::InvalidPath
        );
        assert!(retry(|| service.is_current(target, old)));
    }
    let long = PathBuf::from(format!("C:\\{}", "a".repeat(PATH_UNITS)));
    assert_eq!(
        refused(|| service.request(target, &long)),
        Failure::PathTooLong
    );
    assert!(retry(|| service.is_current(target, old)));
    let path = fixture.write("oversized.png", b"not PNG");
    let file = fs::OpenOptions::new().write(true).open(&path).unwrap();
    file.set_len(INPUT_CAP + 1).unwrap();
    drop(file);
    assert_eq!(
        result(&service, target, &path).unwrap_err(),
        Failure::Oversize
    );
    assert_eq!(fs::metadata(path).unwrap().len(), INPUT_CAP + 1);
    let unsupported = fixture.write("unsupported.gif", b"GIF89a");
    assert_eq!(
        result(&service, target, &unsupported).unwrap_err(),
        Failure::Unsupported
    );
    assert_eq!(
        result(&service, target, &fixture.0.join("missing-é")).unwrap_err(),
        Failure::Unavailable
    );
    assert_eq!(
        result(&service, target, &fixture.0).unwrap_err(),
        Failure::Unavailable
    );
    let name = OsString::from_wide(&[110, 0xd800, 46, 112]);
    let bytes = encode(Format::Png, None);
    let path = fixture.write(name, &bytes);
    let frame = result(&service, target, &path).unwrap();
    assert_eq!(frame.format, Format::Png);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    drop(frame);
    // No following handle fallback: the final file's path has no retained handle.
    fs::remove_file(&path).unwrap();
    assert_eq!(
        result(&service, target, &path).unwrap_err(),
        Failure::Unavailable
    );
    stop(&mut service);
    drop(service);
    assert_eq!(budget.used(), 0);
}
#[test]
fn shared_budget_exhaustion_pdf_held_buffer_and_external_frame_lifetime() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let path = fixture.write("small.png", &encode(Format::Png, None));
    let budget = CpuBudget::new();
    let mut service = Service::start(budget.clone()).unwrap();
    let target = retry(|| service.open_target());
    let frame = result(&service, target, &path).unwrap();
    assert_eq!(budget.used(), CONTROL_CAP + 24);
    // Real accepted PDF-service allocation on the SAME counter, no PDFium load.
    let pdf_held = budget.clone().allocate(96 * 1024 * 1024).unwrap();
    assert_eq!(
        result(&service, target, &path).unwrap_err(),
        Failure::BudgetExceeded
    );
    assert_eq!(budget.used(), CONTROL_CAP + 24 + 96 * 1024 * 1024);
    drop(pdf_held);
    let next = result(&service, target, &path).unwrap();
    assert_eq!(budget.used(), CONTROL_CAP + 48);
    stop(&mut service);
    drop(service);
    assert_eq!(budget.used(), 48);
    drop(frame);
    assert_eq!(budget.used(), 24);
    drop(next);
    assert_eq!(budget.used(), 0);
}
#[test]
fn bounded_coalescing_fifo_close_cancel_stop_and_recreation() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let a = fixture.write("a.png", &encode(Format::Png, None));
    let b = fixture.write("b.png", &encode(Format::Png, Some(6)));
    let budget = CpuBudget::new();
    let mut service = Service::start(budget.clone()).unwrap();
    let targets: Vec<_> = (0..MAX_TARGETS)
        .map(|_| retry(|| service.open_target()))
        .collect();
    assert_eq!(refused(|| service.open_target()), Failure::TargetLimit);
    let old = retry(|| service.request(targets[0], &a));
    let new = retry(|| service.request(targets[0], &b));
    assert!(new != old);
    let other = retry(|| service.request(targets[1], &a));
    assert!(retry(|| service.is_current(targets[0], new)));
    assert!(!retry(|| service.is_current(targets[0], old)));
    let r = ready(&service, targets[0]);
    assert_eq!(r.generation, new);
    assert_eq!(r.result.unwrap().orientation, 6);
    assert_eq!(ready(&service, targets[1]).generation, other);
    for _ in 0..100 {
        retry(|| service.request(targets[0], &a));
        retry(|| service.request(targets[0], &b));
        let s = service.shared.state.lock().unwrap();
        assert!(s.queue.len() <= MAX_TARGETS);
        drop(s);
    }
    retry(|| service.cancel(targets[0]));
    assert!(retry(|| service.take_current(targets[0])).is_none());
    retry(|| service.close_target(targets[1]));
    let replacement = retry(|| service.open_target());
    assert!(!targets.contains(&replacement));
    assert_eq!(
        refused(|| service.request(targets[1], &a)),
        Failure::UnknownTarget
    );
    stop(&mut service);
    assert_eq!(service.request(replacement, &a), Err(Failure::Stopped));
    assert_eq!(Service::start(budget.clone()).err(), Some(Failure::Busy));
    drop(service);
    assert_eq!(budget.used(), 0);
    let mut next = Service::start(budget.clone()).unwrap();
    let target = retry(|| next.open_target());
    assert!(!targets.contains(&target));
    assert_eq!(result(&next, target, &a).unwrap().format, Format::Png);
    stop(&mut next);
    drop(next);
    assert_eq!(budget.used(), 0);
}
#[test]
fn native_job_cap_enforcement_assignment_before_run_and_reap() {
    let _serial = SERIAL.lock().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = native::Handle::event().unwrap();
    let mut c = child(&budget);
    let r = probe(&c, &wake, 1, 0);
    let f = receive(&mut c, &r, &wake, || true).unwrap();
    let refusal = u32::from_le_bytes(f.pixels().try_into().unwrap());
    assert!(
        [8, 1455].contains(&refusal),
        "native VirtualAlloc refusal {refusal}"
    );
    let peak = c.peak_commit().unwrap();
    assert!(
        peak >= 64 * 1024 * 1024 && peak <= HELPER_CAP,
        "native job peak {peak}"
    );
    assert_eq!(c.exit_code().unwrap(), 0);
    assert_eq!(budget.used(), CONTROL_CAP + HELPER_CAP + 4);
    eprintln!(
        "Native contained helper: 64MiB commit succeeded; 144MiB attempt refused Win32 {refusal}; Job peak {peak}; assignment verified before ResumeThread."
    );
    drop(c);
    assert_eq!(budget.used(), CONTROL_CAP + 4);
    drop(f);
    assert_eq!(budget.used(), CONTROL_CAP);
}
#[test]
fn native_crash_survival_and_bounded_malformed_wire() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let sentinel = fixture.write("sentinel.png", &encode(Format::Png, None));
    let bytes = fs::read(&sentinel).unwrap();
    let timestamp = fs::metadata(&sentinel).unwrap().modified().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = native::Handle::event().unwrap();
    for mode in [2, 5, 6, 7, 8] {
        let mut c = child(&budget);
        let r = probe(&c, &wake, mode, 0);
        let error = receive(&mut c, &r, &wake, || true).unwrap_err();
        let error = native::child_failure(&c, error);
        if mode == 2 {
            assert_eq!(error, Failure::ChildFailure(0xc0000005));
            eprintln!(
                "Genuine helper access-violation exception exited 0xc0000005; parent continued."
            );
        } else {
            assert_eq!(error, Failure::MalformedProtocol);
        }
        assert_eq!(fs::read(&sentinel).unwrap(), bytes);
        assert_eq!(
            fs::metadata(&sentinel).unwrap().modified().unwrap(),
            timestamp
        );
        drop(c);
        assert_eq!(budget.used(), CONTROL_CAP);
    }
}
#[test]
fn native_overlapped_cancellation_keeps_charges_through_ack_and_exit() {
    let _serial = SERIAL.lock().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = Arc::new(native::Handle::event().unwrap());
    let c = child(&budget);
    let _r = probe(&c, &wake, 3, 0);
    let buffer = Box::new([0u8; 4]); // Stable control buffer under the live 8MiB envelope.
    let cancel = Arc::new(AtomicBool::new(false));
    let observed = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let (w, flag, seen, count) = (
        wake.clone(),
        cancel.clone(),
        observed.clone(),
        calls.clone(),
    );
    // One bounded native probe driver, no production per-request threads.
    let driver = thread::spawn(move || {
        let mut buffer = buffer;
        let r = c.transfer(&mut *buffer, false, &w, || {
            if count.fetch_add(1, Ordering::AcqRel) > 0 {
                seen.store(true, Ordering::Release);
            }
            !flag.load(Ordering::Acquire)
        });
        assert_eq!(r, Err(Failure::Cancelled));
        drop(buffer);
        assert_eq!(c.exit_code().unwrap(), 995);
        drop(c);
    });
    let start = Instant::now();
    while !observed.load(Ordering::Acquire) {
        assert!(start.elapsed() < Duration::from_secs(20));
        thread::yield_now();
    }
    assert_eq!(budget.used(), CONTROL_CAP + HELPER_CAP + OUTPUT_CAP);
    cancel.store(true, Ordering::Release);
    wake.signal();
    driver.join().unwrap();
    assert_eq!(budget.used(), CONTROL_CAP);
}
#[test]
fn native_unwind_preserves_pending_resources_until_real_cleanup() {
    let _serial = SERIAL.lock().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = native::Handle::event().unwrap();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let c = child(&budget);
        probe(&c, &wake, 3, 0);
        let mut b = Box::new([0u8; 4]); // Control reservation outlives native ack.
        let mut calls = 0;
        c.transfer(&mut *b, false, &wake, || {
            calls += 1;
            if calls == 2 {
                panic!("owned pending unwind probe");
            }
            true
        })
        .unwrap();
    }));
    assert!(r.is_err());
    assert_eq!(budget.used(), CONTROL_CAP);
}

#[test]
fn actual_child_allocator_failure_survival_has_native_exit_evidence() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let bytes = encode(Format::Png, None);
    let source = fixture.write("oom-sentinel.png", &bytes);
    let before = fs::metadata(&source).unwrap().modified().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = native::Handle::event().unwrap();
    let c = child(&budget);
    probe(&c, &wake, 10, 0);
    // This diagnostic checks fatal allocator stderr + real exit directly. The
    // production parser instead rejects this non-protocol output and terminates
    // promptly, which can race abort and must NOT be labeled MemoryLimit.
    let mut diagnostic = [0; ERROR_CAP];
    let mut length = 0;
    loop {
        let mut part = [0; 128];
        let n = c.transfer(&mut part, false, &wake, || true).unwrap();
        if n == 0 {
            break;
        }
        assert!(length + n <= ERROR_CAP);
        diagnostic[length..length + n].copy_from_slice(&part[..n]);
        length += n;
    }
    let message = std::str::from_utf8(&diagnostic[..length]).unwrap();
    assert!(
        message.contains("memory allocation of 150994944 bytes failed"),
        "allocator diagnostic {message:?}"
    );
    let code = c.wait_exit(&wake, || true).unwrap();
    assert_ne!(code, 0);
    assert_ne!(code, 995);
    assert_ne!(code, 65);
    assert!(c.peak_commit().unwrap() <= HELPER_CAP);
    assert_eq!(fs::read(&source).unwrap(), bytes);
    assert_eq!(fs::metadata(&source).unwrap().modified().unwrap(), before);
    drop(c);
    assert_eq!(budget.used(), CONTROL_CAP);
    eprintln!(
        "Real capped-child Rust allocator fatal diagnostic for 144MiB attempt, native exit {code:#x}; parent/source survive. No arbitrary exit is relabeled MemoryLimit."
    );
}
#[test]
fn actual_worker_survives_child_crash_and_allocator_failure_then_decodes_next() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let bytes = encode(Format::Png, None);
    let path = fixture.write("survivor.png", &bytes);
    let before = fs::metadata(&path).unwrap().modified().unwrap();
    let budget = CpuBudget::new();
    let mut service = Service::start(budget.clone()).unwrap();
    let target = retry(|| service.open_target());
    for mode in [2, 10] {
        let generation = Generation(id().unwrap());
        {
            let mut state = service.shared.state.lock().unwrap();
            state
                .slots
                .iter_mut()
                .find(|s| s.target == target)
                .unwrap()
                .current = Some(generation);
            state.queue.push_back(Request {
                target,
                generation,
                path: Vec::new(),
                probe_mode: mode,
            });
        }
        service.shared.wake.signal();
        let response = ready(&service, target);
        assert_eq!(response.generation, generation);
        let error = response.result.unwrap_err();
        assert!(
            matches!(error, Failure::ChildFailure(_) | Failure::MalformedProtocol),
            "fatal helper fallback: {error:?}"
        );
        assert_eq!(budget.used(), CONTROL_CAP);
        assert_eq!(native::CHILDREN.load(Ordering::SeqCst), 0);
        assert_eq!(native::PENDING.load(Ordering::SeqCst), 0);
        let frame = result(&service, target, &path).unwrap();
        assert_eq!(frame.format, Format::Png);
        assert_eq!(frame.charged_bytes(), 24);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), before);
        drop(frame);
    }
    stop(&mut service);
    drop(service);
    assert_eq!(budget.used(), 0);
}
fn wait_until(mut condition: impl FnMut() -> bool) {
    let start = Instant::now();
    while !condition() {
        assert!(start.elapsed() < Duration::from_secs(20));
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn actual_worker_cancel_close_stop_and_fair_replacement_never_overlap_helpers() {
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let path = fixture.write("next.png", &encode(Format::Png, None));
    native::PEAK_CHILDREN.store(0, Ordering::SeqCst);
    for action in 0..3 {
        let budget = CpuBudget::new();
        let mut service = Service::start(budget.clone()).unwrap();
        let target = retry(|| service.open_target());
        let other = retry(|| service.open_target());
        let generation = Generation(id().unwrap());
        // Private diagnostic request enters the REAL window worker. The public
        // request API cannot select diagnostic modes; production always sends 0.
        {
            let mut state = service.shared.state.lock().unwrap();
            state
                .slots
                .iter_mut()
                .find(|s| s.target == target)
                .unwrap()
                .current = Some(generation);
            state.queue.push_back(Request {
                target,
                generation,
                path: Vec::new(),
                probe_mode: 3,
            });
        }
        service.shared.wake.signal();
        wait_until(|| native::PENDING.load(Ordering::SeqCst) == 1);
        assert_eq!(native::CHILDREN.load(Ordering::SeqCst), 1);
        assert_eq!(budget.used(), CONTROL_CAP + HELPER_CAP + OUTPUT_CAP);
        let other_generation = retry(|| service.request(other, &path));
        match action {
            0 => {
                // Active replacement must join behind the already pending other
                // target, and its still-running old generation cannot publish.
                let replacement = retry(|| service.request(target, &path));
                let s = service.shared.state.lock().unwrap();
                assert_eq!(s.submitted_order[0], Some(other));
                drop(s);
                let r = ready(&service, other);
                assert_eq!(r.generation, other_generation);
                drop(r);
                let r = ready(&service, target);
                assert_eq!(r.generation, replacement);
                assert_ne!(r.generation, generation);
                drop(r);
                // Also exercise explicit cancel of a real suspended IPC reader.
                {
                    let generation = Generation(id().unwrap());
                    let mut s = service.shared.state.lock().unwrap();
                    s.slots
                        .iter_mut()
                        .find(|s| s.target == target)
                        .unwrap()
                        .current = Some(generation);
                    s.queue.push_back(Request {
                        target,
                        generation,
                        path: Vec::new(),
                        probe_mode: 3,
                    });
                }
                service.shared.wake.signal();
                wait_until(|| native::PENDING.load(Ordering::SeqCst) == 1);
                retry(|| service.cancel(target));
                wait_until(|| native::CHILDREN.load(Ordering::SeqCst) == 0);
                assert!(retry(|| service.take_current(target)).is_none());
            }
            1 => {
                retry(|| service.close_target(target));
                let r = ready(&service, other);
                assert_eq!(r.generation, other_generation);
                drop(r);
                assert_eq!(
                    service.take_current(target).unwrap_err(),
                    Failure::UnknownTarget
                );
            }
            _ => service.stop(),
        }
        stop(&mut service);
        assert_eq!(native::CHILDREN.load(Ordering::SeqCst), 0);
        assert_eq!(native::PENDING.load(Ordering::SeqCst), 0);
        assert_eq!(budget.used(), CONTROL_CAP);
        drop(service);
        assert_eq!(budget.used(), 0);
    }
    assert_eq!(native::PEAK_CHILDREN.load(Ordering::SeqCst), 1);
}

#[test]
fn native_request_header_oversize_missing_helper_and_nonfollowing_sources() {
    use std::{
        io::Read,
        os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
        ptr,
    };
    use windows_sys::Win32::{Storage::FileSystem::*, System::IO::DeviceIoControl};
    let _serial = SERIAL.lock().unwrap();
    let fixture = Fixture::new();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = native::Handle::event().unwrap();
    let absent = fixture.0.join("missing-own-helper.exe");
    assert!(matches!(
        native::Child::launch(&absent, &budget, || true, |_, _| Ok(())),
        Err(Failure::MissingHelper)
    ));
    assert_eq!(budget.used(), CONTROL_CAP);
    for at in [4, 24] {
        let c = child(&budget);
        let mut wire = protocol::request(1, 2, &[], 0, 0);
        protocol::put32(&mut wire, at, u32::MAX);
        c.exact(&mut wire, true, &wake, || true).unwrap();
        let mut out = [0; HEADER_SIZE];
        assert!(c.exact(&mut out, false, &wake, || true).is_err());
        assert_eq!(c.exit_code().unwrap(), 65);
        drop(c);
        assert_eq!(budget.used(), CONTROL_CAP);
    }
    let target = fixture.0.join("owned-target");
    fs::create_dir(&target).unwrap();
    let sentinel = target.join("sentinel.png");
    let bytes = encode(Format::Png, None);
    fs::write(&sentinel, &bytes).unwrap();
    let before = fs::metadata(&sentinel).unwrap().modified().unwrap();
    let link = fixture.0.join("owned-junction");
    fs::create_dir(&link).unwrap();
    let junction = fs::OpenOptions::new()
        .access_mode(FILE_WRITE_ATTRIBUTES)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(&link)
        .unwrap();
    let mut substitute: Vec<u16> = "\\??\\".encode_utf16().collect();
    substitute.extend(target.as_os_str().encode_wide().skip(4));
    let length = (substitute.len() * 2) as u16;
    let mut buffer = Vec::new();
    buffer.extend(0xa0000003u32.to_le_bytes());
    buffer.extend((8 + length + 4).to_le_bytes());
    buffer.extend(0u16.to_le_bytes());
    buffer.extend(0u16.to_le_bytes());
    buffer.extend(length.to_le_bytes());
    buffer.extend((length + 2).to_le_bytes());
    buffer.extend(0u16.to_le_bytes());
    for unit in substitute {
        buffer.extend(unit.to_le_bytes());
    }
    buffer.extend([0; 4]);
    let mut returned = 0;
    // Generated owned junction only; no privilege, shell or external executable.
    assert_ne!(
        unsafe {
            DeviceIoControl(
                junction.as_raw_handle(),
                0x900a4,
                buffer.as_ptr() as _,
                buffer.len() as u32,
                ptr::null_mut(),
                0,
                &mut returned,
                ptr::null_mut(),
            )
        },
        0,
        "owned junction fixture failed: {}",
        std::io::Error::last_os_error()
    );
    drop(junction);
    let alias = link.join("sentinel.png");
    // Positive control proves this is a functioning owned junction, not an
    // invalid target that every filesystem open would reject.
    assert_eq!(fs::read(&alias).unwrap(), bytes);
    let refusal = source::open(&alias.as_os_str().encode_wide().collect::<Vec<_>>()).unwrap_err();
    assert_eq!(refusal, 4395); // Absolute NtCreateFile parsing refusal on this host.
    assert_eq!(fs::read(&sentinel).unwrap(), bytes);
    assert_eq!(fs::metadata(&sentinel).unwrap().modified().unwrap(), before);
    let mut pinned = source::open(&sentinel.as_os_str().encode_wide().collect::<Vec<_>>()).unwrap();
    assert_eq!(
        fs::OpenOptions::new()
            .write(true)
            .open(&sentinel)
            .unwrap_err()
            .raw_os_error(),
        Some(32)
    );
    let mut actual = Vec::new();
    pinned.read_to_end(&mut actual).unwrap();
    assert_eq!(actual, bytes);
    drop(pinned);
    eprintln!(
        "Actual functioning source ancestor junction refused by OBJ_DONT_REPARSE/Win32 4395; same regular read-only handle pins source, bytes/time unchanged."
    );
}
#[test]
fn eof_before_process_exit_remains_cancellable() {
    let _serial = SERIAL.lock().unwrap();
    let budget = CpuBudget::new();
    let _control = budget.reserve_capacity(CONTROL_CAP).unwrap();
    let wake = Arc::new(native::Handle::event().unwrap());
    let c = child(&budget);
    probe(&c, &wake, 9, 0);
    let mut bytes = [0; HEADER_SIZE + 4];
    c.exact(&mut bytes, false, &wake, || true).unwrap();
    let mut eof = [0];
    assert_eq!(c.transfer(&mut eof, false, &wake, || true).unwrap(), 0);
    let cancel = Arc::new(AtomicBool::new(false));
    let entered = Arc::new(AtomicBool::new(false));
    let (w, flag, seen) = (wake.clone(), cancel.clone(), entered.clone());
    let driver = thread::spawn(move || {
        let r = c.wait_exit(&w, || {
            seen.store(true, Ordering::Release);
            !flag.load(Ordering::Acquire)
        });
        assert_eq!(r, Err(Failure::Cancelled));
        drop(c);
    });
    wait_until(|| entered.load(Ordering::Acquire));
    assert_eq!(budget.used(), CONTROL_CAP + HELPER_CAP + OUTPUT_CAP);
    cancel.store(true, Ordering::Release);
    wake.signal();
    driver.join().unwrap();
    assert_eq!(budget.used(), CONTROL_CAP);
}
