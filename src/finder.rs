use crate::{
    directory::normalize_destination, native, navigation::Navigation, panes::PaneId, tabs::TabId,
};
use std::{
    io,
    path::PathBuf,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

pub const SESSION_LIMIT: usize = 128;
const CHILD_LIMIT: usize = 4096;
const SCAN_LIMIT: usize = 8192;
pub const QUERY_LIMIT: usize = 4096;
pub const PATH_LIMIT: usize = 32766; // UTF-16 units, excluding the Windows terminator.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub path: PathBuf,
    pub name: String,
    pub category: &'static str,
}
impl Location {
    pub fn new(path: PathBuf, category: &'static str) -> Self {
        let name = native::display(path.file_name().unwrap_or(path.as_os_str()));
        Self {
            path,
            name,
            category,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Match {
    pub location: Location,
    pub path_text: String,
    pub name_marks: Vec<usize>,
    pub path_marks: Vec<usize>,
}
#[derive(Default)]
pub struct Catalogue {
    pub recents: Vec<Location>,
    pub bookmarks: Vec<Location>,
    pub revision: u64,
}
impl Catalogue {
    pub fn visit(&mut self, path: PathBuf) {
        if self.recents.first().is_some_and(|item| item.path == path) {
            return;
        }
        self.revision += 1;
        self.recents.retain(|item| item.path != path);
        self.recents.insert(0, Location::new(path, "Recent"));
        self.recents.truncate(SESSION_LIMIT);
    }
    pub fn toggle_bookmark(&mut self, path: PathBuf) -> Result<(), &'static str> {
        if let Some(index) = self.bookmarks.iter().position(|item| item.path == path) {
            self.bookmarks.remove(index);
        } else {
            if self.bookmarks.len() == SESSION_LIMIT {
                return Err("Session bookmark limit (128) reached; unbookmark a location first.");
            }
            self.bookmarks.push(Location::new(path, "Bookmark"));
        }
        self.revision += 1;
        Ok(())
    }
    pub fn forget(&mut self, path: &std::path::Path) {
        let count = self.recents.len();
        self.recents.retain(|item| item.path != path);
        if count != self.recents.len() {
            self.revision += 1;
        }
    }
    pub fn items(&self) -> Vec<Location> {
        self.recents
            .iter()
            .chain(&self.bookmarks)
            .cloned()
            .collect()
    }
}
#[derive(Clone, Debug)]
pub struct Target {
    pub pane: PaneId,
    pub tab: TabId,
    pub base: PathBuf,
    pub generation: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Navigate,
    NewTab,
    Split,
}
#[derive(Clone)]
pub enum Work {
    Rank {
        query: String,
        session: Vec<Location>,
        children: Option<Vec<Location>>,
    },
    Expand(PathBuf),
    Probe(PathBuf, Action),
}
pub enum Output {
    Ranked(Vec<Match>, Vec<String>),
    Expanded(PathBuf, Vec<Location>, Vec<Match>, Option<String>),
    Probed(PathBuf, Action),
    Error(String),
}
pub struct Response {
    pub revision: u64,
    pub output: Output,
}
struct Slots {
    pending: Option<(u64, Work)>,
    result: Option<Response>,
    stop: bool,
}
struct Shared {
    slots: Mutex<Slots>,
    wake: Condvar,
    latest: AtomicU64,
}
pub struct Service {
    shared: Arc<Shared>,
    thread: Option<thread::JoinHandle<()>>,
}
impl Service {
    pub fn new(repaint: Arc<dyn Fn() + Send + Sync>) -> io::Result<Self> {
        Self::with_inventory(repaint, None)
    }
    pub fn with_inventory(
        repaint: Arc<dyn Fn() + Send + Sync>,
        injected: Option<Vec<Location>>,
    ) -> io::Result<Self> {
        let shared = Arc::new(Shared {
            slots: Mutex::new(Slots {
                pending: None,
                result: None,
                stop: false,
            }),
            wake: Condvar::new(),
            latest: AtomicU64::new(0),
        });
        let worker = shared.clone();
        let handle = thread::Builder::new()
            .name("folder-finder".into())
            .spawn(move || {
                let mut inventory = injected.map(|items| (items, Vec::new()));
                loop {
                    let (revision, work) = {
                        let mut slots = worker.slots.lock().unwrap();
                        while slots.pending.is_none() && !slots.stop {
                            slots = worker.wake.wait(slots).unwrap();
                        }
                        if slots.stop {
                            break;
                        }
                        slots.pending.take().unwrap()
                    };
                    let cancelled = || worker.latest.load(Ordering::Acquire) != revision;
                    let output = match work {
                        Work::Rank {
                            query,
                            mut session,
                            children,
                        } => {
                            let warnings;
                            if let Some(children) = children {
                                session = children;
                                warnings = Vec::new();
                            } else {
                                let (places, problems) =
                                    inventory.get_or_insert_with(system_inventory);
                                session.extend(places.iter().cloned());
                                warnings = problems.clone();
                            }
                            Output::Ranked(rank(&query, session, &cancelled), warnings)
                        }
                        Work::Probe(path, action) => match probe(&path) {
                            Ok(path) => Output::Probed(path, action),
                            Err(error) => Output::Error(error),
                        },
                        Work::Expand(path) => match expand(&path, &cancelled) {
                            Ok((path, children, warning)) => Output::Expanded(
                                path,
                                children.clone(),
                                rank("", children, &cancelled),
                                warning,
                            ),
                            Err(error) => Output::Error(error),
                        },
                    };
                    let mut slots = worker.slots.lock().unwrap();
                    if !slots.stop && !cancelled() {
                        slots.result = Some(Response { revision, output });
                        drop(slots);
                        repaint();
                    }
                }
            })?;
        Ok(Self {
            shared,
            thread: Some(handle),
        })
    }
    pub fn request(&self, work: Work) -> u64 {
        let mut slots = self.shared.slots.lock().unwrap();
        let revision = self
            .shared
            .latest
            .fetch_add(1, Ordering::AcqRel)
            .checked_add(1)
            .expect("Finder revision exhausted");
        slots.pending = Some((revision, work));
        slots.result = None;
        self.shared.wake.notify_one();
        revision
    }
    pub fn cancel(&self) {
        let mut slots = self.shared.slots.lock().unwrap();
        self.shared.latest.fetch_add(1, Ordering::AcqRel);
        slots.pending = None;
        slots.result = None;
    }
    pub fn take(&self) -> Option<Response> {
        self.shared.slots.lock().unwrap().result.take()
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        self.cancel();
        self.shared.slots.lock().unwrap().stop = true;
        self.shared.wake.notify_one();
        if let Some(handle) = self.thread.take() {
            if handle.is_finished() {
                let _ = handle.join();
            }
        }
    }
}

pub struct Modal {
    pub target: Target,
    pub query: String,
    pub path_mode: bool,
    pub results: Vec<Match>,
    pub selected: usize,
    pub children: Option<Vec<Location>>,
    pub revision: u64,
    pub catalogue_revision: u64,
    pub pending: bool,
    pub error: Option<String>,
    pub notices: Vec<String>,
    pub focus_input: bool,
}
impl Modal {
    pub fn new(target: Target) -> Self {
        Self {
            target,
            query: String::new(),
            path_mode: false,
            results: Vec::new(),
            selected: 0,
            children: None,
            revision: 0,
            catalogue_revision: 0,
            pending: false,
            error: None,
            notices: Vec::new(),
            focus_input: true,
        }
    }
    pub fn selected_path(&self) -> Option<PathBuf> {
        self.results
            .get(self.selected)
            .map(|item| item.location.path.clone())
    }
    pub fn validate_input(&self) -> Result<(), String> {
        if self.path_mode {
            if self.query.encode_utf16().take(PATH_LIMIT + 1).count() > PATH_LIMIT {
                return Err("Path exceeds 32766 UTF-16 units. The complete input is retained; no truncated path will be opened.".into());
            }
        } else if self.query.chars().take(QUERY_LIMIT + 1).count() > QUERY_LIMIT {
            return Err("Fuzzy query exceeds 4096 characters. Shorten it, or enable Path entry. The complete input is retained.".into());
        }
        Ok(())
    }
    pub fn typed_path(&self) -> Result<PathBuf, String> {
        self.validate_input()?;
        if self.query.is_empty() {
            return Err(
                "Enter a folder path relative to the captured base, or an absolute path.".into(),
            );
        }
        if self.query.contains("://") {
            return Err("Enter a filesystem folder path, not a URL or remote protocol.".into());
        }
        if self.query.contains('\0') {
            return Err("Folder paths cannot contain NUL characters.".into());
        }
        Navigation::new(self.target.base.clone())
            .resolve(&self.query)
            .map_err(str::to_owned)
    }
    pub fn move_selection(&mut self, down: bool) {
        if !self.pending && !self.results.is_empty() {
            self.selected = if down {
                (self.selected + 1).min(self.results.len() - 1)
            } else {
                self.selected.saturating_sub(1)
            };
        }
    }
    pub fn accept(&mut self, response: Response) -> Option<(PathBuf, Action)> {
        if response.revision != self.revision {
            return None;
        }
        self.pending = false;
        self.error = None;
        match response.output {
            Output::Ranked(results, warnings) => {
                self.results = results;
                self.selected = 0;
                if self.children.is_none() {
                    self.notices = warnings;
                }
            }
            Output::Expanded(path, children, results, warning) => {
                self.query.clear();
                self.path_mode = false;
                self.results = results;
                self.children = Some(children);
                self.selected = 0;
                self.notices = vec![format!(
                    "Children of {} (not navigated)",
                    native::display(path.as_os_str())
                )];
                self.notices.extend(warning);
                self.focus_input = true;
            }
            Output::Error(error) => self.error = Some(error),
            Output::Probed(path, action) => return Some((path, action)),
        }
        None
    }
}

// Terms independently match a subsequence of the name or path. Lower gap/start
// penalties win, name matches precede path-only matches; native path breaks ties.
fn term_match(text: &str, term: &str) -> Option<(usize, Vec<usize>)> {
    let mut wanted = term.chars().peekable();
    let mut marks = Vec::new();
    let mut score = 0;
    let mut previous = None;
    for (index, character) in text.chars().enumerate() {
        for lower in character.to_lowercase() {
            if wanted.peek() == Some(&lower) {
                wanted.next();
                score += previous.map_or(index, |last| index.saturating_sub(last + 1));
                marks.push(index);
                previous = Some(index);
                if wanted.peek().is_none() {
                    return Some((score, marks));
                }
            }
        }
    }
    None
}
pub fn rank(query: &str, items: Vec<Location>, cancelled: &impl Fn() -> bool) -> Vec<Match> {
    let terms: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    let mut scored = Vec::new();
    for (order, location) in items.into_iter().enumerate() {
        if cancelled() {
            return Vec::new();
        }
        let path_text = native::display(location.path.as_os_str());
        let mut name_marks = Vec::new();
        let mut path_marks = Vec::new();
        let mut score = 0;
        let mut path_terms = 0;
        let matches = terms.iter().all(|term| {
            if let Some((cost, marks)) = term_match(&location.name, term) {
                score += cost;
                name_marks.extend(marks);
                true
            } else if let Some((cost, marks)) = term_match(&path_text, term) {
                score += cost;
                path_terms += 1;
                path_marks.extend(marks);
                true
            } else {
                false
            }
        });
        if matches {
            name_marks.sort_unstable();
            name_marks.dedup();
            path_marks.sort_unstable();
            path_marks.dedup();
            scored.push((
                (path_terms, score),
                order,
                Match {
                    location,
                    path_text,
                    name_marks,
                    path_marks,
                },
            ));
        }
    }
    if !terms.is_empty() {
        scored.sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then_with(|| a.2.location.name.cmp(&b.2.location.name))
                .then_with(|| a.2.location.path.cmp(&b.2.location.path))
                .then_with(|| a.1.cmp(&b.1))
        });
    }
    scored.into_iter().map(|(_, _, item)| item).collect()
}
fn probe(path: &std::path::Path) -> Result<PathBuf, String> {
    let run = || -> io::Result<PathBuf> {
        let path = normalize_destination(path)?;
        if !std::fs::metadata(&path)?.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "Not a folder"));
        }
        // Check actual enumeration access, not just metadata permission.
        let mut entries = std::fs::read_dir(&path)?;
        if let Some(entry) = entries.next() {
            entry?;
        }
        Ok(path)
    };
    run().map_err(|error| {
        format!(
            "Cannot open folder {}: {} ({:?}). Edit the path or choose another location.",
            native::display(path.as_os_str()),
            error,
            error.kind()
        )
    })
}
fn expand(
    path: &std::path::Path,
    cancelled: &impl Fn() -> bool,
) -> Result<(PathBuf, Vec<Location>, Option<String>), String> {
    let path = probe(path)?;
    let mut children = Vec::new();
    let mut warning = None;
    let entries = std::fs::read_dir(&path).map_err(|error| {
        format!(
            "Cannot expand {}: {error}",
            native::display(path.as_os_str())
        )
    })?;
    for (count, entry) in entries.enumerate() {
        if cancelled() {
            return Err("Expansion cancelled".into());
        }
        if count == SCAN_LIMIT || children.len() == CHILD_LIMIT {
            warning = Some("Expansion limited to 8192 scanned entries / 4096 folders; use an explicit path for omitted children.".into());
            break;
        }
        let entry = entry.map_err(|error| {
            format!(
                "Cannot expand {}: {error}",
                native::display(path.as_os_str())
            )
        })?;
        // Non-following metadata also catches Windows reparse points which
        // are not necessarily represented as ordinary symlinks.
        match entry.metadata() {
            Ok(metadata) if metadata.is_dir() && !crate::directory::is_link(&metadata) => {
                children.push(Location::new(entry.path(), "Child folder"))
            }
            Ok(_) => {}
            Err(error) => warning = Some(format!("Some child metadata unavailable: {error}")),
        }
    }
    children.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.path.cmp(&b.path)));
    Ok((path, children, warning))
}

