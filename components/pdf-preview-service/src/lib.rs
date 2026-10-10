//! One window-owned PDF worker. No GUI adapter or image decoder.
use mado_pdf_preview::{self as pdf, RenderRequest, RenderedPage, Renderer};
use std::collections::VecDeque;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, TryLockError};
use std::thread::{self, JoinHandle};

pub const CPU_BUFFER_LIMIT: usize = 256 * 1024 * 1024;
/// Covers two capped source allocations during growth plus the 16 KiB read chunk.
/// Short reads can leave the old allocation nearly as large as the input cap.
pub const SOURCE_RESERVATION: usize = 2 * pdf::MAX_INPUT_BYTES + 16 * 1024;
pub const MAX_TARGETS: usize = 16;
pub const MAX_PATH_BYTES: usize = 128 * 1024;
const MAX_ERROR_BYTES: usize = 2048;

/// Share ONE instance across the window's preview producers and retained buffers.
#[derive(Clone, Debug)]
pub struct CpuBudget(Arc<AtomicUsize>);
impl Default for CpuBudget {
    fn default() -> Self {
        Self::new()
    }
}
impl CpuBudget {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }
    pub fn used(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }
    fn reserve(&self, bytes: usize) -> Result<Lease, Failure> {
        let mut used = self.used();
        // Bounded contention refusal, never wait for a frame to be released.
        for _ in 0..8 {
            let next = used.checked_add(bytes).ok_or(Failure::BudgetExceeded)?;
            if next > CPU_BUFFER_LIMIT {
                return Err(Failure::BudgetExceeded);
            }
            match self
                .0
                .compare_exchange_weak(used, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => {
                    return Ok(Lease {
                        budget: self.clone(),
                        bytes,
                    });
                }
                Err(actual) => used = actual,
            }
        }
        Err(Failure::Busy)
    }
    /// Reserve capacity on this same counter without allocating a byte buffer.
    /// For external resources, the caller must retain the reservation until the
    /// resource is actually freed, including waiting for a helper's actual exit.
    pub fn reserve_capacity(&self, bytes: usize) -> Result<CpuReservation, Failure> {
        Ok(CpuReservation {
            lease: self.reserve(bytes)?,
        })
    }
    /// Optional caller-owned CPU copies. Admission precedes allocation, and the
    /// returned buffer cannot surrender its Vec. GUI-owned allocations still need
    /// integration accounting; this does not charge egui conversion/upload/cache.
    pub fn allocate(&self, bytes: usize) -> Result<CpuBuffer, Failure> {
        self.reserve_capacity(bytes)?.into_buffer(bytes)
    }
}

/// Non-cloneable capacity charge, with no byte allocation or owned buffer export.
/// Dropping it refunds the charge. Conversion consumes it exactly once:
///
/// ```compile_fail
/// let budget = mado_pdf_preview_service::CpuBudget::new();
/// let reservation = budget.reserve_capacity(64).unwrap();
/// let first = reservation.into_buffer(16).unwrap();
/// let second = reservation.into_buffer(16).unwrap(); // Already moved.
/// ```
///
/// ```compile_fail
/// let budget = mado_pdf_preview_service::CpuBudget::new();
/// let reservation = budget.reserve_capacity(64).unwrap();
/// let duplicate = reservation.clone(); // Reservations cannot be cloned.
/// ```
#[derive(Debug)]
pub struct CpuReservation {
    lease: Lease,
}
impl CpuReservation {
    pub fn charged_bytes(&self) -> usize {
        self.lease.bytes
    }
    /// Allocate a zero-filled buffer within this reservation, without admitting
    /// a second charge. Oversize returns BudgetExceeded before allocation.
    /// All errors consume and refund the reservation; success refunds only the
    /// unused capacity after the real buffer exists. This does not copy or free
    /// an external resource: it must be freed before this conversion is invoked.
    pub fn into_buffer(mut self, bytes: usize) -> Result<CpuBuffer, Failure> {
        if bytes > self.lease.bytes {
            return Err(Failure::BudgetExceeded);
        }
        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(bytes)
            .map_err(|_| Failure::AllocationFailed)?;
        pixels.resize(bytes, 0);
        // Global Vec<u8>::try_reserve_exact retains the requested capacity on the
        // supported Rust/Windows allocator. Validate that backend/stdlib contract.
        if pixels.capacity() != bytes {
            drop(pixels);
            return Err(Failure::BufferContract);
        }
        self.lease.shrink(pixels.capacity());
        Ok(CpuBuffer {
            pixels: Some(pixels),
            lease: self.lease,
        })
    }
}

