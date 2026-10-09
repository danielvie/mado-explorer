//! Window-owned request queue and status UI. No executor or filesystem authorization.

use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

pub const MAX_PENDING: usize = 16;
pub const MAX_HISTORY: usize = 32;
pub const MAX_SOURCES: usize = 128;
pub const MAX_ISSUES: usize = 64;
pub const MAX_PATH_BYTES: usize = 65_536;
pub const MAX_METADATA_BYTES: usize = 262_144;
pub const MAX_ERROR_BYTES: usize = 4_096;

/// Caller supplies a unique window namespace and strictly increasing sequence.
/// IDs cannot be reused, including after explicitly dismissing a terminal report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperationId {
    pub window: u64,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    CreateFile,
    CreateFolder,
    Rename,
    Copy,
    Move,
    Trash,
    Delete,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Self::CreateFile => "Create file",
            Self::CreateFolder => "Create folder",
            Self::Rename => "Rename",
            Self::Copy => "Copy",
            Self::Move => "Move",
            Self::Trash => "Trash",
            Self::Delete => "Permanent delete",
        }
    }
}

/// Native display/request metadata only. Exact filesystem semantics, captured
/// file identities, validation and confirmations belong to the future backend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    id: OperationId,
    kind: Kind,
    sources: Vec<PathBuf>,
    destination: Option<PathBuf>,
}

impl Request {
    pub fn new(
        id: OperationId,
        kind: Kind,
        sources: Vec<PathBuf>,
        destination: Option<PathBuf>,
    ) -> Result<Self, Refusal> {
        let paths = sources.iter().chain(destination.iter());
        if sources.len() > MAX_SOURCES || !paths_fit(paths) {
            return Err(Refusal::MetadataCapacity);
        }
        Ok(Self {
            id,
            kind,
            sources,
            destination,
        })
    }
    pub fn id(&self) -> OperationId {
        self.id
    }
    pub fn kind(&self) -> Kind {
        self.kind
    }
    pub fn sources(&self) -> &[PathBuf] {
        &self.sources
    }
    pub fn destination(&self) -> Option<&Path> {
        self.destination.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub completed: u64,
    pub skipped: u64,
    pub failed: u64,
}

/// Units must be consistent within a supplied report, e.g. bytes or items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Progress {
    #[default]
    Unknown,
    Enumerating,
    WaitingForConflict,
    Known {
        processed: u64,
        total: u64,
    },
}

