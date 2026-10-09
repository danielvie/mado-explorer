//! Bounded Windows file-clipboard packets. No filesystem access or background work.
//! Native transport is explicit: only `read_clipboard` and `write_clipboard` access
//! global clipboard contents. Call them from a caller-owned background service.
//! Paths are lexical snapshots, not verified filesystem identities or operations.

#![cfg(windows)]

mod native;
pub use native::{
    PublicationState, TransportError, TransportFailure, read_clipboard, write_clipboard,
};

use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use windows_sys::Win32::Globalization::{CP_ACP, MB_ERR_INVALID_CHARS, MultiByteToWideChar};

/// Windows native path identity, including unpaired UTF-16 surrogates.
pub type NativePathBuf = PathBuf;

pub const MAX_PAYLOAD_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PATHS: usize = 4096;
/// Excludes the terminating NUL.
pub const MAX_PATH_UNITS: usize = 32767;
pub const MAX_EFFECT_BYTES: usize = 64;
const HEADER_BYTES: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    Copy,
    Move,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntentWarning {
    MissingPreferredEffect,
    /// Link, mixed Copy/Move, zero and unknown bits all use Copy, never guessed Move.
    AmbiguousPreferredEffect(u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodecError {
    PayloadTooLarge,
    TooManyPaths,
    PathTooLong,
    EmptyList,
    HeaderTooShort,
    InvalidOffset,
    InvalidBoolean,
    OddUtf16,
    MissingTerminator,
    TrailingData,
    InvalidNamespace,
    InvalidPath,
    InvalidEffect,
    ArithmeticOverflow,
    AllocationFailed,
    AnsiConversion(u32),
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file-clipboard packet: {self:?}")
    }
}
impl std::error::Error for CodecError {}

/// Owned immutable selection snapshot. Does not capture file IDs or a destination.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilePacket {
    paths: Vec<NativePathBuf>,
    intent: Intent,
    warning: Option<IntentWarning>,
}

impl FilePacket {
    /// Copies the caller's selection; later selection changes cannot retarget it.
    pub fn snapshot(paths: &[NativePathBuf], intent: Intent) -> Result<Self, CodecError> {
        check_count(paths.len())?;
        let mut total = HEADER_BYTES + 2;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(paths.len())
            .map_err(|_| CodecError::AllocationFailed)?;
        for path in paths {
            let units = native_units(path)?;
            validate_path(&units)?;
            total = add_path_size(total, units.len())?;
            owned.push(path.clone());
        }
        Ok(Self {
            paths: owned,
            intent,
            warning: None,
        })
    }

    pub fn paths(&self) -> &[NativePathBuf] {
        &self.paths
    }
    pub fn intent(&self) -> Intent {
        self.intent
    }
    pub fn warning(&self) -> Option<IntentWarning> {
        self.warning
    }

    /// Generates Unicode DROPFILES with no lossy conversion or path normalization.
    pub fn encode_hdrop(&self) -> Result<Vec<u8>, CodecError> {
        let mut total = HEADER_BYTES + 2;
        for path in &self.paths {
            total = add_path_size(total, path.as_os_str().encode_wide().count())?;
        }
        let mut bytes = zeroed_bytes(total)?;
        bytes[..4].copy_from_slice(&(HEADER_BYTES as u32).to_le_bytes());
        bytes[16..20].copy_from_slice(&1u32.to_le_bytes());
        let mut offset = HEADER_BYTES;
        for path in &self.paths {
            for unit in path.as_os_str().encode_wide() {
                bytes[offset..offset + 2].copy_from_slice(&unit.to_le_bytes());
                offset += 2;
            }
            offset += 2;
        }
        Ok(bytes)
    }

    pub fn encode_preferred_effect(&self) -> [u8; 4] {
        match self.intent {
            Intent::Copy => 1u32,
            Intent::Move => 2u32,
        }
        .to_le_bytes()
    }
}

fn check_count(count: usize) -> Result<(), CodecError> {
    if count == 0 {
        Err(CodecError::EmptyList)
    } else if count > MAX_PATHS {
        Err(CodecError::TooManyPaths)
    } else {
        Ok(())
    }
}

fn add_path_size(total: usize, units: usize) -> Result<usize, CodecError> {
    let bytes = units
        .checked_add(1)
        .and_then(|n| n.checked_mul(2))
        .and_then(|n| total.checked_add(n))
        .ok_or(CodecError::ArithmeticOverflow)?;
    if bytes > MAX_PAYLOAD_BYTES {
        Err(CodecError::PayloadTooLarge)
    } else {
        Ok(bytes)
    }
}

fn zeroed_bytes(len: usize) -> Result<Vec<u8>, CodecError> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(len)
        .map_err(|_| CodecError::AllocationFailed)?;
    bytes.resize(len, 0);
    Ok(bytes)
}

fn native_units(path: &NativePathBuf) -> Result<Vec<u16>, CodecError> {
    let len = path.as_os_str().encode_wide().count();
    if len > MAX_PATH_UNITS {
        return Err(CodecError::PathTooLong);
    }
    let mut units = Vec::new();
    units
        .try_reserve_exact(len)
        .map_err(|_| CodecError::AllocationFailed)?;
    units.extend(path.as_os_str().encode_wide());
    Ok(units)
}

