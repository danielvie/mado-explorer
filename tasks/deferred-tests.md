# Deferred user-dependent acceptance tests

User instruction, 2026-10-13: continue implementation despite tests requiring the user; mark those tests deferred so they can be run later. DEFERRED means unverified, never passed or waived. This register is linked from the canonical [todo](todo.md). It does not authorize external actions or tests on user data.

## How to use this register

Each task below retains its original scenario and acceptance expectations. All 45 task implementation surfaces now have integration coverage in app/tests/windows-acceptance.md; T23/T42 manual release acceptance remains unverified. Original scenario/expected-result checklists below are preserved for later direct testing. As code lands, replace the related-change placeholder with its milestone/date/version and record automated coverage separately. Do not use this list as a reason to skip feasible fixture/API tests. T19/T21/T43 can often be substantially automated; defer only the remaining user-dependent observation.

Before testing, record build/version, OS, fixture root, device/DPI configuration and tester/date. Use newly created disposable fixtures with explicit ownership; never use real-user files, empty the global Recycle Bin, change system/security defaults or run FPilot.exe. Preserve unrelated clipboard/Recycle Bin entries and use action-time confirmations for destructive operations. Do not create denied-directory fixtures by changing system permissions. Run only implemented scenarios, record expected versus actual results, attach logs/screenshots where useful, and mark PASS or FAIL only with direct evidence. A failure becomes a tracked defect; absence of evidence remains DEFERRED.

Suggested sessions: native host/menu/OLE (T01–T09); MVP interaction and transfer/persistence (T10–T24, T43–T45); extended features (T25–T42). Combine Narrator/IME/high-contrast and 100/150/200% mixed-monitor checks across applicable views. No repeated permission to continue implementation is needed. Release/support-matrix acceptance remains pending until evidence exists.

