//! Creation only. All captures, mutations and pin retirement belong to one fixed worker.
use crate::{panes::PaneId, tabs::TabId, watch_adapter::NATIVE_CAPABILITY_GATE_CLEARED};
use mado_fs_create::{CapturedDirectory, Identity};
use mado_operation_drawer::{
    Command, Counts, Issue, Kind, OperationId, Operations, Outcome, Progress, Report, Request,
    State, Status, Update,
};
use std::{
    collections::VecDeque,
    ffi::OsString,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender, TrySendError},
    },
    thread::{self, JoinHandle},
};

pub(super) const UNAVAILABLE: &str = "Creation unavailable: native capability verification pending. No files changed. Initial backend supports local NTFS only.";
#[derive(Clone, Debug)]
pub(super) struct Target {
    pub pane: PaneId,
    pub tab: TabId,
    pub path: PathBuf,
    pub generation: u64,
}
pub(super) struct Editor {
    pub token: u64,
    pub origin_generation: u64,
    pub target: Target,
    pub kind: Kind,
    pub name: String,
    pub identity: Option<Identity>,
    pub expected: Option<Identity>,
    pub preparing: bool,
    pub error: Option<String>,
    pub focus: bool,
    pub input: InputOwner,
}
pub(super) struct Payload {
    pub token: u64,
    pub origin_generation: u64,
    pub target: Target,
    pub request: Request,
    pub name: OsString,
    pub identity: Identity,
    pub cancel: Arc<AtomicBool>,
    pub retired: bool,
    pub selection_generation: Option<u64>,
    pub selected: bool,
    pub filtered: bool,
    pub slot: usize,
    pub input: InputOwner,
}
// Finite presentation IDs need a fresh gesture when their owner changes.
#[derive(Default)]
pub(super) struct InputOwner {
    pub first: Option<u64>,
    pub pointer_blocked: bool,
    pub keys_blocked: bool,
    frame: Option<u64>,
    allowed: bool,
}
impl InputOwner {
    pub fn prepare(&mut self, ctx: &eframe::egui::Context, id: eframe::egui::Id) -> bool {
        use eframe::egui::Key;
        let frame = ctx.cumulative_frame_nr();
        let (pointer, keys) = ctx.input(|i| {
            (
                i.pointer.primary_down(),
                i.key_down(Key::Enter) || i.key_down(Key::Space),
            )
        });
        if self.frame == Some(frame) {
            return self.allowed;
        }
        self.frame = Some(frame);
        if self.first.is_none() {
            self.first = Some(frame);
            self.pointer_blocked = pointer;
            self.keys_blocked = keys;
            ctx.memory_mut(|m| m.surrender_focus(id));
        }
        self.allowed = self.first != Some(frame) && !self.pointer_blocked && !self.keys_blocked;
        // The release frame still belongs to the old gesture. Rearm only the next frame.
        self.pointer_blocked &= pointer;
        self.keys_blocked &= keys;
        self.allowed
    }
}
enum Work {
    Prepare {
        token: u64,
        path: PathBuf,
        expected: Option<Identity>,
    },
    Admit {
        token: u64,
        request: Request,
        name: OsString,
        cancel: Arc<AtomicBool>,
    },
    Start(Request),
    Retire(u64),
    Stop,
}
enum Event {
    Prepared(u64, Result<Identity, String>),
    Report(Report),
    Stopped,
}
struct Pin {
    token: u64,
    directory: CapturedDirectory,
    admitted: Option<(Request, OsString, Arc<AtomicBool>)>,
}
struct Worker {
    tx: SyncSender<Work>,
    rx: Receiver<Event>,
    thread: JoinHandle<()>,
    stopping: Arc<AtomicBool>,
}
#[cfg(test)]
#[derive(Default)]
pub(super) struct BeforeCallGate {
    pub entered: AtomicBool,
    pub release: AtomicBool,
    pub after_call: bool,
}
fn same_request(a: &Request, b: &Request) -> bool {
    a.id() == b.id()
        && a.kind() == b.kind()
        && a.sources().len() == b.sources().len()
        && a.sources()
            .iter()
            .zip(b.sources())
            .all(|(a, b)| a.as_os_str() == b.as_os_str())
        && a.destination().map(|p| p.as_os_str()) == b.destination().map(|p| p.as_os_str())
}
fn mutation_error(error: mado_fs_create::Error) -> String {
    // Drawer already retains the exact unresolved native path. Do not duplicate a long
    // path into its bounded error field or truncate the actual provider error.
    use mado_fs_create::Error;
    match error {
        Error::Io { action, source, .. } => format!("{action}: {source}"),
        Error::InvalidName(reason) => reason.to_string(),
        Error::UnsupportedPath(_) => "Unsupported absolute filesystem path".into(),
        Error::UnsupportedProvider(_) => {
            "Creation requires a supported local NTFS directory with native identity".into()
        }
        Error::ReparsePoint(_) => "Captured directory chain contains a reparse point".into(),
        Error::NotDirectory(_) => "Captured destination is not a directory".into(),
    }
}
fn worker(
    rx: Receiver<Work>,
    tx: SyncSender<Event>,
    stopping: Arc<AtomicBool>,
    #[cfg(test)] gate: Option<Arc<BeforeCallGate>>,
    #[cfg(test)] captures: Arc<std::sync::atomic::AtomicUsize>,
) {
    let mut pins: Vec<Pin> = Vec::new();
    while let Ok(work) = rx.recv() {
        match work {
            Work::Prepare {
                token,
                path,
                expected,
            } => {
                let result = if stopping.load(Ordering::Acquire) {
                    Err("Destination preparation cancelled before provider call.".into())
                } else if pins.len() >= 17 {
                    Err("Creation capture capacity busy while pins retire. Retry preparation explicitly; nothing created.".into())
                } else {
                    #[cfg(test)]
                    captures.fetch_add(1, Ordering::AcqRel);
                    CapturedDirectory::capture(path).map_err(|e| e.to_string()).and_then(|directory| {
                    if expected.is_some_and(|id| id != directory.identity()) {
                        return Err("Captured destination identity changed. Retry refused; nothing created.".into());
                    }
                    let identity = directory.identity();
                    pins.push(Pin { token, directory, admitted: None });
                    Ok(identity)
                })
                };
                if tx.send(Event::Prepared(token, result)).is_err() {
                    break;
                }
            }
            Work::Admit {
                token,
                request,
                name,
                cancel,
            } => {
                if let Some(pin) = pins.iter_mut().find(|p| p.token == token) {
                    pin.admitted = Some((request, name, cancel));
                }
            }
            Work::Retire(token) => pins.retain(|p| p.token != token),
            Work::Start(request) => {
                let pin = pins
                    .iter()
                    .position(|p| {
                        p.admitted
                            .as_ref()
                            .is_some_and(|(r, _, _)| r.id() == request.id())
                    })
                    .map(|i| pins.remove(i));
                let item = request.destination().unwrap().to_owned();
                let running = Report {
                    id: request.id(),
                    revision: 1,
                    update: Update::Running(Status {
                        current_item: Some(item.clone()),
                        progress: Progress::Known {
                            processed: 0,
                            total: 1,
                        },
                        counts: Counts::default(),
                    }),
                };
                if tx.send(Event::Report(running)).is_err() {
                    break;
                }
                let mut outcome = Outcome::Failed;
                let mut error = None;
                let mut completed = 0;
                if let Some(pin) = pin {
                    let (metadata, name, cancel) = pin.admitted.as_ref().unwrap();
                    let matches = same_request(metadata, &request)
                        && request.sources().is_empty()
                        && request.destination().map(|p| p.as_os_str())
                            == Some(pin.directory.original_path().join(name).as_os_str())
                        && matches!(request.kind(), Kind::CreateFile | Kind::CreateFolder);
                    #[cfg(test)]
                    if let Some(g) = gate.as_ref().filter(|g| !g.after_call) {
                        g.entered.store(true, Ordering::Release);
                        while !g.release.load(Ordering::Acquire) {
                            thread::yield_now();
                        }
                    }
                    if !matches {
                        error = Some(
                            "Worker payload does not match drawer request. Nothing created.".into(),
                        );
                    } else if cancel.load(Ordering::Acquire) || stopping.load(Ordering::Acquire) {
                        outcome = Outcome::Cancelled;
                    } else {
                        let result = if request.kind() == Kind::CreateFolder {
                            pin.directory.create_directory(name)
                        } else {
                            pin.directory.create_empty_file(name)
                        };
                        match result {
                            Ok(path) => {
                                assert_eq!(path, item);
                                outcome = Outcome::Completed;
                                completed = 1;
                            }
                            Err(e) => error = Some(mutation_error(e)),
                        }
                    }
                    // Test-only acknowledgement gate models an OS call finishing before cancellation.
                    #[cfg(test)]
                    if let Some(g) = gate.as_ref().filter(|g| g.after_call) {
                        g.entered.store(true, Ordering::Release);
                        while !g.release.load(Ordering::Acquire) {
                            thread::yield_now();
                        }
                    }
                    drop(pin); // Release every pin before terminal acknowledgement or successor.
                } else {
                    error = Some("Captured destination unavailable. Nothing created.".into());
                }
                let report = Report {
                    id: request.id(),
                    revision: 2,
                    update: Update::Stopped {
                        status: Status {
                            current_item: Some(item.clone()),
                            progress: Progress::Known {
                                processed: completed,
                                total: 1,
                            },
                            counts: Counts {
                                completed,
                                failed: u64::from(error.is_some()),
                                skipped: 0,
                            },
                        },
                        outcome,
                        issues: error
                            .into_iter()
                            .map(|error| Issue {
                                path: item.clone(),
                                error,
                            })
                            .collect(),
                    },
                };
                if tx.send(Event::Report(report)).is_err() {
                    break;
                }
            }
            Work::Stop => break,
        }
    }
    drop(pins);
    let _ = tx.send(Event::Stopped);
}

