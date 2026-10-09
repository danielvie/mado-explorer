# P03b PDF backend handoff

## Candidate and boundary

- Checkout C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview, branch lane/pdf-preview.
- Starting clean HEAD 418b02b5b90284bbfe06d3a186518bcfe7632283. The candidate commit SHA is reported separately after commit.
- Writes only components/pdf-preview/**. No main/S011/sibling/root changes, delegation, merges, pushes, acceptance-tracker edits, personal data, active PDF actions, or binary redistribution.
- Gate remains unpromoted. Supervisor must obtain fresh safety review before promotion/enabling.

Changed paths:

```text
components/pdf-preview/Cargo.toml
components/pdf-preview/Cargo.lock
components/pdf-preview/Taskfile.yml
components/pdf-preview/src/lib.rs
components/pdf-preview/src/native.rs
components/pdf-preview/tests/backend.rs
components/pdf-preview/README.md
components/pdf-preview/HANDOFF.md
```

Existing tests/support/mod.rs is reused unchanged for uniquely claimed disposable directories. Generated PDF/DLL fixtures are removed by their guards. The supervisor-provided native archive/DLL/header/licenses remain under ignored target/native-probe-pdfium-7881, not in the commit.

## Blocker corrections

1. The render path no longer invokes either upstream memory-load trait wrapper. Engine stores the directly resolved FPDF_LoadMemDocument64 symbol from the held secure module. Its exact native signature uses const void*, usize, const char* and FPDF_DOCUMENT with extern system. The installed 7881 fpdfview.h specifies size_t. Windows x64 C/system calling conventions agree, including the header's optional __stdcall. The adapter fails unavailable outside Windows x64, without a less secure fallback. The original 32 MiB read cap stays unchanged and is not used as an ABI argument.
2. The first load now uses libloading 0.9.0's Windows load_with_flags with LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32, 0x900. It uses the selected canonical absolute path, fails without fallback, and does not alter global process loader policy. Direct libloading is exactly pinned to the already-transitive version. Cargo's sole lockfile change is its addition to this component's direct dependencies.

The upstream private DynamicPdfiumBindings cannot accept an injected Library. Pdfium::bind_to_library still performs a second flags-zero load of the exact same canonical module while our secure handle remains held. Windows reuses that module and increments its reference count, without resolving imports again. Microsoft LoadLibrary/LoadLibraryEx/module-search documentation is linked in README. The real probe also asserts secure and flags-zero loads return the same HMODULE. This is not a claim that flags zero is independently safe. Engine retains both handles for process lifetime.

The other 13 invoked dynamic declarations/forwarding bodies were compared with the installed 7881 fpdfview.h/fpdf_progressive.h. They use actual int/float/unsigned long and native pointers/handles, without the demonstrated size_t narrowing. IFSDK_PAUSE retains its header-declared C callback. No duplicate wholesale PDFium binding or dependency upgrade/vendor was added.

Precise lifetimes remain intact. render_file owns immutable source bytes throughout render and native document cleanup. DocumentPage closes page then document before that input is dropped. BitmapRender borrows output exclusively until progressive/bitmap cleanup. Native symbol and both modules live beyond every request. Engine serializes native calls, error access and cleanup.

## Regression evidence

- corrected_signature_and_route calls a harmless function using the exact LoadMemDocument64 type with a value greater than u32::MAX. It never accesses a buffer or calls PDFium with that large value. It asserts Windows usize/c_ulong widths, loader flags 0x900, direct render call, and absence of both upstream load-wrapper routes in production source. Header inspection establishes the native declaration; the test/native render alone are not ABI proof.
- dependencies_do_not_fall_back_to_cwd_or_path compiles two harmless generated Rust DLLs using the installed toolchain. It starts isolated test children with the dependency only in CWD/PATH and asserts load failure. After copying the dependency beside the top DLL, another child must load and return the fixture's value 7. Parent CWD/PATH/loader state stay unchanged. The ignored search_path_child is an auxiliary entry point actually invoked twice by this passing parent test, not an unavailable check.
- Existing missing/invalid/relative checks pass. Native probe adds explicit library-switch refusal and exact 1x1/four-byte output. Request tests add exact native signed-index boundary checks. Existing page/read/budget/fit/error/deadline checks remain.

## Exact validation commands and results

All shell commands began in the exact checkout:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview
```

Final checks:

```sh
cargo fmt --manifest-path components/pdf-preview/Cargo.toml -- --check
# Passed.
cargo check --locked --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
# Passed.
cargo test --locked --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
# Passed: 8 unit checks and 1 integration check.
# Parent ignores auxiliary loader child and opt-in native probe, both run separately as described.
cargo test --locked --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03 --lib native::tests::dependencies_do_not_fall_back_to_cwd_or_path -- --nocapture
# Passed, including both isolated generated-DLL scenarios.
PDFIUM_LIBRARY_PATH='C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll' cargo test --locked --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03 --test backend native_fixture_probe -- --ignored --nocapture
# Passed: native corrected loading/rendering, exact HMODULE reuse, pixels/channel order,
# metadata, sharper resizing, page bounds, malformed/password states, limits,
# missing source, library-switch refusal and unchanged generated source bytes.
cargo tree --locked --manifest-path components/pdf-preview/Cargo.toml -e features -i libloading
# Passed: one libloading 0.9.0, shared by this component and pdfium-render 0.9.4.
task --taskfile components/pdf-preview/Taskfile.yml
# Passed: described default/run/test/check/clean task list.
task --taskfile components/pdf-preview/Taskfile.yml check
task --taskfile components/pdf-preview/Taskfile.yml test
PDFIUM_LIBRARY_PATH='C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll' task --taskfile components/pdf-preview/Taskfile.yml run
# All three passed, executing the locked compile/default/native commands above.
git diff --check
# Passed, with only existing Git LF-to-CRLF notices. Staged diff checked before commit too.
```

The one unlocked `cargo check --offline --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03` refreshed the component-local lockfile after adding the direct pin. No existing package version/checksum changed. All subsequent checks use --locked; component check/test/run tasks now do too. No clean command ran, preserving incremental output and the trusted artifact.

One initial test run failed while linking the generated top DLL, LNK1181 missing mado_p03_loader_dep.lib. Rust's cdylib output is mado_p03_loader_dep.dll.lib. The fixture now names that actual import library with +verbatim. The first focused fix rerun passed; no two failed fixes remain and no assertion was removed.

## Native artifact and inventory

Retained directory C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881:

- pdfium-win-x64.tgz SHA-256 independently recomputed as 73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08, matching the supervisor/release digest.
- bin/pdfium.dll SHA-256 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118.
- VERSION 151.0.7881.0 and args.gn inspected. target_os win, target_cpu x64, pdf_enable_v8 false, pdf_enable_xfa false.
- LICENSE and 15 notice files retained. Covered names: PDFium, abseil, agg23, fast_float, freetype, icu, lcms, libjpeg_turbo, libopenjpeg, libpng, libtiff, llvm-libc, simdutf, zlib. libjpeg_turbo has two notice files. Inventory is not legal redistribution clearance.

Exact inventory/hash commands:

```sh
MSYS_NO_PATHCONV=1 'C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/dumpbin.exe' /dependents target/native-probe-pdfium-7881/bin/pdfium.dll
MSYS_NO_PATHCONV=1 'C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.44.35207/bin/Hostx64/x64/dumpbin.exe' /imports target/native-probe-pdfium-7881/bin/pdfium.dll
node -e "const fs=require('fs');console.log(require('crypto').createHash('sha256').update(fs.readFileSync('target/native-probe-pdfium-7881/pdfium-win-x64.tgz')).digest('hex'));"
node -e "const fs=require('fs');console.log('DLL SHA256 '+require('crypto').createHash('sha256').update(fs.readFileSync('target/native-probe-pdfium-7881/bin/pdfium.dll')).digest('hex'));console.log('licenses '+fs.readdirSync('target/native-probe-pdfium-7881/licenses').join(', '));"
```

Passed. dumpbin 14.44.35225.0 reports static imports KERNEL32.dll, ADVAPI32.dll, GDI32.dll, USER32.dll, no separate native runtime/V8/XFA DLL. It also lists imported LoadLibraryA/LoadLibraryExW. The static inventory does not establish every runtime-loaded dependency. An initial PowerShell Get-FileHash command was unavailable, so Node computed hashes. An initial dumpbin command failed because Git Bash rewrote /dependents as a path; MSYS_NO_PATHCONV=1 corrected that tooling failure.

## Remaining risks and decisions

No complete F08 acceptance criterion is claimed verified. Backend portions of F08.1-F08.6 have code/native evidence; pane UI, page/zoom controls, stale-result identity, reset on document change, shared CPU/GPU budgets, packaging, accessibility/scaling and clean-machine checks remain supervisor work. Main tracker and S011 are untouched.

Trusted caller DLL/directory provenance is still the deployment contract, not enforced by API authenticity/build attestation. Use the inspected non-V8/non-XFA x64 build, protect its installation directory/dependencies, and make this component the sole PDFium loader/initializer. Windows known-module/API-set/SxS/redirection rules still apply, including preloaded dependencies. Per-load search flags do not constrain every later LoadLibrary call made inside native code and are not a sandbox. No global policy was set; adopting one would need a separate supervisor decision.

Application-owned input/output caps remain 32 MiB each, 8192 physical pixels per axis and the caller's smaller output allowance. Native internal memory, total process memory, uninterruptible parse/I/O/call time, caller-retained results and GPU allocations are not capped by this component. Rasterization deadline is cooperative only. Native security updates/provider trust, final license review and redistribution approval remain release decisions.

Ready for fresh read-only review of the adapter, retained-handle load sequence, fixtures and lifetime proof. Do not promote or enable on personal data solely from this handoff.
