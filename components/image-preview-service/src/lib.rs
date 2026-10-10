//! One window worker, one hard-capped own helper, and leased CPU RGBA output.
//! Job containment is a resource limit, NOT a filesystem security sandbox.
#![cfg(windows)]
mod native;
mod protocol;
mod source;
pub use mado_pdf_preview_service::CpuBudget;
use mado_pdf_preview_service::{CpuBuffer, CpuReservation};
pub use protocol::{DIMENSION_CAP, HELPER_CAP, INPUT_CAP, OUTPUT_CAP};
use protocol::{ERROR_CAP, HEADER_SIZE, Header, PATH_UNITS};
use std::{
    collections::VecDeque,
    os::windows::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, MutexGuard, TryLockError,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread::{self, JoinHandle},
};
// ponytail: the HANDOFF peak proof assumes 16 targets and one worker/child.
// Increasing cardinality or adding retained storage requires a new budget proof.
pub const CONTROL_CAP: usize = 8 * 1024 * 1024;
pub const MAX_TARGETS: usize = 16;
static RUNNING: AtomicBool = AtomicBool::new(false);
static NEXT: AtomicU64 = AtomicU64::new(1);
fn id() -> Result<u64, Failure> {
    NEXT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .map_err(|_| Failure::IdExhausted)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Generation(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Png,
    Jpeg,
    WebP,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Busy,
    Stopped,
    UnknownTarget,
    TargetLimit,
    PathTooLong,
    InvalidPath,
    IdExhausted,
    BudgetExceeded,
    AllocationFailed,
    BufferContract,
    MissingHelper,
    SetupFailure(u32),
    IpcFailure(u32),
    MalformedProtocol,
    Unsupported,
    MalformedImage,
    Unavailable,
    Oversize,
    ChildFailure(u32),
    Cancelled,
    WorkerPanicked,
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image preview: {self:?}")
    }
}
impl std::error::Error for Failure {}
#[derive(Debug)]
pub struct Frame {
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub encoded_width: u32,
    pub encoded_height: u32,
    pub orientation: u8,
    pub file_bytes: u64,
    /// PNG/WebP return the codec's default/first still image, not playback.
    pub still_image_only: bool,
    pixels: CpuBuffer,
}
impl Frame {
    pub fn pixels(&self) -> &[u8] {
        self.pixels.bytes()
    }
    pub fn charged_bytes(&self) -> usize {
        self.pixels.capacity()
    }
}
#[derive(Debug)]
pub struct Response {
    pub target: Target,
    pub generation: Generation,
    pub result: Result<Frame, Failure>,
}
struct Request {
    target: Target,
    generation: Generation,
    path: Vec<u16>,
    #[cfg(test)]
    probe_mode: u32,
}
struct Slot {
    target: Target,
    current: Option<Generation>,
    ready: Option<Response>,
}
struct State {
    slots: Vec<Slot>,
    queue: VecDeque<Request>,
    #[cfg(test)]
    submitted_order: [Option<Target>; MAX_TARGETS],
}
struct Gate;
impl Drop for Gate {
    fn drop(&mut self) {
        RUNNING.store(false, Ordering::Release);
    }
}
struct Shared {
    state: Mutex<State>,
    stop: AtomicBool,
    wake: native::Handle,
    budget: CpuBudget,
    executable: PathBuf,
    // Drop order frees bounded storage/native event before refunding its envelope.
    _control: CpuReservation,
    _gate: Gate,
}
pub struct Service {
    shared: Arc<Shared>,
    worker: Option<JoinHandle<()>>,
}
fn charged<T>(r: Result<T, mado_pdf_preview_service::Failure>) -> Result<T, Failure> {
    r.map_err(|error| match error {
        mado_pdf_preview_service::Failure::BudgetExceeded => Failure::BudgetExceeded,
        mado_pdf_preview_service::Failure::Busy => Failure::Busy,
        mado_pdf_preview_service::Failure::AllocationFailed => Failure::AllocationFailed,
        _ => Failure::BufferContract,
    })
}
impl Service {
    /// Trusted installed OUR helper sibling. No caller-selected executable, shell,
    /// config startup or file I/O here. Helper admission/loading is worker-side.
    pub fn start(budget: CpuBudget) -> Result<Self, Failure> {
        if RUNNING
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(Failure::Busy);
        }
        let gate = Gate;
        let control = charged(budget.reserve_capacity(CONTROL_CAP))?;
        let mut executable = std::env::current_exe().map_err(|_| Failure::MissingHelper)?;
        executable.pop();
        // Cargo's test binaries live in deps, while the own helper lives alongside
        // the other component binaries. No caller input participates in this path.
        #[cfg(test)]
        if executable.file_name() == Some(std::ffi::OsStr::new("deps")) {
            executable.pop();
        }
        executable.push("mado-image-helper.exe");
        if executable.as_os_str().encode_wide().count() > PATH_UNITS {
            return Err(Failure::PathTooLong);
        }
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                slots: Vec::with_capacity(MAX_TARGETS),
                queue: VecDeque::with_capacity(MAX_TARGETS),
                #[cfg(test)]
                submitted_order: [None; MAX_TARGETS],
            }),
            stop: AtomicBool::new(false),
            wake: native::Handle::event()?,
            budget,
            executable,
            _control: control,
            _gate: gate,
        });
        let copy = shared.clone();
        let worker = thread::Builder::new()
            .name("image-preview".into())
            .stack_size(512 * 1024)
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&copy)));
                if let Err(panic) = result {
                    copy.stop.store(true, Ordering::Release);
                    let mut state = copy.state.lock().unwrap_or_else(|p| p.into_inner());
                    state.queue.clear();
                    for slot in &mut state.slots {
                        slot.ready = None;
                    }
                    drop(state);
                    std::panic::resume_unwind(panic);
                }
            })
            .map_err(|_| Failure::AllocationFailed)?;
        Ok(Self {
            shared,
            worker: Some(worker),
        })
    }
    fn state(&self) -> Result<MutexGuard<'_, State>, Failure> {
        if self.shared.stop.load(Ordering::Acquire) {
            return Err(Failure::Stopped);
        }
        match self.shared.state.try_lock() {
            Ok(s) => Ok(s),
            Err(TryLockError::WouldBlock) => Err(Failure::Busy),
            Err(TryLockError::Poisoned(_)) => Err(Failure::WorkerPanicked),
        }
    }
    pub fn open_target(&self) -> Result<Target, Failure> {
        let mut state = self.state()?;
        if state.slots.len() == MAX_TARGETS {
            return Err(Failure::TargetLimit);
        }
        let target = Target(id()?);
        state.slots.push(Slot {
            target,
            current: None,
            ready: None,
        });
        Ok(target)
    }
    /// Coalesces a pending target in its existing FIFO position. Active replacement
    /// joins the tail, allowing other targets to run. Refusal preserves old state.
    /// Only bounded lexical copies, no filesystem/process/provider waits.
    pub fn request(&self, target: Target, path: &Path) -> Result<Generation, Failure> {
        let mut state = self.state()?;
        if !state.slots.iter().any(|s| s.target == target) {
            return Err(Failure::UnknownTarget);
        }
        let n = path.as_os_str().encode_wide().take(PATH_UNITS + 1).count();
        if n > PATH_UNITS {
            return Err(Failure::PathTooLong);
        }
        let mut units = Vec::new();
        units
            .try_reserve_exact(n)
            .map_err(|_| Failure::AllocationFailed)?;
        units.extend(path.as_os_str().encode_wide());
        source::admit(&units).map_err(|_| Failure::InvalidPath)?;
        let generation = Generation(id()?);
        let request = Request {
            target,
            generation,
            path: units,
            #[cfg(test)]
            probe_mode: 0,
        };
        if let Some(old) = state.queue.iter_mut().find(|j| j.target == target) {
            *old = request;
        } else {
            state.queue.push_back(request);
        }
        let slot = state.slots.iter_mut().find(|s| s.target == target).unwrap();
        slot.current = Some(generation);
        slot.ready = None;
        #[cfg(test)]
        {
            // Capture under the submission lock, before the worker can pop FIFO.
            state.submitted_order.fill(None);
            for i in 0..state.queue.len() {
                state.submitted_order[i] = Some(state.queue[i].target);
            }
        }
        drop(state);
        self.shared.wake.signal();
        Ok(generation)
    }
    pub fn cancel(&self, target: Target) -> Result<(), Failure> {
        let mut state = self.state()?;
        let slot = state
            .slots
            .iter_mut()
            .find(|s| s.target == target)
            .ok_or(Failure::UnknownTarget)?;
        slot.current = None;
        slot.ready = None;
        state.queue.retain(|j| j.target != target);
        drop(state);
        self.shared.wake.signal();
        Ok(())
    }
    pub fn close_target(&self, target: Target) -> Result<(), Failure> {
        let mut state = self.state()?;
        let index = state
            .slots
            .iter()
            .position(|s| s.target == target)
            .ok_or(Failure::UnknownTarget)?;
        state.slots.swap_remove(index);
        state.queue.retain(|j| j.target != target);
        drop(state);
        self.shared.wake.signal();
        Ok(())
    }
    pub fn take_current(&self, target: Target) -> Result<Option<Response>, Failure> {
        let mut state = self.state()?;
        Ok(state
            .slots
            .iter_mut()
            .find(|s| s.target == target)
            .ok_or(Failure::UnknownTarget)?
            .ready
            .take())
    }
    pub fn is_current(&self, target: Target, generation: Generation) -> Result<bool, Failure> {
        let state = self.state()?;
        Ok(state
            .slots
            .iter()
            .any(|s| s.target == target && s.current == Some(generation)))
    }
    /// Signals only. No deadline or premature cleanup acknowledgement.
    pub fn stop(&self) {
        self.shared.stop.store(true, Ordering::Release);
        self.shared.wake.signal();
    }
    /// None means worker/children/IPC are NOT acknowledged gone yet.
    pub fn poll_join(&mut self) -> Option<thread::Result<()>> {
        use std::os::windows::io::AsRawHandle;
        let worker = self.worker.as_ref()?;
        // Rust's is_finished can precede final OS thread/TLS teardown. A zero-time
        // native probe requires actual thread termination before join on the UI.
        if worker.is_finished()
            && unsafe {
                windows_sys::Win32::System::Threading::WaitForSingleObject(
                    worker.as_raw_handle(),
                    0,
                )
            } == windows_sys::Win32::Foundation::WAIT_OBJECT_0
        {
            Some(self.worker.take().unwrap().join())
        } else {
            None
        }
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        // Shutdown must stop/poll first. As a final ownership safety net, dropping
        // without acknowledgement waits, rather than detach a still-charged worker.
        self.stop();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
fn current(shared: &Shared, request: &Request) -> bool {
    if shared.stop.load(Ordering::Acquire) {
        return false;
    }
    shared
        .state
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .slots
        .iter()
        .any(|s| s.target == request.target && s.current == Some(request.generation))
}
fn receive(
    child: &mut native::Child,
    request: &Request,
    wake: &native::Handle,
    mut valid: impl FnMut() -> bool,
) -> Result<Frame, Failure> {
    let mut header = [0; HEADER_SIZE];
    child.exact(&mut header, false, wake, &mut valid)?;
    let h = Header::parse(&header, request.target.0, request.generation.0)
        .map_err(|_| Failure::MalformedProtocol)?;
    if h.status != 0 {
        let mut error = [0; ERROR_CAP];
        child.exact(&mut error[..h.error_len as usize], false, wake, &mut valid)?;
        // Wire errors are bounded and validated, but not stored as child-controlled
        // strings in the parent. Native exit is reported separately.
        if std::str::from_utf8(&error[..h.error_len as usize]).is_err() {
            return Err(Failure::MalformedProtocol);
        }
        eof(child, wake, &mut valid)?;
        return Err(match h.status {
            1 => Failure::Unsupported,
            2 => Failure::MalformedImage,
            3 => Failure::Unavailable,
            4 => Failure::Oversize,
            _ => Failure::AllocationFailed,
        });
    }
    // Consume the output reservation AFTER validated size; direct native read into
    // exact-capacity leased storage. No uncharged intermediate pixel allocation.
    let output = child.output.take().ok_or(Failure::BufferContract)?;
    let mut pixels = charged(output.into_buffer(h.payload as usize))?;
    child.exact(pixels.bytes_mut(), false, wake, &mut valid)?;
    eof(child, wake, &mut valid)?;
    if !valid() {
        return Err(Failure::Cancelled);
    }
    Ok(Frame {
        format: match h.format {
            1 => Format::Png,
            2 => Format::Jpeg,
            _ => Format::WebP,
        },
        width: h.width,
        height: h.height,
        encoded_width: h.encoded_width,
        encoded_height: h.encoded_height,
        orientation: h.orientation as u8,
        file_bytes: h.file_bytes,
        still_image_only: h.format != 2,
        pixels,
    })
}
fn eof(
    child: &native::Child,
    wake: &native::Handle,
    mut valid: impl FnMut() -> bool,
) -> Result<(), Failure> {
    let mut extra = [0];
    if child.transfer(&mut extra, false, wake, &mut valid)? != 0 {
        return Err(Failure::MalformedProtocol);
    }
    let code = child.wait_exit(wake, valid)?;
    if code == 0 {
        Ok(())
    } else {
        Err(Failure::ChildFailure(code))
    }
}
fn decode(shared: &Shared, request: &Request) -> Result<Frame, Failure> {
    if !current(shared, request) {
        return Err(Failure::Cancelled);
    }
    let mut child = native::Child::launch(
        &shared.executable,
        &shared.budget,
        || current(shared, request),
        |_, _| Ok(()),
    )?;
    #[cfg(test)]
    let mode = request.probe_mode;
    #[cfg(not(test))]
    let mode = 0;
    let mut wire = protocol::request(
        request.target.0,
        request.generation.0,
        &request.path,
        mode,
        0,
    );
    let result = child
        .exact(&mut wire, true, &shared.wake, || current(shared, request))
        .and_then(|_| {
            receive(&mut child, request, &shared.wake, || {
                current(shared, request)
            })
        });
    result.map_err(|error| native::child_failure(&child, error))
    // Child Drop terminates/reaps BEFORE another request can launch. Its full
    // reservation outlives every IPC operation; output lives with the Frame.
}
fn run(shared: &Shared) {
    loop {
        let request = {
            let mut state = shared.state.lock().unwrap_or_else(|p| p.into_inner());
            if shared.stop.load(Ordering::Acquire) {
                state.queue.clear();
                state.slots.clear();
                return;
            }
            state.queue.pop_front()
        };
        let Some(request) = request else {
            unsafe {
                windows_sys::Win32::System::Threading::WaitForSingleObject(shared.wake.0, 50);
            }
            shared.wake.reset();
            continue;
        };
        if !current(shared, &request) {
            continue;
        }
        let result = decode(shared, &request);
        let mut state = shared.state.lock().unwrap_or_else(|p| p.into_inner());
        if !shared.stop.load(Ordering::Acquire) {
            if let Some(slot) = state
                .slots
                .iter_mut()
                .find(|s| s.target == request.target && s.current == Some(request.generation))
            {
                slot.ready = Some(Response {
                    target: request.target,
                    generation: request.generation,
                    result,
                });
            }
        }
    }
}
#[cfg(test)]
mod tests;
