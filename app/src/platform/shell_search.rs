//! Searchable projection of a live Shell menu. Unsupported owner-drawn items
//! remain unavailable; callers must retain the native-menu fallback.
use std::{
    collections::HashSet, marker::PhantomData, os::windows::ffi::OsStrExt, path::PathBuf, rc::Rc,
};
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::Com::CoTaskMemFree,
        UI::{Shell::Common::ITEMIDLIST, Shell::*, WindowsAndMessaging::*},
    },
    core::{Interface, PCSTR, PCWSTR, PSTR},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub id: u32,
    pub label: String,
    pub enabled: bool,
    pub canonical_verb: Option<String>,
}
struct Entry {
    candidate: Candidate,
    menu: HMENU,
    position: u32,
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
/// Own on the UI STA. Dropping a session invalidates its IDs; IDs are never
/// persisted or mapped across two selections.
pub struct MenuSession {
    hwnd: HWND,
    context: IContextMenu,
    menu: Menu,
    entries: Vec<Entry>,
    count: u32,
    invoked: bool,
    initialized: bool,
    _pidls: Vec<Pidl>,
    _thread: PhantomData<Rc<()>>,
}
fn display_label(raw: &str) -> String {
    let mut text = String::new();
    let mut chars = raw.split('\t').next().unwrap_or("").chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '&' {
            if chars.peek() == Some(&'&') {
                text.push('&');
                chars.next();
            }
        } else {
            text.push(ch);
        }
    }
    text
}
impl MenuSession {
    pub fn new(hwnd: HWND, paths: &[PathBuf]) -> Result<Self, String> {
        let first = paths.first().ok_or("Select an item first")?;
        let parent_path = first.parent().ok_or("Root menus require the native menu")?;
        if paths
            .iter()
            .any(|p| !p.is_absolute() || p.parent() != Some(parent_path))
        {
            return Err("Shell menu selection must use absolute paths from one folder".into());
        }
        unsafe {
            let mut pidls = Vec::new();
            let mut children = Vec::new();
            let mut parent: Option<IShellFolder> = None;
            for path in paths {
                let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
                if wide.contains(&0) {
                    return Err("Invalid NUL in path".into());
                }
                wide.push(0);
                let mut raw = std::ptr::null_mut();
                SHParseDisplayName(PCWSTR(wide.as_ptr()), None, &mut raw, 0, None)
                    .map_err(|e| e.to_string())?;
                let pidl = Pidl(raw);
                let mut child = std::ptr::null_mut();
                let folder: IShellFolder =
                    SHBindToParent(pidl.0, Some(&mut child)).map_err(|e| e.to_string())?;
                if parent.is_none() {
                    parent = Some(folder);
                }
                children.push(child as *const ITEMIDLIST);
                pidls.push(pidl);
            }
            let context: IContextMenu = parent
                .ok_or("Missing parent")?
                .GetUIObjectOf(hwnd, &children, None)
                .map_err(|e| e.to_string())?;
            let menu = Menu(CreatePopupMenu().map_err(|e| e.to_string())?);
            let query = context.QueryContextMenu(menu.0, 0, 1, 0x7fff, CMF_NORMAL);
            query.ok().map_err(|e| e.to_string())?;
            let session = Self {
                hwnd,
                context,
                menu,
                entries: Vec::new(),
                count: (query.0 as u32) & 0xffff,
                invoked: false,
                initialized: false,
                _pidls: pidls,
                _thread: PhantomData,
            };
            Ok(session)
        }
    }
    fn prepare(&self, menu: HMENU, position: u32) -> Result<(), String> {
        unsafe {
            let w = WPARAM(menu.0 as usize);
            let l = LPARAM(position as isize);
            if let Ok(extended) = self.context.cast::<IContextMenu3>() {
                let mut result = LRESULT(0);
                extended
                    .HandleMenuMsg2(WM_INITMENUPOPUP, w, l, Some(&mut result))
                    .map_err(|e| e.to_string())?;
            } else if let Ok(extended) = self.context.cast::<IContextMenu2>() {
                extended
                    .HandleMenuMsg(WM_INITMENUPOPUP, w, l)
                    .map_err(|e| e.to_string())?;
            }
            Ok(())
        }
    }
    fn collect(
        &mut self,
        menu: HMENU,
        index: u32,
        prefix: &str,
        parent_enabled: bool,
        depth: usize,
        visited: &mut HashSet<usize>,
    ) -> Result<(), String> {
        if depth > 8 || !visited.insert(menu.0 as usize) {
            return Err("Shell submenu exceeds safe traversal limits; use the native menu".into());
        }
        self.prepare(menu, index)?;
        let count = unsafe { GetMenuItemCount(Some(menu)) };
        if !(0..=2048).contains(&count) {
            return Err("Invalid or excessive Shell menu size".into());
        }
        for position in 0..count as u32 {
            unsafe {
                if self.entries.len() >= 2048 {
                    return Err("Shell menu exceeds 2048 entries; use native menu".into());
                }
                let mut info = MENUITEMINFOW {
                    cbSize: std::mem::size_of::<MENUITEMINFOW>() as u32,
                    fMask: MIIM_FTYPE | MIIM_STATE | MIIM_ID | MIIM_SUBMENU,
                    ..Default::default()
                };
                GetMenuItemInfoW(menu, position, true, &mut info).map_err(|e| e.to_string())?;
                if info.fType.0 & MFT_SEPARATOR.0 != 0 {
                    continue;
                }
                let length = GetMenuStringW(menu, position, None, MF_BYPOSITION);
                if length > 4096 {
                    return Err("Shell menu label exceeds supported length".into());
                }
                let mut buffer = vec![0u16; length.max(0) as usize + 1];
                let copied = GetMenuStringW(menu, position, Some(&mut buffer), MF_BYPOSITION).max(0)
                    as usize;
                let name = display_label(&String::from_utf16_lossy(&buffer[..copied]));
                let unsupported = info.fType.0 & MFT_OWNERDRAW.0 != 0 || name.is_empty();
                let name = if name.is_empty() {
                    "Native menu item".into()
                } else {
                    name
                };
                let label = if prefix.is_empty() {
                    name
                } else {
                    format!("{prefix} > {name}")
                };
                let enabled = parent_enabled
                    && info.fState.0 & (MFS_DISABLED.0 | MFS_GRAYED.0) == 0
                    && !unsupported;
                if !info.hSubMenu.is_invalid() && !unsupported {
                    self.collect(info.hSubMenu, position, &label, enabled, depth + 1, visited)?;
                } else {
                    let available =
                        enabled && info.wID > 0 && info.wID <= self.count && info.wID <= 0x7fff;
                    let canonical_verb = if available {
                        let mut verb = [0u16; 512];
                        self.context
                            .GetCommandString(
                                (info.wID - 1) as usize,
                                GCS_VERBW,
                                None,
                                PSTR(verb.as_mut_ptr().cast()),
                                verb.len() as u32,
                            )
                            .ok()
                            .and_then(|_| {
                                let end = verb.iter().position(|&c| c == 0)?;
                                if end == 0 {
                                    None
                                } else {
                                    Some(String::from_utf16_lossy(&verb[..end]))
                                }
                            })
                    } else {
                        None
                    };
                    self.entries.push(Entry {
                        candidate: Candidate {
                            id: info.wID,
                            label: if unsupported {
                                format!("{label} (use native menu)")
                            } else {
                                label
                            },
                            enabled: available,
                            canonical_verb,
                        },
                        menu,
                        position,
                    });
                }
            }
        }
        Ok(())
    }
    /// Dynamic submenus are initialized lazily on the first nonempty search.
    /// An empty initial query returns no candidates; prompt the user to type.
    pub fn query(&mut self, query: &str) -> Result<Vec<Candidate>, String> {
        if !self.initialized {
            if query.trim().is_empty() {
                return Ok(Vec::new());
            }
            self.entries.clear();
            self.collect(self.menu.0, 0, "", true, 0, &mut HashSet::new())?;
            // Ambiguous extension IDs cannot safely identify a unique command.
            let mut ids = HashSet::new();
            let duplicates: HashSet<u32> = self
                .entries
                .iter()
                .filter_map(|e| {
                    if !ids.insert(e.candidate.id) {
                        Some(e.candidate.id)
                    } else {
                        None
                    }
                })
                .collect();
            for entry in &mut self.entries {
                if duplicates.contains(&entry.candidate.id) {
                    entry.candidate.enabled = false;
                }
            }
            self.initialized = true;
        }
        let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
        Ok(self
            .entries
            .iter()
            .filter(|entry| {
                let label = entry.candidate.label.to_lowercase();
                let verb = entry
                    .candidate
                    .canonical_verb
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase();
                words
                    .iter()
                    .all(|word| label.contains(word) || verb.contains(word))
            })
            .map(|entry| entry.candidate.clone())
            .collect())
    }
    /// Consumes invocation eligibility even if the Shell returns an error: retry
    /// requires a new selection/session, avoiding accidental duplicate actions.
    pub fn invoke(&mut self, id: u32) -> Result<(), String> {
        if self.invoked {
            return Err("This Shell menu session has already been invoked".into());
        }
        let entry = self
            .entries
            .iter()
            .find(|e| e.candidate.id == id && e.candidate.enabled)
            .ok_or("Shell command is unavailable")?;
        unsafe {
            let mut current = MENUITEMINFOW {
                cbSize: std::mem::size_of::<MENUITEMINFOW>() as u32,
                fMask: MIIM_ID | MIIM_STATE | MIIM_FTYPE,
                ..Default::default()
            };
            GetMenuItemInfoW(entry.menu, entry.position, true, &mut current)
                .map_err(|e| e.to_string())?;
            if current.wID != id
                || current.fState.0 & (MFS_DISABLED.0 | MFS_GRAYED.0) != 0
                || current.fType.0 & MFT_OWNERDRAW.0 != 0
            {
                return Err("Shell menu command changed; reopen the menu".into());
            }
            self.invoked = true;
            let command = CMINVOKECOMMANDINFO {
                cbSize: std::mem::size_of::<CMINVOKECOMMANDINFO>() as u32,
                hwnd: self.hwnd,
                fMask: SEE_MASK_NOASYNC,
                lpVerb: PCSTR((id - 1) as usize as *const u8),
                nShow: SW_SHOWNORMAL.0,
                ..Default::default()
            };
            self.context
                .InvokeCommand(&command)
                .map_err(|e| e.to_string())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shell_search_labels_preserve_literal_ampersands() {
        assert_eq!(display_label("&Open\tEnter"), "Open");
        assert_eq!(display_label("Save && &Close"), "Save & Close");
    }
}
