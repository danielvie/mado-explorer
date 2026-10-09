//! Read-only PDFium page previews. Call from a background worker, never a UI update.
//! No cache, form environment, JavaScript, action, or link APIs are used.

use pdfium_render::prelude::{
    FPDF_BITMAP, FPDF_DOCUMENT, FPDF_PAGE, IFSDK_PAUSE, Pdfium, PdfiumLibraryBindings,
};
use std::ffi::c_void;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::ptr::null_mut;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

pub const MAX_INPUT_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_RGBA_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_DIMENSION: u32 = 8192;
/// Cooperative only. PDFium parsing and individual native calls cannot be interrupted.
pub const RENDER_DEADLINE: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug)]
pub struct RenderRequest {
    /// Zero-based page index. Changing documents is the caller's responsibility.
    pub page_index: u32,
    /// Physical-pixel bounding box. The page is fitted without intentional stretching.
    pub width: u32,
    pub height: u32,
    /// Caller-assigned output allowance, at most MAX_RGBA_BYTES.
    pub max_rgba_bytes: usize,
}

#[derive(Debug)]
pub struct DocumentMetadata {
    /// Native source path, including the file name. No lossy conversion.
    pub path: PathBuf,
    pub input_bytes: usize,
    pub page_count: u32,
    /// PDFium's integer version, e.g. 17 for PDF 1.7; None if unavailable.
    pub pdf_version: Option<i32>,
}

#[derive(Debug)]
pub struct PageMetadata {
    pub index: u32,
    /// PDFium page dimensions in points, including the page's intrinsic rotation.
    pub width_points: f64,
    pub height_points: f64,
}

#[derive(Debug)]
pub struct RenderedPage {
    pub document: DocumentMetadata,
    pub page: PageMetadata,
    pub width: u32,
    pub height: u32,
    /// Top-down, tightly packed, opaque RGBA8 on white; len = width * height * 4.
    pub rgba: Vec<u8>,
}

