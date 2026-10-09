use super::{Error, validate_units};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    mem::{offset_of, size_of},
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        io::{AsRawHandle, FromRawHandle},
    },
    path::{Path, PathBuf},
    ptr,
};
use windows_sys::{
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        Storage::FileSystem::{
            FILE_CREATE, FILE_DIRECTORY_FILE, FILE_NON_DIRECTORY_FILE, FILE_OPEN,
            FILE_OPEN_REPARSE_POINT, FILE_RENAME_INFORMATION, FILE_SYNCHRONOUS_IO_NONALERT,
            FileRenameInformation, NtCreateFile, NtSetInformationFile,
        },
    },
    Win32::{
        Foundation::{HANDLE, RtlNtStatusToDosError, UNICODE_STRING},
        Storage::FileSystem::{
            CreateFileW, DELETE, FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_REPARSE_POINT,
            FILE_ATTRIBUTE_TAG_INFO, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
            FILE_ID_INFO, FILE_NAME_INFO, FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ,
            FILE_SHARE_WRITE, FILE_TRAVERSE, FileAttributeTagInfo, FileIdInfo, FileNameInfo,
            GetFileInformationByHandleEx, OPEN_EXISTING, SYNCHRONIZE,
        },
        System::IO::IO_STATUS_BLOCK,
    },
};

pub fn validate_name(name: &OsStr) -> Result<(), Error> {
    validate_units(&name.encode_wide().collect::<Vec<_>>())
}

fn io_error(action: &'static str, path: &Path, source: std::io::Error) -> Error {
    Error::Io {
        action,
        path: path.to_owned(),
        source,
    }
}

fn nt_result(status: i32, action: &'static str, path: &Path) -> Result<(), Error> {
    if status >= 0 {
        Ok(())
    } else {
        // SAFETY: status is the value returned by an NT system call.
        let code = unsafe { RtlNtStatusToDosError(status) };
        Err(io_error(
            action,
            path,
            std::io::Error::from_raw_os_error(code as i32),
        ))
    }
}

fn raw(file: &File) -> HANDLE {
    file.as_raw_handle()
}

fn open_child(
    parent: &File,
    name: &OsStr,
    path: &Path,
    access: u32,
    share: u32,
    disposition: u32,
    options: u32,
    action: &'static str,
) -> Result<File, Error> {
    validate_name(name)?;
    let mut wide: Vec<u16> = name.encode_wide().collect();
    let mut string = UNICODE_STRING {
        Length: (wide.len() * 2) as u16,
        MaximumLength: (wide.len() * 2) as u16,
        Buffer: wide.as_mut_ptr(),
    };
    let attributes = OBJECT_ATTRIBUTES {
        Length: size_of::<OBJECT_ATTRIBUTES>() as u32,
        RootDirectory: raw(parent),
        ObjectName: &mut string,
        // No OBJ_CASE_INSENSITIVE override: respect the directory's actual case rules.
        Attributes: 0,
        ..Default::default()
    };
    let mut handle = ptr::null_mut();
    let mut status_block = IO_STATUS_BLOCK::default();
    // SAFETY: structs and UTF-16 remain live; the name is one validated relative component.
    // Synchronous I/O prevents STATUS_PENDING; no reparse-point contents are opened.
    let status = unsafe {
        NtCreateFile(
            &mut handle,
            access | SYNCHRONIZE,
            &attributes,
            &mut status_block,
            ptr::null(),
            FILE_ATTRIBUTE_NORMAL,
            share,
            disposition,
            options | FILE_OPEN_REPARSE_POINT | FILE_SYNCHRONOUS_IO_NONALERT,
            ptr::null(),
            0,
        )
    };
    nt_result(status, action, path)?;
    // SAFETY: successful NtCreateFile returned one owned handle, closed by File::drop.
    Ok(unsafe { File::from_raw_handle(handle) })
}

