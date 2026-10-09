//! Synchronous Windows create/rename backend. Run on an operation worker, never the UI thread.
//! Names and paths stay native. Rename is confined to the captured parent directory.
//! Reparse entries/ancestors and special device namespaces are deliberately unsupported.
//! Rename is DISABLED by default pending a native hard-link collision fix and safety review.
//! The unreviewed-native-rename feature is for disposable diagnostic probes only.

#[cfg(not(windows))]
use std::ffi::OsStr;
use std::{fmt, io, path::PathBuf};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::{CapturedRename, create_directory, create_empty_file, validate_name};

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
            Self::IllegalCharacter => {
                "A name cannot contain control characters or < > : \" / \\ | ? *"
            }
            Self::TrailingDotOrSpace => "A name cannot end in a dot or space",
            Self::Reserved => "This name is reserved by Windows",
            Self::TooLong => "A name cannot exceed 255 UTF-16 code units",
        })
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidName(NameError),
    UnsupportedPath(PathBuf),
    ReparsePoint(PathBuf),
    SourceReplaced(PathBuf),
    RenameSafetyBlocked,
    /// The OS returned success, but the final spelling could not be confirmed.
    /// Reconcile both paths before retrying. The rename may already have committed.
    CommitUnverified {
        source: PathBuf,
        destination: PathBuf,
        detail: Box<Error>,
    },
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
            Self::RenameSafetyBlocked => f.write_str("Rename disabled: native hard-link collision safety blocker requires supervisor review"),
            Self::UnsupportedPath(path) => write!(
                f,
                "Unsupported absolute filesystem path: {}",
                path.display()
            ),
            Self::ReparsePoint(path) => {
                write!(f, "Reparse points are unsupported: {}", path.display())
            }
            Self::SourceReplaced(path) => {
                write!(f, "Captured source was replaced: {}", path.display())
            }
            Self::CommitUnverified {
                source,
                destination,
                detail,
            } => write!(
                f,
                "Rename may have committed from {} to {}; reconcile before retrying: {detail}",
                source.display(),
                destination.display()
            ),
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
            Self::InvalidName(_)
            | Self::UnsupportedPath(_)
            | Self::ReparsePoint(_)
            | Self::SourceReplaced(_)
            | Self::RenameSafetyBlocked => None,
            Self::Io { source, .. } => Some(source),
            Self::CommitUnverified { detail, .. } => Some(detail.as_ref()),
        }
    }
}

// This checks native UTF-16 without requiring valid Unicode or doing lossy conversions.
fn validate_units(name: &[u16]) -> Result<(), Error> {
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
    if name
        .iter()
        .any(|c| *c < 32 || b"<>:\"/\\|?*".iter().any(|bad| *c == u16::from(*bad)))
    {
        return invalid(NameError::IllegalCharacter);
    }
    if matches!(name.last(), Some(32 | 46)) {
        return invalid(NameError::TrailingDotOrSpace);
    }
    let stem = name.split(|c| *c == 46).next().unwrap();
    let end = stem.iter().rposition(|c| *c != 32).map_or(0, |i| i + 1);
    let upper: Vec<u16> = stem[..end]
        .iter()
        .map(|c| if (97..=122).contains(c) { c - 32 } else { *c })
        .collect();
    let reserved = ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$", "CLOCK$"]
        .iter()
        .any(|word| upper.iter().copied().eq(word.encode_utf16()));
    let numbered = upper.len() == 4
        && (upper[..3] == [67, 79, 77] || upper[..3] == [76, 80, 84])
        && matches!(upper[3], 49..=57 | 0x00b9 | 0x00b2 | 0x00b3);
    if reserved || numbered {
        return invalid(NameError::Reserved);
    }
    Ok(())
}

// Non-Windows builds intentionally expose no mutating entry points.
#[cfg(not(windows))]
pub fn validate_name(name: &OsStr) -> Result<(), Error> {
    let name = name
        .to_str()
        .ok_or(Error::InvalidName(NameError::IllegalCharacter))?;
    validate_units(&name.encode_utf16().collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_name_rules() {
        for name in [
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
            "COM1",
            "Lpt9.log",
            "COM¹",
            "LPT².txt",
            "COM³",
            "CONIN$",
            "conout$.txt",
            "CLOCK$",
        ] {
            assert!(
                validate_units(&name.encode_utf16().collect::<Vec<_>>()).is_err(),
                "{name:?}"
            );
        }
        for name in [
            "README",
            "data.txt",
            "résumé-資料-𐐀",
            "COM0",
            "COM10",
            "LPT0",
            "console",
            ".gitignore",
            " name",
            "a..b",
        ] {
            assert!(
                validate_units(&name.encode_utf16().collect::<Vec<_>>()).is_ok(),
                "{name:?}"
            );
        }
        assert!(validate_units(&vec![97; 255]).is_ok());
        assert!(matches!(
            validate_units(&vec![97; 256]),
            Err(Error::InvalidName(NameError::TooLong))
        ));
        assert!(validate_units(&[0xd800, 97]).is_ok());
    }
}
