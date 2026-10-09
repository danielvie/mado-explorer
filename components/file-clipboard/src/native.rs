use crate::{CodecError, FilePacket, MAX_EFFECT_BYTES, MAX_PAYLOAD_BYTES, decode_packet};
use std::marker::PhantomData;
use std::ptr::{copy_nonoverlapping, null_mut};
use std::rc::Rc;
use windows_sys::Win32::Foundation::{GetLastError, GlobalFree, HGLOBAL, HWND};
use windows_sys::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    RegisterClipboardFormatW, SetClipboardData,
};
use windows_sys::Win32::System::Memory::{
    GMEM_MOVEABLE, GMEM_ZEROINIT, GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock,
};
use windows_sys::Win32::System::Threading::GetCurrentProcessId;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowThreadProcessId, IsWindow};

const CF_HDROP: u32 = 15;
const EFFECT_NAME: &[u16] = &[
    80, 114, 101, 102, 101, 114, 114, 101, 100, 32, 68, 114, 111, 112, 69, 102, 102, 101, 99, 116,
    0,
];

/// What this call has done to the clipboard, even when the call returns an error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublicationState {
    /// EmptyClipboard was not successfully called. Not a cross-process retention guarantee.
    Untouched,
    /// Previous clipboard contents were removed; neither format was published.
    Cleared,
    /// Only Preferred DropEffect was published. No usable file list was published.
    IntentOnly,
    /// Both formats were published, including when CloseClipboard subsequently fails.
    Complete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransportFailure {
    Codec(CodecError),
    InvalidOwner,
    FileListUnavailable,
    /// OpenClipboard failed on the single attempt. Caller may offer explicit retry.
    BusyOrOpenFailed(u32),
    Native {
        api: &'static str,
        code: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportError {
    pub failure: TransportFailure,
    pub publication: PublicationState,
    /// Preserves the original failure if closing also failed.
    pub close_error: Option<u32>,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "file clipboard: {:?}; publication={:?}",
            self.failure, self.publication
        )?;
        if let Some(code) = self.close_error {
            write!(f, "; CloseClipboard error={code}")?;
        }
        Ok(())
    }
}
impl std::error::Error for TransportError {}
impl From<CodecError> for TransportError {
    fn from(error: CodecError) -> Self {
        failure(TransportFailure::Codec(error), PublicationState::Untouched)
    }
}

fn failure(failure: TransportFailure, publication: PublicationState) -> TransportError {
    TransportError {
        failure,
        publication,
        close_error: None,
    }
}
fn native_failure(api: &'static str, code: u32, publication: PublicationState) -> TransportError {
    failure(TransportFailure::Native { api, code }, publication)
}
fn last_error() -> u32 {
    unsafe { GetLastError() }
}

/// One explicit system-clipboard read. No retries, polling, threads or filesystem
/// calls. Invoke only after user Paste from a caller-owned background service.
/// Returns an owned immutable packet, never a borrowed clipboard handle.
/// Native delayed-rendering calls may block; this is not a hard-timeout API.
pub fn read_clipboard() -> Result<FilePacket, TransportError> {
    let effect_format = register_effect()?;
    let clipboard = Clipboard::open(null_mut())?;
    let result = (|| {
        // SAFETY: clipboard is open on this thread. Availability does not fetch data.
        if unsafe { IsClipboardFormatAvailable(CF_HDROP) } == 0 {
            return Err(failure(
                TransportFailure::FileListUnavailable,
                PublicationState::Untouched,
            ));
        }
        let hdrop = copy_clipboard_format(CF_HDROP, 20, MAX_PAYLOAD_BYTES)?;
        let effect = if unsafe { IsClipboardFormatAvailable(effect_format) } != 0 {
            Some(copy_clipboard_format(effect_format, 4, MAX_EFFECT_BYTES)?)
        } else {
            None
        };
        decode_packet(&hdrop, effect.as_deref()).map_err(Into::into)
    })();
    clipboard.finish(result, PublicationState::Untouched)
}

