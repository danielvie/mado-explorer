use super::*;
use std::{
    fs::OpenOptions,
    os::windows::{
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
    },
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

// Each fixture creates a unique root, never adopts an existing directory, and retains
// its native identity. Guards refuse replaced/reparse roots. No source contents or
// ownership-marker contents are read. All mutations below are test-only and owned.
struct Fixture {
    root: PathBuf,
    owner: Snapshot,
}
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "mado-p13-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let units: Vec<_> = root.as_os_str().encode_wide().collect();
        assert!(
            units.len() >= 3 && matches!(units[0], 65..=90 | 97..=122) && units[1..3] == [58, 92],
            "Fixtures require a local absolute temporary parent"
        );
        let root = PathBuf::from(OsString::from_wide(
            &[92, 92, 63, 92]
                .into_iter()
                .chain(units)
                .collect::<Vec<_>>(),
        ));
        fs::create_dir(&root).unwrap();
        let owner = Snapshot::capture(root.clone()).unwrap();
        assert_eq!(owner.kind(), ObservedKind::Directory);
        Self { root, owner }
    }
    fn guard(&self) {
        self.owner.revalidate_original_name().unwrap();
    }
    fn path(&self, leaf: impl AsRef<OsStr>) -> PathBuf {
        self.guard();
        validate_name(&leaf.as_ref().encode_wide().collect::<Vec<_>>()).unwrap();
        self.root.join(leaf.as_ref())
    }
    fn guard_path(&self, path: &Path) {
        self.guard();
        assert!(path.starts_with(&self.root) && path != self.root);
        assert!(
            !path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        );
    }
    fn file(&self, leaf: impl AsRef<OsStr>) -> PathBuf {
        let path = self.path(leaf);
        self.guard_path(&path);
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .unwrap();
        path
    }
    fn dir(&self, leaf: impl AsRef<OsStr>) -> PathBuf {
        let path = self.path(leaf);
        self.guard_path(&path);
        fs::create_dir(&path).unwrap();
        path
    }
    fn rename(&self, from: &Path, to: &Path) {
        self.guard_path(from);
        self.guard_path(to);
        fs::rename(from, to).unwrap();
    }
    fn remove_dir(&self, path: &Path) {
        self.guard_path(path);
        fs::remove_dir(path).unwrap();
    }
    fn entries(&self) -> Vec<OsString> {
        self.guard();
        let mut names = fs::read_dir(&self.root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>();
        names.sort();
        names
    }
    fn planner(&self, name: impl Into<OsString>, kind: EntryKind) -> Planner {
        Planner::capture(name.into(), kind, self.root.clone()).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.guard();
        // std Windows remove_dir_all does not follow reparse leaves. The junction test
        // explicitly removes its owned leaf first, and its target has separate ownership.
        fs::remove_dir_all(&self.root).unwrap();
    }
}
fn wide(name: &OsStr) -> Vec<u16> {
    name.encode_wide().collect()
}