## T01 — Launch the application window

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 foundation, 2026-10-13; native probes/build/lint evidence in todo completion record.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Launch and close the Rust window ten times; verify no FPilot.exe execution or writes to analysis/user state.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: A native window opens and closes using the candidate UI stack on Windows x64.
- Expected: Cargo.lock is committed when version control is established; toolchain and dependency choices are recorded.
- Expected: Application Taskfile provides default --list and described run/build/test/clean tasks without changing the root analysis Taskfile.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T02 — Establish the native window lifetime

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 foundation, 2026-10-13; native probes/build/lint evidence in todo completion record.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Differently scaled displays; T23 requires 100/150/200% and monitor moves. Narrator and configured IME; T23 also high-contrast observation.
- Steps: Repeat open/close and DPI changes; inspect callback teardown and basic Narrator/window focus behavior.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: The adapter obtains the real HWND through supported APIs and checks COM initialization results on its owning thread.
- Expected: Owned registrations/callback contexts survive callbacks and are removed before window destruction; no unsafe Send/Sync is introduced.
- Expected: A native callback probe preserves toolkit keyboard/mouse input and reports incompatible initialization instead of continuing.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T03 — Browse one directory asynchronously

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Sufficient disk/memory for owned 10,000/100,000-entry fixtures; record measured resource use.
- Steps: Browse a 10,000-entry owned fixture and an inaccessible directory while continuing to resize/type.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Home or a supplied directory displays name/type/size/modified rows with loading, empty, and error states.
- Expected: Rapid navigation discards late results by tab/panel generation; authoritative paths remain lossless.
- Expected: Instrumented row construction is bounded by visible rows plus documented overscan rather than total entries.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T04 — Select and activate entries

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Use keyboard only on Unicode and spaced names; refresh after deleting an entry externally in the owned fixture.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Ctrl-toggle, Shift-range, Select All, and cursor movement use the visible snapshot, not stale row positions.
- Expected: Activation navigates directories or opens files through the Windows adapter; vanished entries report errors.
- Expected: Keyboard focus is visible and ordinary text editing/IME input is not intercepted by file actions.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T05 — Open a native Shell menu

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Open/dismiss menus with mouse and Shift+F10; expand a dynamic submenu and open Properties on owned files.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: The correct HWND/selection obtains IContextMenu and opens the native menu with valid command-ID ranges.
- Expected: Required IContextMenu2/3 messages reach dynamic submenus and keyboard cancellation restores focus.
- Expected: The acceptance probe invokes only an approved read-only Properties action; interfaces/menu handles are released.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T06 — Copy files with explicit outcomes

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Copy owned files with a collision and a locked destination; request cancellation, then close during an active copy and verify cancel/wait/keep-open choices and final worker cleanup.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Copy runs outside the UI thread, creates interfaces inside its STA, and reports per-item results plus aborted status; close-during-copy enforces cancel/wait/keep-open before worker teardown.
- Expected: Cancelled, partial, and failed outcomes are distinguishable; HRESULT success alone cannot mark all items completed.
- Expected: Collision, denied/locked destination, and source/descendant validation paths do not silently overwrite or delete.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T07 — Probe incoming Explorer file drops

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Differently scaled displays; T23 requires 100/150/200% and monitor moves. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Drag Explorer files onto two probe regions at 100% and 150% DPI; cancel and verify files are unchanged.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: The pinned toolkit's existing drop registration is identified; exclusive supported registration is established without revoking toolkit-owned objects, with correct teardown and scaled-DPI coordinates.
- Expected: File names are extracted with queried lengths and bounded validation, including long/Unicode paths; unsupported formats are identified.
- Expected: The probe returns no successful transfer effect and never copies/moves/deletes dropped files.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T08 — Drag selected files to Explorer

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Copy and move owned files to a separate Explorer fixture folder with modifiers; cancel a drag and verify no mutation.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Outbound data objects and source state have explicit owning-thread lifetimes across the modal loop.
- Expected: Copy/move negotiation and cancellation use the Shell result; the application does not independently delete sources.
- Expected: Unsupported selections are rejected before entering a drag; callbacks remain valid through release/shutdown.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T09 — Filter and order directory rows

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Sufficient disk/memory for owned 10,000/100,000-entry fixtures; record measured resource use.
- Steps: Filter mixed-case/Unicode names, sort numbered names, toggle hidden/system files, and navigate during refresh.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Substring filtering, folder-first ordering, and logical name/size/modified sorting follow the new documented policies with deterministic ties.
- Expected: Filtering prunes hidden selection; back/forward/parent/path navigation preserves lossless targets and drops stale results.
- Expected: 10,000/100,000-entry measurements record hardware, time/memory, and visible-row work without claiming File Pilot equivalence.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T10 — Preserve independent tabs

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Open enough tabs to overflow, switch during loading, and close/reopen the active tab using keyboard only.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: New/duplicate/reopened tabs retain their own location/history/filter/selection with stable IDs.
- Expected: Overflow tabs remain reachable and closing tabs cancels stale non-mutating requests without affecting active mutations.
- Expected: Last-tab behavior follows the final-pane Home safeguard rather than accidentally closing the application.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T11 — Browse resizable split panes

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Use two folders in right/bottom splits, resize them, move focus, and verify independent filters/selections.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Splits resize and equalize without sharing mutable tab/location/selection state.
- Expected: Keyboard pane selection and operation targeting follow the active pane, including after closure.
- Expected: Layout invariants reject invalid ratios/indices and leave one Home pane when the final pane would close.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T12 — Execute focus-aware commands

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Use the palette and Ctrl+T/W, Alt navigation, and F2 where implemented; verify shortcuts inside a filter editor.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Palette entries and shortcuts invoke the same eligible typed actions; unimplemented commands cannot mutate files.
- Expected: Text editors and IME composition retain their shortcuts; single-letter Y does not copy files by default.
- Expected: Search uses the documented initial policy and deterministic ties; labels/availability track the active pane.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T13 — Copy file paths through the clipboard

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Copy owned files to Explorer and back; copy paths with spaces/Unicode and replace clipboard contents externally.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Copy publishes validated CF_HDROP and appropriate Unicode path text with correct global-memory ownership.
- Expected: Copy As Path uses a documented quoting policy while operations continue to use authoritative paths.
- Expected: Publication failure and malformed incoming clipboard data do not leak handles, read beyond bounds, or claim success.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T14 — Move cut files through paste

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items. Disposable locations on distinct volumes for cross-volume coverage.
- Steps: Cut several owned files, cancel or force one failure, and verify successful and remaining items against the ledger.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Paste resolves an immutable destination/request and distinguishes copy from move using validated clipboard formats.
- Expected: Only confirmed moved items update cut state, and only while the clipboard still matches the captured request.
- Expected: Partial failure, cancellation, clipboard replacement, same-folder moves, and cross-volume moves preserve remaining source/cut items.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T15 — Trash selected files safely

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Trash owned fixture files and verify their identities in Recycle Bin without altering pre-existing entries.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Trash requests cannot silently fall back to permanent deletion when recycling is unavailable.
- Expected: Cancellation, denied access, and partial results are visible and selection refresh follows confirmed outcomes.
- Expected: Tests touch only identified owned fixture items; no code or test empties the global Recycle Bin.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T16 — Create directory entries

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Create an owned file/folder, cancel another creation, and attempt invalid/colliding names.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Valid names create exactly the requested type in the captured current directory and then refresh/select the result.
- Expected: Invalid names, reserved names, collisions, and disappearing destinations produce explicit errors without replacement defaults.
- Expected: Cancelling the name dialog submits no operation and leaves existing entries unchanged.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T17 — Rename a selected entry

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Rename spaced/Unicode files, preserve or explicitly edit extensions, cancel, and test a collision.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Rename edits one authoritative selected entry, previews the full resulting name, and rejects empty/invalid names.
- Expected: Commit/cancel and collision/vanished-source outcomes are distinct; no successful operation is inferred from text acceptance.
- Expected: F2 and the palette invoke the same eligible action without taking over ordinary text editing.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T18 — Inspect bounded text contents

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Inspect normal, binary, oversized, and inaccessible owned files; change selection repeatedly while loading.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: The selected file displays bounded UTF-8 text and metadata without blocking navigation or editing the file.
- Expected: Oversized, binary, malformed, missing, or unreadable data produces a clear fallback; reads stop at the configured bound.
- Expected: Stale results are discarded and inspector text controls own their focus/selection/clipboard shortcuts.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T19 — Decode bounded image data

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Decode owned transparent PNG/JPEG and malformed/oversized fixtures; inspect owned memory and returned pixel format.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: WIC interfaces are created/released on a verified compatible thread and return owned pixels with an explicit channel/alpha format.
- Expected: Dimensions, decoded byte counts, API results, corrupt input, and configured application-owned limits are checked before buffer use.
- Expected: Native fixture tests verify decoding/error cleanup without requiring new UI or shipping a second decoder.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T45 — Display bounded image previews

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Preview transparent images in both panes, change selection rapidly, zoom, close the inspector, and inspect cache/texture counts.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Selected supported images display without blocking the UI; stale/cancelled results cannot install textures.
- Expected: Pixel alpha/channel format is converted correctly; zoom/fit, replacing, and closing the view release or reuse resources within the cache budget.
- Expected: Decode failures remain visible and text inspector/focus behavior remains intact.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T20 — Refresh listings from filesystem changes

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items. Authorized test UNC share; unavailable-share coverage remains deferred if none is available.
- Steps: Change an owned directory in Explorer, generate an event burst, and disconnect/reconnect a test share when available.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: External create/rename/delete changes update the current listing without changing authoritative selection incorrectly.
- Expected: Overflow/error/disconnect triggers visible invalidation and rescan recovery rather than a false empty listing.
- Expected: Closing tabs removes their watchers; sustained events and network failures do not block UI input.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T21 — Validate the session format

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Inspect representative current/future/corrupt fixture results and compare lossless path round-trips; no application restore is claimed yet.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Owned pane/tab/location/view/inspector values round-trip with lossless paths; live handles/PIDLs/callbacks/jobs are not serializable.
- Expected: Version, size/depth, finite ratios, unique IDs, active indices, and path encoding validation reject invalid input before state application.
- Expected: Corrupt/future input produces typed diagnostics and safe fallback data without modifying any input file.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T43 — Preserve state during replacement failures

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Force write/replace failures and process interruption on owned fixtures; verify old or committed-new state and preserved unreadable files.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: A complete validated serialized state is written/synchronized before replacement; replacement failure leaves prior good contents intact.
- Expected: Corrupt/future existing state is read-only until explicit recovery; saving defaults cannot silently replace it.
- Expected: Interruption/failure checks distinguish pre/post-commit outcomes and clean only owned temporary files; no universal power-loss guarantee is asserted.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T44 — Restore sessions through the window lifecycle

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Restart a layout, remove a saved directory, supply corrupt/future state, and start a second instance to verify writer ownership and warnings.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Startup restores validated tabs/panes/preferences with monitor-safe geometry; missing folders remain visible as warned locations.
- Expected: The writer lease prevents a competing process from overwriting state and releases on shutdown/crash according to checked Windows ownership.
- Expected: Close/restart saves only safe persistent state, never pending mutations; corrupt/future load and failed saves keep preservation/read-only rules.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T22 — Complete incoming file transfers

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Differently scaled displays; T23 requires 100/150/200% and monitor moves. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Drop owned Explorer files onto both panes with copy/move modifiers; cancel and verify no duplicate or premature deletion.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Current-pane targeting is correct across split panes/DPI; unsupported drops and stale destinations return no successful effect.
- Expected: The acquired Shell target is the only transfer authority; Drop's returned status/effect is used, no app operation is also queued, and no per-item success is invented.
- Expected: Explorer copy/move/cancel/collision tests report actual outcomes or delegated status, with rescans and complete lifecycle cleanup.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T23 — Pass the Windows interaction gate

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Differently scaled displays; T23 requires 100/150/200% and monitor moves. Narrator and configured IME; T23 also high-contrast observation.
- Steps: Run the entire MVP workflow with keyboard and Narrator, then IME/high-contrast and mixed-DPI monitors.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Keyboard-only browsing, menus, palette, tabs/panes, inspector, and operations have visible focus and no focus traps.
- Expected: Narrator identifies rows/selection/active tab/filter/progress; IME text input remains functional.
- Expected: High contrast and 100/150/200% DPI including monitor moves preserve readable content, hit targets, and native menu/drop coordinates.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T24 — Package the MVP executable

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Run the release executable outside the source directory with fresh state and complete the MVP checklist.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Release executable starts on the approved Windows baseline with an asInvoker/long-path-aware manifest and no console surprise.
- Expected: MVP acceptance records show supported/unsupported cases and measured performance budgets; no original equivalence is claimed.
- Expected: README/Taskfile describe run/build/test/clean and release use; package changes no registry handlers, PATH, or original analysis.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T25 — Search Shell commands in a custom popup

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Search/expand/invoke owned-file commands with keyboard; compare fallback and test registered extensions available on the machine.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Labels, verbs, IDs, disabled states, and lazy submenus stay bound to their owning Shell menu object.
- Expected: Matching/ranking/ties are documented new policies; disabled results cannot invoke and native fallback remains available.
- Expected: Known/common file commands route once to the intended authority; owner-drawn/unsupported extensions are not falsely advertised as compatible.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T26 — Browse Recycle Bin items

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: View owned recycled fixtures alongside existing items without changing either; restart a Recycle Bin tab.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Recycle Bin rows identify items through Shell values rather than pretending they are ordinary directory paths.
- Expected: Original location/deletion date/size properties use checked SDK conversions and missing-property fallbacks.
- Expected: Refresh/stale selection rules apply; the virtual location persists by kind, never by live PIDL/interface bytes.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T27 — Act on selected Recycle Bin items

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items. Explicit confirmation of exact disposable deletion targets; no unrelated Recycle Bin item may be changed.
- Steps: Restore one owned item, force a collision, and explicitly delete another owned recycled item; verify unrelated entries unchanged.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Restore/delete acts only on revalidated selected Shell identities, never all items by default.
- Expected: Collision/cancel/failure and disappeared-item outcomes are visible; failed items remain selectable.
- Expected: Automated/manual checks cannot empty the global bin or change unrelated recycled entries.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T28 — Expand a directory listing

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Expand an owned hierarchy containing duplicate leaf names and a junction; cancel and navigate away.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Expanded rows retain full authoritative paths and depth without ambiguously acting on duplicate display names.
- Expected: Traversal defaults to not following reparse points; cycles, inaccessible descendants, and cancellation are reported.
- Expected: Partial/complete listing status is visible and results cannot populate a closed or navigated tab.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T29 — Calculate folder statistics

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Calculate owned-tree totals, deny one subtree, cancel, and verify a disabled network policy submits no scan.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Counts/sizes distinguish complete, partial, cancelled, and failed scans using the same authoritative paths.
- Expected: SSD/HDD/network policies are documented and do not start unrestricted work when disabled.
- Expected: Concurrency/queues are bounded; refresh/cancellation and reparse points do not double-count or install stale results.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T30 — Browse virtualized icon views

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Differently scaled displays; T23 requires 100/150/200% and monitor moves. Sufficient disk/memory for owned 10,000/100,000-entry fixtures; record measured resource use.
- Steps: Scroll a large image fixture, change icon sizes, resize panes, and inspect request/cache instrumentation.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Grid layout constructs only visible/overscan rows and requests only bounded visible thumbnails.
- Expected: Sizes/view mode preserve tab selection and authoritative entry identity across sorting and resizing.
- Expected: Placeholder/error thumbnails, eviction, zoom, and DPI changes never allocate an unbounded texture cache.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T31 — Preview batch rename plans

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Preview numbering/date transformations on an owned mixed-extension selection and verify every file remains unchanged.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Preview lists old/new authoritative paths with explicit extension, date/timezone, numbering-origin, and padding policies.
- Expected: Invalid/reserved names, duplicate proposals, and existing-target conflicts are detected without changing files.
- Expected: Changing selection/options or cancelling discards the plan; no original template syntax is inferred from labels.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T32 — Commit a batch rename plan

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Commit an owned preview, create a destination race, cancel after partial work, and inspect the result ledger.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Source identities/destinations are revalidated and cycle/swap or collision cases are rejected or handled by an explicitly tested policy.
- Expected: Cancellation/partial completion reports each final path; no all-or-nothing rollback promise or overwrite shortcut is made.
- Expected: UI selection/status tracks confirmed changes and leaves failed items discoverable.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T33 — Save named layout presets

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Save/load two layouts, cancel an overwrite/deletion, and load a preset whose folder no longer exists.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: A preset restores pane/tab/view/inspector configuration with independent IDs and missing-path handling.
- Expected: Names/overwrite/delete choices cannot replace arbitrary files outside the application state directory.
- Expected: Corrupt/future presets are preserved and failed saves leave the last good preset intact.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T34 — Navigate Quick Access bookmarks

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Authorized test UNC share; unavailable-share coverage remains deferred if none is available.
- Steps: Bookmark local/UNC fixtures, group/reorder them, then open an unavailable path and restart.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Users add/remove/reorder/group bookmarks and open them in a chosen existing tab or new tab.
- Expected: Persisted bookmarks keep lossless paths; unavailable locations report errors without deleting bookmark data.
- Expected: Keyboard/filter behavior and accessible group expansion work without invoking unimplemented integrations.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T35 — Track and clear recent locations

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Visit/revisit fixtures, change persistence modes, clear recents, and verify independent bookmark/layout data.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Recents follow the documented new MRU/deduplication/limit policy and never become operation paths via display text.
- Expected: Never/session/persistent modes and explicit Clear behave distinctly without clearing bookmarks or layouts.
- Expected: Unavailable recents remain intelligible; sorting/filtering and restart behavior follow the selected mode.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T36 — Configure typography and appearance

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Differently scaled displays; T23 requires 100/150/200% and monitor moves.
- Steps: Change fonts/density/theme on both panes, try an unavailable font, and rerun contrast/DPI checks.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Font/density/appearance controls affect visible UI and persist; failed font loading falls back without bundling proprietary fonts.
- Expected: Numeric limits and supported theme choices are documented new policies; no inert original options are exposed.
- Expected: Changes preserve selection/focus, contrast, readable row geometry, and mixed-DPI behavior.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T37 — Configure listing input behavior

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Narrator and configured IME; T23 also high-contrast observation.
- Steps: Exercise each exposed click/checkbox/typing mode with files/folders and an active filter editor.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Supported input preferences have documented precedence and do not accidentally execute file actions during text editing.
- Expected: Mouse/keyboard/IME and selection tests cover each exposed mode, including folder-only single-click policy if enabled.
- Expected: Settings changes persist and revert to defaults without exposing unsupported mouse-hotkey customization.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T38 — Open external filesystem tools

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Open Explorer/terminal in owned directories containing spaces/shell metacharacters and inspect the working directory.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Actions target the captured current directory and report unsupported/unavailable executables or paths.
- Expected: Paths are passed through documented APIs/argument quoting, not concatenated into an executable shell command.
- Expected: Cancellation/dismissal has no mutation or registry/PATH side effect.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T39 — Create filesystem shortcuts

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explorer/Shell session; use only installed extensions and owned items.
- Steps: Create/open an owned shortcut, test a collision and cancellation, then separately verify any newly enabled link drag effect.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Shortcut target/name/destination are explicit and revalidated; existing shortcuts are never silently replaced.
- Expected: COM resources and failed-save cleanup are owned; invalid/cancelled requests leave no partial output.
- Expected: Link effects stay disabled until supported source/target interoperability tests and command eligibility pass.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T40 — Permanently delete confirmed selections

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Explicit confirmation of exact disposable deletion targets; no unrelated Recycle Bin item may be changed.
- Steps: Approve deletion of owned fixtures, cancel another request, and verify no Trash fallback or unrelated path mutation.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: A clear confirmation identifies the exact captured selection and permanent nature; cancellation submits no work.
- Expected: Only revalidated items are submitted and partial/cancel/error outcomes are reported without rollback promises.
- Expected: Ignored native tests are confined to owned disposable fixtures and prove Trash remains a different command.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T41 — Edit command hotkeys

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root. Narrator and configured IME; T23 also high-contrast observation.
- Steps: Rebind/reset an eligible action, create a conflict, enter invalid chords, and verify behavior inside text editors.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: Unknown commands/keys/modifiers, oversized chords, and conflicting focus contexts are rejected or require an explicit resolution.
- Expected: Bindings preserve reserved text/IME behavior and can be reset without importing original command-table layouts.
- Expected: Saved aliases round-trip and failures preserve the last good configuration; multi-step/mouse bindings remain deferred.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## T42 — Verify extended capability coverage

