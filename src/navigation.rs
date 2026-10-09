use crate::{
    directory::{DirectoryState, Failure, FailureKind, Response, Status, Update},
    sorting::{Key, Sort, Sorted},
};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    New,
    Back,
    Forward,
    Refresh,
}

struct Pending {
    directory: DirectoryState,
    intent: Intent,
}

pub struct Navigation {
    pub current: DirectoryState,
    pub back: Vec<PathBuf>,
    pub forward: Vec<PathBuf>,
    pending: Option<Pending>,
    pub error: Option<(PathBuf, Failure)>,
    pub sort: Sort,
    pub sorted_key: Option<Key>,
    active_generation: u64,
}

impl Navigation {
    pub fn new(location: PathBuf) -> Self {
        Self {
            current: DirectoryState::new(location),
            back: Vec::new(),
            forward: Vec::new(),
            pending: None,
            error: None,
            sort: Sort::default(),
            sorted_key: None,
            active_generation: 0,
        }
    }

    pub fn resolve(&self, text: &str) -> PathBuf {
        // Submitting unchanged displayed text must not turn a native path into a
        // lossy display string. User edits necessarily go through Unicode text.
        if text == self.current.location.to_string_lossy() {
            return self.current.location.clone();
        }
        let path = PathBuf::from(text);
        if path.is_absolute() {
            path
        } else {
            self.current.location.join(path)
        }
    }

    pub fn up(&self) -> Option<PathBuf> {
        self.current
            .location
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map(Path::to_owned)
    }

    pub fn begin(&mut self, location: PathBuf, generation: u64, intent: Intent) {
        self.active_generation = generation;
        self.error = None;
        self.pending = None;
        if intent == Intent::Refresh || location == self.current.location {
            self.current.begin(location, generation);
            self.sorted_key = None;
        } else {
            if matches!(self.current.status, Status::Loading) {
                self.current.status = Status::Failed(Failure {
                    kind: FailureKind::Other,
                    detail: "Loading interrupted by navigation; Refresh to retry.".into(),
                });
            }
            let mut directory = DirectoryState::new(location.clone());
            directory.begin(location, generation);
            self.pending = Some(Pending { directory, intent });
        }
    }

    pub fn pending_location(&self) -> Option<&Path> {
        self.pending
            .as_ref()
            .map(|p| p.directory.location.as_path())
    }

    pub fn visible(&self) -> &DirectoryState {
        self.pending
            .as_ref()
            .map_or(&self.current, |p| &p.directory)
    }

    pub fn pending_count(&self) -> usize {
        self.pending
            .as_ref()
            .map_or(0, |p| p.directory.raw_entries.len())
    }

    // Successful load, including refresh (path-editor synchronization).
    pub fn apply(&mut self, response: Response) -> bool {
        if response.generation != self.active_generation {
            return false;
        }
        if let Some(pending) = &mut self.pending {
            if response.generation != pending.directory.generation {
                return false;
            }
            let finished = matches!(response.update, Update::Finished(_));
            pending.directory.apply(response);
            if !finished {
                return false;
            }
            let pending = self.pending.take().unwrap();
            match pending.directory.status {
                Status::Ready => {
                    match pending.intent {
                        Intent::New => {
                            self.back.push(self.current.location.clone());
                            self.forward.clear();
                        }
                        Intent::Back => {
                            self.back.pop();
                            self.forward.push(self.current.location.clone());
                        }
                        Intent::Forward => {
                            self.forward.pop();
                            self.back.push(self.current.location.clone());
                        }
                        Intent::Refresh => unreachable!(),
                    }
                    self.current = pending.directory;
                    return true;
                }
                Status::Failed(failure) => {
                    self.error = Some((pending.directory.location, failure));
                }
                Status::Loading => unreachable!(),
            }
        } else {
            if response.generation != self.current.generation {
                return false;
            }
            let success = matches!(response.update, Update::Finished(Ok(())));
            self.current.apply(response);
            return success;
        }
        false
    }

    pub fn key(&self) -> Key {
        Key {
            generation: self.visible().generation,
            revision: self.visible().revision,
            sort: self.sort,
        }
    }

    pub fn apply_sorted(&mut self, sorted: Sorted) -> bool {
        if sorted.key != self.key() {
            return false;
        }
        if let Some(pending) = &mut self.pending {
            pending.directory.entries = sorted.entries;
        } else {
            self.current.entries = sorted.entries;
        }
        self.sorted_key = Some(sorted.key);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io, sync::Arc};

    fn finish(nav: &mut Navigation, generation: u64, success: bool) -> bool {
        nav.apply(Response {
            generation,
            update: Update::Finished(if success {
                Ok(())
            } else {
                Err(Failure {
                    kind: FailureKind::Missing,
                    detail: io::ErrorKind::NotFound.to_string(),
                })
            }),
        })
    }

    fn ready() -> Navigation {
        let mut nav = Navigation::new(PathBuf::from("root").join("a"));
        nav.begin(nav.current.location.clone(), 1, Intent::Refresh);
        finish(&mut nav, 1, true);
        nav
    }

