# Mado Explorer

S005: a Windows-first, read-only directory browser using Rust 1.95+, eframe/egui
and the wgpu renderer. This is a runnable slice, not the full release in SPEC.md.
A working graphics adapter/driver is required.

```sh
task run                         # browse the working directory
task run -- "C:\path\to\folder"   # or: cargo run -- "C:\path\to\folder"
task test                        # cargo test; disposable generated fixtures
task clean                       # removes target only
task design                      # opens the Excalidraw reference
task design-test                 # validates the reference
```

Enter a folder path and press Enter/Go, double-click a folder row (or make it the
active row and press Enter), or use Back, Forward, Up and the breadcrumb buttons.
Double-click a regular file or make it the active row and press Enter to request
its Windows default application. Only that row opens, even with multiple selections;
selection, filtering, refresh and browsing never automatically launch a file.
Alt+Left/Right/Up navigate; Refresh/F5 reloads. These shortcuts are suspended while
the path or filter editor has focus (including its submitting frame). Relative typed
paths resolve against the **committed** location, including when a new destination
is still loading. Breadcrumbs identify
that committed location; an explicit Opening message identifies the pending
listing shown progressively below it. A new location/history commits only when
its enumeration succeeds. A failed destination restores the previous listing and
retains history, with a visible error and the path input available for retry.
Back/forward failures do not consume history; a successful new branch clears
forward history. Refresh/startup failures show the current location's error state,
including missing folders, with Up and Refresh still available.

Click Name, Type, Modified or Size to sort; click the active heading again to
reverse direction. Initially names are ascending. Folders stay first in both
directions. Unknown sizes/times follow known values within each group in both
directions. Equal values use ascending native name, then full native path, as the
deterministic tie-breaker. Sorting uses native filesystem name ordering, not
locale-aware or natural-number ordering. Sorting preserves selected paths, the
active row and range anchor; subsequent ranges use the currently displayed order.

Type in Filter to constrain immediate entry **names only**, using case-insensitive
fuzzy subsequences: `rdtxt` matches `ReadMe.txt`. This is not recursive search,
path/content search or fuzzy ranking; results retain the chosen column order.
Clear (or delete the query) restores entries without changing location/history.
The one current browsing context retains its filter through refresh and ordinary
navigation, including Back/Forward; each destination is shown with that same query.
Per-tab ownership will be added with tabs, not simulated now. Counts show displayed /
total entries for a fresh projection, with an incomplete suffix during loading/errors.
A completed fresh listing with zero matches explicitly says so; an empty folder
remains distinct. While filter/sort changes are pending, old rows may stay visible,
but row clicks, Ctrl+A, movement and activation are paused. With a nonempty filter,
row commands also pause until enumeration succeeds and its complete projection is
accepted. A failed/incomplete filtered read retains selection identities but offers
retry/navigation/filter editing rather than commands on uncertain rows.

Matching uses stdlib Unicode lowercase, not locale-aware full case folding or
Unicode normalization; query whitespace is literal. Non-Unicode names match their
lossy displayed name, while selection/activation continue to use native paths.
No fuzzy dependency, index, debounce machinery or result-count cap is introduced.

Click anywhere in a row to select it. Ctrl+click toggles a path; Shift+click replaces
selection with the anchor-to-click range (Ctrl+Shift adds that range). Ctrl+A selects
only the currently displayed entries. The selected count is visible above the table.
The active row has a border, independent of whether it is selected; the Entries
heading shows keyboard focus. Up/Down, Home/End and Page Up/Down move the active row
and scroll it into view; Shift extends/reduces the anchor range. With no active row,
Down/Page Down/Home start at the first row and Up/Page Up/End at the last row. Plain
movement replaces selection; a missing anchor falls back to the active row or target.
Path/filter-editor focus (including the frame that submits it) suspends browsing keys;
Ctrl+A selects editor text, not rows. Escape exits filter editing without clearing it;
Enter on another focused control is not also row activation. Holding Enter does not
repeat file launches.

Refresh retains selection/active/anchor during empty or progressive snapshots and
failed reads. Only a successfully completed listing removes vanished paths; surviving
paths remain selected. Fresh completed filter projections deselect hidden native
paths and remove hidden active/anchor identities. Still-visible selections survive filter/sort/refresh changes;
clearing never resurrects hidden prior selections. Ctrl+A selects visible entries only.
Progressive snapshots do not prove that a selected path disappeared. Future file
commands must share `Navigation::rows_selectable` and capture only fresh visible
selection. File opening shares this boundary; no mutating-command abstraction is
introduced here.
Pending destination rows cannot be selected or activated until navigation commits
and its projection is fresh. Failed destinations preserve the committed selection;
successful navigation to another location clears it. Selection is scoped to the one current
browsing context, not yet isolated across tabs/panes.

