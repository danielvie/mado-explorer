use mado_pdf_preview_service::{
    BackendState, CpuBudget, Failure, Frame, Request, Response, Service, Target,
};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const ARTIFACT: &str =
    "C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881";
const LIBRARY: &str =
    "C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll";
const ARCHIVE_HASH: &str = "73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08";
const DLL_HASH: &str = "79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118";

fn verified_library() -> PathBuf {
    assert!(
        cfg!(all(windows, target_arch = "x86_64")),
        "native check pending: this artifact requires a Windows x64 host"
    );
    let supplied = std::env::var_os("MADO_P14_TRUSTED_PDFIUM")
        .expect("native check pending: explicitly supply the documented managed PDFium DLL path");
    let supplied = PathBuf::from(supplied);
    assert!(supplied.is_absolute());
    assert_eq!(supplied, Path::new(LIBRARY), "no arbitrary/environment DLL");
    let root = Path::new(ARTIFACT);
    assert_eq!(
        fs::read_to_string(root.join("VERSION"))
            .unwrap()
            .replace('\r', ""),
        "MAJOR=151\nMINOR=0\nBUILD=7881\nPATCH=0\n"
    );
    assert_eq!(
        fs::read_to_string(root.join("args.gn"))
            .unwrap()
            .replace('\r', ""),
        concat!(
            "is_component_build = false\nis_debug = false\n",
            "pdf_enable_v8 = false\npdf_enable_xfa = false\n",
            "pdf_is_standalone = true\npdf_use_partition_alloc = false\n",
            "target_cpu = \"x64\"\ntarget_os = \"win\"\n",
            "treat_warnings_as_errors = false\n"
        )
    );
    assert_eq!(entries(&root.join("bin")), vec!["pdfium.dll"]);
    // Read-only platform hashing, no new dependency or native load. Only fixed,
    // documented paths enter this command, never the supplied environment value.
    let output = Command::new("pwsh")
        .args([
            "-NoProfile", "-NonInteractive", "-Command",
            &format!(
                "$ErrorActionPreference='Stop'; Get-FileHash -Algorithm SHA256 -LiteralPath '{ARTIFACT}/pdfium-win-x64.tgz','{LIBRARY}' | ForEach-Object {{ $_.Hash.ToLowerInvariant() }}"
            ),
        ])
        .output()
        .expect("native check pending: pwsh SHA-256 verification unavailable");
    assert!(
        output.status.success(),
        "hash verification: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .collect::<Vec<_>>(),
        vec![ARCHIVE_HASH, DLL_HASH],
        "native check blocked: managed artifact provenance mismatch"
    );
    println!(
        "Verified managed PDFium 151.0.7881.0 Windows x64 release, non-V8/non-XFA; archive {ARCHIVE_HASH}, DLL {DLL_HASH}"
    );
    supplied
}

fn entries(path: &Path) -> Vec<String> {
    let mut names: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

struct OwnedFixtures {
    root: PathBuf,
    files: BTreeMap<String, Vec<u8>>,
}
impl OwnedFixtures {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mado-p14-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        // Never claim a pre-existing directory, even on a collision.
        fs::create_dir(&root).unwrap();
        let mut owned = Self {
            root,
            files: BTreeMap::new(),
        };
        fs::create_dir(owned.root.join("sentinel-directory")).unwrap();
        owned.write(
            "sentinel-directory/untouched.txt",
            b"owned directory sentinel\n",
        );
        owned
    }
    fn write(&mut self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.root.join(name);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        self.files.insert(name.into(), bytes.to_vec());
        file.write_all(bytes).unwrap();
        path
    }
    fn unchanged(&self) -> bool {
        for directory in [&self.root, &self.root.join("sentinel-directory")] {
            if !fs::symlink_metadata(directory)
                .is_ok_and(|m| m.is_dir() && !m.file_type().is_symlink())
            {
                return false;
            }
        }
        let mut expected: Vec<_> = self
            .files
            .keys()
            .filter(|name| !name.contains('/'))
            .cloned()
            .collect();
        expected.push("sentinel-directory".into());
        expected.sort();
        entries(&self.root) == expected
            && entries(&self.root.join("sentinel-directory")) == vec!["untouched.txt"]
            && self.files.iter().all(|(name, bytes)| {
                let path = self.root.join(name);
                fs::symlink_metadata(&path)
                    .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink())
                    && fs::read(path).is_ok_and(|actual| actual == *bytes)
            })
    }
}
impl Drop for OwnedFixtures {
    fn drop(&mut self) {
        // Refuse cleanup if preservation/ownership checks fail. Never recursively
        // delete unknown entries or follow a substituted directory/link.
        if !self.unchanged() {
            eprintln!("Owned fixtures retained for inspection at {:?}", self.root);
            return;
        }
        for name in self.files.keys() {
            fs::remove_file(self.root.join(name)).unwrap();
        }
        fs::remove_dir(self.root.join("sentinel-directory")).unwrap();
        fs::remove_dir(&self.root).unwrap();
    }
}

