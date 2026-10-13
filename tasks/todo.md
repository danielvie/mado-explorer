# Rust/Windows file manager task checklist

Implementation authorized 2026-10-13. T01/T02 foundation code exists and automated checks passed; final interactive acceptance is pending. T03 onward is unstarted. [Implementation specification and plan](C:/SANDBOX/REPOS/rev-filepilot/tasks/plan.md) owns proposed decisions and policies; [recovered specification](C:/SANDBOX/REPOS/rev-filepilot/SPECIFICATION.md) owns artifact evidence.

- [x] User authorized implementation with proposed defaults on 2026-10-13; maximum six concurrent agents total.

Commands below become available only after T01. Every task also runs the plan's rustfmt/clippy/full-test Definition of Done. Focused filters are intended test names to add, not existing passing tests. Native tests run only by explicit opt-in, on Windows, serialized and inside owned disposable fixtures. No test launches FPilot.exe. No fixture may change unrelated files, global Recycle Bin contents, registry defaults, or PATH.

Acceptance criteria are at most three per task; likely changed files are at most five. Count module wiring and lockfile changes too. If implementation exceeds this boundary, stop and split the task rather than quietly broadening it. No calendar estimate is claimed. Task IDs must stay stable. T43-T45 were added after review and appear at their dependency position; execute in document order, not numeric ID order.

## Task T01: Launch the application window

- [ ] T01 complete

**Description:** Create the smallest Windows Rust application and its own Taskfile. Resolve compatible UI versions; do not create empty feature modules.

**Acceptance criteria:**
- [ ] A native window opens and closes using the candidate UI stack on Windows x64.
- [ ] Cargo.lock is committed when version control is established; toolchain and dependency choices are recorded.
- [x] Application Taskfile provides default --list and described run/build/test/clean tasks without changing the root analysis Taskfile.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml window_boot`.
- [x] Build: `cargo build --manifest-path app/Cargo.toml --offline` passed 2026-10-13.
- [ ] Manual: Launch and close the Rust window ten times; verify no FPilot.exe execution or writes to analysis/user state.

**Dependencies:** None; user authorization received 2026-10-13.

**Files likely touched:** `app/Cargo.toml`, `app/Cargo.lock`, `app/src/main.rs`, `app/src/lib.rs`, `app/Taskfile.yml`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

**2026-10-13 evidence:** Minimal Wayfarer Files window, Cargo.lock and app Taskfile created. Ten automated native launches rendered four UI frames each and exited normally. The direct main-thread executable probe replaces the planned window_boot test filter, avoiding a Rust test-worker event loop. Interactive/manual acceptance remains unchecked. See completion record below.

## Task T02: Establish the native window lifetime

- [ ] T02 complete

**Description:** Prove HWND acquisition, compatible COM/OLE initialization timing, callback ownership, and safe shutdown with the pinned toolkit.

**Acceptance criteria:**
- [x] The adapter obtains the real HWND through supported APIs and checks COM initialization results on its owning thread.
- [ ] Owned registrations/callback contexts survive callbacks and are removed before window destruction; no unsafe Send/Sync is introduced.
- [ ] A native callback probe preserves toolkit keyboard/mouse input and reports incompatible initialization instead of continuing.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml native_runtime`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_runtime -- --ignored --test-threads=1`.
- [x] Build: `cargo build --manifest-path app/Cargo.toml --offline` passed 2026-10-13.
- [ ] Manual: Repeat open/close and DPI changes; inspect callback teardown and basic Narrator/window focus behavior.

**Dependencies:** T01.

**Files likely touched:** `app/Cargo.toml`, `app/Cargo.lock`, `app/src/lib.rs`, `app/src/platform/mod.rs`, `app/src/platform/windows.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

**2026-10-13 evidence:** Native callback round trip and pre-destruction removal passed ten real window cycles after fixing shutdown ordering. Nested OLE initialization test passed explicitly. Keyboard/Narrator/IME/mixed-DPI checks remain pending, so this task and downstream acceptance are not complete.

## Task T03: Browse one directory asynchronously

- [ ] T03 complete

**Description:** Deliver the first complete browsing slice with owned paths, background enumeration, a virtualized details view, and explicit failure states.

**Acceptance criteria:**
- [ ] Home or a supplied directory displays name/type/size/modified rows with loading, empty, and error states.
- [ ] Rapid navigation discards late results by tab/panel generation; authoritative paths remain lossless.
- [ ] Instrumented row construction is bounded by visible rows plus documented overscan rather than total entries.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml directory_browse`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Browse a 10,000-entry owned fixture and an inaccessible directory while continuing to resize/type.

**Dependencies:** T01, T02.

**Files likely touched:** `app/src/lib.rs`, `app/src/model.rs`, `app/src/directory.rs`, `app/src/ui.rs`, `app/tests/directory_browse.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP01: Window and browse gate

- [ ] Native lifetime and a usable virtualized listing pass; keyboard/Narrator basics are demonstrated.
- [ ] No toolkit failure is hidden; T01-T03 tests/build pass and stale-result/row-count measurements are recorded.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T04: Select and activate entries

- [ ] T04 complete

**Description:** Implement a stable selection/cursor model and open/parent navigation in the visible listing.

**Acceptance criteria:**
- [ ] Ctrl-toggle, Shift-range, Select All, and cursor movement use the visible snapshot, not stale row positions.
- [ ] Activation navigates directories or opens files through the Windows adapter; vanished entries report errors.
- [ ] Keyboard focus is visible and ordinary text editing/IME input is not intercepted by file actions.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml selection`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Use keyboard only on Unicode and spaced names; refresh after deleting an entry externally in the owned fixture.

**Dependencies:** T03.

**Files likely touched:** `app/src/lib.rs`, `app/src/model.rs`, `app/src/selection.rs`, `app/src/ui.rs`, `app/src/platform/windows.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T05: Open a native Shell menu

- [ ] T05 complete

**Description:** Add native system-menu presentation for selected filesystem items before building a searchable custom popup.