Identity lookup for keyboard movement and range endpoints is intentionally O(n) in
the displayed entry count; Ctrl+A/range updates and successful-refresh reconciliation
are also linear. No entry-count cap is enforced. The proposed 20,000-entry target has
not been performance-verified. If measured input latency exceeds its budget, build a
native path-to-index projection on the sorting worker and publish it with the existing
generation/revision key, rather than retaining row indices across sorting.

No previews, settings, tabs/splits or mutation commands are implemented.
The explorer itself remains read-only, but **externally launched applications may
edit their files or execute code**; open only files you intend to open.
Hidden entries are included. Paths used for folder/file activation, row identity,
breadcrumbs and history stay native/lossless;
display/path-editor text is lossy for non-Unicode names. Submitting the unchanged
displayed current path retains its native path instead of reparsing lossy text.

The table shows immediate entries only, modification timestamps in UTC, and
regular-file sizes in bytes. `—` means unknown/not applicable, never zero folder
size. Metadata errors remain attached to their rows and can be read by hovering
names. Links/reparse points are labelled and not followed for metadata or folder
activation.

File opening captures the complete native `PathBuf` when invoked, never a display
string or a later selection. A bounded launch worker rechecks that it is still a
regular file (not a folder/reparse point), then uses `ShellExecuteExW` with the
`open` verb, UTF-16 paths, no command shell, no parameters and no explicit elevation.
Absolute filesystem paths are required; embedded NULs, relative paths, URIs and
non-filesystem device namespaces are rejected. No associations/registry are changed.
Windows security prompts are not suppressed; the explorer never requests `runas`.

Only one launch can be in flight, with no hidden queue or automatic retry. The
status area identifies the captured busy path; another file-open request is visibly
refused and must be explicitly retried later. Folder navigation and selection remain
available while the shell works. Failures show the captured path and OS error without
changing browsing location/history/selection, even if the user navigated elsewhere.
Success means Windows accepted the request, **not** that the external application
rendered the file or finished. External application lifetime is not monitored.
Ordinary shell error dialogs are suppressed in favor of the explorer's error status.

The worker initializes/releases its own COM STA and completes shell/DDE work before
returning; no process handle is requested. Its thread handle is retained and joined
only after completion. Shell extensions/OS calls are not interruptible: a stuck
launch remains visibly busy and prevents further file launches, not browsing. Closing
the explorer does not wait on that call; an unfinished worker is detached and its
resources are released on return or process teardown. There is no queued launch to
run after closing, and external applications are not terminated.

Enumeration and metadata reads run on two background workers, with one coalesced
pending location and a bounded eight-message snapshot/status queue. Enumeration
still uses batches of at most 256 entries; after the first batch, cumulative
snapshots are published at most every 50 ms plus the final snapshot. Snapshots
share entry allocations, but each snapshot holds its own vector of entry handles.
A separate projection/sorting worker has one active projection, one coalesced pending
request and one result slot. Query normalization, name matching, entry-vector building,
sorting and visible-path-set building happen on that worker, not in UI update.
Projection results must match the visible listing's generation, data/completion
revision, query revision and sort choice. Returning quickly to the same query text
still creates a newer query revision. Completion increments data revision so a
progressive projection cannot masquerade as authoritative final data. While updating,
the previous projection can stay visible with an Updating filter/sort indicator.
Rows remain virtualized. Loaded listings and their snapshot vectors are not size-capped; no memory/performance acceptance claim has
been made.

Stale enumeration is cancelled between OS calls and rejected by navigation.
OS calls themselves cannot be interrupted. If both enumeration workers are stuck
on unavailable locations, a new listing waits, but input remains responsive.
The upgrade path is cancellable platform I/O if measured workloads require it.
A running projection is not interrupted, but obsolete results are discarded and only
the latest pending request is processed next. If measured filter latency exceeds its
budget, add cancellation checkpoints and worker-side normalized-name caching; measure
first rather than changing the matching model or ranking.

## Pending Windows smoke procedure

Use only disposable folders containing child folders and small files:

