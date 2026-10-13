#[path = "thumbnails.rs"]
mod thumbnails;
use crate::{
    directory::{Browser, Entry, ListOptions, SortColumn},
    platform::shell,
    preferences::Settings,
    selection::Selection,
};
use eframe::egui;
use egui_dock::tab_viewer::OnCloseResponse;
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use egui_extras::{Column, TableBuilder};
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListingView {
    #[default]
    Details,
    Icons,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LocationKind {
    #[default]
    Filesystem,
    RecycleBin,
}

pub struct Tab {
    pub id: u64,
    pub kind: LocationKind,
    pub bin_view: Option<crate::bin_view::BinView>,
    pub browser: Browser,
    pub selection: Selection,
    pub visible: Vec<PathBuf>,
    pub path_edit: String,
    pub path_dirty: bool,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
    pub rows_rendered: usize,
    pub view: ListingView,
    pub icon_size: f32,
    thumbnails: Option<thumbnails::Thumbnails>,
    pending_selection: Option<Selection>,
    pub inspector_visible: bool,
    pub preview_zoom: f32,
    pub word_wrap: bool,
    pub line_numbers: bool,
}
impl Tab {
    pub fn new(id: u64, path: PathBuf) -> Self {
        let mut browser = Browser::new();
        browser.navigate(path.clone());
        Self {
            id,
            kind: LocationKind::Filesystem,
            bin_view: None,
            browser,
            selection: Selection::default(),
            visible: Vec::new(),
            path_edit: path.to_string_lossy().into_owned(),
            path_dirty: false,
            history: vec![path],
            history_index: 0,
            rows_rendered: 0,
            view: ListingView::Details,
            icon_size: 96.0,
            thumbnails: None,
            pending_selection: None,
            inspector_visible: true,
            preview_zoom: 1.0,
            word_wrap: true,
            line_numbers: false,
        }
    }
    pub fn recycle_bin(id: u64, _home: PathBuf) -> Self {
        Self {
            id,
            kind: LocationKind::RecycleBin,
            bin_view: Some(crate::bin_view::BinView::new()),
            browser: Browser::new(),
            selection: Selection::default(),
            visible: Vec::new(),
            path_edit: "Recycle Bin".into(),
            path_dirty: false,
            history: vec![PathBuf::new()],
            history_index: 0,
            rows_rendered: 0,
            view: ListingView::Details,
            icon_size: 96.0,
            thumbnails: None,
            pending_selection: None,
            inspector_visible: true,
            preview_zoom: 1.0,
            word_wrap: true,
            line_numbers: false,
        }
    }
    pub fn navigate(&mut self, path: PathBuf) {
        let was_virtual = self.kind != LocationKind::Filesystem;
        self.kind = LocationKind::Filesystem;
        self.bin_view = None;
        if was_virtual {
            self.history = vec![path.clone()];
            self.history_index = 0;
        }
        if !was_virtual && path != self.browser.current_path() {
            self.history.truncate(self.history_index + 1);
            self.history.push(path.clone());
            self.history_index = self.history.len() - 1;
        }
        self.path_edit = path.to_string_lossy().into_owned();
        self.path_dirty = false;
        self.selection.clear();
        self.pending_selection = None;
        self.visible.clear();
        self.browser.navigate(path);
    }
    fn history(&mut self, forward: bool) {
        let index = if forward {
            (self.history_index + 1).min(self.history.len() - 1)
        } else {
            self.history_index.saturating_sub(1)
        };
        self.history_index = index;
        let path = self.history[index].clone();
        self.path_edit = path.to_string_lossy().into_owned();
        self.path_dirty = false;
        self.selection.clear();
        self.pending_selection = None;
        self.visible.clear();
        self.browser.navigate(path);
    }
    pub fn refresh(&mut self) {
        if self.kind == LocationKind::RecycleBin {
            if let Some(bin) = &mut self.bin_view {
                bin.refresh();
            }
            return;
        }
        self.pending_selection = Some(
            self.pending_selection
                .as_ref()
                .unwrap_or(&self.selection)
                .clone(),
        );
        self.selection.clear();
        self.visible.clear();
        self.browser
            .navigate(self.browser.current_path().to_owned());
    }
    fn poll(&mut self) {
        if self.kind == LocationKind::RecycleBin {
            if let Some(bin) = &mut self.bin_view {
                bin.poll();
            }
            return;
        }
        if self.browser.poll() {
            if self.browser.is_loading() && self.pending_selection.is_none() {
                self.pending_selection = Some(self.selection.clone());
            }
            self.visible = self
                .browser
                .snapshot()
                .map(|s| s.entries.iter().map(|e| e.path.clone()).collect())
                .unwrap_or_default();
            if self.browser.snapshot().is_some()
                && let Some(selection) = self.pending_selection.take()
            {
                self.selection = selection;
            }
            self.selection.prune(&self.visible);
        }
    }
}

#[derive(Clone)]
struct ClosedTab {
    kind: LocationKind,
    path: PathBuf,
    options: ListOptions,
    history: Vec<PathBuf>,
    history_index: usize,
    selection: Selection,
    view: ListingView,
    icon_size: f32,
    inspector_visible: bool,
    preview_zoom: f32,
    word_wrap: bool,
    line_numbers: bool,
}
impl ClosedTab {
    fn capture(tab: &Tab) -> Self {
        Self {
            kind: tab.kind,
            path: tab.browser.current_path().to_owned(),
            options: tab.browser.options().clone(),
            history: tab.history.clone(),
            history_index: tab.history_index,
            selection: tab
                .pending_selection
                .as_ref()
                .unwrap_or(&tab.selection)
                .clone(),
            view: tab.view,
            icon_size: tab.icon_size,
            inspector_visible: tab.inspector_visible,
            preview_zoom: tab.preview_zoom,
            word_wrap: tab.word_wrap,
            line_numbers: tab.line_numbers,
        }
    }
    fn restore(self, tab: &mut Tab) {
        if self.kind == LocationKind::Filesystem {
            tab.browser.set_options(self.options);
        }
        tab.history = self.history;
        tab.history_index = self.history_index;
        tab.pending_selection = Some(self.selection);
        tab.view = self.view;
        tab.icon_size = self.icon_size;
        tab.inspector_visible = self.inspector_visible;
        tab.preview_zoom = self.preview_zoom;
        tab.word_wrap = self.word_wrap;
        tab.line_numbers = self.line_numbers;
    }
}

pub enum Intent {
    Open(PathBuf),
    Shell(Vec<PathBuf>),
    Drag(Vec<PathBuf>),
}
pub struct BrowserUi {
    pub dock: DockState<Tab>,
    pub active: u64,
    next_id: u64,
    home: PathBuf,
    closed: Vec<ClosedTab>,
    pub status: String,
    pub settings: Settings,
    pub pane_targets: Vec<(egui::Rect, PathBuf)>,
}
impl BrowserUi {
    pub fn new(start: PathBuf, home: PathBuf) -> Self {
        Self {
            dock: DockState::new(vec![Tab::new(1, start)]),
            active: 1,
            next_id: 2,
            home,
            closed: Vec::new(),
            status: String::new(),
            settings: Settings::default(),
            pane_targets: Vec::new(),
        }
    }
    pub fn active_any_tab(&mut self) -> Option<&mut Tab> {
        self.dock
            .iter_all_tabs_mut()
            .find(|(_, tab)| tab.id == self.active)
            .map(|(_, tab)| tab)
    }
    pub fn active_tab(&mut self) -> Option<&mut Tab> {
        self.active_any_tab()
            .filter(|tab| tab.kind == LocationKind::Filesystem)
    }
    pub fn open_recycle_bin(&mut self) {
        if self.dock.iter_all_tabs().count() >= 128 {
            self.status = "Tab limit reached (128)".into();
            return;
        }
        self.next_id = self.next_id.max(
            self.dock
                .iter_all_tabs()
                .map(|(_, tab)| tab.id)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        );
        let id = self.next_id;
        self.next_id += 1;
        self.dock
            .push_to_focused_leaf(Tab::recycle_bin(id, self.home.clone()));
        self.active = id;
    }
    pub fn add_tab(&mut self, path: PathBuf) {
        if self.dock.iter_all_tabs().count() >= 128 {
            self.status = "Tab limit reached (128); close a tab before opening another".into();
            return;
        }
        self.next_id = self.next_id.max(
            self.dock
                .iter_all_tabs()
                .map(|(_, tab)| tab.id)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        );
        let id = self.next_id;
        self.next_id += 1;
        self.dock.push_to_focused_leaf(Tab::new(id, path));
        self.active = id;
    }
    fn reconcile_active(&mut self) {
        if self
            .dock
            .iter_all_tabs()
            .any(|(_, tab)| tab.id == self.active)
        {
            return;
        }
        self.active = self
            .dock
            .find_active_focused()
            .map(|(_, tab)| tab.id)
            .or_else(|| self.dock.iter_all_tabs().next().map(|(_, tab)| tab.id))
            .unwrap_or(0);
    }
    pub fn close_active(&mut self) {
        self.reconcile_active();
        if self.dock.iter_all_tabs().count() <= 1 {
            let home = self.home.clone();
            if let Some(tab) = self.active_any_tab() {
                tab.navigate(home);
            }
            return;
        }
        if let Some(path) = self.dock.find_tab_from(|tab| tab.id == self.active)
            && let Some(tab) = self.dock.remove_tab(path)
        {
            self.closed.push(ClosedTab::capture(&tab));
            if self.closed.len() > 32 {
                self.closed.remove(0);
            }
        }
        self.reconcile_active();
    }
    pub fn duplicate_active(&mut self) {
        if self.dock.iter_all_tabs().count() >= 128 {
            self.status = "Tab limit reached (128)".into();
            return;
        }
        if let Some(saved) = self.active_any_tab().map(|tab| ClosedTab::capture(tab)) {
            if saved.kind == LocationKind::RecycleBin {
                self.open_recycle_bin();
            } else {
                self.add_tab(saved.path.clone());
            }
            if let Some(tab) = self.active_any_tab() {
                saved.restore(tab);
            }
        }
    }
    pub fn reopen_closed(&mut self) {
        if self.dock.iter_all_tabs().count() >= 128 {
            self.status = "Tab limit reached (128)".into();
            return;
        }
        if let Some(saved) = self.closed.pop() {
            if saved.kind == LocationKind::RecycleBin {
                self.open_recycle_bin();
            } else {
                self.add_tab(saved.path.clone());
            }
            if let Some(tab) = self.active_any_tab() {
                saved.restore(tab);
            }
        }
    }
    pub fn equalize_panes(&mut self) {
        for node in self.dock.main_surface_mut().iter_mut() {
            if let egui_dock::Node::Horizontal(split) | egui_dock::Node::Vertical(split) = node {
                split.fraction = 0.5;
            }
        }
    }
    pub fn next_tab(&mut self, previous: bool) {
        let Some(path) = self.dock.find_tab_from(|tab| tab.id == self.active) else {
            return;
        };
        let Some(leaf) = self.dock[path.surface][path.node].get_leaf() else {
            return;
        };
        if leaf.tabs.is_empty() {
            return;
        }
        let index = if previous {
            (path.tab.0 + leaf.tabs.len() - 1) % leaf.tabs.len()
        } else {
            (path.tab.0 + 1) % leaf.tabs.len()
        };
        let id = leaf.tabs[index].id;
        if let Err(error) = self.dock.set_active_tab(egui_dock::TabPath::new(
            path.surface,
            path.node,
            egui_dock::TabIndex(index),
        )) {
            self.status = error.to_string();
            return;
        }
        self.active = id;
    }
    pub fn next_pane(&mut self) {
        let panes: Vec<_> = self
            .dock
            .main_surface()
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                let leaf = node.get_leaf()?;
                let tab = leaf.tabs.get(leaf.active.0)?;
                Some((NodeIndex(index), tab.id))
            })
            .collect();
        if panes.is_empty() {
            return;
        }
        let current = self
            .dock
            .find_tab_from(|tab| tab.id == self.active)
            .map(|path| path.node);
        let index = panes
            .iter()
            .position(|(node, _)| Some(*node) == current)
            .map_or(0, |index| (index + 1) % panes.len());
        self.dock
            .main_surface_mut()
            .set_focused_node(panes[index].0);
        self.active = panes[index].1;
    }
    /// Select confirmed outcomes only in matching filesystem tabs, after fresh visibility pruning.
    pub fn refresh_with_selection(&mut self, paths: Vec<PathBuf>) {
        for (_, tab) in self.dock.iter_all_tabs_mut() {
            if tab.kind != LocationKind::Filesystem {
                continue;
            }
            let selected: Vec<_> = paths
                .iter()
                .filter(|path| {
                    if tab.browser.options().expanded {
                        path.starts_with(tab.browser.current_path())
                            && path.as_path() != tab.browser.current_path()
                    } else {
                        path.parent() == Some(tab.browser.current_path())
                    }
                })
                .cloned()
                .collect();
            tab.refresh();
            if !selected.is_empty() {
                let mut selection = Selection::default();
                selection.select_all(&selected);
                tab.pending_selection = Some(selection);
            }
        }
    }
    pub fn refresh(&mut self) {
        for (_, tab) in self.dock.iter_all_tabs_mut() {
            tab.refresh();
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) -> Vec<Intent> {
        self.reconcile_active();
        self.pane_targets.clear();
        if !ui.ctx().egui_wants_keyboard_input() {
            if ui.input(|input| input.modifiers.ctrl && input.key_pressed(egui::Key::PageUp)) {
                self.next_tab(true);
            }
            if ui.input(|input| input.modifiers.ctrl && input.key_pressed(egui::Key::PageDown)) {
                self.next_tab(false);
            }
        }
        if !ui.ctx().egui_wants_keyboard_input()
            && ui.input(|input| input.key_pressed(egui::Key::F6))
        {
            self.next_pane();
        }

        for (_, tab) in self.dock.iter_all_tabs_mut() {
            tab.poll();
        }
        let mut add = false;
        let mut split = None;
        let mut reopen = false;
        ui.horizontal_wrapped(|ui| {
            add = ui.button("New tab").clicked();
            if ui.button("Duplicate tab").clicked() {
                self.duplicate_active();
            }
            if ui.button("Equalize panes").clicked() {
                self.equalize_panes();
            }
            split = if ui.button("Split right").clicked() {
                Some(false)
            } else if ui.button("Split below").clicked() {
                Some(true)
            } else {
                None
            };
            reopen = ui
                .add_enabled(!self.closed.is_empty(), egui::Button::new("Reopen tab"))
                .clicked();
            if ui.button("Home").clicked() {
                let home = self.home.clone();
                if let Some(t) = self.active_any_tab() {
                    t.navigate(home);
                }
            }
            ui.label(&self.status);
        });
        let current = self
            .active_tab()
            .map(|t| t.browser.current_path().to_owned())
            .unwrap_or_else(|| self.home.clone());
        if add {
            self.add_tab(current.clone());
        }
        if reopen {
            self.reopen_closed();
        }
        if let Some(below) = split {
            let node = self
                .dock
                .main_surface()
                .focused_leaf()
                .unwrap_or(NodeIndex::root());
            if self.dock.iter_all_tabs().count() >= 128 {
                self.status = "Tab limit reached (128); close a tab before splitting".into();
            } else if node.level() >= 12 {
                self.status =
                    "Pane depth limit reached (12); close or rearrange panes before splitting"
                        .into();
            } else {
                self.next_id = self.next_id.max(
                    self.dock
                        .iter_all_tabs()
                        .map(|(_, tab)| tab.id)
                        .max()
                        .unwrap_or(0)
                        .saturating_add(1),
                );
                let tab = Tab::new(self.next_id, current);
                self.active = self.next_id;
                self.next_id += 1;
                if below {
                    self.dock
                        .main_surface_mut()
                        .split_below(node, 0.5, vec![tab]);
                } else {
                    self.dock
                        .main_surface_mut()
                        .split_right(node, 0.5, vec![tab]);
                }
            }
        }
        let depth_limit = self
            .dock
            .main_surface()
            .iter()
            .enumerate()
            .any(|(index, node)| node.is_leaf() && NodeIndex(index).level() >= 12);
        if depth_limit {
            ui.label("Pane depth limit: drag-to-split disabled; tab reordering remains available.");
        }
        let count = self.dock.iter_all_tabs().count();
        let mut viewer = Viewer {
            active: &mut self.active,
            closed: &mut self.closed,
            intents: Vec::new(),
            count,
            home: &self.home,
            settings: &self.settings,
            pane_targets: &mut self.pane_targets,
        };
        DockArea::new(&mut self.dock)
            .allowed_splits(if depth_limit {
                egui_dock::AllowedSplits::None
            } else {
                egui_dock::AllowedSplits::All
            })
            .show_inside(ui, &mut viewer);
        let intents = viewer.intents;
        self.reconcile_active();
        intents
    }
}
struct Viewer<'a> {
    active: &'a mut u64,
    closed: &'a mut Vec<ClosedTab>,
    intents: Vec<Intent>,
    count: usize,
    home: &'a PathBuf,
    settings: &'a Settings,
    pane_targets: &'a mut Vec<(egui::Rect, PathBuf)>,
}
impl TabViewer for Viewer<'_> {
    type Tab = Tab;
    fn id(&mut self, tab: &mut Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }
    fn title(&mut self, tab: &mut Tab) -> egui::WidgetText {
        if tab.kind == LocationKind::RecycleBin {
            return "Recycle Bin".into();
        }
        tab.browser
            .current_path()
            .file_name()
            .unwrap_or(tab.browser.current_path().as_os_str())
            .to_string_lossy()
            .into_owned()
            .into()
    }
    fn allowed_in_windows(&self, _: &mut Tab) -> bool {
        false
    }
    fn scroll_bars(&self, _: &Tab) -> [bool; 2] {
        [false, false]
    }
    fn on_tab_button(&mut self, tab: &mut Tab, r: &egui::Response) {
        if r.clicked() {
            *self.active = tab.id;
        }
    }
    fn on_close(&mut self, tab: &mut Tab) -> OnCloseResponse {
        self.closed.push(ClosedTab::capture(tab));
        if self.closed.len() > 32 {
            self.closed.remove(0);
        }
        if self.count <= 1 {
            tab.navigate(self.home.clone());
            OnCloseResponse::Ignore
        } else {
            OnCloseResponse::Close
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Tab) {
        if tab.kind == LocationKind::RecycleBin {
            if ui.rect_contains_pointer(ui.max_rect())
                && ui.input(|input| input.pointer.any_pressed())
            {
                *self.active = tab.id;
            }
            if let Some(bin) = &mut tab.bin_view {
                bin.show(ui);
            }
            return;
        }
        if ui.rect_contains_pointer(ui.max_rect()) && ui.input(|i| i.pointer.any_pressed()) {
            *self.active = tab.id;
        }
        let mut target = None;
        ui.horizontal(|ui| {
            if ui
                .add_enabled(tab.history_index > 0, egui::Button::new("Back"))
                .clicked()
            {
                tab.history(false);
            }
            if ui
                .add_enabled(
                    tab.history_index + 1 < tab.history.len(),
                    egui::Button::new("Forward"),
                )
                .clicked()
            {
                tab.history(true);
            }
            if ui.button("Up").clicked() {
                target = tab.browser.current_path().parent().map(|p| p.to_owned());
            }
            if ui.button("Refresh").clicked() {
                target = Some(tab.browser.current_path().to_owned());
            }
            let path = ui.text_edit_singleline(&mut tab.path_edit);
            tab.path_dirty |= path.changed();
            if tab.path_dirty && path.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                let candidate = PathBuf::from(&tab.path_edit);
                target = Some(if candidate.is_absolute() {
                    candidate
                } else {
                    tab.browser.current_path().join(candidate)
                });
            }
        });
        let mut options = tab.browser.options().clone();
        let typed = if *self.active == tab.id
            && !ui.ctx().egui_wants_keyboard_input()
            && ui.input(|input| !input.modifiers.ctrl && !input.modifiers.alt)
        {
            ui.input(|input| {
                input
                    .events
                    .iter()
                    .filter_map(|event| {
                        if let egui::Event::Text(text) = event
                            && !text.chars().any(char::is_control)
                            && self
                                .settings
                                .command_for(&text.to_uppercase(), false)
                                .is_none()
                        {
                            return Some(text.as_str());
                        }
                        None
                    })
                    .collect::<String>()
            })
        } else {
            String::new()
        };
        options.filter.push_str(&typed);
        ui.horizontal_wrapped(|ui| {
            ui.label("Filter");
            let filter = ui.add(
                egui::TextEdit::singleline(&mut options.filter)
                    .id(egui::Id::new((tab.id, "filename-filter"))),
            );
            if !typed.is_empty() {
                filter.request_focus();
            }
            ui.checkbox(&mut options.show_hidden, "Hidden");
            ui.checkbox(&mut options.show_system, "System");
            ui.checkbox(&mut options.expanded, "Descendants");
            ui.checkbox(&mut options.directories_first, "Folders first");
            egui::ComboBox::from_id_salt((tab.id, "sort"))
                .selected_text(format!("{:?}", options.sort))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut options.sort, SortColumn::Name, "Name");
                    ui.selectable_value(&mut options.sort, SortColumn::Size, "Size");
                    ui.selectable_value(&mut options.sort, SortColumn::Modified, "Modified");
                    ui.selectable_value(&mut options.sort, SortColumn::Type, "Type");
                });
            ui.checkbox(&mut options.descending, "Descending");
            ui.selectable_value(&mut tab.view, ListingView::Details, "Details");
            ui.selectable_value(&mut tab.view, ListingView::Icons, "Icons");
            if tab.view == ListingView::Icons {
                ui.add(egui::Slider::new(&mut tab.icon_size, 48.0..=192.0).text("Zoom"));
            }
        });
        if &options != tab.browser.options() {
            tab.pending_selection = Some(
                tab.pending_selection
                    .as_ref()
                    .unwrap_or(&tab.selection)
                    .clone(),
            );
            tab.selection.clear();
            tab.visible.clear();
            tab.browser.set_options(options);
        }
        let previous_cursor = tab.selection.cursor().map(PathBuf::from);
        if *self.active == tab.id && !ui.ctx().egui_wants_keyboard_input() {
            let m = ui.input(|i| i.modifiers);
            let step = if tab.view == ListingView::Icons {
                ((ui.available_width() / (tab.icon_size.clamp(48.0, 192.0) + 24.0)).floor()
                    as isize)
                    .max(1)
            } else {
                1
            };
            if !m.alt && ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                tab.selection
                    .move_cursor(step, m.ctrl, m.shift, &tab.visible);
            }
            if !m.alt && ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                tab.selection
                    .move_cursor(-step, m.ctrl, m.shift, &tab.visible);
            }
            if tab.view == ListingView::Icons && !m.alt {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                    tab.selection.move_cursor(-1, m.ctrl, m.shift, &tab.visible);
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                    tab.selection.move_cursor(1, m.ctrl, m.shift, &tab.visible);
                }
            }
            if m.alt && ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                tab.history(false);
            }
            if m.alt && ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                tab.history(true);
            }
            if ui.input(|i| i.key_pressed(egui::Key::Enter))
                && let Some(path) = tab.selection.cursor()
            {
                if tab
                    .browser
                    .snapshot()
                    .is_some_and(|s| s.entries.iter().any(|e| e.path == path && e.is_dir))
                {
                    target = Some(path.to_owned());
                } else {
                    self.intents.push(Intent::Open(path.to_owned()));
                }
            }
            if m.shift && ui.input(|i| i.key_pressed(egui::Key::F10)) {
                self.intents
                    .push(Intent::Shell(tab.selection.paths().cloned().collect()));
            }
        }
        let scroll_to = if tab.selection.cursor() != previous_cursor.as_deref() {
            tab.selection
                .cursor()
                .and_then(|cursor| tab.visible.iter().position(|path| path == cursor))
        } else {
            None
        };
        if let Some(error) = tab.browser.watcher_error() {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if tab.browser.is_loading() {
            ui.spinner();
            ui.label("Loading…");
        }
        tab.rows_rendered = 0;
        if tab.view == ListingView::Icons {
            tab.thumbnails
                .get_or_insert_with(thumbnails::Thumbnails::new)
                .synchronize(tab.browser.generation(), ui.ctx());
        } else {
            tab.thumbnails = None;
        }
        if let Some(snapshot) = tab.browser.snapshot() {
            if let Some(error) = &snapshot.error {
                ui.colored_label(egui::Color32::LIGHT_RED, error);
            }
            if snapshot.entries.is_empty() && snapshot.error.is_none() {
                ui.label("No matching items");
            }
            if tab.browser.options().expanded {
                ui.label("Descendants: reparse points are not followed. Full relative paths identify duplicate names.");
            }
            if let Some(cache) = &tab.thumbnails {
                let (pending, cached, bytes) = cache.stats();
                ui.label(format!("Thumbnails: {pending}/32 pending, {cached}/256 cached; global {} KiB / 32768 KiB", bytes / 1024));
            }
            ui.label(format!(
                "{} items • {} selected",
                snapshot.entries.len(),
                tab.selection.len()
            ));
            if tab.view == ListingView::Icons {
                let edge = tab.icon_size.clamp(48.0, 192.0);
                let cell_width = edge + 24.0;
                let row_height = edge + 62.0;
                let columns = ((ui.available_width() / cell_width).floor() as usize).max(1);
                let rows = snapshot.entries.len().div_ceil(columns);
                let mut scroll = egui::ScrollArea::vertical().id_salt((tab.id, "icons"));
                if let Some(index) = scroll_to {
                    scroll = scroll.vertical_scroll_offset(
                        (index / columns) as f32 * (row_height + ui.spacing().item_spacing.y),
                    );
                }
                scroll.show_rows(ui, row_height, rows, |ui, range| {
                    for row in range {
                        tab.rows_rendered += 1;
                        ui.horizontal(|ui| {
                            for entry in snapshot.entries.iter().skip(row * columns).take(columns) {
                                ui.allocate_ui_with_layout(
                                    egui::vec2(cell_width, row_height),
                                    egui::Layout::top_down(egui::Align::Center),
                                    |ui| {
                                        let editing = ui.ctx().egui_wants_keyboard_input();
                                        let cache = tab
                                            .thumbnails
                                            .as_mut()
                                            .expect("icon cache initialized");
                                        if !entry.is_dir {
                                            cache.request(&entry.path, ui.ctx());
                                        }
                                        let (rect, response) = ui.allocate_exact_size(
                                            egui::vec2(edge, edge),
                                            egui::Sense::click_and_drag(),
                                        );
                                        if tab.selection.contains(&entry.path) {
                                            ui.painter().rect_filled(
                                                rect,
                                                4.0,
                                                ui.visuals().selection.bg_fill,
                                            );
                                        }
                                        if let Some(texture) = cache.texture(&entry.path) {
                                            let size = texture.size_vec2();
                                            let scale = (edge / size.x).min(edge / size.y);
                                            let image_rect = egui::Rect::from_center_size(
                                                rect.center(),
                                                size * scale,
                                            );
                                            ui.painter().image(
                                                texture.id(),
                                                image_rect,
                                                egui::Rect::from_min_max(
                                                    egui::Pos2::ZERO,
                                                    egui::pos2(1.0, 1.0),
                                                ),
                                                egui::Color32::WHITE,
                                            );
                                        } else {
                                            ui.painter().text(
                                                rect.center(),
                                                egui::Align2::CENTER_CENTER,
                                                if entry.is_dir {
                                                    "Folder"
                                                } else if cache.error(&entry.path).is_some() {
                                                    "No preview"
                                                } else {
                                                    "File"
                                                },
                                                egui::FontId::proportional(14.0),
                                                ui.visuals().text_color(),
                                            );
                                        }
                                        let label = if tab.browser.options().expanded {
                                            entry
                                                .path
                                                .strip_prefix(&snapshot.path)
                                                .unwrap_or(&entry.path)
                                                .as_os_str()
                                        } else {
                                            &entry.name
                                        };
                                        ui.add(
                                            egui::Label::new(label.to_string_lossy()).truncate(),
                                        )
                                        .on_hover_text(entry.path.to_string_lossy());
                                        if let Some(error) = cache.error(&entry.path) {
                                            response.clone().on_hover_text(error);
                                        }
                                        if self.settings.checkboxes {
                                            let mut checked = tab.selection.contains(&entry.path);
                                            if ui.checkbox(&mut checked, "Select").changed() {
                                                tab.selection.click(
                                                    entry.path.clone(),
                                                    true,
                                                    false,
                                                    &tab.visible,
                                                );
                                                *self.active = tab.id;
                                            }
                                        }
                                        response.widget_info(|| {
                                            egui::WidgetInfo::selected(
                                                egui::WidgetType::Button,
                                                ui.is_enabled(),
                                                tab.selection.contains(&entry.path),
                                                label.to_string_lossy(),
                                            )
                                        });
                                        EntryInteraction {
                                            active: self.active,
                                            intents: &mut self.intents,
                                            selection: &mut tab.selection,
                                            visible: &tab.visible,
                                            settings: self.settings,
                                            tab_id: tab.id,
                                            target: &mut target,
                                        }
                                        .respond(ui, entry, &response, editing);
                                    },
                                );
                            }
                        });
                    }
                });
            } else {
                let mut table = TableBuilder::new(ui);
                if let Some(index) = scroll_to {
                    table = table.scroll_to_row(index, None);
                }
                table
                    .striped(true)
                    .resizable(true)
                    .column(Column::remainder().at_least(120.0))
                    .column(Column::initial(75.0))
                    .column(Column::initial(90.0))
                    .column(Column::initial(145.0))
                    .header(24.0, |mut row| {
                        for label in ["Name", "Type", "Bytes", "Modified (Unix seconds)"] {
                            row.col(|ui| {
                                ui.strong(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(24.0, snapshot.entries.len(), |mut row| {
                            tab.rows_rendered += 1;
                            let entry = &snapshot.entries[row.index()];
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    let editing = ui.ctx().egui_wants_keyboard_input();
                                    if self.settings.checkboxes {
                                        let mut checked = tab.selection.contains(&entry.path);
                                        if ui
                                            .checkbox(&mut checked, "")
                                            .on_hover_text("Select item")
                                            .changed()
                                        {
                                            tab.selection.click(
                                                entry.path.clone(),
                                                true,
                                                false,
                                                &tab.visible,
                                            );
                                            *self.active = tab.id;
                                        }
                                    }
                                    let r = ui.add(
                                        egui::Button::selectable(
                                            tab.selection.contains(&entry.path),
                                            if tab.browser.options().expanded {
                                                entry
                                                    .path
                                                    .strip_prefix(&snapshot.path)
                                                    .unwrap_or(&entry.path)
                                                    .as_os_str()
                                                    .to_string_lossy()
                                            } else {
                                                entry.name.to_string_lossy()
                                            },
                                        )
                                        .sense(egui::Sense::click_and_drag()),
                                    );
                                    EntryInteraction {
                                        active: self.active,
                                        intents: &mut self.intents,
                                        selection: &mut tab.selection,
                                        visible: &tab.visible,
                                        settings: self.settings,
                                        tab_id: tab.id,
                                        target: &mut target,
                                    }
                                    .respond(ui, entry, &r, editing);
                                });
                            });
                            row.col(|ui| {
                                ui.label(if entry.is_dir { "Folder" } else { "File" });
                            });
                            row.col(|ui| {
                                ui.label(entry.size.map(|v| v.to_string()).unwrap_or_default());
                            });
                            row.col(|ui| {
                                ui.label(
                                    entry
                                        .modified
                                        .and_then(|v| v.duration_since(std::time::UNIX_EPOCH).ok())
                                        .map(|d| d.as_secs().to_string())
                                        .unwrap_or_default(),
                                );
                            });
                        })
                    });
            }
        }
        if let Some(path) = target {
            if path == tab.browser.current_path() {
                tab.refresh();
            } else {
                tab.navigate(path);
            }
        }
        self.pane_targets
            .push((ui.max_rect(), tab.browser.current_path().to_owned()));
    }
}
pub fn execute(intents: Vec<Intent>, hwnd: HWND) -> Option<String> {
    let mut error = None;
    for intent in intents {
        let result = match intent {
            Intent::Open(path) => shell::open_path(&path),
            Intent::Drag(paths) => crate::platform::drag_drop::drag(&paths).map(|_| ()),
            Intent::Shell(paths) => {
                let mut point = windows::Win32::Foundation::POINT::default();
                unsafe {
                    let _ = windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut point);
                }
                shell::show_context_menu(hwnd, &paths, point.x, point.y)
            }
        };
        if let Err(e) = result {
            error = Some(e);
        }
    }
    error
}

