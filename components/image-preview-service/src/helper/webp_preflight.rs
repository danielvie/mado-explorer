//! Allocation-free RIFF range and frame-header guards. This is not a WebP codec.

use super::{DecodeLimits, ErrorKind, check_dimensions};

fn malformed() -> ErrorKind {
    ErrorKind::Malformed("invalid WebP chunk extent or frame header".into())
}

fn u24(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0])
}

// Returns only an in-bounds payload. All arithmetic is on bounded slice offsets.
fn chunk<'a>(bytes: &'a [u8], offset: &mut usize) -> Result<([u8; 4], &'a [u8]), ErrorKind> {
    let header = bytes
        .get(*offset..)
        .and_then(|b| b.get(..8))
        .ok_or_else(malformed)?;
    let tag = header[..4].try_into().unwrap();
    let length = u32::from_le_bytes(header[4..8].try_into().unwrap()) as u64;
    let start = *offset as u64 + 8;
    let end = start + length;
    let padded_end = end + (length & 1);
    if padded_end > bytes.len() as u64 {
        return Err(malformed());
    }
    *offset = padded_end as usize;
    Ok((tag, &bytes[start as usize..end as usize]))
}

fn frame_header(
    tag: [u8; 4],
    bytes: &[u8],
    expected: Option<(u32, u32)>,
    limits: DecodeLimits,
) -> Result<(), ErrorKind> {
    let dimensions = match &tag {
        b"VP8 " => {
            let header = bytes.get(..10).ok_or_else(malformed)?;
            let bits = u24(&header[..3]);
            if bits & 1 != 0 || header[3..6] != [0x9d, 0x01, 0x2a] {
                return Err(malformed());
            }
            // The first partition is allocated from this declared length before reading.
            if (bits >> 5) as u64 > (bytes.len() - 10) as u64 {
                return Err(malformed());
            }
            (
                u32::from(u16::from_le_bytes(header[6..8].try_into().unwrap()) & 0x3fff),
                u32::from(u16::from_le_bytes(header[8..10].try_into().unwrap()) & 0x3fff),
            )
        }
        b"VP8L" => {
            let header = bytes.get(..5).ok_or_else(malformed)?;
            let bits = u32::from_le_bytes(header[1..5].try_into().unwrap());
            if header[0] != 0x2f || bits >> 29 != 0 {
                return Err(malformed());
            }
            ((bits & 0x3fff) + 1, ((bits >> 14) & 0x3fff) + 1)
        }
        _ => return Ok(()),
    };
    check_dimensions(dimensions.0, dimensions.1, limits)?;
    if expected.is_some_and(|expected| expected != dimensions) {
        return Err(ErrorKind::Malformed(
            "WebP canvas/frame dimensions differ".into(),
        ));
    }
    Ok(())
}