impl Progress {
    /// Invalid or zero totals stay indeterminate, never clamped into fake progress.
    pub fn fraction(self) -> Option<f32> {
        match self {
            Self::Known { processed, total } if total > 0 && processed <= total => {
                Some((processed as f64 / total as f64) as f32)
            }
            _ => None,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Unknown => "Progress unknown",
            Self::Enumerating => "Enumerating; total unknown",
            Self::WaitingForConflict => "Waiting for a conflict decision",
            Self::Known { .. } if self.fraction().is_none() => {
                "Invalid or empty total; progress unknown"
            }
            Self::Known { .. } => "Known progress",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Status {
    pub current_item: Option<PathBuf>,
    pub progress: Progress,
    pub counts: Counts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Completed,
    Cancelled,
    Failed,
}

/// Actual error and unresolved or recovery path supplied by the worker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Issue {
    pub path: PathBuf,
    pub error: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Update {
    Running(Status),
    /// The worker has actually stopped. Completed work is never reversed here.
    Stopped {
        status: Status,
        outcome: Outcome,
        issues: Vec<Issue>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub id: OperationId,
    pub revision: u64,
    pub update: Update,
}

/// Outputs are requests to a future reviewed backend, not permission to mutate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Start(Request),
    Cancel(OperationId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    ForeignWindow,
    ReusedId,
    PendingCapacity,
    HistoryCapacity,
    MetadataCapacity,
    StaleReport,
}

impl Refusal {
    pub fn message(self) -> &'static str {
        match self {
            Self::ForeignWindow => "Refused: foreign window namespace.",
            Self::ReusedId => "Refused: operation IDs must increase and cannot be reused.",
            Self::PendingCapacity => "Queue full: at most 16 pending requests. Nothing was added.",
            Self::HistoryCapacity => {
                "Report capacity full: acknowledge retained reports before adding requests. Nothing was added."
            }
            Self::MetadataCapacity => {
                "Metadata capacity exceeded. Nothing was truncated or accepted; caller must retain the request/report."
            }
            Self::StaleReport => "Ignored: stale, duplicate or non-active worker report.",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Queued,
    Active,
    Cancelling,
    Stopped(Outcome),
    CancelledBeforeStart,
}

#[derive(Debug)]
pub struct Record {
    request: Request,
    state: State,
    status: Status,
    issues: Vec<Issue>,
    revision: u64,
    cancel_sent: bool,
    details: bool,
    cancellation_requested: bool,
}

impl Record {
    fn queued(request: Request) -> Self {
        Self {
            request,
            state: State::Queued,
            status: Status::default(),
            issues: vec![],
            revision: 0,
            cancel_sent: false,
            details: false,
            cancellation_requested: false,
        }
    }
    pub fn request(&self) -> &Request {
        &self.request
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn issues(&self) -> &[Issue] {
        &self.issues
    }
}

/// Keep one instance in the application window, not in a tab or pane.
/// All terminal reports are retained until explicit acknowledgement. Pending
/// requests reserve history slots so a terminal acknowledgement cannot overflow.
pub struct Operations {
    window: u64,
    high_water: Option<u64>,
    active: Option<Record>,
    queued: VecDeque<Record>,
    history: VecDeque<Record>,
    notice: Option<Refusal>,
    open: bool,
}

impl Operations {
    pub fn new(window: u64) -> Self {
        Self {
            window,
            high_water: None,
            active: None,
            queued: VecDeque::new(),
            history: VecDeque::new(),
            notice: None,
            open: true,
        }
    }
    pub fn active(&self) -> Option<&Record> {
        self.active.as_ref()
    }
    pub fn queued(&self) -> &VecDeque<Record> {
        &self.queued
    }
    pub fn history(&self) -> &VecDeque<Record> {
        &self.history
    }
    pub fn notice(&self) -> Option<Refusal> {
        self.notice
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    fn pending(&self) -> usize {
        self.queued.len() + usize::from(self.active.is_some())
    }
    fn refuse(&mut self, refusal: Refusal) -> Result<(), Refusal> {
        self.notice = Some(refusal);
        Err(refusal)
    }
    /// Borrowed input lets callers retain refused requests without reconstructing
    /// lossy display paths. Capacity refusal never evicts any existing report.
    pub fn enqueue(&mut self, request: &Request) -> Result<(), Refusal> {
        if request.id.window != self.window {
            return self.refuse(Refusal::ForeignWindow);
        }
        if self
            .high_water
            .is_some_and(|last| request.id.sequence <= last)
        {
            return self.refuse(Refusal::ReusedId);
        }
        if self.pending() >= MAX_PENDING {
            return self.refuse(Refusal::PendingCapacity);
        }
        if self.history.len() + self.pending() >= MAX_HISTORY {
            return self.refuse(Refusal::HistoryCapacity);
        }
        self.high_water = Some(request.id.sequence);
        self.queued.push_back(Record::queued(request.clone()));
        Ok(())
    }
    /// Poll outside presentation. Exactly one Start, then optionally one Cancel,
    /// is emitted per request. No next Start until terminal worker acknowledgement.
    pub fn next_command(&mut self) -> Option<Command> {
        if let Some(active) = &mut self.active {
            if active.state == State::Cancelling && !active.cancel_sent {
                active.cancel_sent = true;
                return Some(Command::Cancel(active.request.id));
            }
            return None;
        }
        let mut record = self.queued.pop_front()?;
        record.state = State::Active;
        let command = Command::Start(record.request.clone());
        self.active = Some(record);
        Some(command)
    }
    pub fn cancel(&mut self, id: OperationId) -> bool {
        if let Some(active) = &mut self.active {
            if active.request.id == id && active.state == State::Active {
                active.state = State::Cancelling;
                active.cancellation_requested = true;
                return true;
            }
        }
        if let Some(index) = self
            .queued
            .iter()
            .position(|record| record.request.id == id)
        {
            let mut record = self.queued.remove(index).unwrap();
            record.state = State::CancelledBeforeStart;
            self.history.push_back(record);
            return true;
        }
        false
    }
    /// Only increasing reports for the active ID are accepted. On refusal caller
    /// retains the full report; no partial update or terminal transition occurs.
    pub fn report(&mut self, report: &Report) -> Result<(), Refusal> {
        if report.id.window != self.window {
            return self.refuse(Refusal::ForeignWindow);
        }
        let Some(active) = &self.active else {
            return self.refuse(Refusal::StaleReport);
        };
        if active.request.id != report.id || report.revision <= active.revision {
            return self.refuse(Refusal::StaleReport);
        }
        let (status, terminal) = match &report.update {
            Update::Running(status) => (status, None),
            Update::Stopped {
                status,
                outcome,
                issues,
            } => (status, Some((*outcome, issues))),
        };
        let issues = terminal.map_or(&[][..], |(_, issues)| issues.as_slice());
        if issues.len() > MAX_ISSUES {
            return self.refuse(Refusal::MetadataCapacity);
        }
        let bytes = issues.iter().fold(0_usize, |sum, issue| {
            sum.saturating_add(issue.error.len())
                .saturating_add(path_bytes(&issue.path))
        });
        if !paths_fit(status.current_item.iter())
            || issues.iter().any(|issue| {
                path_bytes(&issue.path) > MAX_PATH_BYTES || issue.error.len() > MAX_ERROR_BYTES
            })
            || bytes > MAX_METADATA_BYTES
        {
            return self.refuse(Refusal::MetadataCapacity);
        }
        let active = self.active.as_mut().unwrap();
        active.status = status.clone();
        active.revision = report.revision;
        if let Some((outcome, issues)) = terminal {
            let mut record = self.active.take().unwrap();
            record.state = State::Stopped(outcome);
            record.issues = issues.clone();
            self.history.push_back(record);
        }
        Ok(())
    }
    /// Explicit user acknowledgement only, never automatic eviction or retry.
    pub fn acknowledge(&mut self, id: OperationId) -> bool {
        if let Some(index) = self
            .history
            .iter()
            .position(|record| record.request.id == id)
        {
            self.history.remove(index);
            return true;
        }
        false
    }
    /// Nonmodal presenter. No tab IDs, per-operation Area/Modal/ScrollArea state,
    /// file I/O or backend polling. Returned responses support caller focus routing.
    pub fn show(&mut self, ui: &mut egui::Ui) -> DrawerResponse {
        ui.push_id(("operation-drawer", self.window), |ui| {
            let toggle = button(ui, "Operations", Some(self.open));
            if toggle.clicked() {
                self.open = !self.open;
            }
            let mut controls = vec![];
            if self.open {
                ui.label(format!(
                    "Pending: {}/16 | Retained reports and reserved slots: {}/32",
                    self.pending(),
                    self.history.len() + self.pending()
                ));
                if let Some(notice) = self.notice {
                    ui.add(egui::Label::new(notice.message()).wrap());
                }
                let hover_id = ui.id().with("path-hover");
                egui::ScrollArea::vertical()
                    .id_salt("drawer-scroll")
                    .max_height(480.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        let mut cancel = None;
                        let mut acknowledge = None;
                        ui.heading("Active operation");
                        if let Some(record) = &mut self.active {
                            draw_record(
                                ui,
                                record,
                                hover_id,
                                &mut controls,
                                &mut cancel,
                                &mut acknowledge,
                            );
                        } else {
                            ui.label("No active operation.");
                        }
                        ui.heading("Queued requests");
                        if self.queued.is_empty() {
                            ui.label("Queue empty.");
                        }
                        for record in &mut self.queued {
                            draw_record(
                                ui,
                                record,
                                hover_id,
                                &mut controls,
                                &mut cancel,
                                &mut acknowledge,
                            );
                        }
                        ui.heading("Retained reports");
                        if self.history.is_empty() {
                            ui.label("No retained reports.");
                        }
                        for record in &mut self.history {
                            draw_record(
                                ui,
                                record,
                                hover_id,
                                &mut controls,
                                &mut cancel,
                                &mut acknowledge,
                            );
                        }
                        if let Some(id) = cancel {
                            self.cancel(id);
                        }
                        if let Some(id) = acknowledge {
                            self.acknowledge(id);
                        }
                    });
            }
            DrawerResponse { toggle, controls }
        })
        .inner
    }
}

pub struct DrawerResponse {
    pub toggle: egui::Response,
    pub controls: Vec<ControlResponse>,
}
pub struct ControlResponse {
    pub id: OperationId,
    pub action: Action,
    pub response: egui::Response,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Cancel,
    Details,
    Acknowledge,
}

fn draw_record(
    ui: &mut egui::Ui,
    record: &mut Record,
    hover_id: egui::Id,
    controls: &mut Vec<ControlResponse>,
    cancel: &mut Option<OperationId>,
    acknowledge: &mut Option<OperationId>,
) {
    let id = record.request.id;
    // Request-specific buttons prevent a late click/focus from targeting a new job.
    // No operation-specific persistent egui data is inserted by these child IDs.
    ui.push_id(id.sequence, |ui| {
        ui.separator();
        ui.label(format!("{} #{}", record.request.kind.label(), id.sequence));
        ui.label(match record.state {
            State::Queued => "Queued; not started",
            State::Active => "Active",
            State::Cancelling => {
                "Cancelling: requested, waiting for worker to stop. An OS call may still finish."
            }
            State::CancelledBeforeStart => "Cancelled before start; no Start request emitted",
            State::Stopped(Outcome::Completed) if record.cancellation_requested => {
                "Worker finished after cancellation request"
            }
            State::Stopped(Outcome::Completed) => "Worker finished",
            State::Stopped(Outcome::Cancelled) => {
                "Worker acknowledged stop; completed work remains"
            }
            State::Stopped(Outcome::Failed) => "Worker stopped with failure",
        });
        if let Some(source) = record.request.sources.first() {
            path_label(ui, hover_id, "Source", source);
        } else {
            ui.label("Source: None supplied");
        }
        ui.label(format!("Source paths: {}", record.request.sources.len()));
        if let Some(destination) = record.request.destination() {
            path_label(ui, hover_id, "Destination", destination);
        } else {
            ui.label("Destination: Not applicable");
        }
        if let Some(item) = &record.status.current_item {
            path_label(ui, hover_id, "Current item", item);
        } else {
            ui.label("Current item: Not reported");
        }
        let counts = record.status.counts;
        ui.label(format!(
            "Completed: {} | Skipped: {} | Failed: {}",
            counts.completed, counts.skipped, counts.failed
        ));
        if !matches!(record.state, State::Queued | State::CancelledBeforeStart) {
            if let Some(fraction) = record.status.progress.fraction() {
                let Progress::Known { processed, total } = record.status.progress else {
                    unreachable!()
                };
                ui.add(egui::ProgressBar::new(fraction).text(format!("{processed}/{total}")));
            } else if matches!(record.state, State::Active | State::Cancelling) {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(record.status.progress.label());
                });
            } else {
                ui.label(record.status.progress.label());
            }
        }
        if !record.issues.is_empty() {
            ui.label(format!(
                "Unresolved/recovery reports: {}. View paths and reports for errors.",
                record.issues.len()
            ));
        }
        for (action, label) in [
            (Action::Details, "View paths and reports"),
            (
                Action::Cancel,
                if record.state == State::Queued {
                    "Cancel queued request"
                } else {
                    "Request cancellation"
                },
            ),
            (Action::Acknowledge, "Acknowledge and dismiss report"),
        ] {
            if action == Action::Cancel && !matches!(record.state, State::Queued | State::Active) {
                continue;
            }
            if action == Action::Acknowledge
                && !matches!(
                    record.state,
                    State::Stopped(_) | State::CancelledBeforeStart
                )
            {
                continue;
            }
            let response = ui
                .push_id(action as u8, |ui| {
                    button(
                        ui,
                        label,
                        (action == Action::Details).then_some(record.details),
                    )
                })
                .inner;
            if response.clicked() {
                match action {
                    Action::Details => record.details = !record.details,
                    Action::Cancel => *cancel = Some(id),
                    Action::Acknowledge => *acknowledge = Some(id),
                }
            }
            controls.push(ControlResponse {
                id,
                action,
                response,
            });
        }
        if record.details {
            for source in &record.request.sources {
                path_label(ui, hover_id, "Source", source);
            }
            for issue in &record.issues {
                path_label(ui, hover_id, "Unresolved/recovery path", &issue.path);
                ui.add(egui::Label::new(&issue.error).wrap());
            }
        }
    });
}

fn button(ui: &mut egui::Ui, label: &str, selected: Option<bool>) -> egui::Response {
    let response = ui.add(egui::Button::new(label).selected(selected.unwrap_or(false)));
    if response.has_focus() {
        if response.gained_focus() {
            response.scroll_to_me(Some(egui::Align::Center));
        }
        ui.painter().rect_stroke(
            response.rect.expand(1.0),
            0.0,
            egui::Stroke::new(2.0, ui.visuals().text_color()),
            egui::StrokeKind::Outside,
        );
    }
    response
}

fn path_label(ui: &mut egui::Ui, hover_id: egui::Id, prefix: &str, path: &Path) {
    // Lossy conversion is strictly display-only. Full native paths stay in Request.
    let text = format!("{prefix}: {}", path.to_string_lossy());
    let response = ui.add(
        egui::Label::new(&text)
            .truncate()
            .show_tooltip_when_elided(false),
    );
    // Tooltip visibility checks must use the same bounded parent identity as its
    // Area; otherwise egui treats our own open tooltip as a competing tooltip.
    let mut tooltip_response = response.clone();
    tooltip_response.id = hover_id;
    if response.hovered() && egui::Tooltip::should_show_tooltip(&tooltip_response, true) {
        // One fixed tooltip layer per presenter, not a new Area per operation/path.
        egui::Tooltip::always_open(ui.ctx().clone(), ui.layer_id(), hover_id, response.rect).show(
            |ui| {
                ui.add(egui::Label::new(&text).wrap());
            },
        );
    }
}
fn path_bytes(path: &Path) -> usize {
    path.as_os_str().as_encoded_bytes().len()
}
fn paths_fit<'a>(paths: impl Iterator<Item = &'a PathBuf>) -> bool {
    let mut total = 0_usize;
    for path in paths {
        let bytes = path_bytes(path);
        total = total.saturating_add(bytes);
        if bytes > MAX_PATH_BYTES || total > MAX_METADATA_BYTES {
            return false;
        }
    }
    true
}