struct EntryInteraction<'a> {
    active: &'a mut u64,
    intents: &'a mut Vec<Intent>,
    selection: &'a mut Selection,
    visible: &'a [PathBuf],
    settings: &'a Settings,
    tab_id: u64,
    target: &'a mut Option<PathBuf>,
}
impl EntryInteraction<'_> {
    fn respond(&mut self, ui: &egui::Ui, entry: &Entry, r: &egui::Response, editing: bool) {
        if self.selection.cursor() == Some(entry.path.as_path()) && *self.active == self.tab_id {
            ui.painter().rect_stroke(
                r.rect,
                3.0,
                egui::Stroke::new(2.0, ui.visuals().selection.stroke.color),
                egui::StrokeKind::Inside,
            );
        }
        if r.clicked() {
            let m = ui.input(|i| i.modifiers);
            self.selection
                .click(entry.path.clone(), m.ctrl, m.shift, self.visible);
            *self.active = self.tab_id;
        }
        if r.drag_started() {
            if !self.selection.contains(&entry.path) {
                self.selection
                    .click(entry.path.clone(), false, false, self.visible);
            }
            *self.active = self.tab_id;
            self.intents
                .push(Intent::Drag(self.selection.paths().cloned().collect()));
        }
        let modifiers = ui.input(|i| i.modifiers);
        if (r.clicked() || r.double_clicked())
            && !modifiers.ctrl
            && !modifiers.shift
            && self
                .settings
                .click_mode
                .activates(entry.is_dir, r.double_clicked(), editing)
        {
            if entry.is_dir {
                *self.target = Some(entry.path.clone());
            } else {
                self.intents.push(Intent::Open(entry.path.clone()));
            }
        }
        if r.secondary_clicked() {
            if !self.selection.contains(&entry.path) {
                self.selection
                    .click(entry.path.clone(), false, false, self.visible);
            }
            self.intents
                .push(Intent::Shell(self.selection.paths().cloned().collect()));
        }
    }
}

