# P03 PDF backend handoff

## Candidate and ownership

- Checkout: C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview.
- Branch: lane/pdf-preview, based on accepted 6b363f9.
- Candidate is the commit containing this handoff. Its exact SHA is returned to the supervisor after commit; no self-referential SHA is embedded here.
- Only components/pdf-preview/** changed. No root manifests/lockfiles, GUI, tracker, sibling components, main edits, merges, pushes, or delegation.
- Fresh review and supervisor promotion are still required. Nothing is enabled on personal data.

Owned files:

```text
components/pdf-preview/Cargo.toml
components/pdf-preview/Cargo.lock
components/pdf-preview/Taskfile.yml
components/pdf-preview/src/lib.rs
components/pdf-preview/tests/backend.rs
components/pdf-preview/tests/support/mod.rs
components/pdf-preview/README.md
components/pdf-preview/HANDOFF.md
```

## API and behavior

`Renderer::load(&Path)` accepts an absolute, trusted local renderer file, returning a typed renderer-unavailable error instead of searching browsed folders, PATH, or CWD. One initialized engine persists for the process lifetime; repeat loads of its canonical path reuse it. Other renderer paths require restart.

`Renderer::render_file(&Path, RenderRequest)` returns independent RGBA8 bytes, explicit output width/height, native source path/file name, input byte count, page count, optional PDF version, zero-based current index, and intrinsic point dimensions. Requested dimensions are physical-pixel fit bounds. No UI, cache, GPU, threads, or request-routing identity is included.

Limits: 32 MiB input, 32 MiB output or a smaller caller-assigned allowance, 8192 pixels per axis, positive finite page dimensions, checked byte arithmetic, validated native/index bounds, growth-aware chunk reads, fallible application pixel/input allocations. A single engine lock serializes reading/rendering and native last-error access. Native engine memory and uninterruptible I/O/parsing/native-call time are not bounded. The five-second rasterization deadline is cooperative, not a hard timeout.

A direct external BGRA bitmap avoids a second output allocation. Guards close progressive resources, bitmap, page, and document in order; channel conversion happens afterward. No source writes or save/form/action/link/JavaScript/XFA calls. Password input is deferred. Error variants distinguish missing renderer, malformed document, password required, unsupported security, unexpected native load codes, invalid/index/size bounds, I/O/allocation failures, native render stages/status, and deadline expiration.

Integration obligations remain: dedicated background invocation, document-change reset, page controls, zoom/display-scale sizing, pane preview state, stale-result rejection, shared CPU/GPU reservation/cache eviction, fallback UI, release installation and security-update policy. All PDFium initialization in the application must go through this component; mixing another owner is unsupported.

## Exact checks and outcomes

All commands began with:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview
```

Final focused commands:

```sh
cargo fmt --manifest-path components/pdf-preview/Cargo.toml
# Passed; formats only this standalone crate.
task --taskfile components/pdf-preview/Taskfile.yml check
# Passed: cargo fmt --manifest-path components/pdf-preview/Cargo.toml -- --check
# Passed: cargo check --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
task --taskfile components/pdf-preview/Taskfile.yml test
# Passed: cargo test --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
# Six unit tests + one missing/invalid/relative-library regression passed.
# One native fixture probe explicitly ignored, pending, not passed.
cargo test --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03 --test backend missing_or_invalid_local_library_is_explicit
# Final affected test rerun passed after making rejection assertions compatible with
# a process engine already initialized by the opt-in probe. Missing/relative paths still
# require RendererUnavailable; an invalid existing binary may instead be refused as
# RendererAlreadyLoaded when another engine is already live. Formatting/check passed again.
task --taskfile components/pdf-preview/Taskfile.yml
# Passed: default lists all described tasks.
task --taskfile components/pdf-preview/Taskfile.yml --dry clean
# Passed dry-run only: cargo clean --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
# Actual clean was not executed, to retain the incremental build.
git diff --check
# Passed. The staged candidate is checked again before commit.
```

Bootstrap also passed direct `cargo check --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03` and direct `cargo test --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03`. Toolchain is cargo/rustc 1.95.0 on Windows; Task 3.50.0. The standalone lockfile resolved 54 packages. Builds stay under ignored root target/p03.

One initial Taskfile check failed with `manifest path components/pdf-preview/Cargo.toml does not exist`. Changing a top-level directory variable alone did not fix it. Diagnosis: Task requires task-level `dir`, not a top-level `dir`. Task-level checkout-root directories fixed the second focused attempt; the final commands above pass. No tests were removed or weakened.

The reader tests cover exact-limit, over-limit, multichunk and growth-equivalent inputs and capped capacity. Other tests cover zero/excessive dimensions, impossible budgets, native integer overflow, page count/index bounds, fit/aspect/DPI-size arithmetic, NaN/infinite/negative dimensions, load-error classification, and the cooperative deadline callback.

## Native evidence and pending checks

`PDFIUM_LIBRARY_PATH`, `PDFIUM_DYNAMIC_LIB_PATH`, and `PDFIUM_LIB_PATH` were unset. `where.exe pdfium.dll` found no PATH library; a lane file search found no pdfium.dll. No native archive was downloaded, installed, or redistributed. Native rendering is unavailable in this environment.

The opt-in `native_fixture_probe` is compiled but not executed. It generates only disposable simple/malformed/encrypted documents and checks actual pixels/channel order, metadata, sharper size rerendering, engine reuse, page/byte limits, I/O failures, distinct malformed/password errors, and unchanged source bytes. The missing-library regression really executed and passed; it does not stand in for rendering.

Resolve the pending native check by installing the trusted non-V8/non-XFA 7881 renderer and running:

```powershell
cd C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview
$env:PDFIUM_LIBRARY_PATH = 'C:/operator-owned/pdfium/bin/pdfium.dll'
task --taskfile components/pdf-preview/Taskfile.yml run
```

An independent fixture-only preflight passed with pypdf 6.20.0. It authenticated the generated encrypted fixture with `secret`, rejected `wrong`, and found one page. Exact command, run in Git Bash after the explicit checkout cd:

```sh
uvx --from pypdf python -c "import io,re;from pathlib import Path;from pypdf import PdfReader;s=Path('components/pdf-preview/tests/support/mod.rs').read_text(encoding='utf-8');id=re.search(r'let id = \"([0-9a-f]+)\"',s)[1];enc=re.search(r'\"(<< /Filter /Standard[^\"]+)\"',s)[1];objs=['<< /Type /Catalog /Pages 2 0 R >>','<< /Type /Pages /Count 1 /Kids [3 0 R] >>','<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << >> >>',enc];b=b'%PDF-1.3\n';offsets=[]
for i,o in enumerate(objs):
 offsets.append(len(b));b+=f'{i+1} 0 obj\n{o}\nendobj\n'.encode()
x=len(b);b+=b'xref\n0 5\n0000000000 65535 f \n'+b''.join(f'{o:010} 00000 n \n'.encode() for o in offsets);b+=f'trailer\n<< /Size 5 /Root 1 0 R /Encrypt 4 0 R /ID [<{id}> <{id}>] >>\nstartxref\n{x}\n%%EOF\n'.encode();r=PdfReader(io.BytesIO(b),strict=True);assert r.is_encrypted;assert r.decrypt('wrong')==0;assert r.decrypt('secret')>0;assert len(r.pages)==1;print('Generated encrypted fixture authenticated with secret, rejects wrong password, one page; not a PDFium render probe.')"
```

This does not verify PDFium parsing/rendering or add a Python application dependency.

## Dependency, license and deployment evidence

`pdfium-render = 0.9.4` is exactly pinned with only `pdfium_7881` and `thread_safe`; no default image/V8/XFA/static/Win32 features. Low-level bindings were chosen because the high-level document constructor initializes forms, and progressive rendering provides failure status whereas normal rendering returns void.

Primary URLs and factual limits are recorded in the component README:

- https://docs.rs/crate/pdfium-render/0.9.4/source/Cargo.toml
- https://github.com/ajrcarey/pdfium-render/blob/master/LICENSE.md declares MIT OR Apache-2.0.
- https://pdfium.googlesource.com/pdfium/+/refs/heads/chromium/7881/LICENSE permits binary redistribution subject to notices/disclaimer/non-endorsement conditions; third-party notices still require review.
- https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium/7881 provides the proposed Windows x64 non-V8 renderer.
- https://github.com/bblanchon/pdfium-binaries/blob/chromium/7881/steps/05-configure.sh establishes non-V8/non-XFA default build flags.
- https://github.com/bblanchon/pdfium-binaries/blob/chromium/7881/steps/08-licenses.sh establishes collection of native third-party notices and warnings for unknown dependencies.

The release API reports SHA-256 `73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08` for pdfium-win-x64.tgz. It was not downloaded or independently checked. README gives explicit local installation support, DLL location, probe setup, trust/architecture/API requirements, and no-current-directory fallback. Actual archive license inventory, clean-machine installation, final redistribution approval and packaged explorer integration remain pending.

## Acceptance and blockers

Backend-only partial F08.1-F08.6 preparation. No complete F08 criterion or UI acceptance is proposed as verified. Page/index/size guards and missing-renderer behavior have focused evidence; actual native successful/error rendering remains pending.

No Rust implementation/build blocker. Native DLL absence blocks the native probe, not independent backend preparation. Promotion needs fresh read-only review; enabling previews needs native evidence and supervisor-owned UI/cache/routing/deployment work. No two unsuccessful focused fix attempts remained unresolved.
