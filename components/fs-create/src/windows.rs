use super::{Error, ascii_eq, validate_name, validate_spelling, validate_units};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io,
    mem::size_of,
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
            FILE_OPEN_REPARSE_POINT, FILE_SYNCHRONOUS_IO_NONALERT, FileFsDeviceInformation,
            NtCreateFile, NtQueryVolumeInformationFile,
        },
        System::SystemServices::{
            FILE_FS_DEVICE_INFORMATION, FILE_REMOTE_DEVICE, FILE_REMOTE_DEVICE_VSMB,
        },
    },
    Win32::{
        Foundation::{
            HANDLE, INVALID_HANDLE_VALUE, OBJ_DONT_REPARSE, RtlNtStatusToDosError, UNICODE_STRING,
        },
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_DEVICE, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL,
            FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_TAG_INFO, FILE_DEVICE_DISK,
            FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_ID_INFO,
            FILE_READ_ATTRIBUTES, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_TRAVERSE, FILE_TYPE_DISK,
            FileAttributeTagInfo, FileIdInfo, GetFileInformationByHandleEx, GetFileType,
            GetVolumeInformationByHandleW, OPEN_EXISTING, SYNCHRONIZE,
        },
        System::IO::IO_STATUS_BLOCK,
    },
};

fn io_error(action: &'static str, path: &Path, source: io::Error) -> Error {
    Error::Io {
        action,
        path: path.to_owned(),
        source,
    }
}
fn raw(file: &File) -> HANDLE {
    file.as_raw_handle()
}

