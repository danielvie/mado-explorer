//! All process/provider work is confined to the fixed worker. No running-then-assign.
use crate::{Failure, protocol::HELPER_CAP};
#[cfg(test)]
pub(crate) static PENDING: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static CHILDREN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]
pub(crate) static PEAK_CHILDREN: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
use mado_pdf_preview_service::{CpuBudget, CpuReservation};
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, path::Path, ptr};
use windows_sys::Win32::{
    Foundation::*,
    Security::{
        Cryptography::{BCRYPT_USE_SYSTEM_PREFERRED_RNG, BCryptGenRandom},
        SECURITY_ATTRIBUTES,
    },
    Storage::FileSystem::*,
    System::{IO::*, JobObjects::*, Pipes::*, Threading::*},
};

pub(crate) struct Handle(pub HANDLE);
// SAFETY: kernel handles are transferable; ownership is exclusive and close occurs once.
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}
impl Handle {
    fn checked(h: HANDLE) -> Result<Self, Failure> {
        if h.is_null() || h == INVALID_HANDLE_VALUE {
            Err(last_setup())
        } else {
            Ok(Self(h))
        }
    }
    pub fn event() -> Result<Self, Failure> {
        unsafe { Self::checked(CreateEventW(ptr::null(), 1, 0, ptr::null())) }
    }
    pub fn signal(&self) {
        unsafe {
            SetEvent(self.0);
        }
    }
    pub fn reset(&self) {
        unsafe {
            ResetEvent(self.0);
        }
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
fn last_setup() -> Failure {
    Failure::SetupFailure(unsafe { GetLastError() })
}
fn wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(Some(0)).collect()
}

struct Attributes {
    storage: Vec<usize>,
    initialized: bool,
}
impl Attributes {
    fn new() -> Result<Self, Failure> {
        let mut size = 0;
        unsafe {
            InitializeProcThreadAttributeList(ptr::null_mut(), 2, 0, &mut size);
        }
        if size == 0 || size > 64 * 1024 {
            return Err(Failure::SetupFailure(87));
        }
        let mut a = Self {
            storage: vec![0; size.div_ceil(size_of::<usize>())],
            initialized: false,
        };
        if unsafe { InitializeProcThreadAttributeList(a.raw(), 2, 0, &mut size) } == 0 {
            return Err(last_setup());
        }
        a.initialized = true;
        Ok(a)
    }
    fn raw(&mut self) -> LPPROC_THREAD_ATTRIBUTE_LIST {
        self.storage.as_mut_ptr() as _
    }
    fn set(&mut self, kind: u32, handles: &mut [HANDLE]) -> Result<(), Failure> {
        if unsafe {
            UpdateProcThreadAttribute(
                self.raw(),
                0,
                kind as usize,
                handles.as_mut_ptr() as _,
                size_of_val(handles),
                ptr::null_mut(),
                ptr::null(),
            )
        } == 0
        {
            Err(last_setup())
        } else {
            Ok(())
        }
    }
}
impl Drop for Attributes {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                DeleteProcThreadAttributeList(self.raw());
            }
        }
    }
}

fn pipes() -> Result<(Handle, Handle), Failure> {
    let mut random = [0u8; 16];
    if unsafe {
        BCryptGenRandom(
            ptr::null_mut(),
            random.as_mut_ptr(),
            random.len() as u32,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        )
    } < 0
    {
        return Err(Failure::SetupFailure(1));
    }
    let mut name = String::from("\\\\.\\pipe\\mado-p21-");
    use std::fmt::Write;
    for b in random {
        write!(&mut name, "{b:02x}").unwrap();
    }
    let name = wide(OsStr::new(&name));
    // Parent creates and opens BOTH private ends before launching. The helper
    // gets only the already-connected client handle, never the pipe name.
    let server = unsafe {
        Handle::checked(CreateNamedPipeW(
            name.as_ptr(),
            PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED | FILE_FLAG_FIRST_PIPE_INSTANCE,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            64 * 1024,
            64 * 1024,
            0,
            ptr::null(),
        ))
    }?;
    let sa = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        bInheritHandle: 1,
        lpSecurityDescriptor: ptr::null_mut(),
    };
    let client = unsafe {
        Handle::checked(CreateFileW(
            name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            0,
            &sa,
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        ))
    }?;
    // CreateFile connected already, so this cannot need an outstanding connect.
    if unsafe { ConnectNamedPipe(server.0, ptr::null_mut()) } == 0
        && unsafe { GetLastError() } != ERROR_PIPE_CONNECTED
    {
        return Err(last_setup());
    }
    Ok((server, client))
}

