//! Synchronous Windows-only creation. Call on a background worker after explicit user intent.
//! Capture pins a non-reparse directory chain; creation uses its retained target handle and
//! FILE_CREATE, never an existence preflight or a path-based mutation fallback.
//! Initial provider ceiling: local NTFS only. Absolute drive/UNC spellings and their
//! verbatim equivalents are validated, but UNC/remote/unknown providers are refused. Capture
//! denies data-write/delete sharing on every ancestor, so conflicting handles fail closed.
//! Attribute-only reparse conversion is still possible. Native OBJ_DONT_REPARSE prevents
//! creation from following a converted target; observed converted ancestors are refused.
//! See HANDOFF.md for provider/path restrictions and pending platform evidence.
#![cfg(windows)]

use std::{ffi::OsStr, fmt, io, os::windows::ffi::OsStrExt, path::PathBuf};

mod windows;
pub use windows::{CapturedDirectory, Identity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameError {
    Empty,
    Dot,
    IllegalCharacter,
    TrailingDotOrSpace,
    Reserved,
    TooLong,
}
impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "A name cannot be empty",
            Self::Dot => "Dot and dot-dot are not entry names",
            Self::IllegalCharacter => "A name cannot contain controls or < > : \" / \\ | ? *",
            Self::TrailingDotOrSpace => "A name cannot end in a dot or space",
            Self::Reserved => "This name is reserved by the supported Windows name policy",
            Self::TooLong => "A name cannot exceed 255 UTF-16 code units",
        })
    }
}
#[derive(Debug)]
pub enum Error {
    InvalidName(NameError),
    UnsupportedPath(PathBuf),
    UnsupportedProvider(PathBuf),
    ReparsePoint(PathBuf),
    NotDirectory(PathBuf),
    Io {
        action: &'static str,
        path: PathBuf,
        source: io::Error,
    },
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(reason) => reason.fmt(f),
            Self::UnsupportedPath(path) => write!(
                f,
                "Unsupported absolute filesystem path: {}",
                path.display()
            ),
            Self::UnsupportedProvider(path) => write!(
                f,
                "Initial creation backend requires a local NTFS directory with native identity: {}",
                path.display()
            ),
            Self::ReparsePoint(path) => {
                write!(f, "Reparse directory is unsupported: {}", path.display())
            }
            Self::NotDirectory(path) => write!(f, "Target is not a directory: {}", path.display()),
            Self::Io {
                action,
                path,
                source,
            } => write!(f, "{action} {}: {source}", path.display()),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Lossless UTF-16 validation. Supported unpaired surrogates are not replaced or rejected.
pub fn validate_name(name: &OsStr) -> Result<(), Error> {
    validate_units(&name.encode_wide().take(256).collect::<Vec<_>>())
}
fn ascii_eq(units: &[u16], word: &str) -> bool {
    units.len() == word.len()
        && units
            .iter()
            .zip(word.bytes())
            .all(|(&a, b)| a <= 127 && (a as u8).eq_ignore_ascii_case(&b))
}
fn validate_spelling(name: &[u16]) -> Result<(), Error> {
    let invalid = |reason| Err(Error::InvalidName(reason));
    if name.is_empty() {
        return invalid(NameError::Empty);
    }
    if name == [46] || name == [46, 46] {
        return invalid(NameError::Dot);
    }
    if name.len() > 255 {
        return invalid(NameError::TooLong);
    }
    if name.iter().any(|&c| {
        c < 32 || (127..=159).contains(&c) || b"<>:\"/\\|?*".iter().any(|&bad| c == u16::from(bad))
    }) {
        return invalid(NameError::IllegalCharacter);
    }
    if matches!(name.last(), Some(32 | 46)) {
        return invalid(NameError::TrailingDotOrSpace);
    }
    Ok(())
}
fn validate_units(name: &[u16]) -> Result<(), Error> {
    validate_spelling(name)?;
    let mut stem = name.split(|&c| c == 46).next().unwrap();
    while stem.last() == Some(&32) {
        stem = &stem[..stem.len() - 1];
    }
    if ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$", "CLOCK$"]
        .iter()
        .any(|word| ascii_eq(stem, word))
        || (stem.len() == 4
            && (ascii_eq(&stem[..3], "COM") || ascii_eq(&stem[..3], "LPT"))
            && matches!(stem[3], 48..=57 | 0x00b9 | 0x00b2 | 0x00b3))
    {
        return Err(Error::InvalidName(NameError::Reserved));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
