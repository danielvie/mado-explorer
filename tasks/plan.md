# Rust/Windows file manager implementation specification and plan

## Status and authority

Implementation authorized by the user on 2026-10-13, including the proposed defaults. Native gates and actual acceptance evidence remain required. Working application identity: Wayfarer Files (wayfarer-files); independent versioned state remains planned. Native integration checkpoints must pass before the candidate stack is accepted for the release.

The existing [recovered specification](C:/SANDBOX/REPOS/rev-filepilot/SPECIFICATION.md) remains an evidence document. This plan defines proposed behavior for an independent Rust application, not claims about File Pilot. Unknowns in the recovered spec become explicit design choices or later investigation tasks, never guessed compatibility requirements.

The canonical task checklist is [tasks/todo.md](C:/SANDBOX/REPOS/rev-filepilot/tasks/todo.md). Task status lives there, not in this plan. No existing plan/task files were present when this draft was prepared. The app/ Cargo project and application Taskfile now exist; see tasks/todo.md for implementation and verification status.

## Objective

Build a Windows desktop file manager with responsive directory browsing, keyboard selection, tabs/splits, safe file operations, Explorer interoperability, previews, and reliable session restoration. A first release must be useful without copying File Pilot's proprietary source, assets, shaders, brand, or persistence files.

Success means the MVP acceptance checkpoint passes on Windows with disposable fixtures. It does not mean pixel equivalence, binary/source recovery, identical search scoring, or complete File Pilot feature parity.

## Confirmed context and proposed decisions

| ID | Status | Decision and reason |
| --- | --- | --- |
| D01 | Confirmed context | Rust application; Windows is the initial target. |
| D02 | Proposed | Develop/test on Windows 11 x64 first. Windows 10 and ARM support are not claimed until separately verified. |
| D03 | Proposed, gated | Use eframe/egui, egui_extras, and egui_dock for a Rust-rendered UI. Accept only after HWND/COM/menu/drop/accessibility gates pass. |
| D04 | Proposed | Create one application crate under `app/`. Keep `FPilot.exe`, `analysis/`, the evidence spec, and root Taskfile unchanged. |
| D05 | Proposed | Use Microsoft's `windows` bindings for Shell, COM, clipboard, WIC, and HWND integration. Standard-library workers handle ordinary enumeration. No async runtime, database, or plugin framework initially. |
| D06 | Proposed | Use application-owned typed values and a versioned session schema. No original JSON interoperability or recovered memory layouts. |
| D07 | Proposed | Native Shell popup first; searchable custom Shell presentation later. This makes compatibility testable before recreating menu styling. |
| D08 | Proposed | Read-only Shell/drag probes precede production transfer wiring. Use one transfer authority per operation/drop to prevent duplicate mutations. |
| D09 | Proposed | Implement safe outcomes, clipboard handling, and path validation rather than copying recovered unchecked behavior. |
| D10 | Proposed | Delegate MVP incoming filesystem transfers synchronously to the acquired Shell IDropTarget. Do not also enqueue an application transfer; return delegated status/effect and rescan. A custom asynchronous OLE protocol requires separate review. |

Pin compatible dependency versions and the supported Rust toolchain during T01 after approval. The workstation currently provides Rust/Cargo 1.95.0 and Task 3.50.0; this is environment inventory, not a chosen minimum supported Rust version.

Potential supporting dependencies are `serde`, `serde_json`, and `notify`, added only when their tasks need them. WIC is the initial Windows image backend. Do not add a second image decoder, Tokio, Rayon, a fuzzy-search crate, or a native-dialog crate without an actual need and review.