**Acceptance criteria:**
- [ ] The correct HWND/selection obtains IContextMenu and opens the native menu with valid command-ID ranges.
- [ ] Required IContextMenu2/3 messages reach dynamic submenus and keyboard cancellation restores focus.
- [ ] The acceptance probe invokes only an approved read-only Properties action; interfaces/menu handles are released.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml shell_menu`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_shell_menu -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Open/dismiss menus with mouse and Shift+F10; expand a dynamic submenu and open Properties on owned files.

**Dependencies:** T02, T04.

**Files likely touched:** `app/src/platform/windows.rs`, `app/src/platform/windows/shell.rs`, `app/src/ui.rs`, `app/Cargo.toml`, `app/Cargo.lock`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T06: Copy files with explicit outcomes

- [ ] T06 complete

**Description:** Expose Copy To a chosen directory using a message-pumped STA operation owner and immutable requests.

**Acceptance criteria:**
- [ ] Copy runs outside the UI thread, creates interfaces inside its STA, and reports per-item results plus aborted status; close-during-copy enforces cancel/wait/keep-open before worker teardown.
- [ ] Cancelled, partial, and failed outcomes are distinguishable; HRESULT success alone cannot mark all items completed.
- [ ] Collision, denied/locked destination, and source/descendant validation paths do not silently overwrite or delete.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml operations_copy`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_operations -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Copy owned files with a collision and a locked destination; request cancellation, then close during an active copy and verify cancel/wait/keep-open choices and final worker cleanup.

**Dependencies:** T02, T04.

**Files likely touched:** `app/src/model.rs`, `app/src/platform/windows.rs`, `app/src/platform/windows/operations.rs`, `app/src/ui.rs`, `app/tests/native_operations.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP02: Shell and operation gate

- [ ] Native menu focus/messages and copy outcomes are demonstrated on owned fixtures.
- [ ] Cancellation/collisions/partial errors do not imply total success; review COM ownership before proceeding.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T07: Probe incoming Explorer file drops

- [ ] T07 complete

**Description:** Validate IDropTarget lifetime, coordinates, and bounded CF_HDROP extraction without production transfers.

**Acceptance criteria:**
- [ ] The pinned toolkit's existing drop registration is identified; exclusive supported registration is established without revoking toolkit-owned objects, with correct teardown and scaled-DPI coordinates.
- [ ] File names are extracted with queried lengths and bounded validation, including long/Unicode paths; unsupported formats are identified.
- [ ] The probe returns no successful transfer effect and never copies/moves/deletes dropped files.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml drop_probe`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_drop_probe -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Drag Explorer files onto two probe regions at 100% and 150% DPI; cancel and verify files are unchanged.

**Dependencies:** T02, T03.

**Files likely touched:** `app/src/platform/windows.rs`, `app/src/platform/windows/drag_drop.rs`, `app/src/ui.rs`, `app/tests/native_drop_probe.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T08: Drag selected files to Explorer

- [ ] T08 complete

**Description:** Obtain a Shell data object for supported selected filesystem items and enter the native drag loop.

**Acceptance criteria:**
- [ ] Outbound data objects and source state have explicit owning-thread lifetimes across the modal loop.
- [ ] Copy/move negotiation and cancellation use the Shell result; the application does not independently delete sources.
- [ ] Unsupported selections are rejected before entering a drag; callbacks remain valid through release/shutdown.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml drag_source`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_drag_source -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Copy and move owned files to a separate Explorer fixture folder with modifiers; cancel a drag and verify no mutation.

**Dependencies:** T02, T04, T07.

**Files likely touched:** `app/src/platform/windows/drag_drop.rs`, `app/src/platform/windows/shell.rs`, `app/src/ui.rs`, `app/tests/native_drag_source.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T09: Filter and order directory rows

- [ ] T09 complete

**Description:** Add documented filename filtering, logical-name ordering, visibility controls, and navigation history without an indexed-search engine.

**Acceptance criteria:**
- [ ] Substring filtering, folder-first ordering, and logical name/size/modified sorting follow the new documented policies with deterministic ties.
- [ ] Filtering prunes hidden selection; back/forward/parent/path navigation preserves lossless targets and drops stale results.
- [ ] 10,000/100,000-entry measurements record hardware, time/memory, and visible-row work without claiming File Pilot equivalence.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml listing_controls`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Filter mixed-case/Unicode names, sort numbered names, toggle hidden/system files, and navigate during refresh.

**Dependencies:** T03, T04.

**Files likely touched:** `app/src/directory.rs`, `app/src/selection.rs`, `app/src/ui.rs`, `app/src/platform/windows.rs`, `app/tests/listing_controls.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP03: Candidate stack decision

- [ ] Incoming/outgoing OLE probes, text input, accessibility basics, listing controls and baseline measurements pass.
- [ ] Human approves the candidate stack or stops to revise D03; no unsafe workaround substitutes for this gate.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T10: Preserve independent tabs

- [ ] T10 complete

**Description:** Add tab creation, switching, closing/reopening, and per-tab browsing state using the approved docking component.

**Acceptance criteria:**
- [ ] New/duplicate/reopened tabs retain their own location/history/filter/selection with stable IDs.
- [ ] Overflow tabs remain reachable and closing tabs cancels stale non-mutating requests without affecting active mutations.
- [ ] Last-tab behavior follows the final-pane Home safeguard rather than accidentally closing the application.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml tabs`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Open enough tabs to overflow, switch during loading, and close/reopen the active tab using keyboard only.

**Dependencies:** T03, T04, T09.

**Files likely touched:** `app/src/model.rs`, `app/src/ui.rs`, `app/src/ui/tabs.rs`, `app/Cargo.toml`, `app/Cargo.lock`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T11: Browse resizable split panes

- [ ] T11 complete

**Description:** Support horizontal/vertical splits with independent tabs and explicit active-pane targeting.

