# PDF preview service handoff

## P14 public Service native integration check

Checkout C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport, branch lane/raster-viewport. Clean accepted starting HEAD f522f04ebc35dffdab2525c3614c782d091abe2e. Changes are restricted to components/pdf-preview-service/tests/native_service.rs, Taskfile.yml and this HANDOFF.md. No production source, dependency/lock, root/shared/sibling edits, redistribution, GUI/GPU, clipboard, elevation, personal-document access or enablement. The existing PDF artifact lane was read-only. Terminal SHA is reported separately for fresh exact-head review; no self-approval or integration.

### Provenance before native execution

Read accepted service/backend production APIs, native loader, backend fixture/probe support, README trust/deployment evidence, SPEC.md/CONTEXT.md/AGENTS.md and main PROGRESS.md P14. Rechecked the existing artifact locally, without downloads:

- Managed root C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881.
- Archive pdfium-win-x64.tgz, documented provider https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium/7881, SHA-256 73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08.
- bin/pdfium.dll SHA-256 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118.
- VERSION reports MAJOR=151, MINOR=0, BUILD=7881, PATCH=0.
- Complete args.gn checked against the retained release configuration: Windows/x64, release, standalone, non-component, pdf_enable_v8=false, pdf_enable_xfa=false, partition allocator disabled, warnings-as-errors disabled. bin contains only pdfium.dll.

A read-only pwsh Get-FileHash command independently matched both documented hashes before the first native run. The ignored test repeats local VERSION/configuration/bin inventory and SHA-256 verification before Service::start. It requires MADO_P14_TRUSTED_PDFIUM to equal the fixed absolute managed DLL path. Arbitrary/environment-selected libraries are refused, with no fallback or download. The PowerShell command hashes only fixed documented paths, never interpolated operator input. Missing artifacts, missing pwsh, unexpected configuration or hash mismatch fail the explicit test before native loading; they must be reported pending/blocked, not counted as passing. An explicit runtime Windows-x64 guard keeps the named ignored test present and fail-closed on unsupported hosts, rather than allowing a zero-test native task to pass. Default tests never call this gate or load this DLL.

This is verification of the retained operator-controlled artifact, not a sandbox, independent provider attestation, protection against concurrent trusted-directory replacement or packaging approval. The accepted secure loader/sole-initializer constraints remain unchanged.

### Actual API procedure and assertions

One test calls public Service::start with the verified path and one CpuBudget. No private RenderCall/start_inner, direct Renderer calls, alternate engine initialization or GUI is used.

1. Create two targets. Submit an owned Unicode-named two-page PDF at page index 0 in a 400x400 physical box. Mutate the caller's PathBuf immediately afterward. The actual taken response still captures the original native path, generation and metadata. PDF 1.3, page count 2, input byte count, 200x100 point page, 400x200 RGBA output, white background, red/green/blue sample regions and every alpha byte are checked.
2. Request page index 1 on the independent target. Resize the first target's SAME document to 600x600. Its new actual bitmap is 600x300, with the same RGB content and metadata; the already-taken old response is deterministically no longer current. Page 2 is 100x200 points, rendered 200x400 with its blue center. No assertion guesses that old native work is unfinished.
3. Replace the first target's document with a separately generated PDF whose red/blue regions are reversed. The taken response has the replacement native path/content/current generation. The earlier sharper frame remains held but is no longer current; the other target remains current.
4. Through that same public worker, request generated malformed and password-required documents and an owned missing source name. Assert actual Backend/MalformedDocument, PasswordRequired and Io states, nonempty messages, captured paths/generations, current-error routing and unchanged independent target.
5. Cancel a newly accepted valid request and prove its generation invalidated. A subsequent actual render on the independent target proves previous cancelled work has passed the worker boundary, whether it was pending, active or already complete. No cancelled result is taken. Close a target with a current valid request; the target/generation is retired and take/request refuse UnknownTarget. Another actual independent render proves closed work/allocations have finished. No timing-dependent pending/active claim.
6. Taken native frames retain exact capacity charges across replacement, errors, cancel, close and actual worker join. The four held frames charge 320,000 + 720,000 + 720,000 + 320,000 = 2,080,000 bytes. Temporary 200x100 barrier frames add/release 80,000 bytes each. Native errors release their working admission. Signal request_stop, observe Stopped, poll try_join until actual successful completion, and verify a subsequent try_join has no handle. Drop service, then each held frame; exact budget totals decrease to zero without underflow/leak.
7. Verify every generated source byte and owned directory sentinel byte, plus exact root/sentinel entry inventories. The fixture guard claims a fresh directory with create_dir and individual files with create_new, rejects changed/link/unknown-entry cleanup, and removes only its enumerated files and empty claimed directories. No recursive deletion. Confirm the owned root no longer exists after successful cleanup.

