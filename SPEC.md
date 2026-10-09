# Mado Explorer specification

Status: proposed. This document defines the first usable release and its acceptance criteria. It does not authorize changing user files during development or claim that the application exists yet.

## Goal and assumptions

Build a Rust desktop explorer with the compact File Pilot-inspired layout already designed: directory tables, independent tabs and split panes, a Ctrl+O folder finder, image/PDF previews, and safe filesystem operations.

Proposed starting assumptions:

- Windows 11 first. Keep filesystem and UI logic separate, but do not promise Linux/macOS support in the first release.
- Use eframe/egui with its wgpu backend. Custom wgpu drawing is not needed initially.
- Operate with the user's existing permissions. No elevation or administrator requirement.
- Local files and mounted filesystem paths only. A slow or unavailable network/removable location must not block the UI.
- Single application window. Multiple panes and tabs inside it.
- PDF previews display rasterized pages, not a complete PDF reader.
- Destructive commands require explicit user action. Nothing modifies files merely because a folder is browsed or a file is previewed.

Confirm these assumptions before implementation. In particular, confirm whether first-release Linux/macOS support or outbound drag-and-drop is required; either changes the integration work.

## Design and terminology

The editable visual reference is [file-explorer-layout.excalidraw](C:/SANDBOX/REPOS/mado-explorer.git/design/file-explorer-layout.excalidraw). The three views show browsing with a PDF preview, the folder finder, and split panes with an image preview.

[CONTEXT.md](C:/SANDBOX/REPOS/mado-explorer.git/CONTEXT.md) defines the vocabulary. A tab owns location, history, selection, filter, sort, and scroll position. A pane owns its tabs and preview visibility/width. A preview follows that pane's active tab, never a selection in another pane.

Cyan indicates focus or selection; focus also has a visible border/indicator so color is not the only signal. The sidebar starts collapsed. Long names truncate visually, but commands always use the complete path.

## Acceptance rules

- Every criterion has a stable ID. Implementation checkpoints must cite the IDs they satisfy.
- Filesystem behavior needs automated tests using disposable directories. UI and OS integration also need reproducible manual checks where automation cannot establish correctness.
- A feature is not accepted solely because a control is drawn. Its normal, empty, failure, and stale-result cases must work where relevant.
- Data-loss safeguards are release blockers. Performance targets are proposed measurement gates, not existing results.
- No tests run destructive operations on the real Downloads, Documents, or other user folders.

## Required features

### F01. Directory browsing

- F01.1 Opening an accessible folder shows its immediate entries with name, type, modified time, and size for regular files. Folder size is unknown, not fabricated as zero.
- F01.2 Folders sort before files. Clicking a supported column toggles ascending/descending sorting, with a deterministic tie-breaker. Sorting/filtering does not change which paths are selected.
- F01.3 Back, forward, up, breadcrumbs, and an editable full path navigate the active tab only. Failed navigation leaves the previous location and history usable.
- F01.4 Empty folders, loading, access denied, missing folders, and unavailable drives have distinct visible states with retry/navigation available.
- F01.5 Directory enumeration and metadata loading run away from the UI thread. A previous folder's late results cannot populate the newly opened folder.
- F01.6 Unicode names and supported long paths work without lossy conversions. Hidden/system entries can be shown or hidden; links/reparse points are visibly distinguished.
- F01.7 Double-click or Enter opens a folder in the current tab, or opens a regular file using the OS default application. Failure to launch is reported without changing the file.

### F02. Selection and keyboard control

- F02.1 Click selects one entry; Ctrl+click toggles entries; Shift+click selects a range in the current displayed order. Ctrl+A selects all currently displayed entries.
- F02.2 Arrow keys, Home, End, Page Up, and Page Down move the active row and keep it visible. Shift extends selection. A keyboard-only user can navigate and select without a mouse.
- F02.3 Selection belongs to the tab. Switching tabs or focusing another pane does not overwrite it.
- F02.4 Commands operate on an immutable snapshot of selected paths taken when invoked. Capture entry identity where supported for destructive operations; reject a replaced source before rename, move, or delete. Later selection, sorting, or navigation does not retarget an operation.
- F02.5 Explorer shortcuts do not run while typing in the finder, filter, path editor, or rename field. Escape exits the current transient UI before affecting browsing.
- F02.6 Refresh removes vanished selections while preserving surviving selected paths, rather than preserving row numbers.

### F03. Tabs