#[derive(Debug)]
struct Lease {
    budget: CpuBudget,
    bytes: usize,
}
impl Lease {
    fn shrink(&mut self, bytes: usize) {
        assert!(bytes <= self.bytes);
        self.budget
            .0
            .fetch_sub(self.bytes - bytes, Ordering::AcqRel);
        self.bytes = bytes;
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        let previous = self.budget.0.fetch_sub(self.bytes, Ordering::AcqRel);
        assert!(previous >= self.bytes, "preview budget underflow");
    }
}

/// Non-cloneable allocation with a lifetime charge. No owned/unleased Vec export.
#[derive(Debug)]
pub struct CpuBuffer {
    pixels: Option<Vec<u8>>,
    lease: Lease,
}
impl CpuBuffer {
    pub fn bytes(&self) -> &[u8] {
        self.pixels.as_deref().unwrap()
    }
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.pixels.as_deref_mut().unwrap()
    }
    pub fn capacity(&self) -> usize {
        self.pixels.as_ref().unwrap().capacity()
    }
}
impl Drop for CpuBuffer {
    fn drop(&mut self) {
        // Free the actual allocation BEFORE the lease field releases its charge.
        drop(self.pixels.take());
    }
}

#[derive(Debug)]
pub struct Frame {
    pub document: pdf::DocumentMetadata,
    pub page: pdf::PageMetadata,
    pub width: u32,
    pub height: u32,
    pixels: CpuBuffer,
}
impl Frame {
    pub fn pixels(&self) -> &[u8] {
        self.pixels.bytes()
    }
    pub fn charged_bytes(&self) -> usize {
        self.pixels.lease.bytes
    }
}

/// Opaque, process-unique, never reused, including across service recreation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Generation(u64);
static NEXT_ID: AtomicU64 = AtomicU64::new(1);
fn fresh() -> Result<u64, Failure> {
    let mut next = NEXT_ID.load(Ordering::Relaxed);
    for _ in 0..8 {
        let after = next.checked_add(1).ok_or(Failure::IdExhausted)?;
        match NEXT_ID.compare_exchange_weak(next, after, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return Ok(next),
            Err(actual) => next = actual,
        }
    }
    Err(Failure::Busy)
}

#[derive(Clone, Copy, Debug)]
pub struct Request {
    pub page_index: u32,
    /// Physical pixels including caller display scaling and zoom.
    pub width: u32,
    pub height: u32,
}
impl Request {
    fn output_bytes(self) -> Result<usize, Failure> {
        if self.width == 0
            || self.height == 0
            || self.width > pdf::MAX_DIMENSION
            || self.height > pdf::MAX_DIMENSION
            || self.page_index > i32::MAX as u32
        {
            return Err(Failure::InvalidRequest);
        }
        let bytes = (self.width as usize)
            .checked_mul(self.height as usize)
            .and_then(|n| n.checked_mul(4))
            .ok_or(Failure::InvalidRequest)?;
        if bytes > pdf::MAX_RGBA_BYTES {
            return Err(Failure::OutputTooLarge);
        }
        Ok(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendState {
    RendererUnavailable,
    PasswordRequired,
    MalformedDocument,
    ResourceLimit,
    Io,
    Other,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    Busy,
    Stopped,
    UnknownTarget,
    TargetLimit,
    PathTooLong,
    InvalidRequest,
    OutputTooLarge,
    BudgetExceeded,
    AllocationFailed,
    BufferContract,
    IdExhausted,
    Backend {
        state: BackendState,
        message: String,
    },
}
impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend { message, .. } => f.write_str(message),
            other => write!(f, "PDF preview request refused: {other:?}"),
        }
    }
}
impl std::error::Error for Failure {}
fn backend_error(error: pdf::Error) -> Failure {
    let state = match &error {
        pdf::Error::RendererUnavailable { .. } | pdf::Error::RendererAlreadyLoaded { .. } => {
            BackendState::RendererUnavailable
        }
        pdf::Error::PasswordRequired => BackendState::PasswordRequired,
        pdf::Error::MalformedDocument => BackendState::MalformedDocument,
        pdf::Error::InputTooLarge
        | pdf::Error::OutputTooLarge { .. }
        | pdf::Error::AllocationFailed => BackendState::ResourceLimit,
        pdf::Error::Io { .. } | pdf::Error::NotRegularFile(_) => BackendState::Io,
        _ => BackendState::Other,
    };
    struct BoundedMessage(String);
    impl fmt::Write for BoundedMessage {
        fn write_str(&mut self, text: &str) -> fmt::Result {
            let mut end = text.len().min(MAX_ERROR_BYTES - self.0.len());
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            self.0.push_str(&text[..end]);
            Ok(())
        }
    }
    let mut message = BoundedMessage(String::with_capacity(MAX_ERROR_BYTES));
    let _ = fmt::write(&mut message, format_args!("{error}"));
    Failure::Backend {
        state,
        message: message.0,
    }
}

