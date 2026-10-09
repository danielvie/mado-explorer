# P08 conflict dialog handoff

## S013-R03 bounded presentation IDs and request-epoch input

- Baseline `d548665cdb85208f14312e1e76517a196242d228`, branch `lane/conflict-dialog`, checkout `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`. This component-only commit is the new terminal candidate, not an approved P08. Resolve its SHA with the candidate command below.
- Exclusive changed paths: `components/conflict-dialog/src/lib.rs`, `components/conflict-dialog/tests/retained_areas/mod.rs`, and this document. Original `src/tests.rs` and all sixteen behavior checks remain unchanged. Supervisor-owned dirty `PROGRESS.md` remains unchanged and unstaged. No root, sibling, manifest, lock, dependency, Taskfile, executor, filesystem, clipboard, native probe or enablement changes.
- R02 bounded only the Modal Area. Its request-specific child IDs still leaked retained tooltip Areas and focus-cache entries. Installed egui `0.36.2` evidence under `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.36.2/src/`: `containers/tooltip.rs:133-143,210-212` derives tooltip IDs from widget IDs; `containers/popup.rs:580-604` creates their Areas; `memory/mod.rs:1236-1241,1358-1405` retains hidden Areas/layer order. `memory/mod.rs:647-658` inserts interested focus IDs, while `732-739` prunes only during cardinal navigation. Pointer/Enter usage did not prune the old request IDs.
- Authorized API/mechanism tradeoff: one fixed Modal/body/checkbox/action namespace per window Context, shared across requests and operation controllers. `UiBuilder::id` provides explicit parent-independent child scopes. Path widget IDs and both tooltip IDs stay fixed too. Operation/conflict IDs remain immutable request DATA and still pass through `decide`'s current-ID/type check. No per-operation IDs, ring, global-memory clearing, forced cardinal navigation or dependency patch. Draw exactly one conflict presenter per window, as before.
- Each new `Pending` request starts a blocked primary-pointer epoch. It becomes ready only after the opening frame and an observed up state. Only a new primary press while armed can authorize a later release. An old or opening-frame press remains blocked through later empty frames until release. Invalid release batches cannot answer or change apply policy and restore Skip focus. Mixed batches containing an inherited release are conservatively refused. Pointer events are processed once per frame during layout retries.
- Fresh frames refuse all decisions and reset checkbox/default Skip focus. Opening-frame Tab/arrow navigation is stopped with `move_focus(None)` only while this presenter is the top modal. This cancels current modal navigation before egui can schedule next-frame inherited focus; it neither prunes caches nor navigates another window/control. Later Tab/Shift+Tab remains native. Enter always skips, focused Space works, Escape cancels, and modified/repeated decision keys remain refused. Installed `input_state/mod.rs:401-412` marks already-held key presses as repeats before presentation. Native cached keyboard clicks never authorize a choice; pointer authorization uses `clicked_by(Primary)`. The checkbox's native temporary toggle never changes operation policy without the same epoch checks. Answer-frame ownership and one-shot immutable response routing are unchanged.

### Regression evidence

- Replaced R02's mechanism-only assertions requiring different widget IDs with bounded ID cardinality plus stronger input checks. Requiring new IDs was the leaking mechanism, not a product requirement. No original behavior test or failed behavior assertion was deleted or weakened.
- Actual hovered-tooltip test renders both unique full source/destination paths for every one of 384 requests across twelve controllers, alternating file/folder conflicts. After hiding each request for two frames, it compares retained `Memory::layer_ids` and `AreaState::load` sets. Both tooltip Areas remain present but the set stays equal to the first request. This is not a visible-layer count.
- Focus test traverses 768 requests across twelve controllers with actual pointer clicks and Enter, collecting the five action/checkbox response IDs on every request. The own focusable control set remains exactly five without cardinal navigation or cache clearing. Installed focus-cache insertion consumes those same response IDs. It does not claim access to egui's private cache map or a process-memory benchmark.
- Delayed-release test presses each of the four actions and the checkbox, replaces the request or controller, renders three fresh/idle frames, then releases. No decision, checkbox change or inherited Replace/checkbox focus occurs. A new intentional click works for all controls, and subsequent focused Space works. Opening-frame pointer/Space, held-key/default-focus, and deferred Tab/Shift+Tab focus have separate checks. A forced two-pass pointer-checkbox frame toggles exactly once and preserves apply policy.
- RED before production changes: `cargo test --locked --offline --lib retained_areas --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target` exited 101. Four tests failed: tooltip retained set grew at operation 1/conflict 2, focusable set grew to ten, delayed old release lost default Skip focus, and opening pointer/Space failed to produce the later intentional Skip. Sixteen original tests were filtered, not changed.
- While inspecting fixed-ID focus, added `opening_focus_navigation_cannot_schedule_inherited_focus`. Its focused run exited 101 at the next-frame Skip-focus assertion before the navigation guard. One focused correction passed. The two-pass checkbox check passed on its first run and is not claimed as red evidence.
- Initial test compile used the unavailable `Context::style_mut`. Read installed API and corrected the fixture to `global_style_mut` before recording the behavioral red run. No source behavior/assertion workaround or tool/model fallback.

