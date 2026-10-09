use crate::directory::{Entry, EntryKind};
use std::{
    cmp::Ordering,
    io,
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
}

pub struct Sorted {
    pub key: Key,
    pub entries: Arc<Vec<Arc<Entry>>>,
}

#[derive(Default)]
struct Mailbox {
    request: Option<Sorted>,
    result: Option<Sorted>,
    latest: Option<Key>,
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
                        while mailbox.request.is_none() && !mailbox.shutdown {
                            mailbox = wake.wait(mailbox).unwrap();
                        }
                        if mailbox.shutdown {
                            return;
                        }
                        mailbox.request.take().unwrap()
                    };
                    let mut entries = request.entries.as_ref().clone();
                    entries.sort_unstable_by(|a, b| request.key.sort.compare(a, b));
                    let mut mailbox = lock.lock().unwrap();
                    if mailbox.shutdown {
                        return;
                    }
                    if mailbox.latest == Some(request.key) {
                        mailbox.result = Some(Sorted {
                            key: request.key,
                            entries: Arc::new(entries),
                        });
                        repaint();
                    }
                }
            })?;
        Ok(Self { shared })
    }

    pub fn request(&self, key: Key, entries: Arc<Vec<Arc<Entry>>>) {
        let (lock, wake) = &*self.shared;
        let mut mailbox = lock.lock().unwrap();
        mailbox.latest = Some(key);
        mailbox.request = Some(Sorted { key, entries });
        mailbox.result = None;
        wake.notify_one();
    }

    pub fn take(&self) -> Option<Sorted> {
        self.shared.0.lock().unwrap().result.take()
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
        })
    }

    #[test]
    fn background_sorter_delivers_latest_data_and_sort_without_filesystem_reads() {
        let sorter = Sorter::new(Arc::new(|| {})).unwrap();
        let first = Key {
            generation: 1,
            revision: 1,
            sort: Sort::default(),
        };
        sorter.request(
            first,
            Arc::new(vec![entry("old", EntryKind::File, Some(1), None)]),
        );
        let latest = Key {
            generation: 2,
            revision: 3,
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
        assert_eq!(nav.resolve(&a.path.to_string_lossy()), a.path);
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