pub(super) struct CreateAdapter {
    pub operations: Operations,
    pub editor: Option<Editor>,
    pub payloads: Vec<Payload>,
    pub dirty: Vec<TabId>,
    pub notice: Option<String>,
    pub refused_report: Option<Report>,
    pub close_prompt: bool,
    pub waiting_close: bool,
    pub cleanup_acknowledged: bool,
    pub close_input: InputOwner,
    pub ui_owned_frame: Option<u64>,
    next: u64,
    worker: Option<Worker>,
    outbox: VecDeque<Work>,
    enabled: bool,
    #[cfg(test)]
    owned_root: Option<PathBuf>,
    #[cfg(test)]
    pub hold_start: bool,
    #[cfg(test)]
    pub worker_live: Arc<AtomicBool>,
    #[cfg(test)]
    pub capture_calls: Arc<std::sync::atomic::AtomicUsize>,
}
impl CreateAdapter {
    pub fn production() -> Self {
        Self {
            operations: Operations::new(1),
            editor: None,
            payloads: Vec::new(),
            dirty: Vec::new(),
            notice: None,
            refused_report: None,
            close_prompt: false,
            waiting_close: false,
            cleanup_acknowledged: true,
            close_input: InputOwner::default(),
            ui_owned_frame: None,
            next: 1,
            worker: None,
            outbox: VecDeque::new(),
            enabled: NATIVE_CAPABILITY_GATE_CLEARED,
            #[cfg(test)]
            owned_root: None,
            #[cfg(test)]
            hold_start: false,
            #[cfg(test)]
            worker_live: Arc::new(AtomicBool::new(false)),
            #[cfg(test)]
            capture_calls: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
    fn start(&mut self, #[cfg(test)] gate: Option<Arc<BeforeCallGate>>) -> Result<(), String> {
        if self.worker.is_some() {
            return Ok(());
        }
        let (tx, requests) = mpsc::sync_channel(34);
        let (reports, rx) = mpsc::sync_channel(34);
        let stopping = Arc::new(AtomicBool::new(false));
        let stop = stopping.clone();
        #[cfg(test)]
        let captures = self.capture_calls.clone();
        #[cfg(test)]
        let live = self.worker_live.clone();
        #[cfg(test)]
        live.store(true, Ordering::Release);
        let thread = thread::Builder::new()
            .name("create-worker".into())
            .spawn(move || {
                #[cfg(test)]
                struct Alive(Arc<AtomicBool>);
                #[cfg(test)]
                impl Drop for Alive {
                    fn drop(&mut self) {
                        self.0.store(false, Ordering::Release);
                    }
                }
                #[cfg(test)]
                let _alive = Alive(live);
                worker(
                    requests,
                    reports,
                    stop,
                    #[cfg(test)]
                    gate,
                    #[cfg(test)]
                    captures,
                )
            })
            .map_err(|e| {
                #[cfg(test)]
                self.worker_live.store(false, Ordering::Release);
                format!("Start creation worker: {e}")
            })?;
        self.worker = Some(Worker {
            tx,
            rx,
            thread,
            stopping,
        });
        self.cleanup_acknowledged = false;
        Ok(())
    }
    #[cfg(test)]
    pub fn enable_owned(root: PathBuf, gate: Option<Arc<BeforeCallGate>>) -> Self {
        assert!(root.is_absolute());
        let mut adapter = Self::production();
        adapter.enabled = true;
        adapter.owned_root = Some(root);
        adapter.start(gate).unwrap();
        adapter
    }
    pub fn open(&mut self, target: Target, kind: Kind) {
        if self.outbox.len() >= 16 {
            self.notice = Some("Creation control delivery busy. Retry explicitly when the worker catches up; destination was not captured.".into());
            return;
        }
        if self.editor.is_some() || self.waiting_close {
            self.notice = Some("Finish or cancel the existing creation editor first.".into());
            return;
        }
        if !self.enabled {
            self.notice = Some(UNAVAILABLE.into());
            return;
        }
        #[cfg(test)]
        if let Some(root) = &self.owned_root {
            assert!(
                target.path.starts_with(root)
                    && target.path.parent().is_some_and(|p| p.starts_with(root))
            );
        }
        if let Err(e) = self.start(
            #[cfg(test)]
            None,
        ) {
            self.notice = Some(e);
            return;
        }
        let token = self.next;
        self.next = self.next.checked_add(1).expect("Creation ID exhausted");
        self.editor = Some(Editor {
            token,
            origin_generation: target.generation,
            target,
            kind,
            name: String::new(),
            identity: None,
            expected: None,
            preparing: true,
            error: None,
            focus: true,
            input: InputOwner::default(),
        });
        self.prepare();
    }
    pub fn prepare(&mut self) {
        let Some(editor) = &mut self.editor else {
            return;
        };
        if editor.identity.is_some() {
            return;
        }
        editor.preparing = true;
        editor.error = None;
        self.outbox.push_back(Work::Prepare {
            token: editor.token,
            path: editor.target.path.clone(),
            expected: editor.expected,
        });
    }
    pub fn dismiss(&mut self) {
        if let Some(editor) = self.editor.take() {
            self.outbox.push_back(Work::Retire(editor.token));
        }
    }
    pub fn commit(&mut self) -> bool {
        let Some(editor) = &mut self.editor else {
            return false;
        };
        if let Err(e) = mado_fs_create::validate_name(editor.name.as_ref()) {
            editor.error = Some(e.to_string());
            return false;
        }
        let Some(identity) = editor.identity.filter(|_| !editor.preparing) else {
            editor.error = Some("Destination preparation must succeed before creation.".into());
            return false;
        };
        let id = OperationId {
            window: 1,
            sequence: self.next,
        };
        let name = OsString::from(&editor.name);
        let request = match Request::new(
            id,
            editor.kind,
            vec![],
            Some(editor.target.path.join(&name)),
        ) {
            Ok(request) => request,
            Err(e) => {
                editor.error = Some(e.message().into());
                return false;
            }
        };
        if let Err(e) = self.operations.enqueue(&request) {
            editor.error = Some(e.message().into());
            return false;
        }
        self.next = self.next.checked_add(1).expect("Creation ID exhausted");
        let editor = self.editor.take().unwrap();
        let cancel = Arc::new(AtomicBool::new(false));
        let slot = (0..32)
            .find(|s| self.payloads.iter().all(|p| p.slot != *s))
            .unwrap();
        self.outbox.push_back(Work::Admit {
            token: editor.token,
            request: request.clone(),
            name: name.clone(),
            cancel: cancel.clone(),
        });
        self.payloads.push(Payload {
            token: editor.token,
            origin_generation: editor.origin_generation,
            target: editor.target,
            request,
            name,
            identity,
            cancel,
            retired: false,
            selection_generation: None,
            selected: false,
            filtered: false,
            slot,
            input: InputOwner::default(),
        });
        true
    }
    pub fn retry(&mut self, id: OperationId) {
        if self.editor.is_some() || self.waiting_close {
            return;
        }
        if !self.operations.history().iter().any(|r| {
            r.request().id() == id
                && r.state() == State::Stopped(Outcome::Failed)
                && !r.issues().is_empty()
        }) {
            return;
        }
        let Some(payload) = self.payloads.iter().find(|p| p.request.id() == id) else {
            return;
        };
        let target = payload.target.clone();
        let kind = payload.request.kind();
        let name = payload.name.to_str().unwrap().to_owned(); // UI-entered names only.
        let identity = payload.identity;
        let origin_generation = payload.origin_generation;
        self.open(target, kind);
        if let Some(editor) = &mut self.editor {
            editor.name = name;
            editor.expected = Some(identity);
            editor.origin_generation = origin_generation;
            // Replace unsent initial preparation with identity-checked retry.
            self.outbox.pop_back();
            self.prepare();
        }
    }
    pub fn context_refreshed(
        &mut self,
        tab: TabId,
        path: &std::path::Path,
        previous: u64,
        generation: u64,
    ) {
        // Forward selection eligibility only across an explicit same-context refresh.
        // Captured Target remains immutable. New navigation/F5 does not call this hook.
        if let Some(editor) = &mut self.editor {
            if editor.target.tab == tab
                && editor.target.path.as_os_str() == path.as_os_str()
                && editor.origin_generation == previous
            {
                editor.origin_generation = generation;
            }
        }
        for p in &mut self.payloads {
            if p.target.tab == tab
                && p.target.path.as_os_str() == path.as_os_str()
                && p.origin_generation == previous
            {
                p.origin_generation = generation;
                if !p.selected && p.selection_generation == Some(previous) {
                    p.selection_generation = Some(generation);
                }
            }
        }
    }
    pub fn busy(&self) -> bool {
        self.editor.is_some()
            || self.operations.active().is_some()
            || !self.operations.queued().is_empty()
            || !self.outbox.is_empty()
    }
    pub fn cancel_and_stop(&mut self) {
        self.close_prompt = false;
        self.waiting_close = true;
        if let Some(worker) = &self.worker {
            worker.stopping.store(true, Ordering::Release);
        }
        self.dismiss();
        let ids: Vec<_> = self
            .operations
            .queued()
            .iter()
            .map(|r| r.request().id())
            .chain(self.operations.active().map(|r| r.request().id()))
            .collect();
        for id in ids {
            self.operations.cancel(id);
        }
        for p in &self.payloads {
            p.cancel.store(true, Ordering::Release);
        }
        self.outbox.push_back(Work::Stop);
        if self.worker.is_none() {
            self.cleanup_acknowledged = true;
            self.outbox.clear();
        }
    }
    pub fn poll(&mut self) -> Vec<OperationId> {
        let mut completed = Vec::new();
        if let Some(w) = &self.worker {
            while let Ok(event) = w.rx.try_recv() {
                match event {
                    Event::Prepared(token, result) => {
                        if let Some(e) = self.editor.as_mut().filter(|e| e.token == token) {
                            e.preparing = false;
                            match result {
                                Ok(id) => {
                                    e.identity = Some(id);
                                    e.error = None;
                                }
                                Err(error) => e.error = Some(error),
                            }
                        }
                    }
                    Event::Report(report) => {
                        if self.operations.report(&report).is_ok() {
                            if matches!(
                                report.update,
                                Update::Stopped {
                                    outcome: Outcome::Completed,
                                    ..
                                }
                            ) {
                                completed.push(report.id);
                            }
                        } else {
                            self.notice = Some("Worker report refused; full report retained and active operation cannot safely advance.".into());
                            self.refused_report = Some(report);
                        }
                    }
                    Event::Stopped => self.cleanup_acknowledged = true,
                }
            }
        }
        // Terminal queued cancellations retire pins without ever emitting Start.
        for p in &mut self.payloads {
            let pending = self
                .operations
                .active()
                .is_some_and(|r| r.request().id() == p.request.id())
                || self
                    .operations
                    .queued()
                    .iter()
                    .any(|r| r.request().id() == p.request.id());
            if !p.retired && !pending {
                p.cancel.store(true, Ordering::Release);
                self.outbox.push_back(Work::Retire(p.token));
                p.retired = true;
            }
        }
        #[cfg(test)]
        let dispatch = !self.hold_start;
        #[cfg(not(test))]
        let dispatch = true;
        if !self.waiting_close && dispatch {
            if let Some(command) = self.operations.next_command() {
                match command {
                    Command::Start(request) => self.outbox.push_back(Work::Start(request)),
                    Command::Cancel(id) => {
                        if let Some(p) = self.payloads.iter().find(|p| p.request.id() == id) {
                            p.cancel.store(true, Ordering::Release);
                        }
                    }
                }
            }
        }
        // Only acknowledgement removes retained metadata. A dismissed terminal's pin is already retired.
        self.payloads.retain(|p| {
            !p.retired
                || self
                    .operations
                    .history()
                    .iter()
                    .any(|r| r.request().id() == p.request.id())
        });
        assert!(self.outbox.len() <= 34);
        if let Some(w) = &self.worker {
            while let Some(work) = self.outbox.pop_front() {
                match w.tx.try_send(work) {
                    Ok(()) => {}
                    Err(TrySendError::Full(work)) => {
                        self.outbox.push_front(work);
                        break;
                    }
                    Err(TrySendError::Disconnected(_)) => {
                        self.outbox.clear();
                        break;
                    }
                }
            }
        }
        if self.cleanup_acknowledged && self.worker.as_ref().is_some_and(|w| w.thread.is_finished())
        {
            self.worker
                .take()
                .unwrap()
                .thread
                .join()
                .expect("Creation worker panicked");
        }
        completed
    }
    pub fn needs_tick(&self) -> bool {
        self.worker.is_some()
    }
    #[cfg(test)]
    pub fn inject_mismatched_start(&mut self, destination: PathBuf) {
        assert!(destination.starts_with(self.owned_root.as_ref().unwrap()));
        let Some(Command::Start(request)) = self.operations.next_command() else {
            panic!("expected owned queued Start");
        };
        let altered =
            Request::new(request.id(), request.kind(), vec![], Some(destination)).unwrap();
        self.outbox.push_back(Work::Start(altered));
    }
    #[cfg(test)]
    pub fn production_disabled(&self) -> bool {
        !self.enabled && self.worker.is_none()
    }
}
impl Drop for CreateAdapter {
    fn drop(&mut self) {
        // Unexpected host teardown: disconnect, signal cancellation, detach. Never UI-join live I/O.
        if let Some(worker) = &self.worker {
            worker.stopping.store(true, Ordering::Release);
        }
        for p in &self.payloads {
            p.cancel.store(true, Ordering::Release);
        }
    }
}
