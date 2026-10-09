//! Decision-only conflict controls. No filesystem access or execution authority.
use std::path::{Path, PathBuf};

use egui::{Context, Id, Key, Modifiers};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OperationId(pub u64);

/// Strictly increasing within one operation, including revalidation conflicts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConflictId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    RegularFile,
    Directory,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConflictType {
    pub source: EntryKind,
    pub destination: EntryKind,
}

impl ConflictType {
    pub fn allows_replace(self) -> bool {
        self.source == EntryKind::RegularFile && self.destination == EntryKind::RegularFile
    }

    fn description(self) -> String {
        fn kind(kind: EntryKind) -> &'static str {
            match kind {
                EntryKind::RegularFile => "regular file",
                EntryKind::Directory => "folder",
                EntryKind::Other => "other entry",
            }
        }
        format!(
            "Source: {}; existing destination: {}",
            kind(self.source),
            kind(self.destination)
        )
    }
}

/// Immutable supplied snapshot. Display conversion never becomes command identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictRequest {
    operation: OperationId,
    conflict: ConflictId,
    source: PathBuf,
    destination: PathBuf,
    kind: ConflictType,
}

impl ConflictRequest {
    pub fn new(
        operation: OperationId,
        conflict: ConflictId,
        source: PathBuf,
        destination: PathBuf,
        kind: ConflictType,
    ) -> Result<Self, Refusal> {
        if !source.is_absolute() || !destination.is_absolute() {
            return Err(Refusal::FullPathsRequired);
        }
        Ok(Self {
            operation,
            conflict,
            source,
            destination,
            kind,
        })
    }

    pub fn operation(&self) -> OperationId {
        self.operation
    }
    pub fn conflict(&self) -> ConflictId {
        self.conflict
    }
    pub fn source(&self) -> &Path {
        &self.source
    }
    pub fn destination(&self) -> &Path {
        &self.destination
    }
    pub fn kind(&self) -> ConflictType {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Skip,
    KeepBoth,
    Replace,
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecisionOrigin {
    User,
    RemainingPolicy,
}

/// A user choice, NOT a successful validation or permission to mutate disk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    request: ConflictRequest,
    choice: Choice,
    origin: DecisionOrigin,
    apply_to_remaining: bool,
}

impl Decision {
    pub fn request(&self) -> &ConflictRequest {
        &self.request
    }
    pub fn choice(&self) -> Choice {
        self.choice
    }
    pub fn origin(&self) -> DecisionOrigin {
        self.origin
    }
    pub fn apply_to_remaining(&self) -> bool {
        self.apply_to_remaining
    }
    pub fn matches(&self, request: &ConflictRequest) -> bool {
        &self.request == request
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    FullPathsRequired,
    WrongOperation,
    StaleRequest,
    NoPendingRequest,
    ReplaceNotAllowed,
    OperationEnded,
}

struct Pending {
    request: ConflictRequest,
    apply: bool,
    first_frame: Option<u64>,
    focus_set: bool,
}

struct Policy {
    kind: ConflictType,
    choice: Choice,
}

#[derive(Default)]
pub struct DialogOutput {
    pub decision: Option<Decision>,
    /// True through the decision frame. Host must suppress background shortcuts.
    pub owns_input: bool,
}

/// One controller per operation, owned by the window rather than a tab or pane.
/// Retains at most one request and one exact-type policy; no response queue.
/// Only one conflict presenter may be drawn in a window's egui Context at a time.
/// Sequential requests and operation controllers reuse that window's modal area.
pub struct ConflictDialog {
    operation: OperationId,
    latest: Option<ConflictId>,
    pending: Option<Pending>,
    policy: Option<Policy>,
    ended: bool,
    input_frame: Option<u64>,
    #[cfg(test)]
    controls: Option<([egui::Response; 4], egui::Response, [egui::Response; 2])>,
}

impl ConflictDialog {
    pub fn new(operation: OperationId) -> Self {
        Self {
            operation,
            latest: None,
            pending: None,
            policy: None,
            ended: false,
            input_frame: None,
            #[cfg(test)]
            controls: None,
        }
    }

