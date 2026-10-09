# P06 selection-summary handoff

## Candidate and boundary

- Checkout `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`, branch `lane/selection-summary`, accepted baseline `36798781b4f9bf8701c54197f90e80d39c1d9cb6`.
- Candidate is the component-only lane commit containing this document. Exact SHA accompanies the worker report and is available with `git rev-parse HEAD` in this checkout.
- Writes confined to `components/selection-summary/**`. No root application adapter, dependency, acceptance-tracker or sibling changes.
- Supervisor-owned `PROGRESS.md` commission was already dirty on entry, exactly 10 added lines. It remains unchanged and unstaged, separately from the committed component.
- Ready for fresh read-only review, not production enablement. F07.5 is partial preparation only. No complete criterion or native runtime verification claim.

## Public API and input DTO

Library `mado-selection-summary`, import `mado_selection_summary`. Its only direct dependency is the installed `egui = "=0.36.2"`, matching the accepted root lockfile. Standard library otherwise. Component Cargo.lock is tracked; it resolves independently of the application.

Exact input fields:

```rust
use std::{ffi::OsStr, path::Path};

pub struct RequestTag {
    pub context: u64,
    pub revision: u64,
}

pub enum EntryKind { Folder, RegularFile, Link, Other, Unknown }

pub struct SelectedEntry<'a> {
    pub name: &'a OsStr,
    pub path: &'a Path,
    pub kind: EntryKind,
    pub size_bytes: Option<u64>,
    pub modified_utc: Option<&'a str>,
    pub metadata_error: Option<&'a str>,
}

pub enum SnapshotState<'a> {
    Loading,
    MetadataError(&'a str),
    Ready(&'a [SelectedEntry<'a>]),
}

pub struct SelectionSnapshot<'a> {
    pub tag: RequestTag,
    pub state: SnapshotState<'a>,
}
```

These DTO types derive Clone, Copy and Debug. RequestTag and EntryKind also derive PartialEq and Eq.

- `SelectionSummary::new(pane_id: u64) -> SelectionSummary` uses a stable unique pane ID within the egui context. Keep one instance per pane.
- `show(&mut self, ui: &mut egui::Ui, active: RequestTag, snapshot: Option<&SelectionSnapshot<'_>>) -> Option<egui::Response>` renders inside the caller's preview region. The response is the single-entry details toggle, for optional focus routing. It carries no command or file-operation intent.
- Tags compare by exact equality before any snapshot state or metadata is rendered. No ordering heuristic, paths, metadata or last-good snapshot are retained.
- Caller changes context on active-tab/activation changes and never reuses a revision within that context. Every selection/listing/projection/loading/error change needs a fresh revision, including returning to previous values. Do not relabel an old snapshot with a new tag.
- Ready contains exactly distinct visible selected entries. Metadata errors may be partial, such as unavailable modification time with known regular-file size. Known bytes remain known in that case.
- Native name/path references retain identity without conversion. Lossy display text is read-only and never becomes a command path. No filesystem reads, decoders, threads, operations or shared cache.

## Implemented component UI

- None means waiting for metadata. Tagged Loading, stale tags and snapshot-level metadata error have distinct text. Ready with no entries shows no selection.
- Multiple selections show visible count, exact known regular-file bytes, known-size regular-file count, explicitly unknown-size entry count, and metadata-error count when nonzero. It never chooses an arbitrary file or shows a content preview.
- A single folder shows name/type and explicitly unknown size, even if a caller supplied a folder size. No recursive scan/count. The usable Basic details toggle exposes path and supplied UTC modification time, or Unknown.
- A single unsupported regular file shows name/type, known or unknown size, content-preview fallback, partial metadata errors and the same details toggle. Link/reparse/other/unknown types get basic fallback without following targets.
- Actual egui button handles click, Tab focus, Space and Enter. Selected state uses egui toggle semantics. A two-point focus border is explicit, not color-only. Details and scroll IDs are pane-local. Text wraps inside a vertical scroll area.
- Component retains one boolean and one pane ID. Multi-selection sum is O(selected entries), with u128 byte accumulation and aggregate-only rendering. It does not copy/store the selection. If measurements require it, move aggregation to the listing worker during integration.

## Focused evidence

