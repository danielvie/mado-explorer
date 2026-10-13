//! Typed Recycle Bin items. Identity is a Shell parsing name, never a filesystem
//! path. No API in this module empties the bin or acts on unselected entries.
use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};
use windows::{
    Win32::{
        Foundation::{HWND, PROPERTYKEY, SYSTEMTIME},
        System::{Com::CoTaskMemFree, Time::FileTimeToSystemTime},
        UI::{Shell::PropertiesSystem::PSGetPropertyKeyFromName, Shell::*, WindowsAndMessaging::*},
    },
    core::{Interface, PCSTR, PCWSTR, PSTR, PWSTR, w},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinItem {
    pub identity: Vec<u16>,
    pub display_name: String,
    pub original_path: Option<PathBuf>,
    pub size: Option<u64>,
    pub deleted: Option<String>,
}
#[derive(Debug)]
pub struct BinOutcome {
    pub identity: Vec<u16>,
    pub result: Result<(), String>,
}
fn take_wide(value: PWSTR) -> Vec<u16> {
    unsafe {
        let text = value.as_wide().to_vec();
        CoTaskMemFree(Some(value.0.cast()));
        text
    }
}
fn property(name: PCWSTR) -> Option<PROPERTYKEY> {
    let mut key = PROPERTYKEY::default();
    unsafe { PSGetPropertyKeyFromName(name, &mut key) }
        .ok()
        .map(|_| key)
}
fn display_leaf(value: String) -> String {
    let path = std::path::Path::new(&value);
    if path.is_absolute()
        && let Some(name) = path.file_name()
    {
        return name.to_string_lossy().into_owned();
    }
    value
}
fn snapshot(item: &IShellItem) -> Result<BinItem, String> {
    unsafe {
        let identity = take_wide(
            item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING)
                .map_err(|e| e.to_string())?,
        );
        let display_name = item
            .GetDisplayName(SIGDN_NORMALDISPLAY)
            .map(take_wide)
            .map(|v| display_leaf(String::from_utf16_lossy(&v)))
            .unwrap_or_else(|_| "Unnamed recycled item".into());
        let detail: Option<IShellItem2> = item.cast().ok();
        let original_path = detail
            .as_ref()
            .and_then(|d| {
                property(w!("System.Recycle.DeletedFrom")).and_then(|key| d.GetString(&key).ok())
            })
            .map(take_wide)
            .filter(|v| !v.is_empty())
            .map(|v| PathBuf::from(OsString::from_wide(&v)));
        let size = detail
            .as_ref()
            .and_then(|d| property(w!("System.Size")).and_then(|key| d.GetUInt64(&key).ok()));
        let deleted = detail
            .as_ref()
            .and_then(|d| {
                property(w!("System.Recycle.DateDeleted")).and_then(|key| d.GetFileTime(&key).ok())
            })
            .and_then(|ft| {
                let mut time = SYSTEMTIME::default();
                FileTimeToSystemTime(&ft, &mut time).ok()?;
                Some(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
                    time.wYear, time.wMonth, time.wDay, time.wHour, time.wMinute, time.wSecond
                ))
            });
        Ok(BinItem {
            identity,
            display_name,
            original_path,
            size,
            deleted,
        })
    }
}
fn enumerate() -> Result<Vec<(BinItem, IShellItem)>, String> {
    unsafe {
        let bin: IShellItem =
            SHGetKnownFolderItem(&FOLDERID_RecycleBinFolder, KF_FLAG_DEFAULT, None)
                .map_err(|e| e.to_string())?;
        let enumerator: IEnumShellItems = bin
            .BindToHandler(None, &BHID_EnumItems)
            .map_err(|e| e.to_string())?;
        let mut rows = Vec::new();
        loop {
            let mut items = [None];
            let mut count = 0;
            enumerator
                .Next(&mut items, Some(&mut count))
                .map_err(|e| e.to_string())?;
            if count == 0 {
                break;
            }
            let item = items[0]
                .take()
                .ok_or("Shell returned an empty Recycle Bin item")?;
            rows.push((snapshot(&item)?, item));
            if rows.len() > 100_000 {
                return Err(
                    "Recycle Bin contains more than 100,000 entries; listing stopped".into(),
                );
            }
        }
        Ok(rows)
    }
}
/// Read-only; call on a COM/OLE initialized thread. original_path is the original
/// containing folder (System.Recycle.DeletedFrom), not a synthesized file path.
pub fn list() -> Result<Vec<BinItem>, String> {
    Ok(enumerate()?.into_iter().map(|(value, _)| value).collect())
}
struct Menu(HMENU);
impl Drop for Menu {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyMenu(self.0);
        }
    }
}
fn invoke(hwnd: HWND, item: &IShellItem, verb: &str) -> Result<(), String> {
    unsafe {
        let context: IContextMenu = item
            .BindToHandler(None, &BHID_SFUIObject)
            .map_err(|e| e.to_string())?;
        let menu = Menu(CreatePopupMenu().map_err(|e| e.to_string())?);
        let query = context.QueryContextMenu(menu.0, 0, 1, 0x7fff, CMF_NORMAL);
        query.ok().map_err(|e| e.to_string())?;
        let count = (query.0 as u32) & 0xffff;
        for offset in 0..count {
            let mut text = [0u16; 512];
            if context
                .GetCommandString(
                    offset as usize,
                    GCS_VERBW,
                    None,
                    PSTR(text.as_mut_ptr().cast()),
                    text.len() as u32,
                )
                .is_err()
            {
                continue;
            }
            let end = text.iter().position(|&c| c == 0).unwrap_or(text.len());
            if String::from_utf16_lossy(&text[..end]) != verb {
                continue;
            }
            let command = CMINVOKECOMMANDINFO {
                cbSize: std::mem::size_of::<CMINVOKECOMMANDINFO>() as u32,
                hwnd,
                fMask: SEE_MASK_NOASYNC,
                lpVerb: PCSTR(offset as usize as *const u8),
                nShow: SW_SHOWNORMAL.0,
                ..Default::default()
            };
            return context.InvokeCommand(&command).map_err(|e| e.to_string());
        }
        Err(format!(
            "The selected item does not offer the {verb} action"
        ))
    }
}
fn validate_selection(identities: &[Vec<u16>]) -> Result<(), String> {
    if identities.is_empty() {
        return Err("Select specific Recycle Bin items first".into());
    }
    if identities.iter().any(|v| v.is_empty() || v.contains(&0)) {
        return Err("Invalid Shell identity".into());
    }
    if identities
        .iter()
        .enumerate()
        .any(|(i, id)| identities[..i].contains(id))
    {
        return Err("Duplicate selected Shell identity".into());
    }
    Ok(())
}
fn act(hwnd: HWND, identities: &[Vec<u16>], verb: &str) -> Result<Vec<BinOutcome>, String> {
    validate_selection(identities)?;
    let mut outcomes = Vec::new();
    for identity in identities {
        // Re-enumerate immediately before every action, so stale selections
        // cannot become arbitrary parsing-name requests outside the bin.
        let result = (|| {
            let current = enumerate()?;
            let (_, item) = current
                .iter()
                .find(|(row, _)| &row.identity == identity)
                .ok_or("Selected item has disappeared; refresh the Recycle Bin")?;
            invoke(hwnd, item, verb)?;
            if enumerate()?
                .iter()
                .any(|(row, _)| &row.identity == identity)
            {
                return Err("Item remains in the Recycle Bin: operation cancelled, pending, or unsuccessful; refresh before retrying".into());
            }
            Ok(())
        })();
        outcomes.push(BinOutcome {
            identity: identity.clone(),
            result,
        });
    }
    Ok(outcomes)
}
pub fn restore_selection(hwnd: HWND, identities: &[Vec<u16>]) -> Result<Vec<BinOutcome>, String> {
    act(hwnd, identities, "undelete")
}
pub fn delete_selection(
    hwnd: HWND,
    identities: &[Vec<u16>],
    confirmed: bool,
) -> Result<Vec<BinOutcome>, String> {
    if !confirmed {
        return Err(
            "Permanent deletion requires explicit confirmation for the selected items".into(),
        );
    }
    act(hwnd, identities, "delete")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bin_actions_reject_empty_duplicate_and_nul_identities() {
        assert!(validate_selection(&[]).is_err());
        assert!(validate_selection(&[vec![65, 0]]).is_err());
        assert!(validate_selection(&[vec![65], vec![65]]).is_err());
        assert!(validate_selection(&[vec![65], vec![66]]).is_ok());
    }
    #[test]
    fn bin_actions_require_confirmation_before_shell_access() {
        assert!(delete_selection(HWND::default(), &[vec![65]], false).is_err());
    }
    #[test]
    #[ignore = "Read-only diagnostic limited to owned Wayfarer workflow fixture metadata"]
    fn native_bin_owned_metadata_diagnostic() {
        let _ole = crate::platform::windows::OleGuard::initialize().expect("OLE initialized");
        let rows = list().expect("Recycle Bin listing");
        let mut matched = 0;
        for row in &rows {
            let identity = String::from_utf16_lossy(&row.identity);
            let own = row.display_name.contains("wayfarer-owned-workflow-")
                || row
                    .original_path
                    .as_ref()
                    .is_some_and(|p| p.to_string_lossy().contains("wayfarer-owned-workflow-"));
            if own {
                matched += 1;
                println!("OWNED BIN METADATA {row:?}; identity={identity}");
            }
        }
        println!(
            "Owned fixture rows matched: {matched}; total enumeration count: {}",
            rows.len()
        );
    }
    #[test]
    fn bin_listing_display_name_normalizes_original_absolute_path() {
        assert_eq!(
            display_leaf(r"C:\owned\destination\fixture.txt".into()),
            "fixture.txt"
        );
        assert_eq!(display_leaf("fixture.txt".into()), "fixture.txt");
        assert_eq!(
            display_leaf(r"\\server\share\fixture.txt".into()),
            "fixture.txt"
        );
    }
}