pub(crate) struct Child {
    pipe: Handle,
    process: Handle,
    job: Handle,
    pub(crate) before_resume_verified: bool,
    pub(crate) output: Option<CpuReservation>,
    // Last field, refunded only after Drop has terminated/reaped the child.
    _charge: CpuReservation,
}
impl Child {
    pub(crate) fn launch(
        executable: &Path,
        budget: &CpuBudget,
        mut current: impl FnMut() -> bool,
        verified: impl FnOnce(&Child, HANDLE) -> Result<(), Failure>,
    ) -> Result<Self, Failure> {
        if !current() {
            return Err(Failure::Cancelled);
        }
        // EVERY launch, including private native probes, reserves both complete
        // ceilings on this same counter before opening/setup/creation resources.
        let charge = crate::charged(budget.reserve_capacity(HELPER_CAP))?;
        let output = crate::charged(budget.reserve_capacity(crate::OUTPUT_CAP))?;
        // Read-only trusted deployment pin. Admission refuses device/pipe/reparse
        // executable spellings, and sharing denies executable replacement.
        let units: Vec<u16> = executable.as_os_str().encode_wide().collect();
        let _executable_pin = crate::source::open(&units).map_err(|code| {
            if code == 2 || code == 3 {
                Failure::MissingHelper
            } else {
                Failure::SetupFailure(code)
            }
        })?;
        let (pipe, client) = pipes()?;
        let job = unsafe { Handle::checked(CreateJobObjectW(ptr::null(), ptr::null())) }?;
        let limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
            BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION {
                LimitFlags: JOB_OBJECT_LIMIT_PROCESS_MEMORY
                    | JOB_OBJECT_LIMIT_JOB_MEMORY
                    | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
                    | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
                ActiveProcessLimit: 1,
                ..Default::default()
            },
            ProcessMemoryLimit: HELPER_CAP,
            JobMemoryLimit: HELPER_CAP,
            ..Default::default()
        };
        if unsafe {
            SetInformationJobObject(
                job.0,
                JobObjectExtendedLimitInformation,
                &limits as *const _ as _,
                size_of_val(&limits) as u32,
            )
        } == 0
        {
            return Err(last_setup());
        }
        let mut attrs = Attributes::new()?;
        let mut inherited = [client.0];
        let mut jobs = [job.0];
        attrs.set(PROC_THREAD_ATTRIBUTE_HANDLE_LIST, &mut inherited)?;
        attrs.set(PROC_THREAD_ATTRIBUTE_JOB_LIST, &mut jobs)?;
        let mut startup = STARTUPINFOEXW::default();
        startup.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
        startup.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
        startup.StartupInfo.hStdInput = client.0;
        startup.StartupInfo.hStdOutput = client.0;
        startup.StartupInfo.hStdError = client.0;
        startup.lpAttributeList = attrs.raw();
        let executable_wide = wide(executable.as_os_str());
        // Fixed executable, no shell and no caller path in command line.
        // Build from native units, never display() which can replace surrogates.
        let mut command = Vec::with_capacity(executable_wide.len() + 64);
        command.push(34);
        command.extend_from_slice(&executable_wide[..executable_wide.len() - 1]);
        command.extend(format!("\" --ipc {}", client.0 as usize).encode_utf16());
        command.push(0);
        let environment = [0u16, 0];
        let mut process = PROCESS_INFORMATION::default();
        // SAFETY: all attribute lists, handle arrays and UTF-16 buffers stay alive
        // through the call. Job assignment occurs INSIDE process creation, and
        // the primary thread is suspended before any helper code can execute.
        if !current() {
            return Err(Failure::Cancelled);
        }
        if unsafe {
            CreateProcessW(
                executable_wide.as_ptr(),
                command.as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                1,
                EXTENDED_STARTUPINFO_PRESENT
                    | CREATE_SUSPENDED
                    | CREATE_NO_WINDOW
                    | CREATE_UNICODE_ENVIRONMENT,
                environment.as_ptr() as _,
                ptr::null(),
                &startup.StartupInfo,
                &mut process,
            )
        } == 0
        {
            return Err(last_setup());
        }
        #[cfg(test)]
        {
            let active = CHILDREN.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            PEAK_CHILDREN.fetch_max(active, std::sync::atomic::Ordering::SeqCst);
        }
        let thread = Handle(process.hThread);
        let mut child = Self {
            pipe,
            process: Handle(process.hProcess),
            job,
            before_resume_verified: false,
            output: Some(output),
            _charge: charge,
        };
        let mut contained = 0;
        let mut actual = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        if unsafe { IsProcessInJob(child.process.0, child.job.0, &mut contained) } == 0
            || contained == 0
            || unsafe {
                QueryInformationJobObject(
                    child.job.0,
                    JobObjectExtendedLimitInformation,
                    &mut actual as *mut _ as _,
                    size_of_val(&actual) as u32,
                    ptr::null_mut(),
                )
            } == 0
        {
            return Err(last_setup());
        }
        if actual.BasicLimitInformation.LimitFlags != limits.BasicLimitInformation.LimitFlags
            || actual.BasicLimitInformation.ActiveProcessLimit != 1
            || actual.ProcessMemoryLimit != HELPER_CAP
            || actual.JobMemoryLimit != HELPER_CAP
        {
            return Err(Failure::SetupFailure(87));
        }
        child.before_resume_verified = true;
        verified(&child, thread.0)?;
        if !current() {
            return Err(Failure::Cancelled);
        }
        if unsafe { ResumeThread(thread.0) } != 1 {
            return Err(last_setup());
        }
        // The parent copy of client closes now, making EOF meaningful.
        Ok(child)
    }
    pub(crate) fn wait_exit(
        &self,
        wake: &Handle,
        mut current: impl FnMut() -> bool,
    ) -> Result<u32, Failure> {
        loop {
            if !current() {
                self.terminate();
                return Err(Failure::Cancelled);
            }
            let handles = [self.process.0, wake.0];
            let result = unsafe { WaitForMultipleObjects(2, handles.as_ptr(), 0, INFINITE) };
            if result == WAIT_OBJECT_0 {
                return self.exit_code();
            }
            if result == WAIT_OBJECT_0 + 1 {
                wake.reset();
                continue;
            }
            return Err(last_setup());
        }
    }
    pub(crate) fn exit_code(&self) -> Result<u32, Failure> {
        if unsafe { WaitForSingleObject(self.process.0, INFINITE) } != WAIT_OBJECT_0 {
            return Err(last_setup());
        }
        let mut code = 0;
        if unsafe { GetExitCodeProcess(self.process.0, &mut code) } == 0 {
            Err(last_setup())
        } else {
            Ok(code)
        }
    }
    pub(crate) fn terminate(&self) {
        unsafe {
            TerminateJobObject(self.job.0, 995);
            // Also target our owned process directly. A containment-verification
            // failure must reap its still-suspended child even if Job membership
            // cannot be confirmed. Neither call is completion acknowledgement.
            TerminateProcess(self.process.0, 995);
        }
    }
    #[cfg(test)]
    pub(crate) fn peak_commit(&self) -> Result<usize, Failure> {
        let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        if unsafe {
            QueryInformationJobObject(
                self.job.0,
                JobObjectExtendedLimitInformation,
                &mut info as *mut _ as _,
                size_of_val(&info) as u32,
                ptr::null_mut(),
            )
        } == 0
        {
            Err(last_setup())
        } else {
            Ok(info.PeakJobMemoryUsed)
        }
    }
    pub(crate) fn transfer(
        &self,
        buffer: &mut [u8],
        write: bool,
        wake: &Handle,
        mut current: impl FnMut() -> bool,
    ) -> Result<usize, Failure> {
        if !current() {
            return Err(Failure::Cancelled);
        }
        let event = Handle::event()?;
        let mut pending = Pending {
            pipe: self.pipe.0,
            overlapped: Box::new(OVERLAPPED {
                hEvent: event.0,
                ..Default::default()
            }),
            active: false,
        };
        let mut n = 0;
        // Bound each native operation/pipe quota expansion and MDL, while reading
        // pixels directly into their already leased destination slice.
        let length = buffer.len().min(crate::protocol::IO_CHUNK);
        let buffer = &mut buffer[..length];
        // SAFETY: buffer is borrowed for the entire operation including Pending's
        // Drop; boxed OVERLAPPED/event stay at stable addresses through native ack.
        let success = unsafe {
            if write {
                WriteFile(
                    self.pipe.0,
                    buffer.as_ptr(),
                    buffer.len() as u32,
                    &mut n,
                    &mut *pending.overlapped,
                )
            } else {
                ReadFile(
                    self.pipe.0,
                    buffer.as_mut_ptr(),
                    buffer.len() as u32,
                    &mut n,
                    &mut *pending.overlapped,
                )
            }
        };
        if success != 0 {
            return Ok(n as usize);
        }
        let error = unsafe { GetLastError() };
        if !write && error == ERROR_BROKEN_PIPE {
            return Ok(0);
        }
        if error != ERROR_IO_PENDING {
            return Err(Failure::IpcFailure(error));
        }
        pending.active = true;
        #[cfg(test)]
        PENDING.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        loop {
            if !current() {
                self.terminate();
                return Err(Failure::Cancelled);
            }
            let handles = [event.0, wake.0];
            let result = unsafe { WaitForMultipleObjects(2, handles.as_ptr(), 0, INFINITE) };
            if result == WAIT_OBJECT_0 {
                break;
            }
            if result == WAIT_OBJECT_0 + 1 {
                wake.reset();
                continue;
            }
            return Err(last_setup());
        }
        if unsafe { GetOverlappedResult(self.pipe.0, &*pending.overlapped, &mut n, 0) } != 0 {
            pending.disarm();
            return Ok(n as usize);
        }
        let error = unsafe { GetLastError() };
        if error != ERROR_IO_INCOMPLETE {
            pending.disarm();
        }
        if !write && error == ERROR_BROKEN_PIPE {
            Ok(0)
        } else {
            Err(Failure::IpcFailure(error))
        }
    }
    pub(crate) fn exact(
        &self,
        buffer: &mut [u8],
        write: bool,
        wake: &Handle,
        mut current: impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        let mut offset = 0;
        while offset < buffer.len() {
            let n = self.transfer(&mut buffer[offset..], write, wake, &mut current)?;
            if n == 0 {
                return Err(Failure::MalformedProtocol);
            }
            offset += n;
        }
        Ok(())
    }
}
struct Pending {
    pipe: HANDLE,
    overlapped: Box<OVERLAPPED>,
    active: bool,
}
impl Pending {
    fn disarm(&mut self) {
        self.active = false;
        #[cfg(test)]
        PENDING.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}
impl Drop for Pending {
    fn drop(&mut self) {
        if self.active {
            // Cancellation request/NOT_FOUND is NOT completion. Keep OVERLAPPED,
            // event and borrowed payload alive through GetOverlappedResult(wait).
            unsafe {
                CancelIoEx(self.pipe, &*self.overlapped);
                let mut n = 0;
                loop {
                    if GetOverlappedResult(self.pipe, &*self.overlapped, &mut n, 1) != 0
                        || GetLastError() != ERROR_IO_INCOMPLETE
                    {
                        break;
                    }
                    // A premature event cannot waive acknowledgement. No deadline
                    // or buffer refund while native still reports pending.
                    Sleep(10);
                }
            }
            self.disarm();
        }
    }
}
impl Drop for Child {
    fn drop(&mut self) {
        self.terminate();
        while unsafe { WaitForSingleObject(self.process.0, INFINITE) } != WAIT_OBJECT_0 {
            // A failed wait is not reap. Preserve ownership/charge until the
            // native process handle actually acknowledges termination.
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        // Do not refund on kill/job-close or merely on requesting cancellation.
        // Wait for the process AND actual job accounting to reach zero.
        loop {
            let mut info = JOBOBJECT_BASIC_ACCOUNTING_INFORMATION::default();
            if unsafe {
                QueryInformationJobObject(
                    self.job.0,
                    JobObjectBasicAccountingInformation,
                    &mut info as *mut _ as _,
                    size_of_val(&info) as u32,
                    ptr::null_mut(),
                )
            } != 0
                && info.ActiveProcesses == 0
            {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        #[cfg(test)]
        CHILDREN.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
#[path = "native_tests.rs"]
mod tests;

pub(crate) fn child_failure(child: &Child, error: Failure) -> Failure {
    child.terminate();
    match child.exit_code() {
        Ok(code) if code != 0 && code != 995 => Failure::ChildFailure(code),
        _ => error,
    }
}
