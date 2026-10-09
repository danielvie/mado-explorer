//! Read-only Windows Keep both name planning. All calls that observe the filesystem
//! are synchronous and must run on a future operation worker, never the UI thread.
//!
//! A returned candidate was absent at one observation, followed by a matching directory
//! identity observation. This does not reserve it, pin ancestors, or authorize a commit.
//! A future executor must treat later destination creation as a NEW conflict and obtain
//! a fresh choice, never silently replace it. No source contents are read here.
#![cfg(windows)]

use mado_fs_source_snapshot::{EntryKind as ObservedKind, Snapshot, SnapshotError};
use std::{
    ffi::{OsStr, OsString},
    fmt, fs, io,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};

pub const MAX_COMPONENT_UNITS: usize = 255;
/// Checks numbers 2 through 1001, then returns Exhausted. No unbounded scan.
pub const MAX_CANDIDATES: u32 = 1000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryKind {
    File,
    Folder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NameRejection {
    Empty,
    TooLong,
    InvalidUnit,
    TrailingDotOrSpace,
    ReservedDosName,
}

#[derive(Debug)]
pub enum PlanError {
    InvalidName(NameRejection),
    CannotFit { number: u32, extension_units: usize },
    Destination(SnapshotError),
    NotDirectory(ObservedKind),
    CandidateIo { path: PathBuf, error: io::Error },
    Exhausted { checked: u32 },
}
impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(reason) => {
                write!(f, "Keep both basename rejected before I/O: {reason:?}")
            }
            Self::CannotFit {
                number,
                extension_units,
            } => write!(
                f,
                "Keep both suffix {number} and complete extension of {extension_units} UTF-16 units cannot fit"
            ),
            Self::Destination(error) => {
                write!(f, "Captured destination observation failed: {error}")
            }
            Self::NotDirectory(kind) => {
                write!(f, "Destination must be an ordinary directory, got {kind:?}")
            }
            Self::CandidateIo { path, error } => {
                write!(f, "Keep both metadata failed for {path:?}: {error}")
            }
            Self::Exhausted { checked } => {
                write!(f, "Keep both exhausted {checked} occupied candidate names")
            }
        }
    }
}
impl std::error::Error for PlanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Destination(error) => Some(error),
            Self::CandidateIo { error, .. } => Some(error),
            _ => None,
        }
    }
}

/// Native basename and full native destination, not display text or commit authority.
#[derive(Debug, Eq, PartialEq)]
pub struct Candidate {
    basename: OsString,
    path: PathBuf,
    number: u32,
}
impl Candidate {
    pub fn basename(&self) -> &OsStr {
        &self.basename
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn number(&self) -> u32 {
        self.number
    }
}

/// Immutable input and fixed destination directory observation. No setters or retargeting.
#[derive(Debug)]
pub struct Planner {
    original_basename: OsString,
    kind: EntryKind,
    units: Vec<u16>,
    extension_start: usize,
    destination: Snapshot,
}
impl Planner {
    /// Validates the basename and first suffix before any I/O, then captures the directory.
    /// Final reparse destinations are refused. Ancestor reparse points are not pinned.
    /// Ordinary long paths must be supplied in the accepted snapshot's verbatim spelling.
    pub fn capture(
        original_basename: OsString,
        kind: EntryKind,
        destination: PathBuf,
    ) -> Result<Self, PlanError> {
        let units: Vec<_> = original_basename.encode_wide().collect();
        validate_name(&units).map_err(PlanError::InvalidName)?;
        // Equivalent to Path::extension's last-suffix rule for admitted native basenames.
        // A leading dot alone is not an extension; folders never split an extension.
        let extension_start = match kind {
            EntryKind::File => units.iter().rposition(|&u| u == 46).filter(|&i| i > 0),
            EntryKind::Folder => None,
        }
        .unwrap_or(units.len());
        numbered_name(&units, extension_start, 2)?;
        #[cfg(test)]
        CAPTURE_COUNT.with(|count| count.set(count.get() + 1));
        let destination = Snapshot::capture(destination).map_err(PlanError::Destination)?;
        if destination.kind() != ObservedKind::Directory {
            return Err(PlanError::NotDirectory(destination.kind()));
        }
        Ok(Self {
            original_basename,
            kind,
            units,
            extension_start,
            destination,
        })
    }
    pub fn original_basename(&self) -> &OsStr {
        &self.original_basename
    }
    pub fn kind(&self) -> EntryKind {
        self.kind
    }
    pub fn destination(&self) -> &Path {
        self.destination.original_path()
    }

