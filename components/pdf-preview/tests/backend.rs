mod support;

use mado_pdf_preview::{
    Error, MAX_DIMENSION, MAX_INPUT_BYTES, MAX_RGBA_BYTES, RenderRequest, Renderer,
};
use std::fs;
use std::path::Path;
use support::{FixtureDirectory, encrypted_pdf, simple_pdf};

fn request() -> RenderRequest {
    RenderRequest {
        page_index: 0,
        width: 400,
        height: 400,
        max_rgba_bytes: MAX_RGBA_BYTES,
    }
}

#[test]
fn missing_or_invalid_local_library_is_explicit() {
    let fixtures = FixtureDirectory::new();
    assert!(matches!(
        Renderer::load(&fixtures.path().join("absent-pdfium.dll")),
        Err(Error::RendererUnavailable { .. })
    ));
    let invalid = fixtures.write("invalid-pdfium.dll", b"not a native library");
    // With --include-ignored the native probe may have initialized the process engine.
    // Either reject this bad binary at load or refuse switching an already loaded engine.
    assert!(matches!(
        Renderer::load(&invalid),
        Err(Error::RendererUnavailable { .. } | Error::RendererAlreadyLoaded { .. })
    ));
    assert!(matches!(
        Renderer::load(Path::new("pdfium.dll")),
        Err(Error::RendererUnavailable { .. })
    ));
}

#[test]
#[cfg(all(windows, target_arch = "x86_64"))]
#[ignore = "requires trusted PDFium 7881 without V8/XFA; set absolute PDFIUM_LIBRARY_PATH and run task run"]
fn native_fixture_probe() {
    let library = std::env::var_os("PDFIUM_LIBRARY_PATH").expect(
        "native probe pending: set PDFIUM_LIBRARY_PATH to an absolute trusted library file",
    );
    let renderer = Renderer::load(Path::new(&library)).unwrap();
    // Reproduce the crate's flags-zero second load of the *same canonical held DLL*.
    // Windows must return the same HMODULE, not remap it or resolve dependencies anew.
    let canonical = Path::new(&library).canonicalize().unwrap();
    unsafe {
        use libloading::os::windows::{
            LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LOAD_LIBRARY_SEARCH_SYSTEM32, Library,
        };
        let secure = Library::load_with_flags(
            &canonical,
            LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32,
        )
        .unwrap();
        let repeated = Library::new(&canonical).unwrap();
        let secure_handle = secure.into_raw();
        let repeated_handle = repeated.into_raw();
        // Restore ownership before asserting, so failure also releases both test references.
        let _secure = Library::from_raw(secure_handle);
        let _repeated = Library::from_raw(repeated_handle);
        assert_eq!(
            secure_handle, repeated_handle,
            "same held canonical DLL must be reused"
        );
    }
    // The same library can be requested again without reinitializing PDFium.
    let second_renderer = Renderer::load(Path::new(&library)).unwrap();
    let fixtures = FixtureDirectory::new();
    let bytes = simple_pdf();
    let source = fixtures.write("page-é-𐐀.pdf", &bytes);
    let result = renderer.render_file(&source, request()).unwrap();
    assert_eq!(
        (result.width, result.height, result.rgba.len()),
        (400, 200, 320_000)
    );
    assert_eq!(result.document.path, source);
    assert_eq!(result.document.input_bytes, bytes.len());
    assert_eq!(result.document.page_count, 1);
    assert_eq!(result.document.pdf_version, Some(13));
    assert_eq!(result.page.index, 0);
    assert_eq!(
        (result.page.width_points, result.page.height_points),
        (200.0, 100.0)
    );
    assert_eq!(&result.rgba[..4], &[255, 255, 255, 255]);
    let center = ((result.height / 2 * result.width + result.width / 2) * 4) as usize;
    assert_eq!(&result.rgba[center..center + 4], &[255, 0, 0, 255]);
    let sharper = second_renderer
        .render_file(
            &source,
            RenderRequest {
                width: 600,
                height: 600,
                ..request()
            },
        )
        .unwrap();
    assert_eq!((sharper.width, sharper.height), (600, 300));
    assert_eq!(
        fs::read(&source).unwrap(),
        bytes,
        "preview must not change source bytes"
    );
    assert!(matches!(
        renderer.render_file(
            &source,
            RenderRequest {
                page_index: 1,
                ..request()
            }
        ),
        Err(Error::PageOutOfRange { page_count: 1, .. })
    ));
    assert!(matches!(
        renderer.render_file(
            &source,
            RenderRequest {
                max_rgba_bytes: 4,
                ..request()
            }
        ),
        Err(Error::OutputTooLarge { .. })
    ));
    assert!(matches!(
        renderer.render_file(
            &source,
            RenderRequest {
                width: MAX_DIMENSION + 1,
                ..request()
            }
        ),
        Err(Error::InvalidRequest(_))
    ));
    let malformed = fixtures.write("malformed.pdf", b"%PDF-1.7\nnot a document\n");
    assert!(matches!(
        renderer.render_file(&malformed, request()),
        Err(Error::MalformedDocument)
    ));
    let encrypted = fixtures.write("encrypted.pdf", &encrypted_pdf());
    assert!(matches!(
        renderer.render_file(&encrypted, request()),
        Err(Error::PasswordRequired)
    ));
    assert!(matches!(
        renderer.render_file(&fixtures.path().join("missing.pdf"), request()),
        Err(Error::Io { .. })
    ));
    let oversized = fixtures.write("oversized.pdf", b"");
    fs::OpenOptions::new()
        .write(true)
        .open(&oversized)
        .unwrap()
        .set_len(MAX_INPUT_BYTES as u64 + 1)
        .unwrap();
    assert!(matches!(
        renderer.render_file(&oversized, request()),
        Err(Error::InputTooLarge)
    ));
    let other = fixtures.write("other-pdfium.dll", b"not a native library");
    assert!(
        matches!(Renderer::load(&other), Err(Error::RendererAlreadyLoaded { library: loaded }) if loaded == canonical)
    );
    let minimum = renderer
        .render_file(
            &source,
            RenderRequest {
                width: 1,
                height: 1,
                max_rgba_bytes: 4,
                ..request()
            },
        )
        .unwrap();
    assert_eq!(
        (minimum.width, minimum.height, minimum.rgba.len()),
        (1, 1, 4)
    );
    println!(
        "Native PDFium probe passed: held canonical HMODULE reuse, RGBA channel order, metadata, resize, page bounds, corrupt/password input, exact limits, library-switch refusal and unchanged sources."
    );
}