// Fixture writer only. Reuse the accepted backend's generated xref pattern.
fn pdf(objects: &[&str], extra_trailer: &str) -> Vec<u8> {
    let mut bytes = b"%PDF-1.3\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(bytes.len());
        bytes.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", index + 1).as_bytes());
    }
    let xref = bytes.len();
    bytes.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets {
        bytes.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    bytes.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R {extra_trailer} >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    bytes
}
fn two_pages(reverse: bool) -> Vec<u8> {
    let (left, right) = if reverse {
        ("0 0 1", "1 0 0")
    } else {
        ("1 0 0", "0 0 1")
    };
    let first = format!(
        "{left} rg 10 25 40 50 re f\n0 1 0 rg 75 25 50 50 re f\n{right} rg 150 25 40 50 re f\n"
    );
    let second = "0 0 1 rg 25 75 50 50 re f\n";
    pdf(
        &[
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Count 2 /Kids [3 0 R 5 0 R] >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << >> /Contents 4 0 R >>",
            &format!("<< /Length {} >>\nstream\n{first}endstream", first.len()),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 200] /Resources << >> /Contents 6 0 R >>",
            &format!("<< /Length {} >>\nstream\n{second}endstream", second.len()),
        ],
        "",
    )
}
fn encrypted_pdf() -> Vec<u8> {
    // Accepted backend fixture: pypdf 6.20.0 R2/RC4-40 constants,
    // user=secret, owner=owner. No crypto implementation/runtime dependency.
    let id = "6461613635663736666539376465383532613130666433313231656236363333";
    pdf(
        &[
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << >> >>",
            "<< /Filter /Standard /V 1 /R 2 /Length 40 /P -4 /O <92fe0f4454ad4c9644693f33c07cb54f587dce1e2682fe9ecea6107a1ef630dd> /U <c9b4a4046bedb8d07f5630089ff86342a08d1c39dccbdfdc07841faa9bc65046> >>",
        ],
        &format!("/Encrypt 4 0 R /ID [<{id}> <{id}>]"),
    )
}

fn retry<T>(mut call: impl FnMut() -> Result<T, Failure>) -> T {
    let started = Instant::now();
    loop {
        match call() {
            Err(Failure::Busy) if started.elapsed() < Duration::from_secs(30) => {
                thread::sleep(Duration::from_millis(2))
            }
            result => return result.unwrap(),
        }
    }
}
fn refused<T>(mut call: impl FnMut() -> Result<T, Failure>) -> Failure {
    retry(|| match call() {
        Err(Failure::Busy) => Err(Failure::Busy),
        Err(error) => Ok(error),
        Ok(_) => panic!("retired/stopped target must refuse the action"),
    })
}
fn take(service: &Service, target: Target) -> Response {
    let started = Instant::now();
    loop {
        if let Some(response) = retry(|| service.take(target)) {
            return response;
        }
        // Observation timeout for these tiny generated fixtures, not a native
        // cancellation guarantee. Service still owns/joins its worker on failure.
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "generated native response not observed"
        );
        thread::sleep(Duration::from_millis(2));
    }
}
fn request(page_index: u32, size: u32) -> Request {
    Request {
        page_index,
        width: size,
        height: size,
    }
}
fn pixel(frame: &Frame, x: u32, y: u32) -> &[u8] {
    let offset = ((y * frame.width + x) * 4) as usize;
    &frame.pixels()[offset..offset + 4]
}
fn check_frame(frame: &Frame, path: &Path, bytes: usize, page: u32, size: u32, reverse: bool) {
    assert_eq!(frame.document.path, path);
    assert_eq!(frame.document.input_bytes, bytes);
    assert_eq!(frame.document.page_count, 2);
    assert_eq!(frame.document.pdf_version, Some(13));
    assert_eq!(frame.page.index, page);
    let (points, dimensions) = if page == 0 {
        ((200.0, 100.0), (size, size / 2))
    } else {
        ((100.0, 200.0), (size / 2, size))
    };
    assert_eq!((frame.page.width_points, frame.page.height_points), points);
    assert_eq!((frame.width, frame.height), dimensions);
    assert_eq!(
        frame.pixels().len(),
        (frame.width * frame.height * 4) as usize
    );
    assert_eq!(frame.charged_bytes(), frame.pixels().len());
    assert_eq!(pixel(frame, 0, 0), [255, 255, 255, 255]);
    assert!(frame.pixels().chunks_exact(4).all(|p| p[3] == 255));
    if page == 0 {
        let (left, right) = if reverse {
            ([0, 0, 255, 255], [255, 0, 0, 255])
        } else {
            ([255, 0, 0, 255], [0, 0, 255, 255])
        };
        assert_eq!(pixel(frame, frame.width * 15 / 100, frame.height / 2), left);
        assert_eq!(
            pixel(frame, frame.width / 2, frame.height / 2),
            [0, 255, 0, 255]
        );
        assert_eq!(
            pixel(frame, frame.width * 85 / 100, frame.height / 2),
            right
        );
    } else {
        assert_eq!(
            pixel(frame, frame.width / 2, frame.height / 2),
            [0, 0, 255, 255]
        );
    }
}

