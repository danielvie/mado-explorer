//! STA-thread Shell integration. Call only after OleGuard initialization.
use std::{
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
};
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::Com::CoTaskMemFree,
        UI::{
            Input::KeyboardAndMouse::{GetFocus, SetFocus},
            Shell::Common::ITEMIDLIST,
            Shell::*,
            WindowsAndMessaging::*,
        },
    },
    core::{Interface, PCSTR, PCWSTR},
};

fn wide(path: &Path) -> Result<Vec<u16>, String> {
    let mut value: Vec<u16> = path.as_os_str().encode_wide().collect();
    if value.contains(&0) {
        return Err("Paths cannot contain NUL characters".into());
    }
    value.push(0);
    Ok(value)
}
pub fn home_directory() -> Result<PathBuf, String> {
    // SAFETY: Shell allocates a NUL-terminated path with the COM allocator.
    unsafe {
        let value = SHGetKnownFolderPath(&FOLDERID_Profile, KF_FLAG_DEFAULT, None)
            .map_err(|e| e.to_string())?;
        let path = PathBuf::from(OsString::from_wide(value.as_wide()));
        CoTaskMemFree(Some(value.0.cast()));
        Ok(path)
    }
}
pub fn open_path(path: &Path) -> Result<(), String> {
    let path = wide(path)?;
    // SAFETY: Strings remain valid during this synchronous call.
    let status = unsafe {
        ShellExecuteW(
            None,
            windows::core::w!("open"),
            PCWSTR(path.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    if status.0 as isize <= 32 {
        Err(format!(
            "Windows could not open the item (Shell code {})",
            status.0 as isize
        ))
    } else {
        Ok(())
    }
}
struct Pidl(*mut ITEMIDLIST);
impl Drop for Pidl {
    fn drop(&mut self) {
        unsafe {
            CoTaskMemFree(Some(self.0.cast()));
        }
    }
}
struct Menu(HMENU);
impl Drop for Menu {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyMenu(self.0);
        }
    }
}
struct Forwarder {
    menu2: Option<IContextMenu2>,
    menu3: Option<IContextMenu3>,
}
unsafe extern "system" fn menu_proc(
    hwnd: HWND,
    msg: u32,
    w: WPARAM,
    l: LPARAM,
    _id: usize,
    data: usize,
) -> LRESULT {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if matches!(msg, WM_INITMENUPOPUP | WM_MENUCHAR)
            || (matches!(msg, WM_DRAWITEM | WM_MEASUREITEM) && w.0 == 0)
        {
            // SAFETY: Registration owns this boxed context until removal.
            let state = unsafe { &*(data as *const Forwarder) };
            if let Some(menu) = &state.menu3 {
                let mut result = LRESULT(0);
                if unsafe { menu.HandleMenuMsg2(msg, w, l, Some(&mut result)) }.is_ok() {
                    return Some(result);
                }
            } else if msg != WM_MENUCHAR
                && let Some(menu) = &state.menu2
                && unsafe { menu.HandleMenuMsg(msg, w, l) }.is_ok()
            {
                return Some(LRESULT(0));
            }
        }
        None
    }));
    if let Ok(Some(value)) = result {
        value
    } else {
        unsafe { DefSubclassProc(hwnd, msg, w, l) }
    }
}
struct Subclass {
    hwnd: HWND,
    state: *mut Forwarder,
}
impl Drop for Subclass {
    fn drop(&mut self) {
        // A failed detach deliberately retains the context to avoid a dangling callback.
        unsafe {
            if RemoveWindowSubclass(self.hwnd, Some(menu_proc), self.state as usize).as_bool() {
                drop(Box::from_raw(self.state));
            }
        }
    }
}
fn selection_parent(paths: &[PathBuf]) -> Result<&Path, String> {
    let first = paths.first().ok_or("Select at least one item")?;
    if !first.is_absolute() {
        return Err("Shell selections require absolute paths".into());
    }
    let parent = first.parent().ok_or("Root menus are not supported")?;
    if paths
        .iter()
        .any(|p| !p.is_absolute() || p.parent() != Some(parent))
    {
        return Err("Shell menu items must share one parent folder".into());
    }
    Ok(parent)
}
/// Shows the real native menu; the chosen command is explicitly invoked by the user.
/// Never call while holding application state locks across the native modal loop.
pub fn show_context_menu(
    hwnd: HWND,
    paths: &[PathBuf],
    screen_x: i32,
    screen_y: i32,
) -> Result<(), String> {
    selection_parent(paths)?;
    unsafe {
        let mut owned = Vec::with_capacity(paths.len());
        let mut children = Vec::with_capacity(paths.len());
        let mut parent: Option<IShellFolder> = None;
        for path in paths {
            let value = wide(path)?;
            let mut pidl = std::ptr::null_mut();
            SHParseDisplayName(PCWSTR(value.as_ptr()), None, &mut pidl, 0, None)
                .map_err(|e| e.to_string())?;
            let pidl = Pidl(pidl);
            let mut child = std::ptr::null_mut();
            let folder: IShellFolder =
                SHBindToParent(pidl.0, Some(&mut child)).map_err(|e| e.to_string())?;
            if parent.is_none() {
                parent = Some(folder);
            }
            children.push(child as *const ITEMIDLIST);
            owned.push(pidl);
        }
        let context: IContextMenu = parent
            .ok_or("Missing parent folder")?
            .GetUIObjectOf(hwnd, &children, None)
            .map_err(|e| e.to_string())?;
        let menu = Menu(CreatePopupMenu().map_err(|e| e.to_string())?);
        let query = context.QueryContextMenu(menu.0, 0, 1, 0x7fff, CMF_NORMAL);
        query.ok().map_err(|e| e.to_string())?;
        let count = (query.0 as u32) & 0xffff;
        let state = Box::into_raw(Box::new(Forwarder {
            menu2: context.cast().ok(),
            menu3: context.cast().ok(),
        }));
        if !SetWindowSubclass(hwnd, Some(menu_proc), state as usize, state as usize).as_bool() {
            drop(Box::from_raw(state));
            return Err("Could not install Shell menu message forwarding".into());
        }
        let forwarding = Subclass { hwnd, state };
        let focus = GetFocus();
        let selected = TrackPopupMenuEx(
            menu.0,
            TPM_RETURNCMD.0 | TPM_RIGHTBUTTON.0,
            screen_x,
            screen_y,
            hwnd,
            None,
        )
        .0 as u32;
        drop(forwarding);
        if !focus.is_invalid() && IsWindow(Some(focus)).as_bool() {
            let _ = SetFocus(Some(focus));
        }
        if selected == 0 {
            return Ok(());
        }
        if selected > count || selected > 0x7fff {
            return Err("Shell returned an invalid menu command".into());
        }
        let command = CMINVOKECOMMANDINFO {
            cbSize: std::mem::size_of::<CMINVOKECOMMANDINFO>() as u32,
            hwnd,
            // SDK: CMIC_MASK_NOASYNC aliases SEE_MASK_NOASYNC.
            fMask: SEE_MASK_NOASYNC,
            lpVerb: PCSTR((selected - 1) as usize as *const u8),
            nShow: SW_SHOWNORMAL.0,
            ..Default::default()
        };
        context.InvokeCommand(&command).map_err(|e| e.to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shell_menu_selection_rejects_cross_folder_and_empty() {
        assert!(selection_parent(&[]).is_err());
        assert!(
            selection_parent(&[PathBuf::from(r"C:\a\one"), PathBuf::from(r"C:\b\two")]).is_err()
        );
        assert!(
            selection_parent(&[PathBuf::from(r"C:\a\one"), PathBuf::from(r"C:\a\two")]).is_ok()
        );
    }
    #[test]
    fn shell_menu_preserves_non_unicode_and_rejects_nul() {
        let raw = vec![67, 58, 92, 0xd800];
        let path = PathBuf::from(OsString::from_wide(&raw));
        assert_eq!(&wide(&path).unwrap()[..raw.len()], raw);
        assert!(wide(Path::new(&OsString::from_wide(&[65, 0, 66]))).is_err());
    }
}
