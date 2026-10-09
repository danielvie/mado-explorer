use eframe::egui::{
    FontData, FontDefinitions, FontFamily,
    epaint::text::{Fonts, TextOptions},
};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
    sync::Arc,
};

const MAX_FONT_BYTES: usize = 24 * 1024 * 1024;
const MAX_TOTAL_BYTES: usize = 64 * 1024 * 1024;
// Fixed installed assets, not a font-directory scan. Use face zero of each TTC;
// Yu Gothic is an alternative only when MS Gothic cannot be loaded.
const FALLBACKS: &[(&str, &[&str])] = &[
    ("Japanese", &["msgothic.ttc", "YuGothR.ttc"]),
    ("Korean", &["malgun.ttf"]),
    ("Chinese", &["msyh.ttc"]),
    ("symbols", &["seguisym.ttf"]),
    ("historic letters", &["seguihis.ttf"]),
];

pub struct InstalledFonts {
    pub definitions: FontDefinitions,
    pub warning: Option<String>,
}

// Called once before run_native, never from UI update. Optional failures retain
// builtins; no installed proprietary assets are embedded or redistributed.
pub fn load_installed() -> InstalledFonts {
    match font_directory() {
        Ok(directory) => load_from(&directory),
        Err(error) => InstalledFonts {
            definitions: FontDefinitions::default(),
            warning: Some(format!(
                "Installed font fallback unavailable: {error}. Builtin fonts remain; some Unicode characters may appear as boxes."
            )),
        },
    }
}

fn load_from(directory: &Path) -> InstalledFonts {
    let mut definitions = FontDefinitions::default();
    let mut total = 0;
    let mut unavailable = Vec::new();
    for (coverage, candidates) in FALLBACKS {
        let mut loaded = false;
        let mut errors = Vec::new();
        for filename in *candidates {
            let result = read_font(
                &directory.join(filename),
                MAX_FONT_BYTES.min(MAX_TOTAL_BYTES - total),
            );
            match result {
                Ok(data) => {
                    let name = format!("installed:{filename}");
                    total += data.font.len();
                    definitions.font_data.insert(name.clone(), data);
                    for family in [FontFamily::Proportional, FontFamily::Monospace] {
                        definitions
                            .families
                            .entry(family)
                            .or_default()
                            .push(name.clone());
                    }
                    loaded = true;
                    break;
                }
                Err(error) => errors.push(format!("{filename}: {error}")),
            }
        }
        if !loaded {
            unavailable.push(format!("{coverage} ({})", errors.join("; ")));
        }
    }
    InstalledFonts {
        definitions,
        warning: (!unavailable.is_empty()).then(|| format!("Installed font fallback limited: {}. Builtin fonts remain; uncovered Unicode characters may appear as boxes.", unavailable.join(", "))),
    }
}

fn read_font(path: &Path, limit: usize) -> io::Result<Arc<FontData>> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > limit as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "font is empty, nonregular or exceeds the byte budget",
        ));
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.is_empty() || bytes.len() > limit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "font changed or exceeds the byte budget",
        ));
    }
    let data = Arc::new(FontData::from_owned(bytes)); // index zero supports TTF/TTC
    // epaint's public constructor panics on invalid font data, rather than
    // returning an error. Validate each optional asset at this startup boundary
    // so a damaged installed file cannot later panic UI font initialization.
    let mut probe = FontDefinitions::empty();
    probe
        .font_data
        .insert("installed-probe".into(), data.clone());
    if std::panic::catch_unwind(|| Fonts::new(TextOptions::default(), probe)).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "epaint could not parse installed font face zero",
        ));
    }
    Ok(data)
}

