use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use image::codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder};
use image::{ExtendedColorType, ImageEncoder};
use mado_image_preview::{DecodeLimits, ErrorKind, Format, decode};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let name = format!(
            "mado-p01-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let path = std::env::temp_dir().join(name);
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write(&self, name: impl AsRef<OsStr>, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(Path::new(name.as_ref()));
        fs::write(&path, bytes).unwrap();
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn limits() -> DecodeLimits {
    DecodeLimits {
        max_input_bytes: 1024 * 1024,
        max_width: 256,
        max_height: 256,
        max_output_bytes: 1024 * 1024,
        max_buffer_bytes: 4 * 1024 * 1024,
        max_decoder_bytes: 4 * 1024 * 1024,
    }
}

// Little-endian TIFF IFD containing just the Exif orientation tag.
fn exif(orientation: u8) -> Vec<u8> {
    vec![
        b'I',
        b'I',
        42,
        0,
        8,
        0,
        0,
        0,
        1,
        0,
        0x12,
        1,
        3,
        0,
        1,
        0,
        0,
        0,
        orientation,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]
}

fn encode(format: Format, orientation: Option<u8>) -> Vec<u8> {
    let rgb: Vec<u8> = [20, 50, 80, 110, 140, 170]
        .into_iter()
        .flat_map(|value| [value, value, value])
        .collect();
    let rgba: Vec<u8> = rgb
        .chunks_exact(3)
        .enumerate()
        .flat_map(|(i, pixel)| [pixel[0], pixel[1], pixel[2], 20 + i as u8 * 40])
        .collect();
    let mut bytes = Vec::new();
    match format {
        Format::Png => {
            let mut encoder = PngEncoder::new(&mut bytes);
            if let Some(orientation) = orientation {
                encoder.set_exif_metadata(exif(orientation)).unwrap();
            }
            encoder
                .write_image(&rgba, 2, 3, ExtendedColorType::Rgba8)
                .unwrap();
        }
        Format::Jpeg => {
            let mut encoder = JpegEncoder::new_with_quality(&mut bytes, 100);
            if let Some(orientation) = orientation {
                encoder.set_exif_metadata(exif(orientation)).unwrap();
            }
            encoder
                .write_image(&rgb, 2, 3, ExtendedColorType::Rgb8)
                .unwrap();
        }
        Format::WebP => {
            let mut encoder = WebPEncoder::new_lossless(&mut bytes);
            if let Some(orientation) = orientation {
                encoder.set_exif_metadata(exif(orientation)).unwrap();
            }
            encoder
                .write_image(&rgba, 2, 3, ExtendedColorType::Rgba8)
                .unwrap();
        }
    }
    bytes
}

#[test]
fn three_formats_metadata_rgba_and_read_only_native_path() {
    let fixture = Fixture::new();
    for format in [Format::Png, Format::Jpeg, Format::WebP] {
        let bytes = encode(format, None);
        // Content detection works even with a misleading extension and Unicode name.
        let path = fixture.write("日本語-é-𐐀.txt", &bytes);
        let preview = decode(&path, limits()).unwrap();
        assert_eq!((preview.width, preview.height), (2, 3));
        assert_eq!(preview.metadata.format, format);
        assert_eq!(preview.metadata.file_bytes, bytes.len() as u64);
        assert_eq!(preview.rgba.len(), 24);
        let expected_alpha = if format == Format::Jpeg {
            vec![255; 6]
        } else {
            vec![20, 60, 100, 140, 180, 220]
        };
        assert_eq!(
            preview
                .rgba
                .chunks_exact(4)
                .map(|p| p[3])
                .collect::<Vec<_>>(),
            expected_alpha
        );
        assert_eq!(fs::read(&path).unwrap(), bytes);
        // No file handle retained by the returned CPU bitmap.
        fs::remove_file(&path).unwrap();
    }
}

#[test]
fn all_eight_orientations_for_three_formats() {
    let fixture = Fixture::new();
    let orders = [
        [0, 1, 2, 3, 4, 5],
        [1, 0, 3, 2, 5, 4],
        [5, 4, 3, 2, 1, 0],
        [4, 5, 2, 3, 0, 1],
        [0, 2, 4, 1, 3, 5],
        [4, 2, 0, 5, 3, 1],
        [5, 3, 1, 4, 2, 0],
        [1, 3, 5, 0, 2, 4],
    ];
    for format in [Format::Png, Format::Jpeg, Format::WebP] {
        let path = fixture.write("base", &encode(format, None));
        let base = decode(&path, limits()).unwrap();
        for orientation in 1..=8 {
            let bytes = encode(format, Some(orientation));
            let path = fixture.write("oriented", &bytes);
            let preview = decode(&path, limits()).unwrap();
            assert_eq!(preview.metadata.orientation, orientation);
            assert_eq!(
                (
                    preview.metadata.encoded_width,
                    preview.metadata.encoded_height
                ),
                (2, 3)
            );
            let dimensions = if orientation >= 5 { (3, 2) } else { (2, 3) };
            assert_eq!((preview.width, preview.height), dimensions);
            let expected: Vec<u8> = orders[orientation as usize - 1]
                .iter()
                .flat_map(|index| base.rgba[index * 4..index * 4 + 4].iter().copied())
                .collect();
            assert_eq!(
                preview.rgba, expected,
                "{format:?}, orientation {orientation}"
            );
            assert_eq!(fs::read(&path).unwrap(), bytes);
        }
    }
}

#[test]
fn corrupt_unsupported_and_unavailable_are_distinct() {
    let fixture = Fixture::new();
    for format in [Format::Png, Format::Jpeg, Format::WebP] {
        let mut bytes = encode(format, None);
        bytes.truncate(bytes.len() / 2);
        let path = fixture.write("truncated", &bytes);
        assert!(
            matches!(
                decode(&path, limits()).unwrap_err().kind,
                ErrorKind::Malformed(_)
            ),
            "{format:?}"
        );
    }
    let path = fixture.write("broken.png", b"not PNG");
    assert!(matches!(
        decode(&path, limits()).unwrap_err().kind,
        ErrorKind::Malformed(_)
    ));
    let path = fixture.write("unsupported.gif", b"GIF89a");
    assert!(matches!(
        decode(&path, limits()).unwrap_err().kind,
        ErrorKind::Unsupported
    ));
    let path = fixture.0.join("missing-東京");
    let error = decode(&path, limits()).unwrap_err();
    assert_eq!(error.path, path);
    assert!(matches!(error.kind, ErrorKind::Unavailable(_)));
}

#[test]
fn input_dimension_output_peak_and_decoder_limits() {
    let fixture = Fixture::new();
    for format in [Format::Png, Format::Jpeg, Format::WebP] {
        let bytes = encode(format, None);
        let path = fixture.write("image", &bytes);
        let mut cases = vec![limits(); 6];
        cases[0].max_input_bytes = bytes.len() as u64 - 1;
        cases[1].max_width = 1;
        cases[2].max_height = 2;
        cases[3].max_output_bytes = 23;
        cases[4].max_buffer_bytes = bytes.len() as u64 + 24;
        cases[5].max_decoder_bytes = 0;
        for case in cases {
            assert!(
                matches!(
                    decode(&path, case).unwrap_err().kind,
                    ErrorKind::Oversize(_)
                ),
                "{format:?}"
            );
        }
        let mut exact = limits();
        exact.max_input_bytes = bytes.len() as u64;
        exact.max_width = 2;
        exact.max_height = 3;
        exact.max_output_bytes = 24;
        // These are exact boundaries of the currently accounted phases, NOT
        // evidence that unresolved codec allocations fit this budget.
        exact.max_buffer_bytes = if format == Format::Jpeg {
            3 * bytes.len() as u64 + 18 + 2048 * 4 * 4 + 8192 * 8 * 4
        } else if format == Format::WebP {
            (2 * bytes.len() as u64).max(bytes.len() as u64 + 48 + 24)
        } else {
            bytes.len() as u64 + 48 + 24
        };
        assert_eq!(
            decode(&path, exact)
                .unwrap_or_else(|error| panic!("{format:?} exact phase admission: {error:?}"))
                .rgba
                .len(),
            24
        );
        exact.max_buffer_bytes -= 1;
        assert!(matches!(
            decode(&path, exact).unwrap_err().kind,
            ErrorKind::Oversize(_)
        ));
    }
}

#[test]
fn jpeg_constructor_refuses_before_header_parsing() {
    let fixture = Fixture::new();
    // Signature but no valid headers. A decoder would report Malformed.
    // Oversize proves constructor admission happened first, without large fixtures.
    let bytes = [0xff, 0xd8, 0xff, 0x00];
    let path = fixture.write("constructor.jpg", &bytes);
    let mut budget = limits();
    budget.max_buffer_bytes = 15; // F + max(F, 8) + metadata payload F requires 16
    assert!(matches!(
        decode(&path, budget).unwrap_err().kind,
        ErrorKind::Oversize("component buffer")
    ));
    budget.max_buffer_bytes = 16;
    assert!(matches!(
        decode(&path, budget).unwrap_err().kind,
        ErrorKind::Malformed(_)
    ));

    let bytes = encode(Format::Jpeg, None);
    let path = fixture.write("valid.jpg", &bytes);
    budget.max_buffer_bytes = 3 * bytes.len() as u64 - 1;
    assert!(matches!(
        decode(&path, budget).unwrap_err().kind,
        ErrorKind::Oversize("component buffer")
    ));
}

#[test]
fn oversized_jpeg_dimensions_rejected_before_pixel_decode() {
    let fixture = Fixture::new();
    let mut bytes = encode(Format::Jpeg, None);
    let sof = bytes.windows(2).position(|p| p == [0xff, 0xc0]).unwrap();
    bytes[sof + 5..sof + 9].copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    let path = fixture.write("huge.jpg", &bytes);
    assert!(matches!(
        decode(&path, limits()).unwrap_err().kind,
        ErrorKind::Oversize(_)
    ));
    assert_eq!(fs::read(&path).unwrap(), bytes);
}

#[test]
fn rotated_dimensions_are_bounded() {
    let fixture = Fixture::new();
    let path = fixture.write("rotate.jpg", &encode(Format::Jpeg, Some(6)));
    let mut budget = limits();
    budget.max_width = 2;
    assert!(matches!(
        decode(&path, budget).unwrap_err().kind,
        ErrorKind::Oversize(_)
    ));
    budget.max_width = 3;
    assert_eq!(decode(&path, budget).unwrap().width, 3);
}

#[cfg(windows)]
#[test]
fn windows_path_with_unpaired_utf16_is_not_lossily_retargeted() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    let fixture = Fixture::new();
    let name = OsString::from_wide(&[b'n' as u16, 0xd800, b'.' as u16, b'p' as u16]);
    let path = fixture.write(name, &encode(Format::Png, None));
    assert_eq!(
        decode(&path, limits()).unwrap().metadata.format,
        Format::Png
    );
    fs::remove_file(&path).unwrap();
    assert_eq!(decode(&path, limits()).unwrap_err().path, path);
}