pub(crate) fn validate(bytes: &[u8], limits: DecodeLimits) -> Result<(), ErrorKind> {
    let header = bytes.get(..12).ok_or_else(malformed)?;
    if &header[..4] != b"RIFF" || &header[8..12] != b"WEBP" {
        return Err(malformed());
    }
    let riff_end = u64::from(u32::from_le_bytes(header[4..8].try_into().unwrap())) + 8;
    // Do not pass unchecked trailing bytes to image-webp's permissive extended scan.
    if riff_end != bytes.len() as u64 {
        return Err(malformed());
    }
    let mut offset = 12;
    let mut canvas = None;
    let mut first = true;
    while offset < bytes.len() {
        let (tag, payload) = chunk(bytes, &mut offset)?;
        if first && &tag == b"VP8X" {
            let extended = payload.get(..10).ok_or_else(malformed)?;
            let dimensions = (u24(&extended[4..7]) + 1, u24(&extended[7..10]) + 1);
            check_dimensions(dimensions.0, dimensions.1, limits)?;
            canvas = Some(dimensions);
        }
        first = false;
        if &tag == b"ANMF" {
            let frame = payload.get(..16).ok_or_else(malformed)?;
            let dimensions = (u24(&frame[6..9]) + 1, u24(&frame[9..12]) + 1);
            check_dimensions(dimensions.0, dimensions.1, limits)?;
            let (canvas_width, canvas_height) = canvas.ok_or_else(malformed)?;
            if u64::from(u24(&frame[..3])) * 2 + u64::from(dimensions.0) > u64::from(canvas_width)
                || u64::from(u24(&frame[3..6])) * 2 + u64::from(dimensions.1)
                    > u64::from(canvas_height)
            {
                return Err(malformed());
            }
            // Exactly one RIFF nesting level; ANMF stores a 16-byte frame header,
            // then ALPH/VP8/VP8L chunks. Never follow arbitrary nested containers.
            let mut suboffset = 16;
            while suboffset < payload.len() {
                let (subtag, subpayload) = chunk(payload, &mut suboffset)?;
                frame_header(subtag, subpayload, Some(dimensions), limits)?;
            }
        } else {
            frame_header(tag, payload, canvas, limits)?;
        }
    }
    if first {
        return Err(malformed());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> DecodeLimits {
        DecodeLimits {
            max_input_bytes: 1024,
            max_width: 256,
            max_height: 256,
            max_output_bytes: 1024,
            max_buffer_bytes: 1024,
            max_decoder_bytes: 1024,
        }
    }

    fn append(bytes: &mut Vec<u8>, tag: &[u8; 4], payload: &[u8]) {
        bytes.extend_from_slice(tag);
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(payload);
        if payload.len() & 1 != 0 {
            bytes.push(0);
        }
    }

    fn riff(chunks: &[u8]) -> Vec<u8> {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend_from_slice(&(chunks.len() as u32 + 4).to_le_bytes());
        bytes.extend_from_slice(b"WEBP");
        bytes.extend_from_slice(chunks);
        bytes
    }

    fn vp8(width: u16, height: u16) -> Vec<u8> {
        let mut bytes = vec![0, 0, 0, 0x9d, 0x01, 0x2a];
        bytes.extend_from_slice(&width.to_le_bytes());
        bytes.extend_from_slice(&height.to_le_bytes());
        bytes
    }

    fn extended(width: u16, height: u16, animated: bool) -> Vec<u8> {
        let mut payload = vec![if animated { 2 } else { 0 }, 0, 0, 0];
        payload.extend_from_slice(&(u32::from(width) - 1).to_le_bytes()[..3]);
        payload.extend_from_slice(&(u32::from(height) - 1).to_le_bytes()[..3]);
        let mut chunks = Vec::new();
        append(&mut chunks, b"VP8X", &payload);
        chunks
    }

    #[test]
    fn forged_extent_never_reaches_a_decoder() {
        let mut chunks = extended(2, 3, false);
        append(&mut chunks, b"VP8 ", &vp8(2, 3));
        chunks.extend_from_slice(b"EXIF");
        chunks.extend_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(
            validate(&riff(&chunks), limits()),
            Err(ErrorKind::Malformed(_))
        ));
        let mut bytes = riff(&chunks);
        bytes[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(
            validate(&bytes, limits()),
            Err(ErrorKind::Malformed(_))
        ));
        let mut odd = Vec::new();
        append(&mut odd, b"VP8 ", &vp8(2, 3));
        append(&mut odd, b"EXIF", &[0]);
        validate(&riff(&odd), limits()).unwrap();
        odd.pop();
        assert!(matches!(
            validate(&riff(&odd), limits()),
            Err(ErrorKind::Malformed(_))
        ));
    }

    #[test]
    fn outer_canvas_cannot_hide_huge_or_mismatched_vp8() {
        for (width, height, oversize) in [(16383, 16383, true), (3, 3, false), (2, 3, false)] {
            let mut chunks = extended(2, 3, false);
            append(&mut chunks, b"VP8 ", &vp8(width, height));
            match validate(&riff(&chunks), limits()) {
                Err(ErrorKind::Oversize("dimensions")) => assert!(oversize),
                Err(ErrorKind::Malformed(_)) => assert_eq!((width, height), (3, 3)),
                Ok(()) => assert_eq!((width, height), (2, 3)),
                other => panic!("unexpected preflight result: {other:?}"),
            }
        }
        let mut chunks = extended(2, 3, false);
        let mut frame = vp8(2, 3);
        frame[..3].copy_from_slice(&[0xe0, 0xff, 0xff]);
        append(&mut chunks, b"VP8 ", &frame);
        assert!(matches!(
            validate(&riff(&chunks), limits()),
            Err(ErrorKind::Malformed(_))
        ));
    }

    #[test]
    fn animation_subchunks_are_bounded_before_first_frame_decode() {
        let mut chunks = extended(2, 3, true);
        let mut frame = vec![0; 16];
        frame[6] = 1;
        frame[9] = 2;
        append(&mut frame, b"ALPH", &[0, 1, 2, 3, 4, 5, 6]);
        append(&mut frame, b"VP8 ", &vp8(16383, 16383));
        append(&mut chunks, b"ANMF", &frame);
        assert!(matches!(
            validate(&riff(&chunks), limits()),
            Err(ErrorKind::Oversize("dimensions"))
        ));
        let mut chunks = extended(2, 3, true);
        frame.truncate(16);
        frame.extend_from_slice(b"EXIF");
        frame.extend_from_slice(&u32::MAX.to_le_bytes());
        append(&mut chunks, b"ANMF", &frame);
        assert!(matches!(
            validate(&riff(&chunks), limits()),
            Err(ErrorKind::Malformed(_))
        ));
    }
}
