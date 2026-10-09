use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicU64, Ordering},
        mpsc::{self, Receiver, SyncSender, TrySendError},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const BATCH_SIZE: usize = 256;
const QUEUED_RESPONSES: usize = 8;
const WORKERS: usize = 2;

#[derive(Debug, PartialEq, Eq)]
pub enum EntryKind {
    Folder,
    File,
    Link,
    Other,
    Unknown,
}

impl EntryKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Folder => "Folder",
            Self::File => "File",
            Self::Link => "Link / reparse point",
            Self::Other => "Other",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub kind: EntryKind,
    pub modified: String,
    pub modified_time: Option<SystemTime>,
    pub size: Option<u64>,
    pub metadata_error: Option<String>,
    pub hidden: bool,
    pub system: bool,
}

impl Entry {
    fn read(entry: fs::DirEntry) -> Self {
        let path = entry.path();
        let name = crate::native::display(&entry.file_name());
        let mut row = Self {
            path,
            name,
            kind: EntryKind::Unknown,
            modified: "—".into(),
            modified_time: None,
            size: None,
            metadata_error: None,
            hidden: false,
            system: false,
        };
        // DirEntry metadata never follows links and uses cached enumeration data on Windows.
        match entry.metadata() {
            Ok(metadata) => {
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    use windows_sys::Win32::Storage::FileSystem::{
                        FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM,
                    };
                    let attributes = metadata.file_attributes();
                    row.hidden = attributes & FILE_ATTRIBUTE_HIDDEN != 0;
                    row.system = attributes & FILE_ATTRIBUTE_SYSTEM != 0;
                }
                row.kind = if is_link(&metadata) {
                    EntryKind::Link
                } else if metadata.is_dir() {
                    EntryKind::Folder
                } else if metadata.is_file() {
                    EntryKind::File
                } else {
                    EntryKind::Other
                };
                if row.kind == EntryKind::File {
                    row.size = Some(metadata.len());
                }
                match metadata.modified() {
                    Ok(value) => match modified_timestamp(value) {
                        Ok(timestamp) => {
                            row.modified = timestamp;
                            row.modified_time = Some(value);
                        }
                        Err(error) => row.metadata_error = Some(format!("Modified time: {error}")),
                    },
                    Err(error) => row.metadata_error = Some(format!("Modified time: {error}")),
                }
            }
            Err(error) => row.metadata_error = Some(format!("Read metadata: {error}")),
        }
        row
    }
}

fn modified_timestamp(value: SystemTime) -> Result<String, time::error::ComponentRange> {
    let nanos = match value.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as i128,
        Err(error) => -(error.duration().as_nanos() as i128),
    };
    // Unusual filesystem timestamps must show an unknown value, never panic the worker.
    let date = time::OffsetDateTime::from_unix_timestamp_nanos(nanos)?;
    Ok(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        date.year(),
        u8::from(date.month()),
        date.day(),
        date.hour(),
        date.minute(),
        date.second()
    ))
}

fn is_link(metadata: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        metadata.file_attributes()
            & windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT
            != 0
    }
    #[cfg(not(windows))]
    {
        metadata.file_type().is_symlink()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FailureKind {
    Missing,
    AccessDenied,
    Unavailable,
    NotFolder,
    Other,
}

impl FailureKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Missing => "Folder missing",
            Self::AccessDenied => "Access denied",
            Self::Unavailable => "Drive or network location unavailable",
            Self::NotFolder => "Location is not a folder",
            Self::Other => "Unable to read folder",
        }
    }
}

#[derive(Debug)]
pub struct Failure {
    pub kind: FailureKind,
    pub detail: String,
}

impl Failure {
    fn from_io(error: io::Error) -> Self {
        #[cfg(windows)]
        let unavailable = matches!(
            error.raw_os_error(),
            Some(15 | 21 | 53 | 55 | 67 | 1117 | 1167 | 1231)
        );
        #[cfg(not(windows))]
        let unavailable = false;
        let kind = if unavailable {
            FailureKind::Unavailable
        } else {
            match error.kind() {
                io::ErrorKind::NotFound => FailureKind::Missing,
                io::ErrorKind::PermissionDenied => FailureKind::AccessDenied,
                io::ErrorKind::NotADirectory => FailureKind::NotFolder,
                _ => FailureKind::Other,
            }
        };
        Self {
            kind,
            detail: error.to_string(),
        }
    }

