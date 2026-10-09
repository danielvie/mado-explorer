# P07 raster viewport handoff

## Candidate and boundary

- Checkout: C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport.
- Branch: lane/raster-viewport. Accepted baseline: 36798781b4f9bf8701c54197f90e80d39c1d9cb6.
- Candidate is the component-only commit containing this handoff. Resolve with `cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git rev-parse HEAD`; its exact SHA is also returned to the supervisor.
- Changes are confined to components/raster-viewport/**. No root source, root lockfile, sibling component, production adapter or acceptance tracker edits.
- PROGRESS.md was already modified by the supervisor's commission. It remains unedited and unstaged, separate from component cleanliness.

## Public API and adapter contract

The standalone library depends only on egui =0.36.2, matching the checkout's root Cargo.lock, plus the Rust standard library. Component Cargo.lock is tracked. Its own workspace and target directory keep preparation separate from the explorer.

- `RasterViewport::default()` creates one bounded per-instance view state.
- `RasterViewport::show(&mut self, &mut egui::Ui, id_salt, document_key: u64, &TextureHandle, image_pixels: Vec2, viewport_size: Vec2) -> Option<ViewportOutput>` renders working Fit, 100%, Zoom out and Zoom in buttons, the raster, and a scale/dimensions line.
- `mode() -> ViewMode` exposes Fit or physical-pixel Scale.
- `ViewportOutput` returns the raster `Response`, full unclipped `image_rect` in egui points, and `physical_scale`, display pixels per image pixel. The response rectangle is the actual raster region after available-space reduction.
- Supply a stable unique UI salt per instance and a stable document key. Use a new key for a different selected document or PDF page. Key changes reset Fit/pan and change widget identity, preventing an old document's captured drag from continuing into the replacement.
- Supply positive finite oriented full-image dimensions, not a downsampled texture's dimensions. The caller applies orientation before passing both texture and dimensions. Same-document sharper replacement keeps these dimensions and the key unchanged, preserving scale and normalized pan.
- Supply a live borrowed texture from the same egui context. The caller retains it through rendering. Production code does not create, clone, store, update or release texture handles.
- `viewport_size` requests the raster area's size in logical points. The parent must reserve room for the wrapped controls and status line. Available space can reduce the raster area; invalid/unrepresentable geometry returns None with visible refusal.
- For a later sharper-render request, physical raster size is `output.response.rect.size() * ui.ctx().pixels_per_point()`. At the current scale, full displayed image size is `output.image_rect.size() * pixels_per_point`. This component does not schedule a render or allocate its output.

## UI and state coverage

- Fit preserves aspect ratio, centers the image, and recomputes from the current raster rectangle and display density each frame.
- 100% maps one supplied image pixel to one physical display pixel, including current egui UI zoom and monitor density through pixels_per_point.
- Explicit zoom buttons use 0.8/1.25 factors. Ctrl/command-wheel and native Zoom events zoom around the pointer. Primary-button dragging pans only the captured raster. The painter intersects raster and parent clip rectangles.
- Manual zoom is bounded to 1%-6400%, extended to include an unusual Fit scale. Pan cannot expose empty space along an overflowing axis; non-overflowing axes remain centered.
- Raw unsmoothed pointer zoom events are applied in order at their preceding event-time pointer position, transformed from global to layer-local points. Targeting checks this enabled raster's clipped interaction rectangle and top-most layer at that position, not final hover or final drag capture. Moving from a text control into a raster does not retarget an earlier zoom; moving out does not discard an earlier raster zoom. Outside/text/sibling and invalid events stay unconsumed by this instance. Neighboring instances do not share mode/pan. No explorer shortcuts or text-edit keys are intercepted.
- Captured primary-pan motion and pointer zoom replay together in raw event order. Each motion uses the current drawn size and layer-local displacement, then clamps before the next event. The primary press establishes the baseline; release ends pan before subsequent motion. There is no additional whole-frame pan. Egui grants capture; the event-time press must also be inside this raster's clipped interaction rectangle/top layer.
- Same-key texture replacement and display-density changes preserve manual scale and the image-relative center, subject to bounds. Fit remains responsive to resize. New keys reset the view.
- Zero, negative, nonfinite and unrepresentable dimensions, density and zoom input are refused. State contains document key, mode, pan, previous pointer position and one admitted primary-capture widget ID; no workers, event queue or cache. Stationary batches use the context's known current pointer, including fresh instances and return after hiding. Batches containing PointerMoved, PointerButton or PointerGone still replay from the previous instance pointer and never backfill final position.

## S013-R01 correction and validation

- Fix base: 408951307755e5e512893e18ef6138caeec53558, lane/raster-viewport. Fresh exact-head review is pending. No acceptance/integration/enablement claim.
- Owned changes: src/lib.rs, tests/support/batched_input.rs and this HANDOFF.md, all under components/raster-viewport. Existing src/tests.rs assertions, manifest, lockfile, example and Taskfile are unchanged. Parent-only dirty PROGRESS.md remains unedited and unstaged.
- Installed egui 0.36.2 definitions inspected: Sense::drag, Response drag_started_by/drag_delta/total_drag_delta/hover_pos, PointerState event processing/press_origin/frame delta, Event ordering and Context layer transforms/layer_id_at. Response total_drag_delta already converts global displacement to layer-local units.
- No new retained input state. Zoom processing temporarily takes and restores the existing event vector, retaining untargeted/invalid events. This permits event-position layer queries without nested context locks and avoids another event buffer. Each valid targeted Zoom or Ctrl/command-wheel event applies its own scale limit and pointer anchor before the next event.
- Regression module lives under tests/support and is included only by the library's test build, so existing task test with --lib runs it. Tests inject actual RawInput batches into Context::run_ui with generated textures, no direct pan/zoom calls.

Red check on the original implementation plus new tests:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && cargo test --offline --locked --target-dir target --lib batched_input -- --nocapture
```

Exit 101, all five new checks failed. Exact evidence: no-post-press drag center 230 instead of 150, same-frame post-press drag center 250 instead of 170, normalized zoom anchor 0.55 instead of 0.5, raster zoom ending elsewhere stayed 1x instead of 2x, interleaved limit/zoom stayed 64x instead of 32x. No assertion was removed or weakened.

Final coherent correction checks:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && cargo fmt && cargo fmt --check && cargo check --offline --locked --target-dir target --all-targets && cargo test --offline --locked --target-dir target --lib batched_input -- --nocapture && cargo test --offline --locked --target-dir target --lib
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && task test
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git diff --check
```

Passed formatting and locked offline incremental all-target check; six focused checks pass; all thirteen library checks pass, zero failed/ignored. Task test passes the same thirteen using its existing component target convention. Coverage adds pre-press travel, movement after press in the same frame and incremental continuation, translated/scaled layers, zoom-before-move and move-before-zoom for Zoom and Ctrl-wheel, interleaved anchors/scale limits, event-time raster/text/sibling/outside routing and invalid-event retention. A sixth no-op guard confirms neutral targeted zoom preserves Fit mode. All seven existing focus/keyboard, independent viewport, density/aspect, pan limits, resize, replacement and document-reset tests remain unchanged and pass.

Commit inspection is restricted to the three owned paths:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git diff --check && git add -- components/raster-viewport/src/lib.rs components/raster-viewport/tests/support/batched_input.rs components/raster-viewport/HANDOFF.md && git diff --cached --check && git diff --cached --stat && git diff --cached --name-only && git diff --cached -- components/raster-viewport
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git commit -m "Fix raster viewport batched drag and zoom input" && git rev-parse HEAD && git status --short && git diff --check
```

Diff checks pass. The terminal SHA is reported separately. Native/GPU/Narrator checks remain pending; no root suite, Clippy, source I/O, decoder, cache, clipboard or native probe ran. F07.2/F08.3 remain partial component controls only. Event targeting uses event-time pointer positions with current-frame raster/layer geometry, not an event-time layout replay. Pan remains egui primary-drag response handling, not raw mouse-motion or a kinetic gesture controller. Wait for this lane's fresh review, parent integration/validation, then a new commission against updated main.

## S013-R04 correction and validation

- Frozen fix base: 02d88a8244a1a5c7cd44591b74e34b11a162640a, lane/raster-viewport. The component-only commit containing this section is the next candidate, not an approved integration. Its exact SHA is returned separately and resolves with `git rev-parse HEAD` in this checkout.
- Owned paths: src/lib.rs, tests/support/batched_input.rs, new tests/support/ordered_pan.rs and HANDOFF.md. All thirteen existing behavior/regression functions and assertions remain unchanged. Existing test support gains document keys and a geometry comparison helper. No dependency, manifest, lockfile, Taskfile, src/tests.rs, root or sibling edits.
- Parent-only PROGRESS.md remains unedited and unstaged. Its blob hash is fd9c683c2fc1bf42307928580f08989a1c295364, matching the initial dirty diff's new blob.
- Installed egui 0.36.2 APIs inspected: Context dragged_id/drag_started_id/drag_stopped_id/interaction_snapshot, interaction.rs capture calculation, Response dragged/drag_stopped_by and layer-scaled deltas, PointerState position/button event processing and primary_down, Event definitions, Ui clipped interaction rectangle, Context layer transforms/layer_id_at. Egui does not grant a drag for press-and-release wholly within one batch. The component does not synthesize capture for that uncaptured gesture.
- Pan now replays interleaved with zoom. One retained admitted primary-capture ID prevents a rejected event-time press from becoming a pan in a later held frame merely because egui's batch hit-test ends over the raster. The ID must still be owned or have a primary release reported by egui. Different document/widget IDs, disabled/invalid geometry and lost capture cannot inherit it. Moves before press and after release remain unconsumed and do not pan. Each movement transforms displacement into layer-local points and clamps at its event-time scale.
- Stationary batches use the context's current known pointer. Any PointerMoved, PointerButton or PointerGone keeps the ordered replay's previous-pointer starting rule. Events are taken/restored as before; only targeted valid zoom events are consumed. No public input is discarded or augmented.

Red check before source correction:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && cargo test --offline --locked --target-dir target --lib ordered_pan -- --nocapture
```

The first invocation failed to compile the new test because an Event was moved between loop iterations. Cloning that test value fixed the harness without changing assertions. The next invocation exited 101: four failed, one passed, thirteen filtered out. Held-drag batched image min x was -650 instead of sequential -630. The captured release/ordering test gave -220 instead of -149.99998. Fresh native Zoom and Ctrl-wheel each stayed at 0.375 Fit instead of 0.75. These are failure evidence, not passing checks.

The first source correction passed those five new checks and all thirteen old checks. A stronger same-batch elsewhere-press/next-frame-motion guard then exposed a distinct retargeting failure, image center 170 instead of 150. Retaining the single admitted capture ID fixed it on the first focused correction; all five then passed. A sixth check confirms no final-position backfill across ordered PointerMoved/Button/Gone and no stale fallback after PointerGone. No assertions were removed or weakened.

Final coherent checks:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && cargo fmt && cargo fmt --check && cargo check --offline --locked --target-dir target --all-targets && cargo test --offline --locked --target-dir target --lib ordered_pan -- --nocapture && cargo test --offline --locked --target-dir target && task test
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git diff --check
```

Passed formatting and locked offline incremental all-target compilation. Six focused tests pass; full component suite passes nineteen tests, zero failed/ignored, plus zero doc-tests. Existing task test passes the same nineteen. Added checks cover held-drag batched/sequential equivalence in both zoom/move orders for native Zoom/Ctrl-wheel, translated/scaled layers, new-press and captured-release boundaries, per-move clamping, no double-count on idle continuation, document reset, other-widget/uncaptured motion, fresh stationary zoom/wheel, stale hidden-instance context pointer in both directions, and outside/text/sibling isolation. All thirteen previous behaviors pass unchanged.

Commit inspection and publication to this lane only:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git diff --check && git add -- components/raster-viewport/src/lib.rs components/raster-viewport/tests/support/batched_input.rs components/raster-viewport/tests/support/ordered_pan.rs components/raster-viewport/HANDOFF.md && git diff --cached --check && git diff --cached --stat && git diff --cached --name-only && git diff --cached -- components/raster-viewport
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git commit -m "Preserve captured raster pan and zoom event chronology" && git rev-parse HEAD && git status --short && git hash-object PROGRESS.md && git diff --check
```

F07.2/F08.3 remain partial component controls only. Event-time input uses current-frame layout/layer geometry, not layout history. Fresh instances with ordered position-changing batches cannot infer a missing pre-batch pointer and do not guess from final position. Native/GPU/Narrator/actual monitor checks remain pending. No native/data probe, source I/O, decoder, cache, clipboard, root suite, dependency change, production enablement, merge/rebase/push or nested delegation occurred. This section supersedes the historical R01 whole-frame response-delta pan mechanism. Await fresh exact-head review and parent approval/serial integration/validation before any next feature.

## Original P07 validation commands and outcomes

Every shell invocation used an explicit checkout/component cwd. The final coherent code checks were:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && cargo fmt && cargo fmt --check && cargo check --offline --locked --all-targets && cargo test --offline --locked --lib && task --list && task run && task test
```

Passed formatting, locked incremental all-target compilation, seven headless regression tests, task listing, generated demonstration and task tests. The suite uses generated 8x4 and 16x8 textures with injected real egui UI input. It covers 1x/1.5x/2x physical-pixel Fit/100%, landscape/portrait geometry, controls, pointer anchor, bounds, resize/density changes, document reset during a drag, same-document replacement, separate instances, text-field isolation, clipping, visible scale and focused-button Enter activation. Texture upload deltas are deliberately cleared in the headless harness; no GPU rendering is claimed.

After pinning task output paths, the task contract was checked again:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && task && task run && task test && task clean && test ! -d target && printf 'Component target output removed.\n'
```

Passed. Default lists tasks. Run prints `Generated Fit: 62.50% physical scale, [500.0 250.0] points`. Test passes seven checks. Clean removed only the component target output; the final assertion confirmed its absence. Run/test explicitly use `--target-dir target` and each task has a description.

Bootstrap used `cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/components/raster-viewport && cargo generate-lockfile --offline`. No new direct dependency beyond the required matching egui.

Earlier failed checks were preserved in the session evidence, not counted as passes:

1. Initial all-target compilation failed because egui 0.36 uses AsIdSalt and Context::run_ui. Updated to those installed APIs; next compile passed.
2. Initial tests failed on egui's unapplied texture-delta drop assertion. Explicit headless delta clearing fixed the harness; next run passed three tests and exposed distinct layout/focus failures.
3. Fit assertions showed 134.875-point widths instead of requested 300/200 because the control inherited its parent's horizontal layout. The control now explicitly uses vertical layout. The Enter test incorrectly assumed clicking an egui button grants keyboard focus; it now injects focus before testing real Enter activation. No behavior assertion was removed or weakened. All six then passed, and the added seventh regression also passes.

Diff inspection/check and commit are component-only:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git diff --check && git add -- components/raster-viewport && git diff --cached --check && git diff --cached --stat && git diff --cached -- components/raster-viewport
cd C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport && git commit -m "Add isolated borrowed-texture raster viewport controls" && git rev-parse HEAD && git status --short && git diff --check
```

## Partial acceptance and limits

- F07.2 controls portion only. Fit/100%/zoom/pan and displayed supplied dimensions have headless evidence. Orientation application and source file byte-size display remain adapter-owned. No complete F07.2 claim.
- F08.3 display-scale controls portion only. Geometry and sharper borrowed-texture replacement work. PDF loading, actual page rendering/resolution scheduling and nonblocking replacement are not implemented here. No complete F08.3 claim.
- No image decoding, source access, PDF engine, stale-result routing, texture/cache ownership, memory-budget enforcement or decoder-completion claim. No root runtime enablement.
- Native interaction, actual monitor changes, GPU appearance, narrow-pane visual checks and Narrator remain pending. No personal filesystem, clipboard, elevation, foreground or native-window probe was performed. Root suite and Clippy were not run.

## Smallest serial follow-up

Fresh read-only review of this component first. The supervisor can then add a pane-owned instance and a narrow adapter supplying reviewed borrowed texture, oriented full-image dimensions, stable document/page key and bounded UI allocation. Route only the pane's active selection; pass sharper same-document textures without changing logical dimensions. Keep decoder/PDF work, stale routing and shared CPU/GPU budgets separately gated. Native checkpoint keyboard/accessibility and clipboard probes still precede production preview enablement. No gate is waived by this preparation.
