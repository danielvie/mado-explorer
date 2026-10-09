# S011 worker handoff

Baseline: `36798781b4f9bf8701c54197f90e80d39c1d9cb6`, main checkout `C:/SANDBOX/REPOS/mado-explorer.git`.
No commits, delegation, promotions, native smoke, personal settings writes or personal-folder mutation checks. Supervisor-owned PROGRESS.md, LANES.md and handoff/ were not edited.

## Changed paths

- `C:/SANDBOX/REPOS/mado-explorer.git/src/settings.rs` new explicit snapshot/schema validation, lossless native paths, bounded load/serialization, atomic replacement and fixed coalescing worker.
- `C:/SANDBOX/REPOS/mado-explorer.git/src/settings_workspace.rs` new startup/restore adapter, geometry, autosave scheduling, visible gate/warnings/retry and close coordination.
- `C:/SANDBOX/REPOS/mado-explorer.git/src/settings_tests.rs` new twelve focused disposable/headless checks.
- `C:/SANDBOX/REPOS/mado-explorer.git/src/main.rs` startup-folder precedence, settings construction and App logic/ui/on_exit wiring; test constructor adds empty persistence state.
- `C:/SANDBOX/REPOS/mado-explorer.git/src/finder_ui.rs` settings/gate disclosure and bookmark/recent labels.
- `C:/SANDBOX/REPOS/mado-explorer.git/Cargo.toml` direct serde derive and serde_json dependencies.
- `C:/SANDBOX/REPOS/mado-explorer.git/Cargo.lock` serde_json 1.0.151 plus itoa/zmij lock entries; existing transitive serde version reused.
- `C:/SANDBOX/REPOS/mado-explorer.git/README.md` candidate behavior, limits, gate and pending checks.

Taskfile.yml already satisfies run/test/clean/design/default contract and needed no changes. This handoff is the explicitly requested generated artifact under target/.

## Candidate behavior and API choices

Version 1 serde JSON snapshot contains only normal geometry/maximized state, split tree/ratios, all tabs' committed native locations, active indices, focused pane, bookmark and recent paths. No operation, clipboard, pending navigation, selection, history, filter, scroll or editor-draft state. Restored IDs are fresh startup identities. Missing active locations use existing asynchronous refresh/error/Up; inactive tabs load when activated. Explicit startup folder replaces only the restored focused active tab. No-argument startup restores; defaults use cwd.

Windows paths serialize UTF-16 units, including lone surrogates, UNC and verbatim prefixes. Non-Windows development fallback uses native bytes. Display text is never the serialized identity. Destination native paths also reject NUL/relative/device forms before save calls.

Startup uses SHGetKnownFolderPath(FOLDERID_LocalAppData), no create flags, then `Mado Explorer/workspace.json`. A 2 MiB capped read and normal serde recursion limit precede validation. Bounds: 64 panes, 256 total tabs, split depth 16, one leaf per pane, valid active/focus indices and ratios, bounded finite geometry, native path units <=32766, catalogue lists <=128 with no duplicates. Unknown schema fields and incompatible versions are rejected. Bad/unreadable files remain untouched, defaults and visible warning appear, and that path stays write-disabled for the session. Repair and restart is the recovery path; no implicit backup or corrupt-file deletion.

One fixed settings worker holds one running save, one replaceable pending snapshot and one report. UI schedules changes after 500 ms. Filesystem work and bounded serde serialization run on that worker. A save failure pauses the worker and retains the failure even if the report was consumed or newer snapshots were queued. Only explicit Retry resumes it, preventing coalescing from hiding cleanup failures. Errors have bounded details and a separately retained full temp recovery path shown on hover.

Writer validates before touching disk, reserves a unique same-directory create_new temp, writes all bytes, flushes, sync_all, and checks native CloseHandle. Commit uses same-directory MoveFileExW with MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH, without delete-before-rename or cross-volume copy. Failure cleans only its reserved temp; cleanup failure returns its path. No unrelated temp discovery/deletion. Standard rename is the non-Windows fallback.

Close cancels an open finder, freezes custom workspace/finder handlers and directory-result acceptance, captures the final committed snapshot and cancels native close while waiting. Latest successful revision permits close. Failure/unavailable saving exposes Keep browsing, explicit Retry save and close where available, and Close without saving. on_exit joins, never detaches a last save; unexpected final failure uses MessageBoxW. A previously shown failure accepted through Close without saving does not produce a duplicate popup. Worker Drop also joins.