1. Launch with `task run -- "<fixture>"`; check names/types, UTC times, byte counts
   and unknown folder sizes. Click every heading twice; verify folders first,
   ascending/descending values, unknowns last and deterministic equal-value order.
   Resize and scroll a larger fixture.
2. Open A then B; use Back/Forward, Alt+Left/Right, Up/Alt+Up, breadcrumbs and full
   path input. Back to A and branch to C; Forward should be disabled. Check that
   the path input is updated to the committed full path after navigation.
3. From a loaded fixture, enter a missing path. Confirm its error, the restored
   previous listing/history, and successful retry via edited path/Go. With the
   path editor focused, Alt+Left/Right/Up and F5 must not navigate or refresh.
4. Remove an open disposable child externally, then Refresh/F5. Confirm an
   explicit missing state and use Up to recover. Also launch at a missing child.
5. Use a separately prepared access-denied fixture and an unavailable drive;
   check distinct errors and usable navigation. Classification is unit-tested,
   but real ACL-denial and unavailable-drive smoke checks are pending.
6. Select a row by clicking its name, type, modified time, size and blank row space.
   Check Ctrl toggling, Shift ranges, Ctrl+A and selected count, then sort each column:
   selected/active paths must stay the same; new ranges use the displayed order.
   Use keyboard-only arrows, Home/End, Page Up/Down and Shift at the first/last rows
   and in an empty folder. The bordered active row must stay visible while scrolling.
   With the path editor focused, Ctrl+A must select text; arrows/Home/End/Page keys
   and Enter must not also select rows or activate a file/folder. Enter on another
   focused control must not launch the active row.
7. Select multiple disposable entries; externally remove one, then refresh. Surviving
   selections must remain through loading and the vanished one must disappear only
   after successful completion. Test a failed destination with selection, followed
   by a successful folder navigation: failure retains selection, success clears it.
   Pending destination rows must not accept selection or activation.
8. Navigate A to B while A is slow; change sort while loading and navigate again.
   Verify late entries/sorts never populate the wrong listing and history changes
   only on successful latest navigation. Automated state/worker regressions do
   not substitute for a measured slow-drive interaction check.
9. Select several fixture entries, then type a mixed-case subsequence filter. Check
   chosen sort order, displayed/total counts, selected count and visible-only Ctrl+A.
   Filtered-out selections/active/anchor must disappear only when the fresh completed
   result arrives; clearing restores rows, not hidden prior selections. Try a no-match
   query and an empty folder. Rapidly edit/clear, refresh and navigate while filtering:
   old-query rows must not accept selection/activation, and incomplete filtered reads
   must explain the pause. Refresh and Back/Forward retain the query. In Filter,
   Ctrl+A/arrows/Enter/Alt navigation/F5 must not affect browsing; Escape exits editing.
   Resize to the narrow viewport with all controls still reachable.
10. Prepare only disposable small text documents (including Unicode, spaces, `&`
    and parentheses) with a known safe association. Double-click a regular file,
    then use active-row Enter; confirm the expected external application receives
    the intended file. Select multiple rows and verify only the active/clicked file
    opens. Do not hold Enter to trigger repeated launches. Folder activation still
    navigates internally. Test an unassociated extension and a file removed after
    listing: failure must identify the captured file, leave browsing usable and not
    modify it. While a launch is busy, another request must be visibly refused,
    never silently queued. Navigate/select elsewhere before a delayed result arrives;
    it must not navigate back or change that selection. Recheck path/filter editor
    focus and submission, pending navigation and stale filter/sort rows: no launch.
    These real-association GUI checks are pending, not covered by callback tests.

GUI appearance/interaction, display scaling and Windows Narrator checks still
require manual verification. S001's process-level startup/close smoke created the
native window and exited with code 0; it did not verify displayed states. S002
has automated sorting, navigation, background pipeline and headless shortcut
checks. S003 adds selection-state, reconciliation, whole-row hit testing, editor
ownership and virtualized keyboard-scroll regressions. S004 adds matching/projection
staleness, authoritative visible-selection reconciliation, command pause/recovery,
fresh count/no-match display and real headless filter-editor ownership checks.
S005 adds native UTF-16/argument validation, fresh file/folder activation routing,
headless double-click/Enter guards, bounded injected launch/failure capture, and
late-result browsing preservation. Automated launch tests never execute documents
or programs and never alter file associations; missing-file preflight returns before
calling the shell. These are not a foreground-capture or interactive GUI acceptance claim.
No destructive operations or personal-folder checks are needed for this slice.