### Final validation

Commands run from `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog`, using the existing component target:

```text
cargo fmt
cargo fmt --check
cargo check --locked --offline --all-targets --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
cargo test --locked --offline --lib retained_areas --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
cargo test --locked --offline --all-targets --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
```

Final run: formatting and locked offline all-target compile passed. Focused suite: six passed, sixteen filtered, zero failed/ignored. All-target suite: twenty-two passed, zero failed/ignored, example test target also passed. `git diff --check` and `git diff --cached --check` passed from the explicit checkout root. Complete owned delta inspected; `git diff -- components/conflict-dialog/src/tests.rs` is empty. Root acceptance remains supervisor-owned. No native GUI, scaling, Narrator, filesystem or clipboard probe ran. F10.4/F10.6 remain partial decision-control preparation. Fresh independent review, parent serial integration/validation, checkpoint gates and supervisor approval are required before production enablement.

## S013-R02 retained modal state correction, historical

R03 above supersedes R02's request-specific child identity contract. The prior checks/results below describe the original correction, not current acceptance.

- Fix baseline: `83251c0dee036dc5f5efecb9a9e07bac2930af48`, exact checkout and branch below. The frozen correction is the component-only commit containing this section; resolve with the candidate command below.
- Write boundary for this correction: `components/conflict-dialog/src/lib.rs`, `components/conflict-dialog/tests/**`, and this document only. Existing `src/tests.rs` and its sixteen assertions/tests are unchanged. Supervisor-owned dirty `PROGRESS.md` remains untouched and unstaged. No manifest, lock, Taskfile, root/main/sibling source, executor, filesystem, clipboard or enablement changes.
- Source finding: baseline `src/lib.rs:283-284` derived the Modal Area/Layer ID from operation and conflict. `finish` cleared Rust request/policy only. Installed `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.36.2/src/memory/mod.rs:1236-1241` inserts each Area and appends each new layer. Its `end_pass:1358-1405` rebuilds order bookkeeping without pruning hidden areas/layers. `containers/modal.rs` passes the modal ID to its foreground Area. Changing only to an operation-specific ID would still leak across controllers.
- Fix: one `mado-conflict-presenter` Area/Layer identity per window Context, reused across sequential requests and new operation controllers. A `push_id` child scope retains operation/conflict-specific button and checkbox identities. First-frame gating now covers Escape explicitly too, because a reused layer can already be topmost when the request changes. Fresh/repeated/modified keys and stale pointer releases remain refused; Enter Skip, focused Space, later Escape Cancel, exact-type operation policy, file-only Replace and answer-frame input ownership remain intact.
- Host contract: draw at most one conflict presenter in a window Context at a time. F12.3 supplies one active mutating operation. Keep other controllers as operation state if needed, but never draw two conflict presenters concurrently. The existing top-modal-layer guard remains for other modal kinds; this fix adds no presenter manager and does not permit background input through another modal.
- New regression: `tests/retained_areas/mod.rs`, included in the existing `--lib` suite. It exercises 384 requests, 32 increasing conflict IDs in each of twelve newly created operation controllers, sharing one Context. It hides each request for two frames and reads actual retained `Memory::layer_ids` and `AreaState::load`, not visible-layer counts. Retained IDs must stay equal to the first-request baseline across requests, finish and controller retirement. Each corresponding action and checkbox ID must change between requests and between operations.
- Red evidence before production edits: focused command below exited 101 at operation 1 / conflict 2. Both retained layers and retained areas grew from background plus conflict 1 to background plus conflicts 1 and 2. Sixteen existing tests were filtered, not changed.
- First coherent fixed-suite run: retained-state regression passed, but existing `tests::repeated_or_modified_keys_do_not_answer_later_requests` failed at `src/tests.rs:538`, opening-frame decision refusal. Reused topmost layers exposed the old Escape-before-first-frame ordering. Moving the fresh/top guard ahead of Escape corrected it. No assertions were removed or weakened; the next focused and complete runs passed.

