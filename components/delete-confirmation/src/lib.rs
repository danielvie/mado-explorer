//! Window-owned deletion intent UI. No filesystem access or execution authority.
use std::path::{Path, PathBuf};

use egui::{Context, Event, Id, Key, Modifiers, PointerButton};

/// Caller supplies strictly increasing IDs for the window, including controller recreation.
/// Never reuse an ID for another request. The controller rejects its observed stale IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Trash,
    PermanentDelete,
}

/// Caller-supplied immutable snapshot, not a validated filesystem identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    id: RequestId,
    kind: Kind,
    paths: Vec<PathBuf>,
    location: PathBuf,
}

impl Request {
    pub fn new(id: RequestId, kind: Kind, paths: Vec<PathBuf>, location: PathBuf) -> Self {
        Self {
            id,
            kind,
            paths,
            location,
        }
    }
    pub fn id(&self) -> RequestId {
        self.id
    }
    pub fn kind(&self) -> Kind {
        self.kind
    }
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
    pub fn location(&self) -> &Path {
        &self.location
    }
    pub fn count(&self) -> usize {
        self.paths.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Confirm,
    Cancel,
}

/// Confirm means user intent only, NOT OS success or atomic mutation authorization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    request: Request,
    choice: Choice,
}
impl Answer {
    pub fn request(&self) -> &Request {
        &self.request
    }
    pub fn choice(&self) -> Choice {
        self.choice
    }
    pub fn deletion_intent(&self) -> Option<&Request> {
        (self.choice == Choice::Confirm).then_some(&self.request)
    }
}