    fn opening(path: &Path, error: io::Error) -> Self {
        let mut failure = Self::from_io(error);
        // Windows reports PATH_NOT_FOUND for both a missing child and an absent drive.
        // Probe only the volume root, and only on this background error path.
        #[cfg(windows)]
        if failure.kind == FailureKind::Missing
            && let Some(std::path::Component::Prefix(prefix)) = path.components().next()
            && matches!(
                prefix.kind(),
                std::path::Prefix::Disk(_) | std::path::Prefix::VerbatimDisk(_)
            )
        {
            let mut root = PathBuf::from(prefix.as_os_str());
            root.push("\\");
            if let Err(error) = fs::metadata(root)
                && matches!(
                    Self::from_io(error).kind,
                    FailureKind::Missing | FailureKind::Unavailable
                )
            {
                failure.kind = FailureKind::Unavailable;
            }
        }
        #[cfg(not(windows))]
        let _ = path;
        failure
    }
}

#[derive(Debug)]
pub enum Update {
    Snapshot {
        entries: Arc<Vec<Arc<Entry>>>,
        metadata_errors: usize,
    },
    Finished(Result<(), Failure>),
}

#[derive(Debug)]
pub struct Response {
    pub generation: u64,
    pub update: Update,
}

#[derive(Debug)]
pub enum Status {
    Loading,
    Ready,
    Failed(Failure),
}

pub struct DirectoryState {
    pub generation: u64,
    pub location: PathBuf,
    pub entries: Arc<Vec<Arc<Entry>>>,
    pub raw_entries: Arc<Vec<Arc<Entry>>>,
    pub revision: u64,
    pub metadata_errors: usize,
    pub status: Status,
}

impl DirectoryState {
    pub fn new(location: PathBuf) -> Self {
        Self {
            generation: 0,
            location,
            entries: Arc::new(Vec::new()),
            raw_entries: Arc::new(Vec::new()),
            revision: 0,
            metadata_errors: 0,
            status: Status::Loading,
        }
    }

    pub fn begin(&mut self, location: PathBuf, generation: u64) {
        self.location = location;
        self.generation = generation;
        // Replace rather than clear: release the previous location's paths and allocation.
        self.entries = Arc::new(Vec::new());
        self.raw_entries = Arc::new(Vec::new());
        self.revision = 0;
        self.metadata_errors = 0;
        self.status = Status::Loading;
    }

    pub fn apply(&mut self, response: Response) {
        if response.generation != self.generation {
            return;
        }
        match response.update {
            Update::Snapshot {
                entries,
                metadata_errors,
            } => {
                self.raw_entries = entries;
                self.metadata_errors = metadata_errors;
                self.revision += 1;
            }
            Update::Finished(result) => {
                // Completion needs its own projection: progressive rows cannot prune selection.
                self.revision += 1;
                self.status = match result {
                    Ok(()) => Status::Ready,
                    Err(failure) => Status::Failed(failure),
                };
            }
        }
    }
}

struct Request {
    generation: u64,
    location: PathBuf,
}

#[derive(Default)]
struct Mailbox {
    latest: Option<Request>,
    shutdown: bool,
}

struct Shared {
    mailbox: Mutex<Mailbox>,
    wake: Condvar,
    generation: AtomicU64,
}

pub struct DirectoryLoader {
    shared: Arc<Shared>,
    responses: Receiver<Response>,
}