fn attributes(file: &File, path: &Path) -> Result<u32, Error> {
    let mut info = FILE_ATTRIBUTE_TAG_INFO::default();
    // SAFETY: correctly sized output and a live handle.
    let ok = unsafe {
        GetFileInformationByHandleEx(
            raw(file),
            FileAttributeTagInfo,
            (&mut info as *mut FILE_ATTRIBUTE_TAG_INFO).cast(),
            size_of::<FILE_ATTRIBUTE_TAG_INFO>() as u32,
        )
    };
    if ok == 0 {
        return Err(io_error(
            "Read entry attributes",
            path,
            std::io::Error::last_os_error(),
        ));
    }
    if info.FileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(Error::ReparsePoint(path.to_owned()));
    }
    Ok(info.FileAttributes)
}

#[derive(PartialEq, Eq)]
struct Identity {
    volume: u64,
    id: [u8; 16],
}

fn identity(file: &File, path: &Path) -> Result<Identity, Error> {
    let mut info = FILE_ID_INFO::default();
    // SAFETY: correctly sized output and a live handle. Unsupported IDs fail closed.
    let ok = unsafe {
        GetFileInformationByHandleEx(
            raw(file),
            FileIdInfo,
            (&mut info as *mut FILE_ID_INFO).cast(),
            size_of::<FILE_ID_INFO>() as u32,
        )
    };
    if ok == 0 {
        return Err(io_error(
            "Capture entry identity",
            path,
            std::io::Error::last_os_error(),
        ));
    }
    Ok(Identity {
        volume: info.VolumeSerialNumber,
        id: info.FileId.Identifier,
    })
}

fn current_name(file: &File, path: &Path) -> Result<OsString, Error> {
    // FILE_NAME_INFO includes at most a UNICODE_STRING-sized name plus its header.
    let mut buffer = vec![0usize; 65540usize.div_ceil(size_of::<usize>())];
    let info = buffer.as_mut_ptr().cast::<FILE_NAME_INFO>();
    // SAFETY: live handle and aligned, bounded output allocation.
    let ok = unsafe { GetFileInformationByHandleEx(raw(file), FileNameInfo, info.cast(), 65540) };
    if ok == 0 {
        return Err(io_error(
            "Read native entry name",
            path,
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: header is initialized by the successful call; validate length before reading tail.
    let bytes = unsafe { (*info).FileNameLength } as usize;
    let offset = offset_of!(FILE_NAME_INFO, FileName);
    if bytes % 2 != 0 || bytes > 65540 - offset {
        return Err(io_error(
            "Read native entry name",
            path,
            std::io::Error::other("Invalid native name length"),
        ));
    }
    // SAFETY: length lies within the aligned output allocation.
    let name = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>().add(offset).cast::<u16>(),
            bytes / 2,
        )
    };
    let base = name.rsplit(|c| *c == 92).next().unwrap();
    Ok(OsString::from_wide(base))
}

fn separator(c: u16) -> bool {
    c == 47 || c == 92
}

// Accept only absolute drive/UNC paths and their ordinary verbatim equivalents.
// Do not pass caller text through a DOS/device namespace parser or normalize dot components.
fn split_absolute(path: &Path) -> Result<(PathBuf, Vec<OsString>), Error> {
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    let invalid = || Error::UnsupportedPath(path.to_owned());
    let drive = |s: &[u16]| {
        s.len() >= 3 && matches!(s[0], 65..=90 | 97..=122) && s[1] == 58 && separator(s[2])
    };
    let verbatim = wide.starts_with(&[92, 92, 63, 92]);
    if verbatim && wide.contains(&47) {
        return Err(invalid());
    }
    let input = if verbatim { &wide[4..] } else { &wide[..] };
    let (root, tail) = if drive(input) {
        if verbatim && input[2] != 92 {
            return Err(invalid());
        }
        (vec![92, 92, 63, 92, input[0], 58, 92], &input[3..])
    } else {
        let unc = if verbatim {
            if input.len() < 4
                || !input[..3].iter().copied().eq("UNC".encode_utf16())
                || input[3] != 92
            {
                return Err(invalid());
            }
            &input[4..]
        } else if input.len() >= 2 && separator(input[0]) && separator(input[1]) {
            &input[2..]
        } else {
            return Err(invalid());
        };
        let server_end = unc.iter().position(|c| separator(*c)).ok_or_else(invalid)?;
        let share_start = server_end + 1;
        let share_end = unc[share_start..]
            .iter()
            .position(|c| separator(*c))
            .map_or(unc.len(), |i| share_start + i);
        validate_units(&unc[..server_end]).map_err(|_| invalid())?;
        validate_units(&unc[share_start..share_end]).map_err(|_| invalid())?;
        let mut root: Vec<u16> = "\\\\?\\UNC\\".encode_utf16().collect();
        root.extend_from_slice(&unc[..share_end]);
        // Both separators accepted in ordinary paths, native roots always use backslashes.
        root.iter_mut().for_each(|c| {
            if *c == 47 {
                *c = 92;
            }
        });
        root.push(92);
        (
            root,
            if share_end < unc.len() {
                &unc[share_end + 1..]
            } else {
                &[]
            },
        )
    };
    let mut components = Vec::new();
    if !tail.is_empty() {
        let tail = if separator(*tail.last().unwrap()) {
            &tail[..tail.len() - 1]
        } else {
            tail
        };
        for component in tail.split(|c| separator(*c)) {
            validate_units(component).map_err(|_| invalid())?;
            components.push(OsString::from_wide(component));
        }
    }
    Ok((PathBuf::from(OsString::from_wide(&root)), components))
}

struct Directory {
    path: PathBuf,
    // No delete sharing on any ancestor: paths cannot be retargeted by directory rename/removal.
    // ponytail: O(depth) handles live until completion/drop; no junction traversal fallback.
    chain: Vec<File>,
}

impl Directory {
    fn open(path: &Path) -> Result<Self, Error> {
        let (root, components) = split_absolute(path)?;
        let mut wide: Vec<u16> = root.as_os_str().encode_wide().collect();
        wide.push(0);
        // SAFETY: valid null-terminated filesystem root; no caller device namespace is accepted.
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                FILE_READ_ATTRIBUTES | FILE_TRAVERSE | SYNCHRONIZE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
                ptr::null_mut(),
            )
        };
        if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            return Err(io_error(
                "Open filesystem root",
                &root,
                std::io::Error::last_os_error(),
            ));
        }
        // SAFETY: successful CreateFileW transferred an owned handle.
        let file = unsafe { File::from_raw_handle(handle) };
        attributes(&file, &root)?;
        let mut result = Self {
            path: root,
            chain: vec![file],
        };
        for name in components {
            let path = result.path.join(&name);
            let file = open_child(
                result.handle(),
                &name,
                &path,
                FILE_READ_ATTRIBUTES | FILE_TRAVERSE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                FILE_OPEN,
                FILE_DIRECTORY_FILE,
                "Open parent directory",
            )?;
            attributes(&file, &path)?;
            result.chain.push(file);
            result.path = path;
        }
        Ok(result)
    }

    fn handle(&self) -> &File {
        self.chain.last().unwrap()
    }
}

