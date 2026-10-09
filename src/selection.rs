use crate::directory::Entry;
use std::{collections::HashSet, path::PathBuf, sync::Arc};

#[derive(Default, Debug)]
pub struct Selection {
    pub paths: HashSet<PathBuf>,
    pub active: Option<PathBuf>,
    pub anchor: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Movement {
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

impl Selection {
    // Resolve identities against the current displayed order only when commanded.
    // These scans are O(n); no row index survives a projection change.
    pub fn active_index(&self, entries: &[Arc<Entry>]) -> Option<usize> {
        self.active
            .as_ref()
            .and_then(|path| entries.iter().position(|entry| &entry.path == path))
    }

    pub fn click(&mut self, entries: &[Arc<Entry>], index: usize, ctrl: bool, shift: bool) {
        let Some(entry) = entries.get(index) else {
            return;
        };
        if shift {
            let start = self
                .anchor
                .as_ref()
                .and_then(|path| entries.iter().position(|entry| &entry.path == path))
                .or_else(|| self.active_index(entries))
                .unwrap_or(index);
            self.anchor = Some(entries[start].path.clone());
            if !ctrl {
                self.paths.clear();
            }
            self.paths.extend(
                entries[start.min(index)..=start.max(index)]
                    .iter()
                    .map(|entry| entry.path.clone()),
            );
        } else {
            if ctrl {
                if !self.paths.remove(&entry.path) {
                    self.paths.insert(entry.path.clone());
                }
            } else {
                self.paths.clear();
                self.paths.insert(entry.path.clone());
            }
            self.anchor = Some(entry.path.clone());
        }
        self.active = Some(entry.path.clone());
    }

    pub fn select_all(&mut self, entries: &[Arc<Entry>]) {
        self.paths = entries.iter().map(|entry| entry.path.clone()).collect();
        if let Some(index) = self
            .active_index(entries)
            .or_else(|| (!entries.is_empty()).then_some(0))
        {
            self.active = Some(entries[index].path.clone());
            self.anchor = self.active.clone();
        }
    }

    pub fn move_active(
        &mut self,
        entries: &[Arc<Entry>],
        movement: Movement,
        page_rows: usize,
        shift: bool,
    ) {
        if entries.is_empty() {
            // A progressive refresh may temporarily have no rows. It is not
            // authoritative evidence that the retained paths disappeared.
            return;
        }
        let last = entries.len() - 1;
        let active = self.active_index(entries);
        let index = match movement {
            Movement::Home => 0,
            Movement::End => last,
            Movement::Up => active.map_or(last, |i| i.saturating_sub(1)),
            Movement::Down => active.map_or(0, |i| i.saturating_add(1).min(last)),
            Movement::PageUp => active.map_or(last, |i| i.saturating_sub(page_rows.max(1))),
            Movement::PageDown => {
                active.map_or(0, |i| i.saturating_add(page_rows.max(1)).min(last))
            }
        };
        self.click(entries, index, false, shift);
    }

    pub fn reconcile(&mut self, entries: &[Arc<Entry>]) {
        // Successful enumeration can prune vanished paths before projection completes.
        let present: HashSet<_> = entries.iter().map(|entry| &entry.path).collect();
        self.retain_present(|path| present.contains(path));
    }

    pub fn reconcile_visible(&mut self, present: &HashSet<PathBuf>) {
        // Only authoritative completed projections prune hidden identities.
        // The worker builds this set; UI work scales with retained selection, not rows.
        self.retain_present(|path| present.contains(path));
    }

    fn retain_present(&mut self, contains: impl Fn(&PathBuf) -> bool) {
        self.paths.retain(&contains);
        if self.active.as_ref().is_some_and(|path| !contains(path)) {
            self.active = None;
        }
        if self.anchor.as_ref().is_some_and(|path| !contains(path)) {
            self.anchor = None;
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::directory::EntryKind;

    pub(crate) fn rows(names: &[&str]) -> Arc<Vec<Arc<Entry>>> {
        Arc::new(
            names
                .iter()
                .map(|name| {
                    Arc::new(Entry {
                        path: PathBuf::from(name),
                        name: "same display text".into(),
                        kind: EntryKind::Folder,
                        modified: "—".into(),
                        modified_time: None,
                        size: None,
                        metadata_error: None,
                    })
                })
                .collect(),
        )
    }

    fn selected(selection: &Selection, names: &[&str]) {
        assert_eq!(selection.paths, names.iter().map(PathBuf::from).collect());
    }

    #[test]
    fn click_toggle_range_and_select_all_use_native_displayed_paths() {
        let rows = rows(&["a", "b", "c", "d"]);
        let mut selection = Selection::default();
        selection.click(&rows, 1, false, false);
        selected(&selection, &["b"]);
        selection.click(&rows, 3, true, false);
        selected(&selection, &["b", "d"]);
        selection.click(&rows, 3, true, false);
        selected(&selection, &["b"]);
        assert_eq!(selection.active, Some(PathBuf::from("d")));
        selection.click(&rows, 1, false, true);
        selected(&selection, &["b", "c", "d"]);
        selection.click(&rows, 2, false, true);
        selected(&selection, &["c", "d"]);
        selection.select_all(&rows[..2]);
        selected(&selection, &["a", "b"]);
        assert_eq!(selection.active, Some(PathBuf::from("a")));
    }

    #[test]
    fn keyboard_boundaries_pages_and_shift_extension() {
        let rows = rows(&["a", "b", "c", "d", "e"]);
        let mut selection = Selection::default();
        selection.move_active(&rows, Movement::Down, 2, false);
        selected(&selection, &["a"]);
        selection.move_active(&rows, Movement::Up, 2, false);
        selected(&selection, &["a"]);
        selection.move_active(&rows, Movement::PageDown, 2, true);
        selected(&selection, &["a", "b", "c"]);
        selection.move_active(&rows, Movement::End, 2, true);
        selected(&selection, &["a", "b", "c", "d", "e"]);
        selection.move_active(&rows, Movement::Up, 2, true);
        selected(&selection, &["a", "b", "c", "d"]);
        selection.move_active(&rows, Movement::PageUp, 2, false);
        selected(&selection, &["b"]);
        selection.move_active(&rows, Movement::Home, 2, false);
        selected(&selection, &["a"]);
        selection.move_active(&rows, Movement::End, usize::MAX, false);
        selection.move_active(&rows, Movement::Down, usize::MAX, false);
        selected(&selection, &["e"]);
        selection.move_active(&rows, Movement::PageDown, usize::MAX, false);
        selected(&selection, &["e"]);
        selection.move_active(&[], Movement::Down, 2, true);
        selected(&selection, &["e"]);
        for movement in [Movement::Up, Movement::PageUp, Movement::End] {
            let mut fresh = Selection::default();
            fresh.move_active(&rows, movement, 2, true);
            selected(&fresh, &["e"]);
        }
        for movement in [Movement::Down, Movement::PageDown, Movement::Home] {
            let mut fresh = Selection::default();
            fresh.move_active(&rows, movement, 2, true);
            selected(&fresh, &["a"]);
        }
        let mut empty = Selection::default();
        empty.move_active(&[], Movement::End, 0, true);
        empty.select_all(&[]);
        assert!(empty.paths.is_empty() && empty.active.is_none());
    }

    #[cfg(windows)]
    #[test]
    fn distinct_non_unicode_native_paths_do_not_collapse_selection() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};
        let mut entries = rows(&["a", "b"]);
        for (entry, unit) in Arc::get_mut(&mut entries)
            .unwrap()
            .iter_mut()
            .zip([0xD800, 0xD801])
        {
            Arc::get_mut(entry).unwrap().path = PathBuf::from(OsString::from_wide(&[0x78, unit]));
        }
        assert_eq!(
            entries[0].path.to_string_lossy(),
            entries[1].path.to_string_lossy()
        );
        let mut selection = Selection::default();
        selection.select_all(&entries);
        assert_eq!(selection.paths.len(), 2);
        selection.click(&entries, 0, true, false);
        assert_eq!(
            selection.paths,
            [entries[1].path.clone()].into_iter().collect()
        );
        selection.reconcile(&entries[1..]);
        assert_eq!(selection.paths.len(), 1);
        assert!(selection.active.is_none() && selection.anchor.is_none());
    }

    #[test]
    fn reordered_projection_keeps_paths_and_changes_new_range_order() {
        let rows = rows(&["a", "b", "c", "d"]);
        let mut selection = Selection::default();
        selection.click(&rows, 1, false, false);
        selection.click(&rows, 3, true, false);
        let reordered = vec![
            rows[3].clone(),
            rows[1].clone(),
            rows[0].clone(),
            rows[2].clone(),
        ];
        selected(&selection, &["b", "d"]);
        assert_eq!(selection.active_index(&reordered), Some(0));
        selection.click(&reordered, 2, false, true);
        selected(&selection, &["d", "b", "a"]);
        assert_eq!(selection.anchor, Some(PathBuf::from("d")));
    }
}
