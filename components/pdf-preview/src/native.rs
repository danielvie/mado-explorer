//! Minimal Windows x64 adapter for the two upstream dynamic-binding defects.
use pdfium_render::prelude::FPDF_DOCUMENT;
use std::ffi::{c_char, c_void};
use std::path::Path;

// PDFium 7881 public/fpdfview.h: size_t, not Windows' 32-bit unsigned long.
// FPDF_CALLCONV may be __stdcall; on Windows x64 C and system use the same ABI.
pub(crate) type LoadMemDocument64 =
    unsafe extern "system" fn(*const c_void, usize, *const c_char) -> FPDF_DOCUMENT;

#[cfg(all(windows, target_arch = "x86_64"))]
const LOAD_FLAGS: u32 = libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR
    | libloading::os::windows::LOAD_LIBRARY_SEARCH_SYSTEM32;

pub(crate) fn secure_load(library: &Path) -> Result<libloading::Library, String> {
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        // SAFETY: caller supplies a canonical absolute path to an installed trusted DLL.
        // Its directory and dependencies are trusted deployment inputs, never browsed files.
        // No fallback or process-wide loader-policy mutation. Library retains the module.
        unsafe { libloading::os::windows::Library::load_with_flags(library, LOAD_FLAGS) }
            .map(Into::into)
            .map_err(|e| e.to_string())
    }
    #[cfg(not(all(windows, target_arch = "x86_64")))]
    {
        let _ = library;
        Err("PDF renderer adapter supports Windows x64 only".into())
    }
}

#[cfg(all(test, windows, target_arch = "x86_64"))]
#[allow(dead_code)]
#[path = "../tests/support/mod.rs"]
mod support;

#[cfg(all(test, windows, target_arch = "x86_64"))]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;

    #[test]
    fn corrected_signature_and_route() {
        unsafe extern "system" fn echo_size(
            buffer: *const c_void,
            size: usize,
            password: *const c_char,
        ) -> FPDF_DOCUMENT {
            assert!(buffer.is_null());
            assert!(password.is_null());
            // No buffer access or native PDFium call. Exercise all 64 argument bits.
            size as FPDF_DOCUMENT
        }
        let load: LoadMemDocument64 = echo_size;
        let size = u32::MAX as usize + 17;
        assert_eq!(
            unsafe { load(std::ptr::null(), size, std::ptr::null()) } as usize,
            size
        );
        assert_eq!(std::mem::size_of::<usize>(), 8);
        assert_eq!(std::mem::size_of::<std::ffi::c_ulong>(), 4);
        let production = include_str!("lib.rs").split("#[cfg(test)]").next().unwrap();
        assert!(
            !production.contains(".FPDF_LoadMemDocument"),
            "never route via either upstream wrapper"
        );
        assert!(production.contains(
            "(engine.load_mem_document64)(bytes.as_ptr().cast(), bytes.len(), std::ptr::null())"
        ));
        assert_eq!(LOAD_FLAGS, 0x0000_0900);
    }

    #[test]
    fn dependencies_do_not_fall_back_to_cwd_or_path() {
        let fixtures = support::FixtureDirectory::new();
        let install = fixtures.path().join("install");
        let search = fixtures.path().join("cwd-and-path");
        fs::create_dir(&install).unwrap();
        fs::create_dir(&search).unwrap();
        // Build only these harmless generated DLLs, never arbitrary native inputs.
        let dep = fixtures.write(
            "dep.rs",
            b"#[unsafe(no_mangle)] pub extern \"C\" fn mado_dep() -> i32 { 7 }",
        );
        let top = fixtures.write("top.rs", b"#[link(name=\"mado_p03_loader_dep.dll.lib\", kind=\"dylib\", modifiers=\"+verbatim\")] unsafe extern \"C\" { fn mado_dep() -> i32; } #[unsafe(no_mangle)] pub unsafe extern \"C\" fn mado_top() -> i32 { unsafe { mado_dep() } }");
        for (source, name, out) in [
            (&dep, "mado_p03_loader_dep", &search),
            (&top, "mado_p03_loader_top", &install),
        ] {
            let output = Command::new("rustc")
                .args([
                    "--crate-type",
                    "cdylib",
                    "--crate-name",
                    name,
                    "--edition",
                    "2024",
                ])
                .arg(source)
                .arg("--out-dir")
                .arg(out)
                .arg("-L")
                .arg(&search)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "fixture compiler: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let library = install
            .join("mado_p03_loader_top.dll")
            .canonicalize()
            .unwrap();
        for expected in ["missing", "installed"] {
            if expected == "installed" {
                fs::copy(
                    search.join("mado_p03_loader_dep.dll"),
                    install.join("mado_p03_loader_dep.dll"),
                )
                .unwrap();
            }
            // Only the child changes CWD/PATH. No global state or parent loader contamination.
            let output = Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "native::tests::search_path_child",
                    "--ignored",
                    "--nocapture",
                ])
                .current_dir(&search)
                .env("PATH", &search)
                .env("MADO_P03_LOADER_FIXTURE", &library)
                .env("MADO_P03_LOADER_EXPECTED", expected)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "loader child: {} {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    #[test]
    #[ignore = "isolated child invoked by dependencies_do_not_fall_back_to_cwd_or_path"]
    fn search_path_child() {
        let path =
            std::env::var_os("MADO_P03_LOADER_FIXTURE").expect("parent supplies generated fixture");
        let result = secure_load(Path::new(&path));
        if std::env::var("MADO_P03_LOADER_EXPECTED").unwrap() == "missing" {
            assert!(result.is_err(), "dependency in CWD/PATH must not load");
        } else {
            let module = result.unwrap();
            // SAFETY: generated top DLL above exports exactly this harmless signature.
            let value = unsafe {
                module
                    .get::<unsafe extern "C" fn() -> i32>(b"mado_top\0")
                    .unwrap()()
            };
            assert_eq!(value, 7, "dependency in installed DLL directory loads");
        }
    }
}
