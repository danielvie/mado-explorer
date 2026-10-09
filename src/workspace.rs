use super::*;

impl Explorer {
    pub(super) fn navigate(&mut self, location: PathBuf, intent: Intent) {
        self.navigate_in(self.focused, location, intent);
    }

    fn navigate_in(&mut self, pane: usize, location: PathBuf, intent: Intent) {
        let pane = &mut self.panes[pane];
        let tab = &mut pane.tabs[pane.active];
        let generation = self.loader.request_for(tab.id.0, location.clone());
        self.sorter.cancel(tab.id.0);
        tab.begin(location, generation, intent);
    }

    pub(super) fn request_sort(&mut self) {
        for pane in &mut self.panes {
            let tab = &mut pane.tabs[pane.active];
            let key = tab.navigation.key();
            if tab.requested_sort != Some(key) {
                self.sorter.request_for(
                    tab.id.0,
                    key,
                    tab.navigation.visible().raw_entries.clone(),
                    tab.navigation.filter.clone(),
                );
                tab.requested_sort = Some(key);
            }
        }
    }

    pub(super) fn poll_directory(&mut self) -> bool {
        let more = self.loader.drain(|response| {
            if let Some(tab) = self
                .panes
                .iter_mut()
                .map(|p| &mut p.tabs[p.active])
                .find(|tab| tab.navigation.key().generation == response.generation)
            {
                tab.apply(response);
            }
        });
        self.request_sort();
        // One result per visible target; catch-up is bounded by the pane count.
        for _ in 0..self.panes.len() {
            let Some(sorted) = self.sorter.take() else {
                break;
            };
            if let Some(tab) = self
                .panes
                .iter_mut()
                .map(|p| &mut p.tabs[p.active])
                .find(|tab| tab.navigation.key().generation == sorted.key.generation)
            {
                tab.navigation.apply_sorted(sorted);
            }
        }
        more
    }

    pub(super) fn activate_tab(&mut self, index: usize) {
        self.activate_tab_in(self.focused, index);
    }

    fn activate_tab_in(&mut self, pane: usize, index: usize) {
        let p = &mut self.panes[pane];
        if index == p.active {
            return;
        }
        p.cancel_tab(p.active, &self.loader, &self.sorter);
        p.tabs[p.active].deactivate();
        p.active = index;
        p.reveal_active_tab = true;
        self.refresh_tab_in(pane);
    }

    pub(super) fn refresh_tab_in(&mut self, pane: usize) {
        let p = &mut self.panes[pane];
        let tab = &mut p.tabs[p.active];
        let generation = self
            .loader
            .request_for(tab.id.0, tab.navigation.current.location.clone());
        self.sorter.cancel(tab.id.0);
        tab.resume(generation);
    }

    pub(super) fn new_tab(&mut self) {
        let p = &self.panes[self.focused];
        let location = p.tabs[p.active].navigation.current.location.clone();
        self.insert_tab(location);
    }

    pub(super) fn fresh_tab(&mut self, location: PathBuf) -> Tab {
        let id = TabId(self.next_tab_id);
        self.next_tab_id = self.next_tab_id.checked_add(1).expect("Tab ID exhausted");
        Tab::new(id, location)
    }

    pub(super) fn insert_tab(&mut self, location: PathBuf) {
        let tab = self.fresh_tab(location);
        let p = &mut self.panes[self.focused];
        p.tabs.push(tab);
        let index = p.tabs.len() - 1;
        self.activate_tab(index);
    }

    pub(super) fn close_tab(&mut self, index: usize) {
        let p = &self.panes[self.focused];
        p.cancel_tab(index, &self.loader, &self.sorter);
        if p.tabs.len() == 1 {
            self.close_pane();
        } else if index == p.active {
            let p = &mut self.panes[self.focused];
            p.tabs.remove(index);
            p.active = index.min(p.tabs.len() - 1);
            p.reveal_active_tab = true;
            self.refresh_tab_in(self.focused);
        } else {
            let p = &mut self.panes[self.focused];
            p.tabs.remove(index);
            if index < p.active {
                p.active -= 1;
            }
        }
    }