**Acceptance criteria:**
- [ ] Splits resize and equalize without sharing mutable tab/location/selection state.
- [ ] Keyboard pane selection and operation targeting follow the active pane, including after closure.
- [ ] Layout invariants reject invalid ratios/indices and leave one Home pane when the final pane would close.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml panes`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Use two folders in right/bottom splits, resize them, move focus, and verify independent filters/selections.

**Dependencies:** T10.

**Files likely touched:** `app/src/model.rs`, `app/src/ui.rs`, `app/src/ui/panes.rs`, `app/src/ui/tabs.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T12: Execute focus-aware commands

- [ ] T12 complete

**Description:** Introduce typed command IDs, command eligibility, shortcuts, and a basic searchable palette for implemented actions only.

**Acceptance criteria:**
- [ ] Palette entries and shortcuts invoke the same eligible typed actions; unimplemented commands cannot mutate files.
- [ ] Text editors and IME composition retain their shortcuts; single-letter Y does not copy files by default.
- [ ] Search uses the documented initial policy and deterministic ties; labels/availability track the active pane.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml commands`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Use the palette and Ctrl+T/W, Alt navigation, and F2 where implemented; verify shortcuts inside a filter editor.

**Dependencies:** T04, T05, T06, T09, T10, T11.

**Files likely touched:** `app/src/lib.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/src/ui/palette.rs`, `app/src/model.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP04: Multi-panel commands gate

- [ ] Tabs/splits and command routing work end-to-end without shared mutable selection or text-focus theft.
- [ ] Freeze owned intent/result boundaries before assigning parallel adapter/preview work.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T13: Copy file paths through the clipboard

- [ ] T13 complete

**Description:** Wire Copy and Copy As Path to owned Unicode clipboard payloads shared with Explorer.

**Acceptance criteria:**
- [ ] Copy publishes validated CF_HDROP and appropriate Unicode path text with correct global-memory ownership.
- [ ] Copy As Path uses a documented quoting policy while operations continue to use authoritative paths.
- [ ] Publication failure and malformed incoming clipboard data do not leak handles, read beyond bounds, or claim success.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml clipboard_copy`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_clipboard -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Copy owned files to Explorer and back; copy paths with spaces/Unicode and replace clipboard contents externally.

**Dependencies:** T04, T12.

**Files likely touched:** `app/src/platform/windows.rs`, `app/src/platform/windows/clipboard.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/tests/native_clipboard.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T14: Move cut files through paste

- [ ] T14 complete

**Description:** Add Cut/Paste using the operation executor, preferred move effect, and completion-aware clipboard updates.

**Acceptance criteria:**
- [ ] Paste resolves an immutable destination/request and distinguishes copy from move using validated clipboard formats.
- [ ] Only confirmed moved items update cut state, and only while the clipboard still matches the captured request.
- [ ] Partial failure, cancellation, clipboard replacement, same-folder moves, and cross-volume moves preserve remaining source/cut items.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml clipboard_move`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_clipboard_move -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Cut several owned files, cancel or force one failure, and verify successful and remaining items against the ledger.

**Dependencies:** T06, T11, T12, T13.

**Files likely touched:** `app/src/platform/windows/operations.rs`, `app/src/platform/windows/clipboard.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/tests/native_clipboard_move.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T15: Trash selected files safely

- [ ] T15 complete

**Description:** Add the ordinary Delete-to-Recycle-Bin action with explicit eligibility and outcome reporting.

**Acceptance criteria:**
- [ ] Trash requests cannot silently fall back to permanent deletion when recycling is unavailable.
- [ ] Cancellation, denied access, and partial results are visible and selection refresh follows confirmed outcomes.
- [ ] Tests touch only identified owned fixture items; no code or test empties the global Recycle Bin.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml trash`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_trash -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Trash owned fixture files and verify their identities in Recycle Bin without altering pre-existing entries.

**Dependencies:** T06, T12.

**Files likely touched:** `app/src/platform/windows/operations.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/tests/native_trash.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Checkpoint CP05: Clipboard and Trash gate

- [ ] Explorer copy/cut/paste and safe recycling pass cancellation/partial-failure cases.
- [ ] Conditional clipboard updates and no permanent-delete fallback are reviewed with human.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T16: Create directory entries

- [ ] T16 complete

**Description:** Provide new-folder and new-empty-file actions in the current pane through the operation owner.

**Acceptance criteria:**
- [ ] Valid names create exactly the requested type in the captured current directory and then refresh/select the result.
- [ ] Invalid names, reserved names, collisions, and disappearing destinations produce explicit errors without replacement defaults.
- [ ] Cancelling the name dialog submits no operation and leaves existing entries unchanged.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml create_entry`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_create_entry -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Create an owned file/folder, cancel another creation, and attempt invalid/colliding names.

**Dependencies:** T06, T12.

**Files likely touched:** `app/src/platform/windows/operations.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/src/ui/dialogs.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T17: Rename a selected entry

- [ ] T17 complete

**Description:** Add single-entry rename with explicit filename/extension handling and safe submission.

**Acceptance criteria:**
- [ ] Rename edits one authoritative selected entry, previews the full resulting name, and rejects empty/invalid names.
- [ ] Commit/cancel and collision/vanished-source outcomes are distinct; no successful operation is inferred from text acceptance.
- [ ] F2 and the palette invoke the same eligible action without taking over ordinary text editing.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml rename_entry`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_rename_entry -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Rename spaced/Unicode files, preserve or explicitly edit extensions, cancel, and test a collision.

**Dependencies:** T06, T12, T16.

**Files likely touched:** `app/src/platform/windows/operations.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/src/ui/dialogs.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T18: Inspect bounded text contents

- [ ] T18 complete

**Description:** Add a read-only text inspector with generation-aware background reads and explicit unsupported-data behavior.

