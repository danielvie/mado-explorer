# S018 app directory notification adapter

## Frozen source and boundary

Checkout C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace, branch lane/compact-workspace. Preflight returned clean status and exact starting HEAD344ddc6de46f46cc38cdec9211b80f0acb4509c9. Read live root AGENTS.md, SPEC.md, CONTEXT.md, PROGRESS.md S018 and LANES.md read-only, checkout AGENTS.md, and accepted watcher public API, complete native implementation, public tests and HANDOFF/native contract.

Frozen validated source commit: **5ef1947e539eebb10c2c64929ff5921494ffbb8a**. The enclosing documentation commit is the final review candidate, with this source unchanged. Its full SHA is returned in the worker receipt.

Changed paths only: src/main.rs, src/workspace.rs, new src/watch_adapter.rs, new src/watch_tests.rs, Cargo.toml, Cargo.lock, and this handoff. One Windows-only local path dependency; lock delta adds the local package and dependency reference only. No upstream version changes or new external library. Accepted components/directory-watch tree remains exactly aa3d720a4af5718686deef070a36b95912e9aaa0. No driver, legacy diagnostic, tracker, Taskfile, settings schema, design or other source changes. During editing, an accidental empty src/watch_tests.rs.tmp was created outside the exact filename boundary, immediately removed, and never built or committed. Final delta contains only the seven authorized paths. No delegation, merge, rebase, reset, push, clean, elevation, personal-file/clipboard/external-provider probe, application launch or capture.

## Production gate

`NATIVE_CAPABILITY_GATE_CLEARED` in src/watch_adapter.rs is deliberately **false**. Required pre-checkpoint-3 native keyboard/accessibility and system-clipboard authority/evidence remain pending. Generic continuation is not permission to replace personal clipboard contents. There is no persistent, user, environment, CLI or UI override. Only supervisor approval after required checks/decision may clear this source gate.

Production creates no watch Service, registers no targets, has no watch repaint timer and dispatches no watch reconciliation. Footer says `Notifications off; F5`; its tooltip and reachable View menu explain manual refresh and the pending capability gate. Existing Refresh/F5 remains functional. Enabled status/retry controls are rendered only for the internally enabled adapter, never as fake live production controls.

Test-only enablement checks generated `mado-s018-*` absolute containers and keeps each watched directory and its immediate parent inside that container. Subsequent target synchronization repeats the owned-path guard. Tests do not invoke application startup, settings startup, installed-font/native UI probes or common-place inventory.

## App behavior

- The window owns one Service. App::logic reaches poll_directory, synchronizes committed active pane/tab contexts and drains native reports. A 100 ms requested repaint drains bounded reports/controls; it does not poll filesystem paths or spawn per-pane threads.
- Stable tab IDs route immutable native targets. Increasing watch revisions and nonreused watch generations are independent of loader refresh/sort/filter generations. Unchanged targets are not re-registered each frame. Native spelling compares complete OsStr units, not Path equality or displayed text.
- Current snapshot is bounded to MAX_TARGETS plus one detection record. More than 32 active locations refuses all notification targets, visibly, rather than selecting a silent subset. Busy, unavailable, target errors, parent errors, driver errors and overflow remain independently visible with full native display paths/action/codes. Control and report-drain refusals are separate, so a successful drain cannot hide a refused control.
- Reconciliation flags coalesce into one dirty bit per current target. Report readiness is not listing freshness. Refresh/listing success and pressing Retry do not clear provider warnings; actual service reports determine recovery. Retry uses the matching current revision/tab/generation through the accepted immutable Service API. Terminal driver shutdown is not automatically respawned; manual Refresh/F5 remains available and Stopping is reported.
- Dirty A is deferred during explicit A-to-B navigation and during any current load. B success changes the committed context and discards obsolete A work. B failure retains A and then refreshes it through the directory worker, retaining failed-B diagnostics, history, filter, sort, path selection, editor draft and scroll state. Events do not repeatedly cancel inflight loads.
- Inactive tabs have no watch target and still refresh on activation. Tab movement changes pane-owned watcher lifetime without cancelling the moved tab's loader request. Closed/superseded contexts reject stale reports. Closing a pane removes only read-only targets; no mutating-operation behavior changed.
- App::on_exit requests nonblocking native stop before existing persistence exit handling. No production watch join or wait occurs, even after acknowledgement. Service remains window-owned while the app lives; after drop, the accepted driver's worker/Arc retains pending native retirement storage until actual completions drain. No cancellation deadline or cleanup success is promised at request time. Tests alone wait/join after actual cleanup.