#[derive(Debug)]
pub enum Error {
    RendererUnavailable {
        library: PathBuf,
        reason: String,
    },
    RendererAlreadyLoaded {
        library: PathBuf,
    },
    BackendPoisoned,
    InvalidRequest(&'static str),
    NotRegularFile(PathBuf),
    Io {
        path: PathBuf,
        source: io::Error,
    },
    InputTooLarge,
    AllocationFailed,
    MalformedDocument,
    PasswordRequired,
    UnsupportedSecurity,
    DocumentLoadFailed {
        code: u64,
    },
    PageOutOfRange {
        requested: u32,
        page_count: u32,
    },
    InvalidPageDimensions,
    OutputTooLarge {
        required: usize,
        budget: usize,
    },
    RenderFailed {
        stage: &'static str,
        status: Option<i32>,
    },
    RenderTimedOut,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RendererUnavailable { library, reason } => {
                write!(f, "PDF renderer unavailable at {library:?}: {reason}")
            }
            Self::RendererAlreadyLoaded { library } => {
                write!(f, "PDF renderer already loaded from {library:?}")
            }
            Self::BackendPoisoned => f.write_str("PDF renderer lock poisoned; restart required"),
            Self::InvalidRequest(reason) => write!(f, "Invalid PDF render request: {reason}"),
            Self::NotRegularFile(path) => write!(f, "PDF source is not a regular file: {path:?}"),
            Self::Io { path, source } => write!(f, "Cannot read PDF {path:?}: {source}"),
            Self::InputTooLarge => write!(f, "PDF input exceeds {MAX_INPUT_BYTES} bytes"),
            Self::AllocationFailed => f.write_str("Cannot allocate PDF preview buffer"),
            Self::MalformedDocument => f.write_str("Malformed PDF document"),
            Self::PasswordRequired => {
                f.write_str("PDF requires a password; password entry is not supported")
            }
            Self::UnsupportedSecurity => f.write_str("PDF security scheme is unsupported"),
            Self::DocumentLoadFailed { code } => {
                write!(f, "PDF document loading failed, native code {code}")
            }
            Self::PageOutOfRange {
                requested,
                page_count,
            } => write!(
                f,
                "PDF page index {requested} is outside {page_count} pages"
            ),
            Self::InvalidPageDimensions => f.write_str("PDF page has invalid dimensions"),
            Self::OutputTooLarge { required, budget } => write!(
                f,
                "PDF output needs {required} bytes, allowance is {budget}"
            ),
            Self::RenderFailed { stage, status } => write!(
                f,
                "PDF rendering failed at {stage}, native status {status:?}"
            ),
            Self::RenderTimedOut => f.write_str("PDF rendering exceeded its cooperative deadline"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Io { source, .. } = self {
            Some(source)
        } else {
            None
        }
    }
}

struct Engine {
    library: PathBuf,
    bindings: Box<dyn PdfiumLibraryBindings>,
}

// ponytail: one process-lifetime engine, serialized through one lock. This also protects
// PDFium's global last-error and bounds simultaneous input/output working buffers.
// Use process isolation if hard native-memory/time limits or parallel rendering are needed.
static ENGINE: OnceLock<Mutex<Engine>> = OnceLock::new();
static INITIALIZE: Mutex<()> = Mutex::new(());

pub struct Renderer {
    engine: &'static Mutex<Engine>,
}

impl Renderer {
    /// Load a trusted, absolute local PDFium library path, with no system/CWD fallback.
    /// Supported deployment uses PDFium 7881 without V8/XFA. See README.md.
    /// This component must be the sole owner of PDFium initialization in the process.
    pub fn load(library: &Path) -> Result<Self, Error> {
        let unavailable = |reason: String| Error::RendererUnavailable {
            library: library.to_owned(),
            reason,
        };
        if !library.is_absolute() {
            return Err(unavailable(
                "an absolute library file path is required".into(),
            ));
        }
        let library = library
            .canonicalize()
            .map_err(|e| unavailable(e.to_string()))?;
        let _initialize = INITIALIZE.lock().map_err(|_| Error::BackendPoisoned)?;
        if let Some(engine) = ENGINE.get() {
            let loaded = engine.lock().map_err(|_| Error::BackendPoisoned)?;
            if loaded.library != library {
                return Err(Error::RendererAlreadyLoaded {
                    library: loaded.library.clone(),
                });
            }
            return Ok(Self { engine });
        }
        let bindings =
            Pdfium::bind_to_library(&library).map_err(|e| Error::RendererUnavailable {
                library: library.clone(),
                reason: e.to_string(),
            })?;
        // SAFETY: initialization is serialized, only done once, and the bindings/DLL live
        // for the process lifetime. Never instantiate Pdfium's high-level document/form API.
        unsafe {
            bindings.FPDF_InitLibrary();
        }
        let engine = ENGINE.get_or_init(|| Mutex::new(Engine { library, bindings }));
        Ok(Self { engine })
    }

    /// Read and render one page. Each request reopens its source; no native handles escape.
    /// Successful output is independent of the source and renderer. No writes are made.
    pub fn render_file(&self, path: &Path, request: RenderRequest) -> Result<RenderedPage, Error> {
        validate_request(request)?;
        let engine = self.engine.lock().map_err(|_| Error::BackendPoisoned)?;
        let io_error = |source| Error::Io {
            path: path.to_owned(),
            source,
        };
        let file = File::open(path).map_err(io_error)?;
        let metadata = file.metadata().map_err(io_error)?;
        if !metadata.is_file() {
            return Err(Error::NotRegularFile(path.to_owned()));
        }
        if metadata.len() > MAX_INPUT_BYTES as u64 {
            return Err(Error::InputTooLarge);
        }
        let bytes = read_bounded(file, MAX_INPUT_BYTES).map_err(|e| match e {
            ReadError::Io(e) => io_error(e),
            ReadError::TooLarge => Error::InputTooLarge,
            ReadError::Allocation => Error::AllocationFailed,
        })?;
        render(&*engine.bindings, path, &bytes, request)
    }
}

fn validate_request(request: RenderRequest) -> Result<(), Error> {
    if request.width == 0
        || request.height == 0
        || request.width > MAX_DIMENSION
        || request.height > MAX_DIMENSION
    {
        return Err(Error::InvalidRequest(
            "dimensions must be 1..=8192 physical pixels",
        ));
    }
    if request.max_rgba_bytes < 4 || request.max_rgba_bytes > MAX_RGBA_BYTES {
        return Err(Error::InvalidRequest(
            "output allowance must be 4..=33554432 bytes",
        ));
    }
    if request.page_index > i32::MAX as u32 {
        return Err(Error::InvalidRequest(
            "page index exceeds the native integer range",
        ));
    }
    Ok(())
}

fn validate_page(index: u32, count: i32) -> Result<u32, Error> {
    if count <= 0 {
        return Err(Error::MalformedDocument);
    }
    let page_count = count as u32;
    if index >= page_count {
        return Err(Error::PageOutOfRange {
            requested: index,
            page_count,
        });
    }
    Ok(page_count)
}

fn fitted_size(
    points_width: f64,
    points_height: f64,
    request: RenderRequest,
) -> Result<(u32, u32, usize), Error> {
    validate_request(request)?;
    if !points_width.is_finite()
        || !points_height.is_finite()
        || points_width <= 0.0
        || points_height <= 0.0
    {
        return Err(Error::InvalidPageDimensions);
    }
    let scale = (request.width as f64 / points_width).min(request.height as f64 / points_height);
    let width = (points_width * scale)
        .round()
        .max(1.0)
        .min(request.width as f64) as u32;
    let height = (points_height * scale)
        .round()
        .max(1.0)
        .min(request.height as f64) as u32;
    let required = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(4))
        .ok_or(Error::InvalidPageDimensions)?;
    if required > request.max_rgba_bytes {
        return Err(Error::OutputTooLarge {
            required,
            budget: request.max_rgba_bytes,
        });
    }
    Ok((width, height, required))
}

