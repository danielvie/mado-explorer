# P09 operation-drawer handoff

## Candidate and ownership

- Checkout `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`, branch `lane/selection-summary`.
- Original accepted baseline `0f3a5b9ca5e1c32cc8ca13f9c87cd5b8671c8970`. S013-R05 correction starts at frozen, clean P09 candidate `f1a1701bb55b834cedb55abb587ed9fb7b924c09`, not an approved P09 release.
- Candidate is the component-only terminal commit containing this document. Its exact SHA accompanies the worker report; recover it with `git rev-parse HEAD` in this checkout.
- S013-R05 writes confined to `components/operation-drawer/src/lib.rs`, `components/operation-drawer/tests/drawer_ui.rs` and this HANDOFF.md. Root, shared and sibling code, manifests/dependencies/Taskfile, local/main PROGRESS.md, acceptance states and production gates remain untouched.
- Ready for its own immediate fresh review, supervisor approval, serial integration and integration validation. Not self-approved or enabled.

## API

Crate `mado-operation-drawer`, Rust import `mado_operation_drawer`. One direct dependency, installed `egui = "=0.36.2"`; standalone tracked Cargo.lock and workspace.

```rust
pub struct OperationId { pub window: u64, pub sequence: u64 }
pub enum Kind { CreateFile, CreateFolder, Rename, Copy, Move, Trash, Delete }
pub struct Counts { pub completed: u64, pub skipped: u64, pub failed: u64 }
pub enum Progress {
    Unknown, Enumerating, WaitingForConflict,
    Known { processed: u64, total: u64 },
}
pub struct Status {
    pub current_item: Option<std::path::PathBuf>,
    pub progress: Progress,
    pub counts: Counts,
}
pub enum Outcome { Completed, Cancelled, Failed }
pub struct Issue { pub path: std::path::PathBuf, pub error: String }
pub enum Update {
    Running(Status),
    Stopped { status: Status, outcome: Outcome, issues: Vec<Issue> },
}
pub struct Report { pub id: OperationId, pub revision: u64, pub update: Update }
pub enum Command { Start(Request), Cancel(OperationId) }
pub enum State { Queued, Active, Cancelling, Stopped(Outcome), CancelledBeforeStart }
pub enum Refusal {
    ForeignWindow, ReusedId, PendingCapacity, HistoryCapacity,
    MetadataCapacity, StaleReport,
}
```

