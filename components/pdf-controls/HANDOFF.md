# P10 PDF metadata and page controls

## Candidate boundary

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`.
- Branch: `lane/conflict-dialog`.
- Parent baseline: `a47fb01483679890aef367ebb7feb0e60b9ee051`.
- Writes only `components/pdf-controls/**`. No root, shared or sibling changes.
- Fresh exact-head review and supervisor integration/validation required. No acceptance statuses or production gates changed.

## Behavior and partial acceptance IDs

F08.1 metadata/control portion: working pane-local egui filename/full-path labels with complete hover text, supplied page count, requested page and separate confirmed displayed page. Counts are unavailable during loading, missing-renderer, malformed, password-required and supplied failure states. Zero and counts above `u32::MAX` become explicit invalid-metadata failures, not zero-page documents.

F08.2 UI/state portion: actual Previous/Next buttons, editable one-based page number, Go and focused Enter/Space activation. Requests use checked zero-based `u32` indexes. Invalid, empty, overflowed and out-of-bounds input stays in the editor with an explanation. Input is not silently truncated to a different valid decimal command. Selection changes reset to page one. A smaller same-document count resets an out-of-range requested page to page one; an in-range page remains requested. Edited drafts survive metadata changes and automatic requests whether submitted, valid or invalid. A dirty draft is tracked separately from the requested page. Metadata updates explain changed bounds for an invalid draft without replacing its text, cursor/selection or undo history. Successful explicit Go, Enter, Previous, Next and direct `request_page` calls intentionally commit the editor to the requested page and clear the dirty/error flags; rejected submissions retain the draft. A different document resets the editor and its history.

Fallback presentation overlaps F08.4 only as supplied-state UI, not engine verification. No complete F08.1/F08.2/F08.4 acceptance claim. No parser, file reads, DLL loading, PDF rendering, textures, cache, threads, clipboard, native probes or application enablement.

## Exact public API

All types are in `mado_pdf_controls`; request/target/key fields are private and exposed by read-only getters.

- `DocumentKey::new(path: PathBuf, revision: u64) -> Result<DocumentKey, Refusal>`; `path() -> &Path`, `revision() -> u64`.
- `DocumentState::{Loading, Available { page_count: u64 }, MissingRenderer, Malformed, PasswordRequired, Failure(String)}`.
- `PdfControls::new(pane_salt: egui::Id)`.
- `select(Option<DocumentKey>) -> Result<(), Refusal>`.
- `supply_metadata(&DocumentKey, revision: u64, DocumentState) -> Result<(), Refusal>`.
- `show(&mut egui::Ui) -> ControlsOutput`.
- `ControlsOutput { request: Option<TaggedPageRequest>, owns_keyboard: bool }`.
- `target() -> Option<PageTarget>`; `request_page(&PageTarget, PageAction) -> Result<TaggedPageRequest, Refusal>`.
- `PageAction::{Previous, Next, Direct(u64)}`. `Direct` accepts a one-based UI number.
- `PageTarget` getters: `pane() -> egui::Id`, `document() -> &DocumentKey`, `metadata_revision() -> u64`, `page_revision() -> u64`.
- `TaggedPageRequest` getters: `target() -> &PageTarget`, `sequence() -> u64`, `page_index() -> u32`.
- `supply_render(&TaggedPageRequest, RenderResult) -> Result<(), Refusal>`.
- `RenderResult::{Displayed, Failure(String)}`. `Displayed` means the host has actually installed the matching frame, not merely finished a worker request.
- Inspection: `document()`, `state()`, `page_count() -> Option<u32>`, `requested_page() -> u32`, `confirmed_page() -> Option<u32>`, `pending() -> Option<&TaggedPageRequest>`, `editor() -> &str`, `editor_error() -> Option<&str>`.
- `Refusal::{FullPathRequired, StaleDocument, StaleMetadata, StaleAction, MetadataUnavailable, InvalidPage, RequestSequenceExhausted, StaleRender}`.

The first `show` after valid supplied metadata emits one request for the requested page, initially index zero. Each newer metadata revision invalidates pending/confirmed render metadata and schedules a fresh request. UI requests emit once per cumulative egui frame, including layout retries. Direct `request_page` calls return their request immediately and are not re-emitted by `show`.

A successful request increments the sequence and returns a target whose page revision equals that new sequence. Capture `target()` anew for each future action. Replaying an earlier action target is refused, even within the same document/metadata revision. Requests include pane identity, so two panes with the same path and numeric revisions cannot acknowledge each other's results.

`select` rejects older or reused activation revisions, even after `select(None)`. Re-selecting the identical current key is a no-op. Metadata revisions must strictly increase within the active key. Renderer reports must equal the exact latest pending request and current target; reports for older pages, same-page retries, metadata revisions, documents or panes cannot change display metadata. Terminal acknowledgement clears pending; duplicates are refused.

## Input and retained-state contract

Use one controller per pane and a stable, unique caller pane salt. Never derive salts from paths, document revisions or requests. One finite presenter namespace, four focusable controls and two path-hover tooltip identities are reused across selections and controller recreation. The controller retains only current document/state/request data, no history or response queue.

Explicit document/metadata epochs reject inherited opening/held activation keys and old primary press/release gestures. Primary press captures the immutable pane/document/metadata/page target; release after any newer page request is stale even if the widget ID and metadata remain unchanged. Keyboard navigation still captures its current target when explicitly invoked. Only owned stale focus is reset. New-document TextEdit state resets, but same-document metadata keeps the editor's focus, cursor/selection and undo history. No global egui data/cache clearing, forced cardinal navigation, modal layer or window-wide input consumption. Enter/Space are consumed only when an allowed PDF control owns the corresponding activation key. Text editing, unrelated pane input and modal/disabled parent ownership remain available.

On layout retries, the editor still registers focus interest but uses an immutable display buffer and restores its own TextEditState/cursor without surrendering current editor focus. Default egui `run_ui` moves volatile editing commands into the first pass only. No independent Undoer copy/restore is needed or provided for artificial command reinjection. Actions run once per frame. The pinned egui 0.36.2 child-Ui auto-ID rule is used to identify the three button IDs before drawing; tests compare it with the actual Response IDs. An egui upgrade must retain/recheck this contract.

## Archived S013-R07 experiment and proven corrections

The alternative commit `4bd4fa5a8cf7cf0e62cc5c0c51e40ff9570979db` remains in branch history. Its conditional undo experiment is not an ordinary-host defect or a justification for the current production implementation. Supervisor clarification of reviewer `bc2626b6-44ff-406` withdrew that blocker. R09 below removes only the unsupported undo correction; the independently reproduced pointer-target and draft corrections remain.

- Frozen start: `c0d6fbf423d9120f2c18d8adf8e45bb29516e117`, clean `lane/conflict-dialog`.
- Rework writes only `components/pdf-controls/src/lib.rs`, `components/pdf-controls/src/tests.rs` and this handoff. Existing 24 behavior checks and assertions remain unchanged.
- No approval, integration, root/shared/sibling edit, dependency change or production enablement. No file/clipboard/native renderer/actual GUI/cache/thread work.

Pinned SDK source read at `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.36.2/src/`:

- `widgets/text_edit/state.rs` has an `Arc<Mutex<TextEditUndoer>>` inside cloned state. Public `undoer()` clones its contents; `set_undoer()` replaces those contents. Alternative 4bd captured `(state, state.undoer())` before retry rendering and restored the independent contents before storing state. The shared Arc observation is true, but does not prove default-host history corruption. R09 removes that production logic.
- `widgets/text_edit/builder.rs` processes undo/redo and feeds history even for an immutable text buffer. Under the artificial reinjected-command condition this advanced history. It does not establish a default `run_ui` failure.
- `util/undoer.rs` provides `add_undo`, `undo` and `redo`. Regression seeds three points from actual headless editor drafts, then checks real Ctrl+Z/Ctrl+Y changes, cursor/focus stability across three discard passes and subsequent history.
- `context.rs` `run_dyn` uses `new_input.take()` after pass zero. Plain default `run_ui` discard does not replay volatile key events, and the initial plain-discard probe passed on the frozen start. The retained-command regression explicitly keeps the owned Ctrl+Z/Y event on retry via `input_mut`, simulating a host that retains editing commands. This condition is documented, not presented as default egui behavior. It fails on the frozen implementation because retry editing advances the shared history while text remains unchanged. No actual GUI claim.

Red evidence before changing production code:

```text
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10 r07_ -- --nocapture
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10 r07_unsubmitted -- --nocapture
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10 r07_layout -- --nocapture
```

- Held Next, direct request for page 3, then release emitted an unwanted request. Assertion expecting no request failed.
- Never-submitted valid `2` and invalid `9` drafts followed by shrinking metadata were replaced by `1`. Both text equality assertions failed. Initial fixture used committed page 8 to cover metadata clamping. Final fixture also covers committed page 1, which isolates automatic-request overwrite without clamping.
- Retained Ctrl+Z over three discard passes left text `3` but destroyed redo to `4`: `undoer().redo(current).unwrap()` failed with None. The initial plain-discard variant passed for the SDK reason above.
- First combined red run also exposed an unapplied test `TexturesDelta` destructor panic during draft assertion unwinding. The new fixture clears test deltas before assertions; separate red draft rerun reports two ordinary assertion failures. No production workaround and no old test changed.

Alternative 4bd passed all four new focused checks and all 28 component checks. The final fixture strengthens metadata coverage with both committed pages 1 and 8. The undo test's retained-command condition was subsequently withdrawn as a supported-host blocker. The following commands/results are archived 4bd evidence, not R09 reruns.

Archived rework commands, all from explicit `cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`:

```text
cargo fmt --manifest-path components/pdf-controls/Cargo.toml
cargo fmt --manifest-path components/pdf-controls/Cargo.toml --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10 r07_ -- --nocapture
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10
git diff --check
git diff --cached --check
```

Final results: formatting and locked/offline all-target check passed; focused tests 4 passed, 0 failed/ignored; full suite 28 passed, 0 failed/ignored, plus 0 doc tests; diff checks passed. Only the three assigned paths changed. Component-only terminal commit is reported in the worker handoff, not self-approved here.

SDK/public API limits and all future-host gates below remain unchanged. Manual UI, DPI, Narrator, PDF rendering and root integration are unavailable/not performed. No full criterion completion claim.

## S013-R09 supported-host correction

- Frozen start: clean `lane/conflict-dialog`, `4bd4fa5a8cf7cf0e62cc5c0c51e40ff9570979db`.
- Goal: minimal P10 patch for the supported default egui host, not a general retry robustness framework.
- Exact write boundary: `components/pdf-controls/src/lib.rs`, `components/pdf-controls/src/tests.rs`, `components/pdf-controls/HANDOFF.md`.
- `lib.rs` removes only the retry-time `state.undoer()` deep copy, tuple snapshot and `state.set_undoer()` replacement. It restores the original `TextEditState::load`/`store` behavior and updates its comment. No new allocation logic or undo framework.
- Pointer press still captures its immutable target. Dirty draft tracking, metadata-state/focus/history preservation and explicit submission behavior remain unchanged from 4bd.
- `tests.rs` removes only retry-time `ctx.input_mut` command reinjection and its artificial-host comment, and renames the undo check to `r09_default_layout_retries_preserve_real_undo_redo_history`. Three public `add_undo` points, actual Ctrl+Z/Ctrl+Y input, three `request_discard` passes, text/history/cursor/focus assertions and subsequent undo/redo checks all remain. No assertion changed, weakened or removed. All original 24 checks and the three independently red pointer/draft regressions remain unchanged.

Pinned SDK proof, reread during R09 under `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.36.2/src/`:

- `context.rs:855`, `Context::run_dyn`: every pass calls `self.begin_pass(new_input.take())`.
- `data/input/raw_input.rs:120-143`, `RawInput::take`: documented volatile-input move, including `events: core::mem::take(&mut self.events)`. Ctrl+Z/Y is not replayed on retry.
- `util/undoer.rs:185-189`, `Undoer::feed_state`: a state equal to the latest undo point clears flux, not redo. Retry feeding the post-undo state therefore preserves redo.
- `widgets/text_edit/state.rs:47-48,70-76`: cloned TextEditState shares an Arc; this alone does not require a deep copy. No documented legitimate double-show or command-reinjection contract was established.

The supported-host undo check passed on c0 baseline in the prior plain-discard probe and passes in R09. It is retained coverage, not a red-before bug regression. Only held Next/direct page 3/release and valid/invalid unsubmitted-draft loss have proven baseline failure evidence. Artificial reinjection remains archived in 4bd, not supported production behavior.

R09 commands, each from explicit `cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`, with checkout-root `target/p10` build output:

```text
cargo fmt --manifest-path components/pdf-controls/Cargo.toml --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10 r09_default_layout_retries_preserve_real_undo_redo_history -- --nocapture
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10 r07_ -- --nocapture
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10
git diff --check
git diff 4bd4fa5a8cf7cf0e62cc5c0c51e40ff9570979db -- components/pdf-controls/src/lib.rs components/pdf-controls/src/tests.rs components/pdf-controls/HANDOFF.md
git diff c0d6fbf423d9120f2c18d8adf8e45bb29516e117 -- components/pdf-controls/src/lib.rs components/pdf-controls/src/tests.rs components/pdf-controls/HANDOFF.md
git diff --cached --check
```

Results: formatting and locked/offline all-target compile passed. Supported-host undo/redo focused check 1 passed; retained pointer/draft focused checks 3 passed; full suite 28 passed, 0 failed/ignored; 0 doc tests. No failed R09 fix attempts. Diff checks passed. Complete owned delta inspected against both 4bd and c0. No other path changed. Git reported only LF-to-CRLF working-copy warnings.

F08.1/F08.2 remain component-only partial implementation; supplied fallback presentation does not complete F08.4. Native/manual UI, DPI, Narrator, performance, renderer, packaging and root integration remain unperformed. No filesystem/clipboard/native probe, actual GUI, rendering or enablement occurred. Terminal SHA is reported separately for fresh exact-head review only, not approval. Lane continuation requires its own supervisor review, serial integration and validation against updated main.

## Original P10 validation

Every shell command used an explicit `cd C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`.

Final coherent-change checks:

```text
cargo fmt --manifest-path components/pdf-controls/Cargo.toml --check
cargo check --locked --offline --all-targets --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10
cargo test --locked --offline --manifest-path components/pdf-controls/Cargo.toml --target-dir target/p10
task --taskfile components/pdf-controls/Taskfile.yml run
git diff --check
git diff --cached --check
```

- Formatting, locked/offline all-target compile and diff checks passed.
- 24 built-in tests passed, zero failed/ignored. Headless UI checks use actual egui contexts, events and shaped output, not just controller mocks.
- Generated demonstration passed: 15 shapes, supplied three pages, requested index zero, no confirmed displayed page. No file was opened or rendered.
- `task --taskfile components/pdf-controls/Taskfile.yml --list` passed. Descriptions/default listing/run/test/check/clean are present; generated build output goes only to root `target/p10`.
- Lockfile generated offline. Every dependency name/version pair occurs in the root Cargo.lock; egui is exactly 0.36.2. No new dependency beyond installed egui/std.

Checks cover real button bounds, direct valid/invalid/empty/overflow/prefix-truncation inputs, metadata/fallback output, zero/oversized counts, `u32` limits, request-sequence exhaustion, document reset and changed limits, stale keys/actions/render acknowledgements, independent panes, own keyboard/Tab/focus, unrelated text input, modal/disabled ownership, inherited opening/held keys, delayed old release and retry, 128 document cycles with real hovered full-path tooltips and bounded retained Areas/layers/four focus IDs without cardinal cache pruning, and one-shot initial/button/direct requests and text edits during layout retries.

Development failures retained in evidence:

- Initial 18-test run: 16 passed. One test read button enablement before the following frame; corrected its frame timing without weakening the bound assertions. A real retry-focus defect failed twice before correction, with one unsuccessful focused fix. Replacing hover-only retry editing with the immutable-buffer/state-restoration approach fixed it.
- A new captured-action regression failed because a prior same-document Next target could advance a newer requested page. Adding the page revision to the immutable target made it pass.
- A long-leading-zero input regression failed because TextEdit's character cap removed the final digit. Removed silent truncation; the complete out-of-bounds decimal now stays visible and is refused.
- Final 24-test run passed after these corrections. No repeated-failure diagnosis boundary was exceeded.

## Smallest future root adapter and remaining limits

1. Keep a controller alongside each pane preview. Use the native pane ID as its stable salt. Assign a new strictly increasing activation revision whenever the selected PDF context changes or reopens. Preserve revision uniqueness across controller recreation. Do not feed untagged worker results into a newly captured current key.
2. Supply only worker metadata whose native source path and captured activation match the controller's current document. Tag metadata updates with their issued revision, not a fresh revision assigned after a late completion. Map worker failures to the supplied fallback states. Path admission/source identity, read limits and scheduling remain host/backend responsibilities; `DocumentKey::new` performs lexical absolute-path validation only.
3. Collect `ControlsOutput.request` across all layout passes without overwriting a first-pass request with a later `None`. Direct controller actions dispatch only the request they returned. Suppress browsing shortcuts while this pane's controls own keyboard input, including submission frames.
4. Pass `request.page_index()` to the reviewed backend's `RenderRequest`, plus separately validated physical-pixel dimensions and output allowance. Validate `pending() == Some(&request)` and `target() == Some(request.target().clone())` before installing a worker bitmap. Also check returned native source/page metadata against the captured request. Only then acknowledge `Displayed`; acknowledge failures with the same exact request. Changing documents/metadata must also invalidate the host's old frame.
5. The accepted backend currently exposes `Renderer::render_file`, not a metadata-only call. The host needs a captured-key first-page bootstrap render to obtain the initial count, or a separately reviewed metadata API. A retained bootstrap page-zero result may satisfy the subsequent page-zero intent only after its key/path/page/count checks match. P10 adds neither backend API nor cache/render logic.

No native visual, DPI, Narrator, performance, packaging, real PDF or production integration checks were performed. Zoom/fit and physical-pixel viewport/rendering remain outside P10. Supplied error/path strings and pasted editor text have no byte-budget claim here; GUI identities and record cardinality are bounded. The host still owns shared preview budgets and input admission. Page requests alone never claim a frame is displayed.