/// Missing intent means Copy with a warning. Exactly DWORD 2 means Move.
/// Malformed bytes are refused; unknown/mixed values mean Copy with a warning.
/// Zero allocation padding is accepted up to MAX_EFFECT_BYTES.
pub fn decode_preferred_effect(
    bytes: Option<&[u8]>,
) -> Result<(Intent, Option<IntentWarning>), CodecError> {
    let Some(bytes) = bytes else {
        return Ok((Intent::Copy, Some(IntentWarning::MissingPreferredEffect)));
    };
    if bytes.len() < 4 || bytes.len() > MAX_EFFECT_BYTES || bytes[4..].iter().any(|b| *b != 0) {
        return Err(CodecError::InvalidEffect);
    }
    let effect = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    Ok(match effect {
        1 => (Intent::Copy, None),
        2 => (Intent::Move, None),
        other => (
            Intent::Copy,
            Some(IntentWarning::AmbiguousPreferredEffect(other)),
        ),
    })
}

/// Validates the entire input before returning any paths. ANSI DROPFILES use
/// native ACP with invalid-byte rejection; ANSI is never interpreted as UTF-8.
/// Only zero bytes may follow the final list terminator, for allocation padding.
pub fn decode_packet(
    hdrop: &[u8],
    preferred_effect: Option<&[u8]>,
) -> Result<FilePacket, CodecError> {
    if hdrop.len() > MAX_PAYLOAD_BYTES {
        return Err(CodecError::PayloadTooLarge);
    }
    if hdrop.len() < HEADER_BYTES {
        return Err(CodecError::HeaderTooShort);
    }
    let dword = |offset| u32::from_le_bytes(hdrop[offset..offset + 4].try_into().unwrap());
    let offset = usize::try_from(dword(0)).map_err(|_| CodecError::InvalidOffset)?;
    let wide = dword(16);
    if wide > 1 || dword(12) > 1 {
        return Err(CodecError::InvalidBoolean);
    }
    if offset < HEADER_BYTES || offset >= hdrop.len() || (wide == 1 && offset % 2 != 0) {
        return Err(CodecError::InvalidOffset);
    }
    let data = &hdrop[offset..];
    if wide == 1 && data.len() % 2 != 0 {
        return Err(CodecError::OddUtf16);
    }
    let (intent, warning) = decode_preferred_effect(preferred_effect)?;
    let stride = if wide == 1 { 2 } else { 1 };
    let unit_at = |i: usize| -> u16 {
        if wide == 1 {
            u16::from_le_bytes([data[i], data[i + 1]])
        } else {
            data[i] as u16
        }
    };
    let mut paths = Vec::new();
    let mut total = HEADER_BYTES + 2;
    let mut start = 0;
    loop {
        let mut end = start;
        while end < data.len() && unit_at(end) != 0 {
            end += stride;
        }
        if end == data.len() {
            return Err(CodecError::MissingTerminator);
        }
        if end == start {
            // An empty list still needs two NULs. Never accept a truncated prefix.
            if paths.is_empty() {
                if data.len() < stride * 2 || unit_at(stride) != 0 {
                    return Err(CodecError::MissingTerminator);
                }
                return Err(CodecError::EmptyList);
            }
            if data[end..].iter().any(|b| *b != 0) {
                return Err(CodecError::TrailingData);
            }
            break;
        }
        if paths.len() >= MAX_PATHS {
            return Err(CodecError::TooManyPaths);
        }
        let units = if wide == 1 {
            let count = (end - start) / 2;
            if count > MAX_PATH_UNITS {
                return Err(CodecError::PathTooLong);
            }
            let mut units = Vec::new();
            units
                .try_reserve_exact(count)
                .map_err(|_| CodecError::AllocationFailed)?;
            units.extend(
                data[start..end]
                    .chunks_exact(2)
                    .map(|b| u16::from_le_bytes([b[0], b[1]])),
            );
            units
        } else {
            decode_ansi(&data[start..end])?
        };
        validate_path(&units)?;
        total = add_path_size(total, units.len())?;
        paths
            .try_reserve(1)
            .map_err(|_| CodecError::AllocationFailed)?;
        paths.push(PathBuf::from(OsString::from_wide(&units)));
        start = end + stride;
        if start >= data.len() {
            return Err(CodecError::MissingTerminator);
        }
    }
    Ok(FilePacket {
        paths,
        intent,
        warning,
    })
}