#[cfg(windows)]
fn system_inventory() -> (Vec<Location>, Vec<String>) {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows_sys::Win32::{System::Com::CoTaskMemFree, UI::Shell::*};
    let mut items = Vec::new();
    let mut warnings = Vec::new();
    for (name, id) in [
        ("Home", FOLDERID_Profile),
        ("Desktop", FOLDERID_Desktop),
        ("Documents", FOLDERID_Documents),
        ("Downloads", FOLDERID_Downloads),
        ("Pictures", FOLDERID_Pictures),
        ("Music", FOLDERID_Music),
        ("Videos", FOLDERID_Videos),
    ] {
        let mut pointer = std::ptr::null_mut();
        // Current user's actual known folder; no default-path flag, so redirection
        // is respected. Free the caller-owned allocation even on failure.
        let result = unsafe { SHGetKnownFolderPath(&id, 0, std::ptr::null_mut(), &mut pointer) };
        if result >= 0 && !pointer.is_null() {
            let path = unsafe {
                let mut len = 0;
                while *pointer.add(len) != 0 {
                    len += 1;
                }
                PathBuf::from(OsString::from_wide(std::slice::from_raw_parts(
                    pointer, len,
                )))
            };
            items.push(Location {
                path,
                name: name.into(),
                category: "Common place",
            });
        } else {
            warnings.push(format!(
                "{name} unavailable from Windows (HRESULT 0x{:08X})",
                result as u32
            ));
        }
        if !pointer.is_null() {
            unsafe {
                CoTaskMemFree(pointer.cast());
            }
        }
    }
    // GetLogicalDrives lives in SystemInformation (not an enabled feature).
    // Use the same kernel32 API directly without adding dependency features.
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLogicalDrives() -> u32;
    }
    let drives = unsafe { GetLogicalDrives() };
    if drives == 0 {
        warnings.push(format!(
            "Cannot list logical storage roots: {}",
            io::Error::last_os_error()
        ));
    }
    for index in 0..26 {
        if drives & (1 << index) != 0 {
            items.push(Location::new(
                PathBuf::from(format!("{}:\\", (b'A' + index) as char)),
                "Storage root",
            ));
        }
    }
    (items, warnings)
}
#[cfg(not(windows))]
fn system_inventory() -> (Vec<Location>, Vec<String>) {
    (
        vec![Location::new(PathBuf::from("/"), "Storage root")],
        vec!["Windows common places are unavailable on this platform.".into()],
    )
}
