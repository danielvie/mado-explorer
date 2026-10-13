//! STA-owned OLE drag/drop. Shell performs user drops synchronously; this adapter
//! never queues a second operation or deletes sources after an outbound move.
use std::{
    cell::{Cell, RefCell},
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    rc::Rc,
};
use windows::{
    Win32::{
        Foundation::{E_FAIL, E_INVALIDARG, E_POINTER, HWND, POINTL, RECT},
        System::{Com::*, Memory::*, Ole::*, SystemServices::MODIFIERKEYS_FLAGS},
        UI::Shell::{Common::ITEMIDLIST, *},
    },
    core::{Error, PCWSTR, Ref, Result, implement},
};
const LIMIT: usize = 16 * 1024 * 1024;
fn wide(path: &Path) -> Result<Vec<u16>> {
    let mut words: Vec<u16> = path.as_os_str().encode_wide().collect();
    if !path.is_absolute() || words.contains(&0) {
        return Err(Error::from_hresult(E_INVALIDARG));
    }
    words.push(0);
    Ok(words)
}
struct Medium(STGMEDIUM);
impl Drop for Medium {
    fn drop(&mut self) {
        unsafe {
            ReleaseStgMedium(&mut self.0);
        }
    }
}
fn validate_bytes(bytes: &[u8]) -> bool {
    if bytes.len() < 24 || bytes.len() > LIMIT {
        return false;
    }
    let offset = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
    if offset < 20
        || !offset.is_multiple_of(2)
        || offset > bytes.len() - 4
        || u32::from_le_bytes(bytes[16..20].try_into().unwrap()) == 0
    {
        return false;
    }
    let words: Vec<u16> = bytes[offset..]
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    let mut start = 0;
    let mut count = 0;
    for (i, &word) in words.iter().enumerate() {
        if word == 0 {
            if start == i {
                return count > 0;
            }
            if !PathBuf::from(OsString::from_wide(&words[start..i])).is_absolute() {
                return false;
            }
            count += 1;
            if count > 100_000 {
                return false;
            }
            start = i + 1;
        }
    }
    false
}
fn accepts(data: &IDataObject) -> Result<bool> {
    unsafe {
        let format = FORMATETC {
            cfFormat: 15,
            dwAspect: DVASPECT_CONTENT.0,
            lindex: -1,
            tymed: TYMED_HGLOBAL.0 as u32,
            ..Default::default()
        };
        let medium = Medium(data.GetData(&format)?);
        if medium.0.tymed != TYMED_HGLOBAL.0 as u32 {
            return Ok(false);
        }
        let handle = medium.0.u.hGlobal;
        let size = GlobalSize(handle);
        if !(24..=LIMIT).contains(&size) {
            return Ok(false);
        }
        let pointer = GlobalLock(handle);
        if pointer.is_null() {
            return Ok(false);
        }
        // Copy under lock, then release before parsing to keep ownership bounded.
        let bytes = std::slice::from_raw_parts(pointer.cast::<u8>(), size).to_vec();
        let _ = GlobalUnlock(handle);
        Ok(validate_bytes(&bytes))
    }
}
#[derive(Clone, Debug)]
pub struct DropOutcome {
    pub destination: Option<PathBuf>,
    pub effect: u32,
    /// True only when the native target's Drop method was actually called.
    pub delegated: bool,
    /// Shell HRESULT outcome, not an invented per-file completion result.
    pub result: std::result::Result<(), String>,
}
#[derive(Default)]
struct State {
    targets: RefCell<Vec<(RECT, PathBuf)>>,
    fallback: RefCell<Option<PathBuf>>,
    session: RefCell<Option<Session>>,
    probe: Cell<bool>,
    outcome: RefCell<Option<DropOutcome>>,
    delegated: Cell<bool>,
}
struct Session {
    data: IDataObject,
    target: Option<PathBuf>,
    delegate: Option<IDropTarget>,
}
impl State {
    fn destination(&self, point: &POINTL) -> Option<PathBuf> {
        let targets = self.targets.borrow();
        if targets.is_empty() {
            return self.fallback.borrow().clone();
        }
        targets
            .iter()
            .find(|(r, _)| {
                point.x >= r.left && point.x < r.right && point.y >= r.top && point.y < r.bottom
            })
            .map(|(_, p)| p.clone())
    }
    fn leave(&self) -> Result<()> {
        let session = self.session.borrow_mut().take();
        if let Some(delegate) = session.and_then(|s| s.delegate) {
            unsafe {
                delegate.DragLeave()?;
            }
        }
        Ok(())
    }
    fn update(
        &self,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
        drop_now: bool,
    ) -> Result<()> {
        let Some(mut session) = self.session.borrow_mut().take() else {
            unsafe {
                *effect = DROPEFFECT_NONE;
            }
            return Ok(());
        };
        let destination = self.destination(point);
        unsafe {
            *effect = DROPEFFECT((*effect).0 & (DROPEFFECT_COPY.0 | DROPEFFECT_MOVE.0));
        }
        if self.probe.get() {
            unsafe {
                *effect = DROPEFFECT_NONE;
            }
            if !drop_now {
                self.session.replace(Some(session));
            }
            return Ok(());
        }
        let changed = session.target != destination;
        if changed {
            if let Some(old) = session.delegate.take() {
                unsafe {
                    old.DragLeave()?;
                }
            }
            session.target = destination;
            if let Some(path) = &session.target {
                let words = wide(path)?;
                let item: IShellItem =
                    unsafe { SHCreateItemFromParsingName(PCWSTR(words.as_ptr()), None)? };
                let delegate: IDropTarget = unsafe { item.BindToHandler(None, &BHID_SFUIObject)? };
                unsafe {
                    delegate.DragEnter(&session.data, keys, *point, effect)?;
                }
                session.delegate = Some(delegate);
            }
        }
        if let Some(delegate) = &session.delegate {
            if drop_now {
                self.delegated.set(true);
                unsafe {
                    delegate.Drop(&session.data, keys, *point, effect)?;
                }
            } else if !changed {
                unsafe {
                    delegate.DragOver(keys, *point, effect)?;
                }
            }
        } else {
            unsafe {
                *effect = DROPEFFECT_NONE;
            }
        }
        unsafe {
            *effect = DROPEFFECT((*effect).0 & (DROPEFFECT_COPY.0 | DROPEFFECT_MOVE.0));
        }
        if !drop_now {
            self.session.replace(Some(session));
        }
        Ok(())
    }
}
fn boundary(effect: *mut DROPEFFECT, work: impl FnOnce() -> Result<()>) -> Result<()> {
    if effect.is_null() {
        return Err(Error::from_hresult(E_POINTER));
    }
    let result =
        catch_unwind(AssertUnwindSafe(work)).unwrap_or_else(|_| Err(Error::from_hresult(E_FAIL)));
    if result.is_err() {
        unsafe {
            *effect = DROPEFFECT_NONE;
        }
    }
    result
}
#[implement(IDropTarget)]
struct Target {
    state: Rc<State>,
}
impl IDropTarget_Impl for Target_Impl {
    fn DragEnter(
        &self,
        data: Ref<'_, IDataObject>,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> Result<()> {
        boundary(effect, || {
            self.state.leave()?;
            let data = data
                .as_ref()
                .ok_or_else(|| Error::from_hresult(E_POINTER))?;
            if !accepts(data).unwrap_or(false) {
                unsafe {
                    *effect = DROPEFFECT_NONE;
                }
                return Ok(());
            }
            self.state.session.replace(Some(Session {
                data: data.clone(),
                target: None,
                delegate: None,
            }));
            self.state.update(keys, point, effect, false)
        })
    }
    fn DragOver(
        &self,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> Result<()> {
        boundary(effect, || self.state.update(keys, point, effect, false))
    }
    fn DragLeave(&self) -> Result<()> {
        catch_unwind(AssertUnwindSafe(|| self.state.leave()))
            .unwrap_or_else(|_| Err(Error::from_hresult(E_FAIL)))
    }
    fn Drop(
        &self,
        data: Ref<'_, IDataObject>,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> Result<()> {
        self.state.delegated.set(false);
        let mut destination = None;
        let result = boundary(effect, || {
            destination = self.state.destination(point);
            let data = data
                .as_ref()
                .ok_or_else(|| Error::from_hresult(E_POINTER))?;
            if !accepts(data).unwrap_or(false) {
                self.state.leave()?;
                unsafe {
                    *effect = DROPEFFECT_NONE;
                }
                return Ok(());
            }
            // Refresh the actual drop payload rather than retaining a stale provider.
            if let Some(session) = self.state.session.borrow_mut().as_mut() {
                session.data = data.clone();
            }
            self.state.update(keys, point, effect, true)
        });
        let returned_effect = if effect.is_null() {
            0
        } else {
            unsafe { (*effect).0 }
        };
        self.state.outcome.replace(Some(DropOutcome {
            destination,
            effect: returned_effect,
            delegated: self.state.delegated.get(),
            result: result.as_ref().map(|_| ()).map_err(|e| e.to_string()),
        }));
        result
    }
}
/// Keep alive on the HWND/OLE thread; drop before destroying its window.
pub struct DropRegistration {
    hwnd: HWND,
    state: Rc<State>,
    _target: IDropTarget,
}
impl DropRegistration {
    pub fn new(hwnd: HWND) -> std::result::Result<Self, String> {
        let state = Rc::new(State::default());
        let target: IDropTarget = Target {
            state: state.clone(),
        }
        .into();
        unsafe { RegisterDragDrop(hwnd, &target) }.map_err(|e| e.to_string())?;
        Ok(Self {
            hwnd,
            state,
            _target: target,
        })
    }
    pub fn set_target(&self, path: Option<PathBuf>) {
        self.state.fallback.replace(path);
        self.state.targets.borrow_mut().clear();
    }
    /// Physical screen pixel rectangles; points outside all panes reject drops.
    pub fn set_targets(&self, targets: Vec<(RECT, PathBuf)>) {
        self.state.targets.replace(targets);
    }
    /// Bounded latest result slot. Taking clears it; no COM objects cross out.
    pub fn take_outcome(&self) -> Option<DropOutcome> {
        self.state.outcome.borrow_mut().take()
    }
    pub fn set_read_only_probe(&self, value: bool) {
        self.state.probe.set(value);
    }
}
impl Drop for DropRegistration {
    fn drop(&mut self) {
        let _ = self.state.leave();
        unsafe {
            let _ = RevokeDragDrop(self.hwnd);
        }
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
/// Outbound paths must have one parent. Native Shell supplies IDataObject and
/// IDropSource behavior; offered effects never include LINK.
pub fn drag(paths: &[PathBuf]) -> std::result::Result<u32, String> {
    let first = paths.first().ok_or("Select files to drag")?;
    let parent_path = first.parent().ok_or("Cannot drag a filesystem root")?;
    if paths.iter().any(|p| p.parent() != Some(parent_path)) {
        return Err("Drag files from one folder at a time".into());
    }
    let operation = || -> Result<u32> {
        unsafe {
            let mut owned = Vec::new();
            let mut children = Vec::new();
            let mut parent: Option<IShellFolder> = None;
            for path in paths {
                let text = wide(path)?;
                let mut raw = std::ptr::null_mut();
                SHParseDisplayName(PCWSTR(text.as_ptr()), None, &mut raw, 0, None)?;
                let pidl = Pidl(raw);
                let mut child = std::ptr::null_mut();
                let folder: IShellFolder = SHBindToParent(pidl.0, Some(&mut child))?;
                if parent.is_none() {
                    parent = Some(folder);
                }
                children.push(child as *const ITEMIDLIST);
                owned.push(pidl);
            }
            let data: IDataObject = parent
                .ok_or_else(|| Error::from_hresult(E_INVALIDARG))?
                .GetUIObjectOf(HWND::default(), &children, None)?;
            let effect = SHDoDragDrop(None, &data, None, DROPEFFECT_COPY | DROPEFFECT_MOVE)?;
            Ok(effect.0)
        }
    };
    operation().map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drag_payload_rejects_truncation_and_virtual_or_relative_paths() {
        assert!(!validate_bytes(&[]));
        let mut bytes = vec![0u8; 20];
        bytes[..4].copy_from_slice(&20u32.to_le_bytes());
        bytes[16..20].copy_from_slice(&1u32.to_le_bytes());
        bytes.extend(
            "C:\\owned\\雪.txt"
                .encode_utf16()
                .chain([0, 0])
                .flat_map(u16::to_le_bytes),
        );
        assert!(validate_bytes(&bytes));
        bytes.truncate(bytes.len() - 2);
        assert!(!validate_bytes(&bytes));
    }
    #[test]
    fn drag_destination_uses_physical_pane_rectangles() {
        let state = State::default();
        state.targets.replace(vec![(
            RECT {
                left: 100,
                top: 100,
                right: 200,
                bottom: 200,
            },
            PathBuf::from(r"C:\owned"),
        )]);
        assert!(state.destination(&POINTL { x: 99, y: 150 }).is_none());
        assert_eq!(
            state.destination(&POINTL { x: 150, y: 150 }),
            Some(PathBuf::from(r"C:\owned"))
        );
        assert!(state.destination(&POINTL { x: 200, y: 150 }).is_none());
    }
}
