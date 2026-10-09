#![cfg(windows)]

use mado_fs_create_rename::{CapturedRename, Error, create_directory, create_empty_file};
use std::{
    ffi::OsStr,
    fs::{self, OpenOptions},
    os::windows::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    sync::{
        Arc, Barrier,
        atomic::{AtomicU64, Ordering},
    },
};
#[cfg(feature = "unreviewed-native-rename")]
use std::{
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
};
#[cfg(feature = "unreviewed-native-rename")]
use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_WRITE;
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_READ_ATTRIBUTES, FILE_SHARE_READ, SYNCHRONIZE,
};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let base = std::env::temp_dir();
        for _ in 0..100 {
            let path = base.join(format!(
                "mado-p02-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("fixture creation: {e}"),
            }
        }
        panic!("could not reserve fixture directory");
    }
    fn path(&self) -> &Path {
        &self.0
    }
    fn entry(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!(
                "Disposable fixture cleanup failed at {}: {error}",
                self.0.display()
            );
        }
    }
}

#[cfg(feature = "unreviewed-native-rename")]
fn rename(source: &Path, name: &str) -> Result<PathBuf, Error> {
    CapturedRename::capture(source)?.rename(OsStr::new(name))
}

#[test]
fn new_hardlinks_are_possible_despite_exclusive_source_lock() {
    let f = Fixture::new();
    fs::write(f.entry("source"), b"keep").unwrap();
    let lock = OpenOptions::new()
        .access_mode(FILE_READ_ATTRIBUTES | DELETE | SYNCHRONIZE)
        .share_mode(FILE_SHARE_READ)
        .open(f.entry("source"))
        .unwrap();
    // Observed NTFS behavior. Neither delete nor write sharing denial prevents this.
    fs::hard_link(f.entry("source"), f.entry("alias")).unwrap();
    drop(lock);
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 2);
    assert_eq!(fs::read(f.entry("source")).unwrap(), b"keep");
}

#[cfg(not(feature = "unreviewed-native-rename"))]
#[test]
fn default_rename_is_explicitly_blocked_without_mutation() {
    let f = Fixture::new();
    fs::write(f.entry("source"), b"keep").unwrap();
    let capture = CapturedRename::capture(&f.entry("source")).unwrap();
    assert!(matches!(
        capture.rename(OsStr::new("destination")),
        Err(Error::RenameSafetyBlocked)
    ));
    assert_eq!(fs::read(f.entry("source")).unwrap(), b"keep");
    assert!(!f.entry("destination").exists());
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 1);
}

#[test]
fn create_empty_file_and_directory_and_reject_all_collisions() {
    let f = Fixture::new();
    let file = create_empty_file(f.path(), OsStr::new("empty.txt")).unwrap();
    assert_eq!(fs::read(&file).unwrap(), b"");
    fs::write(&file, b"keep contents").unwrap();
    let dir = create_directory(f.path(), OsStr::new("folder")).unwrap();
    fs::write(dir.join("child.txt"), b"keep child").unwrap();
    for name in ["empty.txt", "folder"] {
        assert!(create_empty_file(f.path(), OsStr::new(name)).is_err());
        assert!(create_directory(f.path(), OsStr::new(name)).is_err());
    }
    assert_eq!(fs::read(file).unwrap(), b"keep contents");
    assert_eq!(fs::read(dir.join("child.txt")).unwrap(), b"keep child");
}

#[test]
fn invalid_creation_never_touches_disk() {
    let f = Fixture::new();
    for name in [
        "",
        ".",
        "..",
        "../escape",
        "x:y",
        "CON",
        "NUL.txt",
        "trailing.",
        "trailing ",
        "\\\\?\\GLOBALROOT",
    ] {
        assert!(matches!(
            create_empty_file(f.path(), OsStr::new(name)),
            Err(Error::InvalidName(_))
        ));
        assert!(matches!(
            create_directory(f.path(), OsStr::new(name)),
            Err(Error::InvalidName(_))
        ));
    }
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 0);
}