// One validated basename, never a caller-supplied relative path.
fn child(parent: &File, name: &OsStr, path: &Path, create: Option<bool>) -> Result<File, Error> {
    validate_name(name)?;
    let mut units: Vec<u16> = name.encode_wide().collect();
    let mut string = UNICODE_STRING {
        Length: (units.len() * 2) as u16,
        MaximumLength: (units.len() * 2) as u16,
        Buffer: units.as_mut_ptr(),
    };
    let attributes = OBJECT_ATTRIBUTES {
        Length: size_of::<OBJECT_ATTRIBUTES>() as u32,
        RootDirectory: raw(parent),
        ObjectName: &mut string,
        // Abort native parsing if any reparse is encountered. Do not override the
        // filesystem directory's case-sensitivity policy.
        Attributes: OBJ_DONT_REPARSE,
        ..Default::default()
    };
    let mut handle = ptr::null_mut();
    let mut status_block = IO_STATUS_BLOCK::default();
    let is_directory = create != Some(false);
    let action = if create.is_some() {
        "Create entry"
    } else {
        "Pin directory"
    };
    #[cfg(test)]
    note_io();
    // SAFETY: live pinned parent, validated bounded UTF-16 basename and aligned native
    // structs remain live. Synchronous options exclude pending completion. FILE_CREATE
    // atomically refuses every existing name; FILE_OPEN is used only for capture.
    let status = unsafe {
        NtCreateFile(
            &mut handle,
            FILE_READ_ATTRIBUTES | SYNCHRONIZE | if create.is_none() { FILE_TRAVERSE } else { 0 },
            &attributes,
            &mut status_block,
            ptr::null(),
            FILE_ATTRIBUTE_NORMAL,
            if create.is_some() {
                FILE_SHARE_READ | FILE_SHARE_WRITE
            } else {
                FILE_SHARE_READ
            },
            if create.is_some() {
                FILE_CREATE
            } else {
                FILE_OPEN
            },
            FILE_OPEN_REPARSE_POINT
                | FILE_SYNCHRONOUS_IO_NONALERT
                | if is_directory {
                    FILE_DIRECTORY_FILE
                } else {
                    FILE_NON_DIRECTORY_FILE
                },
            ptr::null(),
            0,
        )
    };
    if status < 0 {
        // SAFETY: only maps the status returned by the native call.
        let code = unsafe { RtlNtStatusToDosError(status) };
        return Err(io_error(
            action,
            path,
            io::Error::from_raw_os_error(code as i32),
        ));
    }
    // SAFETY: successful synchronous call returned a fresh owned handle.
    Ok(unsafe { File::from_raw_handle(handle) })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    pub volume_serial: u64,
    pub file_id: [u8; 16],
}
fn observe_directory(file: &File, path: &Path) -> Result<Identity, Error> {
    #[cfg(test)]
    note_io();
    // SAFETY: live handle; no content reads or mutations.
    if unsafe { GetFileType(raw(file)) } != FILE_TYPE_DISK {
        return Err(Error::UnsupportedProvider(path.to_owned()));
    }
    let mut attrs = FILE_ATTRIBUTE_TAG_INFO::default();
    let mut id = FILE_ID_INFO::default();
    // SAFETY: both buffers have their native alignment/exact sizes; handle remains live.
    unsafe {
        if GetFileInformationByHandleEx(
            raw(file),
            FileAttributeTagInfo,
            (&mut attrs as *mut FILE_ATTRIBUTE_TAG_INFO).cast(),
            size_of::<FILE_ATTRIBUTE_TAG_INFO>() as u32,
        ) == 0
        {
            return Err(io_error(
                "Read directory attributes",
                path,
                io::Error::last_os_error(),
            ));
        }
        if attrs.FileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(Error::ReparsePoint(path.to_owned()));
        }
        if attrs.FileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0
            || attrs.FileAttributes & FILE_ATTRIBUTE_DEVICE != 0
        {
            return Err(Error::NotDirectory(path.to_owned()));
        }
        if GetFileInformationByHandleEx(
            raw(file),
            FileIdInfo,
            (&mut id as *mut FILE_ID_INFO).cast(),
            size_of::<FILE_ID_INFO>() as u32,
        ) == 0
        {
            return Err(io_error(
                "Read directory identity",
                path,
                io::Error::last_os_error(),
            ));
        }
    }
    if id.FileId.Identifier == [0; 16] {
        return Err(Error::UnsupportedProvider(path.to_owned()));
    }
    Ok(Identity {
        volume_serial: id.VolumeSerialNumber,
        file_id: id.FileId.Identifier,
    })
}
// Query the retained handle, never a drive-letter spelling that could map elsewhere.
fn require_local_ntfs(file: &File, path: &Path) -> Result<(), Error> {
    #[cfg(test)]
    note_io();
    let mut device = FILE_FS_DEVICE_INFORMATION::default();
    let mut status_block = IO_STATUS_BLOCK::default();
    // SAFETY: live synchronous handle, native typed/aligned output of its exact size.
    let status = unsafe {
        NtQueryVolumeInformationFile(
            raw(file),
            &mut status_block,
            (&mut device as *mut FILE_FS_DEVICE_INFORMATION).cast(),
            size_of::<FILE_FS_DEVICE_INFORMATION>() as u32,
            FileFsDeviceInformation,
        )
    };
    if status < 0 {
        // SAFETY: maps the status returned by the native query.
        let code = unsafe { RtlNtStatusToDosError(status) };
        return Err(io_error(
            "Query local filesystem device",
            path,
            io::Error::from_raw_os_error(code as i32),
        ));
    }
    if device.DeviceType != FILE_DEVICE_DISK
        || device.Characteristics & (FILE_REMOTE_DEVICE | FILE_REMOTE_DEVICE_VSMB) != 0
    {
        return Err(Error::UnsupportedProvider(path.to_owned()));
    }
    let mut filesystem = [0u16; 32];
    // SAFETY: filesystem name has a bounded writable output; omitted outputs are optional.
    let ok = unsafe {
        GetVolumeInformationByHandleW(
            raw(file),
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            filesystem.as_mut_ptr(),
            filesystem.len() as u32,
        )
    };
    if ok == 0 {
        return Err(io_error(
            "Query filesystem type",
            path,
            io::Error::last_os_error(),
        ));
    }
    let end = filesystem
        .iter()
        .position(|&unit| unit == 0)
        .ok_or_else(|| Error::UnsupportedProvider(path.to_owned()))?;
    if !local_ntfs(&device, &filesystem[..end]) {
        return Err(Error::UnsupportedProvider(path.to_owned()));
    }
    Ok(())
}
pub(super) fn local_ntfs(device: &FILE_FS_DEVICE_INFORMATION, filesystem: &[u16]) -> bool {
    device.DeviceType == FILE_DEVICE_DISK
        && device.Characteristics & (FILE_REMOTE_DEVICE | FILE_REMOTE_DEVICE_VSMB) == 0
        && ascii_eq(filesystem, "NTFS")
}
fn separator(c: u16) -> bool {
    matches!(c, 47 | 92)
}

