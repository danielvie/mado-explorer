//! Explicit restart state only. No operations, clipboard, pending navigation or editor drafts.
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

pub const MAX_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_PANES: usize = 64;
pub const MAX_TABS: usize = 256;
const MAX_DEPTH: usize = 16;
const VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "encoding", content = "units", deny_unknown_fields)]
pub enum NativePath {
    Windows(Vec<u16>),
    Unix(Vec<u8>),
}
impl NativePath {
    pub fn capture(path: &Path) -> Self {
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            Self::Windows(path.as_os_str().encode_wide().collect())
        }
        #[cfg(not(windows))]
        {
            use std::os::unix::ffi::OsStrExt;
            Self::Unix(path.as_os_str().as_bytes().to_vec())
        }
    }
    pub fn restore(&self) -> Result<PathBuf, String> {
        let path;
        #[cfg(windows)]
        {
            use std::{ffi::OsString, os::windows::ffi::OsStringExt};
            let Self::Windows(units) = self else {
                return Err("Path encoding is not Windows UTF-16".into());
            };
            if units.is_empty() || units.len() > crate::finder::PATH_LIMIT || units.contains(&0) {
                return Err("Invalid native path length or NUL".into());
            }
            path = PathBuf::from(OsString::from_wide(units));
            use std::path::{Component, Prefix};
            if !matches!(path.components().next(), Some(Component::Prefix(p)) if matches!(p.kind(), Prefix::Disk(_) | Prefix::UNC(_, _) | Prefix::VerbatimDisk(_) | Prefix::VerbatimUNC(_, _)))
            {
                return Err("Settings path is not a filesystem location".into());
            }
        }
        #[cfg(not(windows))]
        {
            use std::{ffi::OsString, os::unix::ffi::OsStringExt};
            let Self::Unix(units) = self else {
                return Err("Path encoding is not native bytes".into());
            };
            if units.is_empty() || units.len() > crate::finder::PATH_LIMIT || units.contains(&0) {
                return Err("Invalid native path length or NUL".into());
            }
            path = PathBuf::from(OsString::from_vec(units.clone()));
        }
        if !path.is_absolute() {
            return Err("Settings locations must be absolute".into());
        }
        Ok(path)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Geometry {
    // Physical outer position; logical inner size at native scale, independent of egui zoom.
    pub position: Option<[i32; 2]>,
    pub size: [f32; 2],
    pub maximized: bool,
}
impl Default for Geometry {
    fn default() -> Self {
        Self {
            position: None,
            size: [1050.0, 700.0],
            maximized: false,
        }
    }
}
impl Geometry {
    pub fn validate(&self) -> Result<(), String> {
        if !self.size.iter().all(|v| v.is_finite())
            || self.size[0] < 480.0
            || self.size[1] < 280.0
            || self.size.iter().any(|v| *v > 16384.0)
            || self
                .position
                .is_some_and(|p| p.iter().any(|v| v.unsigned_abs() > 10_000_000))
        {
            return Err("Invalid window geometry".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedPane {
    pub tabs: Vec<NativePath>,
    pub active: usize,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum SavedLayout {
    Pane {
        pane: usize,
    },
    Split {
        vertical: bool,
        ratio: f32,
        first: Box<SavedLayout>,
        second: Box<SavedLayout>,
    },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub version: u32,
    pub geometry: Geometry,
    pub panes: Vec<SavedPane>,
    pub focused: usize,
    pub layout: SavedLayout,
    pub bookmarks: Vec<NativePath>,
    pub recents: Vec<NativePath>,
}
impl Snapshot {
    pub fn new(
        geometry: Geometry,
        panes: Vec<SavedPane>,
        focused: usize,
        layout: SavedLayout,
        bookmarks: Vec<NativePath>,
        recents: Vec<NativePath>,
    ) -> Self {
        Self {
            version: VERSION,
            geometry,
            panes,
            focused,
            layout,
            bookmarks,
            recents,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.version != VERSION {
            return Err("Unsupported settings version".into());
        }
        self.geometry.validate()?;
        if self.panes.is_empty()
            || self.panes.len() > MAX_PANES
            || self.focused >= self.panes.len()
            || self.panes.iter().map(|p| p.tabs.len()).sum::<usize>() > MAX_TABS
        {
            return Err("Invalid pane, focus or tab count".into());
        }
        for pane in &self.panes {
            if pane.tabs.is_empty() || pane.active >= pane.tabs.len() {
                return Err("Invalid active tab or empty pane".into());
            }
            for path in &pane.tabs {
                path.restore()?;
            }
        }
        let mut leaves = HashSet::new();
        fn walk(
            node: &SavedLayout,
            depth: usize,
            count: usize,
            leaves: &mut HashSet<usize>,
        ) -> Result<(), String> {
            if depth > MAX_DEPTH {
                return Err("Split tree too deep".into());
            }
            match node {
                SavedLayout::Pane { pane } => {
                    if *pane >= count || !leaves.insert(*pane) {
                        return Err("Invalid or duplicate split leaf".into());
                    }
                }
                SavedLayout::Split {
                    ratio,
                    first,
                    second,
                    ..
                } => {
                    if !ratio.is_finite() || *ratio <= 0.0 || *ratio >= 1.0 {
                        return Err("Invalid split ratio".into());
                    }
                    walk(first, depth + 1, count, leaves)?;
                    walk(second, depth + 1, count, leaves)?;
                }
            }
            Ok(())
        }
        walk(&self.layout, 0, self.panes.len(), &mut leaves)?;
        if leaves.len() != self.panes.len() {
            return Err("Split tree omits panes".into());
        }
        for paths in [&self.bookmarks, &self.recents] {
            if paths.len() > crate::finder::SESSION_LIMIT {
                return Err("Location catalogue too large".into());
            }
            let mut seen = HashSet::new();
            for path in paths {
                if !seen.insert(path.restore()?) {
                    return Err("Duplicate catalogue location".into());
                }
            }
        }
        Ok(())
    }
}

pub struct Loaded {
    pub snapshot: Option<Snapshot>,
    pub warning: Option<String>,
    pub writable: bool,
}
pub fn load(path: &Path) -> Loaded {
    let result = (|| -> Result<Option<Snapshot>, String> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // A dangling settings link is unreadable existing data, not a missing file to replace.
                match fs::symlink_metadata(path) {
                    Err(missing) if missing.kind() == io::ErrorKind::NotFound => return Ok(None),
                    Err(other) => return Err(other.to_string()),
                    Ok(_) => return Err(e.to_string()),
                }
            }
            Err(e) => return Err(e.to_string()),
        };
        if file.metadata().map_err(|e| e.to_string())?.len() > MAX_BYTES as u64 {
            return Err("Settings exceed 2 MiB".into());
        }
        let mut bytes = Vec::new();
        file.take(MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        if bytes.len() > MAX_BYTES {
            return Err("Settings exceed 2 MiB".into());
        }
        let snapshot: Snapshot = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        snapshot.validate()?;
        Ok(Some(snapshot))
    })();
    match result {
        Ok(snapshot) => Loaded {
            snapshot,
            warning: None,
            writable: true,
        },
        Err(e) => Loaded {
            snapshot: None,
            warning: Some(format!(
                "Settings could not be restored: {}. Defaults loaded; original untouched. Saving this path is disabled. Repair the file and restart.",
                bounded(&e)
            )),
            writable: false,
        },
    }
}
pub fn bounded(text: &str) -> String {
    text.chars().take(2048).collect()
}

// A bounded writer prevents an oversized serialization allocation even if many native paths are long.
struct Limited(Vec<u8>);
impl Write for Limited {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_BYTES - self.0.len() {
            return Err(io::Error::other("Settings exceed 2 MiB"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct SaveError {
    pub detail: String,
    pub leftover: Option<PathBuf>,
}
impl From<io::Error> for SaveError {
    fn from(e: io::Error) -> Self {
        Self {
            detail: bounded(&e.to_string()),
            leftover: None,
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Stage {
    Write,
    Flush,
    Sync,
    Close,
    Commit,
    Cleanup,
}

pub fn save(path: &Path, snapshot: &Snapshot) -> Result<(), SaveError> {
    save_with(path, snapshot, |_| Ok(()))
}
// Injection only supplies failure checkpoints; tests still exercise the actual file writes/commit.
pub(super) fn save_with(
    path: &Path,
    snapshot: &Snapshot,
    fault: impl Fn(Stage) -> io::Result<()>,
) -> Result<(), SaveError> {
    NativePath::capture(path).restore().map_err(|e| SaveError {
        detail: e,
        leftover: None,
    })?;
    snapshot.validate().map_err(|e| SaveError {
        detail: e,
        leftover: None,
    })?;
    let mut encoded = Limited(Vec::new());
    serde_json::to_writer(&mut encoded, snapshot).map_err(|e| SaveError {
        detail: bounded(&e.to_string()),
        leftover: None,
    })?;
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("Settings need a parent directory"))?;
    fs::create_dir_all(parent)?;
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut temp = None;
    for _ in 0..16 {
        let candidate = parent.join(format!(
            ".mado-settings-{}-{}-{}.tmp",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temp = Some((candidate, file));
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.into()),
        }
    }
    let (temp, mut file) =
        temp.ok_or_else(|| io::Error::other("Could not reserve a settings temp file"))?;
    let result = (|| -> io::Result<()> {
        fault(Stage::Write)?;
        file.write_all(&encoded.0)?;
        fault(Stage::Flush)?;
        file.flush()?;
        fault(Stage::Sync)?;
        file.sync_all()?;
        Ok(())
    })();
    // Close before replacement and cleanup, including after any write failure.
    let close = close_file(file);
    let result = result
        .and(close)
        .and_then(|_| fault(Stage::Close))
        .and_then(|_| fault(Stage::Commit))
        .and_then(|_| atomic_replace(&temp, path));
    if let Err(e) = result {
        let mut error = SaveError::from(e);
        if let Err(cleanup) = fault(Stage::Cleanup).and_then(|_| fs::remove_file(&temp)) {
            error.detail = bounded(&format!(
                "{}; temp cleanup failed: {}",
                error.detail, cleanup
            ));
            error.leftover = Some(temp);
        }
        return Err(error);
    }
    Ok(())
}
#[cfg(windows)]
fn close_file(file: File) -> io::Result<()> {
    use std::os::windows::io::IntoRawHandle;
    let handle = file.into_raw_handle();
    // Ownership transferred from File; exactly one explicit close, whose error is checked.
    if unsafe { windows_sys::Win32::Foundation::CloseHandle(handle) } == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
#[cfg(not(windows))]
fn close_file(file: File) -> io::Result<()> {
    drop(file);
    Ok(())
}
#[cfg(windows)]
fn atomic_replace(temp: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };
    let src: Vec<u16> = temp.as_os_str().encode_wide().chain(Some(0)).collect();
    let dst: Vec<u16> = destination
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
    // Same-directory native replacement; never delete the destination first, never copy across volumes.
    if unsafe {
        MoveFileExW(
            src.as_ptr(),
            dst.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
#[cfg(not(windows))]
fn atomic_replace(temp: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(temp, destination)
}

pub struct Report {
    pub revision: u64,
    pub result: Result<(), SaveError>,
}
struct Slots {
    pending: Option<(u64, Snapshot)>,
    report: Option<Report>,
    failure: Option<SaveError>,
    stop: bool,
}
struct Shared {
    slots: Mutex<Slots>,
    wake: Condvar,
}
pub struct Worker {
    shared: Arc<Shared>,
    thread: Option<thread::JoinHandle<()>>,
    revision: u64,
}
impl Worker {
    pub fn new(path: PathBuf, repaint: Arc<dyn Fn() + Send + Sync>) -> io::Result<Self> {
        let shared = Arc::new(Shared {
            slots: Mutex::new(Slots {
                pending: None,
                report: None,
                failure: None,
                stop: false,
            }),
            wake: Condvar::new(),
        });
        let worker = shared.clone();
        let thread = thread::Builder::new()
            .name("workspace-settings".into())
            .spawn(move || {
                loop {
                    let work = {
                        let mut slots = worker.slots.lock().unwrap();
                        while (slots.pending.is_none() || slots.failure.is_some()) && !slots.stop {
                            slots = worker.wake.wait(slots).unwrap();
                        }
                        if slots.stop && (slots.pending.is_none() || slots.failure.is_some()) {
                            break;
                        }
                        slots.pending.take().unwrap()
                    };
                    let result = save(&path, &work.1);
                    let mut slots = worker.slots.lock().unwrap();
                    if let Err(error) = &result {
                        // Do not let a later coalesced success hide a failed cleanup or auto-retry.
                        slots.failure = Some(error.clone());
                        slots.pending = None;
                    }
                    slots.report = Some(Report {
                        revision: work.0,
                        result,
                    });
                    drop(slots);
                    repaint();
                }
            })?;
        Ok(Self {
            shared,
            thread: Some(thread),
            revision: 0,
        })
    }
    pub fn request(&mut self, snapshot: Snapshot) -> u64 {
        self.enqueue(snapshot, false)
    }
    pub fn retry(&mut self, snapshot: Snapshot) -> u64 {
        self.enqueue(snapshot, true)
    }
    fn enqueue(&mut self, snapshot: Snapshot, retry: bool) -> u64 {
        self.revision += 1;
        let mut slots = self.shared.slots.lock().unwrap();
        if retry {
            slots.failure = None;
            slots.report = None;
        }
        slots.pending = Some((self.revision, snapshot));
        self.shared.wake.notify_one();
        self.revision
    }
    pub fn take(&self) -> Option<Report> {
        self.shared.slots.lock().unwrap().report.take()
    }
    pub fn finish(&mut self) -> Result<(), SaveError> {
        self.shared.slots.lock().unwrap().stop = true;
        self.shared.wake.notify_one();
        if let Some(thread) = self.thread.take() {
            thread.join().map_err(|_| SaveError {
                detail: "Settings worker failed during shutdown".into(),
                leftover: None,
            })?;
        }
        self.shared
            .slots
            .lock()
            .unwrap()
            .failure
            .clone()
            .map_or(Ok(()), Err)
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        // ponytail: an uninterruptible filesystem call can delay exit; never detach the last save.
        if let Err(e) = self.finish() {
            eprintln!("Settings shutdown failed: {} {:?}", e.detail, e.leftover);
        }
    }
}

// A narrow smoke/portable-settings seam, never a fallback to personal settings on invalid input.
pub fn override_path(path: &Path) -> io::Result<PathBuf> {
    NativePath::capture(path)
        .restore()
        .map_err(io::Error::other)?;
    if path.file_name() != Some(std::ffi::OsStr::new("workspace.json")) {
        return Err(io::Error::other(
            "MADO_SETTINGS_PATH must end with workspace.json",
        ));
    }
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        for component in path.components() {
            match component {
                Component::Prefix(prefix) => {
                    if let Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) =
                        prefix.kind()
                    {
                        // UNC server/share are inside the prefix, not Normal components.
                        // Refuse normalization aliases before any settings file access.
                        override_component(server, false)?;
                        override_component(share, false)?;
                        if share.as_encoded_bytes().eq_ignore_ascii_case(b"pipe") {
                            return Err(io::Error::other(
                                "MADO_SETTINGS_PATH cannot use a UNC named-pipe share",
                            ));
                        }
                    }
                }
                Component::Normal(name) => override_component(name, true)?,
                _ => {}
            }
        }
    }
    Ok(path.to_owned())
}

#[cfg(windows)]
fn override_component(name: &std::ffi::OsStr, reject_dos_devices: bool) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    let units: Vec<u16> = name.encode_wide().collect();
    let mut stem: Vec<u16> = units
        .split(|c| *c == 46)
        .next()
        .unwrap()
        .iter()
        .map(|c| if (97..=122).contains(c) { c - 32 } else { *c })
        .collect();
    while stem.last() == Some(&32) {
        stem.pop();
    }
    let reserved = ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$", "CLOCK$"]
        .iter()
        .any(|s| stem.iter().copied().eq(s.encode_utf16()));
    let numbered = stem.len() == 4
        && (stem[..3] == [67, 79, 77] || stem[..3] == [76, 80, 84])
        && matches!(stem[3], 49..=57 | 0x00b9 | 0x00b2 | 0x00b3);
    // DOS device stems apply to path components, not UNC host/share names.
    if (reject_dos_devices && (reserved || numbered))
        || units.is_empty()
        || units.len() > 255
        || matches!(units.last(), Some(32 | 46))
        || units
            .iter()
            .any(|c| *c < 32 || b"<>:\"/\\|?*".iter().any(|b| *c == u16::from(*b)))
    {
        return Err(io::Error::other(
            "MADO_SETTINGS_PATH contains an invalid or reserved Windows component",
        ));
    }
    Ok(())
}

#[cfg(windows)]
pub fn default_path() -> io::Result<PathBuf> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows_sys::Win32::{
        System::Com::CoTaskMemFree,
        UI::Shell::{FOLDERID_LocalAppData, SHGetKnownFolderPath},
    };
    let mut pointer = std::ptr::null_mut();
    let result = unsafe {
        SHGetKnownFolderPath(
            &FOLDERID_LocalAppData,
            0,
            std::ptr::null_mut(),
            &mut pointer,
        )
    };
    let path = if result >= 0 && !pointer.is_null() {
        let mut len = 0;
        unsafe {
            while *pointer.add(len) != 0 {
                len += 1;
            }
        }
        Ok(PathBuf::from(OsString::from_wide(unsafe {
            std::slice::from_raw_parts(pointer, len)
        })))
    } else {
        Err(io::Error::other(format!(
            "LocalAppData unavailable (HRESULT 0x{:08X})",
            result as u32
        )))
    };
    if !pointer.is_null() {
        unsafe {
            CoTaskMemFree(pointer.cast());
        }
    }
    path.map(|p| p.join("Mado Explorer").join("workspace.json"))
}
#[cfg(not(windows))]
pub fn default_path() -> io::Result<PathBuf> {
    std::env::home_dir()
        .map(|p| p.join(".config/mado-explorer/workspace.json"))
        .ok_or_else(|| io::Error::other("Home unavailable for settings"))
}

// Physical monitor rectangles from winit, no registry edits or assumed primary-monitor origin.
pub fn recover_position(
    position: [i32; 2],
    outer: [i32; 2],
    monitors: &[[i32; 4]],
) -> Option<[i32; 2]> {
    let monitor = monitors
        .iter()
        .find(|r| {
            position[0] as i64 + 64 <= r[2] as i64
                && position[0] as i64 + outer[0] as i64 >= r[0] as i64 + 64
                && position[1] >= r[1]
                && position[1] as i64 + 32 <= r[3] as i64
        })
        .or_else(|| monitors.first())?;
    Some([
        position[0].clamp(monitor[0], (monitor[2] - outer[0]).max(monitor[0])),
        position[1].clamp(monitor[1], (monitor[3] - outer[1]).max(monitor[1])),
    ])
}