    pub(super) fn close_pane(&mut self) {
        let p = &self.panes[self.focused];
        for index in 0..p.tabs.len() {
            p.cancel_tab(index, &self.loader, &self.sorter);
        }
        if self.panes.len() == 1 {
            let tab = self.fresh_tab(self.home.clone());
            self.panes[0].tabs = vec![tab];
            self.panes[0].active = 0;
            self.panes[0].reveal_active_tab = true;
            self.refresh_tab_in(0);
        } else {
            let id = p.id;
            assert!(self.layout.remove(id));
            self.panes.remove(self.focused);
            self.focused = self.focused.min(self.panes.len() - 1);
        }
        self.reveal_focused_pane = true;
        self.layout_error = None;
    }

    pub(super) fn split_pane(&mut self, axis: Axis) -> bool {
        let p = &self.panes[self.focused];
        let Some((_, rect)) = self.pane_rects.iter().find(|(id, _)| *id == p.id) else {
            self.layout_error = Some("Layout is not ready; try the split again.".into());
            return false;
        };
        if !self.visible_workspace.contains_rect(*rect) {
            self.layout_error = Some(
                "Cannot split an offscreen pane: enlarge the window or scroll it fully into view."
                    .into(),
            );
            return false;
        }
        let id = PaneId(self.next_pane_id);
        if !self.layout.split(p.id, id, axis, rect.size()) {
            self.layout_error =
                Some("Cannot split: enlarge this pane first (minimum 320 x 300 per pane).".into());
            return false;
        }
        self.next_pane_id = self.next_pane_id.checked_add(1).expect("Pane ID exhausted");
        let location = p.tabs[p.active].navigation.current.location.clone();
        let tab = self.fresh_tab(location);
        self.panes.push(Pane {
            id,
            tabs: vec![tab],
            active: 0,
            reveal_active_tab: true,
        });
        self.focused = self.panes.len() - 1;
        self.refresh_tab_in(self.focused);
        self.layout_error = None;
        self.reveal_focused_pane = true;
        true
    }

    pub(super) fn focus_pane(&mut self, pane: usize) {
        if self.focused != pane {
            let old = &mut self.panes[self.focused];
            old.tabs[old.active].location_editor_focused = false;
            self.focused = pane;
            self.reveal_focused_pane = true;
        }
    }

    pub(super) fn cycle_pane(&mut self) {
        let id = self.panes[self.focused].id;
        if let Some(index) = self
            .pane_rects
            .iter()
            .position(|(candidate, _)| *candidate == id)
        {
            let next = self.pane_rects[(index + 1) % self.pane_rects.len()].0;
            let pane = self.panes.iter().position(|p| p.id == next).unwrap();
            self.focus_pane(pane);
        }
    }

    pub(super) fn cycle_tab(&mut self, backwards: bool) {
        let p = &self.panes[self.focused];
        let count = p.tabs.len();
        self.activate_tab((p.active + if backwards { count - 1 } else { 1 }) % count);
    }

    pub(super) fn apply_tab_shortcut(&mut self, command: TabShortcut) {
        match command {
            TabShortcut::New => self.new_tab(),
            TabShortcut::Close => self.close_tab(self.panes[self.focused].active),
            TabShortcut::Next => self.cycle_tab(false),
            TabShortcut::Previous => self.cycle_tab(true),
        }
    }

    #[cfg(test)]
    pub(super) fn tab_strip(&mut self, ui: &mut egui::Ui) {
        self.tab_strip_in(ui, self.focused);
    }

