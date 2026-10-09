# P15 deletion confirmation component

## Scope and baseline

- Checkout `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog`, branch `lane/conflict-dialog`.
- Clean starting main baseline `113d7b1a2eab0c2015c261d1655eeaea84855d38`, after approved P10 integration and its twenty-eight tests.
- Exclusive changes `components/delete-confirmation/**`. No root, shared or sibling edits. Component-only candidate, not enabled in the application.
- F11.1/F11.2 confirmation portions and F11.4/F02.4 immutable UI routing portions implemented. No complete criterion or backend claim. F12 integration remains pending.

## Public API and host contract

`Request::new(RequestId, Kind, Vec<PathBuf>, PathBuf)` captures native paths and a location. Fields are private; immutable accessors expose `id`, `kind`, `paths`, `location`, and the count derived from every captured entry. Display conversion never becomes identity. `Kind` is `Trash` or `PermanentDelete`.

`Confirmation::new()` creates the window-owned controller. Keep one controller and one presenter per window Context. `present(Request)` accepts a newer request and replaces the previous pending request with a fresh input epoch. Caller supplies strictly increasing, never-reused request IDs for the window, including controller recreation. The controller rejects IDs at or below its own observed high-water mark. There is no global preference or apply-to-remaining setting.

`show(&Context) -> Output` draws the actual egui modal. `Output.answer` is emitted once. `Output.owns_input` stays true through the answer frame, repeated calls and layout retries; the host must suppress background explorer commands when true. The answered modal remains installed through that frame, then releases its captured snapshot on the next show.

`choose(RequestId, Choice) -> Result<Answer, Refusal>` is the ID-checked intent endpoint used by the UI. Only the currently pending ID can answer; wrong IDs, closed requests and repeated answers are refused. Hosts must not synthesize Confirm without user intent. `close(RequestId)` invalidates without producing an Answer or deletion intent. `pending()` returns only an unanswered snapshot. `refusal()` exposes the visible admission failure.

An `Answer` contains the exact original Request and `Choice::Confirm` or `Choice::Cancel`. `deletion_intent()` returns a borrowed snapshot only for Confirm. Cancel and Escape emit no positive deletion intent. Confirm does not mean completed deletion, OS success, successful validation, or atomic destructive authorization.

## Presentation and input guards

- Trash explicitly names the Windows Recycle Bin and recovery of recoverable items. It warns to leave entries intact if unavailable, never fall back to permanent deletion.
- Permanent Delete explicitly says irreversible and that entries will NOT go to the Recycle Bin.
- Both show complete captured item count, captured location and a scrollable list containing every captured entry. Visual truncation has read-only full-path hover, without copying or opening paths.
- Default focus is Cancel, with egui focus styling and explicit focus text. Enter defaults to Cancel. A fresh Tab navigation to Confirm permits Enter or Space to confirm; Space on focused Cancel cancels. Escape cancels. Arbitrary inherited Confirm focus alone cannot authorize Enter/Space.
- Fresh request/controller opening frames disarm pointer and key intent and cancel only opening-frame focus navigation. They do not clear global Context memory or force cardinal navigation.
- Pointer press/release is admitted only within an armed request epoch. An old held press remains blocked until release, even several frames later. An ambiguous batch containing an inherited release is refused and restores Cancel focus. New valid intent still works.
- Escape/Enter/Space track held state, not just repeat flags. Repeats, modified keys, duplicate downs and opening-frame presses cannot answer. These action key events are consumed only while this modal owns its layer. Unrelated text and another modal's keys remain available to their owner.
- Input batches are processed once per frame. Answer state prevents duplicate intent through layout retries or repeated show calls.
- One fixed presenter Area, two fixed action widget identities, one fixed location slot and sixty-four fixed entry slots. One additional fixed refusal-close button. Full-path tooltip Areas reuse the same slots across requests and controllers. Operation IDs are data, never widget salts.