pub const MAX_ITEMS: usize = 64;
pub const MAX_PATH_UNITS: usize = 32_768;
pub const MAX_TOTAL_UNITS: usize = 131_072;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Empty,
    TooManyItems,
    PathTooLong,
    TotalPathsTooLarge,
    FullPathsRequired,
    StaleRequest,
    WrongRequest,
    Closed,
}
impl Refusal {
    pub fn message(self) -> &'static str {
        match self {
            Self::Empty => "Confirmation refused: no captured entries.",
            Self::TooManyItems => {
                "Confirmation refused: more than 64 captured entries. No entries were omitted. Request a smaller selection."
            }
            Self::PathTooLong => {
                "Confirmation refused: a captured path exceeds 32768 native units. No path was truncated."
            }
            Self::TotalPathsTooLarge => {
                "Confirmation refused: paths exceed 131072 native units in total. No entries were omitted."
            }
            Self::FullPathsRequired => {
                "Confirmation refused: complete absolute source and location paths are required."
            }
            Self::StaleRequest => "Confirmation refused: this request ID is not fresh.",
            Self::WrongRequest => "Confirmation refused: this is not the displayed request.",
            Self::Closed => "Confirmation refused: the request is closed or already answered.",
        }
    }
}
fn units(path: &Path) -> usize {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        path.as_os_str().encode_wide().count()
    }
    #[cfg(not(windows))]
    {
        path.as_os_str().as_encoded_bytes().len()
    }
}
fn admit(request: &Request) -> Result<(), Refusal> {
    if request.paths.is_empty() {
        return Err(Refusal::Empty);
    }
    if request.count() > MAX_ITEMS {
        return Err(Refusal::TooManyItems);
    }
    let mut total = 0;
    for path in request
        .paths
        .iter()
        .map(PathBuf::as_path)
        .chain([request.location()])
    {
        let size = units(path);
        if size > MAX_PATH_UNITS {
            return Err(Refusal::PathTooLong);
        }
        total += size;
        if total > MAX_TOTAL_UNITS {
            return Err(Refusal::TotalPathsTooLarge);
        }
        if !path.is_absolute() {
            return Err(Refusal::FullPathsRequired);
        }
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PointerEpoch {
    Blocked,
    Ready,
    Pressed,
}
struct Pending {
    request: Request,
    answered: bool,
    first_frame: Option<u64>,
    focus_set: bool,
    navigated: bool,
    pointer: PointerEpoch,
    keys_down: [bool; 3],
    batch_frame: Option<u64>,
    click: bool,
    stale_pointer: bool,
    keys: [bool; 3],
}
impl Pending {
    fn new(request: Request) -> Self {
        Self {
            request,
            answered: false,
            first_frame: None,
            focus_set: false,
            navigated: false,
            pointer: PointerEpoch::Blocked,
            keys_down: [true; 3],
            batch_frame: None,
            click: false,
            stale_pointer: false,
            keys: [false; 3],
        }
    }
}

#[derive(Default)]
pub struct Output {
    pub answer: Option<Answer>,
    /// Host suppresses background commands, including on answer frames and retries.
    pub owns_input: bool,
}

/// Exactly one presenter per egui window Context. Keep one controller for that window.
/// Requests replace pending requests, never inherit a global confirmation preference.
/// Retains one bounded snapshot and at most one refusal, with a finite widget namespace.
#[derive(Default)]
pub struct Confirmation {
    latest: Option<RequestId>,
    pending: Option<Pending>,
    refusal: Option<Refusal>,
    input_frame: Option<u64>,
    #[cfg(test)]
    controls: Option<([egui::Response; 2], egui::Response, Vec<egui::Response>)>,
}
impl Confirmation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pending(&self) -> Option<&Request> {
        self.pending
            .as_ref()
            .filter(|p| !p.answered)
            .map(|p| &p.request)
    }
    pub fn refusal(&self) -> Option<Refusal> {
        self.refusal
    }
    pub fn present(&mut self, request: Request) -> Result<(), Refusal> {
        if self.latest.is_some_and(|id| request.id <= id) {
            return Err(Refusal::StaleRequest);
        }
        self.latest = Some(request.id);
        self.pending = None;
        self.refusal = admit(&request).err();
        if let Some(refusal) = self.refusal {
            return Err(refusal);
        }
        self.pending = Some(Pending::new(request));
        Ok(())
    }
    /// ID-checked user choice. Future worker must revalidate sources, permissions,
    /// recycle availability and protected roots. Never fall back from Trash to delete.
    pub fn choose(&mut self, id: RequestId, choice: Choice) -> Result<Answer, Refusal> {
        let pending = self
            .pending
            .as_mut()
            .filter(|p| !p.answered)
            .ok_or(Refusal::Closed)?;
        if pending.request.id != id {
            return Err(Refusal::WrongRequest);
        }
        pending.answered = true;
        Ok(Answer {
            request: pending.request.clone(),
            choice,
        })
    }
    /// Close without emitting any intent. A closed ID cannot be reused.
    pub fn close(&mut self, id: RequestId) -> Result<(), Refusal> {
        let pending = self
            .pending
            .as_mut()
            .filter(|p| !p.answered)
            .ok_or(Refusal::Closed)?;
        if pending.request.id != id {
            return Err(Refusal::WrongRequest);
        }
        pending.answered = true;
        Ok(())
    }

    /// Enter defaults to Cancel. Tab to Confirm then Enter/Space explicitly confirms.
    /// Escape always cancels. Modified/repeated/held and opening-frame keys cannot answer.
    pub fn show(&mut self, ctx: &Context) -> Output {
        let frame = ctx.cumulative_frame_nr();
        let presenter = Id::new("mado-delete-confirmation");
        if let Some(refusal) = self.refusal {
            self.input_frame = Some(frame);
            let mut dismiss = false;
            egui::Modal::new(presenter).show(ctx, |ui| {
                ui.heading("Confirmation unavailable");
                ui.label(refusal.message());
                dismiss = ui.button("Close confirmation").clicked();
            });
            if dismiss {
                self.refusal = None;
            }
            return Output {
                answer: None,
                owns_input: true,
            };
        }
        let Some(pending) = self.pending.as_mut() else {
            return Output {
                answer: None,
                owns_input: self.input_frame == Some(frame),
            };
        };
        // Keep the modal installed through an answer frame and its layout retries.
        // The answered flag prevents a second output from the retained batch.
        if pending.answered && self.input_frame != Some(frame) {
            self.pending = None;
            return Output::default();
        }
        self.input_frame = Some(frame);
        let fresh = *pending.first_frame.get_or_insert(frame) == frame;
        let modal = egui::Modal::new(presenter).show(ctx, |ui| {
            ui.scope_builder(egui::UiBuilder::new().id(presenter.with("body")), |ui| {
                let top = ui.memory(|m| m.top_modal_layer() == Some(ui.layer_id()) && m.allows_interaction(ui.layer_id()));
                if top && fresh { ui.memory_mut(|m| m.move_focus(egui::FocusDirection::None)); }
                // The request epoch, not egui's retained widget identity, owns gestures.
                // Process once per frame; cached releases/keys cannot replay on retries.
                if pending.batch_frame != Some(frame) {
                    pending.batch_frame = Some(frame);
                    pending.click = false;
                    pending.stale_pointer = false;
                    pending.keys = [false; 3];
                    let armed = top && !fresh && pending.focus_set && !pending.answered;
                    if !armed { pending.pointer = PointerEpoch::Blocked; pending.navigated = false; }
                    ui.input(|input| {
                        // A held key must be released before any fresh activation. Do not
                        // trust repeat alone: backend batches may contain duplicate downs.
                        if !armed {
                            pending.keys_down = [Key::Escape, Key::Enter, Key::Space].map(|key| input.key_down(key));
                        }
                        for event in &input.events {
                            match event {
                                Event::PointerButton { button: PointerButton::Primary, pressed, modifiers, .. } => {
                                    if *pressed {
                                        if armed && pending.pointer == PointerEpoch::Ready && *modifiers == Modifiers::NONE {
                                            pending.pointer = PointerEpoch::Pressed;
                                        } else {
                                            pending.pointer = PointerEpoch::Blocked;
                                            pending.stale_pointer = true;
                                        }
                                    } else {
                                        let valid = armed && pending.pointer == PointerEpoch::Pressed && *modifiers == Modifiers::NONE;
                                        pending.click |= valid;
                                        pending.stale_pointer |= !valid;
                                        pending.pointer = PointerEpoch::Ready;
                                    }
                                }
                                Event::Key { key, pressed, repeat, modifiers, .. } if armed => {
                                    if let Some(index) = [Key::Escape, Key::Enter, Key::Space].iter().position(|k| k == key) {
                                        if *pressed && !pending.keys_down[index] && !repeat && *modifiers == Modifiers::NONE { pending.keys[index] = true; }
                                        pending.keys_down[index] = *pressed;
                                    }
                                    if *key == Key::Tab && *pressed && !repeat && (*modifiers == Modifiers::NONE || *modifiers == Modifiers::SHIFT) {
                                        pending.navigated = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if pending.pointer == PointerEpoch::Blocked {
                            pending.stale_pointer |= input.pointer.primary_down();
                            if !input.pointer.primary_down() { pending.pointer = PointerEpoch::Ready; }
                        }
                        pending.click &= !pending.stale_pointer;
                    });
                    if !top { pending.focus_set = false; }
                }
                if top {
                    ui.input_mut(|input| input.events.retain(|event| !matches!(event, Event::Key { key: Key::Escape | Key::Enter | Key::Space, .. })));
                }
                ui.set_width((ctx.content_rect().width() - 48.0).clamp(160.0, 520.0));
                let request = &pending.request;
                ui.heading(match request.kind { Kind::Trash => "Move to Recycle Bin?", Kind::PermanentDelete => "Permanently delete?" });
                ui.label(match request.kind {
                    Kind::Trash => "Trash uses the Windows Recycle Bin. Recoverable items can be restored through the Recycle Bin. If unavailable, leave entries intact; never fall back to permanent deletion.",
                    Kind::PermanentDelete => "Irreversible: these entries will NOT go to the Recycle Bin. Permanent deletion cannot be undone by this application.",
                });
                ui.label(format!("{} captured items", request.count()));
                ui.label("Captured location");
                let location = path_label(ui, presenter.with("location"), request.location());
                ui.label("Captured entries");
                let paths = egui::ScrollArea::vertical().id_salt(presenter.with("paths")).max_height(180.0).show(ui, |ui| {
                    request.paths.iter().enumerate().map(|(index, path)| path_label(ui, presenter.with(("path", index)), path)).collect::<Vec<_>>()
                }).inner;
                ui.label("Enter: Cancel by default. Tab to Confirm, then Enter or Space. Escape: Cancel.");
                let buttons = ui.scope_builder(egui::UiBuilder::new().id(presenter.with("actions")), |ui| {
                    ui.horizontal_wrapped(|ui| [ui.button("Cancel"), ui.button(match request.kind { Kind::Trash => "Confirm Trash", Kind::PermanentDelete => "Confirm Permanent Delete" })]).inner
                }).inner;
                if top && (!pending.focus_set || fresh || pending.stale_pointer) {
                    buttons[0].request_focus();
                    pending.focus_set = true;
                    pending.navigated = false;
                }
                if buttons[1].has_focus() { ui.label("Keyboard focus: Confirm"); }
                else if buttons[0].has_focus() { ui.label("Keyboard focus: Cancel"); }
                let choice = if fresh || !top || pending.answered { None }
                else if pending.keys[0] { Some(Choice::Cancel) }
                else if pending.keys[1] { Some(if pending.navigated && buttons[1].has_focus() { Choice::Confirm } else { Choice::Cancel }) }
                else { buttons.iter().zip([Choice::Cancel, Choice::Confirm]).find_map(|(button, choice)| {
                    ((pending.click && button.clicked_by(PointerButton::Primary)) || (pending.keys[2] && button.has_focus() && (choice == Choice::Cancel || pending.navigated))).then_some(choice)
                }) };
                #[cfg(test)] { self.controls = Some((buttons, location, paths)); }
                #[cfg(not(test))] { let _ = (buttons, location, paths); }
                choice
            }).inner
        });
        let id = pending.request.id;
        Output {
            answer: modal
                .inner
                .map(|choice| self.choose(id, choice).expect("current unanswered request")),
            owns_input: true,
        }
    }
}
fn path_label(ui: &mut egui::Ui, id: Id, path: &Path) -> egui::Response {
    // Presentation only. Fixed slot IDs also bound egui's retained tooltip Areas.
    ui.scope_builder(egui::UiBuilder::new().id(id), |ui| {
        ui.add(
            egui::Label::new(path.to_string_lossy())
                .truncate()
                .show_tooltip_when_elided(false)
                .selectable(false),
        )
        .on_hover_text(path.to_string_lossy())
    })
    .inner
}

#[cfg(test)]
mod tests;
