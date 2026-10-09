use super::*;
use finder::{Action, Work};

impl Explorer {
    pub(super) fn open_finder(&mut self, ctx: &egui::Context) {
        self.finder_service.cancel();
        let pane = &self.panes[self.focused];
        let tab = &pane.tabs[pane.active];
        self.finder = Some(finder::Modal::new(finder::Target {
            pane: pane.id,
            tab: tab.id,
            base: tab.navigation.current.location.clone(),
            generation: tab.navigation.key().generation,
        }));
        egui::DragAndDrop::clear_payload(ctx);
        self.tab_drag_hover = None;
        workspace::surrender_focus(ctx);
        self.rank_finder();
    }
    pub(super) fn dismiss_finder(&mut self, ctx: &egui::Context) {
        if let Some(modal) = self.finder.take() {
            if let Some(pane) = self
                .panes
                .iter()
                .position(|pane| pane.id == modal.target.pane)
            {
                self.focus_pane(pane);
            }
        }
        self.finder_service.cancel();
        workspace::surrender_focus(ctx);
        ctx.request_repaint();
    }
    pub(super) fn finder_target(&self) -> Result<usize, &'static str> {
        let Some(modal) = &self.finder else {
            return Err("Folder finder is closed.");
        };
        let Some(index) = self
            .panes
            .iter()
            .position(|pane| pane.id == modal.target.pane)
        else {
            return Err("Captured pane is gone. Close and reopen the finder.");
        };
        let pane = &self.panes[index];
        let tab = &pane.tabs[pane.active];
        if tab.id != modal.target.tab
            || tab.navigation.current.location != modal.target.base
            || tab.navigation.key().generation != modal.target.generation
        {
            return Err(
                "Captured active tab moved, closed, or changed while the finder was open. Close and reopen the finder; its relative base has not changed.",
            );
        }
        Ok(index)
    }
    fn rank_finder(&mut self) {
        let Some(modal) = &mut self.finder else {
            return;
        };
        modal.catalogue_revision = self.locations.revision;
        if let Err(error) = modal.validate_input() {
            self.finder_service.cancel();
            modal.pending = false;
            modal.results.clear();
            modal.error = Some(error);
            return;
        }
        modal.revision = self.finder_service.request(Work::Rank {
            query: if modal.path_mode {
                String::new()
            } else {
                modal.query.clone()
            },
            session: self.locations.items(),
            children: modal.children.clone(),
        });
        modal.pending = true;
        modal.error = None;
    }
    pub(super) fn reconcile_finder_catalogue(&mut self) {
        if self
            .finder
            .as_ref()
            .is_some_and(|modal| modal.catalogue_revision != self.locations.revision)
        {
            self.rank_finder();
        }
    }
    pub(super) fn request_finder_action(&mut self, action: Option<Action>) {
        if let Err(error) = self.finder_target() {
            if let Some(modal) = &mut self.finder {
                modal.error = Some(error.into());
            }
            return;
        }
        let modal = self.finder.as_mut().unwrap();
        if let Err(error) = modal.validate_input() {
            modal.error = Some(error);
            return;
        }
        if modal.pending {
            modal.error = Some("Finder is still updating. Wait for results, or edit the query to cancel the request.".into());
            return;
        }
        let path = if modal.path_mode {
            modal.typed_path()
        } else {
            modal.selected_path().ok_or_else(|| "No matching location. Change the query, or enable Path entry to open a relative/absolute folder path.".into())
        };
        match path {
            Ok(path) => {
                modal.error = None;
                modal.revision = self.finder_service.request(match action {
                    Some(action) => Work::Probe(path, action),
                    None => Work::Expand(path),
                });
                modal.pending = true;
            }
            Err(error) => modal.error = Some(error),
        }
    }
    pub(super) fn poll_finder(&mut self) -> bool {
        let Some(response) = self.finder_service.take() else {
            return false;
        };
        if self.finder.is_none() {
            return false;
        }
        // Check identity/base again at completion, not just invocation.
        let action = self.finder.as_mut().unwrap().accept(response);
        if let Some((path, action)) = action {
            if let Err(error) = self.commit_finder(path, action) {
                self.finder.as_mut().unwrap().error = Some(error);
            }
        }
        true
    }
    pub(super) fn commit_finder(&mut self, path: PathBuf, action: Action) -> Result<(), String> {
        let pane = self.finder_target().map_err(str::to_owned)?;
        match action {
            Action::Navigate => {
                self.focus_pane(pane);
                self.navigate_in(pane, path, Intent::New);
            }
            Action::NewTab => {
                self.focus_pane(pane);
                self.insert_tab(path);
            }
            Action::Split => {
                let source = self.panes[pane].id;
                let rect = self
                    .pane_rects
                    .iter()
                    .find(|(id, _)| *id == source)
                    .map(|(_, rect)| *rect)
                    .ok_or("Layout is not ready; try again.")?;
                if !self.visible_workspace.contains_rect(rect)
                    || !Layout::split_fits(Axis::LeftRight, rect.size())
                {
                    return Err("Cannot open a left/right split: source must be fully visible with room for two 320 x 300 panes plus a six-point divider. Enlarge the window and retry.".into());
                }
                let id = PaneId(self.next_pane_id);
                // All preflight checks precede any layout/tab/focus mutation.
                if !self.layout.split(source, id, Axis::LeftRight, rect.size()) {
                    return Err("Captured layout changed; close and reopen the finder.".into());
                }
                self.next_pane_id = self.next_pane_id.checked_add(1).expect("Pane ID exhausted");
                let tab = self.fresh_tab(path);
                self.panes.push(Pane {
                    id,
                    tabs: vec![tab],
                    active: 0,
                    reveal_active_tab: true,
                });
                self.focus_pane(self.panes.len() - 1);
                self.refresh_tab_in(self.focused);
                self.layout_error = None;
            }
        }
        self.finder = None;
        self.finder_service.cancel();
        self.reveal_focused_pane = true;
        Ok(())
    }
    pub(super) fn finder_workspace(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let opening = self.finder.is_none()
            && ctx.input_mut(|input| consume_exact_key(input, egui::Modifiers::CTRL, egui::Key::O));
        let modal_frame = opening || self.finder.is_some();
        if modal_frame {
            // Disabled widgets alone do not gate our workspace keyboard helpers.
            // Hide all events from the background, including pointer/drag/F6 and
            // clipboard/IME, then restore them exclusively for the modal.
            let events = ctx.input_mut(|input| std::mem::take(&mut input.events));
            ui.add_enabled_ui(false, |ui| self.workspace(ui));
            ctx.input_mut(|input| input.events = events);
            if opening {
                // The disabled workspace has computed current-frame geometry.
                // Focus the pressed pane before capture, without running its commands.
                self.focus_pointer_pane(ui);
                self.open_finder(&ctx);
            }
            self.finder_ui(&ctx);
            // Process query edits/Escape before accepting a ready probe in the
            // same frame. Otherwise a late action could beat its cancellation.
            if self.poll_finder() {
                ctx.request_repaint();
            }
            if self.finder.is_none() {
                workspace::surrender_focus(&ctx);
            }
        } else {
            self.workspace(ui);
        }
    }
    fn finder_ui(&mut self, ctx: &egui::Context) {
        let mut dismiss = false;
        let mut changed = false;
        let mut command = None;
        let mut bookmark = None;
        let mut forget = None;
        let mut known = false;
        // Consume finder keys before TextEdit (Enter/Tab must not surrender focus
        // or move to background controls). Other editing/clipboard keys stay live.
        let keys = ctx.input_mut(|input| {
            let escape = consume_exact_key(input, egui::Modifiers::NONE, egui::Key::Escape);
            let down = consume_exact_key(input, egui::Modifiers::NONE, egui::Key::ArrowDown);
            let up = consume_exact_key(input, egui::Modifiers::NONE, egui::Key::ArrowUp);
            let expand = consume_exact_key(input, egui::Modifiers::NONE, egui::Key::Tab);
            let action = if consume_exact_key(input, egui::Modifiers::CTRL, egui::Key::Enter) {
                Some(Action::Split)
            } else if consume_exact_key(input, egui::Modifiers::SHIFT, egui::Key::Enter) {
                Some(Action::NewTab)
            } else if consume_exact_key(input, egui::Modifiers::NONE, egui::Key::Enter) {
                Some(Action::Navigate)
            } else {
                None
            };
            (escape, down, up, expand, action)
        });
        let modal = self.finder.as_mut().unwrap();
        if keys.1 {
            modal.move_selection(true);
        }
        if keys.2 {
            modal.move_selection(false);
        }
        dismiss |= keys.0;
        if keys.3 {
            // egui schedules Tab focus movement before widgets consume events.
            ctx.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
            modal.focus_input = true;
            command = Some(None);
        }
        if let Some(action) = keys.4 {
            command = Some(Some(action));
        }
        egui::Modal::new(egui::Id::new("folder-finder-modal")).show(ctx, |ui| {
            ui.set_width((ctx.content_rect().width() - 48.0).clamp(240.0, 700.0));
            egui::ScrollArea::vertical().id_salt("finder-body").max_height((ctx.content_rect().height() - 48.0).max(160.0)).show(ui, |ui| {
            ui.heading("Folder finder");
            ui.label(format!("Captured base: {}", native::display(modal.target.base.as_os_str())));
            ui.label("Known locations and explicit children only. Bookmarks/recents: session only.");
            let input = ui.add(egui::TextEdit::singleline(&mut modal.query).id(egui::Id::new("finder-input")).lock_focus(true).desired_width(f32::INFINITY).hint_text("Fuzzy terms, or enable Path entry"));
            if modal.focus_input { input.request_focus(); modal.focus_input = false; }
            changed |= input.changed();
            ui.horizontal(|ui| {
                changed |= ui.checkbox(&mut modal.path_mode, "Path entry").on_hover_text("Enter an absolute folder path or a path relative to the captured base. Enter opens it; Tab expands it without browsing navigation.").changed();
                if ui.button("Known locations").clicked() { modal.children = None; modal.query.clear(); modal.path_mode = false; changed = true; known = true; }
                if ui.button("Close (Esc)").clicked() { dismiss = true; }
            });
            if modal.pending { ui.horizontal(|ui| { ui.spinner(); ui.label("Checking / updating (OS calls may finish before cancellation)"); }); }
            if let Some(error) = &modal.error { ui.colored_label(egui::Color32::from_rgb(255,180,130), error); }
            for notice in &modal.notices { ui.label(notice); }
            if modal.results.is_empty() && !modal.pending && !modal.path_mode {
                ui.label(if modal.query.trim().is_empty() { "No known locations in this view. Use Path entry or Known locations." } else { "No matching locations. Edit the query or use Path entry." });
            }
            let mut results_scroll = egui::ScrollArea::vertical().id_salt("finder-results").max_height((ctx.content_rect().height() - 310.0).clamp(60.0, 320.0));
            if keys.1 || keys.2 { results_scroll = results_scroll.vertical_scroll_offset(modal.selected as f32 * (48.0 + ui.spacing().item_spacing.y)); }
            results_scroll.show_rows(ui, 48.0, modal.results.len(), |ui, range| {
                for index in range {
                    let item = &modal.results[index];
                    let mut job = emphasis(&format!("{}: ", item.location.category), &item.location.name, &item.name_marks, ui);
                    job.append("\n", 0.0, egui::TextFormat::default());
                    let path_job = emphasis("", &item.path_text, &item.path_marks, ui);
                    // Append the path sections using their original byte ranges.
                    for section in path_job.sections { job.append(&path_job.text[section.byte_range.start.0..section.byte_range.end.0], 0.0, section.format); }
                    job.wrap.max_rows = 2;
                    let response = ui.add_enabled(!modal.pending && !modal.path_mode, egui::Button::new(job).selected(index == modal.selected).min_size(egui::vec2(ui.available_width(), 48.0))).on_hover_text(&item.path_text);
                    if response.clicked() { modal.selected = index; modal.focus_input = true; }
                    if (keys.1 || keys.2) && index == modal.selected { response.scroll_to_me(None); }
                }
            });
            ui.horizontal_wrapped(|ui| {
                if ui.add_enabled(!modal.pending, egui::Button::new("Open (Enter)")).clicked() { command = Some(Some(Action::Navigate)); }
                if ui.add_enabled(!modal.pending, egui::Button::new("New tab (Shift+Enter)")).clicked() { command = Some(Some(Action::NewTab)); }
                if ui.add_enabled(!modal.pending, egui::Button::new("Split (Ctrl+Enter)")).clicked() { command = Some(Some(Action::Split)); }
                if ui.add_enabled(!modal.pending, egui::Button::new("Children (Tab)")).clicked() { command = Some(None); }
            });
            if let Some(path) = modal.selected_path().filter(|_| !modal.pending && !modal.path_mode) {
                ui.horizontal(|ui| {
                    let marked = self.locations.bookmarks.iter().any(|item| item.path == path);
                    if ui.button(if marked { "Unbookmark (session)" } else { "Bookmark (session)" }).clicked() { bookmark = Some(path.clone()); }
                    if ui.add_enabled(self.locations.recents.iter().any(|item| item.path == path), egui::Button::new("Forget recent (session)")).clicked() { forget = Some(path); }
                });
            }
            });
        });
        if dismiss {
            self.dismiss_finder(ctx);
            return;
        }
        if changed {
            self.rank_finder();
            if known {
                self.finder.as_mut().unwrap().notices.clear();
            }
        }
        if let Some(path) = bookmark {
            if let Err(error) = self.locations.toggle_bookmark(path) {
                self.finder.as_mut().unwrap().error = Some(error.into());
            } else {
                self.rank_finder();
            }
        }
        if let Some(path) = forget {
            self.locations.forget(&path);
            self.rank_finder();
        }
        // Text arriving in the same frame as Enter cancels that old selection.
        if !changed {
            if let Some(action) = command {
                self.request_finder_action(action);
            }
        }
        if self.finder.is_none() {
            workspace::surrender_focus(ctx);
            ctx.request_repaint();
        }
    }
}
fn emphasis(prefix: &str, text: &str, marks: &[usize], ui: &egui::Ui) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let normal = egui::TextFormat {
        font_id: egui::FontId::proportional(14.0),
        color: ui.visuals().text_color(),
        ..Default::default()
    };
    job.append(prefix, 0.0, normal.clone());
    for (index, character) in text.chars().enumerate() {
        let mut format = normal.clone();
        if marks.binary_search(&index).is_ok() {
            format.color = egui::Color32::from_rgb(89, 215, 235);
            format.underline = egui::Stroke::new(1.0, format.color);
        }
        job.append(&character.to_string(), 0.0, format);
    }
    job
}
