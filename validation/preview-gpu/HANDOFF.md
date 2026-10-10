# P23 generated-pixel GPU validation

## Frozen candidate and scope

- Final source/evidence commit `f0ccb29fb0e0dce7a8e1ce2ce8a20fdf376be14f`, from exact clean baseline `3ff79a5b894cf51aec7b15d9819255b7871cc8a9`.
- First successful source/run is preserved at `c372fa8ccb6781a33fcf4080b01af29519fae22d`. Final source tightens exact controlled-rejection assertions, observes native device epoch and proves control-reservation release. Both actual native transcripts are retained.
- Branch `lane/preview-gpu-probe`, checkout `C:/SANDBOX/WORKTREES/mado-explorer/preview-gpu-probe`.
- This handoff is a documentation-only follow-up. The lane receipt supplies its final HEAD for fresh review. Source and native evidence remain byte-identical to the source/evidence commit.
- Only `validation/preview-gpu/**` changed. No root application, component, root Cargo/Taskfile, tracker, design, dependency upgrade or gate changes.
- Validation candidate only. No F07/F08 acceptance, Root activation, bridge approval, licensing or packaging claim. Parent owns independent review, decisions and integration.

Files are rooted at `C:/SANDBOX/WORKTREES/mado-explorer/preview-gpu-probe/validation/preview-gpu/`:

- `Cargo.toml`, tracked `Cargo.lock`, `Taskfile.yml`.
- `src/lib.rs` contains checked sizes, one controlled-payload ledger and four pure checks.
- `src/main.rs` contains the fixed generated-only native batch.
- `evidence/checks.txt`, `evidence/native-01.txt`, `evidence/native-02.txt` retain command results and complete native stdout/stderr, including both controlled rejections. `native-02` corresponds to final source hashes; `native-01` remains unchanged historical evidence.
- `HANDOFF.md` is this report.

## Commands and outcomes

Run from the checkout root:

```text
cargo generate-lockfile --offline --manifest-path validation/preview-gpu/Cargo.toml
cargo fmt --manifest-path validation/preview-gpu/Cargo.toml
cargo fmt --manifest-path validation/preview-gpu/Cargo.toml -- --check
cargo check --locked --offline --manifest-path validation/preview-gpu/Cargo.toml --target-dir target/p23/build --all-targets
cargo test --locked --offline --manifest-path validation/preview-gpu/Cargo.toml --target-dir target/p23/build --lib
cargo build --locked --offline --manifest-path validation/preview-gpu/Cargo.toml --target-dir target/p23/build
task -d validation/preview-gpu --list
task -d validation/preview-gpu check
task -d validation/preview-gpu test
task -d validation/preview-gpu run
task -d validation/preview-gpu --dry clean
git diff --check
```

All passed. Pure tests: 4 passed, 0 failed, 0 ignored, rerun after the source tightening. Native task: two actual runs, both exit 0. The final run includes strict expected-error classification, a main-device loss callback that invalidates the epoch, and an observed final CPU counter of zero. No GPU executes in `test`, `check` or default task listing. Running `target/p23/build/debug/mado-preview-gpu-probe.exe` without arguments exited 2 as required, before GPU setup. `run` and `native` explicitly opt in with `--native-owned`.

A preliminary root-only lock-version comparator exited 1 for `libloading 0.9.0`. Diagnosis: that exact pin is already in the unchanged accepted PDF budget-service dependency, not the root lock. Comparing the root plus accepted service locks found no different common-package versions. Direct pins remain installed `wgpu 30.0.1`, `egui-wgpu 0.36.2`, `epaint 0.36.2`. PDF crates compile solely to reuse the accepted budget; no PDFium load, PDF render, image helper or decoder executes.

Build output is local ignored `target/p23/build`; `git check-ignore target/p23/build` passed. Clean was dry-run only, never executed. Its three commands clean dev/release/doc output under this build directory, not the whole target or retained evidence/fixtures.

## Actual native observations

Machine `DANIEL-PC`, Windows x86_64. Rust `1.95.0`, Cargo `1.95.0`, Task `3.50.0`. Adapter `NVIDIA GeForce RTX 4070 SUPER`, DX12, vendor 4318, device 10115, PCI `0000:01:00.0`, driver `32.0.15.9186`. Debug build. No window, surface, desktop, clipboard, filesystem pixel fixture, network, elevation, external decoder or custom shader was used. Rendering uses the installed normal egui renderer and a normal textured quad.

