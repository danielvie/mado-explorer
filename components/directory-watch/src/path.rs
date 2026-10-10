use crate::{Failure, PathRejection};
use std::{os::windows::ffi::OsStrExt, path::Path};
pub(crate) fn admitted_name(path: &Path) -> Result<Vec<u16>, Failure> {
    let units: Vec<u16> = path.as_os_str().encode_wide().collect();
    let extra = admit(&units).map_err(Failure::Rejected)?;
    let mut name = Vec::with_capacity(units.len() + extra + 1);
    let rest = match extra {
        4 => {
            name.extend([92, 92, 63, 92]);
            &units[..]
        }
        6 => {
            name.extend([92, 92, 63, 92, 85, 78, 67, 92]);
            &units[2..]
        }
        _ => &units[..],
    };
    // Verbatim '/' is rejected, not interpreted as a separator. Ordinary separators are
    // equivalent only after admission excludes dot/parent/device/trailing aliases.
    name.extend(rest.iter().map(|&u| if u == 47 { 92 } else { u }));
    name.push(0);
    Ok(name)
}

fn separator(unit: u16) -> bool {
    matches!(unit, 47 | 92)
}

fn ascii_eq(units: &[u16], ascii: &str) -> bool {
    units.len() == ascii.len()
        && units
            .iter()
            .zip(ascii.bytes())
            .all(|(&a, b)| a <= 127 && (a as u8).eq_ignore_ascii_case(&b))
}
fn drive(units: &[u16]) -> bool {
    units.len() >= 3
        && matches!(units[0], 65..=90 | 97..=122)
        && units[1] == b':' as u16
        && separator(units[2])
}
fn volume(units: &[u16]) -> bool {
    units.len() >= 45
        && ascii_eq(&units[..7], "Volume{")
        && units[43] == b'}' as u16
        && units[44] == b'\\' as u16
        && units[7..43].iter().enumerate().all(|(i, &u)| {
            if matches!(i, 8 | 13 | 18 | 23) {
                u == b'-' as u16
            } else {
                matches!(u, 48..=57 | 65..=70 | 97..=102)
            }
        })
}
// Return the net internal-prefix growth. Its terminating NUL is included in the limit.
fn admit(units: &[u16]) -> Result<usize, PathRejection> {
    use PathRejection::*;
    if units.contains(&0) {
        return Err(Nul);
    }
    if units.is_empty() || units.len() > 32766 {
        return Err(Length);
    }
    let verbatim = units.starts_with(&[92, 92, 63, 92]);
    if verbatim && units.contains(&(b'/' as u16)) {
        return Err(UnsupportedSpelling);
    }
    let tail = if verbatim {
        let rest = &units[4..];
        if drive(rest) {
            &rest[3..]
        } else if rest.len() >= 4 && ascii_eq(&rest[..4], "UNC\\") {
            unc_tail(&rest[4..])?
        } else if volume(rest) {
            &rest[45..]
        } else {
            return Err(NotAbsoluteFilesystem);
        }
    } else if drive(units) {
        &units[3..]
    } else if units.len() >= 2 && separator(units[0]) && separator(units[1]) {
        unc_tail(&units[2..])?
    } else {
        return Err(NotAbsoluteFilesystem);
    };
    if !tail.is_empty() {
        for part in tail.split(|&u| separator(u)) {
            admit_component(part)?;
        }
    }
    let extra = if verbatim {
        0
    } else if drive(units) {
        4
    } else {
        6 // \\server\share becomes \\?\UNC\server\share.
    };
    if units.len() + extra + 1 > 32767 {
        return Err(Length);
    }
    Ok(extra)
}
fn unc_tail(units: &[u16]) -> Result<&[u16], PathRejection> {
    let server_end = units
        .iter()
        .position(|&u| separator(u))
        .ok_or(PathRejection::NotAbsoluteFilesystem)?;
    admit_component_spelling(&units[..server_end])?;
    let rest = &units[server_end + 1..];
    let share_end = rest
        .iter()
        .position(|&u| separator(u))
        .unwrap_or(rest.len());
    let share = &rest[..share_end];
    // UNC host/share names are not local DOS-device basenames. All other checks remain.
    admit_component_spelling(share)?;
    if ascii_eq(share, "pipe") {
        return Err(PathRejection::PipeShare);
    }
    Ok(if share_end < rest.len() {
        &rest[share_end + 1..]
    } else {
        &[]
    })
}
fn admit_component_spelling(part: &[u16]) -> Result<(), PathRejection> {
    use PathRejection::*;
    if part.is_empty()
        || part.len() > 255
        || matches!(part.last(), Some(32 | 46))
        || part
            .iter()
            .any(|&u| u < 32 || matches!(u, 34 | 42 | 58 | 60 | 62 | 63 | 124))
    {
        return Err(UnsupportedSpelling);
    }
    Ok(())
}
fn admit_component(part: &[u16]) -> Result<(), PathRejection> {
    admit_component_spelling(part)?;
    let stem_end = part.iter().position(|&u| u == 46).unwrap_or(part.len());
    let mut stem = &part[..stem_end];
    while stem.last() == Some(&32) {
        stem = &stem[..stem.len() - 1];
    }
    if ["CON", "PRN", "AUX", "NUL", "CLOCK$", "CONIN$", "CONOUT$"]
        .iter()
        .any(|name| ascii_eq(stem, name))
        || (stem.len() == 4
            && (ascii_eq(&stem[..3], "COM") || ascii_eq(&stem[..3], "LPT"))
            && matches!(stem[3], 48..=57 | 0x00b9 | 0x00b2 | 0x00b3))
    {
        return Err(PathRejection::DeviceAlias);
    }
    Ok(())
}