#[derive(Debug)]
pub struct Response {
    pub target: Target,
    pub generation: Generation,
    pub path: PathBuf,
    pub result: Result<Frame, Failure>,
}
struct Job {
    target: Target,
    generation: Generation,
    path: PathBuf,
    request: Request,
}
struct Slot {
    target: Target,
    current: Option<Generation>,
    ready: Option<Response>,
}
struct State {
    slots: Vec<Slot>,
    queue: VecDeque<Job>,
}
struct Shared {
    state: Mutex<State>,
    wake: Condvar,
    stop: AtomicBool,
    budget: CpuBudget,
}
type RenderCall = Box<dyn FnMut(&Path, RenderRequest) -> Result<RenderedPage, Failure>>;

pub struct Service {
    shared: Arc<Shared>,
    worker: Option<JoinHandle<()>>,
}
impl Service {
    /// Only captures a trusted deployment path here. Renderer.load, canonicalize,
    /// file reading and native rendering ALL happen on the single worker.
    /// This backend must be the process's sole PDFium initializer.
    pub fn start(library: &Path, budget: CpuBudget) -> Result<Self, Failure> {
        if library.as_os_str().len() > MAX_PATH_BYTES {
            return Err(Failure::PathTooLong);
        }
        let library = library.to_owned();
        Self::start_inner(budget, move || {
            let renderer = Renderer::load(&library).map_err(backend_error);
            Box::new(move |path, request| match &renderer {
                Ok(renderer) => renderer.render_file(path, request).map_err(backend_error),
                Err(error) => Err(error.clone()),
            })
        })
    }
    fn start_inner(
        budget: CpuBudget,
        make: impl FnOnce() -> RenderCall + Send + 'static,
    ) -> Result<Self, Failure> {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                slots: Vec::with_capacity(MAX_TARGETS),
                queue: VecDeque::with_capacity(MAX_TARGETS),
            }),
            wake: Condvar::new(),
            stop: AtomicBool::new(false),
            budget,
        });
        let worker_shared = shared.clone();
        let worker = thread::Builder::new()
            .name("pdf-preview".into())
            .spawn(move || run(worker_shared, make))
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
            Ok(state) => Ok(state),
            Err(TryLockError::WouldBlock) => Err(Failure::Busy),
            Err(TryLockError::Poisoned(_)) => Err(Failure::Stopped),
        }
    }
    pub fn open_target(&self) -> Result<Target, Failure> {
        let mut state = self.state()?;
        if state.slots.len() == MAX_TARGETS {
            return Err(Failure::TargetLimit);
        }
        let target = Target(fresh()?);
        state.slots.push(Slot {
            target,
            current: None,
            ready: None,
        });
        Ok(target)
    }
    /// Accepted requests alone get a fresh generation. A pending target keeps its
    /// FIFO position; an active target's replacement joins the tail behind others.
    /// Busy/refused requests leave the previous current request unchanged.
    pub fn request(
        &self,
        target: Target,
        path: &Path,
        request: Request,
    ) -> Result<Generation, Failure> {
        request.output_bytes()?;
        if path.as_os_str().len() > MAX_PATH_BYTES {
            return Err(Failure::PathTooLong);
        }
        let mut state = self.state()?;
        let slot = state
            .slots
            .iter_mut()
            .find(|s| s.target == target)
            .ok_or(Failure::UnknownTarget)?;
        let generation = Generation(fresh()?);
        slot.current = Some(generation);
        slot.ready = None;
        let job = Job {
            target,
            generation,
            path: path.to_owned(),
            request,
        };
        if let Some(pending) = state.queue.iter_mut().find(|j| j.target == target) {
            *pending = job;
        } else {
            state.queue.push_back(job);
        }
        drop(state);
        self.shared.wake.notify_one();
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
        Ok(())
    }
    /// No worker wait or I/O. Only the target's still-current response can escape.
    pub fn take(&self, target: Target) -> Result<Option<Response>, Failure> {
        let mut state = self.state()?;
        let slot = state
            .slots
            .iter_mut()
            .find(|s| s.target == target)
            .ok_or(Failure::UnknownTarget)?;
        Ok(slot.ready.take())
    }
    pub fn is_current(&self, target: Target, generation: Generation) -> Result<bool, Failure> {
        let state = self.state()?;
        Ok(state
            .slots
            .iter()
            .any(|s| s.target == target && s.current == Some(generation)))
    }
    /// Nonblocking signal. In-progress read/native calls cannot be interrupted.
    pub fn request_stop(&self) {
        self.shared.stop.store(true, Ordering::Release);
        self.shared.wake.notify_one();
    }
    /// Poll on UI; joins only after the thread has finished. Ownership is retained
    /// while None is returned. Native/OS calls may delay completion indefinitely.
    pub fn try_join(&mut self) -> Option<thread::Result<()>> {
        if self.worker.as_ref()?.is_finished() {
            Some(self.worker.take().unwrap().join())
        } else {
            None
        }
    }
    /// Blocking actual join, never call in an ordinary UI update.
    pub fn join(&mut self) -> thread::Result<()> {
        self.request_stop();
        self.worker.take().map_or(Ok(()), |worker| worker.join())
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        // No detach: window shutdown should signal/poll first to avoid blocking UI.
        let _ = self.join();
    }
}