// Validate exact caller units before opening anything. Path::components would hide dots.
// Only drive and UNC filesystem roots are admitted; no volume/device/native aliases.
fn split_absolute(path: &Path) -> Result<(PathBuf, Vec<OsString>), Error> {
    let units: Vec<u16> = path.as_os_str().encode_wide().take(32761).collect();
    let invalid = || Error::UnsupportedPath(path.to_owned());
    if units.is_empty() || units.contains(&0) || units.len() > 32760 {
        return Err(invalid());
    }
    let verbatim = units.starts_with(&[92, 92, 63, 92]);
    if verbatim && units.contains(&47) {
        return Err(invalid());
    }
    let input = if verbatim { &units[4..] } else { &units[..] };
    let drive = input.len() >= 3
        && matches!(input[0], 65..=90 | 97..=122)
        && input[1] == 58
        && separator(input[2]);
    let (root, tail) = if drive {
        (vec![92, 92, 63, 92, input[0], 58, 92], &input[3..])
    } else {
        let unc = if verbatim {
            if input.len() < 4 || !ascii_eq(&input[..4], "UNC\\") {
                return Err(invalid());
            }
            &input[4..]
        } else if input.len() >= 2 && separator(input[0]) && separator(input[1]) {
            &input[2..]
        } else {
            return Err(invalid());
        };
        let server_end = unc.iter().position(|&c| separator(c)).ok_or_else(invalid)?;
        let start = server_end + 1;
        let end = unc[start..]
            .iter()
            .position(|&c| separator(c))
            .map_or(unc.len(), |i| start + i);
        // UNC host/share names are not local DOS devices. Still reject pipe shares,
        // illegal characters, dots and trailing-space/dot aliases before I/O.
        validate_spelling(&unc[..server_end]).map_err(|_| invalid())?;
        validate_spelling(&unc[start..end]).map_err(|_| invalid())?;
        if ascii_eq(&unc[start..end], "pipe") {
            return Err(invalid());
        }
        let mut root: Vec<u16> = "\\\\?\\UNC\\".encode_utf16().collect();
        root.extend(unc[..end].iter().map(|&c| if c == 47 { 92 } else { c }));
        root.push(92);
        (
            root,
            if end < unc.len() {
                &unc[end + 1..]
            } else {
                &[]
            },
        )
    };
    let mut names = Vec::new();
    if !tail.is_empty() {
        let tail = if separator(*tail.last().unwrap()) {
            &tail[..tail.len() - 1]
        } else {
            tail
        };
        // A single final separator is supported, doubled/empty components are not.
        if tail.is_empty() {
            return Err(invalid());
        }
        for name in tail.split(|&c| separator(c)) {
            validate_units(name).map_err(|_| invalid())?;
            names.push(OsString::from_wide(name));
        }
    }
    Ok((PathBuf::from(OsString::from_wide(&root)), names))
}

