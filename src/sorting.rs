use crate::directory::{Entry, EntryKind};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    io,
    path::PathBuf,
    sync::{Arc, Condvar, Mutex},
    thread,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Column {
    #[default]
    Name,
    Type,
    Modified,
    Size,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sort {
    pub column: Column,
    pub descending: bool,
}

impl Sort {
    pub fn toggle(&mut self, column: Column) {
        self.descending = self.column == column && !self.descending;
        self.column = column;
    }

    fn compare(&self, a: &Entry, b: &Entry) -> Ordering {
        let folders = (b.kind == EntryKind::Folder).cmp(&(a.kind == EntryKind::Folder));
        if folders != Ordering::Equal {
            return folders;
        }
        let directed = |order: Ordering| {
            if self.descending {
                order.reverse()
            } else {
                order
            }
        };
        // Unknown values always follow known values within their folder/file group,
        // regardless of direction. Native names/paths break ties, never display text.
        let primary = match self.column {
            Column::Name => directed(a.path.file_name().cmp(&b.path.file_name())),
            Column::Type => directed(a.kind.label().cmp(b.kind.label())),
            Column::Modified => compare_optional(a.modified_time, b.modified_time, self.descending),
            Column::Size => compare_optional(a.size, b.size, self.descending),
        };
        primary
            .then_with(|| a.path.file_name().cmp(&b.path.file_name()))
            .then_with(|| a.path.cmp(&b.path))
    }
}

fn compare_optional<T: Ord>(a: Option<T>, b: Option<T>, descending: bool) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => {
            if descending {
                b.cmp(&a)
            } else {
                a.cmp(&b)
            }
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub generation: u64,
    pub revision: u64,
    pub sort: Sort,
    pub query_revision: u64,
    pub show_hidden_system: bool,
    pub visibility_revision: u64,
}

pub struct Sorted {
    pub key: Key,
    pub entries: Arc<Vec<Arc<Entry>>>,
    pub visible_paths: HashSet<PathBuf>,
}

impl Sorted {
    fn new(key: Key, entries: Arc<Vec<Arc<Entry>>>) -> Self {
        let visible_paths = entries.iter().map(|entry| entry.path.clone()).collect();
        Self {
            key,
            entries,
            visible_paths,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test(key: Key, entries: Arc<Vec<Arc<Entry>>>) -> Self {
        Self::new(key, entries)
    }
}

#[cfg(test)]
pub(crate) fn project_for_test(key: Key, entries: Arc<Vec<Arc<Entry>>>, query: &str) -> Sorted {
    project(Request {
        target: 0,
        key,
        entries,
        query: query.into(),
    })
}

struct Request {
    target: u64,
    key: Key,
    entries: Arc<Vec<Arc<Entry>>>,
    query: String,
}

// Basic fuzzy subsequence, not ranked search: column order remains authoritative.
// Unicode lowercase is stdlib-only, not locale-sensitive case folding or normalization.
fn matches_name(name: &str, query: &[char]) -> bool {
    if query.is_empty() {
        return true;
    }
    let mut remaining = query.iter();
    let mut wanted = remaining.next();
    for character in name.chars().flat_map(char::to_lowercase) {
        if wanted == Some(&character) {
            wanted = remaining.next();
            if wanted.is_none() {
                return true;
            }
        }
    }
    wanted.is_none()
}

fn project(request: Request) -> Sorted {
    // Normalize once here, never scan names or clone entry vectors on UI update.
    let query: Vec<_> = request.query.chars().flat_map(char::to_lowercase).collect();
    let mut entries: Vec<_> = request
        .entries
        .iter()
        .filter(|entry| {
            (request.key.show_hidden_system || !(entry.hidden || entry.system))
                && matches_name(&entry.name, &query)
        })
        .cloned()
        .collect();
    entries.sort_unstable_by(|a, b| request.key.sort.compare(a, b));
    Sorted::new(request.key, Arc::new(entries))
}

#[derive(Default)]
struct Mailbox {
    requests: VecDeque<Request>,
    results: VecDeque<(u64, Sorted)>,
    latest: HashMap<u64, Key>,
    shutdown: bool,
}

pub struct Sorter {
    shared: Arc<(Mutex<Mailbox>, Condvar)>,
}

impl Sorter {
    pub fn new(repaint: Arc<dyn Fn() + Send + Sync>) -> io::Result<Self> {
        let shared = Arc::new((Mutex::new(Mailbox::default()), Condvar::new()));
        let worker_shared = shared.clone();
        thread::Builder::new()
            .name("directory-sort".into())
            .spawn(move || {
                let (lock, wake) = &*worker_shared;
                loop {
                    let request = {
                        let mut mailbox = lock.lock().unwrap();
                        while mailbox.requests.is_empty() && !mailbox.shutdown {
                            mailbox = wake.wait(mailbox).unwrap();
                        }
                        if mailbox.shutdown {
                            return;
                        }
                        mailbox.requests.pop_front().unwrap()
                    };
                    let target = request.target;
                    let sorted = project(request);
                    let mut mailbox = lock.lock().unwrap();
                    if mailbox.shutdown {
                        return;
                    }
                    if mailbox.latest.get(&target) == Some(&sorted.key) {
                        mailbox.results.retain(|(id, _)| *id != target);
                        mailbox.results.push_back((target, sorted));
                        repaint();
                    }
                }
            })?;
        Ok(Self { shared })
    }

    #[cfg(test)]
    pub fn request(&self, key: Key, entries: Arc<Vec<Arc<Entry>>>, query: String) {
        self.request_for(0, key, entries, query);
    }

    pub fn request_for(&self, target: u64, key: Key, entries: Arc<Vec<Arc<Entry>>>, query: String) {
        let (lock, wake) = &*self.shared;
        let mut mailbox = lock.lock().unwrap();
        mailbox.latest.insert(target, key);
        let request = Request {
            target,
            key,
            entries,
            query,
        };
        if let Some(pending) = mailbox.requests.iter_mut().find(|r| r.target == target) {
            *pending = request;
        } else {
            mailbox.requests.push_back(request);
        }
        mailbox.results.retain(|(id, _)| *id != target);
        wake.notify_one();
    }

    pub fn cancel(&self, target: u64) {
        let mut mailbox = self.shared.0.lock().unwrap();
        mailbox.latest.remove(&target);
        mailbox.requests.retain(|r| r.target != target);
        mailbox.results.retain(|(id, _)| *id != target);
    }

    pub fn take(&self) -> Option<Sorted> {
        self.shared
            .0
            .lock()
            .unwrap()
            .results
            .pop_front()
            .map(|(_, sorted)| sorted)
    }
}

impl Drop for Sorter {
    fn drop(&mut self) {
        let (lock, wake) = &*self.shared;
        lock.lock().unwrap().shutdown = true;
        wake.notify_one();
        // Sorting owns no filesystem work; never join a large sort on the UI thread.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    fn entry(
        name: &str,
        kind: EntryKind,
        size: Option<u64>,
        time: Option<SystemTime>,
    ) -> Arc<Entry> {
        Arc::new(Entry {
            path: PathBuf::from(name),
            name: name.into(),
            kind,
            size,
            modified_time: time,
            modified: "—".into(),
            metadata_error: None,
            hidden: false,
            system: false,
        })
    }

    #[test]
    fn background_sorter_delivers_latest_data_and_sort_without_filesystem_reads() {
        let sorter = Sorter::new(Arc::new(|| {})).unwrap();
        let first = Key {
            generation: 1,
            revision: 1,
            query_revision: 0,
            show_hidden_system: false,
            visibility_revision: 0,
            sort: Sort::default(),
        };
        sorter.request(
            first,
            Arc::new(vec![entry("old", EntryKind::File, Some(1), None)]),
            String::new(),
        );
        let latest = Key {
            generation: 2,
            revision: 3,
            query_revision: 0,
            show_hidden_system: false,
            visibility_revision: 0,
            sort: Sort {
                column: Column::Size,
                descending: true,
            },
        };
        sorter.request(
            latest,
            Arc::new(vec![
                entry("small", EntryKind::File, Some(0), None),
                entry("folder", EntryKind::Folder, None, None),
                entry("large", EntryKind::File, Some(2), None),
            ]),
            String::new(),
        );
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let result = loop {
            if let Some(result) = sorter.take() {
                break result;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "sort worker did not finish"
            );
            thread::sleep(Duration::from_millis(5));
        };
        assert_eq!(result.key, latest);
        assert_eq!(
            result
                .entries
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<_>>(),
            ["folder", "large", "small"]
        );
    }

    #[test]
    fn fuzzy_filter_is_case_insensitive_name_only_and_keeps_column_order() {
        let key = Key {
            generation: 1,
            revision: 1,
            query_revision: 1,
            show_hidden_system: false,
            visibility_revision: 0,
            sort: Sort {
                column: Column::Size,
                descending: true,
            },
        };
        let data = Arc::new(vec![
            entry("ReadMe.TXT", EntryKind::File, Some(3), None),
            entry("remote", EntryKind::Folder, None, None),
            entry("room", EntryKind::File, Some(9), None),
            entry("Ångström", EntryKind::File, Some(1), None),
        ]);
        let names = |query: &str| {
            project_for_test(key, data.clone(), query)
                .entries
                .iter()
                .map(|e| e.name.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(names("RM"), ["remote", "room", "ReadMe.TXT", "Ångström"]);
        assert_eq!(names("rdtxt"), ["ReadMe.TXT"]);
        assert_eq!(names("åÖ"), ["Ångström"]);
        assert_eq!(names(""), ["remote", "room", "ReadMe.TXT", "Ångström"]);
        assert!(names("zz").is_empty());
        assert!(names("txtread").is_empty()); // character order matters
        let mut row = entry("other", EntryKind::File, None, None);
        Arc::get_mut(&mut row).unwrap().path = PathBuf::from("ReadMe.TXT/other");
        assert!(
            project_for_test(key, Arc::new(vec![row]), "rdtxt")
                .entries
                .is_empty()
        );
    }

    #[test]
    fn background_projection_coalesces_fast_query_and_data_edits() {
        let sorter = Sorter::new(Arc::new(|| {})).unwrap();
        let mut key = Key {
            generation: 1,
            revision: 1,
            query_revision: 0,
            show_hidden_system: false,
            visibility_revision: 0,
            sort: Sort::default(),
        };
        for (revision, query) in [(1, "a"), (2, "z"), (3, "AL")] {
            key.query_revision = revision;
            key.revision = revision;
            sorter.request(
                key,
                Arc::new(vec![
                    entry("alpha", EntryKind::File, None, None),
                    entry("zeta", EntryKind::File, None, None),
                ]),
                query.into(),
            );
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let result = loop {
            if let Some(result) = sorter.take() {
                break result;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "projection worker did not finish"
            );
            thread::sleep(Duration::from_millis(5));
        };
        assert_eq!(result.key, key);
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].name, "alpha");
        assert_eq!(
            result.visible_paths,
            [PathBuf::from("alpha")].into_iter().collect()
        );
    }

    #[test]
    fn folders_first_in_every_column_and_direction_with_deterministic_ties() {
        let source = vec![
            entry("z-file", EntryKind::File, Some(1), Some(UNIX_EPOCH)),
            entry("b-folder", EntryKind::Folder, None, Some(UNIX_EPOCH)),
            entry("a-file", EntryKind::File, Some(1), Some(UNIX_EPOCH)),
            entry("a-folder", EntryKind::Folder, None, Some(UNIX_EPOCH)),
        ];
        for column in [Column::Name, Column::Type, Column::Modified, Column::Size] {
            for descending in [false, true] {
                let sort = Sort { column, descending };
                let mut rows = source.clone();
                rows.sort_unstable_by(|a, b| sort.compare(a, b));
                assert!(rows[..2].iter().all(|e| e.kind == EntryKind::Folder));
                assert!(rows[2..].iter().all(|e| e.kind == EntryKind::File));
                let expected = if column == Column::Name && descending {
                    ["b-folder", "a-folder", "z-file", "a-file"]
                } else {
                    ["a-folder", "b-folder", "a-file", "z-file"]
                };
                assert_eq!(
                    rows.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(),
                    expected
                );
                rows.reverse();
                rows.sort_unstable_by(|a, b| sort.compare(a, b));
                assert_eq!(
                    rows.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(),
                    expected
                );
            }
        }
    }

    #[cfg(windows)]
    #[test]
    fn native_names_break_ties_even_when_display_strings_match() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};
        let mut a = entry("placeholder", EntryKind::File, Some(1), None);
        let mut b = entry("placeholder", EntryKind::File, Some(1), None);
        Arc::get_mut(&mut a).unwrap().path = PathBuf::from(OsString::from_wide(&[0x78, 0xD800]));
        Arc::get_mut(&mut b).unwrap().path = PathBuf::from(OsString::from_wide(&[0x78, 0xD801]));
        assert_eq!(a.path.to_string_lossy(), b.path.to_string_lossy());
        let sort = Sort {
            column: Column::Size,
            descending: false,
        };
        assert_ne!(sort.compare(&a, &b), Ordering::Equal);
        assert_eq!(sort.compare(&a, &b), sort.compare(&b, &a).reverse());
        let nav = crate::navigation::Navigation::new(a.path.clone());
        assert_eq!(
            nav.resolve(&crate::native::display(a.path.as_os_str()))
                .unwrap(),
            a.path
        );
    }

    #[test]
    fn unknown_sizes_and_times_stay_last_and_known_values_toggle() {
        let source = vec![
            entry("unknown", EntryKind::File, None, None),
            entry("small", EntryKind::File, Some(0), Some(UNIX_EPOCH)),
            entry(
                "large",
                EntryKind::File,
                Some(99),
                Some(UNIX_EPOCH + Duration::from_secs(1)),
            ),
        ];
        for column in [Column::Size, Column::Modified] {
            for descending in [false, true] {
                let sort = Sort { column, descending };
                let mut rows = source.clone();
                rows.sort_unstable_by(|a, b| sort.compare(a, b));
                assert_eq!(rows[2].name, "unknown");
                assert_eq!(rows[0].name, if descending { "large" } else { "small" });
            }
        }
        let mut sort = Sort::default();
        sort.toggle(Column::Name);
        assert!(sort.descending);
        sort.toggle(Column::Size);
        assert!(!sort.descending);
        sort.toggle(Column::Size);
        assert!(sort.descending);
    }
}

#[cfg(test)]
mod target_tests {
    use super::*;

    #[test]
    fn s008_projection_pending_and_results_are_bounded_per_target_not_global() {
        let sorter = Sorter {
            shared: Arc::new((Mutex::new(Mailbox::default()), Condvar::new())),
        };
        let key = Key {
            generation: 1,
            revision: 0,
            sort: Sort::default(),
            query_revision: 0,
            show_hidden_system: false,
            visibility_revision: 0,
        };
        let rows = Arc::new(Vec::new());
        sorter.request_for(10, key, rows.clone(), "first".into());
        sorter.request_for(
            20,
            Key {
                generation: 2,
                ..key
            },
            rows.clone(),
            "neighbor".into(),
        );
        for revision in 1..100 {
            sorter.request_for(
                10,
                Key {
                    query_revision: revision,
                    ..key
                },
                rows.clone(),
                "latest".into(),
            );
        }
        {
            let mut mailbox = sorter.shared.0.lock().unwrap();
            assert_eq!(mailbox.requests.len(), 2);
            assert_eq!(mailbox.latest.len(), 2);
            assert_eq!(
                mailbox
                    .requests
                    .iter()
                    .map(|r| r.target)
                    .collect::<Vec<_>>(),
                [10, 20]
            );
            assert_eq!(mailbox.requests[0].query, "latest");
            assert_eq!(mailbox.latest[&20].generation, 2);
            // Each target may retain one result while a neighbor queues newer work.
            mailbox.results.push_back((
                20,
                Sorted::new(
                    Key {
                        generation: 2,
                        ..key
                    },
                    rows.clone(),
                ),
            ));
        }
        sorter.request_for(
            10,
            Key {
                query_revision: 100,
                ..key
            },
            rows,
            String::new(),
        );
        assert_eq!(sorter.take().unwrap().key.generation, 2);
        sorter.cancel(10);
        {
            let mailbox = sorter.shared.0.lock().unwrap();
            assert_eq!(mailbox.requests.len(), 1);
            assert!(!mailbox.latest.contains_key(&10));
            assert!(mailbox.latest.contains_key(&20));
        }
        sorter.cancel(20);
        let mailbox = sorter.shared.0.lock().unwrap();
        assert!(
            mailbox.requests.is_empty() && mailbox.results.is_empty() && mailbox.latest.is_empty()
        );
    }
}