fn render_job(shared: &Shared, job: &Job, render: &mut RenderCall) -> Result<Frame, Failure> {
    let output_bytes = job.request.output_bytes()?;
    // Single admission includes the source peak and the worst-case fitted output.
    // Output grows nowhere after the backend's exact reservation. Never wait for
    // budget: caller may need this error before releasing an older visible frame.
    let mut lease = shared.budget.reserve(
        SOURCE_RESERVATION
            .checked_add(output_bytes)
            .ok_or(Failure::BudgetExceeded)?,
    )?;
    let page = render(
        &job.path,
        RenderRequest {
            page_index: job.request.page_index,
            width: job.request.width,
            height: job.request.height,
            max_rgba_bytes: output_bytes,
        },
    )?;
    let RenderedPage {
        document,
        page: metadata,
        width,
        height,
        rgba,
    } = page;
    let size = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(4));
    if size != Some(rgba.len())
        || width == 0
        || height == 0
        || width > job.request.width
        || height > job.request.height
        || rgba.capacity() > output_bytes
        || document.path != job.path
        || document.input_bytes > pdf::MAX_INPUT_BYTES
        || document.page_count == 0
        || metadata.index != job.request.page_index
        || metadata.index >= document.page_count
    {
        // Local Vec drops before the admission lease on every error path.
        drop(rgba);
        return Err(Failure::BufferContract);
    }
    // Backend has already destroyed document/page/bitmap AND freed its source.
    lease.shrink(rgba.capacity());
    Ok(Frame {
        document,
        page: metadata,
        width,
        height,
        pixels: CpuBuffer {
            pixels: Some(rgba),
            lease,
        },
    })
}
fn run(shared: Arc<Shared>, make: impl FnOnce() -> RenderCall) {
    let mut render = make();
    loop {
        let job = {
            let mut state = shared.state.lock().unwrap();
            loop {
                if shared.stop.load(Ordering::Acquire) {
                    state.queue.clear();
                    state.slots.clear();
                    return;
                }
                if let Some(job) = state.queue.pop_front() {
                    break job;
                }
                // Stop is an atomic nonblocking signal, so it may race the sleep.
                // Timed worker-side sleep prevents a lost stop notification.
                state = shared
                    .wake
                    .wait_timeout(state, std::time::Duration::from_millis(50))
                    .unwrap()
                    .0;
            }
        };
        let result = render_job(&shared, &job, &mut render);
        let mut state = shared.state.lock().unwrap();
        if !shared.stop.load(Ordering::Acquire) {
            if let Some(slot) = state
                .slots
                .iter_mut()
                .find(|s| s.target == job.target && s.current == Some(job.generation))
            {
                slot.ready = Some(Response {
                    target: job.target,
                    generation: job.generation,
                    path: job.path,
                    result,
                });
            }
        }
        // Unpublished stale/error buffers and leases drop here exactly once.
    }
}

#[cfg(test)]
mod tests;
