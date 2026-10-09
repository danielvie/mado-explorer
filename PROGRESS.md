# Implementation progress

## Current state

- Baseline: S001-S006b accepted; read-only browsing core and R001 corrections are complete for the single-context scope. Native capture confirms readable Japanese names and sort feedback.
- Implementation policy: AGENTS.md. Scope and acceptance contracts: SPEC.md.
- Current checkpoint: 2, workspace. Checkpoint 1 implementation/review gates passed; remaining platform/manual items stay explicit and do not block independent workspace implementation.
- Active slice: S007, independent tabs in one pane, ready to launch.
- Next supervisor action: launch S007, inspect independent-state and stale-response tests, commit accepted progress, then add split panes.
- Ten criteria are implemented and one is verified. Remaining UI/OS/manual checks and cross-stage clauses are still pending.

## Active slice S007

- Behavior: tab strip, new/close/cycle shortcuts, overflow handling and separate navigation/selection/filter/sort/scroll state for each tab in the current pane.
- Targets: F03.1, F03.2, the final-window-tab part of F03.4, F03.5, F02.3 tab isolation and F06.2 per-tab filters. Pane/cross-pane clauses are completed with splits.
- Write boundary: worker owns relevant src/ and README.md only. The supervisor owns tracker/spec/glossary/design/AGENTS/Taskfile and acceptance/commits.
- Focused check: state round trips between tabs, same-location UI-ID isolation, late/closed-tab response rejection, final-tab home behavior, overflow and editor-owned shortcut guards. Preserve actual saved scroll rather than jumping to the selected row after activation/refresh.
- Resource contract: background services remain bounded independently of tab count; fonts and explicit file launcher remain window-owned. Do not add thread pools or a directory worker per tab.
- Exclusions: split tree, cross-pane/tab-edge dragging, filesystem operations, persistence, previews, finder, registry changes and exhaustive UI automation.
- Handoff: changed paths, exact checks/results, complete versus partial IDs, remaining risks, and next useful slice. Worker does not commit or change criterion statuses.

## Accepted slices

- S001: native read-only directory browser, background progressive generation-safe loading, virtualized table and timestamp/error display. Supervisor inspected all source and reran `cargo fmt --check`, `cargo check`, seven directory tests and the headless table-geometry regression: eight checks passed. Worker demonstrated native startup and normal shutdown. Visual interaction, real ACL/unavailable-drive and actual slow-device checks remain pending. No mutations are enabled.

- S002: worker-built shared snapshots and background deterministic column sorting, transactional back/forward/up/breadcrumb/path navigation, and focused-editor shortcut gating. Supervisor inspected source and reran 17 affected checks successfully; existing table regression had passed in the worker's final 18-test run. Inspection found and fixed a drive-root breadcrumb bug: prefix-only drive-relative paths such as C: are not exposed as root destinations. Navigation/editor tests and compilation were rerun after that fix. GUI/manual and selection-preservation checks remain pending.

- S003: whole-row native-path click/toggle/range/Ctrl+A selection, separate active/anchor identities, keyboard first/last/page movement and virtualized scrolling, successful-refresh pruning and failed-navigation preservation. Supervisor inspected production source and reran 15 distinct affected tests successfully, plus formatting/compilation. Pending-destination rows pause selection/activation; no mutations or file launches are enabled. Native interaction/scaling/Narrator and linear-lookup performance measurements remain pending.

- S004: stdlib Unicode-lowercase subsequence name filtering on the projection worker, query/completion revision guards, fresh counts/no-match/Clear UI and visible-path selection reconciliation. Supervisor approved pausing stale row commands and nonempty-filter incomplete/failed listings while leaving navigation/retry/filter controls available. Supervisor inspected source and reran 19 distinct affected checks plus formatting/compile checks successfully. Filter survives ordinary navigation/refresh. Per-tab clauses, future mutating-command integration, native interaction and measured performance remain pending.

- S005: fresh visible explicit activation captures native paths; regular files launch on one bounded background worker through ShellExecuteExW with STA/COM cleanup, no shell interpolation, no runas and no automatic queue/retry. Native preflight rejects missing/nonregular/reparse inputs. Supervisor inspected all launch code, dependency/API wiring and status/routing, and reran 16 affected tests plus formatting/compile checks successfully. Corrected busy-refusal wording so it remains accurate after the earlier launch finishes. Actual associations, shell extensions and long-path launches remain manually pending; external apps may write/execute independently of the explorer.

