# Implementation progress

## Current state

- Baseline: S001 accepted; a runnable read-only Rust/eframe/wgpu explorer exists.
- Implementation policy: AGENTS.md. Scope and acceptance contracts: SPEC.md.
- Current checkpoint: 1, read-only explorer.
- Active slice: S002, sorting and recoverable navigation, ready to launch.
- Next supervisor action: launch S002, inspect its diff and focused checks, then accept/commit or classify a blocker.
- Three criteria are implemented with pending manual checks; no criterion is fully verified yet. Partial behavior does not count as a completed criterion.

## Active slice S002

- Behavior: deterministic folders-first column sorting off the UI thread, and back/forward/up/breadcrumb/path navigation that retains the previous usable location/history when a new destination fails.
- Targets: F01.2 and F01.3. Selection-preservation coverage for F01.2 remains pending until selection is implemented. Missing-current-location recovery also contributes to F13.2.
- Write boundary: src/ and README.md; Cargo.toml/Cargo.lock only if a concrete requirement justifies a dependency. The supervisor owns this tracker, AGENTS.md and the spec/design.
- Focused check: deterministic sort/tie/unknown-value checks plus navigation history, failed-destination recovery and stale completion checks; formatting and incremental compilation.
- Exclusions: filesystem mutations, previews, tabs/splits, finder, full selection/filtering, exhaustive UI automation and benchmarks.
- Handoff: changed paths, exact checks/results, complete versus partial IDs, remaining risks, and next useful slice. Worker does not commit or change criterion statuses.

## Accepted slices

- S001: native read-only directory browser, background progressive generation-safe loading, virtualized table and timestamp/error display. Supervisor inspected all source and reran `cargo fmt --check`, `cargo check`, seven directory tests and the headless table-geometry regression: eight checks passed. Worker demonstrated native startup and normal shutdown. Visual interaction, real ACL/unavailable-drive and actual slow-device checks remain pending. No mutations are enabled.

## Blockers and pending manual checks

- Native bootstrap/build succeeded. No current implementation blocker.
- Supervisor attempted a disposable-fixture window capture, but Windows did not grant foreground access. Capture was refused before reading any desktop pixels; the helper closed its own process and removed its fixture. Visual acceptance remains pending rather than delaying unrelated work.
- Windows GUI smoke, Narrator, real clipboard interoperability, cross-volume operations, and clean-machine packaging will be checked at their relevant checkpoints, not declared passed in advance.

## Acceptance tracker

Statuses: Not started, In progress, Implemented, Verified, Blocked. Evidence must distinguish implemented portions from pending checks.