## Checks

All commands below ran from `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace`. Shared application builds used `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target`; no clean or concurrent root build requested.

- `git status --short && git rev-parse HEAD && git branch --show-current`: clean baseline344ddc6de46f46cc38cdec9211b80f0acb4509c9, lane/compact-workspace.
- `cargo generate-lockfile --offline`: passed; inspected exact eight-line lock addition, no package version updates.
- `cargo fmt`: applied only owned Rust changes. Final `cargo fmt --check`: passed.
- `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo check --locked --offline --all-targets`: final pass without warnings.
- `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo test --locked --offline s018_ -- --nocapture`: **14 passed, zero failed/ignored**. Fixture wait loops call actual `eframe::App::logic`, not a standalone substitute adapter.
- `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo test --locked --offline`: **133 passed, zero failed, one existing installed-font probe ignored**. All original119 passing tests and all9 unchanged S016 checks pass, including affected S007/S008/S009/S010/settings routing. Full root suite is justified by shared application state. Earlier identical full run with `-- --nocapture` passed and printed expected existing settings-failure diagnostics from generated fixtures.
- `git diff --check` and `git diff --cached --check`: passed. Inspected actual source/manifest/lock diff and new adapter/tests.
- `git rev-parse HEAD:components/directory-watch`: aa3d720a4af5718686deef070a36b95912e9aaa0; `git diff 344ddc6de46f46cc38cdec9211b80f0acb4509c9 -- components/directory-watch`: empty.

First compile exposed a private persistence field and two Rust borrowing errors in new code. Removed the unnecessary private-field access and used the active-tab index local; next compile passed. No focused behavioral test failed, no test removed/weakened, no failed check claimed passing.

## Native versus controlled evidence

Actual accepted Service and directory worker, using owned generated fixtures: Unicode create/rename/delete to current visible rows; surviving/vanished path selection; direct watched-root deletion and missing state; root recreation/reattachment; initially missing registration and Retry; usable Up navigation; inactive-tab activation refresh; actual tab edge movement, pane close and remaining-pane native events; native A-dirty notification during pending B; actual pointer Retry producing a native retry report; actual App::on_exit nonblocking stop and subsequent cleanup/thread acknowledgement.

Controlled evidence, explicitly not native failure proof: pending-B terminal success/failure is delivered through the real Tab/Navigation response path because local enumeration cannot reliably remain slow. Its loader request is cancelled for that deterministic timing test; native A dirtiness is real, subsequent reconciliation/listings use actual workers. Permission-denied B failure, overflow, rearm error, capacity error and Busy state are injected into app-facing report/control state. Coalescing/inflight refresh and stale revision/generation/native spelling use controlled reports. The UI Retry check starts from a controlled warning but clicks the actual widget and recovers through a real Service report without filesystem mutation. No actual public-Service overflow, lock-contention or rearm failure induced here; accepted component coverage remains separate and unchanged.

## Coverage proposal and next gate

Source integration and owned-fixture automated evidence cover S018 portions of F13.1/F13.2, preserving F01.3/F01.4/F02.6/F03/F04 and manual F5 behavior. **Do not mark full F13.1/F13.2 verified or production live integration enabled.** Production watching is intentionally inactive pending native capability authority/evidence and supervisor decision.

Fresh routing/state review of the exact candidate, supervisor approval and promotion still required. Native GUI/DPI/Narrator/File Pilot comparison, required clipboard probe, external/slow/removable/network providers, real unknown/rearm/access-permission errors and app-level native overflow remain pending. The legacy notify overflow diagnosis is untouched, not rerun or declared passing. No unavailable manual check passed.