    pub fn pending(&self) -> Option<&ConflictRequest> {
        self.pending.as_ref().map(|pending| &pending.request)
    }

    /// Present once per fresh ID. A newer request invalidates the previous UI.
    /// A compatible policy returns a fresh response immediately, never an old one.
    pub fn present(&mut self, request: ConflictRequest) -> Result<Option<Decision>, Refusal> {
        if request.operation != self.operation {
            return Err(Refusal::WrongOperation);
        }
        if self.ended {
            return Err(Refusal::OperationEnded);
        }
        if self.latest.is_some_and(|id| request.conflict <= id) {
            return Err(Refusal::StaleRequest);
        }
        self.latest = Some(request.conflict);
        self.pending = None;
        if let Some(policy) = &self.policy
            && policy.kind == request.kind
            && (policy.choice != Choice::Replace || request.kind.allows_replace())
        {
            return Ok(Some(Decision {
                request,
                choice: policy.choice,
                origin: DecisionOrigin::RemainingPolicy,
                apply_to_remaining: true,
            }));
        }
        self.pending = Some(Pending {
            request,
            apply: false,
            first_frame: None,
            focus_set: false,
        });
        Ok(None)
    }

    /// ID-checked choice, usable by the UI or a future window-owned controller.
    /// Validation and execution must separately revalidate both filesystem entries.
    pub fn decide(
        &mut self,
        operation: OperationId,
        conflict: ConflictId,
        choice: Choice,
        apply: bool,
    ) -> Result<Decision, Refusal> {
        if operation != self.operation {
            return Err(Refusal::WrongOperation);
        }
        let pending = self.pending.as_ref().ok_or(Refusal::NoPendingRequest)?;
        if pending.request.conflict != conflict {
            return Err(Refusal::StaleRequest);
        }
        if choice == Choice::Replace && !pending.request.kind.allows_replace() {
            return Err(Refusal::ReplaceNotAllowed);
        }
        let pending = self.pending.take().expect("pending request checked above");
        let apply = apply && choice != Choice::Cancel;
        self.policy = apply.then_some(Policy {
            kind: pending.request.kind,
            choice,
        });
        self.ended = choice == Choice::Cancel;
        Ok(Decision {
            request: pending.request,
            choice,
            origin: DecisionOrigin::User,
            apply_to_remaining: apply,
        })
    }

    /// Host calls on completion/cancellation. IDs cannot be reused in this controller.
    pub fn finish(&mut self) {
        self.pending = None;
        self.policy = None;
        self.ended = true;
    }

