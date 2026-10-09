//! Supplied PDF metadata and page requests only. No file access or renderer calls.
use std::path::{Path, PathBuf};

use egui::{Id, Key, Modifiers, PointerButton, Ui};

/// Caller assigns a strictly increasing activation revision per pane, including
/// reopening the same path. Paths remain native data, never GUI identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentKey {
    path: PathBuf,
    revision: u64,
}

impl DocumentKey {
    pub fn new(path: PathBuf, revision: u64) -> Result<Self, Refusal> {
        if !path.is_absolute() {
            return Err(Refusal::FullPathRequired);
        }
        Ok(Self { path, revision })
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentState {
    Loading,
    Available { page_count: u64 },
    MissingRenderer,
    Malformed,
    PasswordRequired,
    Failure(String),
}

/// Capture this target when invoking a controller action. Changed metadata or
/// a newer page request refuses delayed actions even within the same document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageTarget {
    pane: Id,
    document: DocumentKey,
    metadata_revision: u64,
    page_revision: u64,
}
impl PageTarget {
    pub fn pane(&self) -> Id {
        self.pane
    }
    pub fn document(&self) -> &DocumentKey {
        &self.document
    }
    pub fn metadata_revision(&self) -> u64 {
        self.metadata_revision
    }
    pub fn page_revision(&self) -> u64 {
        self.page_revision
    }
}

/// Immutable intent, not a claim that a page was rendered or displayed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaggedPageRequest {
    target: PageTarget,
    sequence: u64,
    page_index: u32,
}
impl TaggedPageRequest {
    pub fn target(&self) -> &PageTarget {
        &self.target
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn page_index(&self) -> u32 {
        self.page_index
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageAction {
    Previous,
    Next,
    /// UI page number, starting at one. Not a renderer index.
    Direct(u64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderResult {
    /// Caller reports this only after the matching frame has been installed.
    Displayed,
    Failure(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    FullPathRequired,
    StaleDocument,
    StaleMetadata,
    StaleAction,
    MetadataUnavailable,
    InvalidPage,
    RequestSequenceExhausted,
    StaleRender,
}

#[derive(Default)]
pub struct ControlsOutput {
    pub request: Option<TaggedPageRequest>,
    /// Own controls only. Host suspends browsing shortcuts while this is true.
    pub owns_keyboard: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PointerEpoch {
    Blocked,
    Ready,
    Pressed,
}

/// One small controller per pane. `pane_salt` must be stable and unique within
/// the Context, including controller recreation. Never use a document key as salt.
/// Host owns selection, renderer scheduling, pixel density, budgets and textures.
pub struct PdfControls {
    pane: Id,
    id: Id,
    document: Option<DocumentKey>,
    latest_document_revision: Option<u64>,
    metadata_revision: Option<u64>,
    state: DocumentState,
    requested: u32,
    confirmed: Option<u32>,
    pending: Option<TaggedPageRequest>,
    sequence: u64,
    needs_request: bool,
    editor: String,
    editor_dirty: bool,
    reset_editor_state: bool,
    editor_error: Option<String>,
    render_error: Option<String>,
    first_frame: Option<u64>,
    processed_frame: Option<u64>,
    pointer: PointerEpoch,
    pressed_target: Option<PageTarget>,
    blocked_enter: bool,
    blocked_space: bool,
    #[cfg(test)]
    controls: Option<([egui::Response; 3], egui::Response, [egui::Response; 2])>,
}

impl PdfControls {
    pub fn new(pane_salt: Id) -> Self {
        Self {
            pane: pane_salt,
            id: pane_salt.with("mado-pdf-controls"),
            document: None,
            latest_document_revision: None,
            metadata_revision: None,
            state: DocumentState::Loading,
            requested: 0,
            confirmed: None,
            pending: None,
            sequence: 0,
            needs_request: false,
            editor: "1".into(),
            editor_dirty: false,
            reset_editor_state: true,
            editor_error: None,
            render_error: None,
            first_frame: None,
            processed_frame: None,
            pointer: PointerEpoch::Blocked,
            pressed_target: None,
            blocked_enter: true,
            blocked_space: true,
            #[cfg(test)]
            controls: None,
        }
    }

    /// Selection data is supplied by the host, not discovered from a path.
    /// None invalidates a selection but does not permit old revisions to reopen it.
    pub fn select(&mut self, document: Option<DocumentKey>) -> Result<(), Refusal> {
        if self.document == document {
            return Ok(());
        }
        if let Some(key) = &document {
            if self
                .latest_document_revision
                .is_some_and(|rev| key.revision <= rev)
            {
                return Err(Refusal::StaleDocument);
            }
            self.latest_document_revision = Some(key.revision);
        }
        self.document = document;
        self.metadata_revision = None;
        self.state = DocumentState::Loading;
        self.requested = 0;
        self.confirmed = None;
        self.pending = None;
        self.needs_request = false;
        self.editor = "1".into();
        self.editor_dirty = false;
        self.reset_editor_state = true;
        self.editor_error = None;
        self.render_error = None;
        self.disarm();
        Ok(())
    }

    /// Revision must increase for each supplied metadata update of this key.
    /// Zero and counts outside the backend's u32 range become explicit failures.
    pub fn supply_metadata(
        &mut self,
        document: &DocumentKey,
        revision: u64,
        state: DocumentState,
    ) -> Result<(), Refusal> {
        if self.document.as_ref() != Some(document) {
            return Err(Refusal::StaleDocument);
        }
        if self.metadata_revision.is_some_and(|rev| revision <= rev) {
            return Err(Refusal::StaleMetadata);
        }
        self.metadata_revision = Some(revision);
        self.state = match state {
            DocumentState::Available { page_count }
                if page_count == 0 || page_count > u32::MAX as u64 =>
            {
                DocumentState::Failure("Invalid PDF page count supplied".into())
            }
            other => other,
        };
        self.confirmed = None;
        self.pending = None;
        self.render_error = None;
        if let Some(count) = self.page_count() {
            if self.requested >= count {
                self.requested = 0;
                // Metadata changes the committed request, not an unsubmitted draft.
                if !self.editor_dirty {
                    self.editor = "1".into();
                }
            }
            if self.editor_dirty {
                self.editor_error = (!self
                    .editor_page()
                    .is_some_and(|page| (1..=u64::from(count)).contains(&page)))
                .then(|| format!("Enter a whole page number from 1 to {count}"));
            }
            self.needs_request = true;
        } else {
            self.needs_request = false;
        }
        self.disarm();
        Ok(())
    }

    pub fn document(&self) -> Option<&DocumentKey> {
        self.document.as_ref()
    }
    pub fn state(&self) -> &DocumentState {
        &self.state
    }
    pub fn page_count(&self) -> Option<u32> {
        match self.state {
            DocumentState::Available { page_count } => {
                u32::try_from(page_count).ok().filter(|c| *c > 0)
            }
            _ => None,
        }
    }
    pub fn requested_page(&self) -> u32 {
        self.requested
    }
    pub fn confirmed_page(&self) -> Option<u32> {
        self.confirmed
    }
    pub fn pending(&self) -> Option<&TaggedPageRequest> {
        self.pending.as_ref()
    }
    pub fn target(&self) -> Option<PageTarget> {
        Some(PageTarget {
            pane: self.pane,
            document: self.document.clone()?,
            metadata_revision: self.metadata_revision?,
            page_revision: self.sequence,
        })
    }
    pub fn editor(&self) -> &str {
        &self.editor
    }
    pub fn editor_error(&self) -> Option<&str> {
        self.editor_error.as_deref()
    }

    /// Returns a checked zero-based request against the captured target.
    /// Direct callers own dispatch; show does not return this request again.
    pub fn request_page(
        &mut self,
        target: &PageTarget,
        action: PageAction,
    ) -> Result<TaggedPageRequest, Refusal> {
        if self.target().as_ref() != Some(target) {
            return Err(Refusal::StaleAction);
        }
        let count = self.page_count().ok_or(Refusal::MetadataUnavailable)?;
        let index = match action {
            PageAction::Previous => self.requested.checked_sub(1),
            PageAction::Next => self.requested.checked_add(1).filter(|index| *index < count),
            PageAction::Direct(page) => page
                .checked_sub(1)
                .and_then(|index| u32::try_from(index).ok())
                .filter(|index| *index < count),
        }
        .ok_or(Refusal::InvalidPage)?;
        self.make_request(target.clone(), index, true)
    }

    /// Only the latest exact request can acknowledge display/failure. Reports for
    /// an older page, metadata revision or activation cannot alter current state.
    pub fn supply_render(
        &mut self,
        request: &TaggedPageRequest,
        result: RenderResult,
    ) -> Result<(), Refusal> {
        if self.pending.as_ref() != Some(request) || self.target().as_ref() != Some(&request.target)
        {
            return Err(Refusal::StaleRender);
        }
        match result {
            RenderResult::Displayed => {
                self.confirmed = Some(request.page_index);
                self.render_error = None;
            }
            RenderResult::Failure(reason) => self.render_error = Some(reason),
        }
        self.pending = None;
        Ok(())
    }

    fn make_request(
        &mut self,
        mut target: PageTarget,
        index: u32,
        commit_editor: bool,
    ) -> Result<TaggedPageRequest, Refusal> {
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or(Refusal::RequestSequenceExhausted)?;
        target.page_revision = self.sequence;
        let request = TaggedPageRequest {
            target,
            sequence: self.sequence,
            page_index: index,
        };
        self.requested = index;
        if commit_editor || !self.editor_dirty {
            self.editor = (u64::from(index) + 1).to_string();
            self.editor_dirty = false;
            self.editor_error = None;
        }
        self.render_error = None;
        self.pending = Some(request.clone());
        self.needs_request = false;
        Ok(request)
    }

    fn editor_page(&self) -> Option<u64> {
        (!self.editor.is_empty() && self.editor.bytes().all(|b| b.is_ascii_digit()))
            .then(|| self.editor.parse().ok())
            .flatten()
    }

    fn disarm(&mut self) {
        self.first_frame = None;
        self.pointer = PointerEpoch::Blocked;
        self.pressed_target = None;
        self.blocked_enter = true;
        self.blocked_space = true;
    }

    fn control_ids(&self) -> [Id; 4] {
        // egui 0.36.2 starts an explicitly identified child Ui's auto IDs
        // at Id::new(child_id.value() + 1). Tests assert the actual responses.
        let [previous, next, go] = ["previous", "next", "go"]
            .map(|salt| Id::new(self.id.with(salt).value().wrapping_add(1)));
        [previous, next, go, self.id.with("page-editor")]
    }

    fn surrender_owned_focus(&self, ui: &Ui, include_editor: bool) {
        ui.memory_mut(|memory| {
            for id in self
                .control_ids()
                .into_iter()
                .take(if include_editor { 4 } else { 3 })
            {
                memory.surrender_focus(id);
            }
        });
    }

    /// Draw inside the host pane. No window-level key/pointer clearing or modal.
    /// All retained widget and hover identities depend only on the caller's salt.
    pub fn show(&mut self, ui: &mut Ui) -> ControlsOutput {
        let frame = ui.ctx().cumulative_frame_nr();
        let once = self.processed_frame != Some(frame);
        self.processed_frame = Some(frame);
        let fresh = *self.first_frame.get_or_insert(frame) == frame;
        if !fresh {
            self.reset_editor_state = false;
        }
        let mut output = ControlsOutput::default();
        let allowed = ui.is_enabled()
            && ui.is_visible()
            && ui.memory(|m| m.allows_interaction(ui.layer_id()));
        let focused = ui.memory(|m| m.focused());
        let owned_before = focused.is_some_and(|id| self.control_ids().contains(&id));
        output.owns_keyboard = allowed && owned_before;
        if fresh && once {
            if owned_before && allowed {
                // Clear only this pane's stale focus and scheduled focus movement.
                ui.memory_mut(|m| m.move_focus(egui::FocusDirection::None));
                self.surrender_owned_focus(ui, self.reset_editor_state);
            }
            if self.reset_editor_state {
                egui::text_edit::TextEditState::default()
                    .store(ui.ctx(), self.id.with("page-editor"));
            }
        }
        let mut click = false;
        let mut stale_pointer = false;
        let mut enter = false;
        let mut space = false;
        if once {
            let current_target = self.target();
            ui.input(|input| {
                for event in &input.events {
                    match event {
                        egui::Event::PointerButton {
                            button: PointerButton::Primary,
                            pressed,
                            ..
                        } => {
                            if *pressed {
                                if !fresh && allowed && self.pointer == PointerEpoch::Ready {
                                    self.pointer = PointerEpoch::Pressed;
                                    self.pressed_target = current_target.clone();
                                } else {
                                    self.pointer = PointerEpoch::Blocked;
                                    self.pressed_target = None;
                                    stale_pointer = true;
                                }
                            } else {
                                let valid = !fresh
                                    && allowed
                                    && self.pointer == PointerEpoch::Pressed
                                    && self.pressed_target.is_some()
                                    && self.pressed_target == current_target;
                                click |= valid;
                                stale_pointer |= !valid;
                                self.pointer = PointerEpoch::Ready;
                                self.pressed_target = None;
                            }
                        }
                        egui::Event::Key {
                            key,
                            pressed,
                            repeat,
                            modifiers,
                            ..
                        } if *key == Key::Enter || *key == Key::Space => {
                            let blocked = if *key == Key::Enter {
                                &mut self.blocked_enter
                            } else {
                                &mut self.blocked_space
                            };
                            if !pressed {
                                *blocked = false;
                            } else {
                                if !fresh && !*blocked && !repeat && *modifiers == Modifiers::NONE {
                                    if *key == Key::Enter {
                                        enter = true;
                                    } else {
                                        space = true;
                                    }
                                }
                                // Including a release/press pair on the opening
                                // frame: this down belongs to the blocked epoch.
                                *blocked = true;
                            }
                        }
                        _ => {}
                    }
                }
                if self.pointer == PointerEpoch::Blocked {
                    stale_pointer |= input.pointer.primary_down();
                    if !input.pointer.primary_down() {
                        self.pointer = PointerEpoch::Ready;
                    }
                }
                if !input.key_down(Key::Enter) {
                    self.blocked_enter = false;
                }
                if !input.key_down(Key::Space) {
                    self.blocked_space = false;
                }
            });
            click &= !stale_pointer;
            if owned_before && allowed {
                // Consume activation keys only for our focused control. The editor
                // keeps spaces, text, arrows, Ctrl+A and unrelated pane input.
                let editor_focused = focused == Some(self.id.with("page-editor"));
                ui.input_mut(|input| input.events.retain(|event| !matches!(event,
                    egui::Event::Key { key, .. } if *key == Key::Enter || (!editor_focused && *key == Key::Space)
                )));
            }
        }
        let count = self.page_count();
        let id = self.id;
        let mut action = None;
        ui.scope_builder(egui::UiBuilder::new().id(id), |ui| {
            ui.heading("PDF preview");
            if let Some(document) = &self.document {
                let name = document
                    .path
                    .file_name()
                    .unwrap_or(document.path.as_os_str());
                let filename = ui
                    .scope_builder(egui::UiBuilder::new().id(id.with("filename")), |ui| {
                        display_label(ui, &name.to_string_lossy())
                    })
                    .inner;
                let path = ui
                    .scope_builder(egui::UiBuilder::new().id(id.with("path")), |ui| {
                        display_label(ui, &document.path.to_string_lossy())
                    })
                    .inner;
                ui.label(match &self.state {
                    DocumentState::Loading => "Loading PDF metadata".into(),
                    DocumentState::Available { .. } => {
                        format!("Pages: {}", count.expect("validated count"))
                    }
                    DocumentState::MissingRenderer => "PDF renderer unavailable".into(),
                    DocumentState::Malformed => "Malformed PDF document".into(),
                    DocumentState::PasswordRequired => {
                        "PDF requires a password; password entry is not supported".into()
                    }
                    DocumentState::Failure(reason) => format!("PDF preview failed: {reason}"),
                });
                if count.is_none() {
                    ui.label("Page count unavailable");
                }
                if let Some(count) = count {
                    ui.label(format!(
                        "Requested page: {} / {count}",
                        u64::from(self.requested) + 1
                    ));
                    ui.label(self.confirmed.map_or_else(
                        || "Displayed page: not confirmed".into(),
                        |page| format!("Displayed page: {}", u64::from(page) + 1),
                    ));
                    if let Some(pending) = &self.pending {
                        ui.label(format!(
                            "Rendering pending for page {}",
                            u64::from(pending.page_index) + 1
                        ));
                    }
                }
                if let Some(reason) = &self.render_error {
                    ui.label(format!("Rendering failed: {reason}"));
                }
                let responses = ui
                    .horizontal_wrapped(|ui| {
                        let previous = page_button(
                            ui,
                            id.with("previous"),
                            count.is_some() && self.requested > 0,
                            "Previous",
                        );
                        let next = page_button(
                            ui,
                            id.with("next"),
                            count.is_some_and(|c| self.requested < c - 1),
                            "Next",
                        );
                        ui.label("Page");
                        // Retry passes still register focus interest. TextEditState
                        // clones share the undoer Arc, so snapshot its contents too.
                        let saved_state = (!once)
                            .then(|| {
                                egui::text_edit::TextEditState::load(
                                    ui.ctx(),
                                    id.with("page-editor"),
                                )
                            })
                            .flatten()
                            .map(|state| {
                                let undoer = state.undoer();
                                (state, undoer)
                            });
                        let displayed = self.editor.clone();
                        let mut readonly = displayed.as_str();
                        let buffer: &mut dyn egui::TextBuffer = if once {
                            &mut self.editor
                        } else {
                            &mut readonly
                        };
                        let editor = ui.add_enabled(
                            count.is_some(),
                            egui::TextEdit::singleline(buffer)
                                .id(id.with("page-editor"))
                                .desired_width(70.0)
                                // Never truncate a decimal command to a valid prefix.
                                .return_key(None)
                                .interactive(
                                    allowed
                                        && (!fresh || !self.reset_editor_state)
                                        && !stale_pointer,
                                ),
                        );
                        if let Some((mut state, undoer)) = saved_state {
                            state.set_undoer(undoer);
                            state.store(ui.ctx(), id.with("page-editor"));
                        }
                        if once && editor.changed() {
                            self.editor_dirty = true;
                        }
                        let go = page_button(ui, id.with("go"), count.is_some(), "Go");
                        ([previous, next, go], editor)
                    })
                    .inner;
                let (buttons, editor) = &responses;
                if once && !fresh && !stale_pointer && allowed {
                    for (button, candidate) in buttons.iter().zip([
                        PageAction::Previous,
                        PageAction::Next,
                        PageAction::Direct(0),
                    ]) {
                        if button.enabled()
                            && ((click && button.clicked_by(PointerButton::Primary))
                                || ((enter || space) && button.has_focus() && owned_before))
                        {
                            action = Some(candidate);
                        }
                    }
                    if enter && focused == Some(editor.id) && editor.enabled() {
                        action = Some(PageAction::Direct(0));
                    }
                }
                if let Some(reason) = &self.editor_error {
                    ui.colored_label(ui.visuals().error_fg_color, reason);
                }
                #[cfg(test)]
                {
                    self.controls = Some((responses.0, responses.1, [filename, path]));
                }
                #[cfg(not(test))]
                {
                    let _ = (filename, path);
                }
            } else {
                ui.label("No PDF selected");
                #[cfg(test)]
                {
                    self.controls = None;
                }
            }
        });
        if fresh || stale_pointer {
            self.surrender_owned_focus(ui, stale_pointer || self.reset_editor_state);
        }
        output.owns_keyboard |= allowed
            && ui
                .memory(|m| m.focused())
                .is_some_and(|id| self.control_ids().contains(&id));
        if once {
            if let Some(action) = action {
                let action = if matches!(action, PageAction::Direct(_)) {
                    // ASCII decimal only. Preserve invalid drafts and keep focus.
                    self.editor_page().map(PageAction::Direct)
                } else {
                    Some(action)
                };
                let target = self.target();
                let result = match (target, action) {
                    (Some(target), Some(action)) => self.request_page(&target, action),
                    _ => Err(Refusal::InvalidPage),
                };
                match result {
                    Ok(request) => output.request = Some(request),
                    Err(Refusal::RequestSequenceExhausted) => {
                        self.render_error = Some("Page request sequence exhausted".into())
                    }
                    Err(_) => {
                        self.editor_error = Some(count.map_or_else(
                            || "Page count unavailable".into(),
                            |c| format!("Enter a whole page number from 1 to {c}"),
                        ))
                    }
                }
            } else if self.needs_request
                && let Some(target) = self.target()
            {
                match self.make_request(target, self.requested, false) {
                    Ok(request) => output.request = Some(request),
                    Err(_) => {
                        self.needs_request = false;
                        self.render_error = Some("Page request sequence exhausted".into());
                    }
                }
            }
        }
        output
    }
}

fn page_button(ui: &mut Ui, id: Id, enabled: bool, text: &str) -> egui::Response {
    ui.scope_builder(egui::UiBuilder::new().id(id), |ui| {
        // The first widget in this fixed explicit scope has a bounded auto ID.
        ui.add_enabled(enabled, egui::Button::new(text))
    })
    .inner
}

fn display_label(ui: &mut Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Label::new(text)
            .truncate()
            .show_tooltip_when_elided(false)
            .selectable(false),
    )
    .on_hover_text(text)
}

#[cfg(test)]
mod tests;