/// One explicit clipboard replacement. Encode/allocate/lock/unlock both buffers
/// before opening or emptying. Cut publishes Move metadata only, never touches
/// source files. No retry or attempted restoration of unknown previous formats.
///
/// `owner` must be a live caller-owned HWND in the current process, not NULL or a
/// foreign window. Caller keeps it alive for this call, even on a service thread.
/// Intent is published first, so a failed Cut cannot leave a file list without
/// its Move intent. Check `publication` on every error after clearing.
pub fn write_clipboard(owner: HWND, packet: &FilePacket) -> Result<(), TransportError> {
    validate_owner(owner)?;
    let effect_format = register_effect()?;
    let prepared = Prepared {
        effect_format,
        effect: OwnedGlobal::new(&packet.encode_preferred_effect())?,
        hdrop: OwnedGlobal::new(&packet.encode_hdrop()?)?,
    };
    let clipboard = Clipboard::open(owner)?;
    // Recheck after opening, before destructive clipboard replacement.
    let result = validate_owner(owner).and_then(|()| {
        publish(
            prepared,
            || {
                if unsafe { EmptyClipboard() } != 0 {
                    Ok(())
                } else {
                    Err(last_error())
                }
            },
            |format, handle| {
                // SAFETY: handle is an unlocked GMEM_MOVEABLE allocation. Its owner
                // relinquishes it only after SetClipboardData reports success.
                if unsafe { SetClipboardData(format, handle) }.is_null() {
                    Err(last_error())
                } else {
                    Ok(())
                }
            },
        )
    });
    clipboard.finish(result, PublicationState::Complete)
}

fn validate_owner(owner: HWND) -> Result<(), TransportError> {
    let mut pid = 0;
    // SAFETY: Windows checks the opaque HWND. Rust does not dereference it.
    if owner.is_null()
        || unsafe { IsWindow(owner) } == 0
        || unsafe { GetWindowThreadProcessId(owner, &mut pid) } == 0
        || pid != unsafe { GetCurrentProcessId() }
    {
        return Err(failure(
            TransportFailure::InvalidOwner,
            PublicationState::Untouched,
        ));
    }
    Ok(())
}

fn register_effect() -> Result<u32, TransportError> {
    // SAFETY: static NUL-terminated standard format name. No contents are accessed.
    let format = unsafe { RegisterClipboardFormatW(EFFECT_NAME.as_ptr()) };
    if format == 0 {
        Err(native_failure(
            "RegisterClipboardFormatW",
            last_error(),
            PublicationState::Untouched,
        ))
    } else {
        Ok(format)
    }
}

struct Clipboard {
    opened: bool,
    // Never send an open clipboard guard to another thread.
    _same_thread: PhantomData<Rc<()>>,
}
impl Clipboard {
    fn open(owner: HWND) -> Result<Self, TransportError> {
        // SAFETY: writer validates owner; NULL is allowed for read-only opens.
        if unsafe { OpenClipboard(owner) } == 0 {
            return Err(failure(
                TransportFailure::BusyOrOpenFailed(last_error()),
                PublicationState::Untouched,
            ));
        }
        Ok(Self {
            opened: true,
            _same_thread: PhantomData,
        })
    }
    fn finish<T>(
        mut self,
        result: Result<T, TransportError>,
        success_state: PublicationState,
    ) -> Result<T, TransportError> {
        self.opened = false;
        // SAFETY: this guard is private, !Send, and opened on this calling thread.
        if unsafe { CloseClipboard() } != 0 {
            return result;
        }
        let code = last_error();
        match result {
            Err(mut error) => {
                error.close_error = Some(code);
                Err(error)
            }
            Ok(_) => Err(native_failure("CloseClipboard", code, success_state)),
        }
    }
}
impl Drop for Clipboard {
    fn drop(&mut self) {
        if self.opened {
            // Unwind cleanup only. Ordinary paths use finish to report close failures.
            unsafe {
                CloseClipboard();
            }
        }
    }
}

fn copy_clipboard_format(format: u32, min: usize, max: usize) -> Result<Vec<u8>, TransportError> {
    // SAFETY: only called while clipboard is open. Borrowed handle is never freed.
    let handle = unsafe { GetClipboardData(format) };
    if handle.is_null() {
        return Err(native_failure(
            "GetClipboardData",
            last_error(),
            PublicationState::Untouched,
        ));
    }
    copy_global(handle, min, max)
}

fn copy_global(handle: HGLOBAL, min: usize, max: usize) -> Result<Vec<u8>, TransportError> {
    // SAFETY: callers supply a live owned or clipboard-held global allocation.
    let size = unsafe { GlobalSize(handle) };
    if size == 0 {
        return Err(native_failure(
            "GlobalSize",
            last_error(),
            PublicationState::Untouched,
        ));
    }
    if size > max {
        return Err(CodecError::PayloadTooLarge.into());
    }
    if size < min {
        return Err(CodecError::HeaderTooShort.into());
    }
    let mut bytes = crate::zeroed_bytes(size)?;
    let lock = GlobalGuard::lock(handle)?;
    // SAFETY: GlobalSize bounded the slice; lock keeps memory stationary until copied.
    unsafe {
        copy_nonoverlapping(lock.pointer.cast::<u8>(), bytes.as_mut_ptr(), size);
    }
    lock.unlock()?;
    Ok(bytes)
}

