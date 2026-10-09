#![cfg(windows)]

// Disposable observations only. These calls do not change the production rename safety gate.
use mado_fs_create_rename::{CapturedRename, Error, create_directory, create_empty_file};
use std::{
    ffi::{OsStr, OsString},
    fs::{self, File, OpenOptions},
    mem::{offset_of, size_of},
    os::windows::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
    },
    path::{Path, PathBuf},
    process::Command,
    ptr,
    sync::atomic::{AtomicU64, Ordering},
};
use windows_sys::{
    Wdk::Storage::FileSystem::{
        FILE_INFORMATION_CLASS, FILE_RENAME_INFORMATION, FILE_RENAME_POSIX_SEMANTICS,
        FILE_RENAME_REPLACE_IF_EXISTS, FileRenameInformation, FileRenameInformationEx,
        NtSetInformationFile,
    },
    Win32::{
        Foundation::RtlNtStatusToDosError,
        Storage::FileSystem::{
            DELETE, FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_BACKUP_SEMANTICS,
            FILE_FLAG_OPEN_REPARSE_POINT, FILE_ID_INFO, FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE,
            FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_TRAVERSE, FileIdInfo,
            GetFileInformationByHandleEx, SYNCHRONIZE,
        },
        System::IO::IO_STATUS_BLOCK,
    },
};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        for _ in 0..100 {
            let path = std::env::temp_dir().join(format!(
                "mado-p02b-{}-{}",
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
    fn entry(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        // All fixtures are flat. Never recursively clean a directory or follow a junction.
        let cleanup = || -> std::io::Result<()> {
            for entry in fs::read_dir(&self.0)? {
                let path = entry?.path();
                if fs::symlink_metadata(&path)?.is_dir() {
                    fs::remove_dir(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
            fs::remove_dir(&self.0)
        };
        if let Err(e) = cleanup() {
            eprintln!("Owned fixture cleanup failed at {}: {e}", self.0.display());
        }
    }
}

struct Junction(Option<PathBuf>);
impl Junction {
    fn remove(&mut self) {
        if let Some(path) = &self.0 {
            fs::remove_dir(path).expect("remove owned junction itself, not its target");
            self.0 = None;
        }
    }
}
impl Drop for Junction {
    fn drop(&mut self) {
        if let Some(path) = &self.0 {
            if let Err(e) = fs::remove_dir(path) {
                if e.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("Owned junction cleanup failed at {}: {e}", path.display());
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Identity(u64, [u8; 16]);
fn identity(file: &File) -> Identity {
    let mut info = FILE_ID_INFO::default();
    // SAFETY: live handle, correctly sized/aligned output; unsupported IDs fail the probe.
    let ok = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FileIdInfo,
            (&mut info as *mut FILE_ID_INFO).cast(),
            size_of::<FILE_ID_INFO>() as u32,
        )
    };
    assert_ne!(ok, 0, "identity: {}", std::io::Error::last_os_error());
    Identity(info.VolumeSerialNumber, info.FileId.Identifier)
}

fn open(path: &Path, access: u32, share: u32) -> File {
    OpenOptions::new()
        .access_mode(access | SYNCHRONIZE)
        .share_mode(share)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .unwrap_or_else(|e| panic!("open {}: {e}", path.display()))
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    name: OsString,
    id: Identity,
    bytes: Vec<u8>,
}
fn entries(f: &Fixture) -> Vec<Entry> {
    let mut entries: Vec<_> = fs::read_dir(&f.0)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let file = open(
                &entry.path(),
                FILE_READ_ATTRIBUTES,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            );
            Entry {
                name: entry.file_name(),
                id: identity(&file),
                bytes: fs::read(entry.path()).unwrap(),
            }
        })
        .collect();
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries
}

struct Prepared {
    // Backend capture pins/rejects ancestors, rejects reparse sources, retains original ID.
    _capture: CapturedRename,
    parent: File,
    source: File,
}
fn prepare(source: &Path) -> Prepared {
    let capture = CapturedRename::capture(source).unwrap();
    let retained = open(
        capture.source(),
        FILE_READ_ATTRIBUTES,
        FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
    );
    let captured_id = identity(&retained);
    let parent = open(
        capture.source().parent().unwrap(),
        FILE_READ_ATTRIBUTES | FILE_TRAVERSE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
    );
    // Reopen the fixed source with DELETE and no delete sharing through native commit.
    let source = open(
        capture.source(),
        FILE_READ_ATTRIBUTES | DELETE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
    );
    assert_eq!(identity(&source), captured_id, "replaced source");
    assert_eq!(
        source.metadata().unwrap().file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT,
        0,
        "reparse source"
    );
    Prepared {
        _capture: capture,
        parent,
        source,
    }
}

fn native_rename(
    prepared: &Prepared,
    name: &str,
    class: FILE_INFORMATION_CLASS,
    flags: u32,
) -> i32 {
    assert_eq!(flags & FILE_RENAME_REPLACE_IF_EXISTS, 0);
    assert!(class == FileRenameInformation || class == FileRenameInformationEx);
    let wide: Vec<u16> = OsStr::new(name).encode_wide().collect();
    let offset = offset_of!(FILE_RENAME_INFORMATION, FileName);
    // Include the documented header-plus-name size, not just the used variable tail.
    let bytes = size_of::<FILE_RENAME_INFORMATION>() + wide.len() * 2;
    let mut storage = vec![0usize; bytes.div_ceil(size_of::<usize>())];
    let info = storage.as_mut_ptr().cast::<FILE_RENAME_INFORMATION>();
    let mut io = IO_STATUS_BLOCK::default();
    // SAFETY: aligned, zero-filled buffer contains the complete relative UTF-16 name;
    // parent/source handles stay live and pinned. Legacy BOOLEAN is false, Ex flags exclude 1.
    unsafe {
        (*info).Anonymous.Flags = flags;
        (*info).RootDirectory = prepared.parent.as_raw_handle();
        (*info).FileNameLength = (wide.len() * 2) as u32;
        ptr::copy_nonoverlapping(
            wide.as_ptr(),
            storage.as_mut_ptr().cast::<u8>().add(offset).cast::<u16>(),
            wide.len(),
        );
        NtSetInformationFile(
            prepared.source.as_raw_handle(),
            &mut io,
            info.cast(),
            bytes as u32,
            class,
        )
    }
}

#[test]
#[ignore = "Disposable native comparison only, not a passing no-overwrite criterion"]
fn native_no_replace_variant_matrix() {
    let variants = [
        ("legacy", FileRenameInformation, 0),
        ("Ex-zero", FileRenameInformationEx, 0),
        (
            "Ex-POSIX-only",
            FileRenameInformationEx,
            FILE_RENAME_POSIX_SEMANTICS,
        ),
    ];
    for (label, class, flags) in variants {
        for scenario in ["ordinary", "hardlink", "case-only"] {
            let f = Fixture::new();
            let (source_name, target_name) = if scenario == "case-only" {
                ("Case.txt", "case.TXT")
            } else {
                ("source", "destination")
            };
            fs::write(f.entry(source_name), b"source bytes").unwrap();
            let prepared = prepare(&f.entry(source_name));
            match scenario {
                "ordinary" => fs::write(f.entry(target_name), b"destination bytes").unwrap(),
                "hardlink" => fs::hard_link(f.entry(source_name), f.entry(target_name)).unwrap(),
                _ => (),
            }
            let before = entries(&f);
            let status = native_rename(&prepared, target_name, class, flags);
            // SAFETY: conversion of the NT call's returned status only.
            let dos_error = if status < 0 {
                unsafe { RtlNtStatusToDosError(status) }
            } else {
                0
            };
            let after = entries(&f);
            eprintln!(
                "{label}/{scenario}: class={class}, flags={flags}, NTSTATUS={:#010x}, Win32={dos_error}\n  before={before:?}\n  after={after:?}",
                status as u32
            );
            // Diagnostic integrity check only. Source-name loss remains an unacceptable
            // collision even when all bytes/IDs survive. Do not assert contract acceptance.
            for entry in &after {
                assert!(
                    before
                        .iter()
                        .any(|old| old.id == entry.id && old.bytes == entry.bytes)
                );
            }
            assert!(!after.is_empty());
        }
    }
}

#[test]
fn junction_source_and_ancestors_reject_without_touching_owned_target() {
    let f = Fixture::new();
    let outside = Fixture::new();
    fs::write(outside.entry("sentinel"), b"untouched").unwrap();
    let before = entries(&outside);
    let junction = f.entry("junction");
    // Drop the junction before its target fixture, including assertion-failure unwinding.
    let mut junction_cleanup = Junction(Some(junction.clone()));
    let output = Command::new("cmd.exe")
        .args(["/D", "/C", "mklink", "/J"])
        .arg(&junction)
        .arg(&outside.0)
        .output()
        .expect("start Windows junction fixture facility");
    assert!(
        output.status.success(),
        "mklink /J failed: status={} stdout={} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(
        fs::symlink_metadata(&junction).unwrap().file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT,
        0
    );
    assert!(matches!(
        create_empty_file(&junction, OsStr::new("escape")),
        Err(Error::ReparsePoint(_))
    ));
    assert!(matches!(
        create_directory(&junction, OsStr::new("escape")),
        Err(Error::ReparsePoint(_))
    ));
    assert!(matches!(
        CapturedRename::capture(&junction),
        Err(Error::ReparsePoint(_))
    ));
    assert!(matches!(
        CapturedRename::capture(&junction.join("sentinel")),
        Err(Error::ReparsePoint(_))
    ));
    assert_eq!(entries(&outside), before);
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 1);
    // Remove only the junction itself before either fixture's nonrecursive cleanup.
    junction_cleanup.remove();
    assert_eq!(entries(&outside), before);
    eprintln!(
        "Static owned junction rejected for file/folder creation and source/ancestor capture; sentinel name/ID/bytes unchanged; junction removed nonrecursively"
    );
}
