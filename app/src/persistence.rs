//! UI/DTO mapping; live workers and selections are never serialized.
use crate::session::{self, Layout, Session, TabState};
use crate::ui::{BrowserUi, Tab};
use egui_dock::{DockState, LeafNode, Node, NodeIndex, Split, TabIndex, Tree};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn snapshot(ui: &BrowserUi) -> Session {
    let tabs: Vec<_> = ui
        .dock
        .iter_all_tabs()
        .map(|(_, tab)| {
            let options = tab.browser.options();
            TabState {
                id: tab.id,
                kind: match tab.kind {
                    crate::ui::LocationKind::Filesystem => session::SavedLocation::Filesystem,
                    crate::ui::LocationKind::RecycleBin => session::SavedLocation::RecycleBin,
                },
                path: if tab.kind == crate::ui::LocationKind::RecycleBin {
                    Vec::new()
                } else {
                    session::encode_path(tab.browser.current_path())
                },
                filter: options.filter.clone(),
                show_hidden: options.show_hidden,
                show_system: options.show_system,
                sort: match options.sort {
                    crate::directory::SortColumn::Name => session::SavedSort::Name,
                    crate::directory::SortColumn::Size => session::SavedSort::Size,
                    crate::directory::SortColumn::Modified => session::SavedSort::Modified,
                    crate::directory::SortColumn::Type => session::SavedSort::Type,
                },
                descending: options.descending,
                directories_first: options.directories_first,
                expanded: options.expanded,
                view: match tab.view {
                    crate::ui::ListingView::Details => session::SavedView::Details,
                    crate::ui::ListingView::Icons => session::SavedView::Icons,
                },
                icon_size: tab.icon_size,
                inspector_visible: tab.inspector_visible,
                preview_zoom: tab.preview_zoom,
                word_wrap: tab.word_wrap,
                line_numbers: tab.line_numbers,
            }
        })
        .collect();
    let active = tabs.iter().position(|tab| tab.id == ui.active).unwrap_or(0);
    Session {
        version: session::VERSION,
        settings: ui.settings.persistent_clone(),
        presets: Vec::new(),
        geometry: None,
        tabs,
        active,
        layout: capture(ui.dock.main_surface(), NodeIndex::root()),
    }
}

fn capture(tree: &Tree<Tab>, index: NodeIndex) -> Option<Layout> {
    match &tree[index] {
        Node::Leaf(leaf) => Some(Layout::Leaf {
            tabs: leaf.tabs.iter().map(|tab| tab.id).collect(),
            active: leaf.tabs.get(leaf.active.0)?.id,
        }),
        Node::Horizontal(split) | Node::Vertical(split) => Some(Layout::Split {
            // Our DTO describes the dividing line, egui_dock the arrangement.
            vertical: matches!(&tree[index], Node::Horizontal(_)),
            ratio: split.fraction,
            first: Box::new(capture(tree, index.left())?),
            second: Box::new(capture(tree, index.right())?),
        }),
        Node::Empty => None,
    }
}

pub fn restore(session: Session, home: PathBuf) -> Result<BrowserUi, String> {
    session.validate()?;
    let active = session.tabs[session.active].id;
    let layout = session.layout.clone().unwrap_or_else(|| Layout::Leaf {
        tabs: session.tabs.iter().map(|tab| tab.id).collect(),
        active,
    });
    // egui_dock uses heap-array indices, not a compact pointer tree. Bound its
    // allocation at the same supported depth as the codec's validation.
    check_dock_depth(&layout, 0)?;
    let mut ids = DockState::new(vec![session.tabs[0].id]);
    apply_layout(ids.main_surface_mut(), NodeIndex::root(), &layout, active);
    let mut tabs = HashMap::new();
    for saved in &session.tabs {
        let mut tab = match saved.kind {
            session::SavedLocation::Filesystem => {
                Tab::new(saved.id, session::decode_path(&saved.path)?)
            }
            session::SavedLocation::RecycleBin => Tab::recycle_bin(saved.id, home.clone()),
        };
        let mut options = tab.browser.options().clone();
        options.filter = saved.filter.clone();
        options.show_hidden = saved.show_hidden;
        options.show_system = saved.show_system;
        options.sort = match saved.sort {
            session::SavedSort::Name => crate::directory::SortColumn::Name,
            session::SavedSort::Size => crate::directory::SortColumn::Size,
            session::SavedSort::Modified => crate::directory::SortColumn::Modified,
            session::SavedSort::Type => crate::directory::SortColumn::Type,
        };
        options.descending = saved.descending;
        options.directories_first = saved.directories_first;
        options.expanded = saved.expanded;
        tab.view = match saved.view {
            session::SavedView::Details => crate::ui::ListingView::Details,
            session::SavedView::Icons => crate::ui::ListingView::Icons,
        };
        tab.icon_size = saved.icon_size;
        tab.inspector_visible = saved.inspector_visible;
        tab.preview_zoom = saved.preview_zoom;
        tab.word_wrap = saved.word_wrap;
        tab.line_numbers = saved.line_numbers;
        if saved.kind == session::SavedLocation::Filesystem {
            tab.browser.set_options(options);
        }
        tabs.insert(saved.id, tab);
    }
    let dock = ids.map_tabs(|id| {
        tabs.remove(id)
            .expect("validated layout references each tab once")
    });
    let mut ui = BrowserUi::new(
        if session.tabs[session.active].kind == session::SavedLocation::Filesystem {
            session::decode_path(&session.tabs[session.active].path)?
        } else {
            home.clone()
        },
        home,
    );
    ui.dock = dock;
    ui.active = active;
    ui.settings = session.settings;
    Ok(ui)
}

