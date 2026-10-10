# S019 creation and operations app integration

## Source and ownership

- Checkout `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace`, branch `lane/compact-workspace`.
- Clean starting source HEAD `d5248f4d1d76d8fa9352fc4f78aaea9191aa6010` verified before writes. No latest-main metadata was merged.
- Validated source commit `65f8fb80682ed2db5c4e497079ef3159fbdc633d`. This document is a subsequent documentation-only commit; its candidate HEAD is supplied in the worker receipt and recoverable with `git rev-parse HEAD`. All application source at that HEAD is identical to the validated source commit.
- Changed source paths only `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/workspace.rs`, `src/settings_workspace.rs`, `src/watch_adapter.rs`, new `src/create_adapter.rs`, `src/create_ui.rs`, `src/create_tests.rs`. This handoff is the tenth owned path.
- `src/watch_adapter.rs` only exposes the unchanged FALSE `NATIVE_CAPABILITY_GATE_CLEARED`. Cargo adds the two existing path dependencies; lockfile delta is 16 additions with no dependency upgrades.
- No component, Taskfile, tracker, settings schema, design, other source, checkout/history integration, clipboard, preview or provider changes. No delegation, native window launch, elevation or personal-folder checks.
- Frozen candidate for fresh independent filesystem/UI/lifecycle review and supervisor inspection. Not self-approved, integrated or production-enabled.

## Actual application route

Explicit window-level `New` menu commands capture focused PaneId, active TabId, complete native committed destination and its browsing generation. The bounded modal shows the captured target, with full native diagnostic display on hover. Opening it does not authorize creation. A fixed background worker captures and pins the actual directory chain through the reviewed P18 backend before Create can be confirmed. Name validation is bounded CPU-only work on UI; all capture, creation and pin destruction occur on the worker. No handle enters UI state.

Enter routes to the focused name/confirm/retry/cancel role. Escape wins over submission and browsing shortcuts. Invalid, reserved, empty, separator and trailing names retain the draft and explanation. Capture failure retains name, exact target, Retry preparation and Cancel. Later focus, selection, navigation and origin closure do not retarget the captured destination. Creation uses only P18 `create_empty_file` or `create_directory`, with unchanged FILE_CREATE and OBJ_DONT_REPARSE semantics. There is no path preflight, overwrite or mutation fallback.

One window-owned accepted `Operations` controller supplies request IDs, FIFO, cancellation and retained drawer presentation. Payload and drawer metadata match by ID, kind and exact native path units before mutation. The worker reports current item and known one-entry progress, then actual Completed/Cancelled/Failed counts and unresolved errors. Full native unresolved paths remain separate from bounded native action/error-code text. An unexpectedly refused worker report is retained, not truncated or used to advance the active slot.

Additional bounds are one editor, 32 payloads, 17 worker-held captures including retiring captures, 34 commands, 34 reports and at most 34 unsent controls. Control pressure refuses new preparation explicitly. Capture pressure refuses before an eighteenth provider call and keeps the editor usable for explicit retry. No thread per request or unbounded channel. The worker may block on a bounded report mailbox, never the UI.

Queued cancellation emits no mutation Start. Active cancellation sets a cooperative flag, retains the active slot and waits for the actual worker terminal report. An OS call already admitted may finish. Its success remains Completed with count 1 even after cancellation. Pins drop before terminal reporting or successor execution. A shared stop flag also prevents queued preparation provider calls after close cancellation. Stop releases every remaining pin and sends cleanup acknowledgement; UI joins only an already-finished thread. Unexpected host teardown signals stop/cancellation and disconnects without joining live I/O.

Only Failed unresolved creation reports offer explicit retry. Retry retains the original destination and expected native directory identity, prepares it again on the worker, and refuses a changed or missing directory. Successful, cancelled and acknowledged requests are not retried automatically. Editing the failed name is an explicit new confirmation; the original failed report remains retained until acknowledgement. No queue or operation is persisted or restored after restart.

