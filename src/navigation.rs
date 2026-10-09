use crate::{
    directory::{DirectoryState, Failure, FailureKind, Response, Status, Update},
    selection::Selection,
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
    pub selection: Selection,
    pub back: Vec<PathBuf>,
    pub forward: Vec<PathBuf>,
    pending: Option<Pending>,
    pub error: Option<(PathBuf, Failure)>,
    pub sort: Sort,
    pub sorted_key: Option<Key>,
    pub filter: String,
    query_revision: u64,
    pub show_hidden_system: bool,
    visibility_revision: u64,
    active_generation: u64,
}

impl Navigation {
    pub fn new(location: PathBuf) -> Self {
        Self {
            current: DirectoryState::new(location),
            selection: Selection::default(),
            back: Vec::new(),
            forward: Vec::new(),
            pending: None,
            error: None,
            sort: Sort::default(),
            sorted_key: None,
            filter: String::new(),
            query_revision: 0,
            show_hidden_system: false,
            visibility_revision: 0,
            active_generation: 0,
        }
    }

    pub fn resolve(&self, text: &str) -> Result<PathBuf, &'static str> {
        // An unchanged diagnostic retains its native identity; edited diagnostic
        // text is never interpreted as an action path.
        if text == crate::native::display(self.current.location.as_os_str()) {
            return Ok(self.current.location.clone());
        }
        if crate::native::is_diagnostic(text) {
            return Err(
                "Non-Unicode diagnostic text is not an editable path. Use breadcrumbs, Up, or enter a Unicode folder path.",
            );
        }
        let path = PathBuf::from(text);
        #[cfg(windows)]
        if let Some(std::path::Component::Prefix(prefix)) = path.components().next() {
            use std::path::Prefix;
            if !path.is_absolute() {
                return Err(
                    "Drive-relative paths (such as C:folder or C:) are ambiguous. Enter a full path such as C:\\folder, or an ordinary relative path.",
                );
            }
            if !matches!(
                prefix.kind(),
                Prefix::Disk(_)
                    | Prefix::UNC(_, _)
                    | Prefix::VerbatimDisk(_)
                    | Prefix::VerbatimUNC(_, _)
            ) {
                return Err(
                    "Enter an absolute filesystem path, not a device namespace or ambiguous prefixed path.",
                );
            }
        }
        Ok(if path.is_absolute() {
            path
        } else {
            self.current.location.join(path)
        })
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
        if intent == Intent::Refresh || location.as_os_str() == self.current.location.as_os_str() {
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
                    if pending.directory.location == self.current.location {
                        // A successfully loaded alias is a refresh, not a new
                        // history branch. Reconcile only now, never on absence
                        // in a progressive/failed alias listing.
                        self.selection.reconcile(&pending.directory.raw_entries);
                        self.current = pending.directory;
                        return true;
                    }
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
                    self.selection = Selection::default();
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
            let success = matches!(response.update, Update::Finished(Ok(_)));
            self.current.apply(response);
            if success {
                self.selection.reconcile(&self.current.raw_entries);
            }
            return success;
        }
        false
    }

    pub fn set_filter(&mut self, query: String) {
        if self.filter != query {
            self.filter = query;
            self.query_revision += 1;
        }
    }

    pub fn set_show_hidden_system(&mut self, show: bool) {
        if self.show_hidden_system != show {
            self.show_hidden_system = show;
            self.visibility_revision += 1;
        }
    }

    pub fn projection_fresh(&self) -> bool {
        self.sorted_key == Some(self.key())
    }

    // Future file commands must share this fresh visible-selection boundary.
    // Filtered incomplete/failed reads retain identities but cannot expose them as sources.
    pub fn rows_selectable(&self) -> bool {
        self.pending.is_none()
            && self.projection_fresh()
            && (self.filter.is_empty() || matches!(self.current.status, Status::Ready))
    }

    pub fn key(&self) -> Key {
        Key {
            generation: self.visible().generation,
            revision: self.visible().revision,
            sort: self.sort,
            query_revision: self.query_revision,
            show_hidden_system: self.show_hidden_system,
            visibility_revision: self.visibility_revision,
        }
    }

    pub fn apply_sorted(&mut self, sorted: Sorted) -> bool {
        if sorted.key != self.key() {
            return false;
        }
        if let Some(pending) = &mut self.pending {
            pending.directory.entries = sorted.entries;
        } else {
            if matches!(self.current.status, Status::Ready) {
                self.selection.reconcile_visible(&sorted.visible_paths);
            }
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
                Ok(nav.visible().location.clone())
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
            hidden: false,
            system: false,
        });
        nav.apply(Response {
            generation: 2,
            update: Update::Snapshot {
                entries: Arc::new(vec![row]),
                metadata_errors: 0,
            },
        });
        nav.apply_sorted(Sorted::for_test(nav.key(), nav.current.raw_entries.clone()));
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
        assert_eq!(nav.resolve("child").unwrap(), a.join("child"));
        assert!(!finish(&mut nav, 2, true));
        assert_eq!(nav.current.location, a);
        assert!(nav.back.is_empty());
        finish(&mut nav, 3, true);
        let entries = nav.current.entries.clone();
        assert!(!nav.apply_sorted(Sorted::for_test(stale_key, entries.clone())));
        let key = nav.key();
        nav.sort.toggle(crate::sorting::Column::Size);
        assert!(!nav.apply_sorted(Sorted::for_test(key, entries.clone())));
        let key = nav.key();
        nav.apply(Response {
            generation: key.generation,
            update: Update::Snapshot {
                entries: entries.clone(),
                metadata_errors: 0,
            },
        });
        assert!(!nav.apply_sorted(Sorted::for_test(key, entries)));
        assert_eq!(
            nav.resolve("child").unwrap(),
            nav.current.location.join("child")
        );
        assert_eq!(nav.back, vec![a]);
    }

    #[test]
    fn selection_reconciles_only_successful_current_refresh_and_survives_failed_navigation() {
        use crate::selection::tests::rows;
        let mut nav = ready();
        let location = nav.current.location.clone();
        let data = rows(&["a", "b", "c"]);
        let snapshot = |generation, entries| Response {
            generation,
            update: Update::Snapshot {
                entries,
                metadata_errors: 0,
            },
        };
        nav.apply(snapshot(1, data.clone()));
        nav.apply_sorted(Sorted::for_test(nav.key(), data.clone()));
        nav.selection.click(&data, 0, false, false);
        nav.selection.click(&data, 2, true, false);
        let paths = nav.selection.paths.clone();
        nav.sort.toggle(crate::sorting::Column::Name);
        let reversed = Arc::new(data.iter().rev().cloned().collect());
        assert!(nav.apply_sorted(Sorted::for_test(nav.key(), reversed)));
        assert_eq!(nav.selection.paths, paths);
        assert_eq!(nav.selection.active_index(&nav.current.entries), Some(0));

        nav.begin(location.clone(), 2, Intent::Refresh);
        assert!(nav.current.entries.is_empty());
        assert_eq!(nav.selection.paths, paths);
        nav.apply(snapshot(2, rows(&["a"])));
        assert_eq!(nav.selection.paths, paths); // progressive rows aren't authoritative
        finish(&mut nav, 2, false);
        assert_eq!(nav.selection.paths, paths); // nor is a failed refresh

        nav.begin(PathBuf::from("missing"), 3, Intent::New);
        assert!(nav.pending_location().is_some());
        nav.apply(snapshot(3, rows(&["destination-row"])));
        finish(&mut nav, 3, false);
        assert_eq!(nav.current.location, location);
        assert_eq!(nav.selection.paths, paths);
        assert_eq!(nav.selection.active, Some(PathBuf::from("c")));
        assert_eq!(nav.selection.anchor, Some(PathBuf::from("c")));
        assert!(!finish(&mut nav, 2, true)); // stale completion cannot prune selection
        assert_eq!(nav.selection.paths, paths);

        nav.begin(location.clone(), 4, Intent::Refresh);
        nav.apply(snapshot(4, rows(&["a", "b"])));
        assert_eq!(nav.selection.paths, paths);
        finish(&mut nav, 4, true);
        assert_eq!(
            nav.selection.paths,
            [PathBuf::from("a")].into_iter().collect()
        );
        assert!(nav.selection.active.is_none() && nav.selection.anchor.is_none());
        nav.begin(location, 5, Intent::Refresh);
        finish(&mut nav, 5, true); // authoritative empty folder
        assert!(nav.selection.paths.is_empty());

        nav.selection.click(&data, 1, false, false);
        nav.begin(PathBuf::from("other"), 6, Intent::New);
        assert!(!finish(&mut nav, 5, true));
        assert!(!nav.selection.paths.is_empty());
        finish(&mut nav, 6, true);
        assert!(nav.selection.paths.is_empty());
        assert!(nav.selection.active.is_none() && nav.selection.anchor.is_none());
    }

    fn filter_rows() -> Arc<Vec<Arc<crate::directory::Entry>>> {
        let mut data = crate::selection::tests::rows(&["alpha", "beta", "alpine"]);
        for row in Arc::get_mut(&mut data).unwrap() {
            let row = Arc::get_mut(row).unwrap();
            row.name = row.path.to_string_lossy().into_owned();
        }
        data
    }

    fn accept_filter(nav: &mut Navigation) -> bool {
        nav.apply_sorted(crate::sorting::project_for_test(
            nav.key(),
            nav.visible().raw_entries.clone(),
            &nav.filter,
        ))
    }

    #[test]
    fn filter_revisions_reject_old_query_data_navigation_and_completion_projections() {
        let mut nav = ready();
        nav.current.raw_entries = filter_rows();
        let old = nav.key();
        nav.set_filter("al".into());
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(old, filter_rows(), "")));
        let old_query = nav.key();
        nav.set_filter("be".into());
        nav.set_filter("al".into()); // returning to text still has a newer revision
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(
            old_query,
            filter_rows(),
            "al"
        )));
        let old_data = nav.key();
        nav.apply(Response {
            generation: old_data.generation,
            update: Update::Snapshot {
                entries: filter_rows(),
                metadata_errors: 0,
            },
        });
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(
            old_data,
            filter_rows(),
            "al"
        )));
        let old_location = nav.key();
        nav.begin(PathBuf::from("elsewhere"), 2, Intent::New);
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(
            old_location,
            filter_rows(),
            "al"
        )));
        let progressive = nav.key();
        finish(&mut nav, 2, true);
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(
            progressive,
            filter_rows(),
            "al"
        )));
        assert_eq!(nav.filter, "al");
        assert!(accept_filter(&mut nav));
        let previous = nav.key();
        nav.begin(nav.current.location.clone(), 3, Intent::Refresh);
        assert_eq!(nav.filter, "al");
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(
            previous,
            filter_rows(),
            "al"
        )));
    }

    #[test]
    fn filter_prunes_hidden_identities_only_after_complete_fresh_projection_and_clear_does_not_restore_them()
     {
        let mut nav = ready();
        let data = filter_rows();
        nav.current.raw_entries = data.clone();
        accept_filter(&mut nav);
        nav.selection.select_all(&data);
        nav.selection.click(&data, 1, true, false);
        nav.selection.click(&data, 1, true, false); // all selected, beta active/anchor
        let history = (
            nav.current.location.clone(),
            nav.back.clone(),
            nav.forward.clone(),
        );
        nav.set_filter("al".into());
        assert!(!nav.rows_selectable());
        assert_eq!(nav.selection.paths.len(), 3);
        accept_filter(&mut nav);
        assert!(nav.rows_selectable());
        assert_eq!(
            nav.selection.paths,
            [PathBuf::from("alpha"), PathBuf::from("alpine")]
                .into_iter()
                .collect()
        );
        assert!(nav.selection.active.is_none() && nav.selection.anchor.is_none());
        nav.selection.select_all(&nav.current.entries);
        assert_eq!(nav.selection.paths.len(), 2);
        nav.set_filter(String::new());
        accept_filter(&mut nav);
        assert_eq!(nav.current.entries.len(), 3);
        assert_eq!(nav.selection.paths.len(), 2); // beta does not resurrect
        assert_eq!(
            history,
            (
                nav.current.location.clone(),
                nav.back.clone(),
                nav.forward.clone()
            )
        );

        nav.set_filter("al".into());
        accept_filter(&mut nav);
        let paths = nav.selection.paths.clone();
        let active = nav.selection.active.clone();
        nav.begin(nav.current.location.clone(), 2, Intent::Refresh);
        accept_filter(&mut nav); // empty progressive snapshot is not absence
        assert!(!nav.rows_selectable());
        assert_eq!(nav.selection.paths, paths);
        assert_eq!(nav.selection.active, active);
        nav.apply(Response {
            generation: 2,
            update: Update::Snapshot {
                entries: Arc::new(vec![data[0].clone()]),
                metadata_errors: 0,
            },
        });
        accept_filter(&mut nav);
        assert!(!nav.rows_selectable());
        assert_eq!(nav.selection.paths, paths);
        finish(&mut nav, 2, false);
        accept_filter(&mut nav);
        assert!(!nav.rows_selectable());
        assert_eq!(nav.selection.paths, paths);
        nav.begin(nav.current.location.clone(), 3, Intent::Refresh);
        nav.apply(Response {
            generation: 3,
            update: Update::Snapshot {
                entries: data,
                metadata_errors: 0,
            },
        });
        finish(&mut nav, 3, true);
        assert!(!nav.rows_selectable());
        accept_filter(&mut nav);
        assert!(nav.rows_selectable());
        assert_eq!(nav.selection.paths, paths);
        assert_eq!(nav.selection.active, active);
        nav.set_filter("zz".into());
        accept_filter(&mut nav);
        assert!(nav.current.entries.is_empty());
        assert!(nav.selection.paths.is_empty());
        assert!(nav.selection.active.is_none() && nav.selection.anchor.is_none());
        nav.set_filter(String::new());
        accept_filter(&mut nav);
        assert_eq!(nav.current.entries.len(), 3);
        assert!(nav.selection.paths.is_empty());
    }

    #[test]
    fn s006_visibility_freshness_pruning_and_progressive_command_boundary() {
        let mut nav = ready();
        let mut data = filter_rows();
        Arc::get_mut(&mut Arc::get_mut(&mut data).unwrap()[1])
            .unwrap()
            .system = true;
        nav.current.raw_entries = data.clone();
        nav.set_show_hidden_system(true);
        accept_filter(&mut nav);
        nav.selection.select_all(&nav.current.entries);
        nav.selection.click(&data, 1, true, false);
        nav.selection.click(&data, 1, true, false); // system entry active/anchor, all selected
        let stale = nav.key();
        nav.set_show_hidden_system(false);
        assert!(!nav.rows_selectable());
        nav.set_show_hidden_system(true);
        assert_ne!(nav.key(), stale); // returning to same boolean still advances revision
        assert!(!nav.apply_sorted(crate::sorting::project_for_test(stale, data.clone(), "")));
        nav.set_show_hidden_system(false);
        let retained = nav.selection.paths.clone();
        nav.current.status = Status::Loading;
        accept_filter(&mut nav);
        assert!(nav.rows_selectable()); // no extra global pause for empty-query browsing
        assert_eq!(nav.selection.paths, retained); // incomplete absence cannot prune
        assert_eq!(crate::active_activation(&nav, false), None); // hidden active path is not visible
        assert_eq!(
            crate::capture_activation(&nav, &data[0].path, false),
            Some(crate::Activation::Folder(data[0].path.clone()))
        );
        nav.current.status = Status::Failed(Failure {
            kind: FailureKind::Other,
            detail: "incomplete".into(),
        });
        accept_filter(&mut nav);
        assert_eq!(nav.selection.paths, retained);
        nav.current.status = Status::Ready;
        nav.current.revision += 1;
        accept_filter(&mut nav);
        assert_eq!(nav.selection.paths.len(), 2);
        assert!(!nav.selection.paths.contains(&data[1].path));
        assert!(nav.selection.active.is_none() && nav.selection.anchor.is_none());
        nav.selection.select_all(&nav.current.entries);
        assert_eq!(nav.selection.paths.len(), 2);
        nav.set_show_hidden_system(true);
        accept_filter(&mut nav);
        assert_eq!(nav.current.entries.len(), 3);
        assert_eq!(nav.selection.paths.len(), 2); // show does not resurrect selection
        nav.set_filter("al".into());
        nav.current.status = Status::Loading;
        accept_filter(&mut nav);
        assert!(!nav.rows_selectable()); // preserve S004 query-loading pause
    }

    #[cfg(windows)]
    #[test]
    fn s006_typed_paths_reject_drive_relative_and_preserve_absolute_prefixes() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};
        let mut nav = Navigation::new(PathBuf::from(r"C:\fixture\committed"));
        nav.selection
            .paths
            .insert(nav.current.location.join("selected"));
        nav.back.push(PathBuf::from(r"C:\previous"));
        let selection = nav.selection.paths.clone();
        nav.begin(PathBuf::from(r"C:\pending"), 1, Intent::New);
        for text in ["C:relative", "C:", "D:child", r"\\?\C:relative"] {
            assert!(nav.resolve(text).is_err(), "ambiguous input {text}");
        }
        assert_eq!(
            nav.resolve("child").unwrap(),
            PathBuf::from(r"C:\fixture\committed\child")
        );
        assert_eq!(
            nav.resolve(r"..\sibling").unwrap(),
            nav.current.location.join(r"..\sibling")
        );
        assert_eq!(
            nav.resolve(r"\rooted").unwrap(),
            PathBuf::from(r"C:\rooted")
        );
        for text in [
            r"D:\full",
            r"\\server\share\folder",
            r"\\?\C:\long\folder",
            r"\\?\UNC\server\share\folder",
        ] {
            assert_eq!(
                nav.resolve(text).unwrap().as_os_str(),
                std::ffi::OsStr::new(text)
            );
            assert!(nav.resolve(text).unwrap().is_absolute());
        }
        finish(&mut nav, 1, false);
        assert_eq!(nav.current.location, PathBuf::from(r"C:\fixture\committed"));
        assert_eq!(nav.back, [PathBuf::from(r"C:\previous")]);
        assert_eq!(nav.selection.paths, selection);
        let mut units: Vec<_> = r"C:\fixture\".encode_utf16().collect();
        units.push(0xD800);
        nav.current.location = PathBuf::from(OsString::from_wide(&units));
        let diagnostic = crate::native::display(nav.current.location.as_os_str());
        assert_eq!(
            nav.resolve(&diagnostic).unwrap().as_os_str(),
            nav.current.location.as_os_str()
        );
        assert!(nav.resolve(&(diagnostic + "edited")).is_err());
        assert_eq!(
            nav.resolve(r"actual-�").unwrap(),
            nav.current.location.join("actual-�")
        );
    }

    #[test]
    fn s006b_normalized_completion_is_transactional_and_generation_guarded() {
        let mut nav = ready();
        let location = nav.current.location.clone();
        let data = crate::selection::tests::rows(&["root/a/keep"]);
        nav.current.raw_entries = data.clone();
        nav.apply_sorted(Sorted::for_test(nav.key(), data.clone()));
        nav.selection.select_all(&data);
        nav.forward.push(PathBuf::from("root/b"));
        let selection = nav.selection.paths.clone();
        let retained = nav.current.entries.clone();
        nav.begin(location.join("."), 2, Intent::New);
        assert!(nav.pending_location().is_some());
        assert_eq!(nav.current.location.as_os_str(), location.as_os_str());
        assert!(!finish(&mut nav, 2, false));
        assert!(Arc::ptr_eq(&retained, &nav.current.entries));
        assert_eq!(nav.selection.paths, selection);
        assert!(nav.back.is_empty());
        assert_eq!(nav.forward, [PathBuf::from("root/b")]);

        nav.begin(location.join("child").join(".."), 3, Intent::New);
        assert!(!nav.apply(Response {
            generation: 2,
            update: Update::Finished(Ok(PathBuf::from("wrong/stale-normalization")))
        }));
        assert_eq!(nav.current.location, location);
        assert!(nav.error.is_none());
        nav.apply(Response {
            generation: 3,
            update: Update::Snapshot {
                entries: data.clone(),
                metadata_errors: 0,
            },
        });
        assert_eq!(nav.selection.paths, selection);
        assert!(nav.apply(Response {
            generation: 3,
            update: Update::Finished(Ok(location.clone()))
        }));
        assert!(nav.back.is_empty());
        assert_eq!(nav.forward, [PathBuf::from("root/b")]);
        assert_eq!(nav.selection.paths, selection);
        assert_eq!(nav.selection.active, Some(PathBuf::from("root/a/keep")));
        assert_eq!(nav.selection.anchor, nav.selection.active);
        assert!(nav.apply_sorted(Sorted::for_test(nav.key(), data)));
        assert!(nav.rows_selectable());

        nav.begin(location.join("..").join("b"), 4, Intent::New);
        assert_eq!(nav.current.location, location);
        assert!(nav.back.is_empty());
        let b = PathBuf::from("root/b");
        nav.apply(Response {
            generation: 4,
            update: Update::Snapshot {
                entries: crate::selection::tests::rows(&["root/b/new"]),
                metadata_errors: 0,
            },
        });
        assert!(nav.apply(Response {
            generation: 4,
            update: Update::Finished(Ok(b.clone()))
        }));
        assert_eq!(nav.current.location, b);
        assert_eq!(nav.back, [location]);
        assert!(nav.forward.is_empty() && nav.selection.paths.is_empty());
        assert_eq!(
            nav.current.raw_entries[0].path.parent(),
            Some(nav.current.location.as_path())
        );
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
