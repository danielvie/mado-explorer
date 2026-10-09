# P04 directory notification dependency blocker

## Terminal handoff

P04 is blocked before service implementation. No application watcher API is exported or enabled. This is a component-only dependency diagnosis, not accepted F13.1/F13.2 behavior. No criterion statuses were edited. The requested native service, bounded debounce and generation logic remain unimplemented.

Checkout: C:/SANDBOX/WORKTREES/mado-explorer/directory-watch. Branch: lane/directory-watch. Baseline: 36798781b4f9bf8701c54197f90e80d39c1d9cb6. Exclusive writes: components/directory-watch/**. No root, sibling, GUI, settings or operation edits. No delegation, merges, rebases or pushes.

All shell commands explicitly changed to this checkout. Fixtures used unique create-new temporary directories with generated files and sentinel contents. No personal locations, privileges, real drive mappings or shell integration were changed.

## Decision needed

The latest stable notify 8.2.0 Windows backend cannot satisfy the assigned contract unchanged. A real native overflow lost notifications without an error/rescan callback. A bounded queue or debounce wrapper cannot recover an error the provider never emits. Polling would not resolve the required native failure contract and was not substituted.

Approve a narrow audited dependency fix before building the service. A component-local patched notify backend with upstream overflow, initial-read-error and stop/rearm fixes plus explicit terminal-error reporting and joinable driver shutdown is the smallest identified route. Alternatively select a released dependency that supplies these guarantees. No vendoring, FFI rewrite or prerelease switch was performed without that decision. Merely switching to 9.0.0-rc.5 is insufficient because its unknown completion error branch still silently unwatches.

No focused fix attempts were made against dependency-owned code. The observed failure is a provider implementation defect, not a test flake or an ordinary application-code compile error. Do not promote or call this lane passing.

## Windows dependency inspection

Pinned source: [notify 8.2.0 Windows backend](C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/notify-8.2.0/src/windows.rs). The component lock tracks the published crate checksum. The inspected windows.rs SHA-256 is cf53732c805266977136622371a1273eaa0bf3148f734893f393fe752c37b168.

- Lines 331-337 ignore `_bytes_written`. Lines 353-356 accept ERROR_SUCCESS and continue parsing the zero-initialized buffer. Zero-byte successful overflow completions therefore produce no Flag::Rescan and no caller error.
- Lines 357-367 log unknown completion errors, queue internal unwatch and return. ERROR_NOTIFY_ENUM_DIR is not recognized. Consumers cannot expose retry or invalidate from that silent failure.
- Lines 315-326 release allocations/semaphore if initial ReadDirectoryChangesW fails, but return no error from start_read. Registration can acknowledge success without an active read.
- Lines 255-266 call CancelIo and CloseHandle on the issuing driver, then wait alertably for completion before closing the per-watch semaphore. Accepted requests retain heap buffer and OVERLAPPED ownership until the callback converts them back to Box. This is why a wrapper must not free kernel buffers or abandon them early.
- Lines 370-376 rearm successful callbacks unconditionally. There is no stopping flag to prevent a queued successful completion from rearming during cancellation after handle close. This is a source-level lifetime/shutdown concern, not a proven UAF from our smoke test.
- Lines 93-122 create one internal driver thread per watcher, not per path. Its spawn result is discarded. Lines 590-596 only queue Stop and wake the driver; Drop does not join it. Lines 530-543 perform synchronous provider/path work and wait for registration acknowledgement. Any future UI methods must only replace a bounded control snapshot; a fixed worker must own these provider calls without holding shared locks.

Comparison only: [notify 9.0.0-rc.5 Windows backend](C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/notify-9.0.0-rc.5/src/windows.rs), SHA-256 7579ee5642ed15042127d8de92bd4a8cdbcb9b65fb9a3ab31db87a51069101f2. Not a component dependency.

- Lines 423-426 and 544-548 add a stopping flag and suppress rearm for already-queued success during stop.
- Lines 528-530 classify ERROR_NOTIFY_ENUM_DIR and zero-byte ERROR_SUCCESS as Flag::Rescan. Lines 615-623 forward overflow and rearm errors separately.
- Lines 592-602 still log and unwatch unknown completion errors without notifying EventHandler. Lines 885-891 still queue Stop without joining the driver. Therefore this prerelease alone does not finish the assigned error/shutdown contract.

Safe shutdown must request stop, retain provider-owned buffers and wait for confirmed driver completion away from the UI. Uninterruptible provider registration/metadata calls and cancellation completions may delay stopping indefinitely. A bounded UI stop request is possible, but a bounded actual stop time cannot be promised. Returning from the current dependency's Drop only means Stop was queued, not that native cleanup finished. Our smoke test confirms prompt Drop and eventual owned-fixture cleanup on this host only.

## Exact current API

The library has no exported functions or types. It is deliberately unavailable for integration.

The native test probe uses notify::RecommendedWatcher::new, Watcher::watch with RecursiveMode::NonRecursive, Watcher::unwatch and Drop directly. There is no directory enumeration, navigation, rows, pane identity, mutation logic or polling service.

When the dependency blocker is resolved, the assigned service must accept bounded current native locations, expose a coalesced current revision/changed-folder-path report with bounded warning/unavailable/retry state, offer manual retry and nonblocking stop requests, and keep all provider registration on fixed workers. That is a requirement, not a claim that these methods exist here. Duplicate paths/parents, stale generations, initial reconcile, missing-root reattachment, bounded dirty/report state and 150-250 ms debounce all remain next implementation work.

Integration must refresh only matching committed visible locations. If explicit navigation from A to B is pending, defer watcher refresh for committed A; never cancel B to refresh A. Inactive tabs already refresh on activation. A missing-folder reconciliation must retain usable navigation/Up controls. Manual F5, failure/retry feedback and UI wiring remain integration-owned.

## Commands and outcomes

Host: Microsoft Windows NT 10.0.26200.0, X64. Rust 1.95.0, Cargo 1.95.0. No cross-platform release claim.

Commands below ran with the exact prefix `cd C:/SANDBOX/WORKTREES/mado-explorer/directory-watch &&`.

- `cargo fetch --manifest-path components/directory-watch/Cargo.toml`: passed; created only the component Cargo.lock, 27 locked packages.
- `cargo search notify --limit 3` and `cargo info notify`: latest published listing was 9.0.0-rc.5, downloaded only for inspection. The manifest remains pinned to stable 8.2.0.
- `cargo fmt --manifest-path components/directory-watch/Cargo.toml`: applied component-only formatting.
- `cargo fmt --manifest-path components/directory-watch/Cargo.toml --check`: passed.
- `cargo check --locked --all-targets --manifest-path components/directory-watch/Cargo.toml --target-dir target/p04`: passed.
- `cargo test --locked --manifest-path components/directory-watch/Cargo.toml --target-dir target/p04 -- --nocapture`: FAILED, exit 101. One native test passed, one failed, none ignored. No identical failure was rerun and no assertions were removed.
- `task --taskfile components/directory-watch/Taskfile.yml --list`: passed.
- `task --taskfile components/directory-watch/Taskfile.yml`: passed; default lists the component tasks.
- `sha256sum` on both absolute windows.rs paths: passed, hashes recorded above.
- `powershell.exe -NoProfile -Command '[Environment]::OSVersion.VersionString; [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture'`: passed, host recorded above.
- `git diff --cached --check`: passed after staging only components/directory-watch. Inspected the complete component code/Taskfile diff; six changed paths, no root or sibling paths.

Native passing test `native_create_metadata_rename_delete_and_pending_drop` observed Unicode/supplementary-letter paths, file create/metadata write/rename/delete, parent notifications for watched-folder rename/delete/recreation, explicitly re-registered the recreated folder, observed its new sentinel file and preserved outside sentinel bytes. It exercised Drop while reads were pending. This is a direct provider smoke check, not service generation/debounce/automatic reattachment or lifetime proof.

Native failing test `windows_overflow_must_report_rescan` holds the first callback while the already-armed 16 KiB read receives 800 generated long-name empty files, then releases it. Observed output:

```
overflow fixture: generated=801, observed creates=2, rescan/error=false
native events were lost without a rescan/error diagnostic
```

The retained regression intentionally exposes the unmet provider contract. `task test` includes it and currently fails. `task run` runs only the safe ordinary native probe and is explicitly labeled a probe, not a service. `task clean` targets only generated target/p04, not source or user data; cleanup task was not executed after evidence capture.

## Remaining checks and acceptance limits

No F13 acceptance criterion is complete. No application service is enabled.

Pending implementation/checks: fixed-worker/control mailbox and bounded reporting, debounce/coalescing, path/parent deduplication, generation/stale removal, automatic root reattachment, initial registration-gap reconcile, injected error/rescan persistence, manual retry, resource-limit visibility and verified safe provider stop ownership. Current tests do not cover these nonexistent behaviors.

Real unavailable/slow provider, access-denied, symlink/junction aliases, root-parent disappearance, long/verbatim path and actual unknown native error checks remain explicitly pending. No UI, provider cancellation deadline, manual refresh, tab matching, missing-folder presentation, Narrator or performance check was passed. The tests compile to zero Windows probes on other platforms; a non-Windows zero-test result would not be native evidence.

Supervisor should inspect this terminal component-only commit and dependency evidence, classify/resolve the dependency decision, then assign the actual service slice. No rebase/squash/promotion is performed by this worker.
