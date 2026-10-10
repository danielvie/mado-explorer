//! Lossless lexical admission before I/O, then one nonfollowing read-only handle.
use crate::protocol::PATH_UNITS;
use std::{fs::File, os::windows::io::FromRawHandle, ptr};
use windows_sys::{
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        Storage::FileSystem::{
            FILE_NON_DIRECTORY_FILE, FILE_OPEN, FILE_OPEN_REPARSE_POINT,
            FILE_SYNCHRONOUS_IO_NONALERT, NtCreateFile,
        },
    },
    Win32::{
        Foundation::{HANDLE, OBJ_DONT_REPARSE, RtlNtStatusToDosError, UNICODE_STRING},
        Storage::FileSystem::{
            FILE_ATTRIBUTE_DEVICE, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_REPARSE_POINT,
            FILE_ATTRIBUTE_TAG_INFO, FILE_READ_ATTRIBUTES, FILE_READ_DATA, FILE_SHARE_READ,
            FILE_TYPE_DISK, FileAttributeTagInfo, GetFileInformationByHandleEx, GetFileType,
            SYNCHRONIZE,
        },
        System::IO::IO_STATUS_BLOCK,
    },
};

fn eq(s: &[u16], text: &str) -> bool {
    s.len() == text.len()
        && s.iter()
            .zip(text.bytes())
            .all(|(&a, b)| a <= 127 && (a as u8).eq_ignore_ascii_case(&b))
}
fn component(s: &[u16], filename: bool) -> bool {
    if s.is_empty()
        || eq(s, ".")
        || eq(s, "..")
        || matches!(s.last(), Some(32 | 46))
        || s.iter().any(|&n| {
            n < 32
                || (127..=159).contains(&n)
                || [0, 34, 42, 47, 58, 60, 62, 63, 92, 124].contains(&n)
        })
    {
        return false;
    }
    if !filename {
        return true;
    }
    let mut stem = &s[..s.iter().position(|&x| x == 46).unwrap_or(s.len())];
    // DOS aliases also trim spaces immediately before the extension separator.
    while stem.last() == Some(&32) {
        stem = &stem[..stem.len() - 1];
    }
    if ["CON", "PRN", "AUX", "NUL", "CLOCK$", "CONIN$", "CONOUT$"]
        .iter()
        .any(|s| eq(stem, s))
    {
        return false;
    }
    if stem.len() == 4
        && (eq(&stem[..3], "COM") || eq(&stem[..3], "LPT"))
        && ((48..=57).contains(&stem[3]) || [0xb9, 0xb2, 0xb3].contains(&stem[3]))
    {
        return false;
    }
    true
}
/// Returns bounded NT filesystem spelling; rejects all other namespaces before I/O.
pub fn admit(units: &[u16]) -> Result<Vec<u16>, ()> {
    if units.is_empty() || units.len() > PATH_UNITS || units.contains(&0) {
        return Err(());
    }
    let mut s = units;
    let verbatim = s.starts_with(&[92, 92, 63, 92]);
    if verbatim {
        s = &s[4..];
    }
    let drive = s.len() >= 3
        && ((65..=90).contains(&s[0]) || (97..=122).contains(&s[0]))
        && s[1..3] == [58, 92];
    let (unc, tail) = if drive {
        (false, &s[3..])
    } else if s.starts_with(&[92, 92]) {
        (true, &s[2..])
    } else if verbatim && s.len() >= 4 && eq(&s[..4], "UNC\\") {
        (true, &s[4..])
    } else {
        return Err(());
    };
    let parts: Vec<&[u16]> = tail.split(|&n| n == 92).collect();
    let start = if unc { 2 } else { 0 };
    if parts.len() <= start {
        return Err(());
    }
    if unc && (!component(parts[0], false) || !component(parts[1], false) || eq(parts[1], "pipe")) {
        return Err(());
    }
    if parts[start..].iter().any(|s| !component(s, true)) {
        return Err(());
    }
    let native_length = if unc { 8 + tail.len() } else { 4 + s.len() };
    if native_length > PATH_UNITS {
        return Err(());
    }
    let mut nt: Vec<u16> = "\\??\\".encode_utf16().collect();
    if unc {
        nt.extend("UNC\\".encode_utf16());
        nt.extend_from_slice(tail);
    } else {
        nt.extend_from_slice(s);
    }
    Ok(nt)
}

