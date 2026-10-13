//! Thread-affine native lifetime, with no application state in callbacks.
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{
    cell::Cell,
    marker::PhantomData,
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
};
use windows::{
    Win32::{
        Foundation::{E_FAIL, E_INVALIDARG, HWND, LPARAM, LRESULT, WPARAM},
        System::{
            Ole::{OleInitialize, OleUninitialize},
            Threading::GetCurrentThreadId,
        },
        UI::{
            Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
            WindowsAndMessaging::{GetWindowThreadProcessId, SendMessageW, WM_NCDESTROY, WM_NULL},
        },
    },
    core::{Error, Result},
};

/// Retain on the main thread from before run_native until it returns.
pub struct OleGuard {
    _thread: PhantomData<Rc<()>>,
}
impl OleGuard {
    pub fn initialize() -> Result<Self> {
        // SAFETY: Each success, including S_FALSE, is balanced on this same thread.
        unsafe {
            OleInitialize(None)?;
        }
        Ok(Self {
            _thread: PhantomData,
        })
    }
}
impl Drop for OleGuard {
    fn drop(&mut self) {
        // SAFETY: Rc marker prevents moving the guard to another thread.
        unsafe {
            OleUninitialize();
        }
    }
}
struct CallbackState {
    attached: Cell<bool>,
    destroyed: Cell<bool>,
    probes: Cell<u64>,
}
/// Owns a subclass and an independent registration reference retained by Windows.
pub struct NativeHost {
    hwnd: HWND,
    state: Rc<CallbackState>,
}
impl NativeHost {
    pub fn is_attached(&self) -> bool {
        self.state.attached.get()
    }
    pub fn was_destroyed(&self) -> bool {
        self.state.destroyed.get()
    }
    pub fn attach(window: &impl HasWindowHandle) -> Result<Self> {
        let handle = window
            .window_handle()
            .map_err(|_| Error::new(E_FAIL, "Native handle unavailable"))?;
        let RawWindowHandle::Win32(handle) = handle.as_raw() else {
            return Err(Error::new(E_INVALIDARG, "Expected HWND"));
        };
        Self::from_hwnd(HWND(handle.hwnd.get() as *mut _))
    }
    /// Attach only on the thread owning this live HWND.
    pub fn from_hwnd(hwnd: HWND) -> Result<Self> {
        // SAFETY: Queries borrowed live toolkit HWND; require same-thread ownership.
        if unsafe { GetWindowThreadProcessId(hwnd, None) != GetCurrentThreadId() } {
            return Err(Error::new(E_INVALIDARG, "HWND belongs to another thread"));
        }
        let state = Rc::new(CallbackState {
            attached: Cell::new(false),
            destroyed: Cell::new(false),
            probes: Cell::new(0),
        });
        let ptr = Rc::into_raw(Rc::clone(&state));
        // SAFETY: Registration owns this Rc until removal, using its unique address as ID.
        if !unsafe { SetWindowSubclass(hwnd, Some(subclass), ptr as usize, ptr as usize) }.as_bool()
        {
            // SAFETY: Failed registration never owns the reference.
            unsafe {
                drop(Rc::from_raw(ptr));
            }
            return Err(Error::new(E_FAIL, "SetWindowSubclass failed"));
        }
        state.attached.set(true);
        Ok(Self { hwnd, state })
    }
    /// Read-only synchronous WM_NULL callback round trip.
    pub fn probe(&self) -> Result<u64> {
        if !self.state.attached.get() || self.state.destroyed.get() {
            return Err(Error::new(E_FAIL, "Native callback detached"));
        }
        let before = self.state.probes.get();
        // SAFETY: Same-thread live HWND; callback borrows no widget state.
        unsafe {
            SendMessageW(self.hwnd, WM_NULL, None, None);
        }
        let after = self.state.probes.get();
        if after == before {
            return Err(Error::new(E_FAIL, "Native callback not delivered"));
        }
        Ok(after)
    }
    pub fn detach(&mut self) -> Result<()> {
        if !self.state.attached.get() {
            return Ok(());
        }
        let ptr = Rc::as_ptr(&self.state);
        // SAFETY: Owner thread and exact registration identity.
        if !unsafe { RemoveWindowSubclass(self.hwnd, Some(subclass), ptr as usize) }.as_bool() {
            return Err(Error::new(
                E_FAIL,
                "Callback removal failed; ownership retained",
            ));
        }
        self.state.attached.set(false);
        // SAFETY: Successful removal releases registration's reference exactly once.
        unsafe {
            drop(Rc::from_raw(ptr));
        }
        Ok(())
    }
}
impl Drop for NativeHost {
    fn drop(&mut self) {
        // Failure keeps registration's Rc alive, avoiding dangling callback data.
        let _ = self.detach();
    }
}
unsafe extern "system" fn subclass(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    id: usize,
    data: usize,
) -> LRESULT {
    let ptr = data as *const CallbackState;
    // SAFETY: Registration retains ptr; local reference protects nested teardown.
    unsafe {
        Rc::increment_strong_count(ptr);
    }
    // SAFETY: Consumes only the temporary increment above.
    let state = unsafe { Rc::from_raw(ptr) };
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if message == WM_NULL {
            state.probes.set(state.probes.get().wrapping_add(1));
        }
        if message == WM_NCDESTROY {
            state.destroyed.set(true);
            // SAFETY: Executing on HWND owner thread with exact subclass ID.
            if state.attached.get()
                && unsafe { RemoveWindowSubclass(hwnd, Some(subclass), id) }.as_bool()
            {
                state.attached.set(false);
                // SAFETY: Successful removal releases the registration reference.
                unsafe {
                    drop(Rc::from_raw(ptr));
                }
            }
        }
    }));
    // SAFETY: Always preserve toolkit processing, including input and destruction.
    unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
}

/// Current monitor work area in logical UI points. The HWND is borrowed on its owner thread.
pub fn work_area(hwnd: HWND, scale: f32) -> std::result::Result<([f32; 2], [f32; 2]), String> {
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
    };
    if !scale.is_finite() || scale <= 0.0 {
        return Err("Invalid window scale".into());
    }
    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    // SAFETY: valid borrowed HWND; API writes only the initialized stack MONITORINFO.
    unsafe {
        if !GetMonitorInfoW(MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST), &mut info).as_bool()
        {
            return Err("Monitor work area unavailable".into());
        }
    }
    Ok((
        [
            info.rcWork.left as f32 / scale,
            info.rcWork.top as f32 / scale,
        ],
        [
            (info.rcWork.right - info.rcWork.left) as f32 / scale,
            (info.rcWork.bottom - info.rcWork.top) as f32 / scale,
        ],
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "Opt-in native COM initialization on test thread"]
    fn native_runtime_balances_nested_ole_initialization() {
        let first = OleGuard::initialize().expect("initial STA");
        let second = OleGuard::initialize().expect("nested STA");
        drop(second);
        drop(first);
    }
}