struct GlobalGuard {
    handle: HGLOBAL,
    pointer: *mut std::ffi::c_void,
    locked: bool,
}
impl GlobalGuard {
    fn lock(handle: HGLOBAL) -> Result<Self, TransportError> {
        let pointer = unsafe { GlobalLock(handle) };
        if pointer.is_null() {
            return Err(native_failure(
                "GlobalLock",
                last_error(),
                PublicationState::Untouched,
            ));
        }
        Ok(Self {
            handle,
            pointer,
            locked: true,
        })
    }
    fn unlock(mut self) -> Result<(), TransportError> {
        self.locked = false;
        // GlobalUnlock returns zero on successful final unlock; distinguish last error.
        unsafe {
            windows_sys::Win32::Foundation::SetLastError(0);
        }
        let still_locked = unsafe { GlobalUnlock(self.handle) };
        let code = last_error();
        if still_locked == 0 && code != 0 {
            Err(native_failure(
                "GlobalUnlock",
                code,
                PublicationState::Untouched,
            ))
        } else {
            Ok(())
        }
    }
}
impl Drop for GlobalGuard {
    fn drop(&mut self) {
        if self.locked {
            unsafe {
                GlobalUnlock(self.handle);
            }
        }
    }
}

#[cfg(test)]
thread_local! {
    static OWNED_FREES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

struct OwnedGlobal(HGLOBAL);
impl OwnedGlobal {
    fn new(bytes: &[u8]) -> Result<Self, TransportError> {
        if bytes.is_empty() || bytes.len() > MAX_PAYLOAD_BYTES {
            return Err(CodecError::PayloadTooLarge.into());
        }
        // SAFETY: size is bounded; zero padding prevents uninitialized clipboard bytes.
        let handle = unsafe { GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, bytes.len()) };
        if handle.is_null() {
            return Err(native_failure(
                "GlobalAlloc",
                last_error(),
                PublicationState::Untouched,
            ));
        }
        let owned = Self(handle);
        let lock = GlobalGuard::lock(handle)?;
        // SAFETY: allocation is at least bytes.len(); both slices are disjoint.
        unsafe {
            copy_nonoverlapping(bytes.as_ptr(), lock.pointer.cast::<u8>(), bytes.len());
        }
        lock.unlock()?;
        Ok(owned)
    }
    fn relinquish(&mut self) {
        self.0 = null_mut();
    }
}
impl Drop for OwnedGlobal {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _result = unsafe { GlobalFree(self.0) };
            #[cfg(test)]
            {
                assert!(_result.is_null(), "owned GlobalFree failed");
                OWNED_FREES.with(|count| count.set(count.get() + 1));
            }
        }
    }
}

struct Prepared {
    effect_format: u32,
    effect: OwnedGlobal,
    hdrop: OwnedGlobal,
}