**Acceptance criteria:**
- [ ] The selected file displays bounded UTF-8 text and metadata without blocking navigation or editing the file.
- [ ] Oversized, binary, malformed, missing, or unreadable data produces a clear fallback; reads stop at the configured bound.
- [ ] Stale results are discarded and inspector text controls own their focus/selection/clipboard shortcuts.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml text_preview`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Inspect normal, binary, oversized, and inaccessible owned files; change selection repeatedly while loading.

**Dependencies:** T03, T04, T11, T12.

**Files likely touched:** `app/src/lib.rs`, `app/src/preview.rs`, `app/src/ui.rs`, `app/src/ui/inspector.rs`, `app/tests/text_preview.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP06: Editing and text preview gate

- [ ] Creation, single rename and bounded text inspection work through both panes.
- [ ] Tests/build pass; validation, stale previews and keyboard/IME focus are checked.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T19: Decode bounded image data

- [ ] T19 complete

**Description:** Add the WIC decoder as an independently checked boundary before connecting images to the inspector. Enable and verify the required binding features here.

**Acceptance criteria:**
- [ ] WIC interfaces are created/released on a verified compatible thread and return owned pixels with an explicit channel/alpha format.
- [ ] Dimensions, decoded byte counts, API results, corrupt input, and configured application-owned limits are checked before buffer use.
- [ ] Native fixture tests verify decoding/error cleanup without requiring new UI or shipping a second decoder.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml image_decode`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_image_decode -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Decode owned transparent PNG/JPEG and malformed/oversized fixtures; inspect owned memory and returned pixel format.

**Dependencies:** T02, T18.

**Files likely touched:** `app/Cargo.toml`, `app/Cargo.lock`, `app/src/platform/windows.rs`, `app/src/platform/windows/preview.rs`, `app/tests/native_image_decode.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T45: Display bounded image previews

- [ ] T45 complete

**Description:** Connect the accepted WIC decoder to the existing preview generations, inspector controls, and bounded texture cache.

**Acceptance criteria:**
- [ ] Selected supported images display without blocking the UI; stale/cancelled results cannot install textures.
- [ ] Pixel alpha/channel format is converted correctly; zoom/fit, replacing, and closing the view release or reuse resources within the cache budget.
- [ ] Decode failures remain visible and text inspector/focus behavior remains intact.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml image_view`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Preview transparent images in both panes, change selection rapidly, zoom, close the inspector, and inspect cache/texture counts.

**Dependencies:** T18, T19.

**Files likely touched:** `app/src/preview.rs`, `app/src/ui/inspector.rs`, `app/src/ui.rs`, `app/src/model.rs`, `app/tests/image_view.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T20: Refresh listings from filesystem changes

- [ ] T20 complete

**Description:** Treat notifications as invalidation hints and coalesce generation-aware rescans.

**Acceptance criteria:**
- [ ] External create/rename/delete changes update the current listing without changing authoritative selection incorrectly.
- [ ] Overflow/error/disconnect triggers visible invalidation and rescan recovery rather than a false empty listing.
- [ ] Closing tabs removes their watchers; sustained events and network failures do not block UI input.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml watch_refresh`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_watch_refresh -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Change an owned directory in Explorer, generate an event burst, and disconnect/reconnect a test share when available.

**Dependencies:** T03, T09, T10.

**Files likely touched:** `app/src/directory.rs`, `app/src/ui.rs`, `app/tests/watch_refresh.rs`, `app/Cargo.toml`, `app/Cargo.lock`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP15: Preview and notification gate

- [ ] WIC decoding, image presentation, and watchers pass bounded-resource/stale-result/native checks.
- [ ] Required binding features and pixel format are recorded; input remains responsive during errors and event bursts.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T21: Validate the session format

- [ ] T21 complete

**Description:** Define the application's versioned session DTO and lossless path codec as a tested persistence boundary. Filesystem replacement and startup integration are separate tasks.

**Acceptance criteria:**
- [ ] Owned pane/tab/location/view/inspector values round-trip with lossless paths; live handles/PIDLs/callbacks/jobs are not serializable.
- [ ] Version, size/depth, finite ratios, unique IDs, active indices, and path encoding validation reject invalid input before state application.
- [ ] Corrupt/future input produces typed diagnostics and safe fallback data without modifying any input file.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml session_codec`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Inspect representative current/future/corrupt fixture results and compare lossless path round-trips; no application restore is claimed yet.

**Dependencies:** T09, T10, T11, T18, T45.

**Files likely touched:** `app/Cargo.toml`, `app/Cargo.lock`, `app/src/lib.rs`, `app/src/session.rs`, `app/tests/session_codec.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T43: Preserve state during replacement failures

- [ ] T43 complete

**Description:** Implement validated state reads and same-directory temporary replacement using verified platform/stdlib guarantees and fault-injection fixtures.

**Acceptance criteria:**
- [ ] A complete validated serialized state is written/synchronized before replacement; replacement failure leaves prior good contents intact.
- [ ] Corrupt/future existing state is read-only until explicit recovery; saving defaults cannot silently replace it.
- [ ] Interruption/failure checks distinguish pre/post-commit outcomes and clean only owned temporary files; no universal power-loss guarantee is asserted.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml session_save`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_session_save -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Force write/replace failures and process interruption on owned fixtures; verify old or committed-new state and preserved unreadable files.

**Dependencies:** T21.

**Files likely touched:** `app/src/session.rs`, `app/tests/native_session_save.rs`. Tests may be inline in these modules.

**Estimated scope:** Small, 1-2 files.

## Task T44: Restore sessions through the window lifecycle

- [ ] T44 complete

**Description:** Connect accepted persistence to startup/shutdown, missing-location handling, and a single-writer lease.

**Acceptance criteria:**
- [ ] Startup restores validated tabs/panes/preferences with monitor-safe geometry; missing folders remain visible as warned locations.
- [ ] The writer lease prevents a competing process from overwriting state and releases on shutdown/crash according to checked Windows ownership.
- [ ] Close/restart saves only safe persistent state, never pending mutations; corrupt/future load and failed saves keep preservation/read-only rules.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml session_lifecycle`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_session_lifecycle -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Restart a layout, remove a saved directory, supply corrupt/future state, and start a second instance to verify writer ownership and warnings.

**Dependencies:** T02, T20, T21, T43.

**Files likely touched:** `app/src/lib.rs`, `app/src/ui.rs`, `app/src/session.rs`, `app/src/platform/windows.rs`, `app/tests/native_session_lifecycle.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP07: Refresh and persistence gate

