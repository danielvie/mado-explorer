//! Owned Unicode clipboard payloads. Tests never touch the user's clipboard.
use std::{
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
};
use windows::{
    Win32::{
        Foundation::{GlobalFree, HANDLE, HGLOBAL, HWND},
        System::{DataExchange::*, Memory::*},
        UI::WindowsAndMessaging::*,
    },
    core::{PCWSTR, w},
};
const HDROP: u32 = 15;
const UNICODE_TEXT: u32 = 13;
const LIMIT: usize = 16 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClipboardFiles {
    pub paths: Vec<PathBuf>,
    pub cut: bool,
    pub sequence: u32,
}
struct Clipboard {
    owner: HWND,
}
impl Clipboard {
    fn open() -> Result<Self, String> {
        unsafe {
            // A real owner is required: OpenClipboard(NULL) followed by EmptyClipboard
            // leaves a NULL owner and makes SetClipboardData fail.
            let owner = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("STATIC"),
                PCWSTR::null(),
                WINDOW_STYLE::default(),
                0,
                0,
                0,
                0,
                Some(HWND_MESSAGE),
                None,
                None,
                None,
            )
            .map_err(|e| e.to_string())?;
            if let Err(error) = OpenClipboard(Some(owner)) {
                let _ = DestroyWindow(owner);
                return Err(error.to_string());
            }
            Ok(Self { owner })
        }
    }
}
impl Drop for Clipboard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseClipboard();
            let _ = DestroyWindow(self.owner);
        }
    }
}
struct Allocation(HGLOBAL);
impl Drop for Allocation {
    fn drop(&mut self) {
        unsafe {
            let _ = GlobalFree(Some(self.0));
        }
    }
}
impl Allocation {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.is_empty() || bytes.len() > LIMIT {
            return Err("Clipboard payload exceeds the 16 MiB limit".into());
        }
        unsafe {
            let value = Self(GlobalAlloc(GMEM_MOVEABLE, bytes.len()).map_err(|e| e.to_string())?);
            let pointer = GlobalLock(value.0);
            if pointer.is_null() {
                return Err("Could not lock clipboard allocation".into());
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer.cast(), bytes.len());
            let _ = GlobalUnlock(value.0);
            Ok(value)
        }
    }
    fn publish(self, format: u32) -> Result<(), String> {
        unsafe {
            SetClipboardData(format, Some(HANDLE(self.0.0))).map_err(|e| {
                format!(
                    "Clipboard publication failed; clipboard may contain a partial payload: {e}"
                )
            })?;
            std::mem::forget(self); // Windows owns it only after successful SetClipboardData.
            Ok(())
        }
    }
}
fn effect_format() -> Result<u32, String> {
    let format = unsafe { RegisterClipboardFormatW(w!("Preferred DropEffect")) };
    if format == 0 {
        Err("Could not register Preferred DropEffect".into())
    } else {
        Ok(format)
    }
}
fn bytes_of_words(words: impl IntoIterator<Item = u16>) -> Vec<u8> {
    words.into_iter().flat_map(u16::to_le_bytes).collect()
}
fn encode_paths(paths: &[PathBuf]) -> Result<Vec<u8>, String> {
    if paths.is_empty() {
        return Err("Select at least one file".into());
    }
    let mut bytes = vec![0; 20];
    bytes[..4].copy_from_slice(&20u32.to_le_bytes());
    bytes[16..20].copy_from_slice(&1u32.to_le_bytes());
    for path in paths {
        if !path.is_absolute() {
            return Err("Clipboard file paths must be absolute".into());
        }
        let words: Vec<u16> = path.as_os_str().encode_wide().collect();
        if words.contains(&0) {
            return Err("Clipboard paths cannot contain NUL".into());
        }
        if bytes
            .len()
            .saturating_add(words.len().saturating_mul(2))
            .saturating_add(4)
            > LIMIT
        {
            return Err("Clipboard file list exceeds 16 MiB".into());
        }
        bytes.extend(bytes_of_words(words));
        bytes.extend([0, 0]);
    }
    bytes.extend([0, 0]);
    Ok(bytes)
}
fn parse_paths(bytes: &[u8]) -> Result<Vec<PathBuf>, String> {
    if bytes.len() < 24 || bytes.len() > LIMIT {
        return Err("Invalid DROPFILES size".into());
    }
    let offset = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
    if offset < 20
        || !offset.is_multiple_of(2)
        || offset > bytes.len() - 4
        || u32::from_le_bytes(bytes[16..20].try_into().unwrap()) == 0
    {
        return Err("Invalid or non-Unicode DROPFILES header".into());
    }
    let words: Vec<u16> = bytes[offset..]
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    let mut paths = Vec::new();
    let mut start = 0;
    for (index, &word) in words.iter().enumerate() {
        if word != 0 {
            continue;
        }
        if index == start {
            if paths.is_empty() {
                return Err("Empty clipboard file list".into());
            }
            return Ok(paths);
        }
        let path = PathBuf::from(OsString::from_wide(&words[start..index]));
        if !path.is_absolute() {
            return Err("Relative clipboard file path rejected".into());
        }
        paths.push(path);
        if paths.len() > 100_000 {
            return Err("Clipboard contains too many files".into());
        }
        start = index + 1;
    }
    Err("Clipboard file list is not double-NUL terminated".into())
}
fn read_bytes(format: u32) -> Result<Vec<u8>, String> {
    unsafe {
        let data = GetClipboardData(format).map_err(|e| e.to_string())?;
        let handle = HGLOBAL(data.0);
        let size = GlobalSize(handle);
        if size == 0 || size > LIMIT {
            return Err("Invalid or oversized clipboard allocation".into());
        }
        let pointer = GlobalLock(handle);
        if pointer.is_null() {
            return Err("Could not lock clipboard data".into());
        }
        let result = std::slice::from_raw_parts(pointer.cast::<u8>(), size).to_vec();
        let _ = GlobalUnlock(handle);
        Ok(result)
    }
}
fn read_locked() -> Result<ClipboardFiles, String> {
    let paths = parse_paths(&read_bytes(HDROP)?)?;
    let format = effect_format()?;
    let cut = if unsafe { IsClipboardFormatAvailable(format) }.is_ok() {
        let effect = read_bytes(format)?;
        if effect.len() < 4 {
            return Err("Truncated clipboard drop effect".into());
        }
        let flags = u32::from_le_bytes(effect[..4].try_into().unwrap());
        // Only an unambiguous preferred MOVE requests move semantics.
        flags == 2
    } else {
        false
    };
    Ok(ClipboardFiles {
        paths,
        cut,
        sequence: unsafe { GetClipboardSequenceNumber() },
    })
}
pub fn read_files() -> Result<ClipboardFiles, String> {
    let _clipboard = Clipboard::open()?;
    read_locked()
}
fn prepared_files(paths: &[PathBuf], cut: bool) -> Result<Vec<(u32, Allocation)>, String> {
    let hdrop = Allocation::from_bytes(&encode_paths(paths)?)?;
    // Human-readable unquoted paths, separated by CRLF; authoritative paths stay HDROP.
    let mut words = Vec::new();
    for (index, path) in paths.iter().enumerate() {
        if index > 0 {
            words.extend([13, 10]);
        }
        words.extend(path.as_os_str().encode_wide());
    }
    words.push(0);
    let text = Allocation::from_bytes(&bytes_of_words(words))?;
    let effect = Allocation::from_bytes(&(if cut { 2u32 } else { 1u32 }).to_le_bytes())?;
    Ok(vec![
        (HDROP, hdrop),
        (UNICODE_TEXT, text),
        (effect_format()?, effect),
    ])
}
fn publish_locked(payloads: Vec<(u32, Allocation)>) -> Result<(), String> {
    unsafe { EmptyClipboard() }.map_err(|e| e.to_string())?;
    for (format, value) in payloads {
        value.publish(format)?;
    }
    Ok(())
}
pub fn write_files(paths: &[PathBuf], cut: bool) -> Result<u32, String> {
    let payloads = prepared_files(paths, cut)?;
    {
        let _clipboard = Clipboard::open()?;
        publish_locked(payloads)?;
    }
    Ok(unsafe { GetClipboardSequenceNumber() })
}
pub fn write_text(text: &str) -> Result<(), String> {
    if text.contains('\0') {
        return Err("Clipboard text cannot contain NUL".into());
    }
    let payload = Allocation::from_bytes(&bytes_of_words(
        text.encode_utf16().chain(std::iter::once(0)),
    ))?;
    let _clipboard = Clipboard::open()?;
    publish_locked(vec![(UNICODE_TEXT, payload)])
}
/// Remove only successfully moved source paths, and only while the complete
/// captured clipboard identity still matches. False means nothing was changed.
pub fn remove_completed(captured: &ClipboardFiles, completed: &[PathBuf]) -> Result<bool, String> {
    if !captured.cut || completed.is_empty() {
        return Ok(false);
    }
    let remaining: Vec<PathBuf> = captured
        .paths
        .iter()
        .filter(|p| !completed.contains(p))
        .cloned()
        .collect();
    if remaining.len() == captured.paths.len() {
        return Ok(false);
    }
    let payloads = if remaining.is_empty() {
        Vec::new()
    } else {
        prepared_files(&remaining, true)?
    };
    let _clipboard = Clipboard::open()?;
    if unsafe { GetClipboardSequenceNumber() } != captured.sequence {
        return Ok(false);
    }
    let current = read_locked()?;
    if &current != captured {
        return Ok(false);
    }
    publish_locked(payloads)?;
    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clipboard_copy_round_trip_lossless_unicode() {
        let paths = vec![
            PathBuf::from(r"C:\space name\雪.txt"),
            PathBuf::from(OsString::from_wide(&[67, 58, 92, 0xd800])),
        ];
        assert_eq!(parse_paths(&encode_paths(&paths).unwrap()).unwrap(), paths);
    }
    #[test]
    fn clipboard_copy_rejects_bad_offsets_and_missing_terminator() {
        let mut bytes = encode_paths(&[PathBuf::from(r"C:\a")]).unwrap();
        bytes.truncate(bytes.len() - 2);
        assert!(parse_paths(&bytes).is_err());
        let mut bytes = encode_paths(&[PathBuf::from(r"C:\a")]).unwrap();
        bytes[..4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(parse_paths(&bytes).is_err());
        assert!(encode_paths(&[PathBuf::from("relative")]).is_err());
        assert!(encode_paths(&[]).is_err());
    }
}
