//! Requested folder statistics. Local SSD/HDD share one worker and fixed budgets;
//! network scans are disabled unless explicitly enabled. No background auto-scans.
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};
const MAX_DEPTH: usize = 128;
const MAX_ENTRIES: u64 = 1_000_000;
#[derive(Clone, Debug, Default)]
pub struct StatisticsResult {
    pub files: u64,
    pub dirs: u64,
    pub bytes: u64,
    pub errors: u64,
    pub cancelled: bool,
    pub complete: bool,
    pub failed: bool,
    pub skipped_reparse: u64,
    pub limit_reached: bool,
    pub message: Option<String>,
}
#[derive(Default)]
struct Mailbox {
    pending: Option<(u64, PathBuf, bool)>,
    result: Option<(u64, StatisticsResult)>,
    stop: bool,
}
pub struct Statistics {
    mailbox: Arc<(Mutex<Mailbox>, Condvar)>,
    generation: Arc<AtomicU64>,
    result: Option<StatisticsResult>,
    allow_network: bool,
}
impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}
impl Statistics {
    pub fn new() -> Self {
        let mailbox = Arc::new((Mutex::new(Mailbox::default()), Condvar::new()));
        let generation = Arc::new(AtomicU64::new(0));
        let worker = mailbox.clone();
        let current = generation.clone();
        thread::spawn(move || {
            loop {
                let (lock, wake) = &*worker;
                let (token, path, allow_network) = {
                    let mut state = lock.lock().unwrap();
                    while state.pending.is_none() && !state.stop {
                        state = wake.wait(state).unwrap();
                    }
                    if state.stop {
                        return;
                    }
                    state.pending.take().unwrap()
                };
                let result = if !allow_network && is_network(&path) {
                    StatisticsResult {
                        failed: true,
                        message: Some("Network statistics disabled by policy".into()),
                        ..Default::default()
                    }
                } else {
                    scan(&path, || current.load(Ordering::Acquire) != token)
                };
                let mut state = lock.lock().unwrap();
                if !state.stop && current.load(Ordering::Acquire) == token {
                    state.result = Some((token, result));
                }
            }
        });
        Self {
            mailbox,
            generation,
            result: None,
            allow_network: false,
        }
    }
    pub fn set_allow_network(&mut self, allow: bool) {
        self.clear();
        self.allow_network = allow;
    }
    pub fn request(&mut self, path: PathBuf) {
        let token = self
            .generation
            .fetch_add(1, Ordering::AcqRel)
            .wrapping_add(1);
        let mut state = self.mailbox.0.lock().unwrap();
        state.pending = Some((token, path, self.allow_network));
        state.result = None;
        self.result = None;
        self.mailbox.1.notify_one();
    }
    pub fn poll(&mut self) -> bool {
        let mut state = self.mailbox.0.lock().unwrap();
        if let Some((token, result)) = state.result.take()
            && token == self.generation.load(Ordering::Acquire)
        {
            self.result = Some(result);
            return true;
        }
        false
    }
    pub fn result(&self) -> Option<&StatisticsResult> {
        self.result.as_ref()
    }
    pub fn clear(&mut self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        let mut state = self.mailbox.0.lock().unwrap();
        state.pending = None;
        state.result = None;
        self.result = None;
    }
    pub fn cancel(&mut self) {
        self.clear();
        self.result = Some(StatisticsResult {
            cancelled: true,
            message: Some("Scan cancelled; totals discarded".into()),
            ..Default::default()
        });
    }
}
impl Drop for Statistics {
    fn drop(&mut self) {
        self.clear();
        let mut state = self.mailbox.0.lock().unwrap();
        state.stop = true;
        self.mailbox.1.notify_one();
    }
}
fn reparse(metadata: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        metadata.file_type().is_symlink()
    }
}
fn is_network(path: &Path) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows::{
            Win32::Storage::FileSystem::{GetDriveTypeW, GetVolumePathNameW},
            core::PCWSTR,
        };
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let mut volume = vec![0u16; 32768];
        // Fail closed when drive type cannot be resolved; do not start a scan.
        if unsafe { GetVolumePathNameW(PCWSTR(wide.as_ptr()), &mut volume) }.is_err() {
            return true;
        }
        matches!(unsafe { GetDriveTypeW(PCWSTR(volume.as_ptr())) }, 0 | 1 | 4)
    }
    #[cfg(not(windows))]
    {
        path.to_string_lossy().starts_with("//")
    }
}
fn scan(path: &Path, cancelled: impl Fn() -> bool) -> StatisticsResult {
    let mut result = StatisticsResult::default();
    if cancelled() {
        result.cancelled = true;
        return result;
    }
    match fs::symlink_metadata(path) {
        Ok(metadata) if reparse(&metadata) => {
            result.skipped_reparse = 1;
            result.message = Some("Reparse root was not followed".into());
            return result;
        }
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => {
            result.failed = true;
            result.message = Some("Statistics require a directory".into());
            return result;
        }
        Err(error) => {
            result.failed = true;
            result.errors = 1;
            result.message = Some(error.to_string());
            return result;
        }
    }
    let root = match fs::read_dir(path) {
        Ok(root) => root,
        Err(error) => {
            result.failed = true;
            result.errors = 1;
            result.message = Some(error.to_string());
            return result;
        }
    };
    let mut stack = vec![root];
    let mut seen = 0u64;
    while !stack.is_empty() {
        if cancelled() {
            result.cancelled = true;
            break;
        }
        let entry = match stack.last_mut().unwrap().next() {
            Some(entry) => entry,
            None => {
                stack.pop();
                continue;
            }
        };
        seen += 1;
        if seen > MAX_ENTRIES {
            result.limit_reached = true;
            break;
        }
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                result.errors += 1;
                continue;
            }
        };
        let metadata = match fs::symlink_metadata(entry.path()) {
            Ok(metadata) => metadata,
            Err(_) => {
                result.errors += 1;
                continue;
            }
        };
        if reparse(&metadata) {
            result.skipped_reparse += 1;
            continue;
        }
        if metadata.is_dir() {
            result.dirs += 1;
            if stack.len() >= MAX_DEPTH {
                result.limit_reached = true;
                continue;
            }
            match fs::read_dir(entry.path()) {
                Ok(children) => stack.push(children),
                Err(_) => result.errors += 1,
            }
        } else if metadata.is_file() {
            result.files += 1;
            match result.bytes.checked_add(metadata.len()) {
                Some(bytes) => result.bytes = bytes,
                None => {
                    result.limit_reached = true;
                    break;
                }
            }
        }
    }
    result.complete = !result.cancelled
        && !result.limit_reached
        && result.errors == 0
        && result.skipped_reparse == 0;
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folder_statistics_owned_counts_and_cancellation() {
        let root = std::env::temp_dir().join(format!("wayfarer-stats-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("child")).unwrap();
        fs::write(root.join("one"), b"123").unwrap();
        fs::write(root.join("child/two"), b"45").unwrap();
        let result = scan(&root, || false);
        assert_eq!((result.files, result.dirs, result.bytes), (2, 1, 5));
        assert!(result.complete);
        let result = scan(&root, || true);
        assert!(result.cancelled);
        assert!(!result.complete);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn folder_statistics_missing_is_failed() {
        let result = scan(
            Path::new("wayfarer-nonexistent-statistics-fixture-374840"),
            || false,
        );
        assert!(result.failed);
        assert!(!result.complete);
    }
}
