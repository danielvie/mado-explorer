# PDF preview backend

P03b is a standalone read-only Rust library, not an enabled explorer feature. Promotion remains gated on fresh safety review. It prepares backend portions of F08.1-F08.6. No UI acceptance, stale-result routing, window cache budget, GPU behavior, or packaging check is claimed.

## API

`Renderer::load(&absolute_library_path)` loads one trusted installed Windows x64 PDFium library. Its first load uses `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32`. Dependency directory search permits only the chosen DLL directory and Windows System32, never CWD/PATH. Failure has no fallback. No download or process-wide loader policy change occurs. Other platforms/architectures return renderer unavailable. Missing files, bad binaries, wrong architecture, and missing exports return `Error::RendererUnavailable` with the loader reason. The same canonical library path reuses the engine; another library needs a process restart.

`renderer.render_file(&path, RenderRequest { page_index, width, height, max_rgba_bytes })` reads a regular file and returns `RenderedPage`:

- Native source path/file name, actual input byte count, positive page count, and optional PDF version.
- Zero-based current page index and intrinsic page dimensions in PDF points.
- Physical-pixel width/height and top-down, tightly packed RGBA8 bytes on a white background.

The page fits inside the requested physical-pixel box. The two dimensions share one scale, subject to integer-pixel rounding and a one-pixel minimum. The caller supplies display scaling and zoom through that box. An output exceeding the assigned allowance is rejected, not silently stretched or allocated.

Errors distinguish malformed documents, password-required documents, unsupported security, page bounds, source I/O, input/output limits, allocation failure, invalid dimensions, native page/bitmap/rasterization failures, and a cooperative render deadline. PDFium may repair some malformed PDFs. Other load failures preserve the native error code instead of guessing their cause. Password entry is not supported.

## Limits and integration

- Input cap: 32 MiB. Metadata is only a preflight; a bounded chunk reader detects growth and never appends an excess byte.
- Output cap: 32 MiB per result; caller allowance can be smaller. Dimensions are 1..=8192 pixels per axis, checked before allocation. Final `width * height * 4` uses checked arithmetic.
- One process-wide lock covers file reading, document loading, rendering, and handle cleanup. At most one input/output working pair exists inside the backend. Stable pixel/input storage totals at most 64 MiB plus a 16 KiB read chunk. Input reallocation can temporarily hold old and new allocations, still below 64 MiB without an output buffer. Allocator bookkeeping, paths, PDFium internals, and total process memory are not covered by this byte accounting.
- PDFium writes directly into the owned output buffer in BGRA. The component closes its bitmap then swaps channels in place. There is no second output copy or cache.
- Dropping returned pixels releases them. The caller can retain arbitrary results, so integration must reserve working space and bound all retained results across the window. F07's 256 MiB CPU and 128 MiB GPU limits remain supervisor-owned. This crate allocates no GPU resources.
- A five-second cooperative deadline applies only to page rasterization. Parsing, file reads, and individual native calls can exceed it. This is not a hard wall-clock limit or a native-memory sandbox. Process isolation is the upgrade path if those become requirements.
- The engine/DLL stays loaded for the process lifetime. Every request closes its native document, page, bitmap, and progressive-render resources on success or error. This component must be the sole owner of PDFium initialization in the process.

Call only from the application's background worker. There are no threads or queues here. Integration must reject stale document/page/size results, reset page index on document changes, enforce shared cache limits, and keep browsing usable after errors. Each request rereads its file; concurrent external writers are not a snapshot guarantee.

## Active content

The component uses a directly resolved memory-load symbol and `pdfium-render`'s low-level bindings for the remaining calls, not a PDF interpreter. The high-level `PdfDocument` path initializes a form-fill environment, so this component intentionally does not use it. It calls memory loading, page count/version/dimensions, page loading, external bitmap creation, and progressive rendering with flags zero. It never initializes forms/XFA, invokes document/page actions, registers JavaScript callbacks, follows links, saves documents, or supplies the source path to PDFium.

Only the non-V8/non-XFA native build is supported. Rust features omit V8/XFA APIs, but that alone does not prove how a supplied DLL was built. Deployment must verify the provider/build configuration. Forms/widgets and interactive annotations are not rendered through a form environment. Native engine vulnerabilities are not made safe by skipping action APIs.

## Local installation support

Dependencies: `pdfium-render = 0.9.4`, exactly pinned, default features disabled, `pdfium_7881` and `thread_safe` enabled; `libloading = 0.9.0`, exactly pinned, already present transitively. Its ISC license and Rust dependency notices remain packaging obligations. The only lockfile change adds the existing libloading package to this component's direct dependency list. No `image`, static-link, Win32 rendering, V8, or XFA features. The component-local lockfile records transitive versions. The binding loads all exports for its selected API, so a missing symbol is a renderer-unavailable error even if this component would not use that symbol.