fn check_dock_depth(layout: &Layout, depth: usize) -> Result<(), String> {
    if depth > 12 {
        return Err("Layout exceeds safe dock depth 12; original state preserved".into());
    }
    if let Layout::Split { first, second, .. } = layout {
        check_dock_depth(first, depth + 1)?;
        check_dock_depth(second, depth + 1)?;
    }
    Ok(())
}

fn apply_layout(tree: &mut Tree<u64>, node: NodeIndex, layout: &Layout, focused: u64) {
    match layout {
        Layout::Leaf { tabs, active } => {
            let mut leaf = LeafNode::new(tabs.clone());
            leaf.active = TabIndex(
                tabs.iter()
                    .position(|id| id == active)
                    .expect("validated active tab"),
            );
            tree[node] = Node::Leaf(leaf);
            if tabs.contains(&focused) {
                tree.set_focused_node(node);
            }
        }
        Layout::Split {
            vertical,
            ratio,
            first,
            second,
        } => {
            let split = if *vertical {
                Split::Right
            } else {
                Split::Below
            };
            let [left, right] = tree.split(node, split, *ratio, Node::leaf(0));
            apply_layout(tree, left, first, focused);
            apply_layout(tree, right, second, focused);
            // Splitting subsequent branches updates toolkit focus. Reapply it
            // after construction in the final caller-independent traversal.
            let focused_node = tree.iter().enumerate().find_map(|(index, node)| {
                node.get_leaf()
                    .filter(|leaf| leaf.tabs.contains(&focused))
                    .map(|_| NodeIndex(index))
            });
            if let Some(index) = focused_node {
                tree.set_focused_node(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_lifecycle_split_mapping_roundtrip() {
        let home = PathBuf::from("C:\\wayfarer-nonexistent-fixture");
        let tab = |id| TabState {
            id,
            kind: session::SavedLocation::Filesystem,
            path: session::encode_path(&home),
            filter: format!("filter{id}"),
            show_hidden: id == 2,
            show_system: false,
            sort: session::SavedSort::Size,
            descending: true,
            directories_first: false,
            expanded: true,
            view: session::SavedView::Icons,
            icon_size: 128.0,
            inspector_visible: false,
            preview_zoom: 2.0,
            word_wrap: false,
            line_numbers: true,
        };
        let original = Session {
            version: session::VERSION,
            settings: crate::preferences::Settings::default(),
            presets: Vec::new(),
            geometry: None,
            tabs: vec![tab(1), tab(2), tab(3)],
            active: 2,
            layout: Some(Layout::Split {
                vertical: true,
                ratio: 0.3,
                first: Box::new(Layout::Leaf {
                    tabs: vec![1],
                    active: 1,
                }),
                second: Box::new(Layout::Leaf {
                    tabs: vec![2, 3],
                    active: 3,
                }),
            }),
        };
        let restored = restore(original.clone(), home).unwrap();
        assert_eq!(snapshot(&restored), original);
    }

    #[test]
    fn session_lifecycle_legacy_and_invalid_layout() {
        let home = PathBuf::from("C:\\wayfarer-nonexistent-fixture");
        let mut original = Session {
            version: session::VERSION,
            settings: crate::preferences::Settings::default(),
            presets: Vec::new(),
            geometry: None,
            tabs: vec![TabState {
                id: 9,
                kind: session::SavedLocation::Filesystem,
                path: session::encode_path(&home),
                filter: String::new(),
                show_hidden: false,
                show_system: false,
                sort: session::SavedSort::Name,
                descending: false,
                directories_first: true,
                expanded: false,
                view: session::SavedView::Details,
                icon_size: 96.0,
                inspector_visible: true,
                preview_zoom: 1.0,
                word_wrap: true,
                line_numbers: false,
            }],
            active: 0,
            layout: None,
        };
        let restored = restore(original.clone(), home.clone()).unwrap();
        assert_eq!(restored.active, 9);
        assert!(matches!(
            snapshot(&restored).layout,
            Some(Layout::Leaf { active: 9, .. })
        ));
        original.active = 2;
        assert!(restore(original, home).is_err());
    }
}