- F03.1 Ctrl+T or the plus button creates a tab at the current location. Ctrl+W closes the focused pane's active tab. Ctrl+Tab and Ctrl+Shift+Tab cycle its tabs.
- F03.2 Returning to a tab restores its location, history, filter, sort, selection, and scroll position. Contents refresh against the filesystem.
- F03.3 Tabs can be reordered within a pane and moved between panes by dragging. Moving a tab preserves its browsing state.
- F03.4 Closing the last tab of a non-final pane closes that pane. Closing the final tab creates a fresh tab at the home location instead of leaving an unusable window.
- F03.5 Tab labels remain usable when many tabs are open, with overflow scrolling or an overflow menu. Full locations are available on hover.

### F04. Split panes and focus

- F04.1 A layout action splits the focused pane left/right or top/bottom. The new pane starts with a tab at the source pane's current location.
- F04.2 Dropping a tab on a pane edge creates a split and moves that tab into it. Dropping on a tab strip moves the tab without creating a split.
- F04.3 Dividers resize neighboring panes. Minimum pane sizes prevent controls from overlapping; impossible additional splits are refused visibly.
- F04.4 Clicking a pane or using F6 changes keyboard focus. The focused pane is clearly indicated; navigation, finder, and file commands target it.
- F04.5 Closing a pane removes that pane's tabs without cancelling already-started filesystem operations. The remaining layout expands to fill the available area.
- F04.6 Narrow panes drop lower-priority metadata columns before losing usable file names. Preview and pane dividers are separate controls.

### F05. Ctrl+O folder finder

- F05.1 Ctrl+O opens a focused input over the existing layout and captures the target pane/tab. Opening or dismissing it does not navigate.
- F05.2 With an empty query, show recent locations, bookmarks, storage roots, and common places. Typing ranks fuzzy matches across their names and paths, with deterministic ordering and visible match emphasis.
- F05.3 Accept an explicit absolute path as well as a path relative to the target tab. Tab expands/completes a selected folder's children without first navigating the underlying tab.
- F05.4 Up/Down selects a result; Enter navigates the target tab; Shift+Enter opens a tab in its pane; Ctrl+Enter opens a new split pane; Escape dismisses and restores browsing focus.
- F05.5 Missing or inaccessible results display an error and leave the finder usable. No-match results are explicit. Late queries cannot replace newer results.
- F05.6 Users can bookmark/unbookmark a location and remove a recent location. Bookmarks and recents survive restart.
- F05.7 This is navigation over known locations and explicit path expansion, not an implicit full-drive index or recursive content search.

### F06. Current-folder filter

- F06.1 Typing in the filter performs case-insensitive fuzzy matching against names in the current directory. Clearing it restores the entries without navigation.
- F06.2 Show displayed/total counts and a no-match state. Each tab retains its own filter.
- F06.3 File commands use only the visible selection. Entries hidden by a filter change are deselected; Ctrl+A never silently includes hidden entries.

### F07. Image previews

- F07.1 With preview enabled, selecting one supported image displays it in that pane. Initially support PNG, JPEG, and WebP; unsupported types get a clear fallback.
- F07.2 Fit, 100%, zoom, and pan work without distorting the aspect ratio. Honor image orientation metadata where supported; dimensions and size are visible.
- F07.3 Decode/resize away from the UI thread. Changing selection replaces the pending preview; stale work cannot display the wrong file.
- F07.4 Malformed files, oversized decoded images, and unavailable files report an error without losing browsing state. Validate dimensions and allocation limits before allocating output buffers. Initial limits are 256 MiB for application-owned CPU preview buffers and 128 MiB for GPU preview textures across the window; reduce resolution or reject work that cannot fit. These do not cap the PDF engine's internal memory or total process memory.
- F07.5 Multiple selected entries show a selection summary, not an arbitrary file preview. Folders show basic information rather than initiating an unrequested recursive scan.
- F07.6 Preview memory is bounded by a documented cache/output budget. Eviction and navigation release resources; preview never writes to source files.

### F08. PDF previews

- F08.1 Selecting a supported PDF displays its first page with file name, page count, current page, and zoom/fit controls. Render through a separate PDF engine, initially evaluate PDFium.
- F08.2 Previous/next page and direct page selection stay within document bounds. Changing document resets the current page appropriately.
- F08.3 Render at the preview's physical pixel size, including display scaling. After zoom or resize, replace the scaled old bitmap with a sharper render without blocking input.
- F08.4 Missing renderer, malformed documents, encrypted documents requiring a password, and rendering failures show explicit fallback states. Password entry is deferred; never pretend an unreadable document has no pages.
- F08.5 Read/render/cache limits apply to PDFs too. Late page/document results cannot overwrite the current preview. Documents are read-only and active content is not executed.
- F08.6 The packaged application includes the required native renderer or documented installation support. Verify redistribution licenses before selecting the final dependency.
- F08.7 Text selection, document search, annotations, forms, and link interaction are not first-release requirements.

