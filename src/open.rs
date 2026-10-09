use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
    thread::{self, JoinHandle},
};

pub struct Report {
    pub path: PathBuf,
    pub result: io::Result<()>,
}

struct Running {
    path: PathBuf,
    worker: JoinHandle<io::Result<()>>,
}

// One explicit launch at a time, no queue, coalescing, or automatic retry.
// Retain the handle until it finishes; joining a finished worker cannot stall input.
#[derive(Default)]
pub struct Launcher {
    running: Option<Running>,
    pub report: Option<Report>,
    pub refused: Option<PathBuf>,
}

impl Launcher {
    pub fn busy_path(&self) -> Option<&Path> {
        self.running.as_ref().map(|running| running.path.as_path())
    }

    pub fn request(&mut self, path: PathBuf, repaint: Arc<dyn Fn() + Send + Sync>) {
        self.request_with(path, repaint, launch_default);
    }

    pub(crate) fn request_with(
        &mut self,
        path: PathBuf,
        repaint: Arc<dyn Fn() + Send + Sync>,
        launch: impl FnOnce(&Path) -> io::Result<()> + Send + 'static,
    ) {
        if self.running.is_some() {
            self.refused = Some(path);
            return;
        }
        self.refused = None;
        self.report = None;
        let captured = path.clone();
        match thread::Builder::new()
            .name("default-file-open".into())
            .spawn(move || {
                let result = launch(&captured);
                repaint();
                result
            }) {
            Ok(worker) => self.running = Some(Running { path, worker }),
            Err(error) => {
                self.report = Some(Report {
                    path,
                    result: Err(error),
                });
            }
        }
    }

    pub fn poll(&mut self) {
        if self
            .running
            .as_ref()
            .is_some_and(|r| r.worker.is_finished())
        {
            let running = self.running.take().unwrap();
            self.report = Some(Report {
                path: running.path,
                result: running.worker.join().unwrap_or_else(|_| {
                    Err(io::Error::other(
                        "Default-application launch worker panicked",
                    ))
                }),
            });
        }
    }
}

// On application exit, dropping an unfinished JoinHandle detaches the single
// uninterruptible OS call. There is no queued work; its COM resources are released
// when it returns (or by process teardown). Never wait for shell extensions on UI.

#[cfg(windows)]
fn shell_path(path: &Path) -> io::Result<Vec<u16>> {
    use std::{
        os::windows::ffi::OsStrExt,
        path::{Component, Prefix},
    };
    let supported_prefix = matches!(path.components().next(), Some(Component::Prefix(prefix))
        if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::UNC(_, _) | Prefix::VerbatimDisk(_) | Prefix::VerbatimUNC(_, _)));
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    if !path.is_absolute() || !supported_prefix || wide.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Expected an absolute filesystem path without embedded NULs (not a URI or device namespace)",
        ));
    }
    wide.push(0);
    Ok(wide)
}

#[cfg(windows)]
fn launch_default(path: &Path) -> io::Result<()> {
    use std::os::windows::fs::MetadataExt;
    use windows_sys::Win32::{
        Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT,
        System::Com::{
            COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoInitializeEx, CoUninitialize,
        },
        UI::{
            Shell::{SEE_MASK_FLAG_NO_UI, SEE_MASK_NOASYNC, SHELLEXECUTEINFOW, ShellExecuteExW},
            WindowsAndMessaging::SW_SHOWNORMAL,
        },
    };

    let wide = shell_path(path)?;
    // Recheck away from UI: disappeared/replaced folders and reparse points must
    // not become shell navigation. This is not a destructive-operation identity guarantee.
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry is no longer a regular file",
        ));
    }
    struct Apartment;
    impl Drop for Apartment {
        fn drop(&mut self) {
            // SAFETY: created only after successful initialization on this thread.
            unsafe { CoUninitialize() };
        }
    }
    // SAFETY: dedicated thread, null reserved pointer, documented STA flags.
    let hr = unsafe {
        CoInitializeEx(
            std::ptr::null(),
            (COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) as u32,
        )
    };
    if hr < 0 {
        return Err(io::Error::other(format!(
            "CoInitializeEx failed: HRESULT 0x{:08X}",
            hr as u32
        )));
    }
    let _apartment = Apartment;
    let verb: Vec<u16> = "open\0".encode_utf16().collect();
    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        // NOASYNC is required for a worker without a message loop. NO_UI routes
        // ordinary shell errors to our report, without suppressing security prompts.
        fMask: SEE_MASK_NOASYNC | SEE_MASK_FLAG_NO_UI,
        lpVerb: verb.as_ptr(),
        lpFile: wide.as_ptr(),
        nShow: SW_SHOWNORMAL,
        ..Default::default()
    };
    // SAFETY: structure size/flags match the W API; buffers remain alive until
    // synchronous return. No parameters, environment expansion, or runas verb.
    // NOCLOSEPROCESS is absent: the shell returns no process handle to own.
    if unsafe { ShellExecuteExW(&mut info) } == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(()) // OS accepted the request, not proof that an external app rendered it.
}