/// Create a new empty regular file, atomically refusing every existing destination entry.
/// Parent must be an absolute drive/UNC path without dot components or reparse ancestors.
pub fn create_empty_file(parent: &Path, name: &OsStr) -> Result<PathBuf, Error> {
    create(parent, name, FILE_NON_DIRECTORY_FILE)
}

/// Create one directory, not its ancestors. Existing entries are never reused or replaced.
pub fn create_directory(parent: &Path, name: &OsStr) -> Result<PathBuf, Error> {
    create(parent, name, FILE_DIRECTORY_FILE)
}

fn create(parent: &Path, name: &OsStr, options: u32) -> Result<PathBuf, Error> {
    validate_name(name)?;
    let parent = Directory::open(parent)?;
    let path = parent.path.join(name);
    let file = open_child(
        parent.handle(),
        name,
        &path,
        FILE_READ_ATTRIBUTES,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        FILE_CREATE,
        options,
        "Create entry",
    )?;
    // No data write, truncation, recursive creation, rollback, or temporary entry.
    drop(file);
    Ok(path)
}

/// An immutable source path plus its retained native identity. No disk change on capture/drop.
/// Ancestor directories remain protected against rename/removal until commit/drop. The source
/// can disappear or be replaced externally; commit reopens its fixed name and checks identity.
pub struct CapturedRename {
    parent: Directory,
    source: PathBuf,
    name: OsString,
    id: Identity,
    // Retention prevents file-ID reuse while waiting to commit. It does not deny source deletion.
    _captured: File,
}