- [ ] Session codec, replacement, and lifecycle slices pass invalid/corrupt/future state, interrupted-save, missing-path, and competing-writer cases.
- [ ] No unsupported/corrupt state is overwritten; memory and queue behavior are recorded.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T22: Complete incoming file transfers

- [ ] T22 complete

**Description:** Replace the read-only drop probe with current-pane filesystem transfers delegated synchronously to the acquired Shell IDropTarget. Application-side queued mutation is not a second authority.

**Acceptance criteria:**
- [ ] Current-pane targeting is correct across split panes/DPI; unsupported drops and stale destinations return no successful effect.
- [ ] The acquired Shell target is the only transfer authority; Drop's returned status/effect is used, no app operation is also queued, and no per-item success is invented.
- [ ] Explorer copy/move/cancel/collision tests report actual outcomes or delegated status, with rescans and complete lifecycle cleanup.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml drop_transfer`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_drop_transfer -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Drop owned Explorer files onto both panes with copy/move modifiers; cancel and verify no duplicate or premature deletion.

**Dependencies:** T06, T07, T11, T14, T20.

**Files likely touched:** `app/src/platform/windows/drag_drop.rs`, `app/src/platform/windows/operations.rs`, `app/src/ui.rs`, `app/src/model.rs`, `app/tests/native_drop_transfer.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T23: Pass the Windows interaction gate

- [ ] T23 complete

**Description:** Verify accessibility, keyboard routing, text composition, contrast, and mixed-DPI behavior across the completed MVP.

**Acceptance criteria:**
- [ ] Keyboard-only browsing, menus, palette, tabs/panes, inspector, and operations have visible focus and no focus traps.
- [ ] Narrator identifies rows/selection/active tab/filter/progress; IME text input remains functional.
- [ ] High contrast and 100/150/200% DPI including monitor moves preserve readable content, hit targets, and native menu/drop coordinates.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml interaction`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Run the entire MVP workflow with keyboard and Narrator, then IME/high-contrast and mixed-DPI monitors.

**Dependencies:** T08, T12, T15, T16, T17, T18, T45, T44, T22.

**Files likely touched:** `app/src/ui.rs`, `app/src/ui/palette.rs`, `app/src/ui/inspector.rs`, `app/src/platform/windows.rs`, `app/tests/windows-acceptance.md`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T24: Package the MVP executable

- [ ] T24 complete

**Description:** Produce a reproducible Windows release build and documented installation-free startup without copying recovered assets.

**Acceptance criteria:**
- [ ] Release executable starts on the approved Windows baseline with an asInvoker/long-path-aware manifest and no console surprise.
- [ ] MVP acceptance records show supported/unsupported cases and measured performance budgets; no original equivalence is claimed.
- [ ] README/Taskfile describe run/build/test/clean and release use; package changes no registry handlers, PATH, or original analysis.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml release_smoke`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml --release`.
- [ ] Manual: Run the release executable outside the source directory with fresh state and complete the MVP checklist.

**Dependencies:** T01, T23.

**Files likely touched:** `app/src/main.rs`, `app/build.rs`, `app/assets/app.manifest`, `app/README.md`, `app/Taskfile.yml`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP08: MVP release acceptance

- [ ] All MVP criteria, including T43-T45, native Windows scenarios, accessibility and release smoke checks pass.
- [ ] Human accepts the MVP/support matrix before optional advanced tasks start.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T25: Search Shell commands in a custom popup

- [ ] T25 complete

**Description:** Add searchable application presentation on top of the already-verified Shell adapter, keeping native fallback.

**Acceptance criteria:**
- [ ] Labels, verbs, IDs, disabled states, and lazy submenus stay bound to their owning Shell menu object.
- [ ] Matching/ranking/ties are documented new policies; disabled results cannot invoke and native fallback remains available.
- [ ] Known/common file commands route once to the intended authority; owner-drawn/unsupported extensions are not falsely advertised as compatible.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml shell_search`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_shell_search -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Search/expand/invoke owned-file commands with keyboard; compare fallback and test registered extensions available on the machine.

**Dependencies:** T05, T12, T24.

**Files likely touched:** `app/src/platform/windows/shell.rs`, `app/src/ui/palette.rs`, `app/src/commands.rs`, `app/tests/native_shell_search.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T26: Browse Recycle Bin items

- [ ] T26 complete

**Description:** Introduce a typed virtual location with Shell enumeration and documented property mappings.

**Acceptance criteria:**
- [ ] Recycle Bin rows identify items through Shell values rather than pretending they are ordinary directory paths.
- [ ] Original location/deletion date/size properties use checked SDK conversions and missing-property fallbacks.
- [ ] Refresh/stale selection rules apply; the virtual location persists by kind, never by live PIDL/interface bytes.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml bin_listing`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_bin_listing -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: View owned recycled fixtures alongside existing items without changing either; restart a Recycle Bin tab.

**Dependencies:** T20, T21, T44, T24.

**Files likely touched:** `app/src/model.rs`, `app/src/platform/windows/bin.rs`, `app/src/platform/windows.rs`, `app/src/ui.rs`, `app/src/session.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T27: Act on selected Recycle Bin items

- [ ] T27 complete

**Description:** Support targeted restore and explicitly confirmed deletion of identified selected bin items only.

**Acceptance criteria:**
- [ ] Restore/delete acts only on revalidated selected Shell identities, never all items by default.
- [ ] Collision/cancel/failure and disappeared-item outcomes are visible; failed items remain selectable.
- [ ] Automated/manual checks cannot empty the global bin or change unrelated recycled entries.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml bin_actions`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_bin_actions -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Restore one owned item, force a collision, and explicitly delete another owned recycled item; verify unrelated entries unchanged.