## Reconciliation and close

Successful completions conservatively dirty every open tab, including alternate spellings, without native watcher authority. Active tabs refresh only after pending navigation settles. Inactive tabs remain dirty and refresh on activation. A pending A-to-B request is never cancelled to refresh A; failed B retains its diagnostic and permits A reconciliation, while successful B rejects A selection.

Selection follows stable original TabId, captured location/path and a specific fresh generation, never focused pane or a retained row number. The immutable captured Target is separate from generation eligibility forwarded only through existing same-context activation/watch/operation refreshes. This lets queued creations select the last intended fresh entry without authorizing unrelated F5/new-navigation generations. Selection occurs only after a complete fresh visible projection. Hidden created entries report that the filter/visibility hides them; the user's filter is not cleared. Origin tab or pane closure does not cancel work or remove the window drawer.

Native close requests reach creation handling before settings-close. Keep running retains the editor. Cancel and wait cancels queued requests, stops active work cooperatively and waits for actual cleanup plus finished-thread ownership before handing off once to the existing settings save/close/failure prompt. Settings schema and existing close controls remain unchanged. A fresh listing may still be loading when settings-close suppresses directory polling; this is not claimed as completed visible-row evidence.

## Validation commands and outcomes

All commands ran in the named owned checkout. Final coherent source validation:

```text
cargo generate-lockfile --offline
cargo fmt
cargo fmt --check
cargo check --all-targets --locked --offline
cargo test --locked --offline create_tests -- --test-threads=1
cargo test --locked --offline -- --test-threads=1
git diff --check
git diff --cached --check
```

Formatting and incremental locked/offline all-target compile pass, no warnings. Final focused group **21 passed, zero failed or ignored**. Final full root suite **157 passed, zero failed, one existing optional installed-font probe ignored**. This includes affected finder, path/filter routing, settings-close, tabs/panes and gated watcher checks. Whitespace checks pass; Git's existing LF-to-CRLF working-copy warnings are not test failures. No Clippy, benchmark, screenshot, native GUI smoke or component safety rerun; reviewed component source was unchanged. Taskfile is unchanged and already runs/tests the root application.

### Actual versus controlled evidence

- Actual P18 native capture and creation through App state, worker and drawer for files/folders, Unicode names, no-overwrite collision with preserved sentinel bytes, explicit retry, changed native identity and missing target. No create backend stub or weakened native options.
- Actual background directory loader and sorter produce fresh current rows, path-based selection, active alias-location updates, inactive-tab activation updates and created-but-filtered status while production watching is off.
- Actual headless egui CentralPanel pointer New menu, focused confirmation Enter, failed retry button after origin closure, Escape/submission isolation, drawer keyboard layout retry and global finder routing. This is not native DPI/Narrator/visual proof.
- Controlled `cfg(test)` Start-delivery hold builds the bounded 16-request FIFO and 32 retained report cases. Preparations and eventual file mutations are actual P18 calls. The before-call gate proves no mutation before cancellation; the after-call acknowledgement gate lets a real successful native call finish before cancellation and preserves Completed/count 1. These are controlled timing boundaries, not empirical OS-call cancellation latency or interruptibility measurements.
- Controlled B loader completion establishes pending A-to-B success/failure timing and stale response suppression. A's creation and its eventual visible rows are actual. Worker metadata mismatch injection uses only generated owned paths and proves fail-closed no mutation.
- Headless viewport Close event and actual prompt buttons exercise the settings intercept. No native window is opened. A queued preparation behind the controlled active call is skipped after stop, verified by the actual capture-call counter staying at one.
- Fixtures reserve create-new uniquely generated owned root/parent/A/B directories with sentinels. No personal/home/root enumeration, malformed namespace I/O, global clipboard or other application launch. Normal checks stop/release worker ownership before cleanup. A test unwind retains its generated fixture if worker cleanup is not established; it does not perform uncertain rollback.

### Preserved failures and diagnosis

