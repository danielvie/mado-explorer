//! Adapted from our preserved P01b beae376 decoder. Helper binary ONLY.
//! Original admission/format/orientation guards remain. Best-effort codec limits
//! do not bound working allocations; the parent must supply the hard Job cap.

#[path = "admission.rs"]
mod admission;
#[path = "webp_preflight.rs"]
mod webp_preflight;

use std::fmt;
use std::fs::File;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};

use image::{DynamicImage, ImageDecoder, ImageError, ImageFormat, ImageReader};

/// All limits are supplied by the caller. There is no implicit window/cache budget.
#[derive(Clone, Copy, Debug)]
pub struct DecodeLimits {
    pub max_input_bytes: u64,
    /// Applies to both encoded and orientation-corrected dimensions.
    pub max_width: u32,
    pub max_height: u32,
    pub max_output_bytes: u64,
    /// Admission for input capacity, native pixels, conversion/orientation overlap,
    /// and audited JPEG input/payload/coefficient/row buffers. NOT a complete CPU bound:
    /// PNG/WebP working allocations and JPEG metadata containers remain unresolved.
    /// See HANDOFF.md. Window-wide reservations/cache ownership belong to the caller.
    pub max_buffer_bytes: u64,
    /// Passed to image::Limits::max_alloc. Not a hard working-memory bound.
    pub max_decoder_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Png,
    Jpeg,
    WebP,
}

#[derive(Debug)]
pub struct Metadata {
    pub format: Format,
    pub encoded_width: u32,
    pub encoded_height: u32,
    pub file_bytes: u64,
    /// Applied Exif orientation, 1..=8. Missing/invalid metadata becomes 1 per image.
    pub orientation: u8,
}

#[derive(Debug)]
pub struct Preview {
    /// Orientation-corrected dimensions; tightly packed straight-alpha RGBA8.
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub enum ErrorKind {
    Unsupported,
    Malformed(String),
    Unavailable(io::Error),
    Oversize(&'static str),
    AllocationFailed,
}

#[derive(Debug)]
pub struct PreviewError {
    pub path: PathBuf,
    pub kind: ErrorKind,
}

impl fmt::Display for PreviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Preview {}: ", self.path.display())?;
        match &self.kind {
            ErrorKind::Unsupported => write!(f, "unsupported image format or feature"),
            ErrorKind::Malformed(message) => write!(f, "malformed image: {message}"),
            ErrorKind::Unavailable(error) => write!(f, "file unavailable: {error}"),
            ErrorKind::Oversize(limit) => write!(f, "image exceeds {limit} limit"),
            ErrorKind::AllocationFailed => write!(f, "unable to allocate preview buffer"),
        }
    }
}

impl std::error::Error for PreviewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Unavailable(error) => Some(error),
            _ => None,
        }
    }
}

/// Decode a regular file by native path. Content signatures take precedence over extensions.
/// Animated PNG/WebP returns the codec's default/first still image, not an animation.
/// Never opens the source for writing. No cache, resize, threads, GPU, or retained file handle.
/// A caller must discard stale results and reserve/release its window-wide budgets separately.
pub(super) fn decode(
    file: File,
    path: &Path,
    limits: DecodeLimits,
) -> Result<Preview, PreviewError> {
    decode_inner(file, path, limits).map_err(|kind| PreviewError {
        path: path.to_path_buf(),
        kind,
    })
}