**Dependencies:** T15, T26.

**Files likely touched:** `app/src/platform/windows/bin.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/tests/native_bin_actions.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Checkpoint CP09: Shell and virtual location gate

- [ ] Custom popup fallback and targeted Recycle Bin actions pass owned-item scenarios.
- [ ] No global bin-emptying or unrelated-item action exists; tested extension compatibility is recorded.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T28: Expand a directory listing

- [ ] T28 complete

**Description:** Offer a cancellable flattened descendant view with an explicit reparse-point policy.

**Acceptance criteria:**
- [ ] Expanded rows retain full authoritative paths and depth without ambiguously acting on duplicate display names.
- [ ] Traversal defaults to not following reparse points; cycles, inaccessible descendants, and cancellation are reported.
- [ ] Partial/complete listing status is visible and results cannot populate a closed or navigated tab.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml expanded_listing`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Expand an owned hierarchy containing duplicate leaf names and a junction; cancel and navigate away.

**Dependencies:** T03, T09, T20, T24.

**Files likely touched:** `app/src/directory.rs`, `app/src/model.rs`, `app/src/ui.rs`, `app/tests/expanded_listing.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T29: Calculate folder statistics

- [ ] T29 complete

**Description:** Add requested cancellable counts/sizes with drive-policy controls and incomplete-result semantics.

**Acceptance criteria:**
- [ ] Counts/sizes distinguish complete, partial, cancelled, and failed scans using the same authoritative paths.
- [ ] SSD/HDD/network policies are documented and do not start unrestricted work when disabled.
- [ ] Concurrency/queues are bounded; refresh/cancellation and reparse points do not double-count or install stale results.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml folder_statistics`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Calculate owned-tree totals, deny one subtree, cancel, and verify a disabled network policy submits no scan.

**Dependencies:** T18, T28.

**Files likely touched:** `app/src/directory.rs`, `app/src/preview.rs`, `app/src/ui/inspector.rs`, `app/src/ui.rs`, `app/tests/folder_statistics.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T30: Browse virtualized icon views

- [ ] T30 complete

**Description:** Add adjustable icon-grid presentation and visible-range thumbnail requests using the existing preview budgets.

**Acceptance criteria:**
- [ ] Grid layout constructs only visible/overscan rows and requests only bounded visible thumbnails.
- [ ] Sizes/view mode preserve tab selection and authoritative entry identity across sorting and resizing.
- [ ] Placeholder/error thumbnails, eviction, zoom, and DPI changes never allocate an unbounded texture cache.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml icon_view`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Scroll a large image fixture, change icon sizes, resize panes, and inspect request/cache instrumentation.

**Dependencies:** T09, T19, T24.

**Files likely touched:** `app/src/ui.rs`, `app/src/ui/icon_view.rs`, `app/src/preview.rs`, `app/src/model.rs`, `app/src/session.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP10: Large listing gate

- [ ] Expanded listings/statistics/icon views pass cancellation, reparse-point and bounded-resource checks.
- [ ] Measured baselines and incomplete-result behavior are reviewed; no hidden row/scan caps.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T31: Preview batch rename plans

- [ ] T31 complete

**Description:** Define reviewed numbering/date rules and a pure proposed-name generator before allowing multiple mutations.

**Acceptance criteria:**
- [ ] Preview lists old/new authoritative paths with explicit extension, date/timezone, numbering-origin, and padding policies.
- [ ] Invalid/reserved names, duplicate proposals, and existing-target conflicts are detected without changing files.
- [ ] Changing selection/options or cancelling discards the plan; no original template syntax is inferred from labels.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml rename_plan`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Preview numbering/date transformations on an owned mixed-extension selection and verify every file remains unchanged.

**Dependencies:** T17, T24.

**Files likely touched:** `app/src/rename.rs`, `app/src/lib.rs`, `app/src/ui/dialogs.rs`, `app/src/model.rs`, `app/tests/rename_plan.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T32: Commit a batch rename plan

- [ ] T32 complete

**Description:** Apply an explicitly confirmed, revalidated plan using per-item outcomes and a documented ordering policy.

**Acceptance criteria:**
- [ ] Source identities/destinations are revalidated and cycle/swap or collision cases are rejected or handled by an explicitly tested policy.
- [ ] Cancellation/partial completion reports each final path; no all-or-nothing rollback promise or overwrite shortcut is made.
- [ ] UI selection/status tracks confirmed changes and leaves failed items discoverable.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml rename_commit`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_rename_commit -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Commit an owned preview, create a destination race, cancel after partial work, and inspect the result ledger.

**Dependencies:** T06, T31.

**Files likely touched:** `app/src/rename.rs`, `app/src/platform/windows/operations.rs`, `app/src/ui/dialogs.rs`, `app/tests/native_rename_commit.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T33: Save named layout presets

- [ ] T33 complete

**Description:** Reuse the validated session DTO to save/load named layouts with overwrite/delete confirmation.

**Acceptance criteria:**
- [ ] A preset restores pane/tab/view/inspector configuration with independent IDs and missing-path handling.
- [ ] Names/overwrite/delete choices cannot replace arbitrary files outside the application state directory.
- [ ] Corrupt/future presets are preserved and failed saves leave the last good preset intact.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml layout_presets`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Save/load two layouts, cancel an overwrite/deletion, and load a preset whose folder no longer exists.

**Dependencies:** T21, T43, T44, T24.