The fixture writer copies only the accepted xref pattern and deterministic password constants, plus the new tiny two-page RGB geometry. Built-in Rust tests/std only, no new framework/dependency or binary fixtures. Maximum requested physical box is 600x600. Thirty-second observation timeouts diagnose these tiny generated checks; they are NOT hard cancellation/shutdown deadlines. Source reads/native calls may be uninterruptible, and Service retains its worker ownership and blocking Drop/join fallback on a failed observation. This check stops an idle worker; it does not pretend to interrupt native I/O or quantify PDFium internal/process memory.

### Exact commands and outcomes

All Cargo/Task/Git commands below ran from C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport. Initial provenance command:

```sh
pwsh -NoProfile -NonInteractive -Command "Get-FileHash -Algorithm SHA256 -LiteralPath 'C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/pdfium-win-x64.tgz','C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll' | Format-List Path,Hash"
```

Both hashes matched. Implemented and ran:

```sh
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml -- --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
MADO_P14_TRUSTED_PDFIUM='C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll' cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11 --test native_service native_service_managed_pdfium_7881 -- --exact --ignored --nocapture
task --taskfile components/pdf-preview-service/Taskfile.yml --list
task --taskfile components/pdf-preview-service/Taskfile.yml run
task --taskfile components/pdf-preview-service/Taskfile.yml --dry native TRUSTED_PDFIUM='C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll'
```

Initial fmt/all-target compile/default twenty checks passed, new native check explicitly ignored by default. Initial explicit native run passed one test, zero ignored/failures. Inspection strengthened the test to distinguish same-document resize from document replacement and added an actual close-cleanup worker barrier. No failing checks or production defect were exposed. After that coherent strengthening, reran fmt/check/default test and verified the actual opt-in task wiring:

```sh
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml -- --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
task --taskfile components/pdf-preview-service/Taskfile.yml native TRUSTED_PDFIUM='C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll'
git diff --check
git diff --cached --check
```

Final fmt and locked offline all-target compile pass. Default suite passes twenty unchanged unit tests, zero failures, plus one explicitly ignored integration test and zero doc tests. After adding that unsupported-host guard, the same fmt/all-target/default-test/native-task commands were rerun successfully on the final code. Actual native task passes one integration test, zero failures/ignored, in 0.48 seconds, with both provenance hashes printed before the public-Service success report. Safe task run passes its unchanged controlled FIFO test without DLL/GUI. Task list and native dry run pass. Diff checks pass. No full root suite, GUI/GPU, benchmark, malicious/OOM input, native package or clean task was run.

### Criterion limits and review gate

P14 now supplies actual public-Service native evidence for the commissioned F08.3 physical output/rerender and F08.5 current-result/read-only/CPU-buffer-lifetime portions. Native malformed/password/missing-source fallbacks also strengthen F08.4 backend evidence. These are partial criteria, NOT complete UI/acceptance verification. Parent owns criterion status, fresh review, approval, promotion and updated-main validation.

Still pending: pane/selection/page controls integration, density/zoom/resize request wiring, old-bitmap UI presentation, input responsiveness, GUI conversion/upload/cache admission, shared image/PDF CPU integration, 128 MiB GPU ownership/eviction, native internal-memory security/update policy, installation/redistribution/license/clean-machine checks, scaling/Narrator/performance and checkpoint clipboard gates. No production preview enablement. Existing controlled tests remain the deterministic proof for forced in-flight/blocked-worker timing and budget-limit/source-growth edge cases. The native check complements, not replaces, those tests.