    fn tab_strip_in(&mut self, ui: &mut egui::Ui, pane: usize) {
        let mut activate = None;
        let mut close = None;
        let mut new = false;
        let mut dragging = false;
        let mut tabs = Vec::new();
        let mut strip_rect = egui::Rect::NOTHING;
        let p = &mut self.panes[pane];
        ui.horizontal(|ui| {
            let width = (ui.available_width() - 40.0).max(80.0);
            let strip = egui::ScrollArea::horizontal()
                .id_salt((p.id, "pane-tabs"))
                .max_width(width)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (index, tab) in p.tabs.iter().enumerate() {
                            let group = ui.push_id(tab.id, |ui| {
                                let location = &tab.navigation.current.location;
                                let label = location.file_name().unwrap_or(location.as_os_str());
                                let response = ui
                                    .add_sized(
                                        [140.0, 26.0],
                                        egui::Button::new(native::display(label))
                                            .selected(index == p.active)
                                            .sense(egui::Sense::click_and_drag())
                                            .truncate(),
                                    )
                                    .on_hover_text(native::display(location.as_os_str()));
                                if index == p.active && p.reveal_active_tab {
                                    response.scroll_to_me(Some(egui::Align::Center));
                                }
                                if response.drag_started_by(egui::PointerButton::Primary) {
                                    egui::DragAndDrop::set_payload(
                                        ui.ctx(),
                                        tab_drag::TabDrag {
                                            source: p.id,
                                            tab: tab.id,
                                        },
                                    );
                                    dragging = true;
                                }
                                if response.clicked() {
                                    activate = Some(index);
                                }
                                if ui
                                    .button("x")
                                    .on_hover_text(format!(
                                        "Close tab: {} (Ctrl+W closes the active tab)",
                                        native::display(location.as_os_str())
                                    ))
                                    .clicked()
                                {
                                    close = Some(index);
                                }
                            });
                            tabs.push((tab.id, group.response.rect));
                        }
                    });
                });
            strip_rect = strip.inner_rect.intersect(ui.clip_rect());
            new = ui
                .button("+")
                .on_hover_text("New tab at the committed location (Ctrl+T)")
                .clicked();
        });
        p.reveal_active_tab = false;
        self.tab_strips.push(tab_drag::TabStripGeometry {
            pane: p.id,
            rect: strip_rect,
            tabs,
        });
        if dragging {
            surrender_focus(ui.ctx());
        }
        if close.is_some() || activate.is_some() || new {
            self.focus_pane(pane);
            if let Some(index) = close {
                self.close_tab(index);
            } else if let Some(index) = activate {
                self.activate_tab(index);
            } else {
                self.new_tab();
            }
            surrender_focus(ui.ctx());
            ui.ctx().request_repaint();
        }
    }

    #[cfg(test)]
    pub(super) fn toolbar(&mut self, ui: &mut egui::Ui) -> bool {
        self.toolbar_in(ui, self.focused)
    }

    fn toolbar_in(&mut self, ui: &mut egui::Ui, pane: usize) -> bool {
        let p = &mut self.panes[pane];
        let tab = &mut p.tabs[p.active];
        let dragging = egui::DragAndDrop::has_payload_of_type::<tab_drag::TabDrag>(ui.ctx());
        let (editing, command) = ui
            .push_id(tab.id, |ui| {
                tab.toolbar(ui, pane == self.focused && !dragging)
            })
            .inner;
        if let Some((location, intent)) = command {
            self.navigate_in(pane, location, intent);
        }
        editing
    }

    #[cfg(test)]
    pub(super) fn status(&self, ui: &mut egui::Ui) {
        self.window_status(ui);
        let p = &self.panes[self.focused];
        let tab = &p.tabs[p.active];
        ui.push_id(tab.id, |ui| tab.status(ui));
    }

    #[cfg(test)]
    pub(super) fn table(&mut self, ui: &mut egui::Ui, editing: bool) -> Option<Activation> {
        self.table_in(ui, self.focused, editing)
    }

    fn table_in(&mut self, ui: &mut egui::Ui, pane: usize, editing: bool) -> Option<Activation> {
        let p = &mut self.panes[pane];
        let tab = &mut p.tabs[p.active];
        let dragging = egui::DragAndDrop::has_payload_of_type::<tab_drag::TabDrag>(ui.ctx());
        ui.push_id(tab.id, |ui| {
            tab.table(ui, editing, pane == self.focused && !dragging)
        })
        .inner
    }
}

pub(super) fn surrender_focus(ctx: &egui::Context) {
    ctx.memory_mut(|memory| {
        if let Some(id) = memory.focused() {
            memory.surrender_focus(id);
        }
    });
}