**Files likely touched:** `app/src/session.rs`, `app/src/ui.rs`, `app/src/ui/dialogs.rs`, `app/src/commands.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Checkpoint CP11: Batch and preset gate

- [ ] Rename preview/commit and named layouts pass collision/race/partial and corrupt-save cases.
- [ ] Preview never mutates; non-transactional commit policy and new template rules are accepted.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T34: Navigate Quick Access bookmarks

- [ ] T34 complete

**Description:** Add explicitly managed path bookmarks with grouping and visible unavailable-location behavior.

**Acceptance criteria:**
- [ ] Users add/remove/reorder/group bookmarks and open them in a chosen existing tab or new tab.
- [ ] Persisted bookmarks keep lossless paths; unavailable locations report errors without deleting bookmark data.
- [ ] Keyboard/filter behavior and accessible group expansion work without invoking unimplemented integrations.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml bookmarks`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Bookmark local/UNC fixtures, group/reorder them, then open an unavailable path and restart.

**Dependencies:** T10, T12, T44, T24.

**Files likely touched:** `app/src/model.rs`, `app/src/session.rs`, `app/src/ui.rs`, `app/src/ui/quick_access.rs`, `app/src/commands.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Task T35: Track and clear recent locations

- [ ] T35 complete

**Description:** Add a documented bounded MRU policy and user-controlled persistence/visibility rather than copying unknown score-decay fields.

**Acceptance criteria:**
- [ ] Recents follow the documented new MRU/deduplication/limit policy and never become operation paths via display text.
- [ ] Never/session/persistent modes and explicit Clear behave distinctly without clearing bookmarks or layouts.
- [ ] Unavailable recents remain intelligible; sorting/filtering and restart behavior follow the selected mode.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml recents`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Visit/revisit fixtures, change persistence modes, clear recents, and verify independent bookmark/layout data.

**Dependencies:** T44, T34.

**Files likely touched:** `app/src/model.rs`, `app/src/session.rs`, `app/src/ui/quick_access.rs`, `app/src/commands.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T36: Configure typography and appearance

- [ ] T36 complete

**Description:** Expose implemented font/density/theme/visibility preferences with validation and safe fallback.

**Acceptance criteria:**
- [ ] Font/density/appearance controls affect visible UI and persist; failed font loading falls back without bundling proprietary fonts.
- [ ] Numeric limits and supported theme choices are documented new policies; no inert original options are exposed.
- [ ] Changes preserve selection/focus, contrast, readable row geometry, and mixed-DPI behavior.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml appearance_settings`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Change fonts/density/theme on both panes, try an unavailable font, and rerun contrast/DPI checks.

**Dependencies:** T44, T23, T24.

**Files likely touched:** `app/src/ui.rs`, `app/src/ui/settings.rs`, `app/src/session.rs`, `app/src/model.rs`, `app/src/platform/windows.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP12: Quick Access and appearance gate

- [ ] Bookmarks/recents and actual appearance settings persist with independent clear/reset behavior.
- [ ] Accessibility/DPI are rechecked; unsupported original settings remain unexposed.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T37: Configure listing input behavior

- [ ] T37 complete

**Description:** Add reviewed click/checkbox/hover/typing preferences only where actual input logic exists.

**Acceptance criteria:**
- [ ] Supported input preferences have documented precedence and do not accidentally execute file actions during text editing.
- [ ] Mouse/keyboard/IME and selection tests cover each exposed mode, including folder-only single-click policy if enabled.
- [ ] Settings changes persist and revert to defaults without exposing unsupported mouse-hotkey customization.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml input_settings`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Exercise each exposed click/checkbox/typing mode with files/folders and an active filter editor.

**Dependencies:** T04, T12, T44, T36.

**Files likely touched:** `app/src/ui.rs`, `app/src/selection.rs`, `app/src/ui/settings.rs`, `app/src/session.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T38: Open external filesystem tools

- [ ] T38 complete

**Description:** Add Explorer and terminal actions with checked process launch and safe working-directory handling.

**Acceptance criteria:**
- [ ] Actions target the captured current directory and report unsupported/unavailable executables or paths.
- [ ] Paths are passed through documented APIs/argument quoting, not concatenated into an executable shell command.
- [ ] Cancellation/dismissal has no mutation or registry/PATH side effect.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml external_tools`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_external_tools -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Open Explorer/terminal in owned directories containing spaces/shell metacharacters and inspect the working directory.

**Dependencies:** T12, T24.

**Files likely touched:** `app/src/platform/windows/shell.rs`, `app/src/commands.rs`, `app/src/ui.rs`, `app/tests/native_external_tools.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T39: Create filesystem shortcuts

- [ ] T39 complete

**Description:** Add explicitly requested .lnk creation through Shell APIs, then review whether link drag effects should be enabled.

**Acceptance criteria:**
- [ ] Shortcut target/name/destination are explicit and revalidated; existing shortcuts are never silently replaced.
- [ ] COM resources and failed-save cleanup are owned; invalid/cancelled requests leave no partial output.
- [ ] Link effects stay disabled until supported source/target interoperability tests and command eligibility pass.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml shortcuts`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_shortcuts -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Create/open an owned shortcut, test a collision and cancellation, then separately verify any newly enabled link drag effect.

**Dependencies:** T06, T08, T22, T24.

**Files likely touched:** `app/src/platform/windows/shell.rs`, `app/src/platform/windows/operations.rs`, `app/src/platform/windows/drag_drop.rs`, `app/src/commands.rs`, `app/src/ui/dialogs.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 5 files.

## Checkpoint CP13: Input and platform convenience gate

- [ ] Input modes, external launches and shortcuts pass focus/quoting/collision tests.
- [ ] Any new link drag effect is explicitly approved only after interoperability tests.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Task T40: Permanently delete confirmed selections

- [ ] T40 complete

**Description:** Add a separate irreversible action after explicit scope approval, never as Trash error fallback.

**Acceptance criteria:**
- [ ] A clear confirmation identifies the exact captured selection and permanent nature; cancellation submits no work.
- [ ] Only revalidated items are submitted and partial/cancel/error outcomes are reported without rollback promises.
- [ ] Ignored native tests are confined to owned disposable fixtures and prove Trash remains a different command.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml permanent_delete`. Opt-in native tests: `cargo test --manifest-path app/Cargo.toml native_permanent_delete -- --ignored --test-threads=1`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Approve deletion of owned fixtures, cancel another request, and verify no Trash fallback or unrelated path mutation.