enum ReadError {
    Io(io::Error),
    TooLarge,
    Allocation,
}

fn read_bounded(mut reader: impl Read, limit: usize) -> Result<Vec<u8>, ReadError> {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 16 * 1024];
    loop {
        // Read one excess byte without appending it, even if the file grew after metadata.
        let allowance = (limit - bytes.len()).saturating_add(1).min(chunk.len());
        let count = match reader.read(&mut chunk[..allowance]) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            other => other.map_err(ReadError::Io)?,
        };
        if count == 0 {
            return Ok(bytes);
        }
        if count > limit - bytes.len() {
            return Err(ReadError::TooLarge);
        }
        if bytes.len() + count > bytes.capacity() {
            let capacity = bytes
                .capacity()
                .saturating_mul(2)
                .max(bytes.len() + count)
                .min(limit);
            bytes
                .try_reserve_exact(capacity - bytes.len())
                .map_err(|_| ReadError::Allocation)?;
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
}

fn load_error(code: u64) -> Error {
    // Values from PDFium's public/fpdfview.h.
    match code {
        3 => Error::MalformedDocument,
        4 => Error::PasswordRequired,
        5 => Error::UnsupportedSecurity,
        _ => Error::DocumentLoadFailed { code },
    }
}

struct DocumentPage<'a> {
    bindings: &'a dyn PdfiumLibraryBindings,
    document: FPDF_DOCUMENT,
    page: FPDF_PAGE,
}

impl Drop for DocumentPage<'_> {
    fn drop(&mut self) {
        // SAFETY: this guard owns the loaded handles; bitmap/render resources close first.
        unsafe {
            if !self.page.is_null() {
                self.bindings.FPDF_ClosePage(self.page);
            }
            self.bindings.FPDF_CloseDocument(self.document);
        }
    }
}

struct BitmapRender<'a> {
    bindings: &'a dyn PdfiumLibraryBindings,
    bitmap: FPDF_BITMAP,
    page: FPDF_PAGE,
    // This borrow keeps the external buffer alive and inaccessible during native rendering.
    _buffer: &'a mut [u8],
}

impl Drop for BitmapRender<'_> {
    fn drop(&mut self) {
        // SAFETY: guard is created immediately before Start, and drops before page/buffer.
        unsafe {
            self.bindings.FPDF_RenderPage_Close(self.page);
            self.bindings.FPDFBitmap_Destroy(self.bitmap);
        }
    }
}

unsafe extern "C" fn deadline_pause(pause: *mut IFSDK_PAUSE) -> i32 {
    // SAFETY: PDFium invokes this synchronously; user points to the stack Instant below.
    let started = unsafe { &*((*pause).user.cast::<Instant>()) };
    i32::from(started.elapsed() >= RENDER_DEADLINE)
}