fn decode_inner(mut file: File, path: &Path, limits: DecodeLimits) -> Result<Preview, ErrorKind> {
    let metadata = file.metadata().map_err(ErrorKind::Unavailable)?;
    if !metadata.is_file() {
        return Err(ErrorKind::Unsupported);
    }
    let file_bytes = metadata.len();
    check_bytes(file_bytes, limits.max_input_bytes, "file input")?;
    check_bytes(file_bytes, limits.max_buffer_bytes, "component buffer")?;
    // Allocate exactly the admitted file length. A concurrent growth cannot make
    // read_to_end grow an unchecked Vec; a shrink/read error is reported as unavailable.
    let mut input = Vec::new();
    input
        .try_reserve_exact(addressable(file_bytes)?)
        .map_err(|_| ErrorKind::AllocationFailed)?;
    input.resize(addressable(file_bytes)?, 0);
    file.read_exact(&mut input)
        .map_err(ErrorKind::Unavailable)?;
    if file.read(&mut [0]).map_err(ErrorKind::Unavailable)? != 0 {
        return Err(ErrorKind::Oversize("file snapshot length"));
    }
    drop(file);

    let image_format = image::guess_format(&input)
        .ok()
        .or_else(|| ImageFormat::from_path(path).ok())
        .ok_or(ErrorKind::Unsupported)?;
    let format = match image_format {
        ImageFormat::Png => Format::Png,
        ImageFormat::Jpeg => Format::Jpeg,
        ImageFormat::WebP => Format::WebP,
        _ => return Err(ErrorKind::Unsupported),
    };
    let input_capacity = input.capacity() as u64;
    // Admission must precede into_decoder: image's JPEG constructor copies F bytes
    // before dimensions are available, and WebP stores untrusted metadata ranges.
    if format == Format::Jpeg {
        admission::admit(
            admission::jpeg_constructor(input_capacity, file_bytes)?,
            limits.max_buffer_bytes,
        )?;
    }
    if format == Format::WebP {
        webp_preflight::validate(&input, limits)?;
        let orientation_peak = input_capacity
            .checked_add(file_bytes)
            .ok_or(ErrorKind::Oversize("component buffer"))?;
        admission::admit(orientation_peak, limits.max_buffer_bytes)?;
    }
    let mut codec_limits = image::Limits::default();
    codec_limits.max_image_width = Some(limits.max_width);
    codec_limits.max_image_height = Some(limits.max_height);
    codec_limits.max_alloc = Some(limits.max_decoder_bytes);
    let mut reader = ImageReader::with_format(Cursor::new(&input), image_format);
    // PNG needs limits at construction, not just after headers have been parsed.
    reader.limits(codec_limits);
    let mut decoder = reader.into_decoder().map_err(codec_error)?;
    let (encoded_width, encoded_height) = decoder.dimensions();
    check_dimensions(encoded_width, encoded_height, limits)?;
    let output_bytes = u64::from(encoded_width)
        .checked_mul(u64::from(encoded_height))
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or(ErrorKind::Oversize("RGBA output"))?;
    check_bytes(output_bytes, limits.max_output_bytes, "RGBA output")?;
    addressable(output_bytes)?;
    let native_bytes = decoder.total_bytes();
    addressable(native_bytes)?;
    let peak = if format == Format::Jpeg {
        admission::jpeg_peak(
            input_capacity,
            file_bytes,
            encoded_width,
            encoded_height,
            native_bytes,
            output_bytes,
        )?
    } else {
        output_bytes
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(native_bytes))
            .and_then(|bytes| bytes.checked_add(input_capacity))
            .ok_or(ErrorKind::Oversize("component buffer"))?
    };
    admission::admit(peak, limits.max_buffer_bytes)?;
    // Reject impossible native output even when a codec ignores max_alloc.
    check_bytes(native_bytes, limits.max_decoder_bytes, "decoder allocation")?;
    let orientation = decoder.orientation().map_err(codec_error)?;
    let swaps_axes = matches!(orientation.to_exif(), 5..=8);
    let (width, height) = if swaps_axes {
        (encoded_height, encoded_width)
    } else {
        (encoded_width, encoded_height)
    };
    check_dimensions(width, height, limits)?;

    let decoded = DynamicImage::from_decoder(decoder).map_err(codec_error)?;
    let mut rgba = DynamicImage::ImageRgba8(decoded.into_rgba8());
    // Conversion and 90/270-degree rotation may each need an extra pixel buffer.
    // Pixel overlap was admitted above. Remaining codec allocations are documented gaps.
    rgba.apply_orientation(orientation);
    let rgba = rgba.into_rgba8().into_raw();
    Ok(Preview {
        width,
        height,
        rgba,
        metadata: Metadata {
            format,
            encoded_width,
            encoded_height,
            file_bytes,
            orientation: orientation.to_exif(),
        },
    })
}

fn check_dimensions(width: u32, height: u32, limits: DecodeLimits) -> Result<(), ErrorKind> {
    if width == 0 || height == 0 {
        return Err(ErrorKind::Malformed("zero image dimension".into()));
    }
    if width > limits.max_width || height > limits.max_height {
        return Err(ErrorKind::Oversize("dimensions"));
    }
    Ok(())
}

fn check_bytes(bytes: u64, maximum: u64, name: &'static str) -> Result<(), ErrorKind> {
    if bytes > maximum {
        Err(ErrorKind::Oversize(name))
    } else {
        Ok(())
    }
}

fn addressable(bytes: u64) -> Result<usize, ErrorKind> {
    usize::try_from(bytes)
        .ok()
        .filter(|bytes| *bytes <= isize::MAX as usize)
        .ok_or(ErrorKind::Oversize("addressable allocation"))
}

fn codec_error(error: ImageError) -> ErrorKind {
    match error {
        ImageError::Limits(_) => ErrorKind::Oversize("decoder allocation or dimensions"),
        ImageError::Unsupported(_) => ErrorKind::Unsupported,
        error => ErrorKind::Malformed(error.to_string()),
    }
}