Everything below records P11/S013-R08 historical evidence. Its original native-service-pending statements are superseded only by this P14 check, not by GUI/GPU or release acceptance.

## S013-R08 source reservation correction

Starting HEAD a545a26ab89cfa8e493f81e3d50437c50275c100 was clean on lane/raster-viewport in C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport. This correction changes only components/pdf-preview-service/src/lib.rs, src/tests.rs and HANDOFF.md. No accepted PDF dependency, manifest, root/shared file, output limit, lease lifetime, routing, shutdown or production gate changes. The fixed shared CPU ceiling remains 256 MiB. The source constant now reserves 2 * pdf::MAX_INPUT_BYTES + 16 * 1024 before the render call. Rust checks this constant arithmetic at compilation; the existing source-plus-output admission remains checked at runtime.

Fresh review af0049cc-2d7a-40a, recorded in latest main PROGRESS.md under P11/S013-R08, found the P1 counterexample in accepted components/pdf-preview/src/lib.rs:348-374. A first read of 16,383 bytes followed by 16,384-byte chunks permits exact capacities 16,383, 32,767, 65,534 and further doublings to 33,553,408, then a clamp to 33,554,432. Assuming allocator growth overlaps old and new allocation layouts, the source peak is 33,553,408 + 33,554,432 + 16,384 = 67,124,224 bytes. That exceeds the old 67,108,864-byte reservation by 15,360. Held logical allocations of 256 MiB minus the old reservation minus a minimal 4-byte output leave room for the old admission, but source growth alone exceeds the ceiling by 15,356 bytes. Exact Vec capacity does not fix that arithmetic. No routing, lease, stop or API defect was reported.

Two added regressions retain all eighteen original tests unchanged:

- A numeric trace follows the accepted capped-read allowance, doubling/clamp capacity rule and excess-byte rejection using public MAX_INPUT_BYTES. It checks the complete read_bounded source body through a read-only include_str snapshot, ignoring whitespace only. A backend algorithm change fails the coupling check and requires re-auditing this trace and reservation. It allocates only the small capacity trace, not a 32 MiB source. This is allocation-layout accounting, not an actual allocator-overlap measurement.
- Worker admission with logical held leases and a 1x1 request must refuse the counterexample before entering the controlled renderer. It also refuses one byte above the new conservative boundary, admits exactly at that boundary, releases the working reservation after an injected backend error, reuses the budget, and retains the actual 4-byte frame charge until frame drop. Releasing held leases restores the budget for another render. No huge held-data fixture or DLL rendering is needed.

Exact commands, all from the checkout above:

```sh
cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11 s013_r08 -- --nocapture
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml -- --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11 s013_r08 -- --nocapture
cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
git diff --check
```

RED before the constant fix: the first command exited 101, two tests failed, eighteen filtered out. The numeric assertion reported left 67108864 versus right 67125248. The admission assertion received Backend/MalformedDocument from the controlled renderer instead of BudgetExceeded, proving that the old admission entered it. GREEN after the constant fix: formatting and locked offline all-target check pass; both focused tests pass; full suite passes twenty tests, zero failures/ignored, plus zero doc tests. No unsuccessful fix attempts or weakened assertions. Git diff check passes, with only Git's existing LF-to-CRLF notices. The full suite retains its original generated-empty-directory missing-library fallback check; no DLL is loaded.

F08.3/F08.5 remain partial. Actual native service rendering, GUI conversion/cache/GPU integration and all existing release/manual gates below remain pending. No GUI, personal data, clipboard, native/DLL probe, dependency update, budget waiver or production enablement was performed. The owned terminal SHA is reported separately for fresh exact-head safety review only, not self-approval or integration. Prior P11 implementation evidence below is historical; the corrected accounting proof is current.

## Candidate and scope