fn decode_ansi(bytes: &[u8]) -> Result<Vec<u16>, CodecError> {
    let len = i32::try_from(bytes.len()).map_err(|_| CodecError::ArithmeticOverflow)?;
    // SAFETY: input is a bounded valid slice; first call only measures the output.
    let count = unsafe {
        MultiByteToWideChar(
            CP_ACP,
            MB_ERR_INVALID_CHARS,
            bytes.as_ptr(),
            len,
            std::ptr::null_mut(),
            0,
        )
    };
    if count <= 0 {
        return Err(CodecError::AnsiConversion(unsafe {
            windows_sys::Win32::Foundation::GetLastError()
        }));
    }
    if count as usize > MAX_PATH_UNITS {
        return Err(CodecError::PathTooLong);
    }
    let mut units = Vec::new();
    units
        .try_reserve_exact(count as usize)
        .map_err(|_| CodecError::AllocationFailed)?;
    units.resize(count as usize, 0);
    // SAFETY: allocation matches the measured length; input is unchanged.
    let written = unsafe {
        MultiByteToWideChar(
            CP_ACP,
            MB_ERR_INVALID_CHARS,
            bytes.as_ptr(),
            len,
            units.as_mut_ptr(),
            count,
        )
    };
    if written != count {
        return Err(CodecError::AnsiConversion(unsafe {
            windows_sys::Win32::Foundation::GetLastError()
        }));
    }
    Ok(units)
}

fn ascii_eq(units: &[u16], ascii: &str) -> bool {
    units.len() == ascii.len()
        && units
            .iter()
            .zip(ascii.bytes())
            .all(|(u, a)| *u < 128 && (*u as u8).eq_ignore_ascii_case(&a))
}

fn letter(unit: u16) -> bool {
    (b'A' as u16..=b'Z' as u16).contains(&unit) || (b'a' as u16..=b'z' as u16).contains(&unit)
}
fn separator(unit: u16, verbatim: bool) -> bool {
    unit == 92 || (!verbatim && unit == 47)
}

fn volume_guid(units: &[u16]) -> bool {
    // Volume{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}, followed by a backslash.
    units.len() >= 45
        && ascii_eq(&units[..7], "Volume{")
        && units[43] == 125
        && units[44] == 92
        && units[7..43].iter().enumerate().all(|(i, u)| {
            if [8, 13, 18, 23].contains(&i) {
                *u == 45
            } else {
                *u < 128 && (*u as u8).is_ascii_hexdigit()
            }
        })
}

fn validate_path(units: &[u16]) -> Result<(), CodecError> {
    if units.len() > MAX_PATH_UNITS {
        return Err(CodecError::PathTooLong);
    }
    if units.contains(&0) {
        return Err(CodecError::InvalidPath);
    }
    let verbatim = units.starts_with(&[92, 92, 63, 92]);
    let tail = if verbatim { &units[4..] } else { units };
    let remainder =
        if tail.len() >= 3 && letter(tail[0]) && tail[1] == 58 && separator(tail[2], verbatim) {
            &tail[3..]
        } else if verbatim && tail.len() >= 4 && ascii_eq(&tail[..4], "UNC\\") {
            unc_remainder(&tail[4..], true)?
        } else if !verbatim && tail.starts_with(&[92, 92]) {
            unc_remainder(&tail[2..], false)?
        } else if verbatim && volume_guid(tail) {
            &tail[45..]
        } else {
            return Err(CodecError::InvalidNamespace);
        };
    for part in remainder.split(|u| separator(*u, verbatim)) {
        if !part.is_empty() {
            validate_component(part, verbatim)?;
        }
    }
    Ok(())
}

fn unc_remainder(units: &[u16], verbatim: bool) -> Result<&[u16], CodecError> {
    let Some(server_end) = units.iter().position(|u| separator(*u, verbatim)) else {
        return Err(CodecError::InvalidNamespace);
    };
    let server = &units[..server_end];
    let rest = &units[server_end + 1..];
    let share_end = rest
        .iter()
        .position(|u| separator(*u, verbatim))
        .unwrap_or(rest.len());
    let share = &rest[..share_end];
    if server.is_empty() || share.is_empty() {
        return Err(CodecError::InvalidNamespace);
    }
    validate_component(server, verbatim)?;
    validate_component(share, verbatim)?;
    // The UNC pipe share is a device namespace, not a filesystem share.
    if ascii_eq(share, "pipe") {
        return Err(CodecError::InvalidNamespace);
    }
    Ok(if share_end == rest.len() {
        &[]
    } else {
        &rest[share_end + 1..]
    })
}

fn validate_component(part: &[u16], verbatim: bool) -> Result<(), CodecError> {
    if ascii_eq(part, ".")
        || ascii_eq(part, "..")
        || part
            .iter()
            .any(|u| *u < 32 || [34, 42, 47, 58, 60, 62, 63, 124].contains(u))
        || (!verbatim && matches!(part.last(), Some(32 | 46)))
    {
        return Err(CodecError::InvalidPath);
    }
    // Refuse DOS device aliases even under verbatim namespaces. No device I/O.
    let stem = part.split(|u| *u == 46).next().unwrap();
    let end = stem.iter().rposition(|u| *u != 32).map_or(0, |i| i + 1);
    let stem = &stem[..end];
    if ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"]
        .iter()
        .any(|s| ascii_eq(stem, s))
        || (stem.len() == 4
            && (ascii_eq(&stem[..3], "COM") || ascii_eq(&stem[..3], "LPT"))
            && ((49..=57).contains(&stem[3]) || [0xb9, 0xb2, 0xb3].contains(&stem[3])))
    {
        return Err(CodecError::InvalidPath);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