- S006: worker-side hidden/system flags and revisioned visibility, distinct malformed-native diagnostics, drive-relative/device input rejection, Unicode/native-path fixtures and verbatim paths beyond 260 UTF-16 units. Supervisor inspected source and changed metadata reads to documented non-following DirEntry::metadata, which reuses cached enumeration data on Windows; all 51 tests and native build passed. Replaced generated emoji fixtures with supplementary non-emoji Unicode coverage. Own-window startup/normal shutdown passed, and DPI-aware PrintWindow capture inspected the full layout and sample metadata. Native capture exposed Japanese/sort-marker missing glyphs, so F01.6 is not considered complete yet.

- S006a: bounded fixed installed-font loading before native startup, builtin-first families, missing/invalid-asset fallback and ASCII sort markers. Independent reviewer verified constructor parsing against installed epaint source and found no new security/data-loss blocker. Supervisor ran all 54 tests/build and inspected an own-window capture with readable Japanese names, correct metadata and clear sort feedback. R001 P1 is a real older navigation bug: after successful .. navigation, lexical Up can descend. R001 P2 exposed an unnecessary optional-font default-test prerequisite; host coverage now runs explicitly via task test:fonts with original assertions. Native associations, broader scripts/scaling/Narrator and performance remain pending.

- S006b / R001 P1 closed: enumeration workers normalize ordinary Windows paths with native full-path behavior, return effective destinations only on successful completion, and build row paths from that same location. Real disposable parent-then-Up, absolute-dot, same-location alias/history/selection and stale/failure regressions pass. Native Unicode/UNC/verbatim representations are preserved; actual unsupported verbatim-dot cases fail explicitly without changing prior state. Supervisor inspected the correction and ran formatting/check, all default checks (56 passed, one explicit reference-host font probe ignored) and native build successfully. R001 P2 remains resolved by separately passing task test:fonts with unchanged assertions. No new dependency or physical/junction-identity promise was introduced.

## Blockers and pending manual checks

- No current implementation blocker. R001 P1 normalization and P2 portable-font-test findings are addressed and recorded. Physical/junction identity, live network-provider behavior, real associations and other later-stage/manual checks remain outside current verification claims.
- The confirmed Japanese/sort-marker rendering defect is fixed and observed in native capture on the reference host. Optional installed fonts still vary by machine, with explicit warnings and an opt-in coverage probe.
- Native own-window printing now provides a safe visual check without foreground access. The earlier foreground capture was refused before any desktop read. Milestone capture corrected harness title timing and DPI virtualization; these were wrapper issues, not app failures. Fixtures/processes were cleaned up.
- Windows GUI smoke, Narrator, real clipboard interoperability, cross-volume operations, and clean-machine packaging will be checked at their relevant checkpoints, not declared passed in advance.

## Acceptance tracker

Statuses: Not started, In progress, Implemented, Verified, Blocked. Evidence must distinguish implemented portions from pending checks.