    /// Produces `stem (2).extension`, increasing the number on occupancy. The original
    /// unsuffixed name is never selected. Case behavior comes only from native metadata,
    /// not normalization, Unicode folding, or guessed filesystem case sensitivity.
    pub fn plan(&self) -> Result<Candidate, PlanError> {
        self.plan_with_metadata(nonfollowing_metadata)
    }

    // Tiny private metadata-call seam for deterministic error and directory-race checks.
    // Directory capture/revalidation always uses the real accepted native Snapshot.
    fn plan_with_metadata(
        &self,
        mut metadata: impl FnMut(&Path) -> io::Result<fs::Metadata>,
    ) -> Result<Candidate, PlanError> {
        self.destination
            .revalidate_original_name()
            .map_err(PlanError::Destination)?;
        for number in 2..=MAX_CANDIDATES + 1 {
            let basename = numbered_name(&self.units, self.extension_start, number)?;
            let path = self.destination().join(&basename);
            match metadata(&path) {
                Ok(_) => continue, // Every non-following entry kind, including dangling links.
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    self.destination
                        .revalidate_original_name()
                        .map_err(PlanError::Destination)?;
                    return Ok(Candidate {
                        basename,
                        path,
                        number,
                    });
                }
                Err(error) => return Err(PlanError::CandidateIo { path, error }),
            }
        }
        Err(PlanError::Exhausted {
            checked: MAX_CANDIDATES,
        })
    }
}

fn nonfollowing_metadata(path: &Path) -> io::Result<fs::Metadata> {
    #[cfg(test)]
    METADATA_COUNT.with(|count| count.set(count.get() + 1));
    fs::symlink_metadata(path)
}

fn numbered_name(
    units: &[u16],
    extension_start: usize,
    number: u32,
) -> Result<OsString, PlanError> {
    let suffix: Vec<_> = format!(" ({number})").encode_utf16().collect();
    let extension = &units[extension_start..];
    let room = MAX_COMPONENT_UNITS
        .checked_sub(extension.len() + suffix.len())
        .ok_or(PlanError::CannotFit {
            number,
            extension_units: extension.len(),
        })?;
    let mut stem_end = extension_start.min(room);
    // Retain native units exactly. Never cut between a valid high/low surrogate pair.
    if stem_end > 0
        && stem_end < extension_start
        && matches!(units[stem_end - 1], 0xd800..=0xdbff)
        && matches!(units[stem_end], 0xdc00..=0xdfff)
    {
        stem_end -= 1;
    }
    Ok(OsString::from_wide(
        &units[..stem_end]
            .iter()
            .copied()
            .chain(suffix)
            .chain(extension.iter().copied())
            .collect::<Vec<_>>(),
    ))
}

fn ascii_eq(units: &[u16], ascii: &str) -> bool {
    units.len() == ascii.len()
        && units
            .iter()
            .zip(ascii.bytes())
            .all(|(&a, b)| a <= 127 && (a as u8).eq_ignore_ascii_case(&b))
}
fn validate_name(units: &[u16]) -> Result<(), NameRejection> {
    use NameRejection::*;
    if units.is_empty() {
        return Err(Empty);
    }
    if units.len() > MAX_COMPONENT_UNITS {
        return Err(TooLong);
    }
    if units
        .iter()
        .any(|&u| u < 32 || matches!(u, 34 | 42 | 47 | 58 | 60 | 62 | 63 | 92 | 124))
    {
        return Err(InvalidUnit);
    }
    if matches!(units.last(), Some(32 | 46)) {
        return Err(TrailingDotOrSpace);
    }
    let first_dot = units.iter().position(|&u| u == 46).unwrap_or(units.len());
    let mut stem = &units[..first_dot];
    while stem.last() == Some(&32) {
        stem = &stem[..stem.len() - 1];
    }
    // Match the accepted snapshot's conservative DOS policy, including COM0/LPT0.
    if ["CON", "PRN", "AUX", "NUL", "CLOCK$", "CONIN$", "CONOUT$"]
        .iter()
        .any(|name| ascii_eq(stem, name))
        || (stem.len() == 4
            && (ascii_eq(&stem[..3], "COM") || ascii_eq(&stem[..3], "LPT"))
            && matches!(stem[3], 48..=57 | 0x00b9 | 0x00b2 | 0x00b3))
    {
        return Err(ReservedDosName);
    }
    Ok(())
}

#[cfg(test)]
thread_local! {
    static CAPTURE_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static METADATA_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
#[cfg(test)]
mod tests;