impl DirectoryLoader {
    pub fn new(repaint: Arc<dyn Fn() + Send + Sync>) -> io::Result<Self> {
        let shared = Arc::new(Shared {
            mailbox: Mutex::new(Mailbox::default()),
            wake: Condvar::new(),
            generation: AtomicU64::new(0),
        });
        let (sender, responses) = mpsc::sync_channel(QUEUED_RESPONSES);
        let loader = Self { shared, responses };
        // Two workers, one coalesced pending location, and eight snapshot/status messages
        // bound stale work/transport. Snapshots share entry allocations; enumeration
        // emits at most once per 50 ms after the first batch (plus the final snapshot).
        // A blocked OS call cannot be cancelled; both workers
        // may be occupied. Upgrade to cancellable platform I/O if measurements require it,
        // not an unbounded thread per navigation. Loaded listings themselves are not capped.
        for index in 0..WORKERS {
            let shared = loader.shared.clone();
            let sender = sender.clone();
            let repaint = repaint.clone();
            thread::Builder::new()
                .name(format!("directory-{index}"))
                .spawn(move || worker(shared, sender, repaint))?;
        }
        Ok(loader)
    }

    pub fn request(&self, location: PathBuf) -> u64 {
        let generation = self.shared.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let mut mailbox = self.shared.mailbox.lock().unwrap();
        mailbox.latest = Some(Request {
            generation,
            location,
        });
        self.shared.wake.notify_one();
        generation
    }

    #[cfg(test)]
    pub fn drain_into(&self, state: &mut DirectoryState) -> bool {
        self.drain(|response| state.apply(response))
    }

    pub fn drain(&self, mut apply: impl FnMut(Response)) -> bool {
        // A fixed message budget keeps catch-up work per frame bounded, including stale drops.
        for _ in 0..QUEUED_RESPONSES {
            match self.responses.try_recv() {
                Ok(response) => apply(response),
                Err(_) => return false,
            }
        }
        true
    }
}

impl Drop for DirectoryLoader {
    fn drop(&mut self) {
        self.shared.generation.fetch_add(1, Ordering::SeqCst);
        self.shared.mailbox.lock().unwrap().shutdown = true;
        self.shared.wake.notify_all();
        // Never join a worker stuck in an OS call on the UI thread. Workers own no mutations.
    }
}

fn worker(shared: Arc<Shared>, sender: SyncSender<Response>, repaint: Arc<dyn Fn() + Send + Sync>) {
    loop {
        let request = {
            let mut mailbox = shared.mailbox.lock().unwrap();
            while mailbox.latest.is_none() && !mailbox.shutdown {
                mailbox = shared.wake.wait(mailbox).unwrap();
            }
            if mailbox.shutdown {
                return;
            }
            mailbox.latest.take().unwrap()
        };
        let cancelled = || shared.generation.load(Ordering::SeqCst) != request.generation;
        let mut entries = Vec::new();
        let mut metadata_errors = 0;
        let mut last_snapshot = Instant::now();
        enumerate(&request.location, &cancelled, |update| {
            let update = match update {
                EnumerationUpdate::Batch(batch) => {
                    metadata_errors += batch.iter().filter(|e| e.metadata_error.is_some()).count();
                    entries.extend(batch.into_iter().map(Arc::new));
                    if entries.len() > BATCH_SIZE
                        && last_snapshot.elapsed() < Duration::from_millis(50)
                    {
                        return true;
                    }
                    last_snapshot = Instant::now();
                    Update::Snapshot {
                        entries: Arc::new(entries.clone()),
                        metadata_errors,
                    }
                }
                EnumerationUpdate::Finished(result) => {
                    // Publish the final data before terminal status, even if the last batch
                    // was coalesced. Cloning entry handles happens only on this worker.
                    if !send_response(
                        &sender,
                        &cancelled,
                        &repaint,
                        Response {
                            generation: request.generation,
                            update: Update::Snapshot {
                                entries: Arc::new(std::mem::take(&mut entries)),
                                metadata_errors,
                            },
                        },
                    ) {
                        return false;
                    }
                    Update::Finished(result)
                }
            };
            let response = Response {
                generation: request.generation,
                update,
            };
            send_response(&sender, &cancelled, &repaint, response)
        });
    }
}