#[cfg(windows)]
fn font_directory() -> io::Result<std::path::PathBuf> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows_sys::Win32::{
        System::Com::CoTaskMemFree,
        UI::Shell::{FOLDERID_Fonts, SHGetKnownFolderPath},
    };
    let mut pointer = std::ptr::null_mut();
    // SAFETY: documented known-folder GUID, current user, output allocation owned
    // by the caller and freed with CoTaskMemFree. No registry edits or enumeration.
    let result =
        unsafe { SHGetKnownFolderPath(&FOLDERID_Fonts, 0, std::ptr::null_mut(), &mut pointer) };
    if result < 0 {
        return Err(io::Error::other(format!(
            "SHGetKnownFolderPath(Fonts) failed: HRESULT 0x{:08X}",
            result as u32
        )));
    }
    if pointer.is_null() {
        return Err(io::Error::other("Windows returned no Fonts location"));
    }
    // SAFETY: a successful call returns a NUL-terminated UTF-16 allocation.
    let path = unsafe {
        let mut length = 0;
        while *pointer.add(length) != 0 {
            length += 1;
        }
        let path = OsString::from_wide(std::slice::from_raw_parts(pointer, length));
        CoTaskMemFree(pointer.cast());
        path
    };
    Ok(path.into())
}

#[cfg(not(windows))]
fn font_directory() -> io::Result<std::path::PathBuf> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "installed Windows fonts require Windows",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::FontId;

    #[cfg(windows)]
    #[test]
    #[ignore = "reference-host probe needs optional Windows font assets; run task test:fonts"]
    fn s006a_installed_windows_loader_covers_japanese_and_common_unicode() {
        let directory = font_directory().expect("Windows Fonts known-folder lookup");
        let installed = load_installed();
        assert!(
            installed.warning.is_none(),
            "{}",
            installed.warning.as_deref().unwrap_or("")
        );
        let builtin = FontDefinitions::default();
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            assert!(
                installed.definitions.families[&family].starts_with(&builtin.families[&family]),
                "builtin Latin priority must remain unchanged"
            );
        }
        let mut bytes = 0;
        for (name, data) in &installed.definitions.font_data {
            if name.starts_with("installed:") {
                println!(
                    "{name}, face {}, {} bytes, from {}",
                    data.index,
                    data.font.len(),
                    directory.display()
                );
                assert_eq!(data.index, 0);
                assert!(data.font.len() <= MAX_FONT_BYTES);
                bytes += data.font.len();
            }
        }
        assert!(bytes <= MAX_TOTAL_BYTES);
        let mut fonts = Fonts::new(TextOptions::default(), installed.definitions);
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            let id = FontId::new(14.0, family);
            for sample in [
                "日本語.txt",
                "ひらがな-カタカナ",
                "中文文件",
                "한글",
                "é-Å-Ελληνικά-Кириллица",
                "\u{10437}",
            ] {
                // has_glyphs in epaint 0.36.2 compares face IDs, incorrectly
                // reporting false for supported characters on the replacement
                // face. characters() exposes actual charmap coverage instead.
                let mut family = fonts.fonts.font(&id.family);
                let characters = family.characters();
                for character in sample.chars() {
                    assert!(
                        characters.contains_key(&character),
                        "missing charmap coverage for {character:?} in {:?}",
                        id.family
                    );
                }
                println!(
                    "{:?} covers {sample:?}; first character available in {:?}",
                    id.family,
                    characters[&sample.chars().next().unwrap()]
                );
            }
        }
    }

    #[test]
    fn s006a_missing_optional_fonts_keep_builtin_latin_and_report_limits() {
        let path = std::env::temp_dir().join(format!(
            "mado-s006a-missing-fonts-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&path).unwrap();
        let installed = load_from(&path);
        let oversized = path.join("bounded.ttf");
        std::fs::write(&oversized, b"not a font").unwrap();
        assert_eq!(
            read_font(&oversized, 1).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        std::fs::remove_file(oversized).unwrap();
        std::fs::remove_dir(&path).unwrap();
        assert_eq!(installed.definitions, FontDefinitions::default());
        let warning = installed.warning.unwrap();
        assert!(warning.contains("Japanese") && warning.contains("Builtin fonts remain"));
        let mut fonts = Fonts::new(TextOptions::default(), installed.definitions);
        assert!(fonts.has_glyphs(&FontId::proportional(14.0), "Name ^ v file.txt"));
        assert!(!fonts.has_glyphs(&FontId::proportional(14.0), "日本語"));
    }
}