pub fn open(units: &[u16]) -> Result<File, u32> {
    let mut nt = admit(units).map_err(|_| 87u32)?;
    let mut name = UNICODE_STRING {
        Length: (nt.len() * 2) as u16,
        MaximumLength: (nt.len() * 2) as u16,
        Buffer: nt.as_mut_ptr(),
    };
    let attributes = OBJECT_ATTRIBUTES {
        Length: size_of::<OBJECT_ATTRIBUTES>() as u32,
        ObjectName: &mut name,
        Attributes: OBJ_DONT_REPARSE,
        ..Default::default()
    };
    let mut handle: HANDLE = ptr::null_mut();
    let mut ios = IO_STATUS_BLOCK::default();
    // SAFETY: bounded native name/structs are alive; synchronous read-only FILE_OPEN.
    // OBJ_DONT_REPARSE refuses ancestor resolution too; no following reopen fallback.
    let status = unsafe {
        NtCreateFile(
            &mut handle,
            FILE_READ_DATA | FILE_READ_ATTRIBUTES | SYNCHRONIZE,
            &attributes,
            &mut ios,
            ptr::null(),
            0,
            FILE_SHARE_READ,
            FILE_OPEN,
            FILE_NON_DIRECTORY_FILE | FILE_OPEN_REPARSE_POINT | FILE_SYNCHRONOUS_IO_NONALERT,
            ptr::null(),
            0,
        )
    };
    if status < 0 {
        return Err(unsafe { RtlNtStatusToDosError(status) });
    }
    // SAFETY: successful synchronous create returned a newly owned handle.
    let file = unsafe { File::from_raw_handle(handle) };
    let mut tag = FILE_ATTRIBUTE_TAG_INFO::default();
    // SAFETY: live file and correctly sized output structure.
    if unsafe { GetFileType(handle) } != FILE_TYPE_DISK
        || unsafe {
            GetFileInformationByHandleEx(
                handle,
                FileAttributeTagInfo,
                &mut tag as *mut _ as _,
                size_of::<FILE_ATTRIBUTE_TAG_INFO>() as u32,
            )
        } == 0
        || tag.FileAttributes
            & (FILE_ATTRIBUTE_DIRECTORY | FILE_ATTRIBUTE_REPARSE_POINT | FILE_ATTRIBUTE_DEVICE)
            != 0
    {
        return Err(50);
    }
    Ok(file)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relative_unc_is_rejected_before_io() {
        for s in [
            "UNC\\server\\share\\x.png",
            "unc\\server\\share\\x.png",
            "uNc\\server\\share\\x.png",
        ] {
            assert!(admit(&s.encode_utf16().collect::<Vec<_>>()).is_err(), "{s}");
        }
    }
    #[test]
    fn absolute_unc_preserves_native_units() {
        let filename = [0xd800, 120, 0xdc00, 233, 46, 112, 110, 103];
        let mut expected: Vec<u16> = "\\??\\UNC\\server\\share\\".encode_utf16().collect();
        expected.extend_from_slice(&filename);
        for prefix in [
            "\\\\server\\share\\",
            "\\\\?\\UNC\\server\\share\\",
            "\\\\?\\unc\\server\\share\\",
            "\\\\?\\uNc\\server\\share\\",
        ] {
            let mut units: Vec<u16> = prefix.encode_utf16().collect();
            units.extend_from_slice(&filename);
            assert_eq!(admit(&units).unwrap(), expected, "{prefix}");
        }
    }
    #[test]
    fn namespaces_and_native_units() {
        for s in [
            "file.png",
            "C:foo",
            "\\\\.\\pipe\\name",
            "\\\\?\\GLOBALROOT\\Device\\x",
            "\\\\server\\PIPE\\x",
            "C:\\NUL.png",
            "C:\\CON .txt",
            "C:\\COM1 .jpg",
            "\\\\?\\C:\\NUL .png",
            "C:\\x:stream",
            "C:\\a\\..\\b",
            "C:\\a ",
            "C:\\x\\",
        ] {
            assert!(admit(&s.encode_utf16().collect::<Vec<_>>()).is_err(), "{s}");
        }
        for s in [
            "C:\\a.png",
            "\\\\?\\C:\\a.png",
            "\\\\CON\\NUL\\é.png",
            "\\\\?\\UNC\\CON\\NUL\\a.png",
        ] {
            assert!(admit(&s.encode_utf16().collect::<Vec<_>>()).is_ok(), "{s}");
        }
        let p = [67, 58, 92, 0xd800];
        assert!(admit(&p).unwrap().ends_with(&[0xd800]));
        assert!(admit(&[67, 58, 92, 0]).is_err());
    }
}