Windows host, Cargo 1.95.0, Task 3.50.0. Exact final commands, all successful unless noted:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && cargo fmt --check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && cargo check --locked --offline --target-dir target --all-targets
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && cargo test --locked --offline --target-dir target --test summary_ui
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && task run
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && task check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && task test
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && task
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary/components/selection-summary && task --dry clean
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && git diff --check
cd C:/SANDBOX/WORKTREES/mado-explorer/selection-summary && git diff --cached --check
```

- Formatting and locked incremental all-target compile pass, no warnings.
- Focused suite passes 7 tests, none ignored. Tests inspect actual egui text shapes and drive button pointer/key events. Coverage includes empty/multiple/single transitions, folder unknown size, unavailable/partial metadata, stale selection/context/error refusal, independent same-tag pane instances, Tab/Space/Enter focus and border, exact sums above u64, and distinct Windows malformed-UTF-16 identities with colliding display text.
- `task run` is a safe generated-data headless egui demonstration. It prints seven rendered states, opens no window and accesses no source files.
- `task`, `task --list`, `task check` and `task test` pass. Clean was dry-run only, not executed. Its command is `cargo clean --target-dir target`, confined to this component's output. All tasks use the Taskfile directory and have descriptions.
- Initial bootstrap `cargo generate-lockfile --offline` failed because accesskit is mandatory, not an egui feature in 0.36.2. Removed the invalid feature request; offline lock generation then succeeded.
- Initial focused tests aborted with Windows exit `0xc0000409`. A diagnostic single-thread run exposed egui's debug panic, `Dropped TexturesDelta with 1 unapplied deltas`. Headless test/demo outputs now explicitly clear font-upload deltas before dropping them, as required by installed epaint. No assertions or production behavior were weakened. Standard parallel focused tests then passed.
- No root suite, Clippy, foreground, clipboard, elevation, personal-data or native renderer probes.

## Smallest serial integration follow-up

After independent review and supervisor checkpoint gates, add the path dependency and a summary instance to each actual pane. The adapter remains supervisor-owned.

1. Capture only after that pane's active tab, listing and selection processing for the frame. Allocate a fresh per-pane revision before each capture/invalidation. `RequestTag.context` can use the active listing/activation generation, which root's loader already allocates window-wide. Keep active tag separate from any captured snapshot tag. Synchronous borrowed captures need no job or cache.
2. Root `Navigation::rows_selectable()` permits some progressive unfiltered rows. For this summary, additionally require `Status::Ready`, no pending destination and a fresh projection. If navigating/loading/projecting, send Loading rather than retained selection metadata. If the listing fails, send MetadataError with supplied action/path/error text.
3. Build Ready from `navigation.current.entries`, filtered by native membership in `navigation.selection.paths`. Never iterate retained hidden selections or use `raw_entries` as visible selection. The fresh displayed vector supplies distinct native paths.
4. Map root Folder/File/Link/Other/Unknown to this Folder/RegularFile/Link/Other/Unknown. Use `entry.path.as_path()` for path and `entry.path.file_name().unwrap_or(entry.path.as_os_str())` for native name, never `entry.name` or a parsed display path. Copy `entry.size`; use `entry.modified_time.map(|_| entry.modified.as_str())` for the already-formatted UTC value; use `entry.metadata_error.as_deref()`. No metadata call is needed or allowed in this adapter.
5. Call this pane's summary for zero/multiple selections, one folder, or unsupported single-file fallback. Supported image/PDF dispatch stays outside this component. Keep the metadata backing the borrowed DTO alive through show. Drop snapshot references after rendering.
6. Add one root-focused regression for filtering away a selected entry, switching active tabs during pending listings, and pane routing. Then perform the authorized native preview/focus/scaling checks. No checkpoint gate is waived by component tests.

## Limits and pending checks

Caller must guarantee fresh tags and visible, distinct selections. The component cannot detect a falsely tagged snapshot or hidden entries supplied as visible. Display can lose malformed-native-name distinctions, but identity cannot. No recursive counts, decoded previews, textures/cache budget, preview divider, main integration or F07.1-F07.4/F07.6 claim. Large-selection latency, long-path/narrow-pane native presentation, Narrator, DPI scaling and runtime integration remain unverified.