- [ ] DEFERRED user-dependent acceptance (not passed).
- Related version/change: Wayfarer Files 0.1.0 integration, 2026-10-13; see tasks/todo.md final command ledger and app/tests/windows-acceptance.md; this manual scenario remains NOT RUN.
- Prerequisites/devices: Implemented task and its technical dependencies; Windows 11 x64 interactive desktop, keyboard/mouse, newly created disposable fixture root.
- Steps: Run the approved advanced workflow matrix on release builds using owned fixtures and available registered Shell extensions.
- Reason deferred: interactive desktop, user observation or required hardware/external fixture is unavailable in this session; implementation is present; record actual tested build and scenario results when the required tester/environment is available.
- Expected results (original acceptance criteria):
- Expected: All implemented later capabilities pass targeted regression/native/manual scenarios and their dependency tests.
- Expected: Resource limits, accessibility, corrupt state, stale requests, partial mutations, and external interoperability are rechecked together.
- Expected: Unsupported original options/locations/formats and new-policy differences remain explicit in release documentation.
- Result/evidence: NOT RUN; tester/date/build and observations pending.

## Checkpoint acceptance record

CP01–CP15 retain technical requirements and automated checks. User-only review is DEFERRED, not a progression veto. CP03 stack acceptance must not be claimed while native/accessibility evidence is missing; a demonstrated stack failure must be fixed or escalated. CP05 requires confirmation of conditional clipboard updates and no permanent-delete fallback. CP08 and CP14 remain unaccepted release/support-matrix reviews while implementation may proceed. CP11 rename/preset policies require recorded concrete behavior and test evidence. CP13 link drag effects remain disabled until interoperability and explicit approval requirements are met. Action-time permissions and the T40 irreversible-action scope boundary are not waived by deferring tests.

- [ ] CP01–CP07 and CP15: batch review corresponding task results; record unresolved defects and evidence.
- [ ] CP08: review MVP release build, support matrix and all deferred MVP tests; do not claim verified release before acceptance.
- [ ] CP09–CP13: review implemented extended scenarios and policy/interop evidence; retain link-effect restrictions.
- [ ] CP14: review extended release/support matrix and remaining deferred scenarios.


## Available final test artifact

Wayfarer Files 0.1.0 integration, 2026-10-13: app/target/release/wayfarer-files.exe, 16,210,944 bytes; SHA-256 ECA98B6213FBB9DF2C9ADE3FA5593A393E261613F778BD4057F736EA54906787. Automated/native/lifecycle evidence is in tasks/todo.md. This identifies the build available for deferred testing and does not mark any user-dependent scenario passed.