    #[test]
    fn successful_back_forward_and_new_branch_preserve_history() {
        let mut nav = ready();
        let a = nav.current.location.clone();
        let b = PathBuf::from("root/b");
        let c = PathBuf::from("root/c");
        nav.begin(b.clone(), 2, Intent::New);
        assert_eq!(nav.current.location, a);
        assert!(nav.back.is_empty());
        assert!(finish(&mut nav, 2, true));
        nav.begin(a.clone(), 3, Intent::Back);
        finish(&mut nav, 3, true);
        assert!(nav.back.is_empty());
        assert_eq!(nav.forward, vec![b.clone()]);
        nav.begin(b.clone(), 4, Intent::Forward);
        finish(&mut nav, 4, true);
        assert_eq!(nav.back, vec![a.clone()]);
        assert!(nav.forward.is_empty());
        nav.begin(a.clone(), 5, Intent::Back);
        finish(&mut nav, 5, true);
        nav.begin(c.clone(), 6, Intent::New);
        finish(&mut nav, 6, true);
        assert_eq!(nav.current.location, c);
        assert_eq!(nav.back, vec![a]);
        assert!(nav.forward.is_empty());
        nav.begin(c, 7, Intent::New); // same-location navigation is refresh
        finish(&mut nav, 7, true);
        assert_eq!(nav.back.len(), 1);
    }

    #[test]
    fn failed_new_or_history_destination_keeps_location_listing_and_history() {
        let mut nav = ready();
        let a = nav.current.location.clone();
        let b = PathBuf::from("root/b");
        nav.begin(b.clone(), 2, Intent::New);
        finish(&mut nav, 2, true);
        let row = Arc::new(crate::directory::Entry {
            path: b.join("retained"),
            name: "retained".into(),
            kind: crate::directory::EntryKind::File,
            modified: "—".into(),
            modified_time: None,
            size: Some(1),
            metadata_error: None,
        });
        nav.apply(Response {
            generation: 2,
            update: Update::Snapshot {
                entries: Arc::new(vec![row]),
                metadata_errors: 0,
            },
        });
        nav.apply_sorted(Sorted {
            key: nav.key(),
            entries: nav.current.raw_entries.clone(),
        });
        let data = nav.current.raw_entries.clone();
        let displayed = nav.current.entries.clone();
        nav.begin(a.clone(), 3, Intent::Back);
        // Even a destination which produces rows before failing must not commit.
        nav.apply(Response {
            generation: 3,
            update: Update::Snapshot {
                entries: Arc::new(Vec::new()),
                metadata_errors: 0,
            },
        });
        finish(&mut nav, 3, false);
        assert_eq!(nav.current.location, b);
        assert_eq!(nav.back, vec![a.clone()]);
        assert!(nav.forward.is_empty());
        assert!(Arc::ptr_eq(&data, &nav.current.raw_entries));
        assert!(Arc::ptr_eq(&displayed, &nav.current.entries));
        assert_eq!(nav.current.entries[0].path, b.join("retained"));
        assert!(matches!(nav.current.status, Status::Ready));
        assert_eq!(nav.error.as_ref().unwrap().0, a);
        nav.begin(PathBuf::from("missing"), 4, Intent::New);
        finish(&mut nav, 4, false);
        assert_eq!(nav.current.location, b);
        assert_eq!(nav.back.len(), 1);
        assert!(nav.forward.is_empty());
        nav.begin(a.clone(), 5, Intent::Back);
        finish(&mut nav, 5, true);
        nav.begin(b.clone(), 6, Intent::Forward);
        finish(&mut nav, 6, false);
        assert_eq!(nav.current.location, a);
        assert!(nav.back.is_empty());
        assert_eq!(nav.forward, vec![b]);
        // Queued completions from the retained location are stale after the failed request.
        finish(&mut nav, 5, false);
        assert!(matches!(nav.current.status, Status::Ready));
    }

    #[test]
    fn stale_navigation_and_sort_cannot_commit_and_relative_input_uses_committed_location() {
        let mut nav = ready();
        let a = nav.current.location.clone();
        let stale_key = nav.key();
        nav.begin(PathBuf::from("root/b"), 2, Intent::New);
        nav.begin(PathBuf::from("root/c"), 3, Intent::New);
        assert_eq!(nav.resolve("child"), a.join("child"));
        assert!(!finish(&mut nav, 2, true));
        assert_eq!(nav.current.location, a);
        assert!(nav.back.is_empty());
        finish(&mut nav, 3, true);
        let entries = nav.current.entries.clone();
        assert!(!nav.apply_sorted(Sorted {
            key: stale_key,
            entries: entries.clone()
        }));
        let key = nav.key();
        nav.sort.toggle(crate::sorting::Column::Size);
        assert!(!nav.apply_sorted(Sorted {
            key,
            entries: entries.clone()
        }));
        let key = nav.key();
        nav.apply(Response {
            generation: key.generation,
            update: Update::Snapshot {
                entries: entries.clone(),
                metadata_errors: 0,
            },
        });
        assert!(!nav.apply_sorted(Sorted { key, entries }));
        assert_eq!(nav.resolve("child"), nav.current.location.join("child"));
        assert_eq!(nav.back, vec![a]);
    }

    #[test]
    fn vanished_current_location_has_missing_state_and_usable_parent_without_history_change() {
        let mut nav = ready();
        let location = nav.current.location.clone();
        nav.begin(location.clone(), 2, Intent::Refresh);
        finish(&mut nav, 2, false);
        assert_eq!(nav.current.location, location);
        assert!(matches!(
            nav.current.status,
            Status::Failed(Failure {
                kind: FailureKind::Missing,
                ..
            })
        ));
        assert_eq!(nav.up(), Some(PathBuf::from("root")));
        assert!(nav.back.is_empty() && nav.forward.is_empty());
        let mut startup = Navigation::new(location.clone());
        startup.begin(location, 1, Intent::Refresh);
        finish(&mut startup, 1, false);
        assert!(matches!(startup.current.status, Status::Failed(_)));
        assert!(startup.up().is_some());
    }
}