| Criterion | Status | Requirement summary | Evidence / remaining work |
| --- | --- | --- | --- |
| F01.1 | Implemented | Opening an accessible folder shows its immediate entries with name, type, modified time, and size for regular files | S001: native paths, immediate-entry metadata, UTC time and unknown folder size; disposable metadata and headless geometry checks passed. Actual visual/interaction smoke pending. |
| F01.2 | Implemented | Folders sort before files | S002-S003: background all-column/direction/tie/unknown ordering plus native-path selection/active/anchor preservation and reordered-range regressions pass. Interactive header/selection smoke pending. |
| F01.3 | Implemented | Back, forward, up, breadcrumbs, and an editable full path navigate the active tab only | S006b closes R001 relative-parent/Up bug through successful effective-path commits and alias-safe history/selection, with real background/native-prefix regressions. Broader native interaction and tab isolation checks continue in S007. |
| F01.4 | Implemented | Empty folders, loading, access denied, missing folders, and unavailable drives have distinct visible states with retry/navigation available. | S001: explicit UI states and retry; real empty/missing/not-folder and synthetic denied/device checks pass. Real ACL/drive and interactive UI verification pending. |
| F01.5 | Verified | Directory enumeration and metadata loading run away from the UI thread | R001/source inspection confirms all enumeration/metadata/normalization call sites are worker-owned; actual threaded disposable loading, stale-generation, cancellation/latest-request and final normalized-completion regressions pass. No slow-provider cancellation or measured performance claim; those platform/performance checks remain separate. |
| F01.6 | Implemented | Unicode names and supported long paths work without lossy conversions | S006 native/hidden/diagnostic/long-path fixtures and S006a bounded installed-font fallback pass. Native capture shows readable Japanese names; broader glyph/provider/association/scaling checks remain pending. Font absence is explicitly warned; full reference-host coverage remains a separate recorded probe. |
| F01.7 | Implemented | Double-click or Enter opens a folder in the current tab, or opens a regular file using the OS default application | S005: folder routing and bounded ShellExecuteExW file launch use captured fresh visible native paths, exact editor/control/repeat guards and path-specific OS errors. Six new activation/launch checks and existing guards pass. Real associated-file/extension/long-path/native GUI verification pending. |
| F02.1 | Implemented | Click selects one entry; Ctrl+click toggles entries; Shift+click selects a range in the current displayed order | S003: whole-row click/toggle/range/all, native non-Unicode identity and metadata/gap pointer regressions pass. Native GUI interaction pending; filtering integrates in S004. |
| F02.2 | Implemented | Arrow keys, Home, End, Page Up, and Page Down move the active row and keep it visible | S003: boundaries/pages/Shift and active-row scroll targets pass state/headless checks at small and normal viewports. Native keyboard/visual verification pending. |
| F02.3 | In progress | Selection belongs to the tab | S007 planned: independent native-path selection per tab; pane isolation follows with splits. |
| F02.4 | Not started | Commands operate on an immutable snapshot of selected paths taken when invoked |  |
| F02.5 | In progress | Explorer shortcuts do not run while typing in the finder, filter, path editor, or rename field | S002-S004: path/filter TextEdit and submission-frame ownership, exact modifiers, Escape filter-focus exit and Enter versus other-control focus are regression-tested. Finder/rename later. |
| F02.6 | Implemented | Refresh removes vanished selections while preserving surviving selected paths, rather than preserving row numbers. | S003: reconcile raw native paths only on successful current-generation completion; progressive/failed/stale results preserve identities. Survivor/disappearance/empty-folder tests pass; native refresh smoke pending. |
| F03.1 | In progress | Ctrl+T or the plus button creates a tab at the current location | S007 planned: create/close/cycle in the current pane; input ownership preserved. |
| F03.2 | In progress | Returning to a tab restores its location, history, filter, sort, selection, and scroll position | S007 planned: retained tab state plus refreshed contents without stale response leakage or scroll-to-active overriding saved manual position. |
| F03.3 | Not started | Tabs can be reordered within a pane and moved between panes by dragging |  |
| F03.4 | In progress | Closing the last tab of a non-final pane closes that pane | S007 implements final-window-tab replacement at home; non-final-pane collapse follows with splits. |
| F03.5 | In progress | Tab labels remain usable when many tabs are open, with overflow scrolling or an overflow menu | S007 planned: usable overflow and full-location hover, stable tab UI IDs. |
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
| F06.1 | Implemented | Typing in the filter performs case-insensitive fuzzy matching against names in the current directory | S004: worker-side Unicode-lowercase subsequence name matching, empty/clear restoration without navigation and case/Unicode/name-only/order regressions pass. Literal whitespace; full locale folding/normalization is not promised. Native interaction pending. |
| F06.2 | In progress | Show displayed/total counts and a no-match state | S004 implements fresh displayed/total counts, incomplete annotations and explicit no-match state with UI regressions. Independent per-tab filter retention awaits tabs; native verification pending. |
| F06.3 | In progress | File commands use only the visible selection | S004-S005: hidden/active/anchor pruning, visible-only Ctrl+A and file/folder activation, captured-path snapshots and no resurrection on Clear are tested. Future mutating commands must share rows_selectable and capture current visible paths. |
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
| F14.1 | In progress | Use virtualized rows | S001-S004: virtualized rows; worker enumeration/metadata/query normalization/matching/vector work/sorting/visible-path-set construction with generation/data/query/sort keys. Listings are not capped; preview paths and measurements pending. |
| F14.2 | Not started | On a recorded reference Windows machine, a generated 20,000-entry local directory remains navigable during enumeration |  |
| F14.3 | Not started | On the same benchmark, ordinary scrolling reaches at least 30 frames/sec and preview caches stay within their documented budget |  |
| F14.4 | Not started | At 100%, 150%, and 200% display scaling, controls and dialogs do not overlap or clip their actions |  |
| F14.5 | Not started | All essential actions work by keyboard with visible focus |  |
| F14.6 | Not started | Access denied, malformed preview input, clipboard errors, and ordinary I/O failures do not panic the application |  |
