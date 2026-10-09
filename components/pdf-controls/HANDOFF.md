# P10 PDF metadata and page controls

## Candidate boundary

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`.
- Branch: `lane/conflict-dialog`.
- Parent baseline: `a47fb01483679890aef367ebb7feb0e60b9ee051`.
- Writes only `components/pdf-controls/**`. No root, shared or sibling changes.
- Fresh exact-head review and supervisor integration/validation required. No acceptance statuses or production gates changed.

## Behavior and partial acceptance IDs

F08.1 metadata/control portion: working pane-local egui filename/full-path labels with complete hover text, supplied page count, requested page and separate confirmed displayed page. Counts are unavailable during loading, missing-renderer, malformed, password-required and supplied failure states. Zero and counts above `u32::MAX` become explicit invalid-metadata failures, not zero-page documents.

F08.2 UI/state portion: actual Previous/Next buttons, editable one-based page number, Go and focused Enter/Space activation. Requests use checked zero-based `u32` indexes. Invalid, empty, overflowed and out-of-bounds input stays in the editor with an explanation. Input is not silently truncated to a different valid decimal command. Selection changes reset to page one. A smaller same-document count resets an out-of-range requested page to page one; an in-range page remains requested. Invalid drafts survive metadata changes and automatic requests.

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

Explicit document/metadata epochs reject inherited opening/held activation keys and old primary press/release gestures. Only owned stale focus and owned TextEdit state are reset. No global egui data/cache clearing, forced cardinal navigation, modal layer or window-wide input consumption. Enter/Space are consumed only when an allowed PDF control owns the corresponding activation key. Text editing, unrelated pane input and modal/disabled parent ownership remain available.

On layout retries, the editor still registers focus interest but uses an immutable display buffer and restores its own cursor/undo state. This avoids repeated edits without surrendering current editor focus. Actions run once per frame. The pinned egui 0.36.2 child-Ui auto-ID rule is used to identify the three button IDs before drawing; tests compare it with the actual Response IDs. An egui upgrade must retain/recheck this contract.

## Validation

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