    /// Show actual modal controls. Enter always skips; Space activates focused controls.
    /// Outside clicks do not answer. Escape explicitly cancels this operation.
    pub fn show(&mut self, ctx: &Context) -> DialogOutput {
        let frame = ctx.cumulative_frame_nr();
        let Some(pending) = self.pending.as_mut() else {
            return DialogOutput {
                decision: None,
                owns_input: self.input_frame == Some(frame),
            };
        };
        self.input_frame = Some(frame);
        let request = &pending.request;
        let fresh = *pending.first_frame.get_or_insert(frame) == frame;
        // egui retains Area state and layer order after hiding. One area per window,
        // including across controller recreation; child IDs still isolate requests.
        let id = Id::new("mado-conflict-presenter");
        let modal = egui::Modal::new(id).show(ctx, |ui| {
            ui.push_id((self.operation, request.conflict), |ui| {
                let top = ui.memory(|memory| memory.top_modal_layer() == Some(ui.layer_id()));
                let (escape, enter, space, key_activation) = if top {
                    ui.input_mut(|input| {
                        let pressed = |wanted| {
                            input.events.iter().any(|event| matches!(event,
                        egui::Event::Key { key, pressed: true, repeat: false, modifiers, .. }
                        if *key == wanted && *modifiers == Modifiers::NONE
                    ))
                        };
                        let keys = (
                            pressed(Key::Escape),
                            pressed(Key::Enter),
                            pressed(Key::Space),
                            input.key_pressed(Key::Enter) || input.key_pressed(Key::Space),
                        );
                        // Consume repeats and modified variants too, preventing native button
                        // activation from a held key belonging to a previous request.
                        input.events.retain(|event| {
                            !matches!(
                                event,
                                egui::Event::Key {
                                    key: Key::Escape | Key::Enter | Key::Space,
                                    ..
                                }
                            )
                        });
                        keys
                    })
                } else {
                    (false, false, false, false)
                };
                ui.set_width((ctx.content_rect().width() - 48.0).clamp(160.0, 520.0));
                ui.heading("Destination conflict");
                ui.label(format!(
                    "Operation {} / conflict {}",
                    request.operation.0, request.conflict.0
                ));
                ui.label(request.kind.description());
                ui.label("Source");
                let source = path_label(ui, &request.source);
                ui.label("Destination");
                let destination = path_label(ui, &request.destination);
                if !request.kind.allows_replace() {
                    ui.label("Replace is unavailable. Folders are never merged or replaced.");
                }
                let previous_apply = pending.apply;
                let checkbox = ui.checkbox(
                    &mut pending.apply,
                    "Apply to remaining conflicts of this type in this operation",
                );
                if key_activation {
                    // egui may already have cached a keyboard click before consumption.
                    pending.apply = previous_apply;
                }
                ui.label(
                    "Enter: Skip. Escape: Cancel operation. Tab then Space: choose an action.",
                );
                let buttons = ui
                    .horizontal_wrapped(|ui| {
                        [
                            ui.button("Skip"),
                            ui.button("Keep both"),
                            ui.add_enabled(
                                request.kind.allows_replace(),
                                egui::Button::new("Replace"),
                            ),
                            ui.button("Cancel"),
                        ]
                    })
                    .inner;
                if top && !pending.focus_set {
                    // egui's modal layer becomes current after its initial sizing pass.
                    buttons[0].request_focus();
                    pending.focus_set = true;
                }
                if fresh {
                    // A new request cannot inherit a prior checkbox activation.
                    pending.apply = false;
                } else if space && checkbox.has_focus() {
                    pending.apply = !pending.apply;
                }
                let choices = [
                    Choice::Skip,
                    Choice::KeepBoth,
                    Choice::Replace,
                    Choice::Cancel,
                ];
                let choice = if fresh || !top {
                    None
                } else if escape {
                    Some(Choice::Cancel)
                } else if enter {
                    Some(Choice::Skip)
                } else {
                    buttons.iter().zip(choices).find_map(|(button, choice)| {
                        (button.enabled()
                            && ((!key_activation && button.clicked())
                                || (space && button.has_focus())))
                        .then_some(choice)
                    })
                };
                #[cfg(test)]
                {
                    self.controls = Some((buttons, checkbox, [source, destination]));
                }
                #[cfg(not(test))]
                {
                    let _ = (buttons, checkbox, source, destination);
                }
                choice
            })
            .inner
        });
        let operation = request.operation;
        let conflict = request.conflict;
        let apply = pending.apply;
        let decision = modal.inner.map(|choice| {
            self.decide(operation, conflict, choice, apply)
                .expect("UI choice uses current immutable request and enabled action")
        });
        DialogOutput {
            decision,
            owns_input: true,
        }
    }
}

fn path_label(ui: &mut egui::Ui, path: &Path) -> egui::Response {
    // Lossy conversion is presentation only; native identity stays in the request.
    ui.add(
        egui::Label::new(path.to_string_lossy())
            .truncate()
            .show_tooltip_when_elided(false)
            .selectable(false),
    )
    .on_hover_text(path.to_string_lossy())
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "../tests/retained_areas/mod.rs"]
mod retained_areas;