The initial compile found two mutable/immutable pane-index borrow errors and test-only layout-ID/private-settings-field mistakes. Owned indexing and headless public-observation checks corrected them. Final compile is clean.

The first focused group was **10/11 passed**, not a passing suite. `s019_close_wait_counts_actual_success_after_cancel_and_preserves_settings_flow` failed at the assertion requiring `raw_entries` to contain the created path or `create.dirty` to remain nonempty after cleanup/settings handoff. Reconciliation had already consumed the dirty bit and started its fresh generation, but existing settings-close suppressed further directory polling. Supervisor classified this provisionally as a close-harness expectation mismatch, not an established production failure. No production workaround or settings-close weakening was made to satisfy it.

The corrected close check preserves actual created bytes/path, Completed/count 1 despite late cancellation, cleanup acknowledgement before settings handoff and the existing settings prompt. It additionally asserts original stable TabId, immutable captured location/generation, exact created destination, specific origin refresh generation `captured + 1` and matching selection marker. Separate open-window checks verify actual fresh rows/all affected tabs and pending A-to-B behavior; a scheduled generation alone is not full F13.3 evidence. The corrected 11-test group passed before further coverage was added.

Self-inspection added a stronger FIFO selection assertion. `cargo test --locked --offline s019_bounded_fifo -- --test-threads=1` was red: final selection was empty rather than the last successful queued creation `fifo-15`. The original eligibility compared each queued request only to its immutable capture generation, so operation-owned refreshes invalidated later queued selection. The same-context refresh hook fixes that real state bug without mutating captured destinations. The retained assertion now passes; a separate explicit-refresh regression proves unrelated generations remain unauthorized. Subsequent focused groups of 18, 20 and final 21 checks passed. The final full suite above is against the corrected source, not the earlier intermediate 154-pass root run.

## Acceptance limits and next owner

No complete release criterion is marked Verified here. Creation-route portions of F09.1/F09.3/F09.5 and creation-only F12.1-F12.6/F13.3/F02.5 are implemented and have the focused evidence above. Supervisor owns criterion states and acceptance.

Production capability remains FALSE with no feature/env/CLI/settings/UI override. Both explicit production New commands honestly explain unavailability and change no files. Private `cfg(test)` enablement asserts generated owned descendants and is used only by these fixtures. It grants no personal-folder authority. Watch, clipboard and previews remain disabled as before.

P18's initial local-NTFS-only provider guard remains a partial support ceiling, not a permanent product-scope deferral. Other required providers, real ACL/permission and locking UI outcomes, successfully converted ancestors/privileged cases, full native capability evidence, manual creation/editor/drawer interaction, DPI/Narrator and release safety checks remain pending. Rename, transfer conflicts/copy/move, trash/delete, their chunk cancellation/partial recovery and operation persistence/crash recovery are not implemented by S019. Fresh independent filesystem/UI/lifecycle review and supervisor integration validation are required before acceptance; production mutation enablement needs an explicit separate decision/evidence.

## S019-R01 selection-state correction

Fresh correction started from clean `a39b4b37a502230334f7ad7cf092c1a6e7329877` on the same checkout and branch. Original candidate remains preserved at `archive/s019-selection-blocked`. Only changed paths are `src/create_ui.rs`, `src/create_tests.rs` and this handoff. Supervisor owns main, PROGRESS, review and integration. Final correction SHA is supplied in the worker receipt and recoverable with `git rev-parse HEAD`.

UI reviewer `8bf4cd5f` reported two P2 findings, independently confirmed by the supervisor:

1. `rows_selectable()` intentionally permits fresh empty-filter Loading/Failed projections for ordinary browsing. Creation reconciliation nevertheless consumed its marker on such rows, either selecting prematurely or permanently treating an incomplete absence as final. The correction adds an explicit `Status::Ready` requirement alongside the unchanged pending-navigation, freshness, TabId, native path and generation guards. Ordinary browsing selection is unchanged.
2. Deferred `Some(0)` checked TabId/path but not mutable `origin_generation`. Explicit New navigation to A/child/.. could finish normalized to A at g2 after creation completed against committed A/g1. Dirty refresh then incorrectly authorized selection at g3. The correction rechecks mutable eligibility before the dirty refresh can assign its marker. Stale selection is disarmed; affected-tab refresh still runs. Immutable captured Target and the existing same-context refresh hook are unchanged.

