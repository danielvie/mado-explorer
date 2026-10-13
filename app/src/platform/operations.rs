//! Owned operation requests; every COM interface stays on the pumped STA worker.
use super::windows::OleGuard;
use std::{
    cell::RefCell,
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use windows::{
    Win32::{
        Foundation::E_ABORT,
        System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, CoTaskMemFree, IPersistFile},
        UI::{Shell::*, WindowsAndMessaging::*},
    },
    core::{Error, HRESULT, Interface, PCWSTR, Ref, Result, implement},
};

#[derive(Clone, Debug)]
pub enum Operation {
    Shortcut {
        source: PathBuf,
        destination: PathBuf,
    },
    PermanentDelete {
        source: PathBuf,
        confirmed: bool,
    },
    Copy {
        source: PathBuf,
        destination: PathBuf,
    },
    Move {
        source: PathBuf,
        destination: PathBuf,
    },
    Trash {
        source: PathBuf,
    },
    Create {
        directory: PathBuf,
        name: OsString,
        folder: bool,
    },
    Rename {
        source: PathBuf,
        name: OsString,
        expected: Option<crate::rename::Signature>,
    },
}
#[derive(Clone, Debug)]
pub struct OperationRequest {
    pub id: u64,
    pub items: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemStatus {
    Completed,
    Failed(String),
    Cancelled,
    NotAttempted,
}
#[derive(Clone, Debug)]
pub struct ItemResult {
    pub operation: Operation,
    pub status: ItemStatus,
    pub final_path: Option<PathBuf>,
    pub hresult: Option<i32>,
}
#[derive(Clone, Debug)]
pub struct OperationResult {
    pub id: u64,
    pub items: Vec<ItemResult>,
    pub aborted: bool,
    pub error: Option<String>,
}
/// One admitted request at a time. UI must drain its result before submitting again.
pub struct OperationWorker {
    requests: Option<SyncSender<OperationRequest>>,
    results: Receiver<OperationResult>,
    cancel: Arc<AtomicBool>,
    busy: bool,
    thread: Option<JoinHandle<()>>,
}
impl Default for OperationWorker {
    fn default() -> Self {
        Self::new()
    }
}
impl OperationWorker {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::sync_channel::<OperationRequest>(1);
        let (out, results) = mpsc::sync_channel(1);
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = cancel.clone();
        let thread = thread::spawn(move || {
            let apartment = OleGuard::initialize();
            loop {
                pump();
                match rx.recv_timeout(Duration::from_millis(15)) {
                    Ok(request) => {
                        let result = match &apartment {
                            Ok(_) => execute(request, &flag),
                            Err(error) => failed_request(request, error.to_string()),
                        };
                        if out.send(result).is_err() {
                            break;
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    Err(_) => break,
                }
            }
        });
        Self {
            requests: Some(tx),
            results,
            cancel,
            busy: false,
            thread: Some(thread),
        }
    }
    pub fn submit(&mut self, request: OperationRequest) -> std::result::Result<(), String> {
        if self.busy {
            return Err("An operation is already active".into());
        }
        if request.items.is_empty() || request.items.len() > 4096 {
            return Err("Request requires 1–4096 items".into());
        }
        self.cancel.store(false, Ordering::Release);
        self.requests
            .as_ref()
            .ok_or("Worker is closing")?
            .try_send(request)
            .map_err(|e| e.to_string())?;
        self.busy = true;
        Ok(())
    }
    pub fn try_result(&mut self) -> Option<OperationResult> {
        let result = self.results.try_recv().ok()?;
        self.busy = false;
        Some(result)
    }
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Release);
    }
    pub fn is_busy(&self) -> bool {
        self.busy
    }
    /// Call after showing Cancel/Wait/Keep-open in UI. Never blocks the UI.
    pub fn begin_shutdown(&mut self) {
        self.cancel();
        self.requests.take();
    }
    pub fn is_finished(&self) -> bool {
        self.thread.as_ref().is_none_or(JoinHandle::is_finished)
    }
}
impl Drop for OperationWorker {
    fn drop(&mut self) {
        self.begin_shutdown();
        if self.is_finished()
            && let Some(thread) = self.thread.take()
        {
            let _ = thread.join();
        } /* An active thread retains its own STA until cancellation finishes. UI normally waits before close. */
    }
}
fn pump() {
    unsafe {
        let mut msg = MSG::default();
        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
fn failed_request(request: OperationRequest, error: String) -> OperationResult {
    OperationResult {
        id: request.id,
        items: request
            .items
            .into_iter()
            .map(|operation| ItemResult {
                operation,
                status: ItemStatus::NotAttempted,
                final_path: None,
                hresult: None,
            })
            .collect(),
        aborted: false,
        error: Some(error),
    }
}
fn execute(request: OperationRequest, cancel: &Arc<AtomicBool>) -> OperationResult {
    let mut result = OperationResult {
        id: request.id,
        items: Vec::with_capacity(request.items.len()),
        aborted: false,
        error: None,
    };
    for operation in request.items {
        pump();
        let mut item = ItemResult {
            operation: operation.clone(),
            status: ItemStatus::NotAttempted,
            final_path: None,
            hresult: None,
        };
        if cancel.load(Ordering::Acquire) {
            item.status = ItemStatus::Cancelled;
            result.aborted = true;
        } else if let Err(error) = validate(&operation) {
            item.status = ItemStatus::Failed(error);
        } else {
            let record = Rc::new(RefCell::new(None));
            let sink: IFileOperationProgressSink = Sink {
                cancel: cancel.clone(),
                record: record.clone(),
                allow_permanent: matches!(
                    operation,
                    Operation::PermanentDelete {
                        confirmed: true,
                        ..
                    }
                ),
            }
            .into();
            let run = (|| -> Result<bool> {
                if cancel.load(Ordering::Acquire) {
                    return Err(Error::from_hresult(E_ABORT));
                }
                unsafe {
                    let op: IFileOperation =
                        CoCreateInstance(&FileOperation, None, CLSCTX_INPROC_SERVER)?;
                    // Never overwrite automatically. A late collision gets an explicit alternate final path.
                    op.SetOperationFlags(
                        FOF_NOERRORUI
                            | FOF_SILENT
                            | FOF_NO_CONNECTED_ELEMENTS
                            | FOFX_EARLYFAILURE
                            | FOF_RENAMEONCOLLISION
                            | if matches!(
                                operation,
                                Operation::PermanentDelete {
                                    confirmed: true,
                                    ..
                                }
                            ) {
                                FOF_NOCONFIRMATION
                            } else {
                                FOFX_RECYCLEONDELETE
                            },
                    )?;
                    match &operation {
                        Operation::Shortcut {
                            source,
                            destination,
                        } => {
                            save_shortcut(source, destination, cancel)?;
                            *record.borrow_mut() = Some((HRESULT(0), Some(destination.clone())));
                            return Ok(false);
                        }
                        Operation::PermanentDelete {
                            confirmed: false, ..
                        } => return Err(Error::from_hresult(E_ABORT)),
                        Operation::Copy {
                            source,
                            destination,
                        } => op.CopyItem(
                            &shell_item(source)?,
                            &shell_item(destination)?,
                            PCWSTR::null(),
                            &sink,
                        )?,
                        Operation::Move {
                            source,
                            destination,
                        } => op.MoveItem(
                            &shell_item(source)?,
                            &shell_item(destination)?,
                            PCWSTR::null(),
                            &sink,
                        )?,
                        Operation::Trash { source }
                        | Operation::PermanentDelete {
                            source,
                            confirmed: true,
                        } => op.DeleteItem(&shell_item(source)?, &sink)?,
                        Operation::Create {
                            directory,
                            name,
                            folder,
                        } => op.NewItem(
                            &shell_item(directory)?,
                            if *folder { 0x10 } else { 0x80 },
                            PCWSTR(wide(name).as_ptr()),
                            PCWSTR::null(),
                            &sink,
                        )?,
                        Operation::Rename { source, name, .. } => {
                            op.RenameItem(&shell_item(source)?, PCWSTR(wide(name).as_ptr()), &sink)?
                        }
                    }
                    let performed = op.PerformOperations();
                    let aborted = match op.GetAnyOperationsAborted() {
                        Ok(value) => value.as_bool(),
                        Err(error) => {
                            propagate_abort(cancel, true);
                            return Err(error);
                        }
                    };
                    propagate_abort(cancel, aborted);
                    performed?;
                    Ok(aborted)
                }
            })();
            if let Some((hr, path)) = record.borrow_mut().take() {
                item.hresult = Some(hr.0);
                item.final_path = path;
                item.status = if hr == E_ABORT
                    || hr == COPYENGINE_E_USER_CANCELLED
                    || hr.0 == 0x800704C7u32 as i32
                {
                    propagate_abort(cancel, true);
                    ItemStatus::Cancelled
                } else if callback_confirms_completion(&operation, hr, item.final_path.as_deref()) {
                    ItemStatus::Completed
                } else {
                    ItemStatus::Failed(format!("Shell item result 0x{:08X}", hr.0))
                };
            }
            match run {
                Ok(aborted) => {
                    result.aborted |= aborted;
                    if item.status == ItemStatus::NotAttempted {
                        item.status = if aborted {
                            ItemStatus::Cancelled
                        } else {
                            ItemStatus::Failed(
                                "Shell returned without an item completion callback".into(),
                            )
                        };
                    }
                }
                Err(error) => {
                    if error.code() == E_ABORT
                        || error.code() == COPYENGINE_E_USER_CANCELLED
                        || error.code().0 == 0x800704C7u32 as i32
                    {
                        propagate_abort(cancel, true);
                    }
                    result.aborted |= cancel.load(Ordering::Acquire);
                    if item.status != ItemStatus::Completed {
                        item.status = if cancel.load(Ordering::Acquire) {
                            ItemStatus::Cancelled
                        } else {
                            ItemStatus::Failed(error.to_string())
                        };
                    }
                }
            }
        }
        result.aborted |= cancel.load(Ordering::Acquire);
        result.items.push(item);
    }
    result
}
fn wide(value: &std::ffi::OsStr) -> Vec<u16> {
    value.encode_wide().chain(Some(0)).collect()
}
fn shell_item(path: &Path) -> Result<IShellItem> {
    unsafe { SHCreateItemFromParsingName(PCWSTR(wide(path.as_os_str()).as_ptr()), None) }
}
/// Validation resolves existing ancestors to catch junction/self/descendant transfers.
pub fn validate(operation: &Operation) -> std::result::Result<(), String> {
    if matches!(
        operation,
        Operation::PermanentDelete {
            confirmed: false,
            ..
        }
    ) {
        return Err("Permanent deletion requires explicit action confirmation".into());
    }
    // Do not let a NUL truncate a Shell path or relative paths depend on process CWD.
    let paths: Vec<&Path> = match operation {
        Operation::Copy {
            source,
            destination,
        }
        | Operation::Move {
            source,
            destination,
        }
        | Operation::Shortcut {
            source,
            destination,
        } => vec![source, destination],
        Operation::Create { directory, .. } => vec![directory],
        Operation::Rename { source, .. }
        | Operation::Trash { source }
        | Operation::PermanentDelete { source, .. } => vec![source],
    };
    for path in paths {
        if !path.is_absolute() || path.as_os_str().encode_wide().any(|u| u == 0) {
            return Err("Operation requires absolute paths without NUL characters".into());
        }
    }
    match operation {
        Operation::PermanentDelete {
            confirmed: false, ..
        } => return Err("Permanent deletion requires explicit action confirmation".into()),
        Operation::PermanentDelete {
            source,
            confirmed: true,
        } => {
            source.symlink_metadata().map_err(|e| e.to_string())?;
            if source
                .canonicalize()
                .map_err(|e| e.to_string())?
                .parent()
                .is_none()
            {
                return Err("Cannot permanently delete root or relative path".into());
            }
        }
        Operation::Shortcut {
            source,
            destination,
        } => {
            source.symlink_metadata().map_err(|e| e.to_string())?;
            if !source.is_absolute() || !destination.is_absolute() {
                return Err("Shortcut paths must be absolute".into());
            }
            validate_name(destination.file_name().ok_or("Missing shortcut name")?)?;
            if !destination
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("lnk"))
            {
                return Err("Shortcut requires .lnk extension".into());
            }
            if !destination.parent().is_some_and(Path::is_dir)
                || destination.try_exists().map_err(|e| e.to_string())?
            {
                return Err("Shortcut destination unavailable or already exists".into());
            }
        }
        Operation::Copy {
            source,
            destination,
        }
        | Operation::Move {
            source,
            destination,
        } => {
            let source_name = source
                .file_name()
                .ok_or("Cannot transfer a filesystem root")?;
            let source = source.canonicalize().map_err(|e| e.to_string())?;
            let dest = destination.canonicalize().map_err(|e| e.to_string())?;
            if !dest.is_dir() {
                return Err("Destination is not a directory".into());
            }
            if dest.starts_with(&source) {
                return Err("Cannot transfer into self or descendant".into());
            }
            let target = dest.join(source_name);
            if target.try_exists().map_err(|e| e.to_string())? {
                return Err("Destination already exists; replacement is not allowed".into());
            }
        }
        Operation::Create {
            directory, name, ..
        } => {
            validate_name(name)?;
            if !directory.is_dir() {
                return Err("Destination directory is unavailable".into());
            }
            if directory
                .join(name)
                .try_exists()
                .map_err(|e| e.to_string())?
            {
                return Err("Name already exists".into());
            }
        }
        Operation::Rename {
            source,
            name,
            expected,
        } => {
            validate_name(name)?;
            if let Some(expected) = expected
                && &crate::rename::Signature::read(source)? != expected
            {
                return Err("Source changed since rename preview".into());
            }
            source.symlink_metadata().map_err(|e| e.to_string())?;
            if source
                .canonicalize()
                .map_err(|e| e.to_string())?
                .parent()
                .is_none()
            {
                return Err("Cannot rename a filesystem root".into());
            }
            let parent = source.parent().ok_or("Cannot rename a root")?;
            if parent.join(name).try_exists().map_err(|e| e.to_string())? {
                return Err("Name already exists".into());
            }
        }
        Operation::Trash { source } => {
            source.symlink_metadata().map_err(|e| e.to_string())?;
            if source
                .canonicalize()
                .map_err(|e| e.to_string())?
                .parent()
                .is_none()
            {
                return Err("Cannot trash a root".into());
            }
        }
    }
    Ok(())
}
pub fn validate_name(name: &std::ffi::OsStr) -> std::result::Result<(), String> {
    let units: Vec<_> = name.encode_wide().collect();
    if units.is_empty()
        || units.len() > 255
        || units
            .iter()
            .any(|c| *c < 32 || b"<>:\"/\\|?*".contains(&(*c as u8)) && *c < 128)
        || matches!(units.last(), Some(32 | 46))
    {
        return Err("Invalid Windows entry name".into());
    }
    let text = name.to_string_lossy().to_uppercase();
    let base = text.split('.').next().unwrap_or("");
    if matches!(base, "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$")
        || ["COM", "LPT"].iter().any(|prefix| {
            base.strip_prefix(prefix).is_some_and(|n| {
                matches!(
                    n,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        })
    {
        return Err("Reserved Windows device name".into());
    }
    Ok(())
}
type Completion = Rc<RefCell<Option<(HRESULT, Option<PathBuf>)>>>;
#[implement(IFileOperationProgressSink)]
struct Sink {
    cancel: Arc<AtomicBool>,
    record: Completion,
    allow_permanent: bool,
}
impl Sink {
    fn check(&self) -> Result<()> {
        pump();
        if self.cancel.load(Ordering::Acquire) {
            Err(Error::from_hresult(E_ABORT))
        } else {
            Ok(())
        }
    }
    fn post(&self, hr: HRESULT, item: Ref<IShellItem>) -> Result<()> {
        let path = item.as_ref().and_then(|item| unsafe {
            item.GetDisplayName(SIGDN_FILESYSPATH).ok().map(|name| {
                let value = PathBuf::from(OsString::from_wide(name.as_wide()));
                CoTaskMemFree(Some(name.0.cast()));
                value
            })
        });
        *self.record.borrow_mut() = Some((hr, path));
        Ok(())
    }
}

#[allow(non_snake_case, unused_variables)]
impl IFileOperationProgressSink_Impl for Sink_Impl {
    fn StartOperations(&self) -> Result<()> {
        self.check()
    }
    fn FinishOperations(&self, hrresult: HRESULT) -> Result<()> {
        Ok(())
    }
    fn PreRenameItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        psznewname: &PCWSTR,
    ) -> Result<()> {
        self.check()
    }
    fn PostRenameItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        psznewname: &PCWSTR,
        hrrename: HRESULT,
        psinewlycreated: Ref<IShellItem>,
    ) -> Result<()> {
        self.post(hrrename, psinewlycreated)
    }
    fn PreMoveItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        psidestinationfolder: Ref<IShellItem>,
        psznewname: &PCWSTR,
    ) -> Result<()> {
        self.check()
    }
    fn PostMoveItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        psidestinationfolder: Ref<IShellItem>,
        psznewname: &PCWSTR,
        hrmove: HRESULT,
        psinewlycreated: Ref<IShellItem>,
    ) -> Result<()> {
        self.post(hrmove, psinewlycreated)
    }
    fn PreCopyItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        psidestinationfolder: Ref<IShellItem>,
        psznewname: &PCWSTR,
    ) -> Result<()> {
        self.check()
    }
    fn PostCopyItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        psidestinationfolder: Ref<IShellItem>,
        psznewname: &PCWSTR,
        hrcopy: HRESULT,
        psinewlycreated: Ref<IShellItem>,
    ) -> Result<()> {
        self.post(hrcopy, psinewlycreated)
    }
    fn PreDeleteItem(&self, dwflags: u32, psiitem: Ref<IShellItem>) -> Result<()> {
        self.check()?;
        if !self.allow_permanent && dwflags & TSF_DELETE_RECYCLE_IF_POSSIBLE.0 as u32 == 0 {
            return Err(Error::from_hresult(E_ABORT));
        }
        Ok(())
    }
    fn PostDeleteItem(
        &self,
        dwflags: u32,
        psiitem: Ref<IShellItem>,
        hrdelete: HRESULT,
        psinewlycreated: Ref<IShellItem>,
    ) -> Result<()> {
        self.post(hrdelete, psinewlycreated)
    }
    fn PreNewItem(
        &self,
        dwflags: u32,
        psidestinationfolder: Ref<IShellItem>,
        psznewname: &PCWSTR,
    ) -> Result<()> {
        self.check()
    }
    fn PostNewItem(
        &self,
        dwflags: u32,
        psidestinationfolder: Ref<IShellItem>,
        psznewname: &PCWSTR,
        psztemplatename: &PCWSTR,
        dwfileattributes: u32,
        hrnew: HRESULT,
        psinewitem: Ref<IShellItem>,
    ) -> Result<()> {
        self.post(hrnew, psinewitem)
    }
    fn UpdateProgress(&self, iworktotal: u32, iworksofar: u32) -> Result<()> {
        self.check()
    }
    fn ResetTimer(&self) -> Result<()> {
        Ok(())
    }
    fn PauseTimer(&self) -> Result<()> {
        Ok(())
    }
    fn ResumeTimer(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_entry_rejects_invalid_and_reserved_names() {
        for name in [
            "",
            ".",
            "..",
            "a/b",
            "a\\b",
            "a:b",
            "trailing.",
            "trailing ",
            "CON",
            "nul.txt",
            "COM1.txt",
            "LPT²",
        ] {
            assert!(validate_name(std::ffi::OsStr::new(name)).is_err(), "{name}");
        }
    }
    #[test]
    fn rename_entry_accepts_unicode_and_extensions() {
        for name in ["résumé 日本語.txt", "COM10", "my folder"] {
            assert!(validate_name(std::ffi::OsStr::new(name)).is_ok());
        }
    }
    #[test]
    fn operations_copy_cancel_callback() {
        let cancel = Arc::new(AtomicBool::new(true));
        let sink = Sink {
            cancel,
            record: Rc::new(RefCell::new(None)),
            allow_permanent: false,
        };
        assert_eq!(sink.check().unwrap_err().code(), E_ABORT);
    }
    #[test]
    fn operations_copy_no_completion_is_not_success() {
        let result = failed_request(
            OperationRequest {
                id: 7,
                items: vec![Operation::Trash {
                    source: PathBuf::from("C:\\owned-fixture"),
                }],
            },
            "STA unavailable".into(),
        );
        assert_eq!(result.items[0].status, ItemStatus::NotAttempted);
        assert_eq!(result.id, 7);
        assert!(result.error.is_some());
    }
    #[test]
    fn trash_denies_non_recycle_callback() {
        let sink: IFileOperationProgressSink = Sink {
            cancel: Arc::new(AtomicBool::new(false)),
            record: Rc::new(RefCell::new(None)),
            allow_permanent: false,
        }
        .into();
        assert!(unsafe { sink.PreDeleteItem(0, None) }.is_err());
    }
}
#[cfg(test)]
mod native_tests {
    use super::*;
    #[test]
    #[ignore = "Opt-in Shell mutation test; creates and removes only a unique temporary fixture"]
    fn native_operations_owned_fixture() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-operation-fixture-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let destination = root.join("destination");
        std::fs::create_dir(&destination).unwrap();
        let source = root.join("source.txt");
        std::fs::write(&source, b"owned fixture").unwrap();
        let mut worker = OperationWorker::new();
        worker
            .submit(OperationRequest {
                id: 1,
                items: vec![Operation::Copy {
                    source: source.clone(),
                    destination: destination.clone(),
                }],
            })
            .unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        let result = loop {
            if let Some(result) = worker.try_result() {
                break result;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "Shell operation timed out; fixture retained at {}",
                root.display()
            );
            std::thread::sleep(Duration::from_millis(10));
        };
        assert!(!result.aborted);
        assert_eq!(result.items[0].status, ItemStatus::Completed);
        assert_eq!(
            std::fs::read(destination.join("source.txt")).unwrap(),
            b"owned fixture"
        );
        assert!(
            validate(&Operation::Copy {
                source: source.clone(),
                destination: destination.clone()
            })
            .is_err()
        );
        assert!(
            validate(&Operation::Move {
                source: root.clone(),
                destination: destination.clone()
            })
            .is_err()
        );
        worker.begin_shutdown();
        while !worker.is_finished() {
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(10));
        }
        // Root was created uniquely by this test, never derived from application/user selection.
        std::fs::remove_file(destination.join("source.txt")).unwrap();
        std::fs::remove_file(source).unwrap();
        std::fs::remove_dir(destination).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}

