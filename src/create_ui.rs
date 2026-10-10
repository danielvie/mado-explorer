use super::*;
use create_adapter::Target;
use mado_operation_drawer::{Kind, Outcome, State};

fn intentional_click(response: &egui::Response, allow: bool) -> bool {
    allow
        && response.clicked()
        && response
            .interact_pointer_pos()
            .is_none_or(|p| response.interact_rect.contains(p))
}

impl Explorer {
    pub(super) fn open_create(&mut self, ctx: &egui::Context, kind: Kind) {
        let pane = &self.panes[self.focused];
        let tab = &pane.tabs[pane.active];
        self.create.open(
            Target {
                pane: pane.id,
                tab: tab.id,
                path: tab.navigation.current.location.clone(),
                generation: tab.navigation.current.generation,
            },
            kind,
        );
        if self.create.editor.is_some() {
            workspace::surrender_focus(ctx);
            egui::DragAndDrop::clear_payload(ctx);
            self.tab_drag_hover = None;
        }
    }
    pub(super) fn create_logic(&mut self) {
        let completed = self.create.poll();
        for id in completed {
            // Conservative all-open-tab refresh covers alternate spellings/aliases without UI I/O.
            for pane in &self.panes {
                for tab in &pane.tabs {
                    if !self.create.dirty.contains(&tab.id) {
                        self.create.dirty.push(tab.id);
                    }
                }
            }
            if let Some(p) = self
                .create
                .payloads
                .iter_mut()
                .find(|p| p.request.id() == id)
            {
                let origin = self
                    .panes
                    .iter()
                    .flat_map(|pane| &pane.tabs)
                    .find(|t| t.id == p.target.tab);
                if origin.is_some_and(|t| {
                    t.navigation.current.location == p.target.path
                        && t.navigation.current.generation == p.origin_generation
                }) {
                    p.selection_generation = Some(0);
                }
            }
        }
        self.reconcile_create();
    }
    pub(super) fn reconcile_create(&mut self) {
        for payload in &mut self.create.payloads {
            if payload.selection_generation == Some(0)
                && !self.panes.iter().flat_map(|p| &p.tabs).any(|t| {
                    t.id == payload.target.tab
                        && t.navigation.current.location == payload.target.path
                })
            {
                payload.selected = true;
                payload.selection_generation = None;
            }
        }
        self.create.dirty.retain(|id| {
            self.panes
                .iter()
                .any(|p| p.tabs.iter().any(|t| t.id == *id))
        });
        for pane in 0..self.panes.len() {
            let p = &self.panes[pane];
            let tab = &p.tabs[p.active];
            if tab.navigation.pending_location().is_some()
                || matches!(tab.navigation.current.status, Status::Loading)
            {
                continue;
            }
            if let Some(index) = self.create.dirty.iter().position(|id| *id == tab.id) {
                self.create.dirty.remove(index);
                let id = tab.id;
                let path = tab.navigation.current.location.clone();
                let active = self.panes[pane].active;
                let error = self.panes[pane].tabs[active].navigation.error.take();
                self.refresh_tab_in(pane);
                let tab = &mut self.panes[pane].tabs[active];
                tab.navigation.error = error;
                for payload in &mut self.create.payloads {
                    if payload.target.tab == id
                        && payload.target.path == path
                        && payload.selection_generation == Some(0)
                    {
                        payload.selection_generation = Some(tab.navigation.current.generation);
                    }
                }
            }
        }
        for payload in &mut self.create.payloads {
            let Some(generation) = payload.selection_generation.filter(|g| *g != 0) else {
                continue;
            };
            if payload.selected {
                continue;
            }
            // Stable TabId, captured location and fresh refresh generation, never focused pane/row number.
            let Some(tab) = self
                .panes
                .iter_mut()
                .flat_map(|p| &mut p.tabs)
                .find(|t| t.id == payload.target.tab)
            else {
                payload.selected = true;
                continue;
            };
            if tab.navigation.current.location != payload.target.path
                || tab.navigation.current.generation != generation
            {
                payload.selected = true;
                continue;
            }
            if tab.navigation.pending_location().is_some() || !tab.navigation.rows_selectable() {
                continue;
            }
            let path = payload.request.destination().unwrap();
            if let Some(index) = tab
                .navigation
                .current
                .entries
                .iter()
                .position(|e| e.path == path)
            {
                tab.navigation.selection.click(
                    &tab.navigation.current.entries,
                    index,
                    false,
                    false,
                );
                tab.scroll_to_active = true;
            } else {
                payload.filtered = tab
                    .navigation
                    .current
                    .raw_entries
                    .iter()
                    .any(|e| e.path == path);
            }
            payload.selected = true;
        }
    }
    pub(super) fn create_workspace(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let transient =
            self.create.editor.is_some() || self.create.close_prompt || self.create.waiting_close;
        let mut opening = None;
        let mut control_focus = false;
        let mut control_action = false;
        // Window-owned controls and retained drawer survive origin tab/pane closure.
        egui::ScrollArea::vertical().id_salt("create-controls").max_height(220.0).show(ui, |ui| {
            ui.add_enabled_ui(!transient && self.finder.is_none(), |ui| {
                ui.menu_button("New", |ui| {
                    if ui.button("New folder").clicked() { opening = Some(Kind::CreateFolder); ui.close(); }
                    if ui.button("New empty file").clicked() { opening = Some(Kind::CreateFile); ui.close(); }
                });
                let response = self.create.operations.show(ui);
                control_focus |= response.toggle.has_focus() || response.toggle.lost_focus() || response.controls.iter().any(|c| c.response.has_focus() || c.response.lost_focus());
                control_action |= response.toggle.clicked() || response.controls.iter().any(|c| c.response.clicked());
                let mut retry = None;
                for payload in &mut self.create.payloads {
                    if payload.filtered { ui.label("Created entry is hidden by the current filter/visibility. Filter retained."); }
                    if self.create.operations.history().iter().any(|r| r.request().id() == payload.request.id() && r.state() == State::Stopped(Outcome::Failed) && !r.issues().is_empty()) {
                        let seed = egui::Id::new(("create-retry", payload.slot));
                        let button_id = egui::Id::new(seed.value().wrapping_add(1));
                        let allow = payload.input.prepare(&ctx, button_id);
                        let response = ui.scope_builder(egui::UiBuilder::new().id(seed), |ui| ui.button(format!("Retry failed creation #{}", payload.request.id().sequence))).inner;
                        debug_assert_eq!(response.id, button_id);
                        control_focus |= response.has_focus() || response.lost_focus() || response.clicked();
                        if allow && response.clicked() && response.interact_pointer_pos().is_none_or(|p| response.rect.contains(p)) { retry = Some(payload.request.id()); }
                    }
                }
                if let Some(id) = retry { self.create.retry(id); control_action = true; workspace::surrender_focus(&ctx); }
            });
            if let Some(notice) = &self.create.notice { ui.add(egui::Label::new(notice).wrap()); }
            if let Some(report) = &self.create.refused_report {
                if let mado_operation_drawer::Update::Stopped { issues, .. } = &report.update {
                    for issue in issues {
                        ui.add(egui::Label::new(native::display(issue.path.as_os_str())).truncate()).on_hover_text(native::display(issue.path.as_os_str()));
                        ui.add(egui::Label::new(&issue.error).wrap());
                    }
                }
            }
        });
        if let Some(kind) = opening {
            self.open_create(&ctx, kind);
        }
        let modal = transient || self.create.editor.is_some();
        if modal || control_action || opening.is_some() {
            self.create.ui_owned_frame = Some(ctx.cumulative_frame_nr());
        }
        if modal {
            let events = ctx.input_mut(|i| std::mem::take(&mut i.events));
            ui.add_enabled_ui(false, |ui| self.workspace(ui));
            ctx.input_mut(|i| i.events = events);
            if self.create.close_prompt || self.create.waiting_close {
                self.create_close_ui(&ctx);
            } else {
                self.create_editor_ui(&ctx);
            }
        } else {
            // Drawer/menu activation owns this frame's Enter/Space/Escape before browsing.
            let blocked = self.create.ui_owned_frame == Some(ctx.cumulative_frame_nr())
                || egui::Popup::is_any_open(&ctx);
            let events = (blocked || control_focus).then(|| {
                ctx.input_mut(|i| {
                    let events = i.events.clone();
                    if blocked {
                        i.events.clear();
                    } else {
                        i.events.retain(|e| {
                            !matches!(
                                e,
                                egui::Event::Key {
                                    key: egui::Key::Enter | egui::Key::Space,
                                    ..
                                }
                            )
                        });
                    }
                    events
                })
            });
            self.finder_workspace(ui);
            if let Some(events) = events {
                ctx.input_mut(|i| i.events = events);
            }
        }
    }
    pub(super) fn create_editor_ui(&mut self, ctx: &egui::Context) {
        let mut dismiss = false;
        let mut commit = false;
        let mut retry = false;
        let editor = self.create.editor.as_mut().unwrap();
        let input_id = egui::Id::new("create-name");
        let allow = editor.input.prepare(ctx, input_id);
        let focused = ctx.memory(|m| m.focused());
        let confirm_id = egui::Id::new(egui::Id::new("create-confirm").value().wrapping_add(1));
        let retry_id = egui::Id::new(
            egui::Id::new("create-preparation-retry")
                .value()
                .wrapping_add(1),
        );
        let cancel_id = egui::Id::new(egui::Id::new("create-dismiss").value().wrapping_add(1));
        let (escape, enter) = ctx.input_mut(|i| {
            (
                consume_exact_key(i, egui::Modifiers::NONE, egui::Key::Escape),
                consume_exact_key(i, egui::Modifiers::NONE, egui::Key::Enter),
            )
        });
        dismiss |= escape;
        egui::Modal::new(egui::Id::new("create-editor")).show(ctx, |ui| {
            ui.set_width((ctx.content_rect().width() - 48.0).clamp(240.0, 640.0));
            egui::ScrollArea::vertical().id_salt("create-editor-body").max_height((ctx.content_rect().height() - 48.0).max(160.0)).show(ui, |ui| {
            ui.heading(if editor.kind == Kind::CreateFolder { "New folder" } else { "New empty file" });
            ui.add(egui::Label::new(format!("Captured destination: {}", native::display(editor.target.path.as_os_str()))).truncate()).on_hover_text(native::display(editor.target.path.as_os_str()));
            ui.label("Destination is fixed. Existing entries are never overwritten. Local NTFS backend only.");
            ui.label(format!("Origin pane {}, tab {}", editor.target.pane.0, editor.target.tab.0));
            let input = ui.add(egui::TextEdit::singleline(&mut editor.name).id(input_id).char_limit(256).lock_focus(true).desired_width(f32::INFINITY).hint_text("Name"));
            if editor.focus { input.request_focus(); editor.focus = false; }
            // Route Enter to its focused editor role before TextEdit can surrender focus.
            commit |= allow && enter && (input.has_focus() || input.lost_focus() || focused == Some(confirm_id));
            retry |= allow && enter && focused == Some(retry_id);
            dismiss |= allow && enter && focused == Some(cancel_id);
            if editor.preparing { ui.horizontal(|ui| { ui.spinner(); ui.label("Preparing captured destination on worker..."); }); }
            if let Some(error) = &editor.error { ui.add(egui::Label::new(error).wrap()); }
            ui.horizontal(|ui| {
                let ready = !editor.preparing && editor.identity.is_some();
                let create = ui.scope_builder(egui::UiBuilder::new().id(egui::Id::new("create-confirm")), |ui| ui.add_enabled(ready, egui::Button::new("Create (Enter)"))).inner;
                if intentional_click(&create, allow) { commit = true; }
                if !editor.preparing && editor.identity.is_none() {
                    let response = ui.scope_builder(egui::UiBuilder::new().id(egui::Id::new("create-preparation-retry")), |ui| ui.button("Retry preparation")).inner;
                    if intentional_click(&response, allow) { retry = true; }
                }
                let cancel = ui.scope_builder(egui::UiBuilder::new().id(egui::Id::new("create-dismiss")), |ui| ui.button("Cancel (Esc)")).inner;
                if intentional_click(&cancel, allow) { dismiss = true; }
            });
            });
        });
        if dismiss {
            self.create.dismiss();
            workspace::surrender_focus(ctx);
        } else if retry {
            self.create.prepare();
        } else if commit && self.create.commit() {
            workspace::surrender_focus(ctx);
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
    pub(super) fn create_close_ui(&mut self, ctx: &egui::Context) {
        let allow = self
            .create
            .close_input
            .prepare(ctx, egui::Id::new("create-close"));
        egui::Modal::new(egui::Id::new("create-close")).show(ctx, |ui| {
            ui.heading("Creation worker and application close");
            if self.create.waiting_close {
                ui.label("Cancelling and waiting for actual worker cleanup. An OS call may finish; completed entries remain.");
                ui.spinner();
            } else {
                ui.label("Creation is pending. Keep running, or cancel and wait before saving settings and closing.");
                if ui.button("Keep running").clicked() && allow { self.create.close_prompt = false; self.create.close_input = Default::default(); if let Some(editor) = &mut self.create.editor { editor.focus = true; } workspace::surrender_focus(ctx); }
                if ui.button("Cancel and wait").clicked() && allow { self.create.cancel_and_stop(); workspace::surrender_focus(ctx); }
            }
        });
    }
    // Called before settings-close. Cleanup acknowledgement, not cancellation request, authorizes continuation.
    pub(super) fn create_close_requested(&mut self, requested: bool) -> bool {
        if self.create.waiting_close {
            if self.create.cleanup_acknowledged && !self.create.needs_tick() {
                self.create.waiting_close = false;
                return true;
            }
            return false;
        }
        if self.create.close_prompt {
            return false;
        }
        if !requested {
            return false;
        }
        if self.create.needs_tick() {
            if self.create.busy() {
                self.create.close_prompt = true;
            } else {
                self.create.cancel_and_stop();
            }
            return false;
        }
        true
    }
}