Commands ran with explicit component cwd:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog
cargo test --locked --offline --lib retained_areas_and_layers_are_bounded_across_requests_and_controllers --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
RED before production edits, exit 101; final PASS, 1 passed / 16 filtered
cargo fmt
PASS, formatted only owned changed source/tests; original src/tests.rs unchanged
cargo fmt --check
PASS
cargo check --locked --offline --all-targets --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
PASS, incremental
cargo test --locked --offline --lib --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
PASS, 17 passed / 0 failed / 0 ignored, including all original sixteen tests
cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog
git diff --check
git diff --cached --check
PASS; complete owned delta inspected before commit
```

No native GUI or data probes ran. This correction addresses the review blocker only; F10.4/F10.6 remain partial decision-control preparation, not accepted application behavior. Integration, production enablement, native interaction, scaling and Narrator remain pending. Ready for fresh independent exact-head review before any supervisor integration.

## Candidate and ownership

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`
- Branch: `lane/conflict-dialog`; accepted baseline `36798781b4f9bf8701c54197f90e80d39c1d9cb6`.
- Candidate is the component-only commit containing this document. Resolve its SHA with `cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog && git log -1 --format=%H -- components/conflict-dialog`.
- Exclusive changes: `components/conflict-dialog/**`. Root `PROGRESS.md` was already modified by the supervisor. It is untouched and must remain unstaged. No main/root/sibling source or acceptance tracker changes.
- This is isolated preparation for the decision portions of F10.4/F10.6. Neither criterion is complete, accepted, runtime-verified, or production-enabled by this component.

## Public API and host contract

The standalone library is `mado_conflict_dialog`, with egui exactly `0.36.2`, matching the checkout root lock. All 72 dependency versions also match that lock. No other direct dependency.

- `OperationId(u64)` is supplied by the host and must never be reused across operations in the window. `ConflictId(u64)` must increase strictly within its operation, including new conflicts discovered during revalidation. The controller rejects duplicate/older IDs, changed payloads using those IDs, and another operation's IDs.
- `ConflictRequest::new(operation, conflict, source: PathBuf, destination: PathBuf, kind)` requires absolute native paths. Request fields are private; getters expose IDs, full native paths, and `ConflictType`. This constructor does not inspect disk, normalize paths, identify entries, or verify the supplied type.
- `ConflictType { source: EntryKind, destination: EntryKind }` uses `RegularFile`, `Directory`, or `Other`. Only a regular-file/regular-file pair enables Replace. Compatibility is exact equality of both supplied entry kinds. Unknown/link/special kinds must not be labelled regular files.
- `ConflictDialog::new(operation)` owns one operation for its lifetime. It retains at most one pending request and one policy, plus scalar routing/input state. No queue, executor, handles, FFI, clipboard, filename generation, persistence, or global preference.
- `present(request) -> Result<Option<Decision>, Refusal>` is called once per new request. A new request invalidates the previous pending UI and resets its checkbox/focus. An exact-compatible apply policy returns a new response for the new immutable request immediately. Otherwise the modal awaits a choice. Incompatible conflicts never receive a retained Replace response. Requests and their display strings are not byte-capped or shortened in state.
- `show(&egui::Context) -> DialogOutput` draws the actual egui modal, path labels, checkbox, and four buttons. `decision` is emitted once. `owns_input` stays true throughout the answer frame, including repeated egui layout passes. It is false on later frames without a pending request.
- The host must check pending/modal ownership before background shortcut dispatch and continue suppression through the answer frame. Modal backdrop clicks do not answer. Draw this controller only for the operation currently waiting for a conflict decision.
- `decide(operation, conflict, choice, apply)` provides the same ID/type-checked choice path for future window-owned controller wiring. Rejected stale/wrong-operation/Replace inputs leave the current pending request intact. It does not require that a graphical frame has been displayed.
- `Decision` has private fields and getters for the immutable request, `Choice`, `DecisionOrigin`, and apply flag. `matches(request)` checks the entire supplied snapshot, including native paths and type. A decision is a user choice, not validation or authority to execute a filesystem operation.
- `finish()` discards the pending request and policy and permanently ends that controller. Cancel also ends it, clears its policy, and never applies to remaining conflicts. A manual choice without the checkbox clears an earlier policy. Controllers cannot be reset to another operation.

## Controls and headless coverage