#[test]
fn creation_is_atomic_against_concurrent_creators() {
    let f = Fixture::new();
    let barrier = Arc::new(Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let parent = f.0.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                create_empty_file(&parent, OsStr::new("only-one")).is_ok()
            })
        })
        .collect();
    assert_eq!(
        workers
            .into_iter()
            .filter_map(|worker| worker.join().unwrap().then_some(()))
            .count(),
        1
    );
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 1);
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn rename_preserves_file_bytes_and_nonempty_directory() {
    let f = Fixture::new();
    fs::write(f.entry("source.bin"), b"\0binary\xffcontents").unwrap();
    let result = rename(&f.entry("source.bin"), "result.bin").unwrap();
    assert!(!f.entry("source.bin").exists());
    assert_eq!(fs::read(result).unwrap(), b"\0binary\xffcontents");
    fs::create_dir(f.entry("folder")).unwrap();
    fs::write(f.entry("folder").join("child"), b"child bytes").unwrap();
    let result = rename(&f.entry("folder"), "renamed-folder").unwrap();
    assert!(!f.entry("folder").exists());
    assert_eq!(fs::read(result.join("child")).unwrap(), b"child bytes");
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn late_file_directory_and_hardlink_collisions_preserve_every_entry() {
    let f = Fixture::new();
    for target_kind in 0..3 {
        let source = f.entry(&format!("source-{target_kind}"));
        let destination = f.entry(&format!("destination-{target_kind}"));
        fs::write(&source, b"source bytes").unwrap();
        let capture = CapturedRename::capture(&source).unwrap();
        match target_kind {
            0 => fs::write(&destination, b"destination bytes").unwrap(),
            1 => {
                fs::create_dir(&destination).unwrap();
                fs::write(destination.join("child"), b"child").unwrap();
            }
            _ => fs::hard_link(&source, &destination).unwrap(),
        }
        let result = capture.rename(destination.file_name().unwrap());
        eprintln!(
            "collision {target_kind} result={result:?}, source_exists={}, destination_exists={}",
            source.exists(),
            destination.exists()
        );
        assert!(result.is_err(), "collision kind {target_kind}");
        assert_eq!(fs::read(&source).unwrap(), b"source bytes");
        match target_kind {
            0 => assert_eq!(fs::read(&destination).unwrap(), b"destination bytes"),
            1 => assert_eq!(fs::read(destination.join("child")).unwrap(), b"child"),
            _ => assert_eq!(fs::read(&destination).unwrap(), b"source bytes"),
        }
    }
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn disappeared_source_is_not_retargeted() {
    let f = Fixture::new();
    fs::write(f.entry("source"), b"source bytes").unwrap();
    let capture = CapturedRename::capture(&f.entry("source")).unwrap();
    fs::remove_file(f.entry("source")).unwrap();
    assert!(capture.rename(OsStr::new("destination")).is_err());
    assert!(!f.entry("destination").exists());
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 0);
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn replacement_identity_is_rejected_without_mutating_either_file() {
    let f = Fixture::new();
    fs::write(f.entry("source"), b"captured bytes").unwrap();
    let capture = CapturedRename::capture(&f.entry("source")).unwrap();
    fs::rename(f.entry("source"), f.entry("parked-original")).unwrap();
    fs::write(f.entry("source"), b"replacement bytes").unwrap();
    assert!(matches!(
        capture.rename(OsStr::new("destination")),
        Err(Error::SourceReplaced(_))
    ));
    assert_eq!(
        fs::read(f.entry("parked-original")).unwrap(),
        b"captured bytes"
    );
    assert_eq!(fs::read(f.entry("source")).unwrap(), b"replacement bytes");
    assert!(!f.entry("destination").exists());
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn locked_source_and_invalid_rename_preserve_data_without_temporary_names() {
    let f = Fixture::new();
    fs::write(f.entry("source"), b"source bytes").unwrap();
    let capture = CapturedRename::capture(&f.entry("source")).unwrap();
    let lock = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .open(f.entry("source"))
        .unwrap();
    assert!(capture.rename(OsStr::new("destination")).is_err());
    drop(lock);
    assert_eq!(fs::read(f.entry("source")).unwrap(), b"source bytes");
    let capture = CapturedRename::capture(&f.entry("source")).unwrap();
    assert!(matches!(
        capture.rename(OsStr::new("NUL")),
        Err(Error::InvalidName(_))
    ));
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 1);
    assert_eq!(fs::read(f.entry("source")).unwrap(), b"source bytes");
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn case_only_rename_and_same_name_leave_one_entry_with_requested_spelling() {
    let f = Fixture::new();
    fs::write(f.entry("Case.txt"), b"keep").unwrap();
    let result = rename(&f.entry("Case.txt"), "case.TXT").unwrap();
    assert_eq!(fs::read(&result).unwrap(), b"keep");
    let names: Vec<_> = fs::read_dir(f.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(names, [OsString::from("case.TXT")]);
    rename(&result, "case.TXT").unwrap();
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 1);
}

#[test]
fn ancestors_are_pinned_and_capture_drop_does_not_rename_source() {
    let f = Fixture::new();
    fs::create_dir(f.entry("parent")).unwrap();
    let source = f.entry("parent").join("source");
    fs::write(&source, b"keep").unwrap();
    let capture = CapturedRename::capture(&source).unwrap();
    assert!(fs::rename(f.entry("parent"), f.entry("retargeted")).is_err());
    drop(capture);
    assert_eq!(fs::read(&source).unwrap(), b"keep");
    fs::rename(f.entry("parent"), f.entry("retargeted")).unwrap();
}

#[test]
fn unsupported_namespace_relative_dot_and_ads_paths_fail_before_mutation() {
    let f = Fixture::new();
    for path in [
        "relative",
        "C:",
        "\\root-relative",
        "\\\\.\\C:\\",
        "\\\\?\\GLOBALROOT\\Device\\HarddiskVolume1\\",
        "\\\\?\\Volume{00000000-0000-0000-0000-000000000000}\\",
    ] {
        assert!(
            matches!(
                create_empty_file(Path::new(path), OsStr::new("no-write")),
                Err(Error::UnsupportedPath(_))
            ),
            "{path}"
        );
    }
    for path in [
        f.path().join("."),
        f.path().join(".."),
        f.path().join("stream:ads"),
    ] {
        assert!(
            matches!(
                create_empty_file(&path, OsStr::new("no-write")),
                Err(Error::UnsupportedPath(_))
            ),
            "{}",
            path.display()
        );
    }
    assert!(CapturedRename::capture(Path::new("C:\\")).is_err());
    assert_eq!(fs::read_dir(f.path()).unwrap().count(), 0);
}

#[cfg(feature = "unreviewed-native-rename")]
#[test]
fn native_unicode_nonunicode_and_long_paths_are_preserved() {
    let f = Fixture::new();
    let initial = OsStr::new("résumé-資料-𐐀.txt");
    let path = create_empty_file(f.path(), initial).unwrap();
    fs::write(&path, b"unicode").unwrap();
    let native = OsString::from_wide(&[0xd800, 0x0061, 0x002e, 0x0074, 0x0078, 0x0074]);
    let result = CapturedRename::capture(&path)
        .unwrap()
        .rename(&native)
        .unwrap();
    assert_eq!(result.file_name().unwrap(), native);
    assert_eq!(fs::read(&result).unwrap(), b"unicode");
    let mut parent = f.0.clone();
    for i in 0..8 {
        parent = create_directory(
            &parent,
            OsStr::new(&format!("long-component-{i}-abcdefghijklmnopqrstuv")),
        )
        .unwrap();
    }
    assert!(parent.as_os_str().encode_wide().count() > 260);
    let source = create_empty_file(&parent, initial).unwrap();
    let renamed = rename(&source, "long-result.txt").unwrap();
    assert_eq!(fs::read(renamed).unwrap(), b"");
}

#[test]
#[ignore = "Requires Windows symlink privilege or Developer Mode; run explicitly"]
fn reparse_source_and_ancestors_are_rejected_without_touching_target() {
    let f = Fixture::new();
    let outside = Fixture::new();
    fs::write(outside.entry("sentinel"), b"untouched").unwrap();
    let link = f.entry("link");
    std::os::windows::fs::symlink_dir(outside.path(), &link)
        .expect("symlink fixture needs privilege or Developer Mode");
    assert!(matches!(
        create_empty_file(&link, OsStr::new("escape")),
        Err(Error::ReparsePoint(_))
    ));
    assert!(matches!(
        create_directory(&link, OsStr::new("escape")),
        Err(Error::ReparsePoint(_))
    ));
    assert!(matches!(
        CapturedRename::capture(&link),
        Err(Error::ReparsePoint(_))
    ));
    assert!(CapturedRename::capture(&link.join("sentinel")).is_err());
    assert_eq!(fs::read(outside.entry("sentinel")).unwrap(), b"untouched");
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 1);
    assert!(link.is_symlink());
}