## Resource limits and refusal

- Maximum 64 captured paths, plus one captured location.
- Maximum 32768 native units per source/location path, and 131072 units total including location. Windows units are UTF-16 code units; non-Windows units are encoded native bytes. Windows native identity retains unpaired UTF-16 and supplementary letters.
- Complete lexical absolute paths are required. This is UI admission, not filesystem/provider validation or protected-root policy.
- Count/length/empty/relative admission failures remove the previous pending request, retain no rejected path list and display an explicit refusal. Nothing is silently omitted or truncated. Rejected newer IDs are consumed, requiring another fresh ID for retry.
- At most one admitted native snapshot plus an emitted caller-owned Answer. At most one refusal. Admission checks are bounded by 65 paths. Drawing never enumerates, sorts or reads sources.
- Caller owns input snapshot creation and caller-held Answer lifetimes. This does not bound allocations before Request construction or caller retention.

## Validation

All checks run from `C:/SANDBOX/WORKTREES/mado-explorer/conflict-dialog/components/delete-confirmation`, using `CARGO_TARGET_DIR=../../target/p15` for Cargo builds.

- `cargo generate-lockfile --offline`: standalone application lock generated using installed packages only.
- `cargo fmt`, then `cargo fmt --check`: pass.
- `cargo check --locked --offline --all-targets`: pass.
- `cargo test --locked --offline --lib`: eighteen passed, zero failed, zero ignored.
- Four newly added checks were additionally run by exact test name: `accessibility_buttons_have_labels_and_cancel_is_focused`, `all_maximum_captured_entries_are_present_with_fixed_slot_ids`, `opening_pointer_press_with_key_cannot_carry_into_current_request`, `invalid_new_request_removes_previous_positive_action`. All pass.
- `task --list`, `task check`, `task test`, `task run`: pass. Run is a generated headless cancelled demonstration. It performs no filesystem access.
- `git diff --check`: pass. Final staged diff inspected; only the seven component files changed.

Tests exercise actual egui frames, pointer/keyboard events, painted warnings/count/location/full hovers, AccessKit button labels and focused Cancel, maximum sixty-four-entry snapshots, empty/oversized refusals, exact one-shot multi-selection responses, later changed selection data, wrong/stale/closed IDs, no inherited preference, old held pointer and opening focus/keys across requests and controller recreation, repeated/modified/held action keys, another modal's ownership, unrelated text, and answer-frame layout retries.

The retention check runs 256 requests across eight controllers in one Context, hovers location and every supplied source with full path text actually observed, and compares retained layers plus actual retained AreaStates after dismissal. Four tooltip Areas remain constant; actual action Response IDs stay a set of two without cardinal navigation. A separate maximum-entry check proves all sixty-four source slot IDs and the location slot remain a fixed set of sixty-five.

All metadata is generated in memory. No fixture file is opened, created, trashed or deleted. No desktop/native GUI, personal path, clipboard, elevation or OS deletion probe occurred. This new component has no pre-existing failing baseline; tests are retained runnable regression coverage, not claimed red-before corrections.

## Remaining gates

The future reviewed backend must capture/revalidate native source identity, reject disappeared/replaced entries, check permissions and OS Recycle Bin availability, enforce OS-resolved protected-root restrictions, handle links without following targets, and report missing/changed/partial failures. Never convert Trash failure into Permanent Delete. User intent is not TOCTOU-safe mutation authority and does not make a multi-entry operation atomic.

Production application command/visible-selection capture, operation worker/drawer routing, pane reconciliation, checkpoint probes, protected-root/link safety, actual disposable OS trash/delete tests, Windows Narrator, native focus/visual interaction and DPI/narrow-window verification remain pending. No full F11.1/F11.2/F11.4/F02.4/F12/F14 acceptance claim. No production enablement.

Fresh read-only exact-head review, parent approval, serial integration and validation on updated main are required. No next sibling task or integration action was taken by this worker.