- `Request::new(id: OperationId, kind: Kind, sources: Vec<PathBuf>, destination: Option<PathBuf>) -> Result<Request, Refusal>` freezes private native-path metadata. Only bounded metadata admission happens here. No filesystem/name/namespace/identity validation or confirmation. Empty paths/sources and all native namespaces are metadata, not accepted filesystem targets. Constructor consumes arguments; retain caller input if constructor refusal needs subsequent inspection. Show its error through `Refusal::message()`.
- Request getters `id() -> OperationId`, `kind() -> Kind`, `sources() -> &[PathBuf]`, `destination() -> Option<&Path>` expose no mutable snapshot access.
- `Operations::new(window: u64) -> Operations`. One instance owned by the window, never a pane/tab. Window namespace must be unique in its egui context. Accepted request sequences must strictly increase, including after terminal dismissal; the constant-size high-water mark rejects reuse without tombstones. A refused ID does not advance it.
- `enqueue(&mut self, request: &Request) -> Result<(), Refusal>` retains a cloned snapshot and reserves its terminal-report slot. Refused borrowed inputs remain caller-owned. No Start is emitted here or by `show`.
- `next_command(&mut self) -> Option<Command>` consumes the next output. With no active request, promotes FIFO front and returns exactly one Start. While active, returns at most one Cancel after a cancellation request, otherwise None. Adapter must retain/deliver the consumed output; there is no retry or delivery acknowledgement mechanism here.
- `cancel(&mut self, id: OperationId) -> bool`. Queued request is removed from FIFO and retained as CancelledBeforeStart; it emits no Start or Cancel. Active becomes Cancelling immediately and remains active until a worker terminal report. Repeated/stale/foreign cancellation returns false.
- `report(&mut self, report: &Report) -> Result<(), Refusal>`. Only the active ID and increasing nonzero revisions are accepted. Queued, foreign, old, duplicate and completed reports cannot alter active status or restart work. Report admission is atomic. Stopped means the worker actually reached its stopping point; it releases active ownership and retains the supplied status/outcome/issues. Later polling may start the next FIFO request.
- Running reports update only status, never clear Cancelling. Completed acknowledgement after cancellation shows "Worker finished after cancellation request", not successful cancellation. Cancelled acknowledgement preserves reported completed counts and explicitly says completed work remains. No rollback or automatic retry/resume exists.
- `acknowledge(&mut self, id: OperationId) -> bool` explicitly removes a retained terminal report only. UI label is "Acknowledge and dismiss report". No silent eviction, filesystem cleanup or retry.
- Read-only inspection `active() -> Option<&Record>`, `queued()/history() -> &VecDeque<Record>`, `notice() -> Option<Refusal>`, `is_open() -> bool`. Record getters `request()`, `state()`, `status()`, `issues()` return the retained data read-only.
- `Progress::fraction() -> Option<f32>` accepts only positive total and processed <= total. Invalid/zero/unknown/enumerating/conflict-waiting progress is indeterminate, not clamped or fabricated. Units in each known report must agree; caller decides bytes versus items.
- `show(&mut self, ui: &mut egui::Ui) -> DrawerResponse` renders a nonmodal Operations toggle and scrolling active/queued/history drawer. Returns `toggle: egui::Response` and `controls: Vec<ControlResponse>`, each with `id: OperationId`, `action: Action`, `response: egui::Response`. `Action` is Cancel, Details or Acknowledge. These are optional focus-routing outputs, not backend commands.

## UI and bounds

- Shows kind, ID, fixed source/destination, current item, known progress and completed/skipped/failed counts. View paths and reports toggles all supplied source paths and actual unresolved/recovery path/error reports. Paths truncate visually with full display text on hover; command data is always native PathBuf, never reconstructed from text.
- Active and queued cancellation, report viewing, report acknowledgement and drawer toggle are functioning controls. No disabled placeholder executor controls. Actual Tab, Space, Enter and pointer activation use egui button semantics. Focus has a two-point outline and newly focused controls scroll into view.
- Nonmodal status/conflict waiting leaves another browsing input usable. Origin contexts are not stored or consulted; closing their data cannot remove requests or reports.
- Pending includes active plus FIFO, maximum 16. History plus pending reserves at most 32 records. At capacity, additions visibly refuse instead of evicting successful, failed or recovery reports. Queued cancellation uses its reserved slot. Retained failure reports require explicit acknowledgement.
- Maximum 128 source paths per request. Each source/destination/current/issue path permits at most 65,536 `OsStr::as_encoded_bytes()` bytes; source plus destination metadata permits at most 262,144 bytes per request. No path is shortened to fit.
- Maximum 64 terminal unresolved/recovery issues, each error at most 4,096 UTF-8 bytes, combined issue path/error bytes at most 262,144. Current item has a separate 65,536-byte limit. Oversized reports refuse atomically, keep the active request and expose a visible notice. Caller retains the refused report; backend integration must provide complete bounded reporting or a reviewed external report store, never silently truncate unresolved items.
- Controller native metadata payload is at most 18 MiB across 32 reserved/retained records, plus bounded containers/record overhead. Caller-owned reports/requests and consumed command storage are outside that bound.
- One stable scroll ID and one stable tooltip presenter layer per window. At admission each record receives one of 32 presentation slots. Slots follow queued/active/history records and are reusable only after explicit terminal acknowledgement. Three independent control IDs per slot and one toggle ID bound the focusable button cache keys to 97 per window, regardless of operation sequences or controller recreation. IDs remain immutable command/report data, not presentation keys. See the ownership guards and SDK proof below.
- Truncated labels still disable egui's automatic elision tooltip, which otherwise leaks per-operation Areas. Tooltip visibility checks still use the same bounded parent ID as its presenter. Details/open/input-guard flags live in the bounded controller, not per-operation egui storage.
- UI work is bounded by the above record/path/issue limits. No threads, file I/O, executor, clipboard, native probes, navigation coupling, retry/resume or production adapter.

