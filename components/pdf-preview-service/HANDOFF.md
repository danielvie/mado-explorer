# P11 PDF preview service handoff

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

- Native actual service rendering against the existing managed hash-verified PDFium 7881 artifact and generated owned PDFs is explicitly pending, not passed. Prior accepted backend evidence is not a substitute for this service probe.
- GUI pane/selection/page controls, density/resize/zoom-to-request wiring, fresh assignment, old-bitmap handling, actual CPU conversion/upload/cache admission, GPU texture ownership/128 MiB ceiling, eviction and image coordination remain integration work.
- Keep checkpoint keyboard/accessibility/system-clipboard permission gates, native trust/license/packaging and clean-machine/manual scaling/performance checks. No production preview enablement.
- Proposed status: F08.3/F08.5 remain partial/in progress. This implements and headlessly checks their commissioned background/current-result/CPU-buffer portions, not complete acceptance.
- Component-only terminal commit is ready for fresh review, not self-approved. No next commission until its own review, supervisor serial integration and updated-main validation.