#[test]
#[ignore = "opt-in managed PDFium 7881 provenance check and public Service native pipeline; see task native"]
fn native_service_managed_pdfium_7881() {
    let library = verified_library(); // All provenance checks precede Service::start.
    let mut fixtures = OwnedFixtures::new();
    let first_bytes = two_pages(false);
    let replacement_bytes = two_pages(true);
    let first = fixtures.write("first-é-𐐀.pdf", &first_bytes);
    let replacement = fixtures.write("replacement.pdf", &replacement_bytes);
    let malformed = fixtures.write("malformed.pdf", b"%PDF-1.7\nnot a document\n");
    let password = fixtures.write("password.pdf", &encrypted_pdf());
    let missing = fixtures.root.join("missing.pdf");
    let budget = CpuBudget::new();
    let mut service = Service::start(&library, budget.clone()).unwrap();
    let a = retry(|| service.open_target());
    let b = retry(|| service.open_target());
    let mut caller_path = first.clone();
    let old_generation = retry(|| service.request(a, &caller_path, request(0, 400)));
    caller_path.clone_from(&replacement); // Accepted native source path is captured.
    let old = take(&service, a);
    assert_eq!(
        (old.target, old.generation, &old.path),
        (a, old_generation, &first)
    );
    assert!(retry(|| service.is_current(a, old_generation)));
    check_frame(
        old.result.as_ref().unwrap(),
        &first,
        first_bytes.len(),
        0,
        400,
        false,
    );
    let old_charge = old.result.as_ref().unwrap().charged_bytes();
    assert_eq!(budget.used(), old_charge);

    let b_generation = retry(|| service.request(b, &first, request(1, 400)));
    let new_generation = retry(|| service.request(a, &first, request(0, 600)));
    assert_ne!(old_generation, new_generation);
    // Deterministic proof: the old frame was ALREADY taken, no scheduling guess.
    assert!(!retry(|| service.is_current(a, old_generation)));
    assert!(retry(|| service.is_current(b, b_generation)));
    let sharper = take(&service, a);
    let second = take(&service, b);
    assert_eq!(
        (sharper.target, sharper.generation, &sharper.path),
        (a, new_generation, &first)
    );
    assert_eq!(
        (second.target, second.generation, &second.path),
        (b, b_generation, &first)
    );
    check_frame(
        sharper.result.as_ref().unwrap(),
        &first,
        first_bytes.len(),
        0,
        600,
        false,
    );
    check_frame(
        second.result.as_ref().unwrap(),
        &first,
        first_bytes.len(),
        1,
        400,
        false,
    );
    assert!(retry(|| service.is_current(a, new_generation)));
    let replacement_generation = retry(|| service.request(a, &caller_path, request(0, 600)));
    assert!(!retry(|| service.is_current(a, new_generation)));
    let changed = take(&service, a);
    assert_eq!(
        (changed.target, changed.generation, &changed.path),
        (a, replacement_generation, &replacement)
    );
    assert!(retry(|| service.is_current(a, replacement_generation)));
    check_frame(
        changed.result.as_ref().unwrap(),
        &replacement,
        replacement_bytes.len(),
        0,
        600,
        true,
    );
    let sharper_charge = sharper.result.as_ref().unwrap().charged_bytes();
    let changed_charge = changed.result.as_ref().unwrap().charged_bytes();
    let second_charge = second.result.as_ref().unwrap().charged_bytes();
    let held = old_charge + sharper_charge + changed_charge + second_charge;
    assert_eq!(budget.used(), held);

    for (path, state) in [
        (&malformed, BackendState::MalformedDocument),
        (&password, BackendState::PasswordRequired),
        (&missing, BackendState::Io),
    ] {
        let generation = retry(|| service.request(a, path, request(0, 400)));
        let response = take(&service, a);
        assert_eq!(
            (response.target, response.generation, &response.path),
            (a, generation, path)
        );
        assert!(retry(|| service.is_current(a, generation)));
        assert!(
            matches!(response.result, Err(Failure::Backend { state: actual, message }) if actual == state && !message.is_empty())
        );
        assert_eq!(
            budget.used(),
            held,
            "failed native work releases its admission"
        );
        assert!(retry(|| service.is_current(b, b_generation)));
    }
    retry(|| service.cancel(a));
    assert!(!retry(|| service.is_current(a, new_generation)));
    assert!(retry(|| service.take(a)).is_none());
    let cancelled = retry(|| service.request(a, &first, request(0, 200)));
    retry(|| service.cancel(a));
    assert!(!retry(|| service.is_current(a, cancelled)));
    // A second-target render is a worker-order barrier, whether the cancelled
    // request was pending, active, or already completed when cancel acquired state.
    let barrier_generation = retry(|| service.request(b, &first, request(0, 200)));
    assert!(!retry(|| service.is_current(b, b_generation)));
    let barrier = take(&service, b);
    assert_eq!(
        (barrier.target, barrier.generation, &barrier.path),
        (b, barrier_generation, &first)
    );
    assert!(retry(|| service.is_current(b, barrier_generation)));
    check_frame(
        barrier.result.as_ref().unwrap(),
        &first,
        first_bytes.len(),
        0,
        200,
        false,
    );
    assert!(retry(|| service.take(a)).is_none());
    assert_eq!(
        budget.used(),
        held + barrier.result.as_ref().unwrap().charged_bytes()
    );
    drop(barrier);
    assert_eq!(budget.used(), held);

    let closed_generation = retry(|| service.request(a, &first, request(0, 200)));
    assert!(retry(|| service.is_current(a, closed_generation)));
    retry(|| service.close_target(a));
    assert!(!retry(|| service.is_current(a, closed_generation)));
    assert_eq!(refused(|| service.take(a)), Failure::UnknownTarget);
    assert_eq!(
        refused(|| service.request(a, &first, request(0, 200))),
        Failure::UnknownTarget
    );
    // Another actual worker result proves any closed-target native work and
    // unpublished allocation have finished before checking the stable held sum.
    let close_barrier_generation = retry(|| service.request(b, &first, request(0, 200)));
    let close_barrier = take(&service, b);
    assert_eq!(close_barrier.generation, close_barrier_generation);
    check_frame(
        close_barrier.result.as_ref().unwrap(),
        &first,
        first_bytes.len(),
        0,
        200,
        false,
    );
    drop(close_barrier);
    retry(|| service.close_target(b));
    assert!(!retry(|| service.is_current(b, close_barrier_generation)));
    assert_eq!(
        budget.used(),
        held,
        "taken frames remain caller-owned after close"
    );
    service.request_stop();
    assert_eq!(service.take(a).unwrap_err(), Failure::Stopped);
    assert_eq!(service.open_target().unwrap_err(), Failure::Stopped);
    let started = Instant::now();
    loop {
        if let Some(joined) = service.try_join() {
            joined.unwrap();
            break;
        }
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "idle worker completion not observed; this is not an interruption deadline"
        );
        thread::sleep(Duration::from_millis(2));
    }
    assert!(service.try_join().is_none());
    assert_eq!(
        budget.used(),
        held,
        "caller leases survive actual worker join"
    );
    drop(service);
    drop(old);
    assert_eq!(
        budget.used(),
        sharper_charge + changed_charge + second_charge
    );
    drop(sharper);
    assert_eq!(budget.used(), changed_charge + second_charge);
    drop(changed);
    assert_eq!(budget.used(), second_charge);
    drop(second);
    assert_eq!(
        budget.used(),
        0,
        "all actual native buffers freed without leak/underflow"
    );
    assert!(
        fixtures.unchanged(),
        "source bytes and directory sentinel must be preserved"
    );
    let owned_root = fixtures.root.clone();
    drop(fixtures);
    assert!(
        !owned_root.exists(),
        "only claimed fixture files/directories cleaned up"
    );
    println!(
        "Public Service native pipeline passed: captured paths, first/page2 RGBA+metadata, physical resize, independent targets/current generations, native malformed/password/missing fallbacks, cancel/close/stop/try_join, held CPU charge/drop and unchanged owned sources/sentinel."
    );
}