## Original P09 checks

Historical evidence for frozen P09 candidate f1a1701, not S013-R05 reruns. Windows host; Cargo 1.95.0, Task 3.50.0. All commands start in the explicitly named worktree. Original successful commands:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo generate-lockfile --manifest-path components/operation-drawer/Cargo.toml --offline
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo fmt --manifest-path components/operation-drawer/Cargo.toml --check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo check --manifest-path components/operation-drawer/Cargo.toml --target-dir target/p09 --locked --offline --all-targets
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo test --manifest-path components/operation-drawer/Cargo.toml --target-dir target/p09 --locked --offline --test drawer_ui
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo run --manifest-path components/operation-drawer/Cargo.toml --target-dir target/p09 --locked --offline --example generated
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && task --taskfile components/operation-drawer/Taskfile.yml
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && task --taskfile components/operation-drawer/Taskfile.yml check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && task --taskfile components/operation-drawer/Taskfile.yml test
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && task --taskfile components/operation-drawer/Taskfile.yml run
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && task --taskfile components/operation-drawer/Taskfile.yml --dry clean
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && git diff --check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && git diff --cached --check
```

- Formatting and locked offline all-target compile pass without warnings. Focused suite passes 13 tests, none ignored. No full root suite or Clippy.
- Real headless egui input covers FIFO/two queued/one active, queued cancel without Start, active cancel plus running/late-completion/actual-stop reports, fixed destination/origin data destruction, stale/foreign/duplicate/queued/completed reports, known/unknown/invalid/conflict totals, failure counts/recovery details/explicit dismissal, Tab/Space/Enter focus and pointer controls, late pointer release, native malformed-UTF-16 display collisions/verbatim namespace retention, visible capacity refusal and retained failures, metadata admission, nonmodal browsing input, all operation kinds, and bounded retained egui data/Areas across 200 sequential operations with real full-path hovering.
- Generated demo renders 40 shapes, asserts one active/two queued and cancellation waiting/terminal acknowledgement with completed counts retained. No desktop window or filesystem source access.
- Task default lists tasks, check/test/run pass. Clean was dry-run only; it targets only worktree-root `target/p09`. All cargo build/test/run/check commands use the explicit manifest and that target directory.
- Initial all-target check used the old egui `Context::style_mut` spelling in a test. Corrected to installed 0.36.2 `global_style_mut`.
- Initial suite had 10 passing tests and two failures. One headless namespace probe dropped unapplied font deltas; fixed by explicitly clearing them. Retained-state check exposed automatic Label elision tooltip Areas. Disabling that automatic tooltip exposed a mismatched tooltip visibility-parent ID, fixed to the presenter's stable identity. The original retained-state and real-hover assertions then passed unchanged; no test was weakened.
- Initial Task default/run used a top-level `dir` that this Task version did not apply. Corrected to task-local `dir: ../..` for cargo tasks and component-local `task --list` for default. Both reruns pass. No root output was cleaned.

## S013-R05 finding and correction

The fresh review correctly found that the original comment about child IDs not retaining egui state was false. Buttons register their IDs in egui's private focus cache. Dismissing Rust records does not remove those cache entries without cardinal navigation. The existing 200-operation data/Area/hover regression did not measure this cache.

### Identity and input ownership

- Admission scans at most 32 reserved/retained records to find a free slot. No second ownership manager, report eviction, generation-key accumulation or dependency changes. Queued cancellation and terminal reports retain the same slot. Refusal, metadata limits, atomic report admission, immutable native paths, one-active FIFO, exactly-once Start/Cancel and honest worker-stop acknowledgement are unchanged.
- The window namespace, each record's explicit slot scope and each button's explicit child `UiBuilder::id` seed are independent of request sequence, text, displayed order, metadata/progress controls, hidden action roles and caller layout. Normal egui Buttons still provide keyboard/accessibility labels and focus outlines. Public API signatures are unchanged. Returned `ControlResponse.id` remains the actual OperationId; `response.id` is now a reusable presentation identity, not a permanent operation identity.
- Each newly admitted record has fresh local guards. Before handling input the presenter surrenders only its own inherited control focus. Recreating the controller also resets the toggle guard. No global memory clearing, input consumption or fake arrow/cardinal navigation occurs. Browsing and another window's focus/input stay usable.
- First-visible-frame activation is ignored, including repeated layout passes. A pre-existing primary hold is disarmed through its release. Held Enter/Space from a previous owner cannot activate the replacement, even if focus is restored externally. Explicit release permits a new intentional press. Hidden Cancel/Acknowledge roles sample the ownership epoch without sharing IDs; reopening the same operation's drawer does not create a different operation's keyboard epoch.
- Frame-local activation guards prevent cached clicks/keys from toggling details twice during layout retry. A pointer release must also be inside the control's current interaction rectangle because egui hit-tests the previous pass. This prevents an action on a moved row's old rectangle.
- Tradeoff: finite IDs require these local input guards rather than egui IDs encoding operation identity. The opening presentation frame is deliberately inert, even for a fresh press arriving before the new row has been presented. The same frame number must be reused across layout retries; integrations must render each window presenter once per pass. SDK Button auto-ID behavior is pinned and checked by debug assertions. No native Narrator/DPI/GUI claim is made.

### Installed SDK evidence

All SDK paths below are under `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.36.2/src/` and were read without modification.

- `context.rs:1256-1270` registers enabled focusable widget IDs with Memory. `memory/mod.rs:655-658` inserts them into `Focus::focus_widgets_cache`; `623-625` calls cardinal lookup only for cardinal navigation and `732-739` prunes inside that lookup. Tab/Enter/pointer cycles need not prune. The cache and Focus type are private; there is no public cache-cardinality accessor.
- Boundedness therefore uses actual registered Response IDs plus this SDK insertion proof, not a proxy `data.len()` count. The sequential regression verifies actual `Context::read_response` focusable/enabled registrations and finds exactly four IDs across 384 pointer/Enter cycles and eight controllers. The all-slot regression registers all 97 possible IDs with 32 retained records and checks identity across promotion, terminal retention and explicit acknowledgement/reuse. Neither invokes cardinal navigation.
- The exact 97 count concerns focusable Buttons. `widgets/label.rs:145-150` also makes labels focusable when `Memory.options.screen_reader` is enabled. Record label identities use explicit bounded slot scopes rather than request sequences; their allocation counts are bounded by the existing path/issue/byte caps. No disabled screen-reader setting or label accessibility change was introduced. Native Narrator remains unverified.
- `ui_builder.rs:59-69` documents parent-independent explicit IDs. `ui.rs:250-260` seeds the explicit child auto ID with `id.value() + 1`. `atomics/atom_layout.rs:298,434-438` uses that first ID and `Ui::interact`; `widgets/button.rs:370` allocates the Button layout. Two debug assertions check the predicted and actual Response ID.
- `context.rs:1466-1473` derives keyboard activation from current focus and Enter/Space; `1515-1528` derives pointer clicks from cached hit-testing. `memory/mod.rs:926-930` surrenders focus only if the supplied ID owns it. `context.rs:1746-1752` defines a frame number that increments once per run, not once per discarded layout pass.

### Red and green checks

New checks are prefixed `r05_`; all thirteen original `p09_` behavioral tests and failure assertions are unchanged.

- Before source correction, the four initial new checks produced three failures and one pass. Sequential focus-ID cardinality reported 193 IDs, promotion changed a record's Details ID, and restoring replacement focus while the old key remained held cancelled the new operation. The old unique-ID pointer test passed on the original source; it now protects the finite-ID correction with delayed release/recreation checks for Cancel, Details and Acknowledge.
- The first coherent correction run passed 15/17. It exposed an overly broad visibility-key epoch in the original keyboard test and a missing key release in the new sequential fixture. Corrected ownership sampling for hidden roles without changing the original test. The new fixture now emits its real key release before the next operation. The subsequent 17/17 run passed.
- A new browsing-focus check initially sent Enter into a single-line TextEdit, which legitimately surrendered its own focus. Corrected that new fixture to type text without submission; it now checks preserved browsing focus and actual other-window Space activation. No production guard was relaxed for this fixture.
- The moved-row regression was red on the bounded-ID implementation: three Source labels instead of two showed stale release activation. Current-rectangle validation made it green.
- Ten new headless checks cover sequential finite registrations, all 32 slots/reservations/queued cancellation, explicit reuse, stale/recreated focus and held Enter/Space, old press/new owner/later release for every action, opening cached pointer clicks, pointer and keyboard layout retries, independent Cancel/Acknowledge roles, browsing/other-window focus, moved-row geometry and batched release/new-intent rearming. Existing tooltip/data/Area and late-pointer-release checks still pass.

Final results: formatting passes; locked offline all-target compile passes without warnings; focused ten new checks pass; full component suite passes 23 tests, none ignored, with zero library/doc tests. Diff whitespace check passes. Git reports its existing LF-to-CRLF working-copy warnings only. All run from the exact owned checkout and use worktree-root `target/p09`. No root suite, Clippy, executor, filesystem-operation, clipboard or actual-GUI checks were run.

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo fmt --manifest-path components/operation-drawer/Cargo.toml --check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo check --manifest-path components/operation-drawer/Cargo.toml --target-dir target/p09 --locked --offline --all-targets
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo test --manifest-path components/operation-drawer/Cargo.toml --target-dir target/p09 --locked --offline --test drawer_ui r05
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && cargo test --manifest-path components/operation-drawer/Cargo.toml --target-dir target/p09 --locked --offline
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && git diff --check
```