fn send_response(
    sender: &SyncSender<Response>,
    cancelled: &impl Fn() -> bool,
    repaint: &Arc<dyn Fn() + Send + Sync>,
    mut response: Response,
) -> bool {
    loop {
        if cancelled() {
            return false;
        }
        match sender.try_send(response) {
            Ok(()) => {
                repaint();
                return true;
            }
            Err(TrySendError::Disconnected(_)) => return false,
            Err(TrySendError::Full(value)) => {
                response = value;
                thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

enum EnumerationUpdate {
    Batch(Vec<Entry>),
    Finished(Result<(), Failure>),
}

fn enumerate(
    path: &Path,
    cancelled: &impl Fn() -> bool,
    mut emit: impl FnMut(EnumerationUpdate) -> bool,
) {
    if cancelled() {
        return;
    }
    let mut directory = match fs::read_dir(path) {
        Ok(directory) => directory,
        Err(error) => {
            if !cancelled() {
                emit(EnumerationUpdate::Finished(Err(Failure::opening(
                    path, error,
                ))));
            }
            return;
        }
    };
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    let mut last_batch = Instant::now();
    loop {
        if cancelled() {
            return;
        }
        let Some(entry) = directory.next() else {
            break;
        };
        if cancelled() {
            return;
        }
        match entry {
            Ok(entry) => batch.push(Entry::read(entry)),
            Err(error) => {
                if !batch.is_empty() && !emit(EnumerationUpdate::Batch(batch)) {
                    return;
                }
                if !cancelled() {
                    emit(EnumerationUpdate::Finished(Err(Failure::from_io(error))));
                }
                return;
            }
        }
        if batch.len() == BATCH_SIZE || last_batch.elapsed() >= Duration::from_millis(50) {
            if cancelled() || !emit(EnumerationUpdate::Batch(batch)) {
                return;
            }
            batch = Vec::with_capacity(BATCH_SIZE);
            last_batch = Instant::now();
        }
    }
    if cancelled() {
        return;
    }
    if !batch.is_empty() && !emit(EnumerationUpdate::Batch(batch)) {
        return;
    }
    if !cancelled() {
        emit(EnumerationUpdate::Finished(Ok(())));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!(
                "mado-s001-{}-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn load(path: &Path) -> DirectoryState {
        let mut state = DirectoryState::new(path.to_owned());
        let mut entries = Vec::new();
        enumerate(path, &|| false, |update| {
            match update {
                EnumerationUpdate::Batch(batch) => entries.extend(batch.into_iter().map(Arc::new)),
                EnumerationUpdate::Finished(result) => state.apply(Response {
                    generation: 0,
                    update: Update::Finished(result),
                }),
            }
            true
        });
        state.metadata_errors = entries
            .iter()
            .filter(|entry| entry.metadata_error.is_some())
            .count();
        state.raw_entries = Arc::new(entries);
        state.entries = state.raw_entries.clone();
        state
    }

    #[test]
    fn modified_timestamp_handles_pre_epoch_and_out_of_range_values() {
        assert_eq!(
            modified_timestamp(UNIX_EPOCH).unwrap(),
            "1970-01-01 00:00:00"
        );
        assert_eq!(
            modified_timestamp(UNIX_EPOCH - Duration::from_secs(1)).unwrap(),
            "1969-12-31 23:59:59"
        );
        assert!(modified_timestamp(UNIX_EPOCH + Duration::from_secs(253_402_300_800)).is_err());
    }

    #[test]
    fn immediate_entries_have_metadata_and_unknown_folder_size() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("child")).unwrap();
        fs::write(fixture.0.join("child/nested.txt"), b"not immediate").unwrap();
        fs::write(fixture.0.join("日本語.txt"), b"abc").unwrap();
        let state = load(&fixture.0);
        assert!(matches!(state.status, Status::Ready));
        assert_eq!(state.entries.len(), 2);
        let file = state
            .entries
            .iter()
            .find(|entry| entry.kind == EntryKind::File)
            .unwrap();
        assert_eq!(file.name, "日本語.txt");
        assert_eq!(file.path, fixture.0.join("日本語.txt"));
        assert_eq!(file.size, Some(3));
        assert_ne!(file.modified, "—");
        assert_eq!(file.modified.len(), 19);
        assert!(file.metadata_error.is_none());
        let folder = state
            .entries
            .iter()
            .find(|entry| entry.kind == EntryKind::Folder)
            .unwrap();
        assert_eq!(folder.size, None);
    }

    #[cfg(windows)]
    #[test]
    fn s006_owned_hidden_system_attributes_project_without_touching_identity() {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM, SetFileAttributesW,
        };
        let fixture = Fixture::new();
        for (name, attributes) in [
            ("hidden.txt", FILE_ATTRIBUTE_HIDDEN),
            ("system.txt", FILE_ATTRIBUTE_SYSTEM),
            ("both", FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM),
        ] {
            let path = fixture.0.join(name);
            if name == "both" {
                fs::create_dir(&path).unwrap();
            } else {
                fs::write(&path, b"fixture").unwrap();
            }
            let wide: Vec<_> = path.as_os_str().encode_wide().chain(Some(0)).collect();
            // SAFETY: NUL-terminated path in this test's generated disposable directory.
            assert_ne!(
                unsafe { SetFileAttributesW(wide.as_ptr(), attributes) },
                0,
                "owned attribute fixture unsupported: {}",
                io::Error::last_os_error()
            );
        }
        fs::write(fixture.0.join("visible-\u{10437}.txt"), b"unicode").unwrap();
        let state = load(&fixture.0);
        assert!(matches!(state.status, Status::Ready));
        assert_eq!(state.metadata_errors, 0);
        for row in state.entries.iter() {
            assert_eq!(row.hidden, row.name == "hidden.txt" || row.name == "both");
            assert_eq!(row.system, row.name == "system.txt" || row.name == "both");
            assert_eq!(row.path, fixture.0.join(&row.name));
        }
        let mut nav = crate::navigation::Navigation::new(fixture.0.clone());
        let hidden = crate::sorting::project_for_test(nav.key(), state.raw_entries.clone(), "");
        assert_eq!(hidden.entries.len(), 1);
        assert_eq!(
            hidden.entries[0].path,
            fixture.0.join("visible-\u{10437}.txt")
        );
        nav.set_show_hidden_system(true);
        let shown = crate::sorting::project_for_test(nav.key(), state.raw_entries.clone(), "");
        assert_eq!(shown.entries.len(), 4);
        assert_eq!(shown.entries[0].kind, EntryKind::Folder);
        let filtered = crate::sorting::project_for_test(nav.key(), state.raw_entries, "SYS");
        assert_eq!(filtered.entries.len(), 1);
        assert_eq!(filtered.entries[0].path, fixture.0.join("system.txt"));
    }

    #[cfg(windows)]
    #[test]
    fn s006_long_verbatim_unicode_fixture_enumerates_native_paths() {
        use std::os::windows::ffi::OsStrExt;
        let fixture = Fixture::new();
        let mut path = fs::canonicalize(&fixture.0).unwrap();
        assert!(
            path.as_os_str()
                .encode_wide()
                .collect::<Vec<_>>()
                .starts_with(&[92, 92, 63, 92])
        );
        for i in 0..4 {
            path.push(format!("segment-{i}-{}", "x".repeat(60)));
            fs::create_dir(&path)
                .expect("generated verbatim long-path directory must be supported");
        }
        assert!(path.as_os_str().encode_wide().count() > 260);
        let file = path.join("日本語-é-\u{10437}.txt");
        fs::write(&file, b"native long path")
            .expect("generated long Unicode file must be supported");
        let state = load(&path);
        assert!(matches!(state.status, Status::Ready));
        assert_eq!(state.entries.len(), 1);
        assert_eq!(state.entries[0].path.as_os_str(), file.as_os_str());
        assert_eq!(state.entries[0].name, "日本語-é-\u{10437}.txt");
        assert_eq!(
            fs::read(&state.entries[0].path).unwrap(),
            b"native long path"
        );
        let mut nav = crate::navigation::Navigation::new(path.clone());
        nav.current.status = Status::Ready;
        nav.current.raw_entries = state.raw_entries;
        assert!(nav.apply_sorted(crate::sorting::project_for_test(
            nav.key(),
            nav.current.raw_entries.clone(),
            "日本\u{10437}"
        )));
        assert_eq!(
            crate::capture_activation(&nav, &file, false),
            Some(crate::Activation::File(file.clone()))
        );
        assert_eq!(
            nav.resolve(&crate::native::display(path.as_os_str()))
                .unwrap()
                .as_os_str(),
            path.as_os_str()
        );
    }

    #[cfg(windows)]
    #[test]
    fn s006_malformed_fixture_names_keep_native_actions_and_distinct_display() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};
        let fixture = Fixture::new();
        let names = [
            OsString::from_wide(&[0x78, 0xD800]),
            OsString::from_wide(&[0x78, 0xD801]),
        ];
        for name in &names {
            fs::write(fixture.0.join(name), b"native")
                .expect("generated malformed UTF-16 fixture must be supported");
        }
        let state = load(&fixture.0);
        assert!(matches!(state.status, Status::Ready));
        assert_eq!(state.entries.len(), 2);
        assert_ne!(state.entries[0].name, state.entries[1].name);
        let mut nav = crate::navigation::Navigation::new(fixture.0.clone());
        nav.current.raw_entries = state.raw_entries;
        nav.current.status = Status::Ready;
        assert!(nav.apply_sorted(crate::sorting::project_for_test(
            nav.key(),
            nav.current.raw_entries.clone(),
            ""
        )));
        nav.selection.select_all(&nav.current.entries);
        assert_eq!(nav.selection.paths.len(), 2);
        for path in &nav.selection.paths {
            assert!(names.iter().any(|name| path == &fixture.0.join(name)));
            assert_eq!(fs::read(path).unwrap(), b"native");
            assert_eq!(
                crate::capture_activation(&nav, path, false),
                Some(crate::Activation::File(path.clone()))
            );
        }
    }

    #[test]
    fn empty_missing_and_not_folder_are_distinct() {
        let fixture = Fixture::new();
        let empty = load(&fixture.0);
        assert!(matches!(empty.status, Status::Ready));
        assert!(empty.entries.is_empty());
        assert!(matches!(
            load(&fixture.0.join("missing")).status,
            Status::Failed(Failure {
                kind: FailureKind::Missing,
                ..
            })
        ));
        let file = fixture.0.join("file");
        fs::write(&file, b"").unwrap();
        assert!(matches!(
            load(&file).status,
            Status::Failed(Failure {
                kind: FailureKind::NotFolder,
                ..
            })
        ));
        assert_eq!(
            Failure::from_io(io::Error::from(io::ErrorKind::PermissionDenied)).kind,
            FailureKind::AccessDenied
        );
    }

    #[cfg(windows)]
    #[test]
    fn unavailable_windows_device_errors_are_not_missing_folders() {
        for code in [15, 21, 53, 55, 67, 1117, 1167, 1231] {
            assert_eq!(
                Failure::from_io(io::Error::from_raw_os_error(code)).kind,
                FailureKind::Unavailable
            );
        }
    }

    #[test]
    fn late_batches_and_errors_cannot_replace_new_location_even_on_refresh() {
        let fixture = Fixture::new();
        fs::write(fixture.0.join("a"), b"a").unwrap();
        let mut a = load(&fixture.0);
        let mut state = DirectoryState::new(fixture.0.clone());
        state.begin(fixture.0.join("b"), 2);
        state.apply(Response {
            generation: 1,
            update: Update::Snapshot {
                entries: std::mem::take(&mut a.entries),
                metadata_errors: 0,
            },
        });
        state.apply(Response {
            generation: 1,
            update: Update::Finished(Err(Failure::from_io(io::Error::from(
                io::ErrorKind::PermissionDenied,
            )))),
        });
        assert!(state.entries.is_empty());
        assert!(matches!(state.status, Status::Loading));
        state.apply(Response {
            generation: 2,
            update: Update::Finished(Ok(())),
        });
        assert!(matches!(state.status, Status::Ready));
        let location = state.location.clone();
        state.begin(location.clone(), 3);
        state.apply(Response {
            generation: 2,
            update: Update::Finished(Ok(())),
        });
        assert_eq!(state.location, location);
        assert!(matches!(state.status, Status::Loading));
    }

    #[test]
    fn cancellation_stops_scheduling_after_a_batch() {
        let fixture = Fixture::new();
        for index in 0..BATCH_SIZE + 1 {
            fs::write(fixture.0.join(index.to_string()), b"").unwrap();
        }
        let cancelled = std::cell::Cell::new(false);
        let mut batches = 0;
        enumerate(&fixture.0, &|| cancelled.get(), |update| {
            match update {
                EnumerationUpdate::Batch(rows) => {
                    assert!(!rows.is_empty() && rows.len() <= BATCH_SIZE);
                    batches += 1;
                    cancelled.set(true);
                }
                EnumerationUpdate::Finished(_) => panic!("cancelled work must not finish"),
            }
            true
        });
        assert_eq!(batches, 1);
    }

    #[test]
    fn background_listing_sort_and_failed_navigation_recover_together() {
        use crate::{
            navigation::{Intent, Navigation},
            sorting::{Column, Sorter},
        };
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("folder")).unwrap();
        fs::write(fixture.0.join("small"), b"").unwrap();
        fs::write(fixture.0.join("large"), b"abc").unwrap();
        let loader = DirectoryLoader::new(Arc::new(|| {})).unwrap();
        let sorter = Sorter::new(Arc::new(|| {})).unwrap();
        let mut nav = Navigation::new(fixture.0.clone());
        nav.sort.toggle(Column::Size);
        nav.sort.toggle(Column::Size);
        nav.begin(
            fixture.0.clone(),
            loader.request(fixture.0.clone()),
            Intent::Refresh,
        );
        let mut requested = None;
        let deadline = Instant::now() + Duration::from_secs(10);
        while (matches!(nav.current.status, Status::Loading) || nav.sorted_key != Some(nav.key()))
            && Instant::now() < deadline
        {
            loader.drain(|response| {
                nav.apply(response);
            });
            let key = nav.key();
            if requested != Some(key) {
                sorter.request(key, nav.visible().raw_entries.clone(), nav.filter.clone());
                requested = Some(key);
            }
            if let Some(sorted) = sorter.take() {
                nav.apply_sorted(sorted);
            }
            thread::sleep(Duration::from_millis(5));
        }
        assert!(matches!(nav.current.status, Status::Ready));
        assert_eq!(nav.sorted_key, Some(nav.key()));
        assert_eq!(
            nav.current
                .entries
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<_>>(),
            ["folder", "large", "small"]
        );
        let retained = nav.current.entries.clone();
        let missing = fixture.0.join("missing");
        nav.begin(
            missing.clone(),
            loader.request(missing.clone()),
            Intent::New,
        );
        let deadline = Instant::now() + Duration::from_secs(10);
        while nav.pending_location().is_some() && Instant::now() < deadline {
            loader.drain(|response| {
                nav.apply(response);
            });
            thread::sleep(Duration::from_millis(5));
        }
        assert!(nav.pending_location().is_none());
        assert_eq!(nav.error.as_ref().unwrap().0, missing);
        assert_eq!(nav.current.location, fixture.0);
        assert!(Arc::ptr_eq(&retained, &nav.current.entries));
        assert!(nav.back.is_empty() && nav.forward.is_empty());
    }

    #[test]
    fn background_loader_delivers_latest_request() {
        let fixture = Fixture::new();
        let a = fixture.0.join("a");
        let b = fixture.0.join("b");
        fs::create_dir(&a).unwrap();
        fs::create_dir(&b).unwrap();
        fs::write(a.join("only-a"), b"a").unwrap();
        fs::write(b.join("only-b"), b"b").unwrap();
        let loader = DirectoryLoader::new(Arc::new(|| {})).unwrap();
        let mut state = DirectoryState::new(a.clone());
        state.begin(a.clone(), loader.request(a));
        state.begin(b.clone(), loader.request(b));
        let deadline = Instant::now() + Duration::from_secs(10);
        while matches!(state.status, Status::Loading) && Instant::now() < deadline {
            loader.drain_into(&mut state);
            thread::sleep(Duration::from_millis(5));
        }
        assert!(matches!(state.status, Status::Ready));
        assert_eq!(state.raw_entries.len(), 1);
        assert_eq!(state.raw_entries[0].name, "only-b");
    }
}