### Retained red-to-green checks

Both new tests were added and formatted before any production change. In the owned checkout:

```text
cargo fmt
cargo test --locked --offline s019_r01 -- --test-threads=1
```

Against unchanged a39 production guards, this command exited 101 with **0 passed, 2 failed, 158 filtered out**:

- `s019_r01_progressive_fresh_rows_do_not_consume_creation_selection`, `src/create_tests.rs:282`, failed `fresh Loading projection must not consume the creation marker, even with the created path present`. The first fresh empty Loading projection had already set `selected` true.
- `s019_r01_normalized_new_navigation_rejects_deferred_old_generation_selection`, `src/create_tests.rs:416`, failed `normalized explicit New navigation must not authorize old-context auto-selection`. Actual selection ended in `normalized-created.txt`; expected selection remained `sentinel-a` in the same generated A fixture.

The original failing assertions remain. One production correction attempt added only the two guard conditions above. Final coherent validation commands:

```text
cargo fmt --check
cargo check --all-targets --locked --offline
cargo test --locked --offline s019_r01 -- --test-threads=1
cargo test --locked --offline create_tests -- --test-threads=1
cargo test --locked --offline -- --test-threads=1
git diff --check
```

All pass. New group **2 passed**; full creation group **23 passed, zero failed or ignored**, including all unchanged original 21 checks. One full root run **159 passed, zero failed, one existing optional installed-font probe ignored** covers affected navigation, tabs, watch and settings/input checks. Compile is incremental and warning-free. Whitespace check passes; existing Git LF-to-CRLF warnings are not failures. Staged whitespace and clean exact-owned commit checks are recorded in the final receipt.

### Actual behavior and controlled timing

- Progressive regression uses actual App preparation, worker, P18 create, successful drawer completion and operation-scheduled refresh. It retains actual background-loader entries and success response, then controls delivery of empty, partial-without-created-path and path-containing snapshots. The unchanged real projection function supplies a fresh key for each controlled stage. Each Loading stage leaves the marker unconsumed and selection empty. Ready with a stale projection still waits. Ready with its fresh projection selects the exact created native path once; later sentinel selection survives another reconciliation. This is deterministic progression timing, not a slow-provider measurement or fake create backend.
- Normalization regression uses actual pinned P18 creation with the existing before-call gate. Explicit New navigation is pending before successful creation completion. Only alias loader timing/completion is controlled: existing actual A rows and a normalized A success response exercise the unchanged `Navigation::apply` same-location alias branch. The ordinary dirty refresh assigns g3, and actual background loader/sorter rows contain the created path. Sentinel selection survives; no authorized marker remains. Mutable eligibility and immutable captured Target both remain at original g1. Existing pending-B failure selection, explicit F5 staleness and queued `fifo-15` selection checks remain green.

No FS backend, worker, adapter, lifetime/pin/cache/thread/queue, navigation, loader, provider, input, settings, Cargo, Taskfile or component source changed. Production capability remains FALSE, with only existing private owned-fixture `cfg(test)` enablement. No personal folders, clipboard, network, elevation, native window/probes, Clippy, screenshots, benchmarks or component reruns were used.

Original S019 **21/157** evidence and supervisor original-a39 native save/close/restart smoke remain historical original-candidate evidence. They are not relabeled with corrected counts. Corrected-head native smoke and fresh corrected UI/state review remain supervisor-owned and pending. Existing optional font/native/manual/accessibility/provider limits and all preserved original failures above remain unchanged. This correction is frozen for review, not self-approved or production-enabled; no full acceptance criterion is claimed.