impl CapturedRename {
    pub fn capture(source: &Path) -> Result<Self, Error> {
        let (root, mut names) = split_absolute(source)?;
        let name = names
            .pop()
            .ok_or_else(|| Error::UnsupportedPath(source.to_owned()))?;
        let parent_path = names.iter().fold(root, |path, name| path.join(name));
        let parent = Directory::open(&parent_path)?;
        let source = parent.path.join(&name);
        let file = open_child(
            parent.handle(),
            &name,
            &source,
            FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            FILE_OPEN,
            0,
            "Capture rename source",
        )?;
        attributes(&file, &source)?;
        let id = identity(&file, &source)?;
        Ok(Self {
            parent,
            source,
            name,
            id,
            _captured: file,
        })
    }

    pub fn source(&self) -> &Path {
        &self.source
    }

    /// Disabled by default pending resolution of a same-file hard-link collision blocker.
    /// The unreviewed diagnostic feature attempts a same-parent native rename with no
    /// overwrite flag or temporary-name workaround. Never enable it on personal data.
    pub fn rename(self, name: &OsStr) -> Result<PathBuf, Error> {
        validate_name(name)?;
        if !cfg!(feature = "unreviewed-native-rename") {
            return Err(Error::RenameSafetyBlocked);
        }
        let destination = self.parent.path.join(name);
        // Deny delete sharing from revalidation through commit so the reopened source cannot
        // be swapped/moved in the check-to-commit interval. The actual mutation uses this handle.
        let source = open_child(
            self.parent.handle(),
            &self.name,
            &self.source,
            FILE_READ_ATTRIBUTES | DELETE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            FILE_OPEN,
            0,
            "Reopen rename source",
        )?;
        if identity(&source, &self.source)? != self.id {
            return Err(Error::SourceReplaced(self.source.clone()));
        }
        attributes(&source, &self.source)?;
        if name == self.name {
            return Ok(destination);
        }
        let before = current_name(&source, &self.source)?;
        let wide: Vec<u16> = name.encode_wide().collect();
        let offset = offset_of!(FILE_RENAME_INFORMATION, FileName);
        let bytes = offset + wide.len() * 2;
        // Aligned, zero-filled variable-size native buffer; replace flag remains false.
        let mut storage = vec![0usize; bytes.div_ceil(size_of::<usize>())];
        let info = storage.as_mut_ptr().cast::<FILE_RENAME_INFORMATION>();
        let mut status_block = IO_STATUS_BLOCK::default();
        // SAFETY: allocation is aligned and sized for the header and full UTF-16 filename.
        // Kernel reads exactly `bytes`; both live source and parent handles are pinned above.
        let status = unsafe {
            (*info).RootDirectory = raw(self.parent.handle());
            (*info).FileNameLength = (wide.len() * 2) as u32;
            ptr::copy_nonoverlapping(
                wide.as_ptr(),
                storage.as_mut_ptr().cast::<u8>().add(offset).cast::<u16>(),
                wide.len(),
            );
            NtSetInformationFile(
                raw(&source),
                &mut status_block,
                info.cast(),
                bytes as u32,
                FileRenameInformation,
            )
        };
        nt_result(status, "Rename entry without replacement", &destination)?;
        // Known blocker: NTFS can coalesce two hard links even with replacement disabled.
        // This postcheck DOES NOT reject that case: source name vanishes and the handle
        // reflects the existing destination. Keep the default safety gate until resolved.
        // Hard links can be added even with delete/write sharing denied.
        match current_name(&source, &destination) {
            Ok(actual) if actual == name => Ok(destination),
            Ok(actual) if actual == before => Err(io_error(
                "Rename destination already refers to the source",
                &destination,
                std::io::Error::from(std::io::ErrorKind::AlreadyExists),
            )),
            observed => Err(Error::CommitUnverified {
                source: self.source.clone(),
                destination: destination.clone(),
                detail: Box::new(observed.err().unwrap_or_else(|| {
                    io_error(
                        "Confirm renamed entry",
                        &destination,
                        std::io::Error::other("Filesystem returned an unexpected entry spelling"),
                    )
                })),
            }),
        }
    }
}