## Acceptance and next adapter

Full release criteria satisfied: none. Component portions implemented and checked:

- F12.1 status drawer, supplied path/current/progress/count data and window-lifetime retention. Actual application window/pane-close adapter and real reports pending.
- F12.2 truthful indeterminate/known progress and nonmodal UI. Background enumeration/transfers/conflict workflow responsiveness pending.
- F12.3 one active Start request, visible FIFO and queued cancellation. Native mutation serialization/execution pending.
- F12.4 cancellation-request UI and terminal-acknowledgement honesty only. Cooperative scheduling/chunk checks, completed work preservation on disk and uninterruptible OS-call behavior are backend gaps.

No F12.5 retry, F12.6 application-close prompt, F12.7 crash recovery or F02.4 captured native file-identity guarantee. Reports can display recovery data, but do not implement recovery.

Integration owner is the supervisor/serial integration lane. Smallest next adapter, after fresh review and checkpoint permission gates: add the component dependency and one Operations instance to root's window state; allocate window-scoped increasing IDs; capture fresh visible native paths and destination at invocation; draw show in the window drawer and route refusal/focus responses. Connect only reviewed backend commands/reports with matching IDs/revisions and bounded delivery. Until an executor is independently reviewed, Start must remain a request, not authorization to mutate files. Capture/revalidate native identities, validation, confirmations, conflict handling and transfer cancellation belong to that backend, not this display-path DTO. Do not persist/restore this queue or automatically retry after restart.

Narrator, native DPI/narrow-window interaction, full application integration, real operation errors/cancellation, performance and personal-data safety are unverified. Existing clipboard/checkpoint gates remain unchanged. No desktop, clipboard, native filesystem or personal-folder checks were performed.
