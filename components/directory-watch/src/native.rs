use super::*;
use std::{
    cell::UnsafeCell,
    mem::{ManuallyDrop, MaybeUninit, size_of},
    os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle},
    panic::{AssertUnwindSafe, catch_unwind},
    ptr,
};
use windows_sys::Win32::{Foundation::*, Storage::FileSystem::*, System::IO::*};

type Key = Vec<u16>;

pub(crate) struct Port(OwnedHandle);
// SAFETY: an IOCP handle supports concurrent PostQueuedCompletionStatus. Arc ownership
// keeps it open through every post and the driver's drain, including Service::Drop.
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
impl Port {
    fn new() -> Result<Self, Failure> {
        // SAFETY: INVALID_HANDLE_VALUE creates a new, unassociated completion port.
        let raw = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 1) };
        if raw.is_null() {
            return Err(last("CreateIoCompletionPort"));
        }
        Ok(Self(unsafe { OwnedHandle::from_raw_handle(raw) }))
    }
    pub(crate) fn wake(&self) -> Result<(), u32> {
        // SAFETY: port is Arc-owned and live. Key zero/null OVERLAPPED is control only.
        if unsafe { PostQueuedCompletionStatus(self.0.as_raw_handle(), 0, 0, ptr::null()) } == 0 {
            Err(unsafe { GetLastError() })
        } else {
            Ok(())
        }
    }
}
fn last(action: &'static str) -> Failure {
    Failure::Native {
        action,
        code: unsafe { GetLastError() },
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Identity {
    volume: u64,
    id: [u8; 16],
}
// DWORD alignment is required by ReadDirectoryChangesW. Both allocations stay stable
// until a submitted completion has been dequeued, not merely CancelIoEx acknowledged.
#[repr(C)]
struct Storage {
    overlapped: OVERLAPPED,
    buffer: [u32; 4096],
}
struct Request {
    handle: ManuallyDrop<OwnedHandle>,
    storage: ManuallyDrop<Box<UnsafeCell<Storage>>>,
    identity: Identity,
    key: Key,
    pending: bool,
    retiring: bool,
}
impl Drop for Request {
    fn drop(&mut self) {
        if self.pending {
            // Last-resort unwind safety only. A broken driver cannot prove completion.
            // Retain this bounded handle+allocation rather than free kernel-visible memory.
            // The driver reports failure and MUST NOT acknowledge cleanup in this case.
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.storage);
            ManuallyDrop::drop(&mut self.handle);
        }
    }
}
impl Request {
    fn new(handle: OwnedHandle, identity: Identity, key: Key) -> Self {
        Self {
            handle: ManuallyDrop::new(handle),
            storage: ManuallyDrop::new(Box::new(UnsafeCell::new(Storage {
                overlapped: unsafe { std::mem::zeroed() },
                buffer: [0; 4096],
            }))),
            identity,
            key,
            pending: false,
            retiring: false,
        }
    }
    fn submit(&mut self) -> Result<(), Failure> {
        assert!(!self.pending && !self.retiring);
        // SAFETY: prior completion was drained. Both buffer and OVERLAPPED are stable and
        // correctly aligned. No completion routine; IOCP owns acknowledgement. Never recursive.
        let storage = self.storage.get();
        let ok = unsafe {
            (*storage).overlapped = std::mem::zeroed();
            ReadDirectoryChangesW(
                self.handle.as_raw_handle(),
                ptr::addr_of_mut!((*storage).buffer).cast(),
                size_of::<[u32; 4096]>() as u32,
                0,
                FILE_NOTIFY_CHANGE_FILE_NAME
                    | FILE_NOTIFY_CHANGE_DIR_NAME
                    | FILE_NOTIFY_CHANGE_ATTRIBUTES
                    | FILE_NOTIFY_CHANGE_SIZE
                    | FILE_NOTIFY_CHANGE_LAST_WRITE
                    | FILE_NOTIFY_CHANGE_CREATION,
                ptr::null_mut(),
                ptr::addr_of_mut!((*storage).overlapped),
                None,
            )
        };
        if ok == 0 {
            let code = unsafe { GetLastError() };
            // IO_PENDING is an accepted overlapped submission, not ownership release.
            if code != ERROR_IO_PENDING {
                return Err(Failure::Native {
                    action: "ReadDirectoryChangesW",
                    code,
                });
            }
        }
        self.pending = true;
        Ok(())
    }
    fn retire(&mut self) -> Option<Failure> {
        self.retiring = true; // BEFORE cancellation, including already queued successes.
        if !self.pending {
            return None;
        }
        // SAFETY: handle/storage remain owned until the completion is dequeued. Neither
        // success nor ERROR_NOT_FOUND acknowledges a completion; pending stays true.
        if unsafe {
            CancelIoEx(
                self.handle.as_raw_handle(),
                ptr::addr_of!((*self.storage.get()).overlapped),
            )
        } == 0
        {
            let code = unsafe { GetLastError() };
            if code != ERROR_NOT_FOUND {
                return Some(Failure::Native {
                    action: "CancelIoEx",
                    code,
                });
            }
        }
        None
    }
}

/// Opens only a losslessly admitted filesystem namespace. No enumeration/content reads.
fn observe(key: &[u16], stop: &AtomicBool) -> Result<Option<(OwnedHandle, Identity)>, Failure> {
    if stop.load(Ordering::Acquire) {
        return Ok(None);
    }
    // SAFETY: key is terminated and validated before I/O. Share delete is needed for root lifecycle.
    let raw = unsafe {
        CreateFileW(
            key.as_ptr(),
            FILE_LIST_DIRECTORY | FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED,
            ptr::null_mut(),
        )
    };
    if raw == INVALID_HANDLE_VALUE {
        return Err(last("CreateFileW"));
    }
    let handle = unsafe { OwnedHandle::from_raw_handle(raw) };
    if stop.load(Ordering::Acquire) {
        return Ok(None);
    }
    if unsafe { GetFileType(raw) } != FILE_TYPE_DISK {
        return Err(Failure::Unsupported("not a disk filesystem directory"));
    }
    let mut id = MaybeUninit::<FILE_ID_INFO>::uninit();
    let mut attr = MaybeUninit::<FILE_ATTRIBUTE_TAG_INFO>::uninit();
    // SAFETY: typed aligned buffers have exact sizes; assume_init only after successful calls.
    unsafe {
        if stop.load(Ordering::Acquire) {
            return Ok(None);
        }
        if GetFileInformationByHandleEx(
            raw,
            FileIdInfo,
            id.as_mut_ptr().cast(),
            size_of::<FILE_ID_INFO>() as u32,
        ) == 0
        {
            return Err(last("GetFileInformationByHandleEx identity"));
        }
        if stop.load(Ordering::Acquire) {
            return Ok(None);
        }
        if GetFileInformationByHandleEx(
            raw,
            FileAttributeTagInfo,
            attr.as_mut_ptr().cast(),
            size_of::<FILE_ATTRIBUTE_TAG_INFO>() as u32,
        ) == 0
        {
            return Err(last("GetFileInformationByHandleEx attributes"));
        }
        let id = id.assume_init();
        let attr = attr.assume_init();
        if attr.FileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0
            || attr.FileAttributes & FILE_ATTRIBUTE_DEVICE != 0
        {
            return Err(Failure::Unsupported("not a directory"));
        }
        if id.FileId.Identifier == [0; 16] {
            return Err(Failure::Unsupported("empty native directory identity"));
        }
        Ok(Some((
            handle,
            Identity {
                volume: id.VolumeSerialNumber,
                id: id.FileId.Identifier,
            },
        )))
    }
}
#[derive(Default)]
struct Attachment {
    request: Option<usize>,
    error: Option<Failure>,
}
fn missing(error: &Failure) -> bool {
    matches!(error, Failure::Disappeared { .. })
        || matches!(
            error,
            Failure::Native {
                action: "CreateFileW",
                code: ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND | ERROR_DELETE_PENDING,
            }
        )
}
struct Current {
    target: Target,
    directory: Result<Key, Failure>,
    parent: Option<Result<Key, Failure>>,
    dirty: Dirty,
    overflow: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Completion {
    Retired,
    Dirty { overflow: bool },
    Failed(u32),
}
fn classify(retiring: bool, code: u32, bytes: u32) -> Completion {
    if retiring {
        Completion::Retired
    } else if code == 0 || code == ERROR_NOTIFY_ENUM_DIR {
        Completion::Dirty {
            overflow: code == ERROR_NOTIFY_ENUM_DIR || bytes == 0,
        }
    } else {
        Completion::Failed(code)
    }
}
struct Driver {
    shared: Arc<Shared>,
    port: Arc<Port>,
    revision: u64,
    current: BTreeMap<u64, Current>,
    attachments: BTreeMap<Key, Attachment>,
    requests: BTreeMap<usize, Box<Request>>,
    next_key: usize,
    #[cfg(test)]
    faults: BTreeMap<&'static str, u32>,
    #[cfg(test)]
    last_native_completion: Option<(usize, u32, u32)>,
    #[cfg(test)]
    observations: usize,
    #[cfg(test)]
    read_submissions: usize,
    #[cfg(test)]
    stop_after_initial_read: bool,
}
impl Driver {
    fn new(shared: Arc<Shared>, port: Arc<Port>) -> Self {
        Self {
            shared,
            port,
            revision: 0,
            current: BTreeMap::new(),
            attachments: BTreeMap::new(),
            requests: BTreeMap::new(),
            next_key: 1,
            #[cfg(test)]
            faults: BTreeMap::new(),
            #[cfg(test)]
            last_native_completion: None,
            #[cfg(test)]
            observations: 0,
            #[cfg(test)]
            read_submissions: 0,
            #[cfg(test)]
            stop_after_initial_read: false,
        }
    }
    fn retire(&mut self, id: usize) {
        if let Some(request) = self.requests.get_mut(&id) {
            if let Some(error) = request.retire() {
                self.shared.failure(error);
            }
            if !request.pending {
                self.requests.remove(&id);
            }
        }
    }
    fn stopping(&mut self) -> bool {
        if !self.shared.stop.load(Ordering::Acquire) {
            return false;
        }
        // Observation is cooperative, not an OS-call deadline. Once observed, discard
        // routing and retire, but keep every submitted allocation until its real drain.
        self.current.clear();
        self.attachments.clear();
        let ids: Vec<usize> = self
            .requests
            .iter()
            .filter(|(_, r)| !r.retiring)
            .map(|(&id, _)| id)
            .collect();
        for id in ids {
            self.retire(id);
        }
        true
    }
    fn paths(current: &Current) -> impl Iterator<Item = &Result<Key, Failure>> {
        std::iter::once(&current.directory).chain(current.parent.iter())
    }
    fn controls(&mut self) {
        if self.stopping() {
            return;
        }
        let (replacement, retries) = {
            let mut m = self.shared.lock();
            let replacement = if m.changed {
                m.changed = false;
                Some((m.revision, m.targets.clone()))
            } else {
                None
            };
            (replacement, std::mem::take(&mut m.retries))
        };
        if let Some((revision, targets)) = replacement {
            self.revision = revision;
            let previous = std::mem::take(&mut self.current);
            self.current = targets
                .into_iter()
                .map(|target| {
                    let directory = path::admitted_name(&target.path);
                    // Never open a parent for a rejected endpoint.
                    let parent = directory
                        .as_ref()
                        .ok()
                        .and_then(|_| target.path.parent())
                        .map(path::admitted_name);
                    let overflow = previous
                        .get(&target.id)
                        .is_some_and(|c| c.target == target && c.overflow);
                    let mut dirty = Dirty::default();
                    dirty.mark(Instant::now(), true);
                    (
                        target.id,
                        Current {
                            target,
                            directory,
                            parent,
                            dirty,
                            overflow,
                        },
                    )
                })
                .collect();
            let desired: BTreeSet<Key> = self
                .current
                .values()
                .flat_map(Self::paths)
                .filter_map(|p| p.as_ref().ok().cloned())
                .collect();
            let removed: Vec<Key> = self
                .attachments
                .keys()
                .filter(|k| !desired.contains(*k))
                .cloned()
                .collect();
            for key in removed {
                if let Some(a) = self.attachments.remove(&key) {
                    if let Some(id) = a.request {
                        self.retire(id);
                    }
                }
            }
            for key in desired {
                self.attachments.entry(key).or_default();
            }
            self.revalidate(false);
            if self.stopping() {
                return;
            }
            self.publish_all(true);
        }
        if !retries.is_empty() {
            let mut keys = BTreeSet::new();
            for id in retries {
                if let Some(c) = self.current.get_mut(&id) {
                    c.overflow = false;
                    c.dirty.mark(Instant::now(), true);
                    keys.extend(Self::paths(c).filter_map(|p| p.as_ref().ok().cloned()));
                }
            }
            for key in keys {
                if let Some(a) = self.attachments.get_mut(&key) {
                    a.error = None;
                }
            }
            self.revalidate(true);
            if self.stopping() {
                return;
            }
            self.publish_all(true);
        }
        let code = self.shared.wake_error.swap(0, Ordering::AcqRel);
        if code != 0 {
            self.shared.failure(Failure::Native {
                action: "PostQueuedCompletionStatus",
                code,
            });
        }
    }
    fn submit(&mut self, id: usize, action: &'static str) -> Result<bool, Failure> {
        if self.stopping() {
            return Ok(false);
        }
        #[cfg(test)]
        if let Some(code) = self.faults.remove(action) {
            return Err(Failure::Native { action, code });
        }
        let _ = action;
        let result = self.requests.get_mut(&id).unwrap().submit();
        #[cfg(test)]
        if result.is_ok() {
            self.read_submissions += 1;
            // Stop immediately after the first real registration read, before the batch continues.
            if action == "initial read" && std::mem::take(&mut self.stop_after_initial_read) {
                Service {
                    shared: self.shared.clone(),
                    thread: None,
                }
                .request_stop();
            }
        }
        result.map(|()| true)
    }
    fn observe(&mut self, key: &[u16]) -> Result<Option<(OwnedHandle, Identity)>, Failure> {
        #[cfg(test)]
        {
            self.observations += 1;
        }
        observe(key, &self.shared.stop)
    }
    fn attach(
        &mut self,
        key: Key,
        handle: OwnedHandle,
        identity: Identity,
    ) -> Result<Option<usize>, Failure> {
        if self.stopping() {
            return Ok(None);
        }
        if self.requests.len() >= MAX_REQUESTS {
            return Err(Failure::Capacity);
        }
        let id = self.next_key;
        self.next_key = self.next_key.checked_add(1).ok_or(Failure::Capacity)?;
        // SAFETY: both owned handles are live. Unique nonzero key identifies this allocation.
        if unsafe {
            CreateIoCompletionPort(handle.as_raw_handle(), self.port.0.as_raw_handle(), id, 0)
        }
        .is_null()
        {
            return Err(last("CreateIoCompletionPort association"));
        }
        self.requests
            .insert(id, Box::new(Request::new(handle, identity, key)));
        match self.submit(id, "initial read") {
            Ok(true) => Ok(Some(id)),
            Ok(false) => Ok(None),
            Err(error) => {
                self.requests.remove(&id);
                Err(error)
            }
        }
    }
    fn revalidate(&mut self, retry_capacity: bool) {
        let keys: Vec<Key> = self.attachments.keys().cloned().collect();
        for key in keys {
            if self.stopping() {
                return;
            }
            let a = &self.attachments[&key];
            // Fatal registration/rearm/unknown errors are sticky until an explicit retry.
            if a.error
                .as_ref()
                .is_some_and(|e| !missing(e) && !(retry_capacity && *e == Failure::Capacity))
            {
                continue;
            }
            if a.request.is_none() && self.requests.len() >= MAX_REQUESTS {
                self.attachments.get_mut(&key).unwrap().error = Some(Failure::Capacity);
                continue;
            }
            let observed = match self.observe(&key) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => {
                    self.stopping();
                    return;
                }
                Err(error) => Err(error),
            };
            if self.stopping() {
                return;
            }
            let previous = self.attachments[&key].request;
            if let (Ok((_, identity)), Some(id)) = (&observed, previous) {
                if self.requests[&id].identity == *identity {
                    continue;
                }
            }
            if let Some(id) = previous {
                self.attachments.get_mut(&key).unwrap().request = None;
                self.retire(id);
            }
            let result =
                observed.and_then(|(handle, identity)| self.attach(key.clone(), handle, identity));
            if self.stopping() {
                return;
            }
            let a = self.attachments.get_mut(&key).unwrap();
            match result {
                Ok(id) => {
                    a.request = id;
                    a.error = None;
                }
                Err(e) => {
                    a.request = None;
                    // Preserve the observed terminal lifecycle code while the name stays missing.
                    if !missing(&e) || !matches!(a.error, Some(Failure::Disappeared { .. })) {
                        a.error = Some(e);
                    }
                }
            }
            for c in self.current.values_mut() {
                if Self::paths(c).any(|p| p.as_ref().ok() == Some(&key)) {
                    c.dirty.mark(Instant::now(), true);
                }
            }
        }
    }
    fn status(&self, key: &Result<Key, Failure>) -> (bool, Option<Failure>) {
        match key {
            Err(e) => (false, Some(e.clone())),
            Ok(key) => self
                .attachments
                .get(key)
                .map(|a| (a.request.is_some(), a.error.clone()))
                .unwrap_or((false, Some(Failure::Capacity))),
        }
    }
    fn publish_all(&mut self, immediate: bool) {
        let now = Instant::now();
        let ids: Vec<u64> = self
            .current
            .iter()
            .filter(|(_, c)| immediate || c.dirty.due(now))
            .map(|(&id, _)| id)
            .collect();
        for id in ids {
            let c = &self.current[&id];
            let (ready, directory_error) = self.status(&c.directory);
            let (parent_ready, parent_error) = c
                .parent
                .as_ref()
                .map(|p| self.status(p))
                .unwrap_or((true, None));
            let report = TargetReport {
                revision: self.revision,
                target: c.target.clone(),
                reconcile: c.dirty.first.is_some(),
                full_reconcile: c.dirty.full,
                overflow: c.overflow
                    || directory_error.iter().chain(parent_error.iter()).any(|e| {
                        matches!(
                            e,
                            Failure::Native {
                                code: ERROR_NOTIFY_ENUM_DIR,
                                ..
                            }
                        )
                    }),
                ready: ready && parent_ready,
                directory_error,
                parent_error,
            };
            self.shared.publish(report);
            self.current.get_mut(&id).unwrap().dirty = Dirty::default();
        }
    }
    fn completion(&mut self, id: usize, overlapped: *mut OVERLAPPED, code: u32, bytes: u32) {
        self.stopping();
        let Some(request) = self.requests.get_mut(&id) else {
            self.shared
                .failure(Failure::Unsupported("unknown IOCP request key"));
            return;
        };
        // Storage starts with OVERLAPPED. UnsafeCell permits kernel writes without
        // creating Rust references to any pending buffer or OVERLAPPED fields.
        if !request.pending || overlapped != request.storage.get().cast::<OVERLAPPED>() {
            self.shared
                .failure(Failure::Unsupported("unexpected IOCP OVERLAPPED"));
            return;
        }
        // The kernel's sole submitted completion is now drained. Only now may ownership end.
        request.pending = false;
        let outcome = classify(request.retiring, code, bytes);
        let key = request.key.clone();
        if outcome == Completion::Retired {
            self.requests.remove(&id);
            // Capacity refusals remain visible; a caller retry is required, not silent admission.
            return;
        }
        let error = match outcome {
            Completion::Dirty { overflow } => {
                for c in self.current.values_mut() {
                    if Self::paths(c).any(|p| p.as_ref().ok() == Some(&key)) {
                        c.dirty.mark(Instant::now(), overflow);
                        c.overflow |= overflow;
                    }
                }
                // Keep successful dirty/overflow data even if this separate rearm fails.
                match self.submit(id, "rearm") {
                    Ok(true) => None,
                    Ok(false) => return,
                    Err(error) => Some(error),
                }
            }
            Completion::Failed(code) => Some(Failure::Native {
                action: "notification completion",
                code,
            }),
            Completion::Retired => unreachable!(),
        };
        if let Some(mut error) = error {
            // On Windows a deleted watched directory can complete with ACCESS_DENIED.
            // Close the drained request first, then confirm the native name is missing.
            // Never reinterpret initial/rearm errors or arbitrary unknown completion codes.
            self.retire(id);
            if self.stopping() {
                return;
            }
            if error
                == (Failure::Native {
                    action: "notification completion",
                    code: ERROR_ACCESS_DENIED,
                })
            {
                if let Err(Failure::Native { code, .. }) = self.observe(&key) {
                    if matches!(
                        code,
                        ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND | ERROR_DELETE_PENDING
                    ) {
                        error = Failure::Disappeared {
                            completion_code: ERROR_ACCESS_DENIED,
                            observation_code: code,
                        };
                    }
                }
            }
            if self.stopping() {
                return;
            }
            if let Some(a) = self.attachments.get_mut(&key) {
                a.request = None;
                a.error = Some(error);
            }
            for c in self.current.values_mut() {
                if Self::paths(c).any(|p| p.as_ref().ok() == Some(&key)) {
                    c.dirty.mark(Instant::now(), true);
                }
            }
            self.publish_all(true);
        }
        // Current named paths, not event filenames, determine disappearance/replacement.
        self.revalidate(false);
    }
    fn dequeue(&mut self, timeout: u32) {
        let mut bytes = 0;
        let mut key = 0;
        let mut ov = ptr::null_mut();
        // SAFETY: IOCP remains Arc-owned. Output pointers are writable stack storage.
        let ok = unsafe {
            GetQueuedCompletionStatus(
                self.port.0.as_raw_handle(),
                &mut bytes,
                &mut key,
                &mut ov,
                timeout,
            )
        };
        let code = if ok == 0 {
            unsafe { GetLastError() }
        } else {
            0
        };
        if !ov.is_null() {
            #[cfg(test)]
            {
                self.last_native_completion = Some((key, code, bytes));
            }
            #[cfg(test)]
            let code = self.faults.remove("completion code").unwrap_or(code);
            #[cfg(test)]
            let bytes = self.faults.remove("completion bytes").unwrap_or(bytes);
            self.completion(key, ov, code, bytes);
        } else if ok != 0 && key == 0 {
            self.shared.wake_pending.store(false, Ordering::Release);
        } else if ok == 0 && code != WAIT_TIMEOUT {
            self.shared.failure(Failure::Native {
                action: "GetQueuedCompletionStatus",
                code,
            });
        }
    }
    fn run(&mut self) {
        while !self.shared.stop.load(Ordering::Acquire) {
            self.controls();
            self.publish_all(false);
            self.dequeue(25);
        }
    }
    fn cleanup(&mut self) {
        // Clear routing FIRST. All queued successful completions are retirement-only now.
        self.current.clear();
        self.attachments.clear();
        let ids: Vec<usize> = self.requests.keys().copied().collect();
        for id in ids {
            self.retire(id);
        }
        while !self.requests.is_empty() {
            self.dequeue(25);
        }
    }
}

pub(crate) fn drive(shared: Arc<Shared>) {
    let port = match Port::new() {
        Ok(port) => Arc::new(port),
        Err(e) => {
            shared.failure(e);
            shared.stopped.store(true, Ordering::Release);
            return;
        }
    };
    shared.lock().port = Some(port.clone());
    let mut driver = Driver::new(shared.clone(), port);
    if catch_unwind(AssertUnwindSafe(|| driver.run())).is_err() {
        shared.failure(Failure::DriverPanicked);
        shared.stop.store(true, Ordering::Release);
    }
    if catch_unwind(AssertUnwindSafe(|| driver.cleanup())).is_err() {
        shared.failure(Failure::DriverPanicked);
        // Request::Drop retains any still-submitted storage. No false stopped acknowledgement.
        return;
    }
    drop(driver);
    shared.stopped.store(true, Ordering::Release);
}

#[cfg(test)]
mod tests;