| Case | Evidence |
| --- | --- |
| Generated upload | Actual 7x3 RGBA source with alpha 0/1/127/128/254/255. Scalar `Color32::from_rgba_unmultiplied(...).to_array()` writes directly into mapped `MAP_WRITE | COPY_SRC`, `mapped_at_creation=false`. No preview-pixel `Vec`, `ColorImage` or `queue.write_texture`. |
| Alignment/readback | Row width 28, stride 256, staging 768 bytes. GPU mirrors the actual staging, including every row's 0xA5 padding, and copies texture bytes into the other half of a charged 1536-byte readback. All 21 pixels equal the scalar reference; staging padding stays 0xA5 and untouched texture-readback padding stays zero. |
| Upload acceptance | Same submitting worker's Validation/Internal/OutOfMemory scopes cover create/map/encode/finish/submit, reverse-pop immediately, all clean. Actual completion callback and successful indexed native poll precede ID registration and Ready. Renderer creation/registration scopes also clean. The main device has epoch 1; its short native lost callback invalidates that epoch to 2. Ready checks the observed epoch after completion/registration. No real main-device loss was induced, so the invalidation predicate's negative cases remain pure checks, not hardware-loss recovery. |
| Positive renderer control | Registered `User(0)` mesh changes the offscreen target from its exact clear bytes. Successful real submission/fence/readback establishes that the mesh samples before withdrawal. |
| Retirement | A second actual normal render is submitted. Its callback was still pending at withdrawal in this run. Before any poll/wait, `free_texture(User(0))` removes the binding but the texture and all charge remain. A replacement registers `User(1)`; current plus retiring payload is charged together. Old `User(0)` meshes encoded after withdrawal render only exact clear bytes. Both actual callbacks complete before destroy/refund. No fixed sleep is used as completion proof. |
| Native texture ownership | A further valid GPU COPY_SRC readback after unregister equals all original scalar pixels. This demonstrates that native `free_texture` did not destroy the application texture. Only then does explicit app destruction precede ledger refund. |
| Direct discard | Own encoder records a small buffer copy and a deferred callback, then is dropped before finish/submit. Callback remains false after native poll. Known non-submission follows direct ownership, not an inferred successful callback or guard drop. |
| Controlled submit rejection | Own valid 16-byte copy is finished; its own source buffer is deliberately destroyed before submit. Validation scope reports exact `Queue::submit` DestroyedResource error; final code requires this error and no Internal/OOM error, rather than accepting any failure. Deferred callback actually fires anyway. `clean=false` prevents Ready despite that callback. Charges remain until a separate clean empty submission and real drain completes. Not OOM, device abuse or hardware failure. |
| Final cleanup | A final clean actual queue fence completes before renderer/buffers/textures/source are freed. Counters first return to GPU 0 and CPU control-only 1,048,576; after dropping the outer owner, a surviving budget observer verifies CPU 0, including release of that nonallocating reservation. |
| Explicit separate-device destroy | A separate owned device with no pixel resources is deliberately destroyed. Actual lost callback reports `Destroyed`, and actual poll returns `Ok(QueueEmpty)`. Epoch/alive refusal is pure injection. This is not genuine hardware loss or proof of its panic/recovery behavior. |

The native transcript contains the expected rejection's full error source/description and the exact destroy poll outcome. No unexpected native failure occurred. If the main device instead errors, times out or panics, the outer owner is retained and reports its charges until process teardown, with no Ready/refund certificate. That failure branch is source-preserved, not dynamically exercised by a fabricated hardware-loss test.

## Capacity accounting and limits

ONE actual accepted `CpuBudget` handles source, external staging/readback/scratch capacity and a nonallocating 1 MiB fixed-batch control allowance. No dummy helper-sized allocation and no second CPU budget. Source 84 + staging 768 + upload readback 1536 + renderer readback 2048 + scratch buffers 32 = 4468 explicitly owned payload bytes. Maximum admitted CPU charge is 1,053,044 bytes, including the control allowance. Source remains leased through conversion and all queued use. Buffer destruction and handle drop precede reservation refund.

