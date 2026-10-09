use super::*;

// A distinct payload type keeps future file drags out of tab transactions.
#[derive(Clone, Copy, Debug)]
pub(super) struct TabDrag {
    pub source: PaneId,
    pub tab: TabId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Edge {
    fn axis(self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::LeftRight,
            Self::Top | Self::Bottom => Axis::TopBottom,
        }
    }

    fn new_first(self) -> bool {
        matches!(self, Self::Left | Self::Top)
    }

    fn feedback(self, rect: egui::Rect) -> egui::Rect {
        let mut band = rect;
        match self {
            Self::Left => band.max.x = rect.left() + 24.0,
            Self::Right => band.min.x = rect.right() - 24.0,
            Self::Top => band.max.y = rect.top() + 24.0,
            Self::Bottom => band.min.y = rect.bottom() - 24.0,
        }
        band
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TabDrop {
    Strip { pane: PaneId, before: Option<TabId> },
    Edge { pane: PaneId, edge: Edge },
}

impl TabDrop {
    fn pane(self) -> PaneId {
        match self {
            Self::Strip { pane, .. } | Self::Edge { pane, .. } => pane,
        }
    }
}

pub(super) struct TabStripGeometry {
    pub pane: PaneId,
    pub rect: egui::Rect,
    pub tabs: Vec<(TabId, egui::Rect)>,
}

impl Explorer {
    fn tab_drop_preflight(&self, drag: TabDrag, drop: TabDrop) -> Result<(), &'static str> {
        let source = self
            .panes
            .iter()
            .find(|p| p.id == drag.source)
            .ok_or("Tab drag cancelled: source pane closed.")?;
        if !source.tabs.iter().any(|t| t.id == drag.tab) {
            return Err("Tab drag cancelled: dragged tab closed or moved.");
        }
        let pane = self
            .panes
            .iter()
            .find(|p| p.id == drop.pane())
            .ok_or("Tab drop refused: destination pane closed.")?;
        if !self.layout.contains(pane.id) || !self.layout.contains(source.id) {
            return Err("Tab drop refused: layout changed; try again.");
        }
        let rect = self
            .pane_rects
            .iter()
            .find(|(id, _)| *id == pane.id)
            .map(|(_, r)| *r)
            .ok_or("Tab drop refused: frame geometry is not ready.")?;
        match drop {
            TabDrop::Strip { before, .. } => {
                if before.is_some_and(|id| !pane.tabs.iter().any(|t| t.id == id)) {
                    return Err("Tab drop refused: insertion tab closed or moved.");
                }
                let strip = self
                    .tab_strips
                    .iter()
                    .find(|s| s.pane == pane.id)
                    .ok_or("Tab drop refused: tab strip is not visible.")?;
                if !strip.rect.is_positive() || !self.visible_workspace.intersects(strip.rect) {
                    return Err("Tab drop refused: tab strip is offscreen.");
                }
            }
            TabDrop::Edge { edge, .. } => {
                if !self.visible_workspace.contains_rect(rect) {
                    return Err(
                        "Cannot split an offscreen pane: enlarge the window or scroll it fully into view.",
                    );
                }
                if !Layout::split_fits(edge.axis(), rect.size()) {
                    return Err(
                        "Cannot split: enlarge this pane first (minimum 320 x 300 per pane).",
                    );
                }
            }
        }
        Ok(())
    }

    pub(super) fn drop_tab(&mut self, drag: TabDrag, drop: TabDrop) -> bool {
        if let Err(reason) = self.tab_drop_preflight(drag, drop) {
            self.layout_error = Some(reason.into());
            return false;
        }
        let source = self.panes.iter().position(|p| p.id == drag.source).unwrap();
        let index = self.panes[source]
            .tabs
            .iter()
            .position(|t| t.id == drag.tab)
            .unwrap();
        if let TabDrop::Strip { pane, before } = drop {
            if pane == drag.source {
                let p = &mut self.panes[source];
                let active = p.tabs[p.active].id;
                let insertion = before.map_or(p.tabs.len(), |id| {
                    p.tabs.iter().position(|t| t.id == id).unwrap()
                });
                let insertion = insertion - usize::from(index < insertion);
                let tab = p.tabs.remove(index);
                p.tabs.insert(insertion, tab);
                p.active = p.tabs.iter().position(|t| t.id == active).unwrap();
                p.reveal_active_tab = true;
                self.focus_pane(source);
                self.layout_error = None;
                return true;
            }
        }

        let source_active = self.panes[source].tabs[self.panes[source].active].id;
        let moved_was_active = source_active == drag.tab;
        let location = self.panes[source].tabs[index]
            .navigation
            .current
            .location
            .clone();
        let destination = match drop {
            TabDrop::Strip { pane, .. } => pane,
            TabDrop::Edge { pane, edge } => {
                let id = PaneId(self.next_pane_id);
                let rect = self.pane_rects.iter().find(|(p, _)| *p == pane).unwrap().1;
                // All fallible checks precede mutation. Source collapse happens only
                // after this split, and destinations are resolved by ID afterwards.
                assert!(
                    self.layout
                        .split_at(pane, id, edge.axis(), rect.size(), edge.new_first())
                );
                self.next_pane_id = self.next_pane_id.checked_add(1).expect("Pane ID exhausted");
                self.panes.push(Pane {
                    id,
                    tabs: Vec::new(),
                    active: 0,
                    reveal_active_tab: true,
                });
                id
            }
        };
        let mut tab = self.panes[source].tabs.remove(index);
        tab.location_editor_focused = false;
        tab.restore_scroll = true;
        tab.scroll_to_active = false;
        // An active moved context keeps its existing target/generation (including
        // pending navigation). Only deactivated/activated neighbors need new work.
        if self.panes[source].tabs.is_empty() {
            if matches!(drop, TabDrop::Edge { pane, .. } if pane == drag.source) {
                let replacement = self.fresh_tab(location);
                self.panes[source].tabs.push(replacement);
                self.panes[source].active = 0;
                self.refresh_tab_in(source);
            } else {
                assert!(self.layout.remove(drag.source));
                self.panes.remove(source);
                if source < self.focused {
                    self.focused -= 1;
                }
                self.focused = self.focused.min(self.panes.len() - 1);
            }
        } else {
            let p = &mut self.panes[source];
            p.active = if moved_was_active {
                index.min(p.tabs.len() - 1)
            } else {
                p.tabs.iter().position(|t| t.id == source_active).unwrap()
            };
            p.reveal_active_tab = true;
            if moved_was_active {
                self.refresh_tab_in(source);
            }
        }
        let dest = self.panes.iter().position(|p| p.id == destination).unwrap();
        let p = &mut self.panes[dest];
        if !p.tabs.is_empty() {
            p.cancel_tab(p.active, &self.loader, &self.sorter);
            p.tabs[p.active].deactivate();
        }
        let insertion = match drop {
            TabDrop::Strip { before, .. } => before.map_or(p.tabs.len(), |id| {
                p.tabs.iter().position(|t| t.id == id).unwrap()
            }),
            TabDrop::Edge { .. } => 0,
        };
        p.tabs.insert(insertion, tab);
        p.active = insertion;
        p.reveal_active_tab = true;
        if !moved_was_active {
            // Activation refresh must not erase the moved context's error message.
            let error = self.panes[dest].tabs[insertion].navigation.error.take();
            self.refresh_tab_in(dest);
            self.panes[dest].tabs[insertion].navigation.error = error;
        }
        self.focus_pane(dest);
        self.reveal_focused_pane = true;
        self.layout_error = None;
        // No consumer may reuse pre-transaction rectangles after collapse/split.
        self.pane_rects.clear();
        self.tab_strips.clear();
        true
    }

    fn tab_drop_at(&self, pointer: egui::Pos2) -> Option<(TabDrop, egui::Rect)> {
        if !self.visible_workspace.contains(pointer) {
            return None;
        }
        // Strips take priority over edges, including their intersection at the top.
        for strip in &self.tab_strips {
            if strip.rect.contains(pointer) {
                let before = strip.tabs.iter().find(|(_, r)| pointer.x < r.center().x);
                let x = before.map_or_else(
                    || {
                        strip
                            .tabs
                            .last()
                            .map_or(strip.rect.left(), |(_, r)| r.right())
                    },
                    |(_, r)| r.left(),
                );
                let line = egui::Rect::from_min_max(
                    egui::pos2(
                        x.clamp(strip.rect.left(), strip.rect.right()),
                        strip.rect.top(),
                    ),
                    egui::pos2(
                        x.clamp(strip.rect.left(), strip.rect.right()) + 3.0,
                        strip.rect.bottom(),
                    ),
                );
                return Some((
                    TabDrop::Strip {
                        pane: strip.pane,
                        before: before.map(|(id, _)| *id),
                    },
                    line,
                ));
            }
        }
        for (pane, rect) in &self.pane_rects {
            if !rect.contains(pointer) {
                continue;
            }
            let edge = [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom]
                .into_iter()
                .find(|edge| edge.feedback(*rect).contains(pointer))?;
            return Some((TabDrop::Edge { pane: *pane, edge }, edge.feedback(*rect)));
        }
        None
    }

    pub(super) fn finish_tab_drag(&mut self, ui: &mut egui::Ui, complete_geometry: bool) {
        let Some(drag) = egui::DragAndDrop::payload::<TabDrag>(ui.ctx()) else {
            self.tab_drag_hover = None;
            return;
        };
        if !self
            .panes
            .iter()
            .any(|p| p.id == drag.source && p.tabs.iter().any(|t| t.id == drag.tab))
        {
            egui::DragAndDrop::clear_payload(ui.ctx());
            self.layout_error = Some("Tab drag cancelled: source tab or pane closed.".into());
            self.tab_drag_hover = None;
            return;
        }
        // Closing the hovered destination must not retarget a release to the
        // expanded sibling that now happens to be under the stationary pointer.
        if self.tab_drag_hover.is_some_and(|drop| {
            !self.panes.iter().any(|p| {
                p.id == drop.pane()
                    && match drop {
                        TabDrop::Strip { before, .. } => {
                            before.is_none_or(|id| p.tabs.iter().any(|t| t.id == id))
                        }
                        TabDrop::Edge { .. } => true,
                    }
            })
        }) {
            egui::DragAndDrop::clear_payload(ui.ctx());
            self.tab_drag_hover = None;
            self.layout_error = Some(
                "Tab drag cancelled: destination pane or insertion tab closed or moved.".into(),
            );
            return;
        }
        let target = if complete_geometry {
            ui.input(|i| i.pointer.interact_pos())
                .and_then(|pos| self.tab_drop_at(pos))
        } else {
            None
        };
        self.tab_drag_hover = target.map(|(drop, _)| drop);
        if let Some((drop, feedback)) = target {
            let reason = self.tab_drop_preflight(*drag, drop).err();
            let color = if reason.is_some() {
                egui::Color32::from_rgb(255, 150, 100)
            } else {
                egui::Color32::from_rgb(89, 215, 235)
            };
            ui.painter()
                .rect_filled(feedback.intersect(self.visible_workspace), 0.0, color);
            egui::Tooltip::always_open(
                ui.ctx().clone(),
                ui.layer_id(),
                egui::Id::new("tab-drop-feedback"),
                feedback,
            )
            .at_pointer()
            .show(|ui| {
                ui.label(reason.unwrap_or(match drop {
                    TabDrop::Strip { .. } => "Move tab here (Escape cancels)",
                    TabDrop::Edge { .. } => "Split here and move tab (Escape cancels)",
                }));
            });
        }
        if ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary)) {
            egui::DragAndDrop::take_payload::<TabDrag>(ui.ctx());
            self.tab_drag_hover = None;
            if let Some((drop, _)) = target {
                if self.drop_tab(*drag, drop) {
                    workspace::surrender_focus(ui.ctx());
                }
            } else {
                self.layout_error = Some(
                    if complete_geometry {
                        "Tab drop cancelled: release on a visible tab strip or pane edge."
                    } else {
                        "Tab drop refused: layout or active context changed this frame; try again."
                    }
                    .into(),
                );
            }
            ui.ctx().request_repaint();
        } else if !ui.input(|i| i.pointer.primary_down()) {
            egui::DragAndDrop::clear_payload(ui.ctx());
            self.tab_drag_hover = None;
        }
    }
}
