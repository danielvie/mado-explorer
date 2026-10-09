use super::*;

fn path(value: &str) -> NativePathBuf {
    NativePathBuf::from(value)
}
fn wide_path(units: &[u16]) -> NativePathBuf {
    NativePathBuf::from(OsString::from_wide(units))
}
fn raw_wide(units: &[u16]) -> Vec<u8> {
    let mut bytes = vec![0; HEADER_BYTES];
    bytes[..4].copy_from_slice(&(HEADER_BYTES as u32).to_le_bytes());
    bytes[16..20].copy_from_slice(&1u32.to_le_bytes());
    bytes.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
    bytes
}
fn raw_ansi(bytes: &[u8]) -> Vec<u8> {
    let mut payload = vec![0; HEADER_BYTES];
    payload[..4].copy_from_slice(&(HEADER_BYTES as u32).to_le_bytes());
    payload.extend_from_slice(bytes);
    payload
}
fn valid() -> Vec<u8> {
    FilePacket::snapshot(&[path("C:\\entry")], Intent::Copy)
        .unwrap()
        .encode_hdrop()
        .unwrap()
}
fn assert_bad(payload: &[u8], error: CodecError) {
    assert_eq!(decode_packet(payload, None).unwrap_err(), error);
}

#[test]
fn immutable_native_snapshot_roundtrip_unicode_surrogates_unc_and_long_paths() {
    let mut unpaired: Vec<u16> = "C:\\native-".encode_utf16().collect();
    unpaired.extend([0xd800, 0x0061, 0xdc00]);
    let mut source = vec![
        path("C:\\données\\漢字-𐐀.txt"),
        wide_path(&unpaired),
        path("\\\\server\\share\\file"),
        path("\\\\?\\UNC\\server\\share\\données"),
        path(&format!("\\\\?\\C:\\{}\\file", "long\\".repeat(100))),
        path("\\\\?\\Volume{01234567-89ab-cdef-0123-456789abcdef}\\item"),
    ];
    let captured = FilePacket::snapshot(&source, Intent::Move).unwrap();
    source.clear();
    source.push(path("D:\\new-selection"));
    let encoded = captured.encode_hdrop().unwrap();
    let before = encoded.clone();
    let decoded = decode_packet(&encoded, Some(&captured.encode_preferred_effect())).unwrap();
    assert_eq!(decoded, captured);
    assert_eq!(decoded.paths().len(), 6);
    assert_eq!(encoded, before);
    assert_eq!(
        decoded.paths()[1]
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        unpaired
    );
    assert!(decoded.paths()[4].as_os_str().encode_wide().count() > 260);
}

#[test]
fn copy_move_missing_and_ambiguous_intent() {
    for intent in [Intent::Copy, Intent::Move] {
        let packet = FilePacket::snapshot(&[path("C:\\entry")], intent).unwrap();
        assert_eq!(
            decode_packet(
                &packet.encode_hdrop().unwrap(),
                Some(&packet.encode_preferred_effect())
            )
            .unwrap(),
            packet
        );
    }
    assert_eq!(
        decode_preferred_effect(None).unwrap(),
        (Intent::Copy, Some(IntentWarning::MissingPreferredEffect))
    );
    for value in [0u32, 3, 4, 5, 6, 7, 0x80000002, u32::MAX] {
        assert_eq!(
            decode_preferred_effect(Some(&value.to_le_bytes())).unwrap(),
            (
                Intent::Copy,
                Some(IntentWarning::AmbiguousPreferredEffect(value))
            )
        );
    }
    for bytes in [
        vec![],
        vec![2, 0, 0],
        vec![2, 0, 0, 0, 1],
        vec![0; MAX_EFFECT_BYTES + 1],
    ] {
        assert_eq!(
            decode_preferred_effect(Some(&bytes)).unwrap_err(),
            CodecError::InvalidEffect
        );
    }
    let mut padded = 2u32.to_le_bytes().to_vec();
    padded.resize(MAX_EFFECT_BYTES, 0);
    assert_eq!(
        decode_preferred_effect(Some(&padded)).unwrap(),
        (Intent::Move, None)
    );
}

