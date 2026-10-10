use super::*;
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    os::windows::{
        ffi::OsStringExt,
        fs::{MetadataExt, OpenOptionsExt},
    },
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc, Barrier,
        atomic::{AtomicU64, Ordering},
    },
};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_BACKUP_SEMANTICS, FILE_READ_ATTRIBUTES,
    FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_WRITE_ATTRIBUTES, FILE_WRITE_DATA,
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/p18/fixtures");
        fs::create_dir_all(&base).unwrap();
        for _ in 0..100 {
            let path = base.join(format!(
                "owned-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(fs::canonicalize(path).unwrap()),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("reserve owned fixture: {e}"),
            }
        }
        panic!("fixture namespace exhausted");
    }
    fn entry(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
    fn capture(&self) -> CapturedDirectory {
        CapturedDirectory::capture(self.0.clone()).unwrap()
    }
    fn names(&self) -> Vec<OsString> {
        names(&self.0)
    }
}
fn names(path: &Path) -> Vec<OsString> {
    let mut result: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    result.sort();
    result
}
// Fixture-owned cleanup only. Never traverse a reparse entry, including dangling junctions.
fn remove_owned(path: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        if metadata.file_attributes() & 16 != 0 {
            fs::remove_dir(path)
        } else {
            fs::remove_file(path)
        }
    } else if metadata.is_dir() {
        for e in fs::read_dir(path)? {
            remove_owned(&e?.path())?;
        }
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Err(e) = remove_owned(&self.0) {
            eprintln!("Owned fixture retained at {}: {e}", self.0.display());
        }
    }
}
fn assert_io_code(error: Error, codes: &[i32]) {
    match error {
        Error::Io { source, .. } => {
            assert!(codes.contains(&source.raw_os_error().unwrap()), "{source}")
        }
        other => panic!("Expected real native I/O failure, got {other:?}"),
    }
}
fn junction(path: &Path, target: &Path) {
    // Generated paths below the checkout contain no shell metacharacters. Strip verbatim
    // prefixes for cmd's mklink facility only, never for backend calls.
    let ordinary = |p: &Path| {
        p.as_os_str()
            .to_str()
            .unwrap()
            .strip_prefix("\\\\?\\")
            .unwrap()
            .to_owned()
    };
    let result = Command::new("cmd.exe")
        .args(["/D", "/C", "mklink", "/J"])
        .arg(ordinary(path))
        .arg(ordinary(target))
        .output()
        .unwrap();
    assert!(result.status.success(), "mklink /J: {result:?}");
    assert_ne!(
        fs::symlink_metadata(path).unwrap().file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT,
        0
    );
}