#[cfg(not(windows))]
fn launch_default(_path: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Default-application opening requires Windows",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::mpsc,
        time::{Duration, Instant},
    };

    fn finish(launcher: &mut Launcher) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while launcher.busy_path().is_some() {
            assert!(
                Instant::now() < deadline,
                "controlled launch worker timed out"
            );
            launcher.poll();
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn open_worker_is_bounded_captures_path_and_reports_controlled_os_failure() {
        let mut launcher = Launcher::default();
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let original = PathBuf::from("captured & file.txt");
        launcher.request_with(original.clone(), Arc::new(|| {}), move |path| {
            started_tx.send(path.to_owned()).unwrap();
            release_rx.recv().unwrap();
            Err(io::Error::from_raw_os_error(5))
        });
        assert_eq!(
            started_rx.recv_timeout(Duration::from_secs(5)).unwrap(),
            original
        );
        launcher.request_with(PathBuf::from("not-queued"), Arc::new(|| {}), |_| {
            panic!("must not launch")
        });
        assert_eq!(launcher.refused, Some(PathBuf::from("not-queued")));
        assert_eq!(launcher.busy_path(), Some(original.as_path()));
        release_tx.send(()).unwrap();
        finish(&mut launcher);
        let report = launcher.report.as_ref().unwrap();
        assert_eq!(report.path, original);
        assert_eq!(report.result.as_ref().unwrap_err().raw_os_error(), Some(5));
        assert!(
            report
                .result
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains('5')
        );
        launcher.request_with(PathBuf::from("explicit-next"), Arc::new(|| {}), |_| Ok(()));
        finish(&mut launcher);
        assert!(launcher.report.as_ref().unwrap().result.is_ok());
        assert!(launcher.refused.is_none());
    }

    #[cfg(windows)]
    #[test]
    fn open_shell_path_preserves_utf16_and_rejects_argument_namespace_edges() {
        use std::{
            ffi::OsString,
            os::windows::ffi::{OsStrExt, OsStringExt},
        };
        for text in [
            r"C:\fixture\-a & (b) %PATH% '日本語.txt",
            r"\\server\share\file.txt",
            r"\\?\C:\fixture\file.txt",
        ] {
            let path = PathBuf::from(text);
            let expected: Vec<_> = path.as_os_str().encode_wide().chain([0]).collect();
            assert_eq!(shell_path(&path).unwrap(), expected);
        }
        let mut wide: Vec<_> = r"C:\fixture\".encode_utf16().collect();
        wide.extend([0xD800, b'.' as u16, b't' as u16, b'x' as u16, b't' as u16]);
        let path = PathBuf::from(OsString::from_wide(&wide));
        assert!(path.to_str().is_none());
        assert_eq!(
            shell_path(&path).unwrap(),
            wide.into_iter().chain([0]).collect::<Vec<_>>()
        );
        for text in [
            "",
            "file.txt",
            r"C:relative.txt",
            r"\rooted.txt",
            "https://example.test/a",
            r"\\.\pipe\a",
            r"\\?\GLOBALROOT\Device\a",
            "C:\\a\0b.txt",
        ] {
            assert_eq!(
                shell_path(Path::new(text)).unwrap_err().kind(),
                io::ErrorKind::InvalidInput,
                "{text:?}"
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn open_native_preflight_rejects_missing_file_without_shell_execution() {
        let path = std::env::temp_dir().join(format!(
            "mado-open-missing-{}-{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        assert_eq!(
            launch_default(&path).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
    }
}