### F09. Create and rename

- F09.1 Create a folder or empty file in the active location using explicit commands. Existing entries are never overwritten by creation.
- F09.2 F2 starts an inline rename for one selected entry. Enter commits; Escape cancels without touching disk. Preserve the extension unless the user intentionally edits it.
- F09.3 Validate names against the target filesystem, including separators, reserved Windows names, trailing dots/spaces, and empty names. Validation failure keeps the editor open with an explanation.
- F09.4 Name collisions and case-only renames follow actual filesystem behavior. Case-only rename succeeds where supported, without overwriting another entry or stranding a temporary name on failure.
- F09.5 Permission, locking, disappearance, and other errors leave existing data intact. Successful creation/rename appears in affected tabs and preserves/selects the intended entry.
- F09.6 Batch rename is deferred. Multiple selection must not accidentally rename only an unspecified member.

### F10. Copy and move

- F10.1 Ctrl+C copies and Ctrl+X cuts a snapshot of selected paths; Ctrl+V pastes into the focused tab's current folder. Cutting alone does not modify the filesystem.
- F10.2 Copy/move supports regular files, non-empty folders, multiple selected entries, and different local volumes. Exclude duplicate sources and redundant descendants of selected parent folders.
- F10.3 Reject a source/destination that refers to the same entry and a destination inside a source directory, including equivalent paths resolved through aliases/reparse points.
- F10.4 Each conflicting entry requires Skip, Keep both, Replace, or Cancel. Apply-to-remaining is scoped to this operation and compatible conflict types, never a permanent global preference. Default is non-destructive.
- F10.5 Keep both generates an unused name while preserving file extensions. A destination created by another process during transfer is treated as a new conflict, not overwritten silently.
- F10.6 Replace is initially for regular files only. Never recursively replace or merge an existing directory as an implicit side effect; folder/type conflicts support Skip, Keep both, or Cancel.
- F10.7 Write copied regular files to an operation-owned temporary destination, then commit after successful read/write/flush/close. Failure or cancellation preserves an existing destination; temporary partial files are cleaned up or reported for recovery if cleanup fails.
- F10.8 Same-volume move uses filesystem rename where possible. Cross-volume move copies first and removes each source only after its destination commits. If source removal fails, report that both copies remain.
- F10.9 A failure or cancellation during folder transfer leaves completed items intact and reports partial completion. It does not delete unprocessed sources or claim whole-operation atomicity/rollback.
- F10.10 Revalidate sources before processing. If a source change during transfer is detected, do not silently report a consistent snapshot or remove the changed source. Concurrent-writer snapshot guarantees are out of scope.
- F10.11 Never recursively follow symlinks/junctions/reparse points during transfer. Initial copy of such entries reports unsupported and skips them rather than copying their targets. Same-volume moves may move the link itself; unsupported cross-volume link moves leave the source intact.
- F10.12 Preserve file contents and directory structure. Document which timestamps/basic attributes are preserved; do not promise ACL, alternate-data-stream, sparse-file, or hard-link preservation in this release.
- F10.13 After partial moves, keep unresolved source paths available for retry and avoid re-moving completed items. Navigating elsewhere does not change the captured destination.

### F11. Trash and permanent deletion

- F11.1 Delete sends selected entries to the Windows Recycle Bin through the OS facility, subject to confirmation showing item count and location. It never silently falls back to permanent deletion.
- F11.2 Shift+Delete requests permanent deletion and requires a distinct confirmation explicitly warning that the entries will not go to the Recycle Bin. Escape/Cancel changes nothing.
- F11.3 If trash is unavailable for a location, report that limitation and leave the entries intact unless the user separately requests and confirms permanent deletion.
- F11.4 Confirmed deletion operates on the captured paths, revalidated before processing. Missing or changed entries and partial failures are reported; row-index reuse cannot delete another file.
- F11.5 Deleting a link removes the link, not its target. Recursive deletion does not follow links/reparse points encountered within the selected directory.
- F11.6 Refuse deletion of filesystem/volume roots, the Windows directory and its descendants, and Program Files directory roots. Resolve these locations from the OS rather than assuming drive C. Explain the restriction; OS permissions remain authoritative elsewhere.
- F11.7 Deleted entries disappear from affected tabs; failed entries remain visible. Show completed/failed/skipped counts. Undo and automatic restoration are not promised; recoverable items can be restored through the OS Recycle Bin.

### F12. Operation status, cancellation, and recovery

