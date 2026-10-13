//! Selection identities are lossless paths; ranges follow the visible order.
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
#[derive(Clone, Debug, Default)]
pub struct Selection {
    selected: HashSet<PathBuf>,
    anchor: Option<PathBuf>,
    cursor: Option<PathBuf>,
}
impl Selection {
    pub fn contains(&self, path: &Path) -> bool {
        self.selected.contains(path)
    }
    pub fn paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.selected.iter()
    }
    pub fn len(&self) -> usize {
        self.selected.len()
    }
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
    pub fn cursor(&self) -> Option<&Path> {
        self.cursor.as_deref()
    }
    pub fn clear(&mut self) {
        self.selected.clear();
        self.anchor = None;
        self.cursor = None;
    }
    pub fn click(&mut self, path: PathBuf, ctrl: bool, shift: bool, visible: &[PathBuf]) {
        if !visible.contains(&path) {
            return;
        }
        if shift {
            let start = self
                .anchor
                .as_ref()
                .and_then(|a| visible.iter().position(|p| p == a));
            let end = visible
                .iter()
                .position(|p| p == &path)
                .expect("visible path");
            if !ctrl {
                self.selected.clear();
            }
            if let Some(start) = start {
                self.selected
                    .extend(visible[start.min(end)..=start.max(end)].iter().cloned());
            } else {
                self.selected.insert(path.clone());
                self.anchor = Some(path.clone());
            }
        } else if ctrl {
            if !self.selected.remove(&path) {
                self.selected.insert(path.clone());
            }
            self.anchor = Some(path.clone());
        } else {
            self.selected.clear();
            self.selected.insert(path.clone());
            self.anchor = Some(path.clone());
        }
        self.cursor = Some(path);
    }
    pub fn select_all(&mut self, visible: &[PathBuf]) {
        self.selected = visible.iter().cloned().collect();
        self.cursor = visible.first().cloned();
        self.anchor = self.cursor.clone();
    }
    pub fn prune(&mut self, visible: &[PathBuf]) {
        let live: HashSet<_> = visible.iter().collect();
        self.selected.retain(|path| live.contains(path));
        if self
            .anchor
            .as_ref()
            .is_some_and(|path| !live.contains(path))
        {
            self.anchor = None;
        }
        if self
            .cursor
            .as_ref()
            .is_some_and(|path| !live.contains(path))
        {
            self.cursor = None;
        }
    }
    /// Ctrl moves focus alone; Shift extends from the stable anchor.
    pub fn move_cursor(&mut self, delta: isize, ctrl: bool, shift: bool, visible: &[PathBuf]) {
        if visible.is_empty() {
            self.clear();
            return;
        }
        let index = self
            .cursor
            .as_ref()
            .and_then(|p| visible.iter().position(|v| v == p));
        let next = match index {
            Some(index) => index.saturating_add_signed(delta).min(visible.len() - 1),
            None if delta < 0 => visible.len() - 1,
            None => 0,
        };
        if ctrl && !shift {
            self.cursor = Some(visible[next].clone());
        } else {
            self.click(visible[next].clone(), ctrl, shift, visible);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn visible() -> Vec<PathBuf> {
        ["a", "b", "c", "d"]
            .into_iter()
            .map(PathBuf::from)
            .collect()
    }
    #[test]
    fn selection_toggle_range_and_prune() {
        let paths = visible();
        let mut selection = Selection::default();
        selection.click(paths[1].clone(), false, false, &paths);
        selection.click(paths[3].clone(), false, true, &paths);
        assert_eq!(selection.len(), 3);
        selection.click(paths[2].clone(), true, false, &paths);
        assert!(!selection.contains(&paths[2]));
        selection.prune(&paths[..2]);
        assert_eq!(selection.len(), 1);
        selection.select_all(&paths[..2]);
        assert_eq!(selection.len(), 2);
    }
    #[test]
    fn selection_range_uses_current_order_and_control_moves_focus() {
        let paths = visible();
        let mut selection = Selection::default();
        selection.click(paths[0].clone(), false, false, &paths);
        selection.move_cursor(2, true, false, &paths);
        assert_eq!(selection.len(), 1);
        assert_eq!(selection.cursor(), Some(paths[2].as_path()));
        let reordered = vec![
            paths[0].clone(),
            paths[3].clone(),
            paths[1].clone(),
            paths[2].clone(),
        ];
        selection.click(paths[1].clone(), false, true, &reordered);
        assert!(selection.contains(&paths[3]));
        assert!(!selection.contains(&paths[2]));
    }
}