- Skip, Keep both, Replace, Cancel are actual buttons with tested pointer activation. Replace is visibly disabled for folders, mixed types, and Other entries, and also refused by the controller.
- Enter always chooses Skip, even with Replace focused. Tab/Shift+Tab use egui focus; Space activates the focused action or toggles the focused checkbox. Escape explicitly emits Cancel for this operation. Default focus is Skip once egui completes its initial modal sizing pass.
- Opening-frame button/Enter/Space/Escape activations and held/repeated or modified decision keys cannot answer a new request. After the first frame, Escape is an explicit non-destructive Cancel. Widgets use fixed bounded egui presentation identities within one window-scoped modal Area. Per-request input epochs prevent inherited gestures; immutable operation/conflict IDs route decisions only. Keyboard click flags cached by egui are suppressed when processing decision keys.
- Paths use visually truncated, non-selectable labels with full display text on hover. Only display uses `to_string_lossy`; requests and responses preserve `PathBuf`. Tests retain Unicode/long paths, verbatim paths, and unpaired UTF-16. Hover output contains both complete supplied display paths.
- The current built-in suite has 22 passing headless tests: sixteen unchanged behavior checks and six bounded-state/input regressions. It covers the controls above, one-shot immutable responses, stale/duplicate and wrong-operation refusal, pending replacement, independent operations, exact-compatible policies, incompatible directory/type conflicts, Cancel/finish cleanup, checkbox reset, held keys, and multi-pass input ownership without response replay.
- Generated fixtures are synthetic paths. No fixture files are opened, created, renamed, transferred, or removed.

## Original P08 validation commands and outcomes

All commands ran with explicit cwd. Build output stayed inside the component's `target` directory. No root suite or Clippy ran.

Final focused checks:

```text
cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog && cargo fmt --check
PASS

cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog && cargo check --locked --offline --all-targets --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
PASS, incremental all-target compile

cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog && cargo test --locked --offline --lib --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
PASS, 16 passed / 0 failed / 0 ignored

cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog && cargo run --locked --offline --example generated --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target
PASS, generated modal, 21 shapes, operation 1 / conflict 1 / Skip; no file operations

cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog && task --list && task && task run && task --dry clean
PASS, task descriptions/default listing, generated headless run; clean dry-run targets only this component's absolute target directory

cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog && task check && task test
PASS, formatting / locked offline all-target compile / 16 focused headless tests

cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog && git diff --check && git diff --cached --check
PASS after staging only the component; staged diff inspected before commit
```

A read-only Node comparison parsed package name/version pairs from both locks. All 72 component dependency pairs exist in the checkout root lock.

Bootstrap and corrected failures, not counted as passing checks:

- Copied this checkout's root lock into the new component. First `cargo check --offline --all-targets --target-dir C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/conflict-dialog/target` refused an `accesskit` feature that egui does not expose. Removed that mistaken feature. AccessKit is already a required egui dependency.
- Next all-target compile found `Context::run` unavailable in installed egui 0.36.2. Changed headless hosts to `run_ui` using the supplied Ui's context.
- First test run hit egui's unapplied texture-delta drop check. Headless tests now explicitly clear generated texture deltas, and the demonstration explicitly discards them. No test assertion was removed.
- Next test run exposed two missing default-focus assertions. egui reports its top modal layer from the previous pass; the controller now sets Skip focus once the modal becomes current.
- Added held-key and multi-pass tests. Their first run exposed cached keyboard-click activation and a fixture that pressed Enter again without releasing it. The controller now ignores cached native keyboard clicks and accepts only fresh unmodified decision presses. Fixtures explicitly release keys before a new intentional press and establish a held key before simulating a repeat. Final 16 tests pass with their behavior assertions intact.

`task clean` was inspected via dry-run, not executed. Deleting and rebuilding a healthy target was unnecessary. Task commands pin their working directory and generated target root; clean names that same absolute output directory.

## Limits and serial follow-up

No native window, foreground probe, clipboard, elevation, personal filesystem, or production adapter was used. Native visual interaction, display scaling, Narrator, host focus restoration, and queue integration remain pending. Headless tests are not runtime or full-criterion verification.

The smallest serial follow-up is a reviewed window-owned queue/UI bridge: feed immutable requests with unique operation and increasing conflict IDs, retain this controller after an originating tab/pane closes, pause the worker for a decision, match returned IDs/snapshot against the worker's current waiting request, and resume only that request. Route Cancel to cooperative cancellation rather than reporting completed cancellation. On completion call `finish` and retire the controller. Gate background shortcuts through the modal and answer frame.

Filesystem validation/execution must remain separate. Before any eventual Replace, revalidate both entries and their identity/type; if the destination changed, issue a fresh conflict ID rather than reuse an old response. Keep both naming and race-safe commit are not implemented here. Independent review, checkpoint probes, later integration tests, and supervisor approval remain required before production enablement.
