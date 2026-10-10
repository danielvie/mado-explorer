# P17 native directory notification service

## Candidate and boundary

Checkout C:/SANDBOX/WORKTREES/mado-explorer/native-directory-watch, branch lane/native-directory-watch. Before any writes, verified exact HEAD 87d90f2f78886d1e3fde9168e34fed8aa909f761, expected branch and empty git status. Read live main AGENTS.md, SPEC.md, CONTEXT.md, PROGRESS.md S016/P17 and LANES.md, checkout AGENTS.md, and original P04 HANDOFF/source/probes. The supervisor's direct ReadDirectoryChangesW/IOCP decision supersedes the old no-overlapped-I/O restriction.

All source/documentation changes are components/directory-watch/**. Build outputs are checkout target/p17 and target/p17-diagnostic-notify. No root/sibling/app/clipboard/settings edits, delegation, rebase/merge/reset/push, personal-folder watching, elevation or external provider probing. Backend-only partial F13.1/F13.2. No acceptance status or production integration claim.

The enclosing component commit is the frozen candidate for fresh exact-head unsafe-code/lifetime review. No promotion or personal-data enablement is authorized by these checks.

## Public integration contract

`Service::new()` starts one fixed window-owned driver. Thread spawn failure returns io::Error; port initialization failure is an asynchronous sticky driver error and stopped acknowledgement. No callback API is exposed, so application callback panics cannot unwind native IO. Provider/driver unwinding is caught and cleanup attempted.

- `replace_targets(revision, Vec<Target>)`: increasing revisions starting above zero. Immutable Target contains ID, generation and original native PathBuf. Whole-snapshot coalescing, no partial admission. Capacity, duplicate IDs, overlong paths, stale revision, stopping and contended control lock return explicit Refusal.
- `retry(revision, id, generation)`: current-target-only retry; coalesced by ID. Re-registers unavailable directory/parent requests and reconciles. Shared parent watches may recover other matching targets too.
- `take_report()`: nonblocking, coalesced current-revision TargetReports; dirty/full flags survive subsequent state-only or error reports. Errors, readiness and overflow are separate fields. The caller must retain the latest per-target state between reports. Sticky driver errors remain readable after report consumption.
- `request_stop()`: atomic nonblocking request plus coalesced IOCP control wake. No filesystem/provider cancellation or waiting on this caller.
- `stopped()`, `join_after_cleanup()`: cleanup acknowledgement followed by a join only after the thread has actually finished. False means keep waiting outside UI blocking paths. Drop requests stop and detaches, never joins. Prefer retaining Service until acknowledgement.

UI methods perform bounded memory/lexical work and try_lock only. PostQueuedCompletionStatus is the sole native control-wake call. Every open/metadata/read/rearm/cancel/drain call belongs to the driver. It never holds the shared mailbox lock over provider calls. The 25 ms IOCP timeout checks controls/stop/debounce only; it does not poll filesystem paths.

App adapter must refresh only matching committed visible locations at matching revision/ID/generation/native path. A pending A-to-B navigation must NOT be cancelled to refresh committed A. Inactive tabs already refresh on activation. Preserve missing-location/Up, warning, unavailable/retry and manual F5 UI. Report.ready is not an authoritative listing.

## Native behavior and limits

A validated internal UTF-16 key deduplicates identical directory and immediate parent paths, including ordinary/verbatim and ordinary separator variants. Original command paths are unchanged. No case folding or guessed alias equivalence. Native volume/file identity detects current named-path replacement. Different aliases/case spellings may consume distinct registrations rather than guessing case sensitivity.

CreateFileW, native disk-directory/identity metadata, IOCP association and initial ReadDirectoryChangesW submission must all succeed before ready. Initial full reconciliation covers the registration gap. Successful nonempty completions invalidate every matching target. There is no FILE_NOTIFY_INFORMATION parsing, enumeration or content read in production. Parent/directory events trigger bounded worker-side named-path revalidation and reattachment.

Debounce is 200 ms quiet time, maximum 800 ms accumulated delay, plus driver scheduling/provider latency. Provider calls can still be slow without blocking the UI. Zero-byte success and ERROR_NOTIFY_ENUM_DIR set sticky overflow plus full reconciliation. An ERROR_NOTIFY_ENUM_DIR submission failure also exposes overflow independently of unavailable status. ERROR_IO_PENDING is retained as an accepted overlapped submission, never freed as an ordinary failure. Rearm failure preserves that dirty/warning data and exposes a separate unavailable code. Initial/read/rearm/unknown completion errors are sticky until retry, even if their numeric code happens to equal FILE_NOT_FOUND. Overflow survives unchanged targets across revision updates; explicit retry resets it.

Windows root deletion actually produced completion ERROR_ACCESS_DENIED. Only this completion code, followed by an independently observed missing native name after the drained request closes, becomes Failure::Disappeared with both codes retained. That confirmed disappearance can reattach on parent events. Access denied for a still-present name, initial/rearm failures, arbitrary unknown codes and unexpected aborts remain unavailable until retry. Lost parent remains unavailable and needs retry/manual F5; no ancestor watch or recursive fallback is added.

Admission accepts validated drive-absolute, UNC and verbatim drive/UNC/volume-GUID filesystem names; ordinary separator variants use a lossless internal verbatim opening spelling. Reject NUL, direct devices/pipes, local DOS aliases, relative paths, unsafe dot/trailing/empty components and unsupported spellings before provider I/O. Supported Unicode, unpaired surrogate and long native paths are preserved. Providers without usable native directory identity fail explicitly. Trailing separators on non-root paths, unusual verbatim component spellings, alternate device namespaces and unrestricted normalization are not supported. Full namespace/provider compatibility beyond this bounded contract would need a separately commissioned shared admission API decision; no sibling changes were made.

## Bounded ownership and shutdown

At most 32 current targets, 32 coalesced retry IDs and 32 report records. One coalesced queued native control-wake packet. At most 64 TOTAL active plus retiring requests, each with one stable boxed UnsafeCell storage allocation holding an aligned 16 KiB buffer and OVERLAPPED, and one owned directory handle. One additional transient observation handle and one IOCP handle give a maximum of 66 native handles owned by this service. Path/target/report/control snapshots have fixed cardinality and native-unit limits. Eight distinct sticky driver errors are retained; further error occurrences have an explicit omitted counter, never an unbounded log.

Retiring is marked before cancellation. Routing is removed before cancellation/stop. A queued successful retired completion cannot rearm. CancelIoEx success or ERROR_NOT_FOUND never clears pending. Only dequeue of the exact submitted key/OVERLAPPED clears pending and permits closing/freeing. Never create Rust references to pending kernel-written buffer/OVERLAPPED fields; UnsafeCell and raw field pointers preserve that interior mutability.

Retirees count against admission until their completions drain. Resource pressure is visible and requires retry rather than admitting unbounded replacement handles. Stop cleanup drains all submitted completions before acknowledging stopped. Arc-owned wake-port lifetime spans posting and shutdown. There is NO provider cancellation or cleanup deadline guarantee.

If cleanup itself unwinds before pending completions can be proven drained, Request::Drop deliberately retains only those bounded kernel-visible handles/allocations and does not acknowledge stopped. This last-resort safety retention is not successful cleanup. The driver error is visible. Normal and injected-unwind fixture checks drain and free normally. Fresh review must inspect this path, pointer alignment, association/submission and retirement ordering.

## Focused checks and evidence

Host: Windows NT 10.0.26200.0 X64; rustc 1.95.0 and cargo 1.95.0. Commands ran with prefix `cd C:/SANDBOX/WORKTREES/mado-explorer/native-directory-watch &&`:

- `cargo generate-lockfile --offline --manifest-path components/directory-watch/Cargo.toml`: passed, two dependency packages. windows-sys pinned =0.61.2, installed dependency only.
- `cargo fmt --manifest-path components/directory-watch/Cargo.toml`: applied owned-source formatting.
- `cargo fmt --manifest-path components/directory-watch/Cargo.toml --check`: passed.
- `cargo check --locked --all-targets --manifest-path components/directory-watch/Cargo.toml --target-dir target/p17`: passed.
- `cargo test --locked --manifest-path components/directory-watch/Cargo.toml --target-dir target/p17 -- --nocapture`: final replacement-service suite passed, 18 tests, zero failed/ignored. The caught injected-unwind test intentionally prints a panic hook message but passes after proving storage retention and explicit drain.
- `cargo check --locked --all-targets --manifest-path components/directory-watch/diagnostic-notify/Cargo.toml --target-dir target/p17-diagnostic-notify`: passed compilation ONLY, not legacy behavior verification.
- `task --taskfile components/directory-watch/Taskfile.yml run`: passed the actual public-Service owned lifecycle check, one passed/zero ignored.
- `task --taskfile components/directory-watch/Taskfile.yml --list` and default task: passed. Diagnostic subcrate default task also lists its separate tasks.
- `git diff --check` and staged diff check: passed. Inspected actual owned production source, tests and manifests before freeze.

Actual owned native checks include public-Service Unicode create/rename/delete, disappearance via rename and recreation, automatic reattachment, initially absent directory, direct watched-root deletion/recreation, lost-parent/retry recovery, long/unpaired native names with original-path preservation, equivalent-spelling deduplication, actual 800-empty-file long-name overflow, queued-success cancellation, delayed retirement pressure and explicit cleanup/join. All fixtures use a create-new owned container with target and parent INSIDE it, never the personal TEMP root itself. Every fixture checks an outside-watched-parent source sentinel before removing its generated container.

Injected checks are separately named: initial-read and rearm errors, zero-byte/ERROR_NOTIFY_ENUM_DIR, arbitrary unknown completion and unexpected abort, access-denied on still-present target, warning plus rearm failure, retry/sticky status and caught unwind. Completion injections occur only AFTER an actual native completion dequeues, not while the kernel still owns pending memory. Pure checks cover control/retry/report coalescing, Busy/capacity/stale refusal, report/error bounds and debounce maximum delay. None of these injection checks claim native provider-failure verification.

During development, the first suite had 10 passing tests and one lost-parent fixture failure: renaming a directory containing an open watched child returned native access denied. Corrected to owned child removal/drain followed by empty parent deletion; focused recovery passed. A later added direct-root-delete check exposed actual completion code 5 blocking reattachment. One diagnostic run retained the code/state evidence; the confirmed-missing-name correction passed its focused check and full suites. No ignored assertions, discarded failing checks, repeated unsuccessful fixes or unsafe fallback.

Pending native evidence: real unknown error/initial-read/rearm failure, real unavailable/slow/removable/network provider, access-denied permissions fixture, junction/alias retarget behavior, volume-GUID provider and broad namespace compatibility. No external/native privileged probes, application UI, Narrator, performance gate or complete F13 acceptance checked. A non-Windows zero-test build is not evidence; this component is Windows-only.

## Preserved legacy notify diagnosis

Original checkout C:/SANDBOX/WORKTREES/mado-explorer/directory-watch remains clean at c39e27322b045e688b623bbc26c5757c7c983324, archived archive/p04-notify-diagnostic. Its known overflow diagnostic remains FAILED: generated=801, observed creates=2, rescan/error=false; assertion says native events were lost without a rescan/error diagnostic. That identical old failure was not rerun here and notify was NOT fixed.

`diagnostic-notify` is an excluded standalone subcrate. Cargo.toml, Cargo.lock, src/lib.rs, tests/native_probe.rs and original HANDOFF.md were copied byte-for-byte and compared to the unchanged old checkout. Staged Git blobs also match the original c39e273 blobs exactly for all five copied files, independently of checkout line-ending conversion. Only its local Taskfile paths were adapted for separate output and clear expected-failure labeling. The failing overflow assertion is intact, not ignored/weakened. Replacement `task test` does not include the legacy provider; `task diagnostic-notify` or the diagnostic subcrate's `task test` explicitly runs the still-failing probes with target/p17-diagnostic-notify.

Copied file SHA-256 values:

| Original-relative file | SHA-256 |
| --- | --- |
| Cargo.toml | 3e012f75176092083a25449322ffb86669857defd8e045db254ec005ff922e45 |
| Cargo.lock | 6fcfd30983368192d8ff8aef7b84ca033f7aab2b1e70df9a5dcf634dc20e3c53 |
| src/lib.rs | d5f80aea0a8c77d67a652d1fa0dd980bc133cb7fd79dc5948408c0283061a05d |
| tests/native_probe.rs | c7473d6e511c188f8d8008e48b32cfe761139cfc72564a9809a7136ba3fe4b56 |
| HANDOFF.md | e9cc1f93ce33880187f392efc3f6de3dd6003a60bf5818ea368f63d25dbb695e |

Next gate: supervisor inspect frozen candidate and dispatch fresh unsafe-code/lifetime review before promotion. Follow-up application refresh adapter remains supervisor-owned and must preserve pending navigation.
