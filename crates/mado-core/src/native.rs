use std::ffi::OsStr;

const DIAGNOSTIC_PREFIX: &str = "[non-Unicode: ";

// Diagnostic text is not a path serialization. Valid Unicode stays unchanged;
// malformed native units are shown explicitly instead of replacement characters.
pub fn display(value: &OsStr) -> String {
    if let Some(text) = value.to_str() {
        return text.to_owned();
    }
    #[cfg(windows)]
    let escaped: String = {
        use std::os::windows::ffi::OsStrExt;
        std::char::decode_utf16(value.encode_wide())
            .map(|unit| match unit {
                Ok(character) => character.escape_debug().to_string(),
                Err(error) => format!("\\u{{{:04X}}}", error.unpaired_surrogate()),
            })
            .collect()
    };
    #[cfg(not(windows))]
    let escaped = format!("{value:?}");
    format!("{DIAGNOSTIC_PREFIX}{escaped}]")
}

pub fn is_diagnostic(text: &str) -> bool {
    text.starts_with(DIAGNOSTIC_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_display_is_unmodified_including_literal_replacement_character() {
        for name in [
            "日本語-é-\u{10437}.txt",
            "actual-�.txt",
            r"C:\folder\literal\u{D800}",
        ] {
            assert_eq!(display(OsStr::new(name)), name);
        }
    }

    #[cfg(windows)]
    #[test]
    fn malformed_native_units_are_distinct_diagnostics_not_replacement_text() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};
        let a = OsString::from_wide(&[0x78, 0xD800]);
        let b = OsString::from_wide(&[0x78, 0xD801]);
        assert_eq!(a.to_string_lossy(), b.to_string_lossy());
        assert_eq!(display(&a), "[non-Unicode: x\\u{D800}]");
        assert_eq!(display(&b), "[non-Unicode: x\\u{D801}]");
    }
}