Geometry uses eframe viewport initial size plus winit window/monitor facilities. Physical normal outer position, logical normal inner size at native scale and maximized state are retained; minimized/maximized bounds do not replace normal geometry. Monitor rectangles recover disconnected/offscreen positions. No direct winit dependency and no eframe persistence feature/save hook reliance.

## Critical gate and acceptance

`C:/SANDBOX/REPOS/mado-explorer.git/src/settings_workspace.rs` contains:

`pub const PRODUCTION_SETTINGS_WRITES_ENABLED: bool = false;`

Production creates no settings writer while this gate is false. Window and finder disclose that changes do not survive restart. The close flow explicitly asks before closing without saving. Disposable tests pass true only through the private injected-path adapter, or construct the backend worker directly on generated paths.

F05.6/F13.4/F13.5 remain partial until supervisor safety review, production enablement and required native/manual checks. No criterion status was edited or claimed Verified. The close test intentionally asserts that the production gate is false; supervisor must adjust that assertion/gated test branch when deliberately enabling the gate.

## Checks and outcomes

All commands executed in `C:/SANDBOX/REPOS/mado-explorer.git`.

First coherent command:
`cargo fmt && cargo fmt --check && cargo check && cargo test s011`
Passed formatting/compile and the initial 10 S011 tests. Initial dependency resolution added serde_json/itoa/zmij.

After shutdown/coalescing safety inspection:
`cargo fmt && cargo fmt --check && cargo check && cargo test s011 && cargo test s007 && cargo test s008 && cargo test s009 && cargo test s010 && cargo test navigation::tests && git diff --check`
Passed fmt/check; S011 11, S007 8, S008 9, S009 10, S010 13, navigation::tests 10. No failed/ignored focused tests.

Final command after adding minimum-window warning/close-control evidence:
`cargo fmt && cargo fmt --check && cargo check && cargo test s011 && git diff --check`
Passed fmt/check and S011 12/12. Final source inspection also distinguishes an existing dangling settings link from an absent settings file when open returns NotFound, preserving the unreadable existing entry. After that small read-side guard, `cargo fmt --check && cargo check && cargo test s011 && git diff --check` passed again, 12/12. No dedicated native symlink fixture claim. Diff check passed, with Git's existing LF-to-CRLF warnings, including supervisor-owned PROGRESS.md. No test failure/fix-attempt boundary was reached. No full-suite, Clippy, benchmark or native smoke claim.

Twelve S011 checks cover lossless native roundtrip; nested tree/tab/active/focus/catalogue restore and argument override; missing folder and Up; unknown schema/version/tree/geometry/index/count limits; corrupt/oversized/unreadable preservation with adapter disabled; every precommit write/flush/sync/close/commit failure preserving old bytes; successful native replacement; real native commit failure; cleanup failure with owned recovery path and unrelated sentinel retained; bounded serialization before temp creation; worker latest snapshot and joined shutdown; failure pause/explicit retry and retained error; close/latest revision, gate prompt and shortcut/finder freeze; headless warnings/actions inside 480x280 bounds.

## Remaining review and platform risks

- Production saving is intentionally disabled. Independent safety review and supervisor enablement are mandatory.
- Native restart, minimize/maximize/normal bounds, disconnected monitors and mixed-DPI checks remain pending. Headless geometry checks are not a DPI or native restore claim.
- Native close/retry/exit warning and keyboard/Narrator accessibility checks remain pending. Headless emitted text/actions and shortcut isolation passed.
- An uninterruptible filesystem call can delay final join/exit. Normal close waits asynchronously with a visible status; no unsafe detach timeout was added.
- Atomic rename and sync are not a power-loss durability guarantee on every filesystem/provider. Same-directory native Windows replacement was exercised on disposable local fixtures only.
- Invalid settings require repair plus restart to enable that path. No in-session repair parser, backup policy, migration or leftover-temp scavenger was added.
- Bounded snapshot limits can refuse saving an oversized workspace. The existing UI remains usable and explicit error/retry preserves prior settings; no silent tab/catalogue truncation.

Next supervisor action: inspect all three new files plus integration diff, commission read-only safety review of replacement/load/worker/close paths, resolve concrete findings, then explicitly enable the production gate and run the pending disposable native restart/geometry/close checks. Do not treat this candidate as completed persistence before enablement.