// Injectable publication calls let tests cover every transfer/failure state without
// opening, reading or replacing the interactive clipboard.
fn publish(
    mut prepared: Prepared,
    empty: impl FnOnce() -> Result<(), u32>,
    mut set: impl FnMut(u32, HGLOBAL) -> Result<(), u32>,
) -> Result<(), TransportError> {
    empty().map_err(|code| native_failure("EmptyClipboard", code, PublicationState::Untouched))?;
    set(prepared.effect_format, prepared.effect.0).map_err(|code| {
        native_failure(
            "SetClipboardData(Preferred DropEffect)",
            code,
            PublicationState::Cleared,
        )
    })?;
    prepared.effect.relinquish();
    set(CF_HDROP, prepared.hdrop.0).map_err(|code| {
        native_failure(
            "SetClipboardData(CF_HDROP)",
            code,
            PublicationState::IntentOnly,
        )
    })?;
    prepared.hdrop.relinquish();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Intent;
    use std::path::PathBuf;
    use windows_sys::Win32::UI::Shell::DragQueryFileW;

    #[test]
    fn owned_hdrop_shell_parser_and_global_size_bounds() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        let mut unpaired: Vec<u16> = "C:\\native-".encode_utf16().collect();
        unpaired.extend([0xd800, 0x61, 0xdc00]);
        let paths = vec![
            PathBuf::from("C:\\données\\漢字.txt"),
            PathBuf::from("\\\\server\\share\\entry"),
            PathBuf::from(OsString::from_wide(&unpaired)),
            PathBuf::from(format!("\\\\?\\C:\\{}\\entry", "long\\".repeat(100))),
            PathBuf::from("\\\\?\\UNC\\server\\share\\entry"),
        ];
        let packet = FilePacket::snapshot(&paths, Intent::Move).unwrap();
        let bytes = packet.encode_hdrop().unwrap();
        let memory = OwnedGlobal::new(&bytes).unwrap();
        assert_eq!(
            unsafe { DragQueryFileW(memory.0, u32::MAX, null_mut(), 0) },
            paths.len() as u32
        );
        for (index, path) in paths.iter().enumerate() {
            use std::os::windows::ffi::OsStrExt;
            let expected: Vec<u16> = path.as_os_str().encode_wide().collect();
            let length = unsafe { DragQueryFileW(memory.0, index as u32, null_mut(), 0) } as usize;
            assert_eq!(length, expected.len());
            let mut actual = vec![0u16; length + 1];
            assert_eq!(
                unsafe {
                    DragQueryFileW(
                        memory.0,
                        index as u32,
                        actual.as_mut_ptr(),
                        actual.len() as u32,
                    )
                } as usize,
                length
            );
            assert_eq!(&actual[..length], expected);
            assert_eq!(actual[length], 0);
        }
        let copied = copy_global(memory.0, 20, MAX_PAYLOAD_BYTES).unwrap();
        assert_eq!(
            decode_packet(&copied, Some(&packet.encode_preferred_effect())).unwrap(),
            packet
        );
        assert!(matches!(
            copy_global(memory.0, 20, 4).unwrap_err().failure,
            TransportFailure::Codec(CodecError::PayloadTooLarge)
        ));
        assert!(
            copy_global(
                memory.0,
                unsafe { GlobalSize(memory.0) } + 1,
                MAX_PAYLOAD_BYTES
            )
            .is_err()
        );
    }

    #[test]
    fn publication_failures_transfer_only_successful_handles() {
        for fail_at in 0..=3 {
            let freed_before = OWNED_FREES.with(|count| count.get());
            let prepared = Prepared {
                effect_format: 0xc000,
                effect: OwnedGlobal::new(&1u32.to_le_bytes()).unwrap(),
                hdrop: OwnedGlobal::new(&[0u8; 32]).unwrap(),
            };
            let mut transferred = Vec::new();
            let mut calls = 0;
            let result = publish(
                prepared,
                || if fail_at == 0 { Err(5) } else { Ok(()) },
                |format, handle| {
                    calls += 1;
                    if fail_at == calls {
                        return Err(8);
                    }
                    transferred.push((format, handle));
                    Ok(())
                },
            );
            let state = match fail_at {
                0 => PublicationState::Untouched,
                1 => PublicationState::Cleared,
                2 => PublicationState::IntentOnly,
                _ => PublicationState::Complete,
            };
            if state == PublicationState::Complete {
                assert!(result.is_ok());
            } else {
                assert_eq!(result.unwrap_err().publication, state);
            }
            assert_eq!(
                transferred.len(),
                match fail_at {
                    0 | 1 => 0,
                    2 => 1,
                    _ => 2,
                }
            );
            assert_eq!(
                OWNED_FREES.with(|count| count.get()) - freed_before,
                2 - transferred.len()
            );
            for (index, (format, handle)) in transferred.into_iter().enumerate() {
                assert_eq!(format, if index == 0 { 0xc000 } else { CF_HDROP });
                // Fake recipient owns successful handles; writer must not have freed them.
                assert!(unsafe { GlobalSize(handle) } > 0);
                assert!(unsafe { GlobalFree(handle) }.is_null());
            }
        }
    }

    #[test]
    fn null_and_invalid_owners_rejected_before_any_clipboard_access() {
        assert_eq!(
            validate_owner(null_mut()).unwrap_err().failure,
            TransportFailure::InvalidOwner
        );
        assert_eq!(
            validate_owner(1usize as HWND).unwrap_err().failure,
            TransportFailure::InvalidOwner
        );
        let packet = FilePacket::snapshot(&[PathBuf::from("C:\\fixture")], Intent::Copy).unwrap();
        assert_eq!(
            write_clipboard(null_mut(), &packet)
                .unwrap_err()
                .publication,
            PublicationState::Untouched
        );
    }
}