Proposed Windows x64 provider is bblanchon's non-V8 PDFium 151.0.7881.0 release. This is a third-party build provider, not Google/Foxit. Installation is explicit:

1. Obtain `pdfium-win-x64.tgz` from https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium/7881, not a `pdfium-v8-*` archive. This adapter supports Windows x64 only.
2. Verify the archive against its release digest and review provider provenance. The release API reports SHA-256 `73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08` for that x64 archive. P03b independently hashed the supervisor-provided archive and matched that value.
3. Extract it to a trusted operator-controlled installation directory, including its DLL dependencies. Protect the directory against untrusted replacement. Preserve its `licenses` directory. Pass the absolute path to its `bin/pdfium.dll` to `Renderer::load`. Do not load a renderer found in a browsed folder.
4. For the component probe only, set `PDFIUM_LIBRARY_PATH` to that absolute DLL file path and run the component `run` task. The library API itself does not read environment variables.

An equivalent trusted local build must use matching PDFium API 7881 and `pdf_enable_v8=false`, `pdf_enable_xfa=false`. Export binding is not a binary authenticity or feature attestation check. Upgrade and native security-update policy remain a release decision.

No native library is included or redistributed. This documents an installation route for F08.6, not completed clean-machine installation or distribution approval.

## Corrected ABI and loader

`pdfium-render` 0.9.4 has a demonstrated Windows x64 ABI defect. Its generated 7881 declaration uses `usize` for `FPDF_LoadMemDocument64`, but `dynamic_bindings.rs:67-71,4027-4034` uses `c_ulong` and casts the length. Windows x64 has 64-bit `size_t` and 32-bit `unsigned long`. The 32 MiB input cap is not an ABI repair, and the legacy trait wrapper forwards to that same wrong method.

The adapter resolves `FPDF_LoadMemDocument64` from the secure held module as `unsafe extern "system" fn(*const c_void, usize, *const c_char) -> FPDF_DOCUMENT`. The installed 7881 header declares `const void*`, `size_t`, and `FPDF_BYTESTRING`, a `const char*`. Its `FPDF_CALLCONV` is `__stdcall` with WIN32/FPDFSDK_EXPORTS, otherwise C. Microsoft documents that x64 ignores `__stdcall` and uses the common x64 ABI. The adapter deliberately supports only Windows x64. The native probe is behavioral evidence, not the proof of argument width.

The remaining invoked dynamic declarations and forwarding bodies were compared with the installed `fpdfview.h`/`fpdf_progressive.h`: InitLibrary, GetLastError, GetPageCount, GetFileVersion, LoadPage, GetPageWidthF/HeightF, Bitmap_CreateEx/Destroy, RenderPageBitmap_Start/RenderPage_Close, ClosePage/CloseDocument. These use handles/pointers, C int/float, or actual unsigned long for GetLastError. No other upstream load wrapper is invoked. IFSDK_PAUSE uses the header's C callback and version 1.

The upstream binding constructor privately owns its libloading handle and exposes no injection API. It still makes a flags-zero second load of the **same canonical absolute path while the secure first handle remains held**. Windows reuses an already-loaded executable module and increments its process reference count; it does not remap that module or resolve its imports again. The real probe compares HMODULE values from secure and flags-zero loads of the same held path and passes. This does not make a flags-zero first load safe. Engine retains both references for process lifetime. Source bytes remain alive and unmoved until DocumentPage closes the document, including error exits; pixels remain borrowed until native bitmap/render cleanup.

Per-load flags do not attest DLL provenance or establish a sandbox. Deployment must use the verified non-V8/non-XFA build in a trusted directory and make this component the sole PDFium loader/initializer. Windows API-set/KnownDLL/loaded-module/SxS rules still apply. Do not preload untrusted same-name dependencies or configure untrusted DLL redirection. The flags constrain this LoadLibraryExW call's directory search, not every later library load inside native code. No process-wide loader-policy claim is made.

## Inspected native artifact

The retained ignored artifact is `C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881`. VERSION is 151.0.7881.0; args.gn has `target_os="win"`, `target_cpu="x64"`, `pdf_enable_v8=false`, `pdf_enable_xfa=false`. Archive SHA-256 matched the release digest above; extracted DLL SHA-256 is `79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118`.

MSVC dumpbin 14.44.35225.0 `/dependents` and `/imports` show static DLL imports KERNEL32.dll, ADVAPI32.dll, GDI32.dll and USER32.dll, no separate V8/XFA/runtime DLL. The import listing includes LoadLibraryA/LoadLibraryExW, so this inventory does not prove absence or safety of runtime native loads. The generated loader regression separately demonstrates that a missing dependency present only in CWD/PATH is rejected, then succeeds when that same harmless generated dependency is installed beside its top DLL. Only isolated child processes change CWD/PATH.