Framework/API references: [egui](https://github.com/emilk/egui), [virtualized table rows](https://docs.rs/egui_extras/latest/egui_extras/struct.TableBody.html), [egui_dock](https://docs.rs/egui_dock/latest/egui_dock/), [Windows bindings](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/index.html), and [IFileOperation requirements](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileoperation). Verify pinned-version documentation before implementing API-dependent tasks.

## Capability map

These are responsibility boundaries, not a requirement to create separate crates or empty modules. Create files only when a task needs them. The map is proposed for approval.

| Module ID | Responsibility | Depends on |
| --- | --- | --- |
| state | Owned paths, locations, listing generations, selection, tabs, panes, operation values | None |
| directory | Asynchronous enumeration, filtering/sorting inputs, watcher invalidation | state |
| windows-adapter | HWND/COM lifecycle, Shell menus, clipboard, OLE, filesystem operations, WIC | state |
| commands | Typed command IDs, eligibility, focus-sensitive shortcuts, palette | state |
| preview | Bounded text/image requests and decoded results | state, windows-adapter for WIC |
| persistence | Validated settings/session files and durable replacement | state |
| ui | Views and user intents; applies owned results without blocking I/O | All provider modules |

Provider modules never depend on egui widgets or the UI module. Windows callbacks receive an adapter-owned snapshot and emit/execute defined intents; they do not borrow a mutable widget frame. Mutual dependencies are a signal to merge responsibilities or redesign the boundary, not to introduce a service locator.

Build order: minimal window and native host, then a folder-browsing vertical slice; Shell/copy/drop risk probes; selection and layout workflows; safe transfer/preview/persistence slices; MVP acceptance; advanced features.

## Release scope

### MVP, T01 through T24 plus T43 through T45

- One ordinary filesystem directory can enumerate asynchronously into a virtualized details view with name/type/size/modified columns and explicit loading/error state.
- Mouse/keyboard selection, open/parent/back/forward, path editing, sorting, basic filename filtering, hidden/system visibility, and refresh work.
- Tabs and horizontal/vertical split panes preserve independent locations and selection.
- Built-in commands and a searchable command palette are focus-aware. Native Shell menus are available separately.
- Copy to a destination, copy/cut/paste, move, Trash, create file/folder, and single-item rename have explicit outcomes and cancellation/partial-failure handling.
- Explorer file-path drag/drop works in both directions for supported ordinary filesystem items. Incoming drops target the current folder of the pane under the pointer. Hover navigation, per-row drop destinations, and virtual-file streams are deferred.
- Bounded text/image inspector, watcher-driven refresh, and validated session restoration work.
- Keyboard-only interaction, visible focus, Narrator semantics, IME text entry, and mixed-DPI checks pass before release.
- Distribute a release executable without changing default folder handlers, registry shell integration, or PATH.

### Later capability coverage, T25 through T42

Searchable custom Shell menus; Recycle Bin listing and selected-item actions; expanded listings; folder statistics; icon views/thumbnails; batch-rename preview/commit; saved layouts; bookmarks/recents; typography/input preferences; Explorer/terminal launching; shortcut creation; permanent deletion; editable hotkeys; extension acceptance.

Later tasks target capabilities, not every original option or unknown interaction. Do not expose settings with no implemented behavior. Native column views, multiple native windows, exact rename-date syntax, all 40 recovered options, and precise original key/matching semantics require additional reviewed scope if demanded.

### Explicit exclusions

No Linux/macOS, archive extraction, media playback, cloud sync, network protocols beyond normal Windows paths, plugins, licensing emulation, original config-file import, proprietary shader/assets reuse, automatic elevation, installer/default-folder registration, global Recycle Bin emptying, or general virtual-file/stream drops.

No production functionality is created merely to satisfy a decompiler quirk. Advanced destructive actions remain absent or visibly unavailable until their task is accepted.

## Behavioral contract for the new application

### Browsing and selection

Keep `PathBuf`/`OsString` as authoritative values. Lossy display text never becomes an operation path. Home resolves through the Windows user-profile known folder. A single positional startup directory is sufficient initially; do not imitate unverified original CLI switches.

Listings are immutable owned snapshots tagged with panel/tab identity and a monotonically increasing generation. Late results are discarded after navigation, tab closure, or refresh. Enumeration, sorting large snapshots, thumbnail decoding, and recursive statistics never run in the UI draw path. Queues and caches are bounded; queue saturation must not block UI input.

Selection is keyed by authoritative entry identity/path, not transient row indices. Shift range selection uses the visible ordered snapshot. Select All selects visible items. Filtering prunes hidden selections before mutation, so invisible entries are not silently acted on. Revalidate an operation's captured intent before execution; this does not claim to eliminate all filesystem races.

Initial filename filtering is locale-independent lowercase substring matching on display names, with no regex/glob grammar. Name ordering uses documented Windows logical-name comparison plus a lossless ordinal-name tie rule. These are proposed new policies, not recovered original algorithms. Directory-first ordering is a separate option. Errors, inaccessible folders, and vanished entries are visible rather than silently treated as empty.

Instrument row construction and verify visible-row work is bounded by the viewport plus documented toolkit overscan, independent of directory size. This is a complexity requirement, not a timing promise.

### Tabs, panes, and commands

Each tab owns location/history, selection, filter/sort/view state, and inspector settings. A split shares no mutable tab state. Closing a pane/tab cancels or detaches its non-mutating requests without deleting files. Closing a final tab closes its pane; the final remaining pane instead retains a Home tab. This final-pane safeguard is a new policy.

Central typed command IDs have explicit eligibility. Global file shortcuts do not steal text editing or IME composition. Use conventional Ctrl+C/X/V, Ctrl+T/W, Alt+Left/Right/Up, and F2 only where their implemented command is eligible. Unmodified `Y` is not a default file-copy binding in this version, despite the recovered constructor mapping. Character typing outside text editors opens the filename filter. Multi-step chords and editable mouse gestures are deferred.

Command-palette filtering initially uses substring matching and deterministic label/ID ties. Original fuzzy-menu constants are evidence, not required parity. The custom Shell-menu task will define its own documented matching policy before adding ranking.

### Windows ownership and transfers

The HWND-owning thread must initialize the compatible COM/OLE apartment at the correct time and retain registration/callback objects until teardown. Validate this against the pinned toolkit, including modal-loop reentrancy. Forward required Shell menu messages and preserve toolkit input/focus. FFI callbacks must not unwind into Windows; callback contexts stay alive until revoked. Do not hold a mutable widget borrow or blocking state lock across a reentrant Shell/modal call.

A dedicated message-pumped STA executor is the candidate file-operation owner. It creates and releases its COM interfaces in that apartment. Cross-thread messages contain owned paths/IDs/progress, never un-marshaled interfaces or widget references. A plain threadpool plus unchecked `OleInitialize` is not the design.

Operation requests capture immutable sources, destination, and explicit action. Outcome states distinguish queued, running, cancellation requested, completed, partial, cancelled, and failed. A successful HRESULT alone is not completion. Observe per-item results and aborted status; externally delegated Shell actions get a returned/delegated status and rescan rather than invented per-item results. No transactional rollback is promised.

Cancel is best effort at supported callback boundaries, not an immediate interruption guarantee. Keep the UI responsive during work. Closing with an active mutation asks to cancel/wait or keep the application open; do not terminate the worker or persist pending mutations for replay.

Trash never falls back silently to permanent deletion. Reject folder-into-descendant/self-target operations where detectable. Collisions use explicit Windows choices, never an unconditional overwrite default. Preserve unsupported paths and report API errors instead of truncating or stripping their roots.

Clipboard payloads use standard Unicode file-path formats with length/bounds validation and explicit global-memory ownership. Cut data is changed only after confirmed outcomes, only for completed items, and only if the clipboard still represents the captured request. Failed/cancelled items remain available. Do not clear cut data merely because work was queued.

MVP incoming filesystem drops use synchronous Shell target delegation only. The acquired target owns transfer execution; the application never queues a duplicate mutation. Return only the delegated status/effect and rescan without inventing per-item success. Keep the native host/callbacks alive through the modal call and defer destructive window teardown. A future application-executed drop needs a reviewed synchronous or negotiated asynchronous OLE completion protocol. Do not return a successful move merely after enqueueing. The initial outgoing mask permits copy/move; link is enabled only with reviewed shortcut support. Unsupported virtual-file formats are rejected visibly without allocating unbounded data or invoking unimplemented fallbacks.

### Previews and persistence

Preview requests use generations and owned results. Proposed initial limits are 1 MiB of text, 64 MiB of application-owned decoded image pixels, and a 128 MiB image/texture cache. Use checked size arithmetic, check all API results, cancel stale work, and release replaced textures. External codec allocations are not covered by these buffer limits. Limits are new policies, not File Pilot guarantees, and must be confirmed by measurement.

Use separate bounded preview work rather than blocking previews behind a long mutation. Verify the chosen WIC apartment/thread rules before implementation. Text is read-only; binary, oversized, or unsupported encoding data gets a clear fallback. Read UTF-8 initially; additional encodings require explicit support, not silent corruption.

Use an application-specific directory under Local AppData and never read/write Voidstar's files by default. Serialize a lossless Windows UTF-16 path representation and explicit location kinds, not live PIDLs, handles, callbacks, queued jobs, or original pointer-linked panels. Preserve raw code units, including unpaired surrogates representable by Windows OsString; validation must not force every path into a Unicode scalar string.

Version the schema. Validate ratios, active indices, unique IDs, tree depth, file size, and path encoding before applying a saved layout; clamp restored window geometry to available monitors. Unsupported future or corrupt state is preserved untouched; startup warns and opens safe defaults without overwriting it. Save through a same-directory temporary file and verified replacement. Retain the last good state on failure. Start with one writing instance; reject a competing writer for the same state directory until multi-instance semantics are designed.

Watcher events are invalidation hints. Coalesce refreshes and rescan after overflow/errors or reconnection. Network failures do not block input or claim an empty successful listing.

## Project structure and style

One crate, growing files only as needed:

```text
app/Cargo.toml, Cargo.lock, Taskfile.yml
app/src/main.rs                 executable entry
app/src/lib.rs                  application/module integration
app/src/model.rs                owned value/state types
app/src/ui.rs, app/src/ui/      views and input
app/src/directory.rs            listing workers
app/src/selection.rs            selection rules
app/src/commands.rs             registry and command eligibility
app/src/preview.rs              bounded preview coordination
app/src/session.rs              validation and saving
app/src/platform/mod.rs
app/src/platform/windows.rs    thread/handle lifecycle
app/src/platform/windows/      Shell, clipboard, operation, WIC, drop adapters
app/tests/                     integration tests added when required
```

Use rustfmt, snake_case functions/modules, PascalCase types, and typed errors at trust boundaries. Keep `unsafe` inside Windows adapters with a safety comment naming ownership and lifetime assumptions. Never add `unsafe impl Send/Sync` to make a COM object cross threads. No generic cross-platform service layer, event bus, or plugin abstractions without a concrete consumer.

Illustrative naming only, not generated application code:

```rust
struct ListingRequest {
    panel_id: PanelId,
    generation: u64,
    path: PathBuf,
}
```

## Commands and Definition of Done

Current analysis verification remains `task test` at the repository root. It checks evidence, not the future application.

The following commands become runnable in T01. Until then, they are planned commands, not completed build/test results:

```text
cargo build --manifest-path app/Cargo.toml
cargo test --manifest-path app/Cargo.toml
cargo fmt --manifest-path app/Cargo.toml -- --check
cargo clippy --manifest-path app/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path app/Cargo.toml
cargo build --manifest-path app/Cargo.toml --release
task -d app --list
task -d app run
task -d app test
task -d app clean
```

The application's Taskfile must have descriptions, a default task running `task --list`, and run/build/test/clean tasks. Root tasks remain analysis-only. Clean removes build output, never analysis, fixtures with unknown ownership, or user/session data.

Every completed task must satisfy its acceptance criteria, focused tests, build, formatting, and linting; update the canonical checklist with evidence. Add one small runnable regression check for non-trivial logic, not a testing framework for its own sake. Keep pure state/codec tests automatic. Mark native Shell/GUI mutation tests ignored and run them explicitly on Windows, one at a time, using newly created owned fixtures. Test commands in the checklist are planned filters; each task must add tests matching its named filter.

Example opt-in command after its tests exist:

```text
cargo test --manifest-path app/Cargo.toml native_operations -- --ignored --test-threads=1
```

Native manual verification cannot be replaced by mocked HRESULTs. Record hardware/OS, tested scenarios, observed errors, and unsupported cases at checkpoints. Measure 10,000- and 100,000-entry browsing after the basic slice; set timing/memory budgets from measured baselines before release rather than inventing File Pilot comparisons.

Always preserve original analysis, validate external data, test cancellation/stale work, and document actual new policies. Ask before new dependencies, changing the supported OS/stack, enabling broader destructive operations, changing storage contracts, or executing the original binary. Never ship recovered proprietary assets, mutate real-user fixtures in tests, empty the global Recycle Bin, silently permanently delete, or mark mock-only interoperability as verified.

## Task order and checkpoints

| Phase | Tasks in order | Exit checkpoint |
| --- | --- | --- |
| Window and browsing foundation | T01, T02, T03 | CP01 |
| Native menu and copy risk | T04, T05, T06 | CP02 |
| OLE feasibility and listing controls | T07, T08, T09 | CP03, candidate-stack approval |
| Multi-panel keyboard workflows | T10, T11, T12 | CP04 |
| Clipboard transfers and Trash | T13, T14, T15 | CP05 |
| Creation, rename, text inspector | T16, T17, T18 | CP06 |
| Image decoding/presentation and watcher | T19, T45, T20 | CP15 |
| Session codec, replacement, lifecycle | T21, T43, T44 | CP07 |
| Production drop and MVP release | T22, T23, T24 | CP08, MVP acceptance |
| Custom Shell and Recycle Bin | T25, T26, T27 | CP09 |
| Expanded/statistical/icon views | T28, T29, T30 | CP10 |
| Batch rename and saved layouts | T31, T32, T33 | CP11 |
| Quick Access and presentation | T34, T35, T36 | CP12 |
| Input and platform convenience | T37, T38, T39 | CP13 |
| Advanced mutation/customization | T40, T41, T42 | CP14, extended acceptance |

The checklist contains dependencies; a task is not ready merely because it appears next. T43-T45 and CP15 were added after review without renumbering existing IDs; document order is the execution order. If a slice grows beyond five files or roughly one focused session, split it before implementation, preserving existing IDs and dependencies. Scope labels are not calendar estimates. Feature additions must count Cargo.toml/Cargo.lock and module wiring inside the five-file budget; if new Windows binding features exceed it, split an explicit prerequisite task before coding.

## Parallel work rules

Per the 2026-10-13 user instruction, limit work to six concurrent agents total, including the main agent. Workers must not spawn extra agents outside this cap. Parallelism is an execution option, not a request to start implementation now.

- HWND/COM ownership, module/dependency wiring, model/schema changes, and command-registry integration have one owner and land sequentially.
- After CP03 and stable listing/intent contracts, layout work T10/T11 and palette T12 can be split, but their shared UI integration must be serialized or assigned to one owner.
- After CP04, adapter-only clipboard work and bounded text preview work can proceed independently of layout presentation when their command/result contracts are fixed.
- T19 image decoding and T20 watcher work can run independently after shared generation/error contracts. T45 presents accepted image results; the main owner merges UI/state integration.
- T21 session codec precedes T43 replacement safety, which precedes T44 lifecycle/single-writer integration. These are a dependency chain, not three parallel writers.
- Later recursive listing/statistics, Quick Access data work, and mutation-free preview/generator tests can be independent. T31 must finish before T32.
- UI files, `Cargo.toml`, `Cargo.lock`, module wiring, schema, and the task checklist are shared write hotspots. Use exclusive file ownership or separate git worktrees after a repository is established, then verify merged builds. Do not assume worktree isolation is currently available: this directory is not a git checkout.

## Risks and mitigation

| Risk | Impact | Mitigation/gate |
| --- | --- | --- |
| Toolkit COM initialization or HWND callback integration incompatible | High | T02/T05/T07/T08 and CP03; stop and revisit D03 rather than adding unsafe workarounds |
| Shell extension blocks/crashes in-process | High | Native popup first, preserve fallback, record tested extensions; process isolation is separate future scope |
| Duplicate or premature drop mutation | High | One transfer authority; read-only probe first; never advertise queued move as complete |
| Clipboard replacement or partial move loses cut state | High | Capture identity; confirmed per-item outcomes; conditional clipboard update |
| Filesystem races, junctions, collisions, access denial | High | Revalidation and explicit results; no silent overwrite/delete fallback; targeted fixtures |
| UI blocks on large/network directories | High | Bounded workers, generations, virtualization, rescan recovery and measured baselines |
| Corrupt session or interrupted save destroys user state | High | Validated own schema, preserve unreadable/future data, replacement failure tests |
| Pixel/algorithm parity expands scope | Medium | Independent UI and documented policies; exact parity excluded |
| Accessibility only checked at the end | High | Keyboard/Narrator checks at CP01/CP03 and per-view tasks, full T23 release gate |
| Advanced task becomes several sessions | Medium | Stop and split before coding; never silently mark partial coverage complete |

## Approval questions

- Q1: Approve the candidate egui/eframe stack and the ordinary-filesystem MVP? Recommended yes, conditional on CP03, because it minimizes custom UI work while testing Shell integration early.
- Q2: Is Windows 11 x64 sufficient for the first verified release? Recommended yes; add Windows 10 only after a defined test environment and toolkit compatibility check.
- Q3: Approve a new application name/state directory and independent session format? Recommended yes; working label is Rust File Manager, not a shipping brand. Original File Pilot files remain untouched.
- Q4: Is capability similarity sufficient, rather than exact pixels, original JSON, all original options, or identical key/search rules? Recommended yes for MVP; exact compatibility needs separate evidence and acceptance scenarios.

The user authorized implementation on 2026-10-13. Q1-Q4 use the proposed defaults. Implementation proceeds subject to dependency and native verification gates; no further routine approval is needed.

