//! Audited JPEG bulk-buffer charges, not a complete codec allocator bound.
//! See HANDOFF.md for pinned source evidence and remaining PNG/WebP allocations.

use crate::{ErrorKind, check_bytes};

fn sum(parts: &[u64]) -> Result<u64, ErrorKind> {
    parts.iter().try_fold(0u64, |sum, part| {
        sum.checked_add(*part)
            .ok_or(ErrorKind::Oversize("component buffer"))
    })
}

fn product(parts: &[u64]) -> Result<u64, ErrorKind> {
    parts.iter().try_fold(1u64, |product, part| {
        product
            .checked_mul(*part)
            .ok_or(ErrorKind::Oversize("component buffer"))
    })
}

pub(crate) fn jpeg_constructor(input_capacity: u64, file_bytes: u64) -> Result<u64, ErrorKind> {
    // Rust 1.95 Cursor::read_to_end reserves once on an empty Vec, then copies
    // the slice. Capacity is max(F, 8), with no old growing allocation to overlap.
    // Header metadata payloads together cannot exceed the bounded input.
    sum(&[input_capacity, file_bytes.max(8), file_bytes])
}

pub(crate) fn jpeg_working(width: u32, height: u32) -> Result<u64, ErrorKind> {
    let columns = u64::from(width).div_ceil(8) + 3;
    let rows = u64::from(height).div_ceil(8) + 3;
    // zune-jpeg 0.5.15 mcu.rs and mcu_prog.rs allocate i16 coefficients:
    // 128 * MCU_x * MCU_y * h_sample * v_sample per component, at most four.
    // ceil(w/(8*hmax))*h_sample <= ceil(w/8)+3, likewise for height.
    // An exceptional grayscale-output path multiplies MCU_y by v_max/h_max,
    // bounded by 4. Charge it even though the adapter normally requests RGB.
    let coefficients = product(&[128, 4, 4, columns, rows])?;
    // width_stride <= 8*columns. Per component: raw_coeff <= 32*stride i16,
    // two saved rows <= 8*stride, first row <= 64*stride, upsample <= 256*stride.
    // 8192*stride covers four components, replacement/growth overlap, and
    // <=8*stride i16 upsampler scratch. Do not mistake this for metadata accounting.
    let row_buffers = product(&[8192, 8, columns])?;
    sum(&[coefficients, row_buffers])
}

pub(crate) fn jpeg_peak(
    input_capacity: u64,
    file_bytes: u64,
    width: u32,
    height: u32,
    native: u64,
    rgba: u64,
) -> Result<u64, ErrorKind> {
    let constructor = jpeg_constructor(input_capacity, file_bytes)?;
    let orientation = sum(&[constructor, file_bytes])?; // Exif clone while headers live
    let decode = sum(&[constructor, native, jpeg_working(width, height)?])?;
    // Decoder/input duplicate/working buffers are dropped before conversion.
    let conversion = sum(&[input_capacity, native, rgba])?;
    let rotation = sum(&[input_capacity, product(&[2, rgba])?])?;
    Ok(orientation.max(decode).max(conversion).max(rotation))
}

pub(crate) fn admit(bytes: u64, maximum: u64) -> Result<(), ErrorKind> {
    check_bytes(bytes, maximum, "component buffer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_and_phase_boundaries_are_checked_without_decoding() {
        assert_eq!(jpeg_constructor(100, 100).unwrap(), 300);
        assert_eq!(jpeg_constructor(2, 2).unwrap(), 12);
        let peak = jpeg_peak(100, 100, 2, 3, 18, 24).unwrap();
        // Four-component coefficient ceiling and all row buffers, including overlap.
        assert_eq!(peak, 300 + 18 + 2048 * 4 * 4 + 8192 * 8 * 4);
        admit(peak, peak).unwrap();
        assert!(matches!(admit(peak, peak - 1), Err(ErrorKind::Oversize(_))));
        assert!(matches!(admit(peak, 0), Err(ErrorKind::Oversize(_))));
        assert!(matches!(
            jpeg_constructor(u64::MAX, 100),
            Err(ErrorKind::Oversize(_))
        ));
        assert!(matches!(
            jpeg_peak(100, 100, 1, 1, 1, u64::MAX),
            Err(ErrorKind::Oversize(_))
        ));
        assert!(jpeg_working(4096, 4096).unwrap() > 256 * 1024 * 1024);
    }
}