**Dependencies:** T06, T15, T24.

**Files likely touched:** `app/src/platform/windows/operations.rs`, `app/src/commands.rs`, `app/src/ui/dialogs.rs`, `app/tests/native_permanent_delete.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T41: Edit command hotkeys

- [ ] T41 complete

**Description:** Add a validated documented single-chord grammar and conflict-aware editor for implemented typed commands.

**Acceptance criteria:**
- [ ] Unknown commands/keys/modifiers, oversized chords, and conflicting focus contexts are rejected or require an explicit resolution.
- [ ] Bindings preserve reserved text/IME behavior and can be reset without importing original command-table layouts.
- [ ] Saved aliases round-trip and failures preserve the last good configuration; multi-step/mouse bindings remain deferred.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml hotkeys`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml`.
- [ ] Manual: Rebind/reset an eligible action, create a conflict, enter invalid chords, and verify behavior inside text editors.

**Dependencies:** T12, T44, T23, T24.

**Files likely touched:** `app/src/commands.rs`, `app/src/ui/settings.rs`, `app/src/session.rs`, `app/tests/hotkeys.rs`. Tests may be inline in these modules.

**Estimated scope:** Medium, 4 files.

## Task T42: Verify extended capability coverage

- [ ] T42 complete

**Description:** Run the accepted advanced feature scenarios and update the release support matrix instead of declaring untested original parity.

**Acceptance criteria:**
- [ ] All implemented later capabilities pass targeted regression/native/manual scenarios and their dependency tests.
- [ ] Resource limits, accessibility, corrupt state, stale requests, partial mutations, and external interoperability are rechecked together.
- [ ] Unsupported original options/locations/formats and new-policy differences remain explicit in release documentation.

**Verification:**
- [ ] Focused tests: `cargo test --manifest-path app/Cargo.toml extended_acceptance`.
- [ ] Build: `cargo build --manifest-path app/Cargo.toml --release`.
- [ ] Manual: Run the approved advanced workflow matrix on release builds using owned fixtures and available registered Shell extensions.

**Dependencies:** T25, T27, T29, T30, T32, T33, T35, T36, T37, T38, T39, T40, T41.

**Files likely touched:** `app/tests/windows-acceptance.md`, `app/README.md`, `tasks/todo.md`. Tests may be inline in these modules.

**Estimated scope:** Medium, 3 files.

## Checkpoint CP14: Extended release acceptance

- [ ] Permanent deletion, hotkeys and advanced regression checks pass the reviewed scope.
- [ ] Human accepts updated coverage; no claim of exact File Pilot parity or recovered-source licensing.
- [ ] Full tests/build/format/lint pass; human reviews results before proceeding.

## Completion record

For each accepted task, append actual commands/results, native/manual scenarios, limitations, and files changed beside that task or link to a project-owned verification record. Do not mark a task complete based only on an agent summary. Failed/blocked native gates remain unchecked. The user subsequently authorized implementation on 2026-10-13.

### 2026-10-13 foundation verification

- Identity: Wayfarer Files, package wayfarer-files 0.1.0; no persistence enabled yet and no File Pilot state accessed.
- Toolchain: Rust/Cargo 1.95.0; edition 2024; rust-version 1.95. Dependencies pinned to eframe 0.36.2, raw-window-handle 0.6.2, windows 0.62.2. Cargo.lock retained; no version control initialized or commit requested.
- Environment: Windows 11 Pro 10.0.26200 x64; AMD Ryzen 7 7800X3D; NVIDIA GeForce RTX 4070 SUPER and AMD Radeon Graphics.
- PASS: cargo build --manifest-path app/Cargo.toml --offline.
- PASS: cargo fmt --manifest-path app/Cargo.toml -- --check.
- PASS: cargo clippy --manifest-path app/Cargo.toml --offline --all-targets -- -D warnings.
- PASS: cargo test --manifest-path app/Cargo.toml --offline (one native test ignored by default; no pure logic tests yet).
- PASS: cargo test --manifest-path app/Cargo.toml --offline native_runtime -- --ignored --test-threads=1 (one nested-OLE test passed).
- PASS: task -d app --list; all six described tasks present.
- PASS: app/target/debug/wayfarer-files.exe --window-boot-probe, ten separate processes: four UI frames each, real HWND callback, pre-destruction detach, normal exit. A previous ten-run baseline also passed before native adapter integration.
- Caught and fixed: post-event-loop subclass removal failed. AccessKit's window-procedure restoration can bypass a later subclass on teardown. Removal now occurs in eframe App::on_exit before toolkit window teardown; the WM_NCDESTROY path remains a fallback and is not claimed independently verified.
- Files: app/Cargo.toml, app/Cargo.lock, app/Taskfile.yml, app/src/main.rs, app/src/lib.rs, app/src/platform/mod.rs, app/src/platform/windows.rs; task status/authorization updated in tasks/plan.md and this checklist. T01 and T02 were separate integration slices. No reference analysis, FPilot.exe, root specification or root Taskfile was modified or executed.
- Parallel work: main integrator plus native adapter worker and read-only browsing-contract reviewer; peak three agents total, no nested workers.
- BLOCKED interactive acceptance: this session lacks a usable native UI automation surface. No claims for actual text entry, keyboard/focus, Narrator announcements, or mixed-DPI behavior. Run task -d app run, tab between the input and probe button, type/compose text, activate the probe with keyboard/mouse, inspect Narrator labels/focus, move across differently scaled displays, and close/reopen. Record results here before accepting T02 and continuing its dependency chain. No further user approval of implementation scope is needed; actual runtime evidence is missing.
- T03 design review completed without implementation: owned lossless paths, bounded nonblocking request/result queues, generations and cancellation, immutable snapshots, visible error states, and instrumented virtualized rows. T03 and CP01 remain unstarted/unaccepted.
