//! Windows named-source observations, not destructive-operation authorization.
//!
//! Capture preserves the exact native path and keeps the observed object open. Revalidation
//! opens that original name again and compares volume serial + 128-bit file ID. Only the
//! final entry is opened without following reparse points. Ancestors may resolve through
//! reparse points, and another process can change names or contents immediately afterward.
//! No content, timestamps, ancestor identities, atomic mutation or concurrent-writer snapshot
//! is captured. A future executor must bind/revalidate its handles through actual commit.
#![cfg(windows)]

use std::{
    fmt, io,
    mem::{MaybeUninit, size_of},
    os::windows::{
        ffi::OsStrExt,
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
    },
    path::{Path, PathBuf},
    ptr,
};
use windows_sys::Win32::{
    Foundation::{
        ERROR_FILE_NOT_FOUND, ERROR_INVALID_FUNCTION, ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED,
        ERROR_PATH_NOT_FOUND, GetLastError, INVALID_HANDLE_VALUE, SetLastError,
    },
    Storage::FileSystem::*,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Identity {
    volume_serial: u64,
    file_id: [u8; 16],
}
impl Identity {
    pub fn volume_serial(&self) -> u64 {
        self.volume_serial
    }
    pub fn file_id(&self) -> &[u8; 16] {
        &self.file_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryKind {
    File,
    Directory,
    ReparsePoint { tag: u32 },
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathRejection {
    Nul,
    Length,
    NotAbsoluteFilesystem,
    UnsupportedSpelling,
    DeviceAlias,
    PipeShare,
}

#[derive(Debug)]
pub enum SnapshotError {
    InvalidPath(PathRejection),
    Missing,
    Replaced {
        observed_identity: Identity,
        observed_kind: EntryKind,
    },
    Unsupported(&'static str),
    Io(io::Error),
}
impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath(reason) => write!(f, "Source path rejected before I/O: {reason:?}"),
            Self::Missing => f.write_str("Original source name is missing"),
            Self::Replaced { .. } => {
                f.write_str("Original source name refers to a different object or kind")
            }
            Self::Unsupported(reason) => write!(f, "Source observation unsupported: {reason}"),
            Self::Io(error) => write!(f, "Source metadata I/O failed: {error}"),
        }
    }
}
impl std::error::Error for SnapshotError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

/// An immutable observation. The private owned handle keeps the captured object alive
/// across rename/unlink/replacement. Sharing grants no lock and permits concurrent writers.
/// Dropping this value closes its handle; no raw handle or mutation API is exposed.
pub struct Snapshot {
    original_path: PathBuf,
    identity: Identity,
    kind: EntryKind,
    _handle: OwnedHandle,
}
impl fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Snapshot")
            .field("original_path", &self.original_path)
            .field("identity", &self.identity)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl Snapshot {
    pub fn capture(original_path: PathBuf) -> Result<Self, SnapshotError> {
        let name = admitted_name(&original_path)?;
        let handle = open_admitted(&name)?;
        let (identity, kind) = observe(&handle)?;
        Ok(Self {
            original_path,
            identity,
            kind,
            _handle: handle,
        })
    }
    pub fn original_path(&self) -> &Path {
        &self.original_path
    }
    pub fn identity(&self) -> Identity {
        self.identity
    }
    pub fn kind(&self) -> EntryKind {
        self.kind
    }

    /// Ok means the original name matched at this observation only. It is not permission
    /// to mutate that name later. Kind changes on the same object also refuse revalidation.
    pub fn revalidate_original_name(&self) -> Result<(), SnapshotError> {
        let name = admitted_name(&self.original_path)?;
        let handle = open_admitted(&name)?;
        let (identity, kind) = observe(&handle)?;
        if identity == self.identity && kind == self.kind {
            Ok(())
        } else {
            Err(SnapshotError::Replaced {
                observed_identity: identity,
                observed_kind: kind,
            })
        }
    }
}

fn io_error(error: io::Error) -> SnapshotError {
    match error.raw_os_error().map(|code| code as u32) {
        Some(ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND) => SnapshotError::Missing,
        _ => SnapshotError::Io(error),
    }
}

fn open_admitted(name: &[u16]) -> Result<OwnedHandle, SnapshotError> {
    #[cfg(test)]
    OPEN_COUNT.with(|count| count.set(count.get() + 1));
    // SAFETY: admission supplied a terminated, interior-NUL-free UTF-16 buffer.
    // OPEN_EXISTING + READ_ATTRIBUTES never requests content reads, DELETE, writes or creation.
    let raw = unsafe {
        CreateFileW(
            name.as_ptr(),
            FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS,
            ptr::null_mut(),
        )
    };
    if raw == INVALID_HANDLE_VALUE {
        return Err(io_error(io::Error::last_os_error()));
    }
    // SAFETY: a successful CreateFileW returned a fresh handle exclusively owned here.
    Ok(unsafe { OwnedHandle::from_raw_handle(raw) })
}

fn observe(handle: &OwnedHandle) -> Result<(Identity, EntryKind), SnapshotError> {
    // SAFETY: the handle is live; clearing last error distinguishes UNKNOWN from failure.
    let (file_type, error) = unsafe {
        SetLastError(0);
        let file_type = GetFileType(handle.as_raw_handle());
        (file_type, GetLastError())
    };
    if file_type != FILE_TYPE_DISK {
        return Err(if file_type == FILE_TYPE_UNKNOWN && error != 0 {
            SnapshotError::Io(io::Error::from_raw_os_error(error as i32))
        } else {
            SnapshotError::Unsupported("handle is not FILE_TYPE_DISK")
        });
    }
    let mut id = MaybeUninit::<FILE_ID_INFO>::uninit();
    let mut attributes = MaybeUninit::<FILE_ATTRIBUTE_TAG_INFO>::uninit();
    // SAFETY: each typed buffer has its native alignment and exact writable size. It is
    // initialized only on successful GetFileInformationByHandleEx, while handle stays live.
    unsafe {
        if GetFileInformationByHandleEx(
            handle.as_raw_handle(),
            FileIdInfo,
            id.as_mut_ptr().cast(),
            size_of::<FILE_ID_INFO>() as u32,
        ) == 0
        {
            return Err(metadata_error(io::Error::last_os_error()));
        }
        if GetFileInformationByHandleEx(
            handle.as_raw_handle(),
            FileAttributeTagInfo,
            attributes.as_mut_ptr().cast(),
            size_of::<FILE_ATTRIBUTE_TAG_INFO>() as u32,
        ) == 0
        {
            return Err(metadata_error(io::Error::last_os_error()));
        }
        let id = id.assume_init();
        let attributes = attributes.assume_init();
        if id.FileId.Identifier == [0; 16] {
            return Err(SnapshotError::Unsupported(
                "provider returned an empty file ID",
            ));
        }
        let kind = if attributes.FileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            EntryKind::ReparsePoint {
                tag: attributes.ReparseTag,
            }
        } else if attributes.FileAttributes & FILE_ATTRIBUTE_DEVICE != 0 {
            EntryKind::Other
        } else if attributes.FileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0 {
            EntryKind::Directory
        } else {
            EntryKind::File
        };
        Ok((
            Identity {
                volume_serial: id.VolumeSerialNumber,
                file_id: id.FileId.Identifier,
            },
            kind,
        ))
    }
}

fn metadata_error(error: io::Error) -> SnapshotError {
    match error.raw_os_error().map(|code| code as u32) {
        Some(ERROR_INVALID_FUNCTION | ERROR_NOT_SUPPORTED | ERROR_INVALID_PARAMETER) => {
            SnapshotError::Unsupported("provider does not support native identity/kind metadata")
        }
        _ => io_error(error),
    }
}

// No Path::components here: it normalizes some spellings. Admission inspects exact native
// units before any open and does not rewrite them. Conservative spelling limits are explicit.
fn admitted_name(path: &Path) -> Result<Vec<u16>, SnapshotError> {
    let mut units: Vec<u16> = path.as_os_str().encode_wide().collect();
    admit(&units).map_err(SnapshotError::InvalidPath)?;
    units.push(0);
    Ok(units)
}

fn ascii_eq(units: &[u16], ascii: &str) -> bool {
    units.len() == ascii.len()
        && units
            .iter()
            .zip(ascii.bytes())
            .all(|(&a, b)| a <= 127 && (a as u8).eq_ignore_ascii_case(&b))
}
fn drive(units: &[u16]) -> bool {
    units.len() >= 3
        && matches!(units[0], 65..=90 | 97..=122)
        && units[1] == b':' as u16
        && units[2] == b'\\' as u16
}
fn volume(units: &[u16]) -> bool {
    units.len() >= 45
        && ascii_eq(&units[..7], "Volume{")
        && units[43] == b'}' as u16
        && units[44] == b'\\' as u16
        && units[7..43].iter().enumerate().all(|(i, &u)| {
            if matches!(i, 8 | 13 | 18 | 23) {
                u == b'-' as u16
            } else {
                matches!(u, 48..=57 | 65..=70 | 97..=102)
            }
        })
}
fn admit(units: &[u16]) -> Result<(), PathRejection> {
    use PathRejection::*;
    if units.contains(&0) {
        return Err(Nul);
    }
    if units.is_empty() || units.len() > 32766 {
        return Err(Length);
    }
    if units.contains(&(b'/' as u16)) {
        return Err(UnsupportedSpelling);
    }
    let verbatim = units.starts_with(&[92, 92, 63, 92]);
    if !verbatim && units.len() > 259 {
        return Err(Length);
    }
    let tail = if verbatim {
        let rest = &units[4..];
        if drive(rest) {
            &rest[3..]
        } else if rest.len() >= 4 && ascii_eq(&rest[..4], "UNC\\") {
            unc_tail(&rest[4..])?
        } else if volume(rest) {
            &rest[45..]
        } else {
            return Err(NotAbsoluteFilesystem);
        }
    } else if drive(units) {
        &units[3..]
    } else if units.starts_with(&[92, 92]) {
        unc_tail(&units[2..])?
    } else {
        return Err(NotAbsoluteFilesystem);
    };
    if !tail.is_empty() {
        for part in tail.split(|&u| u == 92) {
            admit_component(part)?;
        }
    }
    Ok(())
}
fn unc_tail(units: &[u16]) -> Result<&[u16], PathRejection> {
    let server_end = units
        .iter()
        .position(|&u| u == 92)
        .ok_or(PathRejection::NotAbsoluteFilesystem)?;
    admit_component(&units[..server_end])?;
    let rest = &units[server_end + 1..];
    let share_end = rest.iter().position(|&u| u == 92).unwrap_or(rest.len());
    let share = &rest[..share_end];
    admit_component(share)?;
    if ascii_eq(share, "pipe") {
        return Err(PathRejection::PipeShare);
    }
    Ok(if share_end < rest.len() {
        &rest[share_end + 1..]
    } else {
        &[]
    })
}
fn admit_component(part: &[u16]) -> Result<(), PathRejection> {
    use PathRejection::*;
    if part.is_empty()
        || part.len() > 255
        || matches!(part.last(), Some(32 | 46))
        || part
            .iter()
            .any(|&u| u < 32 || matches!(u, 34 | 42 | 58 | 60 | 62 | 63 | 124))
    {
        return Err(UnsupportedSpelling);
    }
    let stem_end = part.iter().position(|&u| u == 46).unwrap_or(part.len());
    let mut stem = &part[..stem_end];
    while stem.last() == Some(&32) {
        stem = &stem[..stem.len() - 1];
    }
    if ["CON", "PRN", "AUX", "NUL", "CLOCK$", "CONIN$", "CONOUT$"]
        .iter()
        .any(|name| ascii_eq(stem, name))
        || (stem.len() == 4
            && (ascii_eq(&stem[..3], "COM") || ascii_eq(&stem[..3], "LPT"))
            && matches!(stem[3], 48..=57 | 0x00b9 | 0x00b2 | 0x00b3))
    {
        return Err(DeviceAlias);
    }
    Ok(())
}

#[cfg(test)]
thread_local! { static OPEN_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[cfg(test)]
mod tests;