#[cfg(test)]
mod browsing_state_tests {
    use super::*;
    use std::time::{Duration, Instant, SystemTime};
    fn settle(tab: &mut Tab) {
        let until = Instant::now() + Duration::from_secs(5);
        while tab.browser.is_loading() {
            tab.poll();
            assert!(Instant::now() < until, "listing timed out");
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    #[test]
    fn watch_refresh_retains_visible_selection_and_prunes_confirmed_outcomes() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-selection-refresh-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let first = root.join("first.txt");
        let second = root.join("second.txt");
        std::fs::write(&first, b"fixture").unwrap();
        std::fs::write(&second, b"fixture").unwrap();
        let mut ui = BrowserUi::new(root.clone(), root.clone());
        let tab = ui.active_tab().unwrap();
        settle(tab);
        tab.selection
            .click(first.clone(), false, false, &tab.visible);
        tab.refresh();
        assert!(tab.selection.is_empty());
        settle(tab);
        assert!(tab.selection.contains(&first));
        let mut options = tab.browser.options().clone();
        options.filter = "first".into();
        tab.browser.set_options(options);
        settle(tab);
        ui.refresh_with_selection(vec![first.clone(), second.clone(), root.join("absent")]);
        let tab = ui.active_tab().unwrap();
        settle(tab);
        assert_eq!(tab.selection.len(), 1);
        assert!(tab.selection.contains(&first));
        drop(ui);
        std::fs::remove_file(&first).unwrap();
        std::fs::remove_file(&second).unwrap();
        let until = Instant::now() + Duration::from_secs(5);
        while let Err(error) = std::fs::remove_dir(&root) {
            assert!(Instant::now() < until, "cleanup: {error}");
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    #[test]
    fn tabs_keyboard_switch_stays_within_active_pane() {
        let path = std::env::temp_dir();
        let mut ui = BrowserUi::new(path.clone(), path.clone());
        ui.add_tab(path);
        let second = ui.active;
        ui.next_tab(true);
        assert_eq!(ui.active, 1);
        ui.next_tab(false);
        assert_eq!(ui.active, second);
    }
}