fn render(
    bindings: &dyn PdfiumLibraryBindings,
    path: &Path,
    bytes: &[u8],
    request: RenderRequest,
) -> Result<RenderedPage, Error> {
    // SAFETY: all native calls run under the engine lock. Source bytes outlive the
    // document; valid handles and bounded external bitmap storage are guarded below.
    let document = unsafe { bindings.FPDF_LoadMemDocument64(bytes, None) };
    if document.is_null() {
        return Err(load_error(unsafe { bindings.FPDF_GetLastError() } as u64));
    }
    let mut handles = DocumentPage {
        bindings,
        document,
        page: null_mut(),
    };
    let page_count = validate_page(request.page_index, unsafe {
        bindings.FPDF_GetPageCount(document)
    })?;
    let mut version = 0;
    let pdf_version = if unsafe { bindings.FPDF_GetFileVersion(document, &mut version) } != 0 {
        Some(version)
    } else {
        None
    };
    handles.page = unsafe { bindings.FPDF_LoadPage(document, request.page_index as i32) };
    if handles.page.is_null() {
        return Err(Error::RenderFailed {
            stage: "page loading",
            status: None,
        });
    }
    let width_points = unsafe { bindings.FPDF_GetPageWidthF(handles.page) } as f64;
    let height_points = unsafe { bindings.FPDF_GetPageHeightF(handles.page) } as f64;
    let (width, height, required) = fitted_size(width_points, height_points, request)?;
    let mut rgba = Vec::new();
    rgba.try_reserve_exact(required)
        .map_err(|_| Error::AllocationFailed)?;
    rgba.resize(required, 255); // Opaque white background in BGRA and RGBA.
    let bitmap = unsafe {
        // FPDFBitmap_BGRA = 4. Explicit tight stride, no additional pixel copy/allocation.
        bindings.FPDFBitmap_CreateEx(
            width as i32,
            height as i32,
            4,
            rgba.as_mut_ptr().cast::<c_void>(),
            (width * 4) as i32,
        )
    };
    if bitmap.is_null() {
        return Err(Error::RenderFailed {
            stage: "bitmap allocation",
            status: None,
        });
    }
    {
        let guard = BitmapRender {
            bindings,
            bitmap,
            page: handles.page,
            _buffer: &mut rgba,
        };
        let mut started = Instant::now();
        let mut pause = IFSDK_PAUSE {
            version: 1,
            NeedToPauseNow: Some(deadline_pause),
            user: (&mut started as *mut Instant).cast(),
        };
        // flags=0: page content only. No annotation widgets, form environment, or actions.
        let status = unsafe {
            bindings.FPDF_RenderPageBitmap_Start(
                guard.bitmap,
                guard.page,
                0,
                0,
                width as i32,
                height as i32,
                0,
                0,
                &mut pause,
            )
        };
        if started.elapsed() >= RENDER_DEADLINE {
            return Err(Error::RenderTimedOut);
        }
        // FPDF_RENDER_DONE = 2. With this callback any suspension is a deadline stop.
        if status != 2 {
            return Err(Error::RenderFailed {
                stage: "page rasterization",
                status: Some(status),
            });
        }
    }
    for pixel in rgba.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    Ok(RenderedPage {
        document: DocumentMetadata {
            path: path.to_owned(),
            input_bytes: bytes.len(),
            page_count,
            pdf_version,
        },
        page: PageMetadata {
            index: request.page_index,
            width_points,
            height_points,
        },
        width,
        height,
        rgba,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn request() -> RenderRequest {
        RenderRequest {
            page_index: 0,
            width: 400,
            height: 400,
            max_rgba_bytes: MAX_RGBA_BYTES,
        }
    }

    #[test]
    fn validates_request_before_native_work() {
        for (width, height) in [(0, 1), (1, 0), (8193, 1), (1, 8193), (u32::MAX, u32::MAX)] {
            assert!(matches!(
                validate_request(RenderRequest {
                    width,
                    height,
                    ..request()
                }),
                Err(Error::InvalidRequest(_))
            ));
        }
        for max_rgba_bytes in [0, 3, MAX_RGBA_BYTES + 1, usize::MAX] {
            assert!(matches!(
                validate_request(RenderRequest {
                    max_rgba_bytes,
                    ..request()
                }),
                Err(Error::InvalidRequest(_))
            ));
        }
        assert!(matches!(
            validate_request(RenderRequest {
                page_index: u32::MAX,
                ..request()
            }),
            Err(Error::InvalidRequest(_))
        ));
        assert!(
            validate_request(RenderRequest {
                width: MAX_DIMENSION,
                height: MAX_DIMENSION,
                ..request()
            })
            .is_ok()
        );
    }

    #[test]
    fn page_indices_are_zero_based_and_checked() {
        assert_eq!(validate_page(0, 1).unwrap(), 1);
        assert_eq!(validate_page(9, 10).unwrap(), 10);
        for index in [1, u32::MAX] {
            assert!(matches!(
                validate_page(index, 1),
                Err(Error::PageOutOfRange { .. })
            ));
        }
        for count in [0, -1, i32::MIN] {
            assert!(matches!(
                validate_page(0, count),
                Err(Error::MalformedDocument)
            ));
        }
    }

    #[test]
    fn fits_physical_pixels_and_checks_budget_before_allocation() {
        assert_eq!(
            fitted_size(200.0, 100.0, request()).unwrap(),
            (400, 200, 320_000)
        );
        assert_eq!(
            fitted_size(100.0, 200.0, request()).unwrap(),
            (200, 400, 320_000)
        );
        assert_eq!(
            fitted_size(
                200.0,
                100.0,
                RenderRequest {
                    width: 600,
                    height: 600,
                    ..request()
                }
            )
            .unwrap(),
            (600, 300, 720_000)
        );
        assert!(matches!(
            fitted_size(
                1.0,
                1.0,
                RenderRequest {
                    width: 8192,
                    height: 8192,
                    ..request()
                }
            ),
            Err(Error::OutputTooLarge { .. })
        ));
        assert!(matches!(
            fitted_size(
                200.0,
                100.0,
                RenderRequest {
                    max_rgba_bytes: 319_999,
                    ..request()
                }
            ),
            Err(Error::OutputTooLarge {
                required: 320_000,
                ..
            })
        ));
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                fitted_size(value, 100.0, request()),
                Err(Error::InvalidPageDimensions)
            ));
            assert!(matches!(
                fitted_size(100.0, value, request()),
                Err(Error::InvalidPageDimensions)
            ));
        }
    }

    #[test]
    fn bounded_reader_rejects_excess_without_trusting_metadata() {
        assert_eq!(read_bounded(Cursor::new(b"abcd"), 4).ok().unwrap(), b"abcd");
        assert!(matches!(
            read_bounded(Cursor::new(b"abcde"), 4),
            Err(ReadError::TooLarge)
        ));
        assert!(read_bounded(Cursor::new(b""), 0).ok().unwrap().is_empty());
        assert!(matches!(
            read_bounded(Cursor::new(vec![0; 32_769]), 32_768),
            Err(ReadError::TooLarge)
        ));
        let bytes = read_bounded(Cursor::new(vec![0; 32_768]), 32_768)
            .ok()
            .unwrap();
        assert_eq!(bytes.len(), 32_768);
        assert!(bytes.capacity() <= 32_768);
        let bytes = read_bounded(Cursor::new(vec![0; 20_000]), 20_000)
            .ok()
            .unwrap();
        assert_eq!(bytes.len(), 20_000);
        assert!(bytes.capacity() <= 20_000);
    }

    #[test]
    fn native_load_errors_remain_distinct() {
        assert!(matches!(load_error(3), Error::MalformedDocument));
        assert!(matches!(load_error(4), Error::PasswordRequired));
        assert!(matches!(load_error(5), Error::UnsupportedSecurity));
        assert!(matches!(
            load_error(1),
            Error::DocumentLoadFailed { code: 1 }
        ));
    }

    #[test]
    fn deadline_callback_stops_expired_rendering() {
        let mut started = Instant::now() - RENDER_DEADLINE;
        let mut pause = IFSDK_PAUSE {
            version: 1,
            NeedToPauseNow: Some(deadline_pause),
            user: (&mut started as *mut Instant).cast(),
        };
        assert_eq!(unsafe { deadline_pause(&mut pause) }, 1);
        started = Instant::now();
        assert_eq!(unsafe { deadline_pause(&mut pause) }, 0);
        std::hint::black_box(started);
    }
}
