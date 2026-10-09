# PDF preview backend

P03 is a standalone read-only Rust library, not an enabled explorer feature. It prepares backend portions of F08.1-F08.6. No UI acceptance, stale-result routing, window cache budget, GPU behavior, or packaging check is claimed.

## API

`Renderer::load(&absolute_library_path)` loads one trusted local PDFium library. There is no current-directory or system search fallback and no download. Missing files, bad binaries, wrong architecture, and missing exports return `Error::RendererUnavailable` with the loader reason. The same canonical library path reuses the engine; another library needs a process restart.

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

The component uses `pdfium-render`'s established low-level bindings, not a PDF interpreter. The high-level `PdfDocument` path initializes a form-fill environment, so this component intentionally does not use it. It calls memory loading, page count/version/dimensions, page loading, external bitmap creation, and progressive rendering with flags zero. It never initializes forms/XFA, invokes document/page actions, registers JavaScript callbacks, follows links, saves documents, or supplies the source path to PDFium.

Only the non-V8/non-XFA native build is supported. Rust features omit V8/XFA APIs, but that alone does not prove how a supplied DLL was built. Deployment must verify the provider/build configuration. Forms/widgets and interactive annotations are not rendered through a form environment. Native engine vulnerabilities are not made safe by skipping action APIs.

## Local installation support

Dependency: `pdfium-render = 0.9.4`, exactly pinned, default features disabled, `pdfium_7881` and `thread_safe` enabled. No `image`, static-link, Win32 rendering, V8, or XFA features. The component-local lockfile records transitive versions. The binding loads all exports for its selected API, so a missing symbol is a renderer-unavailable error even if this component would not use that symbol.

Proposed Windows x64 provider is bblanchon's non-V8 PDFium 151.0.7881.0 release. This is a third-party build provider, not Google/Foxit. Installation is explicit:

1. Obtain `pdfium-win-x64.tgz` from https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium/7881, not a `pdfium-v8-*` archive. Match the process architecture for other targets.
2. Verify the archive against its release digest and review provider provenance. The release API reports SHA-256 `73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08` for that x64 archive. No archive was downloaded or independently hashed in this lane.
3. Extract it to an operator-owned installation directory. Preserve its `licenses` directory. Pass the absolute path to its `bin/pdfium.dll` to `Renderer::load`. Do not load a renderer found in a browsed folder.
4. For the component probe only, set `PDFIUM_LIBRARY_PATH` to that absolute DLL file path and run the component `run` task. The library API itself does not read environment variables.

An equivalent trusted local build must use matching PDFium API 7881 and `pdf_enable_v8=false`, `pdf_enable_xfa=false`. Export binding is not a binary authenticity or feature attestation check. Upgrade and native security-update policy remain a release decision.

No native library is included or redistributed. This documents an installation route for F08.6, not completed clean-machine installation or distribution approval.

## Primary-source evidence

- Crate release/API/features: https://docs.rs/crate/pdfium-render/0.9.4/source/Cargo.toml and https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/trait.PdfiumLibraryBindings.html. The downloaded crates.io 0.9.4 source was also inspected locally.
- Rust binding license declaration, MIT OR Apache-2.0: https://github.com/ajrcarey/pdfium-render/blob/master/LICENSE.md. That short file references the standard license texts; packaging still needs Rust dependency notices.
- High-level form initialization avoided here: https://docs.rs/crate/pdfium-render/0.9.4/source/src/pdf/document/form.rs and https://docs.rs/crate/pdfium-render/0.9.4/source/src/pdf/document.rs.
- PDFium loading errors, memory buffer lifetime, external bitmap/stride and thread serialization contract: https://pdfium.googlesource.com/pdfium/+/refs/heads/chromium/7881/public/fpdfview.h. `FPDF_ERR_FORMAT=3`, `FPDF_ERR_PASSWORD=4`, `FPDF_ERR_SECURITY=5`; bitmap format BGRA is 4.
- Progressive status/deadline callback/cleanup: https://pdfium.googlesource.com/pdfium/+/refs/heads/main/public/fpdf_progressive.h. DONE is 2; FAILED is 3. This status-bearing path avoids pretending that the void-returning normal render API proves successful rendering.
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

`run` executes the opt-in native fixture probe, not a GUI. It fails if `PDFIUM_LIBRARY_PATH` is absent or loading/rendering fails. Default tests explicitly ignore that probe; an ignored probe is pending, not passed. `clean` removes only build output under root `target/p03`.

The native probe creates and removes only a uniquely claimed disposable directory. It generates a simple red-square PDF, malformed input, and a deterministic password-protected blank PDF. It checks metadata, RGBA channel order/pixels, physical-size rerendering, page/output/input bounds, source preservation, I/O failure, malformed/password distinction, and engine reuse. Encryption constants were generated with pypdf 6.20.0 using user password `secret`, owner password `owner`, and RC4-40 solely as a test fixture. No Python runtime or encryption implementation is part of this library.