#[test]
fn many_collisions_next_unused_and_planner_does_not_write() {
    let f = Fixture::new();
    f.file("report.txt");
    for number in 2..=100 {
        f.file(format!("report ({number}).txt"));
    }
    let before = f.entries();
    let planner = f.planner("report.txt", EntryKind::File);
    let candidate = planner.plan().unwrap();
    assert_eq!(candidate.basename(), "report (101).txt");
    assert_eq!(candidate.number(), 101);
    assert_eq!(candidate.path(), f.path("report (101).txt"));
    assert_eq!(
        fs::symlink_metadata(candidate.path()).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(f.entries(), before);
    assert_eq!(planner.destination(), f.root);
    assert_eq!(planner.original_basename(), "report.txt");
    assert_eq!(planner.kind(), EntryKind::File);
}

#[test]
fn file_extension_last_suffix_leading_dot_and_folder_policy() {
    let f = Fixture::new();
    for (original, kind, expected) in [
        ("report.txt", EntryKind::File, "report (2).txt"),
        ("archive.tar.gz", EntryKind::File, "archive.tar (2).gz"),
        (".profile", EntryKind::File, ".profile (2)"),
        (".profile.conf", EntryKind::File, ".profile (2).conf"),
        ("plain", EntryKind::File, "plain (2)"),
        ("a..txt", EntryKind::File, "a. (2).txt"),
        ("folder.txt", EntryKind::Folder, "folder.txt (2)"),
        (".folder", EntryKind::Folder, ".folder (2)"),
    ] {
        let planner = f.planner(original, kind);
        assert_eq!(planner.plan().unwrap().basename(), expected);
        assert_eq!(planner.original_basename(), original);
    }
    assert!(
        f.entries().is_empty(),
        "An unused original still gets a suffix, with no writes"
    );
}

#[test]
fn unicode_and_unpaired_utf16_identity_is_lossless() {
    let f = Fixture::new();
    for original in [
        OsString::from("日本-é-\u{10400}.文字"),
        OsString::from_wide(&[120, 0xd800, 121, 0xdc00, 46, 0xdc01, 122, 0xd801]),
    ] {
        let units = wide(&original);
        let dot = units.iter().rposition(|&u| u == 46).unwrap();
        let expected = units[..dot]
            .iter()
            .copied()
            .chain(" (2)".encode_utf16())
            .chain(units[dot..].iter().copied())
            .collect::<Vec<_>>();
        f.file(&original);
        let planner = f.planner(original.clone(), EntryKind::File);
        let candidate = planner.plan().unwrap();
        assert_eq!(wide(planner.original_basename()), units);
        assert_eq!(wide(candidate.basename()), expected);
        assert_eq!(wide(candidate.path().file_name().unwrap()), expected);
        f.file(candidate.basename());
        assert_eq!(planner.plan().unwrap().number(), 3);
    }
}

#[test]
fn component_limit_shortens_only_stem_and_preserves_full_extension() {
    let f = Fixture::new();
    for (original, kind, expected) in [
        (
            "a".repeat(255),
            EntryKind::Folder,
            format!("{} (2)", "a".repeat(251)),
        ),
        (
            format!("{}.txt", "a".repeat(251)),
            EntryKind::File,
            format!("{} (2).txt", "a".repeat(247)),
        ),
        (
            format!("abcd.{}", "e".repeat(250)),
            EntryKind::File,
            format!(" (2).{}", "e".repeat(250)),
        ),
    ] {
        assert_eq!(original.encode_utf16().count(), 255);
        f.file(&original);
        let candidate = f.planner(original, kind).plan().unwrap();
        assert_eq!(candidate.basename(), expected.as_str());
        assert_eq!(wide(candidate.basename()).len(), 255);
    }
}

#[test]
fn truncation_does_not_split_valid_surrogate_pairs_but_keeps_unpaired_units() {
    let f = Fixture::new();
    for (middle, tail, retained) in [
        (vec![0xd801, 0xdc00], 3, vec![]),
        (vec![0xd800], 4, vec![0xd800]),
        (vec![0xdc00], 4, vec![0xdc00]),
    ] {
        let units = std::iter::repeat_n(97, 250)
            .chain(middle)
            .chain(std::iter::repeat_n(98, tail))
            .collect::<Vec<_>>();
        assert_eq!(units.len(), 255);
        let expected = std::iter::repeat_n(97, 250)
            .chain(retained)
            .chain(" (2)".encode_utf16())
            .collect::<Vec<_>>();
        let candidate = f
            .planner(OsString::from_wide(&units), EntryKind::Folder)
            .plan()
            .unwrap();
        assert_eq!(wide(candidate.basename()), expected);
    }
}

#[test]
fn suffix_growth_recomputes_stem_room_and_can_refuse_instead_of_damaging_extension() {
    let f = Fixture::new();
    let planner = f.planner(format!("{}.txt", "a".repeat(251)), EntryKind::File);
    for n in 2..=9 {
        f.file(format!("{} ({n}).txt", "a".repeat(247)));
    }
    let candidate = planner.plan().unwrap();
    assert_eq!(
        candidate.basename(),
        format!("{} (10).txt", "a".repeat(246)).as_str()
    );
    assert_eq!(wide(candidate.basename()).len(), 255);
    let extension = "e".repeat(250);
    let planner = f.planner(format!("abcd.{extension}"), EntryKind::File);
    for n in 2..=9 {
        f.file(format!(" ({n}).{extension}"));
    }
    assert!(matches!(
        planner.plan(),
        Err(PlanError::CannotFit {
            number: 10,
            extension_units: 251
        })
    ));
}

#[test]
fn invalid_names_and_unfittable_extension_perform_zero_io() {
    let before = (
        CAPTURE_COUNT.with(|n| n.get()),
        METADATA_COUNT.with(|n| n.get()),
    );
    let mut rejected = [
        "",
        ".",
        "..",
        "name.",
        "name ",
        "x/y",
        "x\\y",
        "C:relative",
        "x:stream",
        "a\0b",
        "a\u{001f}b",
        "a*b",
        "a?b",
        "a\"b",
        "a<b",
        "a>b",
        "a|b",
        "CON",
        "con.txt",
        "NUL .txt",
        "AUX",
        "PRN",
        "CLOCK$",
        "CONIN$",
        "CONOUT$.txt",
        "COM0",
        "LPT9.txt",
        "com¹.txt",
        "LPT²",
        "COM³",
        "\\\\?\\x",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    rejected.push(OsString::from("x".repeat(256)));
    for name in rejected {
        assert!(matches!(
            Planner::capture(
                name,
                EntryKind::File,
                PathBuf::from("never-opened-relative")
            ),
            Err(PlanError::InvalidName(_))
        ));
    }
    assert!(matches!(
        Planner::capture(
            OsString::from(format!("a.{}", "e".repeat(253))),
            EntryKind::File,
            PathBuf::from("never-opened-relative")
        ),
        Err(PlanError::CannotFit {
            number: 2,
            extension_units: 254
        })
    ));
    assert_eq!(
        (
            CAPTURE_COUNT.with(|n| n.get()),
            METADATA_COUNT.with(|n| n.get())
        ),
        before
    );
}

#[test]
fn ordinary_io_error_and_synthetic_denied_errors_are_not_absence() {
    let f = Fixture::new();
    let planner = f.planner("report.txt", EntryKind::File);
    for code in [5, 32, 1117] {
        let mut calls = 0;
        let result = planner.plan_with_metadata(|_| {
            calls += 1;
            Err(io::Error::from_raw_os_error(code))
        });
        assert_eq!(calls, 1);
        assert!(matches!(result, Err(PlanError::CandidateIo { path, error })
            if path == f.path("report (2).txt") && error.raw_os_error() == Some(code)));
    }
    // An owned overlong component produces a real ERROR_INVALID_NAME metadata
    // error. The private seam injects it; production input admission still refuses this
    // name before I/O. Windows reports a file-as-ancestor as NotFound, not NotADirectory.
    let overlong = f.root.join("x".repeat(256));
    let mut calls = 0;
    let result = planner.plan_with_metadata(|_| {
        calls += 1;
        fs::symlink_metadata(&overlong)
    });
    assert_eq!(calls, 1);
    assert!(
        matches!(&result, Err(PlanError::CandidateIo { error, .. })
        if error.raw_os_error() == Some(123) && error.kind() == io::ErrorKind::InvalidFilename),
        "Actual owned native metadata result: {result:?}"
    );
}

#[test]
fn exhaustion_is_bounded_visible_and_read_only() {
    let f = Fixture::new();
    let planner = f.planner("item", EntryKind::Folder);
    let occupied = f.file("ordinary-metadata-source");
    let before = f.entries();
    let mut calls = 0;
    let result = planner.plan_with_metadata(|path| {
        calls += 1;
        assert_eq!(path, f.path(format!("item ({})", calls + 1)));
        fs::symlink_metadata(&occupied)
    });
    assert!(matches!(
        result,
        Err(PlanError::Exhausted {
            checked: MAX_CANDIDATES
        })
    ));
    assert_eq!(calls, MAX_CANDIDATES);
    assert_eq!(f.entries(), before);
}

#[test]
fn directory_missing_and_replacement_are_detected_before_search() {
    let f = Fixture::new();
    let destination = f.dir("destination");
    let planner = Planner::capture("item".into(), EntryKind::File, destination.clone()).unwrap();
    f.rename(&destination, &f.path("original-destination"));
    let mut calls = 0;
    assert!(matches!(
        planner.plan_with_metadata(|_| {
            calls += 1;
            unreachable!()
        }),
        Err(PlanError::Destination(SnapshotError::Missing))
    ));
    assert_eq!(calls, 0);
    f.dir("destination");
    assert!(matches!(
        planner.plan_with_metadata(|_| {
            calls += 1;
            unreachable!()
        }),
        Err(PlanError::Destination(SnapshotError::Replaced { .. }))
    ));
    assert_eq!(calls, 0);
    assert_eq!(planner.destination(), destination);
}

#[test]
fn directory_disappearing_or_replaced_during_absence_is_not_a_candidate() {
    for replace in [false, true] {
        let f = Fixture::new();
        let destination = f.dir("destination");
        let planner =
            Planner::capture("item".into(), EntryKind::File, destination.clone()).unwrap();
        let mut calls = 0;
        let result = planner.plan_with_metadata(|path| {
            calls += 1;
            assert_eq!(path, destination.join("item (2)"));
            f.rename(&destination, &f.path("original-destination"));
            if replace {
                f.dir("destination");
            }
            Err(io::Error::from_raw_os_error(2))
        });
        assert_eq!(calls, 1);
        if replace {
            assert!(matches!(
                result,
                Err(PlanError::Destination(SnapshotError::Replaced { .. }))
            ));
        } else {
            assert!(matches!(
                result,
                Err(PlanError::Destination(SnapshotError::Missing))
            ));
        }
    }
}

#[test]
fn a_later_creation_is_a_new_conflict_not_a_reservation() {
    let f = Fixture::new();
    let planner = f.planner("report.txt", EntryKind::File);
    let first = planner.plan().unwrap();
    let before = f.entries();
    assert_eq!(
        planner.plan().unwrap(),
        first,
        "Repeated observations do not reserve names"
    );
    assert_eq!(f.entries(), before);
    // Represents another creator after return. No transfer or silent overwrite is tested.
    f.file(first.basename());
    let next = planner.plan().unwrap();
    assert_eq!(next.basename(), "report (3).txt");
    assert_ne!(next.path(), first.path());
    assert!(fs::symlink_metadata(first.path()).unwrap().is_file());
}

#[test]
fn native_observation_not_string_case_folding_decides_occupancy() {
    let f = Fixture::new();
    f.file("REPORT (2).TXT");
    let candidate_spelling = f.path("report (2).txt");
    // Observe this owned directory's actual behavior, without assuming sensitive/insensitive
    // behavior or changing per-directory case flags. No normalization is applied by planner.
    let observed_occupied = match fs::symlink_metadata(&candidate_spelling) {
        Ok(_) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => panic!("Owned case observation failed: {error}"),
    };
    let planner = f.planner("report.txt", EntryKind::File);
    assert_eq!(
        planner.plan().unwrap().number(),
        if observed_occupied { 3 } else { 2 }
    );
    assert_eq!(planner.original_basename(), "report.txt");
    // Canonically equivalent Unicode spellings remain distinct native input, not rewritten.
    for name in ["é.txt", "e\u{0301}.txt"] {
        let planner = f.planner(name, EntryKind::File);
        assert_eq!(
            wide(planner.original_basename()),
            name.encode_utf16().collect::<Vec<_>>()
        );
    }
}

#[test]
fn destination_file_and_unsupported_path_are_refused() {
    let f = Fixture::new();
    let file = f.file("not-directory");
    assert!(matches!(
        Planner::capture("item".into(), EntryKind::File, file),
        Err(PlanError::NotDirectory(ObservedKind::File))
    ));
    assert!(matches!(
        Planner::capture("item".into(), EntryKind::File, PathBuf::from("relative")),
        Err(PlanError::Destination(SnapshotError::InvalidPath(_)))
    ));
}

// Test-only native junction setup uses std's owned File and one kernel32 call. No new
// dependency, shell process, symlink privilege, elevation, or production mutation API.
#[repr(C)]
struct JunctionBuffer {
    tag: u32,
    data_len: u16,
    reserved: u16,
    substitute_offset: u16,
    substitute_len: u16,
    print_offset: u16,
    print_len: u16,
    path: [u16; 1024],
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn DeviceIoControl(
        handle: std::os::windows::raw::HANDLE,
        code: u32,
        input: *const std::ffi::c_void,
        input_size: u32,
        output: *mut std::ffi::c_void,
        output_size: u32,
        returned: *mut u32,
        overlapped: *mut std::ffi::c_void,
    ) -> i32;
}
fn junction(owner: &Fixture, leaf: &Path, target_owner: &Fixture, target: &Path) {
    owner.guard_path(leaf);
    target_owner.guard_path(target);
    assert!(fs::symlink_metadata(leaf).unwrap().is_dir());
    assert!(fs::symlink_metadata(target).unwrap().is_dir());
    let target_units = wide(target.as_os_str());
    assert!(target_units.starts_with(&[92, 92, 63, 92]));
    let print = &target_units[4..];
    let substitute = [92, 63, 63, 92]
        .into_iter()
        .chain(print.iter().copied())
        .collect::<Vec<_>>();
    let print_offset = substitute.len() + 1;
    let count = print_offset + print.len() + 1;
    assert!(count <= 1024);
    let mut buffer = JunctionBuffer {
        tag: 0xa0000003,
        data_len: (8 + count * 2) as u16,
        reserved: 0,
        substitute_offset: 0,
        substitute_len: (substitute.len() * 2) as u16,
        print_offset: (print_offset * 2) as u16,
        print_len: (print.len() * 2) as u16,
        path: [0; 1024],
    };
    buffer.path[..substitute.len()].copy_from_slice(&substitute);
    buffer.path[print_offset..print_offset + print.len()].copy_from_slice(print);
    let file = OpenOptions::new()
        .access_mode(0x40000000)
        .share_mode(7)
        .custom_flags(0x02200000)
        .open(leaf)
        .unwrap();
    let mut returned = 0;
    // SAFETY: live std-owned directory handle, aligned bounded mount-point buffer, exact
    // initialized size, no output or asynchronous operation. Only this fixture leaf changes.
    let ok = unsafe {
        DeviceIoControl(
            file.as_raw_handle(),
            0x000900a4,
            (&buffer as *const JunctionBuffer).cast(),
            8 + buffer.data_len as u32,
            std::ptr::null_mut(),
            0,
            &mut returned,
            std::ptr::null_mut(),
        )
    };
    assert_ne!(
        ok,
        0,
        "Owned nonprivileged junction setup failed: {}",
        io::Error::last_os_error()
    );
}

#[test]
fn dangling_nonprivileged_junction_counts_occupied_and_destination_reparse_is_refused() {
    let f = Fixture::new();
    let external = Fixture::new();
    let target = external.dir("target");
    let sentinel = external.file("unread-sentinel");
    let leaf = f.dir("folder (2)");
    junction(&f, &leaf, &external, &target);
    assert!(matches!(
        Planner::capture("item".into(), EntryKind::File, leaf.clone()),
        Err(PlanError::NotDirectory(ObservedKind::ReparsePoint {
            tag: 0xa0000003
        }))
    ));
    external.remove_dir(&target);
    let meta = fs::symlink_metadata(&leaf).unwrap();
    assert_ne!(meta.file_attributes() & 0x400, 0);
    assert_eq!(
        fs::metadata(&leaf).unwrap_err().kind(),
        io::ErrorKind::NotFound,
        "The owned link is genuinely dangling, not a live directory"
    );
    let before = f.entries();
    let candidate = f.planner("folder", EntryKind::Folder).plan().unwrap();
    assert_eq!(candidate.basename(), "folder (3)");
    assert_eq!(f.entries(), before);
    f.remove_dir(&leaf);
    external.guard();
    assert!(fs::symlink_metadata(sentinel).unwrap().is_file());
}