- F12.1 A window-level operations drawer shows operation type, source/destination, current item, known progress, and completed/skipped/failed counts. It remains accessible after its originating tab/pane closes.
- F12.2 Enumeration or unknown totals use indeterminate progress rather than invented percentages. Copy/move work and conflict waiting never block browsing.
- F12.3 Start with one mutating operation at a time. Additional requests are queued visibly; cancelling a queued operation changes no files. Read-only browsing/preview remains concurrent.
- F12.4 Cancellation is cooperative. Stop scheduling new items, check between transfer chunks, and preserve completed work. An uninterruptible OS call may finish before cancellation takes effect; show that state honestly.
- F12.5 Reports identify each unresolved item and the actual error, with a retry that does not repeat destructive completed steps. Failed cleanup paths are shown, not hidden behind success.
- F12.6 Closing the application with active operations prompts to keep running or cancel and wait. Do not report successful cancellation before workers have reached a safe stopping point.
- F12.7 After a crash, never auto-delete sources, auto-retry destructive work, or auto-remove unrelated temporary files. Only identify operation-owned leftovers when ownership is established; durable resume/undo is deferred.

### F13. Refresh, persistence, and OS integration

- F13.1 F5 refreshes the focused tab. Filesystem notifications trigger debounced reconciliation; watcher overflow/failure exposes retry/manual refresh and cannot leave a falsely authoritative listing.
- F13.2 External creates, renames, and deletes update visible affected locations. Refresh handles a location that disappears while it is open, including an explicit missing-folder state and a usable Up action.
- F13.3 Operation completions reconcile every affected open tab. Ignoring stale results and refreshing are required even when filesystem notifications are unavailable.
- F13.4 Persist window geometry, split layout, tabs/locations, active tabs, focused pane, bookmarks, and recents. Restore without starting file operations; unavailable locations show errors rather than preventing startup.
- F13.5 Settings save atomically. Corrupt settings produce defaults and a visible warning while preserving the corrupt file for inspection. Do not persist clipboard operations as pending destructive work.
- F13.6 Windows clipboard copy/cut/paste interoperates with Windows Explorer for file paths and intended action. Clipboard input is treated as untrusted and goes through the same validation/conflict rules.
- F13.7 A minimal context menu provides Open, Copy, Cut, Paste, Rename, Trash, and Properties/basic details with valid enablement. A full native shell/third-party context menu is deferred.
- F13.8 Dragging selected entries between panes requests Copy by default or Move with Shift. Use a visible drop target and a confirmed conflict workflow; dragging a tab remains a different interaction.
- F13.9 Inbound external file drops copy into the targeted location after validation and conflict handling. Outbound shell drag-and-drop is deferred unless explicitly required before implementation.

### F14. Responsiveness and accessibility

- F14.1 Use virtualized rows. No file I/O, PDF rendering, image decoding, or unbounded sorting runs synchronously in the UI update path.
- F14.2 On a recorded reference Windows machine, a generated 20,000-entry local directory remains navigable during enumeration. Once its entries are loaded, filter/sort changes settle within 200 ms and input response is at most 100 ms at the 95th percentile over a documented repeated test.
- F14.3 On the same benchmark, ordinary scrolling reaches at least 30 frames/sec and preview caches stay within their documented budget. Record hardware, dataset, display scale, release build, and measurement method before interpreting results.
- F14.4 At 100%, 150%, and 200% display scaling, controls and dialogs do not overlap or clip their actions. Resizing keeps previews correctly scaled.
- F14.5 All essential actions work by keyboard with visible focus. Controls have accessible labels; verify primary navigation, finder, confirmation dialogs, and operation status using Windows Narrator before release.
- F14.6 Access denied, malformed preview input, clipboard errors, and ordinary I/O failures do not panic the application. Error messages identify the action/path without requiring a console.

## Shortcut contract

| Shortcut | Action |
| --- | --- |
| Ctrl+O | Folder finder for the focused pane |
| Ctrl+T / Ctrl+W | New / close active tab |
| Ctrl+Tab / Ctrl+Shift+Tab | Next / previous tab in the focused pane |
| F6 | Next pane |
| Alt+Left / Alt+Right / Alt+Up | Back / forward / parent folder |
| F5 | Refresh |
| F2 | Rename one entry |
| Ctrl+A / Ctrl+C / Ctrl+X / Ctrl+V | Select displayed entries / copy / cut / paste |
| Delete / Shift+Delete | Trash / confirmed permanent deletion |
| Escape | Cancel current transient UI, not an unrelated background operation |