/// Persists to an exclusively reserved adjacent file, then publishes via a
/// non-replacing hard link. Filesystems without hard links fail safely.
fn save_shortcut(source: &Path, destination: &Path, cancel: &AtomicBool) -> Result<()> {
    let io_error = |e: std::io::Error| Error::new(HRESULT(0x80004005u32 as i32), e.to_string());
    let temp = destination.with_file_name(format!(
        ".wayfarer-shortcut-{}-{}.tmp",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let reservation = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(io_error)?;
    drop(reservation);
    let result = (|| unsafe {
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        link.SetPath(PCWSTR(wide(source.as_os_str()).as_ptr()))?;
        let persist: IPersistFile = link.cast()?;
        persist.Save(PCWSTR(wide(temp.as_os_str()).as_ptr()), true)?;
        if cancel.load(Ordering::Acquire) {
            return Err(Error::from_hresult(E_ABORT));
        }
        std::fs::hard_link(&temp, destination).map_err(io_error)?;
        Ok(())
    })();
    let _ = std::fs::remove_file(&temp);
    result
}
#[cfg(test)]
mod extra_operation_tests {
    use super::*;
    #[test]
    fn permanent_delete_requires_confirmation() {
        assert!(
            validate(&Operation::PermanentDelete {
                source: PathBuf::from("C:\\fixture-only"),
                confirmed: false
            })
            .unwrap_err()
            .contains("confirmation")
        );
    }
}

fn propagate_abort(cancel: &AtomicBool, aborted: bool) {
    if aborted {
        cancel.store(true, Ordering::Release);
    }
}
#[cfg(test)]
mod abort_tests {
    use super::*;
    #[test]
    fn operations_copy_shell_abort_cancels_remaining_items() {
        let cancel = AtomicBool::new(false);
        propagate_abort(&cancel, true);
        assert!(cancel.load(Ordering::Acquire));
        propagate_abort(&cancel, false);
        assert!(cancel.load(Ordering::Acquire));
    }
}
#[cfg(test)]
mod native_workflow_tests {
    use super::*;
    use crate::platform::bin;
    use windows::Win32::Foundation::HWND;

    pub(super) fn submit_and_wait(
        worker: &mut OperationWorker,
        id: u64,
        operation: Operation,
    ) -> ItemResult {
        worker
            .submit(OperationRequest {
                id,
                items: vec![operation],
            })
            .unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(45);
        loop {
            pump();
            if let Some(result) = worker.try_result() {
                assert!(!result.aborted, "Operation {id} aborted: {result:?}");
                assert!(result.error.is_none(), "Operation {id} failed: {result:?}");
                assert_eq!(result.items.len(), 1);
                let item = result.items.into_iter().next().unwrap();
                assert_eq!(
                    item.status,
                    ItemStatus::Completed,
                    "Operation {id}: {item:?}"
                );
                return item;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "Operation {id} timed out; uniquely named fixture retained"
            );
            std::thread::sleep(Duration::from_millis(15));
        }
    }

    #[test]
    #[ignore = "Opt-in Shell workflow mutates only a unique temp fixture; recycles and restores its exact identified item"]
    fn native_operations_owned_workflow() {
        let _apartment = OleGuard::initialize().unwrap();
        let token = format!(
            "wayfarer-owned-workflow-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let root = std::env::temp_dir().join(&token);
        std::fs::create_dir(&root).unwrap();
        // Snapshot prevents treating any pre-existing Recycle Bin item as ours.
        let existing_ids: std::collections::HashSet<_> = bin::list()
            .unwrap()
            .into_iter()
            .map(|item| item.identity)
            .collect();
        let mut worker = OperationWorker::new();
        let folder = root.join("destination");
        let created = submit_and_wait(
            &mut worker,
            1,
            Operation::Create {
                directory: root.clone(),
                name: OsString::from("destination"),
                folder: true,
            },
        );
        assert_eq!(created.final_path.as_deref(), Some(folder.as_path()));
        assert!(folder.is_dir());
        let source = root.join("initial");
        let created = submit_and_wait(
            &mut worker,
            2,
            Operation::Create {
                directory: root.clone(),
                name: OsString::from("initial"),
                folder: false,
            },
        );
        assert_eq!(created.final_path.as_deref(), Some(source.as_path()));
        assert_eq!(std::fs::metadata(&source).unwrap().len(), 0);
        std::fs::write(&source, b"owned workflow payload").unwrap();
        let renamed = root.join(&token);
        let outcome = submit_and_wait(
            &mut worker,
            3,
            Operation::Rename {
                source: source.clone(),
                name: OsString::from(&token),
                expected: None,
            },
        );
        assert_eq!(outcome.final_path.as_deref(), Some(renamed.as_path()));
        assert!(!source.exists());
        assert_eq!(std::fs::read(&renamed).unwrap(), b"owned workflow payload");
        let moved = folder.join(&token);
        let outcome = submit_and_wait(
            &mut worker,
            4,
            Operation::Move {
                source: renamed.clone(),
                destination: folder.clone(),
            },
        );
        assert_eq!(outcome.final_path.as_deref(), Some(moved.as_path()));
        assert!(!renamed.exists());
        assert_eq!(std::fs::read(&moved).unwrap(), b"owned workflow payload");
        submit_and_wait(
            &mut worker,
            5,
            Operation::Trash {
                source: moved.clone(),
            },
        );
        assert!(!moved.exists());
        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        let identity = loop {
            pump();
            let matches: Vec<_> = bin::list()
                .unwrap()
                .into_iter()
                .filter(|item| {
                    item.display_name == token
                        && item.original_path.as_deref() == Some(folder.as_path())
                        && !existing_ids.contains(&item.identity)
                })
                .collect();
            assert!(
                matches.len() <= 1,
                "Ambiguous owned recycled identity; no restore attempted"
            );
            if let Some(item) = matches.into_iter().next() {
                break item.identity;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "Owned item not verified in Recycle Bin; no unrelated restore attempted"
            );
            std::thread::sleep(Duration::from_millis(25));
        };
        // Restore only the exact new Shell identity whose name AND original parent match.
        let restored =
            bin::restore_selection(HWND::default(), std::slice::from_ref(&identity)).unwrap();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].identity, identity);
        assert!(
            restored[0].result.is_ok(),
            "Restore result: {:?}",
            restored[0].result
        );
        assert_eq!(std::fs::read(&moved).unwrap(), b"owned workflow payload");
        assert!(
            !bin::list()
                .unwrap()
                .iter()
                .any(|item| item.identity == identity)
        );
        worker.begin_shutdown();
        while !worker.is_finished() {
            pump();
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(10));
        }
        // Exact owned files and empty directories only, never recursive/global bin deletion.
        std::fs::remove_file(moved).unwrap();
        std::fs::remove_dir(folder).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}

#[cfg(test)]
mod request_safety_tests {
    use super::*;
    #[test]
    fn operations_copy_cancelled_request_never_attempts_items() {
        let cancel = Arc::new(AtomicBool::new(true));
        let result = execute(
            OperationRequest {
                id: 99,
                items: vec![
                    Operation::Create {
                        directory: PathBuf::from("C:\\never-created-fixture"),
                        name: OsString::from("one"),
                        folder: false,
                    },
                    Operation::Create {
                        directory: PathBuf::from("C:\\never-created-fixture"),
                        name: OsString::from("two"),
                        folder: false,
                    },
                ],
            },
            &cancel,
        );
        assert!(result.aborted);
        assert!(
            result
                .items
                .iter()
                .all(|item| item.status == ItemStatus::Cancelled)
        );
    }
    #[test]
    fn operations_copy_rejects_relative_and_embedded_nul_paths() {
        for path in [
            PathBuf::from("relative"),
            PathBuf::from(OsString::from_wide(&[67, 58, 92, 97, 0, 98])),
        ] {
            assert!(
                validate(&Operation::Trash { source: path })
                    .unwrap_err()
                    .contains("absolute paths without NUL")
            );
        }
    }
}
#[cfg(test)]
mod native_extra_tests {
    use super::native_workflow_tests::submit_and_wait;
    use super::*;
    fn fixture(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-owned-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        root
    }
    fn stop(worker: &mut OperationWorker) {
        worker.begin_shutdown();
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while !worker.is_finished() {
            pump();
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    #[test]
    #[ignore = "Opt-in irreversible deletion of one uniquely owned temporary test file only"]
    fn native_permanent_delete_owned_fixture() {
        let root = fixture("permanent-delete");
        let source = root.join("owned-delete-target");
        std::fs::write(&source, b"disposable fixture only").unwrap();
        assert!(
            validate(&Operation::PermanentDelete {
                source: source.clone(),
                confirmed: false
            })
            .is_err()
        );
        assert!(source.is_file());
        let mut worker = OperationWorker::new();
        submit_and_wait(
            &mut worker,
            1,
            Operation::PermanentDelete {
                source: source.clone(),
                confirmed: true,
            },
        );
        assert!(!source.exists());
        stop(&mut worker);
        std::fs::remove_dir(root).unwrap();
    }
    #[test]
    #[ignore = "Opt-in creates and reads only an owned shortcut; never launches its target"]
    fn native_shortcut_owned_fixture() {
        let _apartment = OleGuard::initialize().unwrap();
        let root = fixture("shortcut");
        let source = root.join("owned-target.txt");
        let shortcut = root.join("owned-shortcut.lnk");
        std::fs::write(&source, b"shortcut fixture").unwrap();
        let mut worker = OperationWorker::new();
        let result = submit_and_wait(
            &mut worker,
            1,
            Operation::Shortcut {
                source: source.clone(),
                destination: shortcut.clone(),
            },
        );
        assert_eq!(result.final_path.as_deref(), Some(shortcut.as_path()));
        unsafe {
            let link: IShellLinkW =
                CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).unwrap();
            let persist: IPersistFile = link.cast().unwrap();
            persist
                .Load(
                    PCWSTR(wide(shortcut.as_os_str()).as_ptr()),
                    windows::Win32::System::Com::STGM_READ,
                )
                .unwrap();
            let mut target = vec![0u16; 32768];
            link.GetPath(&mut target, std::ptr::null_mut(), 0).unwrap();
            let end = target.iter().position(|u| *u == 0).unwrap();
            assert_eq!(PathBuf::from(OsString::from_wide(&target[..end])), source);
        }
        let original = std::fs::read(&shortcut).unwrap();
        assert!(
            validate(&Operation::Shortcut {
                source: source.clone(),
                destination: shortcut.clone()
            })
            .is_err()
        );
        assert_eq!(std::fs::read(&shortcut).unwrap(), original);
        stop(&mut worker);
        std::fs::remove_file(shortcut).unwrap();
        std::fs::remove_file(source).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}

/// Whitelist only the observed completed-item informational code. Other positive
/// codes (especially PENDING and USER_IGNORED) are not completion evidence.
fn callback_confirms_completion(
    operation: &Operation,
    hr: HRESULT,
    final_path: Option<&Path>,
) -> bool {
    if hr.0 == 0 {
        return true;
    }
    if hr != COPYENGINE_S_DONT_PROCESS_CHILDREN {
        return false;
    }
    let absent = |path: &Path| matches!(path.try_exists(), Ok(false));
    let final_exists = |parent: &Path| {
        final_path.is_some_and(|path| {
            path.try_exists().unwrap_or(false)
                && matches!((path.parent().and_then(|p| p.canonicalize().ok()), parent.canonicalize().ok()), (Some(actual), Some(expected)) if actual == expected)
        })
    };
    match operation {
        Operation::Rename { source, .. } => source
            .parent()
            .is_some_and(|parent| absent(source) && final_exists(parent)),
        Operation::Move {
            source,
            destination,
        } => absent(source) && final_exists(destination),
        // A directory copy cannot be inferred complete from its root's existence.
        Operation::Copy {
            source,
            destination,
        } => {
            source.is_file()
                && final_exists(destination)
                && final_path.is_some_and(|path| {
                    match (std::fs::metadata(source), std::fs::metadata(path)) {
                        (Ok(a), Ok(b)) => b.is_file() && a.len() == b.len(),
                        _ => false,
                    }
                })
        }
        Operation::Create {
            directory, folder, ..
        } => {
            final_exists(directory)
                && final_path.is_some_and(|path| {
                    if *folder {
                        path.is_dir()
                    } else {
                        path.is_file()
                    }
                })
        }
        Operation::Trash { source } => {
            absent(source) && final_path.is_some_and(|path| path.try_exists().unwrap_or(false))
        }
        Operation::PermanentDelete {
            source,
            confirmed: true,
        } => absent(source),
        _ => false,
    }
}
#[cfg(test)]
mod completion_tests {
    use super::*;
    #[test]
    fn operations_copy_informational_completion_requires_real_postconditions() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer-callback-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let source = root.join("before");
        let target = root.join("after");
        std::fs::write(&target, b"owned callback fixture").unwrap();
        let operation = Operation::Rename {
            source: source.clone(),
            name: OsString::from("after"),
            expected: None,
        };
        assert!(callback_confirms_completion(
            &operation,
            COPYENGINE_S_DONT_PROCESS_CHILDREN,
            Some(&target)
        ));
        assert!(!callback_confirms_completion(
            &operation,
            COPYENGINE_S_PENDING,
            Some(&target)
        ));
        assert!(!callback_confirms_completion(
            &operation,
            COPYENGINE_S_USER_IGNORED,
            Some(&target)
        ));
        assert!(!callback_confirms_completion(
            &operation,
            COPYENGINE_S_DONT_PROCESS_CHILDREN,
            None
        ));
        std::fs::write(&source, b"still present").unwrap();
        assert!(!callback_confirms_completion(
            &operation,
            COPYENGINE_S_DONT_PROCESS_CHILDREN,
            Some(&target)
        ));
        std::fs::remove_file(source).unwrap();
        std::fs::remove_file(target).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