#[test]
fn header_offsets_boolean_alignment_and_odd_utf16_rejected() {
    assert_bad(&[0; HEADER_BYTES - 1], CodecError::HeaderTooShort);
    for offset in [0u32, 19, 21, valid().len() as u32, u32::MAX] {
        let mut data = valid();
        data[..4].copy_from_slice(&offset.to_le_bytes());
        assert_bad(&data, CodecError::InvalidOffset);
    }
    for field in [12, 16] {
        for value in [2u32, u32::MAX] {
            let mut data = valid();
            data[field..field + 4].copy_from_slice(&value.to_le_bytes());
            assert_bad(&data, CodecError::InvalidBoolean);
        }
    }
    let mut data = valid();
    data.push(0);
    assert_bad(&data, CodecError::OddUtf16);
    // Legitimate aligned offset beyond the header is honored, not assumed to be 20.
    let mut data = valid();
    data.splice(20..20, [9, 9, 9, 9]);
    data[..4].copy_from_slice(&24u32.to_le_bytes());
    assert_eq!(
        decode_packet(&data, None).unwrap().paths(),
        &[path("C:\\entry")]
    );
}

#[test]
fn complete_terminators_required_no_truncated_prefix_or_nonzero_trailer() {
    let data = valid();
    for trim in [2, 4] {
        assert_bad(&data[..data.len() - trim], CodecError::MissingTerminator);
    }
    let mut data = valid();
    data.extend_from_slice(&[1, 0]);
    assert_bad(&data, CodecError::TrailingData);
    let mut padded = valid();
    padded.extend_from_slice(&[0; 32]);
    assert!(decode_packet(&padded, None).is_ok());
    assert_bad(&raw_wide(&[0]), CodecError::MissingTerminator);
    assert_bad(&raw_wide(&[0, 0]), CodecError::EmptyList);
    assert_bad(&raw_ansi(&[0]), CodecError::MissingTerminator);
    assert_bad(&raw_ansi(&[0, 0]), CodecError::EmptyList);
    let mut units: Vec<u16> = "C:\\first".encode_utf16().collect();
    units.push(0);
    units.extend("C:\\second".encode_utf16());
    assert_bad(&raw_wide(&units), CodecError::MissingTerminator);
    let mut empty_then_path = vec![0, 0];
    empty_then_path.extend("C:\\entry".encode_utf16());
    assert_bad(&raw_wide(&empty_then_path), CodecError::EmptyList);
}

#[test]
fn size_count_path_limits_and_arithmetic_overflow() {
    assert_bad(&vec![0; MAX_PAYLOAD_BYTES + 1], CodecError::PayloadTooLarge);
    assert_eq!(
        add_path_size(0, usize::MAX),
        Err(CodecError::ArithmeticOverflow)
    );
    assert_eq!(
        add_path_size(usize::MAX, 1),
        Err(CodecError::ArithmeticOverflow)
    );
    assert_eq!(
        add_path_size(MAX_PAYLOAD_BYTES, 1),
        Err(CodecError::PayloadTooLarge)
    );
    assert_eq!(
        FilePacket::snapshot(&[], Intent::Copy).unwrap_err(),
        CodecError::EmptyList
    );
    let mut paths = vec![path("C:\\a"); MAX_PATHS];
    assert_eq!(
        FilePacket::snapshot(&paths, Intent::Copy)
            .unwrap()
            .paths()
            .len(),
        MAX_PATHS
    );
    paths.push(path("C:\\b"));
    assert_eq!(
        FilePacket::snapshot(&paths, Intent::Copy).unwrap_err(),
        CodecError::TooManyPaths
    );
    let mut units = Vec::new();
    for _ in 0..=MAX_PATHS {
        units.extend("C:\\a".encode_utf16());
        units.push(0);
    }
    units.push(0);
    assert_bad(&raw_wide(&units), CodecError::TooManyPaths);
    let longest = format!("\\\\?\\C:\\{}", "a".repeat(MAX_PATH_UNITS - 7));
    let packet = FilePacket::snapshot(&[path(&longest)], Intent::Copy).unwrap();
    assert_eq!(
        decode_packet(
            &packet.encode_hdrop().unwrap(),
            Some(&packet.encode_preferred_effect())
        )
        .unwrap(),
        packet
    );
    let too_long = format!("{longest}a");
    assert_eq!(
        FilePacket::snapshot(&[path(&too_long)], Intent::Copy).unwrap_err(),
        CodecError::PathTooLong
    );
    let mut units: Vec<u16> = too_long.encode_utf16().collect();
    units.extend([0, 0]);
    assert_bad(&raw_wide(&units), CodecError::PathTooLong);
    // Individually valid paths can still exceed the aggregate packet budget.
    let oversized = vec![path(&longest); 129];
    assert_eq!(
        FilePacket::snapshot(&oversized, Intent::Copy).unwrap_err(),
        CodecError::PayloadTooLarge
    );
}