The archive retains LICENSE and notices for PDFium, abseil, agg23, fast_float, freetype, icu, lcms, libjpeg_turbo, libopenjpeg, libpng, libtiff, llvm-libc, simdutf and zlib. Presence/inventory is not final redistribution approval. No artifact or binary is committed or redistributed.

## Primary-source evidence

- Crate release/API/features: https://docs.rs/crate/pdfium-render/0.9.4/source/Cargo.toml and https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/trait.PdfiumLibraryBindings.html. The downloaded crates.io 0.9.4 source was also inspected locally.
- Direct loader release/license: https://docs.rs/crate/libloading/0.9.0/source/Cargo.toml, ISC. The installed manifest was inspected.
- Rust binding license declaration, MIT OR Apache-2.0: https://github.com/ajrcarey/pdfium-render/blob/master/LICENSE.md. That short file references the standard license texts; packaging still needs Rust dependency notices.
- High-level form initialization avoided here: https://docs.rs/crate/pdfium-render/0.9.4/source/src/pdf/document/form.rs and https://docs.rs/crate/pdfium-render/0.9.4/source/src/pdf/document.rs.
- PDFium loading errors, memory buffer lifetime, external bitmap/stride and thread serialization contract: https://pdfium.googlesource.com/pdfium/+/refs/heads/chromium/7881/public/fpdfview.h. `FPDF_ERR_FORMAT=3`, `FPDF_ERR_PASSWORD=4`, `FPDF_ERR_SECURITY=5`; bitmap format BGRA is 4.
- Windows per-load flags and module reuse: https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw and https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw. Standard search/loaded-module rules: https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-search-order. The installed libloading 0.9.0 Windows implementation was inspected; `new` forwards flags zero and `load_with_flags` passes its flags to LoadLibraryExW.
- Windows x64 ABI: https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-170 and https://learn.microsoft.com/en-us/cpp/cpp/stdcall?view=msvc-170.
- Progressive status/deadline callback/cleanup: https://pdfium.googlesource.com/pdfium/+/refs/heads/chromium/7881/public/fpdf_progressive.h. DONE is 2; FAILED is 3. This status-bearing path avoids pretending that the void-returning normal render API proves successful rendering.
- Native release assets/digests: https://api.github.com/repos/bblanchon/pdfium-binaries/releases/tags/chromium%2F7881.
- Provider build flags at the chosen release: https://github.com/bblanchon/pdfium-binaries/blob/chromium/7881/steps/05-configure.sh. V8 defaults false; XFA uses the same flag.
- PDFium license at the chosen API branch: https://pdfium.googlesource.com/pdfium/+/refs/heads/chromium/7881/LICENSE. The opening BSD-style conditions permit binary redistribution with notices/disclaimer and prohibit unapproved endorsement. Do not reduce the entire native dependency tree to that opening license.
- Provider notice collection: https://github.com/bblanchon/pdfium-binaries/blob/chromium/7881/steps/08-licenses.sh. It collects PDFium and multiple third-party licenses and warns about unknown libraries. Release packaging must inspect the actual chosen archive's complete notices and dependencies; the build script is not blanket redistribution clearance.

## Checks

From the lane checkout:

```powershell
cd C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview
task --taskfile components/pdf-preview/Taskfile.yml
task --taskfile components/pdf-preview/Taskfile.yml check
task --taskfile components/pdf-preview/Taskfile.yml test
```

`check`, `test`, and `run` use locked Cargo resolution. Default checks require rustc/linker availability for the two small generated DLL fixtures. The default run passes nine checks, with the native probe opt-in and an auxiliary isolated-child test ignored in the parent. The loader test invokes that child itself twice, checking missing-dependency failure then installed-dependency success. The signature check exercises a harmless full-width value greater than u32::MAX without reading a buffer or calling PDFium, and guards against either upstream memory-load wrapper reappearing in production source.

`run` executes the opt-in native fixture probe, not a GUI. It fails if `PDFIUM_LIBRARY_PATH` is absent or loading/rendering fails. Default tests explicitly ignore that probe. P03b separately executed it against the retained trusted DLL successfully; this does not verify a different installed binary or clean-machine deployment. `clean` removes only build output under root `target/p03`.

The native probe creates and removes only a uniquely claimed disposable directory. It generates a simple red-square PDF, malformed input, and a deterministic password-protected blank PDF. It checks metadata, RGBA channel order/pixels, physical-size rerendering, page/output/input bounds including exact four-byte output, source preservation, I/O failure, malformed/password distinction, engine reuse, native HMODULE reuse, and library-switch refusal. Encryption constants were generated with pypdf 6.20.0 using user password `secret`, owner password `owner`, and RC4-40 solely as a test fixture. No Python runtime or encryption implementation is part of this library.