Finder-specific Enter, Shift+Enter, Ctrl+Enter, and Tab behavior is defined in F05. Shortcuts are suspended when an input control owns the corresponding editing command.

## Implementation checkpoints

Keep each checkpoint runnable. Do not build all UI controls first and leave their behavior until the end.

| Checkpoint | Deliverable | Acceptance gate |
| --- | --- | --- |
| 1. Read-only explorer | Rust app, one virtualized folder table, navigation, selection, filter, basic OS open | F01, F02, F06; prove keyboard use and inaccessible-folder recovery |
| 2. Workspace | Independent tabs, split/focus/resize, finder and bookmarks, settings persistence | F03-F05, F13.4-F13.5; reproduce all three designed workspace states |
| 3. Safe operations | Create/rename, copy/move, conflict dialogs, trash/delete, operations drawer | F09-F12; automated failure/partial-transfer checks pass before using personal folders |
| 4. Previews | Image and PDF preview inside each pane | F07-F08; stale-result, corrupt-input and memory-limit checks |
| 5. Integration and release | Notifications, clipboard/drop integration, context commands, packaged PDF dependency, accessibility/performance checks | Remaining F13 and F14; disposable end-to-end workflow passes |

At checkpoint 1, `task run` must launch the Rust application, `task test` must run its automated checks, and `task clean` must remove generated build artifacts without touching user data or design/spec files. Retain a separate `task design` for opening the Excalidraw reference. Until then, the existing Taskfile operates on the design only.

No framework abstraction or custom renderer is required in advance. Separate directory loading, previews, settings, and mutating operations enough to test them without a GUI. Prefer installed/native facilities and small existing libraries over writing codecs, PDF interpreters, or Recycle Bin behavior.

Before expanding beyond checkpoint 2, run a small keyboard/focus/accessibility probe and a system-clipboard integration probe. If eframe cannot satisfy a required workflow without disproportionate platform work, report that blocker before building around it. Do not silently switch frameworks or weaken an acceptance criterion.

## Verification scenarios

Minimum scenarios to retain as automated checks or reproducible manual procedures:

1. Navigate A to B during slow enumeration; late A results never appear in B.
2. Give two panes several tabs and different filters; navigate with Ctrl+O and confirm only its target changes. Move a tab between panes and retain its state.
3. Select image A, then PDF B, then image C while previews load; only C appears. Test corrupt/oversized files and display scaling.
4. Copy a folder with nested files; verify bytes and structure. Test existing destination, same-entry aliases, descendant destination, and destination creation during commit.
5. Inject a mid-copy write failure/cancellation and an overwrite failure; existing destination and source contents remain intact. Verify/report temporary cleanup failures.
6. Move across volumes; test copy failure and source-delete failure separately. Verify source retention or the reported duplicate, never silent loss.
7. Cancel a multi-item operation after one item completes; report partial completion and leave unprocessed sources intact. Close the origin tab and retain operation visibility.
8. Reject invalid/reserved names; test case-only rename, collision, locked files, disappeared sources, and permission denial using platform-correct fixtures.
9. Copy/delete a directory containing a link/junction to an external sentinel folder; that target remains untouched. Unsupported link copy reports its limitation.
10. Cancel trash/permanent-delete confirmations and verify no change. Test unavailable trash without any permanent-deletion fallback. Actual Recycle Bin verification uses only uniquely named disposable entries.
11. Modify an open folder externally, simulate watcher failure, and verify refresh recovery and stable path-based selection.
12. Restart with a split layout, missing location, and corrupt settings. Restore safely; never execute a previously queued operation.
13. Copy/cut files between Windows Explorer and this app using the system clipboard; verify cancellation, conflicts, and default action. Check inbound drops and between-pane drag behavior separately.
14. Run the performance dataset and keyboard/Narrator checks on the recorded reference machine. Verify the packaged app on a clean machine without a Rust toolchain.

Fixtures should include empty directories, Unicode names, hidden entries, long paths, locked/unreadable files, links where available, and enough entries to exercise virtualization. Use bounded generated data; do not require checking huge fixture files into the repository.

## Deferred scope

- Full-drive recursive indexing/content search and recursive folder-size/count calculation.
- Batch rename, duplicate detection, synchronization, archive browsing/extraction, remote protocols, and plugins.
- Editable previews, full PDF-reader features, video/audio previews, and custom GPU image processing.
- Outbound shell drag-and-drop and full native shell context menus unless promoted to requirements.
- Preserving every filesystem metadata feature, elevated operations, whole-operation rollback/undo, crash resume, and multiple application windows.
- Linux/macOS release support until its integration and acceptance checks are explicitly scoped.