#[test]
fn actual_empty_file_and_folder_creation_with_original_reporting_path() {
    let f = Fixture::new();
    let ordinary = PathBuf::from(
        f.0.as_os_str()
            .to_str()
            .unwrap()
            .strip_prefix("\\\\?\\")
            .unwrap()
            .replace('\\', "/"),
    );
    let c = CapturedDirectory::capture(ordinary.clone()).unwrap();
    assert_eq!(c.original_path().as_os_str(), ordinary.as_os_str());
    let file = c.create_empty_file(OsStr::new("empty.txt")).unwrap();
    assert_eq!(file, ordinary.join("empty.txt"));
    assert_eq!(fs::read(file).unwrap(), b"");
    let folder = c.create_directory(OsStr::new("folder")).unwrap();
    assert!(folder.is_dir());
    assert!(names(&folder).is_empty());
    assert_eq!(
        f.names(),
        [OsString::from("empty.txt"), OsString::from("folder")]
    );
}
#[test]
fn existing_file_folder_hardlink_and_case_collisions_preserve_names_and_bytes() {
    let f = Fixture::new();
    fs::write(f.entry("Case.txt"), b"keep bytes").unwrap();
    fs::hard_link(f.entry("Case.txt"), f.entry("alias")).unwrap();
    fs::create_dir(f.entry("folder")).unwrap();
    fs::write(f.entry("folder").join("child"), b"keep child").unwrap();
    let before = f.names();
    let c = f.capture();
    for name in ["Case.txt", "case.TXT", "alias", "folder"] {
        for directory in [false, true] {
            let result = if directory {
                c.create_directory(OsStr::new(name))
            } else {
                c.create_empty_file(OsStr::new(name))
            };
            // NTFS reports access denied for some type-mismatch collisions, not
            // necessarily AlreadyExists. Require refusal and preserve all data below.
            assert_io_code(result.unwrap_err(), &[5, 80, 183]);
        }
    }
    assert_eq!(f.names(), before);
    for name in ["Case.txt", "alias"] {
        assert_eq!(fs::read(f.entry(name)).unwrap(), b"keep bytes");
    }
    assert_eq!(
        fs::read(f.entry("folder").join("child")).unwrap(),
        b"keep child"
    );
}
#[test]
fn competing_file_and_folder_creators_have_exactly_one_success() {
    let f = Fixture::new();
    let barrier = Arc::new(Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|i| {
            let c = f.capture();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                if i % 2 == 0 {
                    c.create_directory(OsStr::new("one"))
                } else {
                    c.create_empty_file(OsStr::new("one"))
                }
            })
        })
        .collect();
    let mut successes = 0;
    for w in workers {
        match w.join().unwrap() {
            Ok(_) => successes += 1,
            // A folder winning can make file losers report type-mismatch access denied.
            Err(e) => assert_io_code(e, &[5, 80, 183]),
        }
    }
    assert_eq!(successes, 1);
    assert_eq!(f.names(), [OsString::from("one")]);
}
#[test]
fn capture_pins_target_and_ancestors_against_replacement_rename_delete_and_reparse_writes() {
    let f = Fixture::new();
    fs::create_dir(f.entry("parent")).unwrap();
    let target = f.entry("parent").join("target");
    fs::create_dir(&target).unwrap();
    let c = CapturedDirectory::capture(target.clone()).unwrap();
    let original_id = c.identity();
    for (from, to) in [
        (&target, f.entry("parked")),
        (&f.entry("parent"), f.entry("moved-parent")),
    ] {
        let e = fs::rename(from, to).unwrap_err();
        assert_eq!(e.raw_os_error(), Some(32));
    }
    assert_eq!(
        fs::remove_dir(&target).unwrap_err().raw_os_error(),
        Some(32)
    );
    // Data-write access required for setting a junction is denied by the pin.
    // Windows share checks do not deny attribute-only handles.
    let write = OpenOptions::new()
        .access_mode(FILE_WRITE_DATA | FILE_WRITE_ATTRIBUTES)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(&target)
        .unwrap_err();
    assert_eq!(write.raw_os_error(), Some(32));
    c.create_empty_file(OsStr::new("bound")).unwrap();
    assert_eq!(fs::read(target.join("bound")).unwrap(), b"");
    drop(c);
    fs::rename(&target, f.entry("parked")).unwrap();
    fs::create_dir(&target).unwrap();
    let replacement = CapturedDirectory::capture(target.clone()).unwrap();
    assert_ne!(replacement.identity(), original_id);
    replacement
        .create_directory(OsStr::new("new-folder"))
        .unwrap();
    assert_eq!(names(&f.entry("parked")), [OsString::from("bound")]);
    assert_eq!(names(&target), [OsString::from("new-folder")]);
    drop(replacement);
    fs::rename(f.entry("parent"), f.entry("moved-parent")).unwrap();
}
// Native mount-point buffer for an owned target. Positive use after releasing pins
// proves negative conversion checks are not merely rejecting malformed fixture data.
fn set_junction(file: &std::fs::File, target: &Path) -> io::Result<()> {
    use std::{os::windows::io::AsRawHandle, ptr};
    use windows_sys::Win32::System::IO::DeviceIoControl;
    let mut substitute: Vec<u16> = "\\??\\".encode_utf16().collect();
    substitute.extend(target.as_os_str().encode_wide().skip(4));
    let length = (substitute.len() * 2) as u16;
    let mut buffer = Vec::new();
    buffer.extend(0xa0000003u32.to_le_bytes()); // IO_REPARSE_TAG_MOUNT_POINT
    buffer.extend((8 + length + 4).to_le_bytes());
    buffer.extend(0u16.to_le_bytes()); // reserved
    buffer.extend(0u16.to_le_bytes()); // substitute offset
    buffer.extend(length.to_le_bytes());
    buffer.extend((length + 2).to_le_bytes()); // empty print name offset
    buffer.extend(0u16.to_le_bytes()); // empty print name length
    for unit in substitute {
        buffer.extend(unit.to_le_bytes());
    }
    buffer.extend([0; 4]); // substitute and print terminators
    let mut returned = 0;
    // SAFETY: bounded complete mount-point buffer and live synchronous fixture-only
    // handle. The ioctl copies its input; no output/overlapped buffer is supplied.
    let result = unsafe {
        DeviceIoControl(
            file.as_raw_handle(),
            0x000900a4,
            buffer.as_ptr().cast(),
            buffer.len() as u32,
            ptr::null_mut(),
            0,
            &mut returned,
            ptr::null_mut(),
        )
    };
    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
#[test]
fn native_relative_create_refuses_target_converted_to_reparse_after_capture() {
    let f = Fixture::new();
    let outside = Fixture::new();
    fs::write(outside.entry("sentinel"), b"untouched").unwrap();
    fs::create_dir(f.entry("target")).unwrap();
    let c = CapturedDirectory::capture(f.entry("target")).unwrap();
    let attrs = OpenOptions::new()
        .access_mode(FILE_WRITE_ATTRIBUTES)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(f.entry("target"))
        .unwrap();
    // Real observed Windows behavior: write sharing denial does not prevent this.
    set_junction(&attrs, &outside.0).unwrap();
    assert!(matches!(
        c.create_empty_file(OsStr::new("must-not-create")),
        Err(Error::ReparsePoint(_))
    ));
    assert!(matches!(
        c.create_directory(OsStr::new("must-not-create")),
        Err(Error::ReparsePoint(_))
    ));
    assert_eq!(outside.names(), [OsString::from("sentinel")]);
    drop(attrs);
    drop(c);
    assert_ne!(
        fs::symlink_metadata(f.entry("target"))
            .unwrap()
            .file_attributes()
            & FILE_ATTRIBUTE_REPARSE_POINT,
        0
    );
    assert_eq!(fs::read(outside.entry("sentinel")).unwrap(), b"untouched");
    fs::remove_dir(f.entry("target")).unwrap();
}
#[test]
fn reparse_conversion_after_last_observation_is_refused_by_native_create() {
    let f = Fixture::new();
    let outside = Fixture::new();
    fs::write(outside.entry("sentinel"), b"untouched").unwrap();
    for directory in [false, true] {
        let path = f.entry(if directory {
            "folder-target"
        } else {
            "file-target"
        });
        fs::create_dir(&path).unwrap();
        let c = CapturedDirectory::capture(path.clone()).unwrap();
        let attrs = OpenOptions::new()
            .access_mode(FILE_WRITE_ATTRIBUTES)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
            .open(&path)
            .unwrap();
        let external = outside.0.clone();
        windows::BEFORE_CREATE.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move || {
                set_junction(&attrs, &external).unwrap();
            }))
        });
        let result = if directory {
            c.create_directory(OsStr::new("must-not-create"))
        } else {
            c.create_empty_file(OsStr::new("must-not-create"))
        };
        // OBJ_DONT_REPARSE protects the native mutation, not the earlier attribute checks.
        eprintln!("Post-check target conversion, directory={directory}: {result:?}");
        assert_io_code(result.unwrap_err(), &[1921]);
        assert_eq!(outside.names(), [OsString::from("sentinel")]);
        assert_eq!(fs::read(outside.entry("sentinel")).unwrap(), b"untouched");
        drop(c);
        fs::remove_dir(path).unwrap();
    }
}
#[test]
fn nonempty_pinned_ancestor_junction_conversion_is_refused_natively() {
    let f = Fixture::new();
    let outside = Fixture::new();
    let ancestor = f.entry("ancestor");
    fs::create_dir(&ancestor).unwrap();
    let target = ancestor.join("target");
    fs::create_dir(&target).unwrap();
    fs::create_dir(outside.entry("target")).unwrap();
    fs::write(outside.entry("target").join("sentinel"), b"untouched").unwrap();
    let c = CapturedDirectory::capture(target.clone()).unwrap();
    let attrs = OpenOptions::new()
        .access_mode(FILE_WRITE_ATTRIBUTES)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(&ancestor)
        .unwrap();
    let attrs = Arc::new(attrs);
    let hook_attrs = attrs.clone();
    let external = outside.0.clone();
    windows::BEFORE_CREATE.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move || {
            let error = set_junction(&hook_attrs, &external).unwrap_err();
            eprintln!("Nonempty pinned ancestor junction conversion: {error}");
            assert_eq!(error.raw_os_error(), Some(145));
        }))
    });
    c.create_empty_file(OsStr::new("bound-child")).unwrap();
    // Real Win32 145 evidence, not coverage of a successfully redirected ancestor.
    // No privileged/provider-dependent ancestor conversion is attempted.
    assert_eq!(
        names(&outside.entry("target")),
        [OsString::from("sentinel")]
    );
    assert_eq!(
        fs::read(outside.entry("target").join("sentinel")).unwrap(),
        b"untouched"
    );
    assert_eq!(
        fs::symlink_metadata(&ancestor).unwrap().file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT,
        0
    );
    assert_eq!(names(&target), [OsString::from("bound-child")]);
    assert_eq!(fs::read(target.join("bound-child")).unwrap(), b"");
    assert!(outside.names().iter().all(|name| name == "target"));
}
#[test]
fn disappearance_non_directory_and_real_locking_fail_without_new_entries() {
    let f = Fixture::new();
    fs::write(f.entry("sentinel"), b"keep").unwrap();
    assert_io_code(
        CapturedDirectory::capture(f.entry("missing")).unwrap_err(),
        &[2, 3],
    );
    assert!(CapturedDirectory::capture(f.entry("sentinel")).is_err());
    fs::create_dir(f.entry("target")).unwrap();
    let lock = OpenOptions::new()
        .access_mode(FILE_READ_ATTRIBUTES | DELETE)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(f.entry("target"))
        .unwrap();
    assert_io_code(
        CapturedDirectory::capture(f.entry("target")).unwrap_err(),
        &[32],
    );
    drop(lock);
    assert!(names(&f.entry("target")).is_empty());
    assert_eq!(fs::read(f.entry("sentinel")).unwrap(), b"keep");
    assert!(CapturedDirectory::capture(f.entry("target")).is_ok());
}
#[test]
fn invalid_names_are_lossless_zero_io_rejections() {
    let f = Fixture::new();
    let c = f.capture();
    let mut invalid: Vec<OsString> = [
        "",
        ".",
        "..",
        "x/child",
        "x\\child",
        "x:y",
        "x?",
        "x*",
        "x<",
        "x>",
        "x|",
        "x\"",
        "x\0",
        "x\n",
        "name.",
        "name ",
        "CON",
        "nul.txt",
        "con .txt",
        "PRN",
        "AUX",
        "COM0",
        "COM1",
        "Lpt9.log",
        "COM¹",
        "LPT².txt",
        "COM³",
        "CONIN$",
        "conout$.txt",
        "CLOCK$",
    ]
    .into_iter()
    .map(OsString::from)
    .collect();
    invalid.extend([
        OsString::from_wide(&[127]),
        OsString::from_wide(&[159]),
        OsString::from_wide(&[97; 256]),
    ]);
    let before = windows::IO_COUNT.with(|n| n.get());
    for name in invalid {
        assert!(
            matches!(c.create_empty_file(&name), Err(Error::InvalidName(_))),
            "{name:?}"
        );
        assert!(
            matches!(c.create_directory(&name), Err(Error::InvalidName(_))),
            "{name:?}"
        );
    }
    assert_eq!(windows::IO_COUNT.with(|n| n.get()), before);
    assert!(f.names().is_empty());
}
#[test]
fn invalid_namespaces_and_aliases_are_zero_open_rejections() {
    let f = Fixture::new();
    let mut paths: Vec<PathBuf> = [
        "",
        "relative",
        "C:",
        "\\root-relative",
        "\\\\.\\C:\\",
        "\\\\.\\pipe\\name",
        "\\??\\C:\\",
        "\\\\?\\GLOBALROOT\\Device\\HarddiskVolume1\\",
        "\\\\?\\Volume{00000000-0000-0000-0000-000000000000}\\",
        "\\\\server\\pipe\\name",
        "\\\\?\\UNC\\server\\PiPe\\name",
        "\\\\server\\pipe.\\name",
        "\\\\server\\pipe \\name",
        "\\\\?\\C:/name",
        "C:\\NUL\\x",
        "C:\\foo\\\\bar",
        "C:\\name\0\\x",
    ]
    .into_iter()
    .map(PathBuf::from)
    .collect();
    let exact_suffix = |suffix: &str| {
        let mut value = f.0.as_os_str().to_owned();
        value.push(suffix);
        PathBuf::from(value)
    };
    paths.extend([
        exact_suffix("\\."),
        exact_suffix("\\.."),
        f.0.join("stream:ads"),
        f.0.join("trailing."),
        f.0.join("trailing "),
    ]);
    paths.push(PathBuf::from(OsString::from_wide(&vec![97; 32761])));
    let before = windows::IO_COUNT.with(|n| n.get());
    for path in paths {
        assert!(
            matches!(
                CapturedDirectory::capture(path.clone()),
                Err(Error::UnsupportedPath(_))
            ),
            "{path:?}"
        );
    }
    assert_eq!(windows::IO_COUNT.with(|n| n.get()), before);
    assert!(f.names().is_empty());
    // Pure lexical UNC compatibility only. No network or named-pipe provider is contacted.
    for p in [
        "\\\\NUL\\CON\\folder",
        "\\\\?\\unc\\host\\share\\folder",
        "//host/share/folder",
    ] {
        windows::lexical_path(Path::new(p)).unwrap();
        assert!(matches!(
            CapturedDirectory::capture(PathBuf::from(p)),
            Err(Error::UnsupportedProvider(_))
        ));
    }
    assert_eq!(windows::IO_COUNT.with(|n| n.get()), before);
}
#[test]
fn local_ntfs_admission_refuses_remote_unknown_and_other_filesystems() {
    use windows_sys::{
        Wdk::System::SystemServices::{
            FILE_FS_DEVICE_INFORMATION, FILE_REMOTE_DEVICE, FILE_REMOTE_DEVICE_VSMB,
        },
        Win32::Storage::FileSystem::FILE_DEVICE_DISK,
    };
    let mut device = FILE_FS_DEVICE_INFORMATION {
        DeviceType: FILE_DEVICE_DISK,
        Characteristics: 0,
    };
    assert!(windows::local_ntfs(
        &device,
        &"NTFS".encode_utf16().collect::<Vec<_>>()
    ));
    for fs in ["", "FAT32", "ReFS", "NTFS-extra"] {
        assert!(!windows::local_ntfs(
            &device,
            &fs.encode_utf16().collect::<Vec<_>>()
        ));
    }
    for flags in [
        FILE_REMOTE_DEVICE,
        FILE_REMOTE_DEVICE_VSMB,
        FILE_REMOTE_DEVICE | FILE_REMOTE_DEVICE_VSMB,
    ] {
        device.Characteristics = flags;
        assert!(!windows::local_ntfs(
            &device,
            &"NTFS".encode_utf16().collect::<Vec<_>>()
        ));
    }
    device.Characteristics = 0;
    device.DeviceType = 0;
    assert!(!windows::local_ntfs(
        &device,
        &"NTFS".encode_utf16().collect::<Vec<_>>()
    ));
}
#[test]
fn unicode_unpaired_surrogates_and_component_limit_work_without_loss() {
    let f = Fixture::new();
    let c = f.capture();
    for name in [
        OsString::from("résumé-資料-𐐀.txt"),
        OsString::from_wide(&[0xd800, 97, 0xdc00]),
        OsString::from_wide(&[97; 255]),
        OsString::from(".gitignore"),
        OsString::from("a..b"),
    ] {
        let result = c.create_empty_file(&name).unwrap();
        assert_eq!(result.file_name().unwrap(), name);
        assert_eq!(fs::read(result).unwrap(), b"");
        assert!(f.names().contains(&name));
    }
    let native_folder = OsString::from_wide(&[0xdc00, 98]);
    let path = c.create_directory(&native_folder).unwrap();
    let target = CapturedDirectory::capture(path.clone()).unwrap();
    assert_eq!(target.original_path().as_os_str(), path.as_os_str());
    target.create_empty_file(OsStr::new("child")).unwrap();
}
#[test]
fn ordinary_and_verbatim_long_paths_bind_same_directory() {
    let f = Fixture::new();
    let mut parent = f.0.clone();
    for i in 0..8 {
        parent = CapturedDirectory::capture(parent)
            .unwrap()
            .create_directory(OsStr::new(&format!(
                "long-component-{i}-abcdefghijklmnopqrstuv"
            )))
            .unwrap();
    }
    assert!(parent.as_os_str().encode_wide().count() > 260);
    let ordinary = PathBuf::from(
        parent
            .as_os_str()
            .to_str()
            .unwrap()
            .strip_prefix("\\\\?\\")
            .unwrap(),
    );
    let v = CapturedDirectory::capture(parent.clone()).unwrap();
    let o = CapturedDirectory::capture(ordinary.clone()).unwrap();
    assert_eq!(v.identity(), o.identity());
    assert_eq!(o.original_path(), ordinary);
    o.create_empty_file(OsStr::new("ordinary")).unwrap();
    v.create_directory(OsStr::new("verbatim")).unwrap();
    assert_eq!(fs::read(parent.join("ordinary")).unwrap(), b"");
    assert!(parent.join("verbatim").is_dir());
}
#[test]
fn junction_and_dangling_collisions_and_reparse_ancestors_preserve_owned_sentinel() {
    let f = Fixture::new();
    let outside = Fixture::new();
    fs::write(outside.entry("sentinel"), b"untouched").unwrap();
    fs::create_dir(outside.entry("nested")).unwrap();
    junction(&f.entry("junction"), &outside.0);
    junction(&f.entry("dangling"), &outside.entry("missing-target"));
    let before = f.names();
    let external_before = outside.names();
    let c = f.capture();
    for name in ["junction", "dangling"] {
        assert_io_code(
            c.create_empty_file(OsStr::new(name)).unwrap_err(),
            &[5, 80, 183],
        );
        assert_io_code(
            c.create_directory(OsStr::new(name)).unwrap_err(),
            &[5, 80, 183],
        );
        assert!(matches!(
            CapturedDirectory::capture(f.entry(name)),
            Err(Error::ReparsePoint(_))
        ));
    }
    assert!(matches!(
        CapturedDirectory::capture(f.entry("junction").join("nested")),
        Err(Error::ReparsePoint(_))
    ));
    assert_eq!(f.names(), before);
    assert_eq!(outside.names(), external_before);
    assert_eq!(fs::read(outside.entry("sentinel")).unwrap(), b"untouched");
    assert!(names(&outside.entry("nested")).is_empty());
    // Explicitly remove links before dropping the sentinel fixture, even though cleanup
    // also refuses recursive reparse traversal.
    fs::remove_dir(f.entry("junction")).unwrap();
    fs::remove_dir(f.entry("dangling")).unwrap();
}
#[test]
#[ignore = "Requires symlink privilege or Developer Mode; pending, run only on owned fixtures"]
fn privileged_file_symlink_and_dangling_symlink_collisions_preserve_target() {
    let f = Fixture::new();
    let outside = Fixture::new();
    fs::write(outside.entry("sentinel"), b"untouched").unwrap();
    for (name, target) in [
        ("link", outside.entry("sentinel")),
        ("dangling", outside.entry("missing")),
    ] {
        std::os::windows::fs::symlink_file(target, f.entry(name))
            .expect("privileged owned symlink setup");
    }
    let c = f.capture();
    let before = f.names();
    for name in ["link", "dangling"] {
        assert_io_code(
            c.create_empty_file(OsStr::new(name)).unwrap_err(),
            &[5, 80, 183],
        );
        assert_io_code(
            c.create_directory(OsStr::new(name)).unwrap_err(),
            &[5, 80, 183],
        );
    }
    assert_eq!(f.names(), before);
    assert_eq!(fs::read(outside.entry("sentinel")).unwrap(), b"untouched");
}