/// Retains the exact original native path and a pinned target, independent of navigation.
/// No raw handle escapes. Drop releases pins without filesystem mutation.
/// ponytail: O(depth) handles stay live; conflicting data-write/delete handles and reparse
/// ancestors are refused. Supporting those paths needs a separately reviewed binding policy.
pub struct CapturedDirectory {
    original_path: PathBuf,
    identity: Identity,
    chain: Vec<File>,
}
impl std::fmt::Debug for CapturedDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CapturedDirectory")
            .field("original_path", &self.original_path)
            .field("identity", &self.identity)
            .finish_non_exhaustive()
    }
}
impl CapturedDirectory {
    pub fn capture(original_path: PathBuf) -> Result<Self, Error> {
        let (root, names) = split_absolute(&original_path)?;
        let mut units: Vec<u16> = root.as_os_str().encode_wide().collect();
        // Valid UNC spelling is not permission to contact a remote provider in this
        // initial local-only backend. Invalid endpoints were already refused lexically.
        if units.len() >= 8 && ascii_eq(&units[4..8], "UNC\\") {
            return Err(Error::UnsupportedProvider(original_path));
        }
        units.push(0);
        #[cfg(test)]
        note_io();
        // SAFETY: admission supplied a terminated filesystem root, not an endpoint.
        // No data-write/delete sharing pins the root name through subsequent open/create.
        // Attribute-only metadata conversion remains possible; native reparse guards apply.
        let handle = unsafe {
            CreateFileW(
                units.as_ptr(),
                FILE_READ_ATTRIBUTES | FILE_TRAVERSE | SYNCHRONIZE,
                FILE_SHARE_READ,
                ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
                ptr::null_mut(),
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(io_error(
                "Pin filesystem root",
                &original_path,
                io::Error::last_os_error(),
            ));
        }
        // SAFETY: successful CreateFileW returned a fresh owned handle.
        let file = unsafe { File::from_raw_handle(handle) };
        require_local_ntfs(&file, &original_path)?;
        let mut identity = observe_directory(&file, &original_path)?;
        let mut chain = vec![file];
        for name in names {
            let file = child(chain.last().unwrap(), &name, &original_path, None)?;
            identity = observe_directory(&file, &original_path)?;
            chain.push(file);
        }
        Ok(Self {
            original_path,
            identity,
            chain,
        })
    }
    pub fn original_path(&self) -> &Path {
        &self.original_path
    }
    pub fn identity(&self) -> Identity {
        self.identity
    }
    pub fn create_empty_file(&self, name: &OsStr) -> Result<PathBuf, Error> {
        self.create(name, false)
    }
    pub fn create_directory(&self, name: &OsStr) -> Result<PathBuf, Error> {
        self.create(name, true)
    }
    fn create(&self, name: &OsStr, directory: bool) -> Result<PathBuf, Error> {
        validate_name(name)?;
        let path = self.original_path.join(name);
        require_local_ntfs(self.chain.last().unwrap(), &self.original_path)?;
        // Attribute-only handles can convert pinned ancestors into reparse points.
        // Refuse observed conversions; this is not the atomic binding guarantee.
        // The retained target handle + OBJ_DONT_REPARSE enforce that at native creation,
        // including a conversion between these observations and FILE_CREATE.
        for file in &self.chain {
            observe_directory(file, &self.original_path)?;
        }
        #[cfg(test)]
        BEFORE_CREATE.with(|hook| {
            if let Some(hook) = hook.borrow_mut().take() {
                hook();
            }
        });
        // All pins stay alive through the only mutation. No re-open of original path,
        // existence test, overwrite, traversal, temporary entry or rollback is attempted.
        let file = child(self.chain.last().unwrap(), name, &path, Some(directory))?;
        drop(file);
        Ok(path)
    }
}
#[cfg(test)]
thread_local! {
    pub(super) static IO_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    pub(super) static BEFORE_CREATE: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(None) };
}
#[cfg(test)]
fn note_io() {
    IO_COUNT.with(|n| n.set(n.get() + 1));
}

#[cfg(test)]
pub(super) fn lexical_path(path: &Path) -> Result<(), Error> {
    split_absolute(path).map(|_| ())
}