| Criterion | Status | Requirement summary | Evidence / remaining work |
| --- | --- | --- | --- |
| F01.1 | Implemented | Opening an accessible folder shows its immediate entries with name, type, modified time, and size for regular files | S001: native paths, immediate-entry metadata, UTC time and unknown folder size; disposable metadata and headless geometry checks passed. Actual visual/interaction smoke pending. |
| F01.2 | In progress | Folders sort before files | S002 planned: background column sorting and deterministic ties; path-based selection preservation will be exercised after selection exists. |
| F01.3 | In progress | Back, forward, up, breadcrumbs, and an editable full path navigate the active tab only | S001 supplies editable path and folder activation only. S002 adds history/up/breadcrumbs and failed-destination recovery; multiple-tab isolation later. |
| F01.4 | Implemented | Empty folders, loading, access denied, missing folders, and unavailable drives have distinct visible states with retry/navigation available. | S001: explicit UI states and retry; real empty/missing/not-folder and synthetic denied/device checks pass. Real ACL/drive and interactive UI verification pending. |
| F01.5 | Implemented | Directory enumeration and metadata loading run away from the UI thread | S001: two workers, coalesced requests, bounded batches and generation guards; cancellation/stale/latest-request tests pass. Actual slow/uninterruptible-device smoke pending. |
| F01.6 | Not started | Unicode names and supported long paths work without lossy conversions |  |
| F01.7 | Not started | Double-click or Enter opens a folder in the current tab, or opens a regular file using the OS default application | S001 has folder activation only; full criterion awaits OS-default regular-file opening and keyboard/selection integration. |
| F02.1 | Not started | Click selects one entry; Ctrl+click toggles entries; Shift+click selects a range in the current displayed order |  |
| F02.2 | Not started | Arrow keys, Home, End, Page Up, and Page Down move the active row and keep it visible |  |
| F02.3 | Not started | Selection belongs to the tab |  |
| F02.4 | Not started | Commands operate on an immutable snapshot of selected paths taken when invoked |  |
| F02.5 | Not started | Explorer shortcuts do not run while typing in the finder, filter, path editor, or rename field |  |
| F02.6 | Not started | Refresh removes vanished selections while preserving surviving selected paths, rather than preserving row numbers. |  |
| F03.1 | Not started | Ctrl+T or the plus button creates a tab at the current location |  |
| F03.2 | Not started | Returning to a tab restores its location, history, filter, sort, selection, and scroll position |  |
| F03.3 | Not started | Tabs can be reordered within a pane and moved between panes by dragging |  |
| F03.4 | Not started | Closing the last tab of a non-final pane closes that pane |  |
| F03.5 | Not started | Tab labels remain usable when many tabs are open, with overflow scrolling or an overflow menu |  |
| F04.1 | Not started | A layout action splits the focused pane left/right or top/bottom |  |
| F04.2 | Not started | Dropping a tab on a pane edge creates a split and moves that tab into it |  |
| F04.3 | Not started | Dividers resize neighboring panes |  |
| F04.4 | Not started | Clicking a pane or using F6 changes keyboard focus |  |
| F04.5 | Not started | Closing a pane removes that pane's tabs without cancelling already-started filesystem operations |  |
| F04.6 | Not started | Narrow panes drop lower-priority metadata columns before losing usable file names |  |
| F05.1 | Not started | Ctrl+O opens a focused input over the existing layout and captures the target pane/tab |  |
| F05.2 | Not started | With an empty query, show recent locations, bookmarks, storage roots, and common places |  |
| F05.3 | Not started | Accept an explicit absolute path as well as a path relative to the target tab |  |
| F05.4 | Not started | Up/Down selects a result; Enter navigates the target tab; Shift+Enter opens a tab in its pane; Ctrl+Enter opens a new split pane; Escape dismisses and restores browsing focus. |  |
| F05.5 | Not started | Missing or inaccessible results display an error and leave the finder usable |  |
| F05.6 | Not started | Users can bookmark/unbookmark a location and remove a recent location |  |
| F05.7 | Not started | This is navigation over known locations and explicit path expansion, not an implicit full-drive index or recursive content search. |  |
| F06.1 | Not started | Typing in the filter performs case-insensitive fuzzy matching against names in the current directory |  |
| F06.2 | Not started | Show displayed/total counts and a no-match state |  |
| F06.3 | Not started | File commands use only the visible selection |  |
| F07.1 | Not started | With preview enabled, selecting one supported image displays it in that pane |  |
| F07.2 | Not started | Fit, 100%, zoom, and pan work without distorting the aspect ratio |  |
| F07.3 | Not started | Decode/resize away from the UI thread |  |
| F07.4 | Not started | Malformed files, oversized decoded images, and unavailable files report an error without losing browsing state |  |
| F07.5 | Not started | Multiple selected entries show a selection summary, not an arbitrary file preview |  |
| F07.6 | Not started | Preview memory is bounded by a documented cache/output budget |  |
| F08.1 | Not started | Selecting a supported PDF displays its first page with file name, page count, current page, and zoom/fit controls |  |
| F08.2 | Not started | Previous/next page and direct page selection stay within document bounds |  |
| F08.3 | Not started | Render at the preview's physical pixel size, including display scaling |  |
| F08.4 | Not started | Missing renderer, malformed documents, encrypted documents requiring a password, and rendering failures show explicit fallback states |  |
| F08.5 | Not started | Read/render/cache limits apply to PDFs too |  |
| F08.6 | Not started | The packaged application includes the required native renderer or documented installation support |  |
| F08.7 | Not started | Text selection, document search, annotations, forms, and link interaction are not first-release requirements. |  |
| F09.1 | Not started | Create a folder or empty file in the active location using explicit commands |  |
| F09.2 | Not started | F2 starts an inline rename for one selected entry |  |
| F09.3 | Not started | Validate names against the target filesystem, including separators, reserved Windows names, trailing dots/spaces, and empty names |  |
| F09.4 | Not started | Name collisions and case-only renames follow actual filesystem behavior |  |
| F09.5 | Not started | Permission, locking, disappearance, and other errors leave existing data intact |  |
| F09.6 | Not started | Batch rename is deferred |  |
| F10.1 | Not started | Ctrl+C copies and Ctrl+X cuts a snapshot of selected paths; Ctrl+V pastes into the focused tab's current folder |  |
| F10.2 | Not started | Copy/move supports regular files, non-empty folders, multiple selected entries, and different local volumes |  |
| F10.3 | Not started | Reject a source/destination that refers to the same entry and a destination inside a source directory, including equivalent paths resolved through aliases/reparse points. |  |
| F10.4 | Not started | Each conflicting entry requires Skip, Keep both, Replace, or Cancel |  |
| F10.5 | Not started | Keep both generates an unused name while preserving file extensions |  |
| F10.6 | Not started | Replace is initially for regular files only |  |
| F10.7 | Not started | Write copied regular files to an operation-owned temporary destination, then commit after successful read/write/flush/close |  |
| F10.8 | Not started | Same-volume move uses filesystem rename where possible |  |
| F10.9 | Not started | A failure or cancellation during folder transfer leaves completed items intact and reports partial completion |  |
| F10.10 | Not started | Revalidate sources before processing |  |
| F10.11 | Not started | Never recursively follow symlinks/junctions/reparse points during transfer |  |
| F10.12 | Not started | Preserve file contents and directory structure |  |
| F10.13 | Not started | After partial moves, keep unresolved source paths available for retry and avoid re-moving completed items |  |
| F11.1 | Not started | Delete sends selected entries to the Windows Recycle Bin through the OS facility, subject to confirmation showing item count and location |  |
| F11.2 | Not started | Shift+Delete requests permanent deletion and requires a distinct confirmation explicitly warning that the entries will not go to the Recycle Bin |  |
| F11.3 | Not started | If trash is unavailable for a location, report that limitation and leave the entries intact unless the user separately requests and confirms permanent deletion. |  |
| F11.4 | Not started | Confirmed deletion operates on the captured paths, revalidated before processing |  |
| F11.5 | Not started | Deleting a link removes the link, not its target |  |
| F11.6 | Not started | Refuse deletion of filesystem/volume roots, the Windows directory and its descendants, and Program Files directory roots |  |
| F11.7 | Not started | Deleted entries disappear from affected tabs; failed entries remain visible |  |
| F12.1 | Not started | A window-level operations drawer shows operation type, source/destination, current item, known progress, and completed/skipped/failed counts |  |
| F12.2 | Not started | Enumeration or unknown totals use indeterminate progress rather than invented percentages |  |
| F12.3 | Not started | Start with one mutating operation at a time |  |
| F12.4 | Not started | Cancellation is cooperative |  |
| F12.5 | Not started | Reports identify each unresolved item and the actual error, with a retry that does not repeat destructive completed steps |  |
| F12.6 | Not started | Closing the application with active operations prompts to keep running or cancel and wait |  |
| F12.7 | Not started | After a crash, never auto-delete sources, auto-retry destructive work, or auto-remove unrelated temporary files |  |
| F13.1 | Not started | F5 refreshes the focused tab |  |
| F13.2 | Not started | External creates, renames, and deletes update visible affected locations |  |
| F13.3 | Not started | Operation completions reconcile every affected open tab |  |
| F13.4 | Not started | Persist window geometry, split layout, tabs/locations, active tabs, focused pane, bookmarks, and recents |  |
| F13.5 | Not started | Settings save atomically |  |
| F13.6 | Not started | Windows clipboard copy/cut/paste interoperates with Windows Explorer for file paths and intended action |  |
| F13.7 | Not started | A minimal context menu provides Open, Copy, Cut, Paste, Rename, Trash, and Properties/basic details with valid enablement |  |
| F13.8 | Not started | Dragging selected entries between panes requests Copy by default or Move with Shift |  |
| F13.9 | Not started | Inbound external file drops copy into the targeted location after validation and conflict handling |  |
| F14.1 | In progress | Use virtualized rows | S001: show_rows and background filesystem reads; UI update has no I/O/sorting. Remaining sort/filter/preview paths must also satisfy this criterion as implemented. |
| F14.2 | Not started | On a recorded reference Windows machine, a generated 20,000-entry local directory remains navigable during enumeration |  |
| F14.3 | Not started | On the same benchmark, ordinary scrolling reaches at least 30 frames/sec and preview caches stay within their documented budget |  |
| F14.4 | Not started | At 100%, 150%, and 200% display scaling, controls and dialogs do not overlap or clip their actions |  |
| F14.5 | Not started | All essential actions work by keyboard with visible focus |  |
| F14.6 | Not started | Access denied, malformed preview input, clipboard errors, and ordinary I/O failures do not panic the application |  |