impl Explorer {
    pub(super) fn workspace(&mut self, ui: &mut egui::Ui) {
        let mut split = None;
        ui.horizontal(|ui| {
            if ui.button("Split left/right").clicked() {
                split = Some(Axis::LeftRight);
                surrender_focus(ui.ctx());
            }
            if ui.button("Split top/bottom").clicked() {
                split = Some(Axis::TopBottom);
                surrender_focus(ui.ctx());
            }
            if ui
                .button("Close pane")
                .on_hover_text("Close focused pane; final pane gets a fresh home tab")
                .clicked()
            {
                self.close_pane();
                surrender_focus(ui.ctx());
            }
        });
        if let Some(error) = &self.layout_error {
            ui.label(error);
        }
        self.window_status(ui);
        let size = ui.available_size().max(self.layout.minimum());
        let mut complete_geometry = true;
        self.tab_strips.clear();
        egui::ScrollArea::both()
            .id_salt("workspace-layout")
            .animated(false)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let (bounds, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                self.visible_workspace = ui.clip_rect();
                self.pane_rects.clear();
                let mut dividers = Vec::new();
                self.layout
                    .rectangles(bounds, &mut self.pane_rects, &mut dividers);
                // Use this frame's viewport, not stale pre-resize geometry.
                if let Some(axis) = split {
                    if self.split_pane(axis) {
                        self.pane_rects.clear();
                        dividers.clear();
                        self.layout
                            .rectangles(bounds, &mut self.pane_rects, &mut dividers);
                    }
                    ui.ctx().request_repaint();
                }
                // Resolve pointer focus before *any* pane can consume keyboard input.
                let clicked = ui.input(|input| {
                    input
                        .pointer
                        .any_pressed()
                        .then(|| input.pointer.interact_pos())
                        .flatten()
                });
                if let Some(pointer) = clicked.filter(|p| ui.clip_rect().contains(*p)) {
                    if let Some((id, _)) = self
                        .pane_rects
                        .iter()
                        .find(|(_, rect)| rect.contains(pointer))
                    {
                        let pane = self.panes.iter().position(|pane| pane.id == *id).unwrap();
                        if pane != self.focused {
                            self.focus_pane(pane);
                            surrender_focus(ui.ctx());
                        }
                    }
                }
                if ui.input_mut(|input| {
                    consume_exact_key(input, egui::Modifiers::NONE, egui::Key::F6)
                }) {
                    self.cycle_pane();
                    surrender_focus(ui.ctx());
                }
                let rectangles = self.pane_rects.clone();
                for (id, rect) in rectangles {
                    let pane = self.panes.iter().position(|p| p.id == id).unwrap();
                    // Keep pointer hit testing live in inactive panes, but never let
                    // their widgets consume another pane's key/text/clipboard input.
                    let saved_events = if pane != self.focused {
                        Some(ui.input_mut(|input| {
                            let original = input.events.clone();
                            input.events.retain(|event| {
                                !matches!(
                                    event,
                                    egui::Event::Key { .. }
                                        | egui::Event::Text(_)
                                        | egui::Event::Ime(_)
                                        | egui::Event::Copy
                                        | egui::Event::Cut
                                        | egui::Event::Paste(_)
                                )
                            });
                            original
                        }))
                    } else {
                        None
                    };
                    let changed = self.render_pane(ui, pane, id, rect);
                    if let Some(events) = saved_events {
                        ui.input_mut(|input| input.events = events);
                    }
                    if changed {
                        complete_geometry = false;
                        return;
                    }
                }
                for divider in dividers {
                    let response = ui
                        .interact(
                            divider.rect,
                            egui::Id::new((divider.id, "divider")),
                            egui::Sense::drag(),
                        )
                        .on_hover_cursor(match divider.axis {
                            Axis::LeftRight => egui::CursorIcon::ResizeHorizontal,
                            Axis::TopBottom => egui::CursorIcon::ResizeVertical,
                        });
                    ui.painter().rect_filled(
                        divider.rect,
                        0.0,
                        if response.hovered() || response.dragged() {
                            ui.visuals().selection.bg_fill
                        } else {
                            ui.visuals().widgets.inactive.bg_fill
                        },
                    );
                    if response.dragged() {
                        if let Some(pointer) = response.interact_pointer_pos() {
                            self.layout.resize(&divider, pointer);
                            ui.ctx().request_repaint();
                        }
                    }
                }
            });
        self.finish_tab_drag(ui, complete_geometry);
        self.request_sort();
    }
    fn render_pane(
        &mut self,
        ui: &mut egui::Ui,
        pane: usize,
        id: PaneId,
        rect: egui::Rect,
    ) -> bool {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(id)
                .max_rect(rect.shrink(5.0))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        child.set_clip_rect(rect.intersect(ui.clip_rect()));
        if self.reveal_focused_pane && pane == self.focused {
            let indicator =
                egui::Rect::from_min_size(rect.min + egui::vec2(5.0, 5.0), egui::vec2(240.0, 28.0));
            if ui.clip_rect().contains_rect(indicator) {
                self.reveal_focused_pane = false;
            } else {
                child.scroll_to_rect(indicator, None);
                ui.ctx().request_repaint();
            }
        }
        let focused = pane == self.focused;
        child.strong(if focused {
            "Focused pane (F6: next)"
        } else {
            "Pane"
        });
        let count = self.panes.len();
        let old_tab = self.panes[pane].tabs[self.panes[pane].active].id;
        self.tab_strip_in(&mut child, pane);
        // Closing/cycling a context cannot dispatch old-context row commands.
        if self.panes.len() != count
            || self.panes[pane].id != id
            || self.panes[pane].tabs[self.panes[pane].active].id != old_tab
        {
            ui.ctx().request_repaint();
            return true;
        }
        egui::ScrollArea::vertical()
            .id_salt((id, "pane-controls"))
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                let dragging =
                    egui::DragAndDrop::has_payload_of_type::<tab_drag::TabDrag>(ui.ctx());
                let editing = self.toolbar_in(ui, pane) || dragging;
                if pane == self.focused && !dragging {
                    if let Some(command) = tab_shortcut(ui.ctx(), editing) {
                        self.apply_tab_shortcut(command);
                        surrender_focus(ui.ctx());
                        ui.ctx().request_repaint();
                        return;
                    }
                }
                ui.separator();
                let p = &self.panes[pane];
                let tab = &p.tabs[p.active];
                ui.push_id(tab.id, |ui| tab.status(ui));
                ui.separator();
                // A short pane scrolls its controls instead of overlapping the
                // table. The table keeps its own independent row scroll state.
                let table_rect = egui::Rect::from_min_size(
                    ui.cursor().min,
                    egui::vec2(
                        rect.width() - 10.0,
                        (rect.bottom() - ui.cursor().top() - 5.0).max(220.0),
                    ),
                );
                let mut table_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .id_salt("pane-table")
                        .max_rect(table_rect)
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );
                table_ui.set_clip_rect(ui.clip_rect().intersect(table_rect));
                if let Some(activation) = self.table_in(&mut table_ui, pane, editing) {
                    match activation {
                        Activation::Folder(location) => {
                            self.navigate_in(pane, location, Intent::New)
                        }
                        Activation::File(path) => {
                            let context = ui.ctx().clone();
                            self.launcher
                                .request(path, Arc::new(move || context.request_repaint()));
                        }
                    }
                }
                ui.advance_cursor_after_rect(table_rect);
            });
        // A tab shortcut can close a pane during the scrolling closure.
        if self.panes.len() != count || self.panes[pane].id != id {
            ui.ctx().request_repaint();
            return true;
        }
        ui.painter().rect_stroke(
            rect.shrink(0.5),
            0.0,
            if pane == self.focused {
                egui::Stroke::new(2.0, egui::Color32::from_rgb(89, 215, 235))
            } else {
                ui.visuals().widgets.inactive.bg_stroke
            },
            egui::StrokeKind::Inside,
        );
        false
    }
}

#[cfg(test)]
impl Explorer {
    pub(super) fn active_tab_mut(&mut self) -> &mut Tab {
        self.panes[self.focused].active_tab_mut()
    }
}
