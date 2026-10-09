# P08 conflict dialog handoff

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
- Opening-frame button/Enter/Space activations and held/repeated or modified decision keys cannot answer a new request. Escape remains an explicit non-destructive Cancel. Widgets use operation/request-scoped egui identities. Keyboard click flags cached by egui are suppressed when processing decision keys.
- Paths use visually truncated, non-selectable labels with full display text on hover. Only display uses `to_string_lossy`; requests and responses preserve `PathBuf`. Tests retain Unicode/long paths, verbatim paths, and unpaired UTF-16. Hover output contains both complete supplied display paths.
- One focused built-in suite has 16 passing headless tests. It covers the controls above, one-shot immutable responses, stale/duplicate and wrong-operation refusal, pending replacement, independent operations, exact-compatible policies, incompatible directory/type conflicts, Cancel/finish cleanup, checkbox reset, held keys, and multi-pass input ownership without response replay.
- Generated fixtures are synthetic paths. No fixture files are opened, created, renamed, transferred, or removed.

## Validation commands and outcomes

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