- Checkout C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport, branch lane/raster-viewport.
- Clean accepted starting HEAD 09f778f047da0a51192e34ac0d4203e5aa7d00d9, after P07 integration and twenty-test validation.
- Exclusive changes in components/pdf-preview-service/**. Accepted components/pdf-preview is a read-only path dependency. No root/shared/sibling changes or dependency upgrades. Final component commit SHA is reported with the terminal result.
- Targets F08.3/F08.5 background/current-result and application-owned CPU buffer portions only. No complete criterion verification or production enablement is claimed. Fresh exact-head review and supervisor serial integration/validation are still required.

Changed paths: Cargo.toml, Cargo.lock, Taskfile.yml, src/lib.rs, src/tests.rs and this HANDOFF.md, all under the owned component. The new lock has the accepted PDF lock's package versions/checksums plus this component, not a dependency upgrade.

## API and execution

Create one CpuBudget per application window and pass clones to all preview producers. Create one Service with Service::start(&trusted_installed_library, budget.clone()). start captures a bounded native PathBuf and starts exactly one named worker. It does not canonicalize, open files or call PDFium on the caller. Both Renderer::load and the actual Renderer::render_file run on that worker. This is a production render implementation, not a dummy renderer. The private RenderCall/start_inner seam permits deterministic unit checks; it is not an exported worker framework or image decoder.

- open_target mints an opaque, process-unique Target. Maximum sixteen live targets.
- request(target, &native_path, Request { page_index, width, height }) returns a fresh opaque Generation only on acceptance. Page indices are zero-based. Dimensions are the caller's physical-pixel box, including display scaling and zoom. It rejects invalid dimensions/indices and checked bounding-box RGBA output above the accepted backend's 32 MiB cap. It conservatively refuses, rather than guessing an unknown page aspect ratio or silently reducing resolution.
- Pending requests coalesce per target without changing its FIFO position. A replacement for an active target joins the queue tail. There are at most sixteen pending jobs, one active job and one completed mailbox per live target. Continually updating one target cannot move it ahead of other queued targets.
- take(target) takes only that target's current completed Response, including native captured path, generation and Result<Frame, Failure>. Its mailboxes never contain a stale publication. Worker publication requires the same still-live target and current generation. Current typed errors obey the same guard as successful frames.
- cancel invalidates the current generation and drops queued/mailbox work. close_target also retires the target. Active OS/native calls may still finish, but their results cannot become current or retarget another pane.
- is_current lets the caller reject an already-taken response after a subsequent request/cancel. Retained frames may still be used as an explicitly old scaled bitmap during a sharper render, not labelled as the latest result.

All ordinary caller routing methods use try_lock, never wait on a worker/state mutex and do no source/DLL I/O or native calls. Contention returns Failure::Busy, leaving state unchanged. Callers must retain their intended action and retry in a later update, not spin on the UI. Rejected requests leave the previous current generation unchanged. Busy from is_current/take is not permission to apply a result. State work scans at most sixteen records and captures paths bounded to 128 KiB of OsStr encoded bytes. No sorting or rendering occurs under the state mutex. Startup thread creation and explicit buffer allocation are not ordinary UI request/cancel/take operations.

The backend is the sole PDFium initializer in the process. Its engine, DLL and bindings live until process exit and serialize all native use. Use the reviewed trusted Windows x64 non-V8/non-XFA PDFium 7881 deployment. The service is not a DLL authenticity check or sandbox. Do not also initialize PDFium through another crate, and do not create a service per pane/tab/query. A failed initial renderer load is retained as an explicit bounded fallback for this service; a repaired installation needs a new service after the previous worker is joined.

## CPU buffer accounting

The fixed shared ceiling is 256 MiB. Admission uses checked arithmetic and a bounded atomic compare/exchange. It returns BudgetExceeded or Busy, never waits for budget and never allocates first to decide whether work fits. Tests fill logical leases, not enormous OOM fixtures.

Phases:

1. Before calling the renderer, reserve SOURCE_RESERVATION of 64 MiB plus 16 KiB, derived as 2 * pdf::MAX_INPUT_BYTES + 16 * 1024, plus checked requested width * height * 4. The output reservation is at most 32 MiB. All already-held Frame/CpuBuffer allocations remain charged in the same CpuBudget, including caller-held frames removed from mailboxes or from closed targets.
2. Accepted read_bounded caps input at 32 MiB and uses a 16 KiB stack chunk. Its exact growth target is capped at 32 MiB. During growth, the prior and replacement Vec allocations may overlap. Short reads allow the prior capacity to be nearly 32 MiB before the final clamp, not at most 16 MiB. Both allocation layouts are bounded individually by MAX_INPUT_BYTES, so 2 * MAX_INPUT_BYTES + 16 KiB covers the full growth peak regardless of the permitted short-read pattern. The new reservation is 67,125,248 bytes, not merely the example's 67,124,224-byte peak. Source bytes live through document/page cleanup. The backend writes directly into one exact-reserved BGRA output, swaps channels in place and makes no second pixel copy. The reservation also covers stable input/output overlap.
3. On render_file return, native page/document/bitmap cleanup and source-buffer destruction have completed. Validate returned dimensions, checked length, native path, page/index/input metadata and Vec capacity against the pre-admitted output allowance. These checks detect a backend contract violation; they are not the primary allocation admission.
4. Shrink the working charge to actual returned Vec capacity, not length. Move that lease and Vec into a private CpuBuffer inside Frame. Frame exposes a borrowed read-only pixel slice, never an owned Vec or an unleased clone.
5. CpuBuffer::drop explicitly destroys its Vec before the lease field subtracts the remaining charge. Stale, cancelled, closed, replaced, error and stopped results follow the same ownership path. Leases are non-cloneable and release exactly once. Caller-held frames retain their charges after service join.

Exact output capacity relies on the accepted backend's Vec<u8>::try_reserve_exact followed by resize within the reservation, and the inspected Rust 1.95 implementation. rustc here is 1.95.0, commit 59807616e1fa2540724bfbac14d7976d7e4a3860. RawVec::grow_exact computes checked len + additional, requests that layout, then set_ptr_and_cap records precisely that cap. Output cannot grow past the requested fitted size in the accepted code. The source similarly uses explicit exact growth, capped before reserve. This accounts allocation layouts/Vec capacity, not allocator bookkeeping or physical heap granularity. A changed backend allocation strategy/toolchain needs renewed capacity accounting, not reliance on the defensive post-render check.

Inspected primary source:
https://raw.githubusercontent.com/rust-lang/rust/59807616e1fa2540724bfbac14d7976d7e4a3860/library/alloc/src/raw_vec/mod.rs
https://raw.githubusercontent.com/rust-lang/rust/59807616e1fa2540724bfbac14d7976d7e4a3860/library/alloc/src/alloc.rs

CpuBudget::allocate(bytes) provides optional concrete, pre-admitted caller copies with private Vec ownership and borrowed mutable/read-only slices. It is not a general allocator. It may allocate/zero significant bytes, so do not call it in an input-critical update. The caller must not make unaccounted to_vec/ColorImage/upload staging/cache copies of borrowed pixels. Actual egui conversion/upload/cache allocations are NOT integrated here and remain a required shared-budget integration gap. Passing separate CpuBudget instances would also violate the window-level contract. The component alone therefore does not establish a whole-window CPU bound.

PDFium internal allocations, total process memory and allocator bookkeeping are outside this buffer ceiling per SPEC. No GPU allocation/cache exists and no 128 MiB GPU claim is made. The unresolved image helper/codec decision is untouched; no image decoding is enabled.

## Metadata and shutdown bounds

Sixteen target slots, sixteen queued jobs, one active job and sixteen completed mailboxes bound service-owned records. Queue paths and returned native paths have a 128 KiB encoded-byte cap; a success retains both Response.path and document.path. Backend fallback strings use a bounded fmt::Write sink with at most 2048 bytes/capacity. Numeric metadata is fixed-size. Paths/queue records/fallback text are metadata, not CPU preview pixel/source buffers, and are bounded separately. Caller-retained Response/Frame metadata count is the caller's responsibility; retire old responses instead of retaining an unbounded history. Pixel allocations remain leased regardless of caller count.

request_stop is a nonblocking atomic signal and notification. After it, take/request/cancel refuse Stopped. The worker discards queued/mailbox work at its next scheduling boundary and never publishes a stopped result. A worker-side 50 ms timed condition wait prevents a lost atomic-stop notification while idle. This is not a wall-clock shutdown deadline.

try_join returns None and keeps its JoinHandle while the worker is running. Once finished it performs the actual join and returns the thread result. join requests stop and blocks until the worker finishes. Drop also joins; it does not detach or silently relinquish ownership. Window integration should signal stop, keep owning the service while polling try_join, and destroy it only after completion. Do not invoke blocking join/drop in an ordinary UI update. Uninterruptible source reads, DLL loads, parse/native calls may delay completion indefinitely. No hard cancellation/deadline claim is made.

## Validation and evidence

Exact checkout prefix for every Cargo/Task/Git command:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport
```

Executed:

```sh
cargo generate-lockfile --offline --manifest-path components/pdf-preview-service/Cargo.toml
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml
cargo fmt --manifest-path components/pdf-preview-service/Cargo.toml -- --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
cargo test --locked --offline --manifest-path components/pdf-preview-service/Cargo.toml --target-dir target/p11
task --taskfile components/pdf-preview-service/Taskfile.yml
task --taskfile components/pdf-preview-service/Taskfile.yml check
task --taskfile components/pdf-preview-service/Taskfile.yml test
task --taskfile components/pdf-preview-service/Taskfile.yml run
git diff --check
git diff --cached --check
```

Formatting and locked offline all-target compilation pass. Final suite: eighteen passing unit checks, zero failed/ignored. No root suite, Clippy, native/DLL render probe, GUI, clipboard, or personal-file check ran. Task run is a safe headless controlled FIFO/routing check, not a preview GUI/native probe. Clean is described and restricted to root target/p11; it was not run.

The checks cover coalesced FIFO fairness; independent target survival; completed/pending/errored stale refusal; active/pending cancel and close; captured ill-formed native UTF-16 path identity without lossy conversion; worker-only initialization/render calls; Busy rather than waiting even with an explicitly held state mutex; idle and blocked-worker stop/join ownership; source/output plus held-frame aggregate accounting; refusal without renderer entry; capacity versus length; lease drop/reuse without underflow; concrete caller copies; target/queue/count/path/output bounds; bounded fallback messages; rejected requests preserving current generation; service recreation IDs; malformed backend-output contract cleanup. The production start/load path reports missing-library fallback from a uniquely created empty disposable directory, removed by its fixture guard. It never loads a DLL or accesses a personal path.

No focused compile/test failed. One attempted shell append hit Bash backtick substitution and appended malformed text; inspection removed it before compilation. An initial lock comparison keyed only by package name falsely suggested a syn change because the accepted lock contains two syn versions. Comparison by name/version confirms this service is the sole new package pair; existing versions/checksums match. A final complete-package-block comparison after CRLF normalization confirms the sole new block is this component. No dependency edit was made to fix those diagnostics.

## Remaining gates

- P11 originally left native actual service rendering pending. P14 above now supplies the explicit public-Service check against the existing managed hash-verified PDFium 7881 artifact and generated owned PDFs. Prior accepted backend evidence alone was not used as a substitute.
- GUI pane/selection/page controls, density/resize/zoom-to-request wiring, fresh assignment, old-bitmap handling, actual CPU conversion/upload/cache admission, GPU texture ownership/128 MiB ceiling, eviction and image coordination remain integration work.
- Keep checkpoint keyboard/accessibility/system-clipboard permission gates, native trust/license/packaging and clean-machine/manual scaling/performance checks. No production preview enablement.
- Proposed status: F08.3/F08.5 remain partial/in progress. This implements and headlessly checks their commissioned background/current-result/CPU-buffer portions, not complete acceptance.
- Component-only terminal commit is ready for fresh review, not self-approved. No next commission until its own review, supervisor serial integration and updated-main validation.