#[test]
fn only_absolute_filesystem_namespaces_not_device_aliases_or_shell_strings() {
    for value in [
        "relative",
        "C:relative",
        "\\rooted",
        "https://server/path",
        "shell:Downloads",
        "\\\\server",
        "\\\\server\\",
        "\\\\?\\GLOBALROOT\\Device\\HarddiskVolume1\\file",
        "\\\\?\\pipe\\name",
        "\\??\\C:\\file",
        "\\\\?\\Volume{not-a-guid}\\file",
    ] {
        assert!(
            FilePacket::snapshot(&[path(value)], Intent::Copy).is_err(),
            "accepted {value}"
        );
        let mut units: Vec<u16> = value.encode_utf16().collect();
        units.extend([0, 0]);
        assert!(
            decode_packet(&raw_wide(&units), None).is_err(),
            "decoded {value}"
        );
    }
    for value in [
        "\\\\.\\pipe\\name",
        "C:\\NUL.txt",
        "C:\\COM1",
        "C:\\lpt².txt",
        "C:\\CONIN$",
        "C:\\CON .txt",
        "C:\\entry:stream",
        "C:\\trailing.",
        "C:\\trailing ",
        "C:\\..\\file",
        "C:\\.\\file",
        "C:\\wild*",
        "C:\\quote\"",
        "C:\\a\0b",
        "\\\\?\\C:\\wrong/slash",
        "\\\\?\\C:\\NUL",
    ] {
        assert!(
            FilePacket::snapshot(&[path(value)], Intent::Copy).is_err(),
            "accepted {value}"
        );
    }
    for value in [
        "C:\\",
        "D:/folder/file",
        "\\\\server\\share",
        "\\\\?\\C:\\trailing. ",
    ] {
        assert!(
            FilePacket::snapshot(&[path(value)], Intent::Copy).is_ok(),
            "refused {value}"
        );
    }
}

#[test]
fn ansi_uses_native_acp_and_requires_complete_list() {
    let decoded = decode_packet(
        &raw_ansi(b"C:\\first\0\\\\server\\share\\second\0\0"),
        Some(&2u32.to_le_bytes()),
    )
    .unwrap();
    assert_eq!(
        decoded.paths(),
        &[path("C:\\first"), path("\\\\server\\share\\second")]
    );
    assert_eq!(decoded.intent(), Intent::Move);
    assert_bad(&raw_ansi(b"C:\\first\0"), CodecError::MissingTerminator);
    assert_bad(&raw_ansi(b"C:\\first\0\0X"), CodecError::TrailingData);
    // Verify byte interpretation against Windows' measured ACP, not a UTF-8 guess.
    let bytes = [b'C', b':', b'\\', 0xc3, 0xa9];
    let mut units = [0u16; 16];
    let written = unsafe {
        MultiByteToWideChar(
            CP_ACP,
            MB_ERR_INVALID_CHARS,
            bytes.as_ptr(),
            bytes.len() as i32,
            units.as_mut_ptr(),
            units.len() as i32,
        )
    };
    let mut terminated = bytes.to_vec();
    terminated.extend([0, 0]);
    if written > 0 {
        assert_eq!(
            decode_packet(&raw_ansi(&terminated), None).unwrap().paths(),
            &[wide_path(&units[..written as usize])]
        );
    } else {
        let code = unsafe { windows_sys::Win32::Foundation::GetLastError() };
        assert_bad(&raw_ansi(&terminated), CodecError::AnsiConversion(code));
    }
}

#[test]
fn bounded_generated_malformed_inputs_never_panic() {
    let base = valid();
    for len in 0..base.len() {
        let _ = decode_packet(&base[..len], Some(&2u32.to_le_bytes()));
    }
    for index in 0..base.len() {
        for byte in [0, 1, 0x7f, 0x80, 0xff] {
            let mut changed = base.clone();
            changed[index] = byte;
            let _ = decode_packet(&changed, None);
        }
    }
}
