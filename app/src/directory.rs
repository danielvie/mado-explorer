//! Lossless filesystem listings; one pending request and one result per browser.
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    cmp::Ordering,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering as AtomicOrdering},
    },
    time::{Duration, Instant, SystemTime},
};

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub name: OsString,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
    pub hidden: bool,
    pub system: bool,
}
impl Entry {
    pub fn depth(&self, root: &Path) -> usize {
        self.path.strip_prefix(root).map_or(0, |relative| {
            relative.components().count().saturating_sub(1)
        })
    }
}
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub error: Option<String>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortColumn {
    #[default]
    Name,
    Size,
    Modified,
    Type,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListOptions {
    pub filter: String,
    pub show_hidden: bool,
    pub show_system: bool,
    pub directories_first: bool,
    pub descending: bool,
    pub sort: SortColumn,
    pub expanded: bool,
}
impl Default for ListOptions {
    fn default() -> Self {
        Self {
            filter: String::new(),
            show_hidden: false,
            show_system: false,
            directories_first: true,
            descending: false,
            sort: SortColumn::Name,
            expanded: false,
        }
    }
}
struct Request {
    generation: u64,
    path: PathBuf,
    options: ListOptions,
}
#[derive(Default)]
struct Mailbox {
    request: Option<Request>,
    result: Option<(u64, Snapshot)>,
    stop: bool,
}
struct Shared {
    mailbox: Mutex<Mailbox>,
    ready: Condvar,
    generation: AtomicU64,
    dirty: AtomicBool,
    watch_failed: AtomicBool,
    watch_error: Mutex<Option<String>>,
}
pub struct Browser {
    shared: Arc<Shared>,
    path: PathBuf,
    snapshot: Option<Snapshot>,
    loading: bool,
    options: ListOptions,
    last_refresh: Instant,
    auto_refresh: bool,
}
impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}
impl Browser {
    pub fn new() -> Self {
        let shared = Arc::new(Shared {
            mailbox: Mutex::new(Mailbox::default()),
            ready: Condvar::new(),
            generation: AtomicU64::new(0),
            dirty: AtomicBool::new(false),
            watch_failed: AtomicBool::new(false),
            watch_error: Mutex::new(None),
        });
        let worker = shared.clone();
        std::thread::spawn(move || {
            let mut watcher: Option<RecommendedWatcher>;
            loop {
                let request = {
                    let mut mail = worker.mailbox.lock().unwrap_or_else(|e| e.into_inner());
                    while mail.request.is_none() && !mail.stop {
                        mail = worker.ready.wait(mail).unwrap_or_else(|e| e.into_inner());
                    }
                    if mail.stop {
                        return;
                    }
                    mail.request.take().expect("pending request")
                };
                // Watch lifetime belongs to this browser worker, never the UI thread.
                // Replace before enumeration so changes during a scan request another pass.
                let callback_state = worker.clone();
                let generation = request.generation;
                worker.watch_failed.store(false, AtomicOrdering::Relaxed);
                *worker.watch_error.lock().unwrap_or_else(|e| e.into_inner()) = None;
                watcher = match notify::recommended_watcher(
                    move |event: notify::Result<notify::Event>| {
                        if callback_state.generation.load(AtomicOrdering::Relaxed) != generation {
                            return;
                        }
                        match event {
                            Ok(event) if matches!(event.kind, EventKind::Access(_)) => (),
                            Ok(_) => {
                                callback_state.dirty.store(true, AtomicOrdering::Relaxed);
                            }
                            Err(error) => {
                                *callback_state
                                    .watch_error
                                    .lock()
                                    .unwrap_or_else(|e| e.into_inner()) = Some(format!(
                                    "Watcher error; periodic rescan enabled: {error}"
                                ));
                                callback_state
                                    .watch_failed
                                    .store(true, AtomicOrdering::Relaxed);
                                callback_state.dirty.store(true, AtomicOrdering::Relaxed);
                            }
                        }
                    },
                ) {
                    Ok(watcher) => Some(watcher),
                    Err(error) => {
                        *worker.watch_error.lock().unwrap_or_else(|e| e.into_inner()) = Some(
                            format!("Watcher unavailable; periodic rescan enabled: {error}"),
                        );
                        worker.watch_failed.store(true, AtomicOrdering::Relaxed);
                        None
                    }
                };
                if let Some(watcher) = watcher.as_mut() {
                    let mode = if request.options.expanded {
                        RecursiveMode::Recursive
                    } else {
                        RecursiveMode::NonRecursive
                    };
                    if let Err(error) = watcher.watch(&request.path, mode) {
                        *worker.watch_error.lock().unwrap_or_else(|e| e.into_inner()) = Some(
                            format!("Cannot watch folder; periodic rescan enabled: {error}"),
                        );
                        worker.watch_failed.store(true, AtomicOrdering::Relaxed);
                    }
                }
                let snapshot = enumerate(&request.path, &request.options, || {
                    worker.generation.load(AtomicOrdering::Relaxed) != request.generation
                });
                if let Some(snapshot) = snapshot {
                    let mut mail = worker.mailbox.lock().unwrap_or_else(|e| e.into_inner());
                    if !mail.stop
                        && worker.generation.load(AtomicOrdering::Relaxed) == request.generation
                    {
                        mail.result = Some((request.generation, snapshot));
                    }
                }
            }
        });
        Self {
            shared,
            path: PathBuf::new(),
            snapshot: None,
            loading: false,
            options: ListOptions::default(),
            last_refresh: Instant::now(),
            auto_refresh: true,
        }
    }
    /// Seed an immutable listing for headless view verification; no filesystem work is requested.
    #[doc(hidden)]
    pub fn from_snapshot(snapshot: Snapshot) -> Self {
        let mut browser = Self::new();
        browser.path = snapshot.path.clone();
        browser.snapshot = Some(snapshot);
        browser
    }
    pub fn navigate(&mut self, path: PathBuf) {
        self.path = path;
        self.auto_refresh = true;
        self.last_refresh = Instant::now();
        self.shared.dirty.store(false, AtomicOrdering::Relaxed);
        let generation = self.shared.generation.fetch_add(1, AtomicOrdering::Relaxed) + 1;
        self.loading = true;
        self.snapshot = None;
        let mut mail = self
            .shared
            .mailbox
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        mail.result = None;
        mail.request = Some(Request {
            generation,
            path: self.path.clone(),
            options: self.options.clone(),
        });
        self.shared.ready.notify_one();
    }
    /// Stop accepting the current scan; OS calls already in progress finish on the worker.
    pub fn cancel(&mut self) {
        self.shared.generation.fetch_add(1, AtomicOrdering::Relaxed);
        self.shared.dirty.store(false, AtomicOrdering::Relaxed);
        self.auto_refresh = false;
        self.loading = false;
        self.snapshot = Some(Snapshot {
            path: self.path.clone(),
            entries: Vec::new(),
            error: Some("Listing cancelled; refresh to resume".into()),
        });
        let mut mail = self
            .shared
            .mailbox
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        mail.request = None;
        mail.result = None;
    }
    pub fn set_options(&mut self, options: ListOptions) {
        if self.options != options {
            self.options = options;
            self.navigate(self.path.clone());
        }
    }
    pub fn options(&self) -> &ListOptions {
        &self.options
    }
    pub fn poll(&mut self) -> bool {
        let due = self.last_refresh.elapsed() >= Duration::from_millis(250);
        let retry = self.shared.watch_failed.load(AtomicOrdering::Relaxed)
            && self.last_refresh.elapsed() >= Duration::from_secs(10);
        if self.auto_refresh
            && !self.loading
            && !self.path.as_os_str().is_empty()
            && due
            && (self.shared.dirty.swap(false, AtomicOrdering::Relaxed) || retry)
        {
            self.navigate(self.path.clone());
            return true;
        }
        let result = self
            .shared
            .mailbox
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .result
            .take();
        if let Some((generation, snapshot)) = result
            && generation == self.generation()
        {
            self.snapshot = Some(snapshot);
            self.loading = false;
            return true;
        }
        false
    }
    pub fn watcher_error(&self) -> Option<String> {
        self.shared
            .watch_error
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
    pub fn snapshot(&self) -> Option<&Snapshot> {
        self.snapshot.as_ref()
    }
    pub fn is_loading(&self) -> bool {
        self.loading
    }
    pub fn current_path(&self) -> &Path {
        &self.path
    }
    pub fn generation(&self) -> u64 {
        self.shared.generation.load(AtomicOrdering::Relaxed)
    }
}
impl Drop for Browser {
    fn drop(&mut self) {
        self.shared.generation.fetch_add(1, AtomicOrdering::Relaxed);
        let mut mail = self
            .shared
            .mailbox
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        mail.stop = true;
        mail.request = None;
        mail.result = None;
        self.shared.ready.notify_one();
    }
}
fn enumerate(path: &Path, options: &ListOptions, cancelled: impl Fn() -> bool) -> Option<Snapshot> {
    let mut snapshot = Snapshot {
        path: path.to_owned(),
        entries: Vec::new(),
        error: None,
    };
    let mut pending = vec![path.to_path_buf()];
    while let Some(folder) = pending.pop() {
        if cancelled() {
            return None;
        }
        let directory = match std::fs::read_dir(&folder) {
            Ok(value) => value,
            Err(error) => {
                snapshot.error = Some(format!(
                    "Incomplete listing at {}: {error}",
                    folder.display()
                ));
                continue;
            }
        };
        for item in directory {
            if cancelled() {
                return None;
            }
            let item = match item {
                Ok(value) => value,
                Err(error) => {
                    snapshot.error = Some(format!("Incomplete listing: {error}"));
                    continue;
                }
            };
            // symlink_metadata preserves reparse identity; never resolve descendants through it.
            let metadata = match std::fs::symlink_metadata(item.path()) {
                Ok(value) => value,
                Err(error) => {
                    snapshot.error = Some(format!(
                        "Metadata unavailable for {:?}: {error}",
                        item.file_name()
                    ));
                    continue;
                }
            };
            #[cfg(windows)]
            let (hidden, system, reparse) = {
                use std::os::windows::fs::MetadataExt;
                let attributes = metadata.file_attributes();
                (
                    attributes & 2 != 0,
                    attributes & 4 != 0,
                    attributes & 0x400 != 0,
                )
            };
            #[cfg(not(windows))]
            let (hidden, system, reparse) = (false, false, metadata.file_type().is_symlink());
            if options.expanded
                && metadata.is_dir()
                && !reparse
                && (options.show_hidden || !hidden)
                && (options.show_system || !system)
            {
                pending.push(item.path());
            }
            snapshot.entries.push(Entry {
                path: item.path(),
                name: item.file_name(),
                is_dir: metadata.is_dir(),
                size: (!metadata.is_dir()).then_some(metadata.len()),
                modified: metadata.modified().ok(),
                hidden,
                system,
            });
        }
    }
    if cancelled() {
        return None;
    }
    filter_sort(&mut snapshot.entries, options);
    if cancelled() { None } else { Some(snapshot) }
}
/// Display-only lowercase substring filter. Windows logical ordering is followed
/// by a raw UTF-16 ordinal tie and authoritative path tie. Descending reverses
/// column order but never directory grouping. Run on a worker for large inputs.
pub fn filter_sort(entries: &mut Vec<Entry>, options: &ListOptions) {
    let filter = options.filter.to_lowercase();
    entries.retain(|entry| {
        (options.show_hidden || !entry.hidden)
            && (options.show_system || !entry.system)
            && entry
                .name
                .to_string_lossy()
                .to_lowercase()
                .contains(&filter)
    });
    entries.sort_by(|a, b| {
        let group = if options.directories_first {
            b.is_dir.cmp(&a.is_dir)
        } else {
            Ordering::Equal
        };
        if group != Ordering::Equal {
            return group;
        }
        let column = match options.sort {
            SortColumn::Name => Ordering::Equal,
            SortColumn::Size => a.size.cmp(&b.size),
            SortColumn::Modified => a.modified.cmp(&b.modified),
            SortColumn::Type => a.path.extension().cmp(&b.path.extension()),
        };
        let order = column
            .then_with(|| logical_name_cmp(&a.name, &b.name))
            .then_with(|| a.path.cmp(&b.path));
        if options.descending {
            order.reverse()
        } else {
            order
        }
    });
}
#[cfg(windows)]
fn logical_name_cmp(a: &OsStr, b: &OsStr) -> Ordering {
    use std::os::windows::ffi::OsStrExt;
    let mut left: Vec<u16> = a.encode_wide().collect();
    let mut right: Vec<u16> = b.encode_wide().collect();
    let ordinal = left.cmp(&right);
    if left.contains(&0) || right.contains(&0) {
        return ordinal;
    }
    left.push(0);
    right.push(0);
    // SAFETY: Both immutable UTF-16 buffers are NUL terminated and live for the call.
    let logical = unsafe {
        windows::Win32::UI::Shell::StrCmpLogicalW(
            windows::core::PCWSTR(left.as_ptr()),
            windows::core::PCWSTR(right.as_ptr()),
        )
    };
    logical.cmp(&0).then(ordinal)
}
#[cfg(not(windows))]
fn logical_name_cmp(a: &OsStr, b: &OsStr) -> Ordering {
    a.cmp(b)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directory_empty_error_and_stale_delivery() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-directory-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        assert!(
            enumerate(&root, &ListOptions::default(), || false)
                .unwrap()
                .entries
                .is_empty()
        );
        assert!(
            enumerate(&root.join("absent"), &ListOptions::default(), || false)
                .unwrap()
                .error
                .is_some()
        );
        let mut browser = Browser::new();
        let old = browser.generation();
        browser
            .shared
            .generation
            .fetch_add(1, AtomicOrdering::Relaxed);
        browser.shared.mailbox.lock().unwrap().result = Some((
            old,
            Snapshot {
                path: root.clone(),
                entries: Vec::new(),
                error: Some("stale".into()),
            },
        ));
        assert!(!browser.poll());
        assert!(browser.snapshot().is_none());
        drop(browser);
        std::fs::remove_dir(root).unwrap();
    }
    #[test]
    fn expanded_listing_duplicate_names_depth_and_cancellation() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-expanded-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        for folder in ["left", "right"] {
            std::fs::create_dir(root.join(folder)).unwrap();
            std::fs::write(root.join(folder).join("same.txt"), b"fixture").unwrap();
        }
        let options = ListOptions {
            expanded: true,
            ..ListOptions::default()
        };
        let snapshot = enumerate(&root, &options, || false).unwrap();
        assert_eq!(snapshot.entries.len(), 4);
        let leaves: Vec<_> = snapshot
            .entries
            .iter()
            .filter(|entry| entry.name == "same.txt")
            .collect();
        assert_eq!(leaves.len(), 2);
        assert_ne!(leaves[0].path, leaves[1].path);
        assert!(leaves.iter().all(|entry| entry.depth(&root) == 1));
        assert!(enumerate(&root, &options, || true).is_none());
        for folder in ["left", "right"] {
            std::fs::remove_file(root.join(folder).join("same.txt")).unwrap();
            std::fs::remove_dir(root.join(folder)).unwrap();
        }
        std::fs::remove_dir(root).unwrap();
    }
    #[test]
    fn watcher_invalidation_coalesces_until_debounce() {
        let mut browser = Browser::new();
        browser.path = PathBuf::from(".");
        browser.shared.dirty.store(true, AtomicOrdering::Relaxed);
        assert!(!browser.poll());
        assert!(browser.shared.dirty.load(AtomicOrdering::Relaxed));
        browser.last_refresh = Instant::now() - Duration::from_secs(1);
        assert!(browser.poll());
        assert_eq!(browser.generation(), 1);
        assert!(browser.is_loading());
        browser.shared.dirty.store(true, AtomicOrdering::Relaxed);
        browser.poll();
        assert_eq!(browser.generation(), 1);
    }
    #[test]
    fn expanded_listing_cancel_stops_publication_until_refresh() {
        let mut browser = Browser::from_snapshot(Snapshot {
            path: PathBuf::from("."),
            entries: Vec::new(),
            error: None,
        });
        let generation = browser.generation();
        browser.cancel();
        assert!(browser.generation() > generation);
        assert!(!browser.is_loading());
        assert!(
            browser
                .snapshot()
                .unwrap()
                .error
                .as_deref()
                .unwrap()
                .contains("cancelled")
        );
        browser.shared.dirty.store(true, AtomicOrdering::Relaxed);
        browser.last_refresh = Instant::now() - Duration::from_secs(20);
        assert!(!browser.poll());
        assert!(!browser.is_loading());
    }
    #[test]
    fn directory_cancelled_scan() {
        assert!(enumerate(Path::new("."), &ListOptions::default(), || true).is_none());
    }
    #[cfg(windows)]
    #[test]
    fn directory_lossless_utf16_and_logical_sort() {
        use std::os::windows::ffi::{OsStrExt, OsStringExt};
        let name = OsString::from_wide(&[0x61, 0xd800]);
        assert_eq!(name.encode_wide().collect::<Vec<_>>(), vec![0x61, 0xd800]);
        assert_eq!(
            logical_name_cmp(OsStr::new("file2"), OsStr::new("file10")),
            Ordering::Less
        );
        assert_ne!(
            logical_name_cmp(&name, &OsString::from_wide(&[0x61, 0xd801])),
            Ordering::Equal
        );
    }
}