The one GPU ledger charges old texture 84 + replacement 84 + offscreen target 512 = maximum 680 bytes. It retains active and retiring resources together and releases only after the explicit known-clean completion sequence. Native factory bounds are eight buffers and four texture slots; this batch creates five buffers and three textures. Pure checks reach the full 256 MiB CPU and 128 MiB GPU counters using nonallocating reservations, reject exhaustion/overflow before allocation, and prove shared-clone/source/staging/readback/current/retiring conservation.

These are controlled application capacities, NOT physical driver heap caps, total process memory caps, allocator telemetry or an integrated service/window budget. The normal renderer's fixed quad geometry/uniform writes use its installed internal `write_buffer` path; no preview pixels use that path. Driver allocations, renderer pipelines/internal GPU buffers and backend implementation overhead are outside the texture-payload ledger. The 1 MiB control allowance is not a measurement of driver internals. Future P21 helper128/control8/output32 reservations must share the same window budget with these staging/readback/output charges; P21 was not executed here.

## Proven ordering and remaining native UI gap

The smallest policy supported by this offscreen native evidence is:

1. Submit own upload with same-thread scopes; require clean scope results, actual completion and a valid owned-device epoch before publishing a newly registered native ID.
2. Permanently withdraw and unregister an old ID BEFORE future renderer encoding. Do not republish or update that numeric ID to a new texture. Renderer recreation requires a separate epoch; this run proves only successive IDs in one renderer.
3. Keep the app texture and full active-plus-retiring charge until actual prior queue work completes. Unregister alone is not a completion proof. Then destroy and refund.
4. Directly owned unsubmitted encoders may be discarded. A rejected submit's deferred callback is not acceptance; drain/reconcile explicitly, never mark Ready from the callback alone.

Actual eframe hidden logic, egui multipass and close/asynchronous UI drain were NOT exercised. This is a real remaining proof gap, not a passing manual check, an adapter failure or a claim that the installed APIs cannot establish it. This probe owns its encoder ordering and queue, unlike an eframe app. Console main joining its one fixed native worker is not a demonstration of nonblocking UI close.

Read-only installed source supports why the gap matters:

- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-wgpu-0.36.2/src/renderer.rs`, 537-562 skips missing native IDs, 755-760 removes native binding without destroying its app texture, 769-875 registers native Rgba8Unorm with `texture=None`.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/eframe-0.36.2/src/native/wgpu_integration.rs`, 700-755 runs hidden root logic-only and returns without paint; 762-848 updates logic/UI before visible paint. An ordinary logic tick is not a host-submit acknowledgement.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/api/queue.rs`, 291-309 executes deferred actions even when submit rejects, now observed natively.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-30.0.1/src/backend/wgpu_core.rs`, 1916-1927 can take a fatal path on device-loss poll. This run's explicit destroy did not take that path.

Before Root adapter commission/activation, parent should consume a fresh resource/API review and decide the remaining own-window proof task. It must establish real hidden logic-only/multipass withdrawal before host encoding, stale shapes that cannot regain an old binding, and close that requests asynchronous drain and polls completion without a live UI join. Preserve charge across unknown host submission/loss/teardown outcomes. No required UI proof was replaced with a fake callback or ignored failure.

## Evidence identity

The committed source and evidence bytes were compared byte-for-byte with the working files before the source/evidence commit. Native transcript records executable and source SHA256 before the run.

```text
src/lib.rs           324599e7766f5c6c9caab905932b1cdba8a8a7da4b8f761054bc3aef611b7c14
src/main.rs          6af5a29be84f7ecdeb54f3655cff4f11c030e2068d9a4c359c33348c6873364c
Cargo.toml           6a43971e3f7969531fd7976ca95e3beeb27a0c6085f97ec990713d67fc4ef2b2
Cargo.lock           fa2e4f749cf9b52a7064999ee8bc94ab37bc3c951455dce32a2031fab04f2ae8
final executable     bd612ac848b613b4432056431f48c6ef22770e33bf31743cfc17ad92e55536ce
evidence/native-01   eec37554fa91c945971cc3e2d24f7607cebdfa26556146eaad98e4fca6cc5337
evidence/native-02   d10fa98e1568e1b2a2d7df135f7d3eb70376603a5fe1f8dc639d5cece352ffda
evidence/checks      87143ae1f21488edd6ae3a350e403c0a81fa07047f51336a484434febfe44eea
```
