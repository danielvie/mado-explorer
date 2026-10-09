# Mado Explorer

S011: a Windows-first directory browser using Rust 1.95+, eframe/egui
and the wgpu renderer. This is a runnable slice, not the full release in SPEC.md.
A working graphics adapter/driver is required.

```sh
task run                         # restore workspace, or browse working directory
task run -- "C:\path\to\folder"   # or: cargo run -- "C:\path\to\folder"
task test                        # portable default checks; disposable fixtures
task test:fonts                  # explicit Windows reference-host font coverage
task clean                       # removes target only
task design                      # opens the Excalidraw reference
task design-test                 # validates the reference
```

Each pane has independent tabs. Use **+** or Ctrl+T to create a fresh
tab at the active tab's **committed** location (not a still-opening destination).
Use a tab's **x** button to close it, Ctrl+W to close the active tab, and
Ctrl+Tab / Ctrl+Shift+Tab to cycle forward/backward in the focused pane. Closing
the last tab of a non-final pane closes that pane and expands its sibling. Closing
the final window tab creates
a fresh tab at the user's home location; it does not close the window. Home is
resolved once at startup, and missing/unreadable home contents use normal browsing
error/retry handling. Reviewed workspace settings save and restore tabs,
panes, bookmarks and recents across restarts, as described below.

Every tab owns location/history, native-path selection/active row/range anchor,
filter, sort, hidden/system visibility, path-editor draft/errors and scroll position.
Switching back refreshes the committed location and reconciles surviving selection.
Its saved manual scroll position is restored only after a complete fresh projection;
empty/progressive refresh data and the selected row do not replace that position.
Keyboard movement still scrolls its new active row into view. A removed entry can
therefore disappear from selection when that tab's refresh succeeds.
Pending navigation is cancelled on deactivation without committing history; returning
refreshes the last committed location rather than leaving a cancelled spinner.

Tab IDs are stable, never reused within the window, and scope editor/table/scroll
widget state even for two tabs at the same folder. The horizontal tab strip scrolls
with overflow; new/close/cycle activation reveals the active tab, and the plus button
stays outside overflow. Hover a tab to see its complete native location. Tab
shortcuts use exact modifiers and are suspended while a path/filter editor owns the
input, including its submission/exit frame.

Drag a tab's label (not its **x**) to reorder it in its strip, move it into another
pane's strip, or split at a pane's left/right/top/bottom edge. A cyan insertion line
or edge band and a tooltip identify the destination; orange feedback explains a
refused split. Strips take priority where they overlap an edge. The original tab
moves, keeping its ID, committed location/history, native selection/active/anchor,
filter/sort/visibility, drafts/errors and manual row scroll. Reordering preserves
the active tab's identity; a cross-pane move activates and focuses the moved tab
in the destination. Its active label is revealed. Plus, close and ordinary tab
clicks remain separate interactions.

Moving the last tab into a different pane collapses its empty source. Splitting
at its own pane's edge moves the original into the new pane and, if necessary,
replenishes the source with a fresh tab at the original committed location. This
keeps the requested split usable, even from the window's lone tab. Edge splits
require the entire target pane to be visible and room for two 320 x 300 panes plus
a six-point divider; otherwise no tab/layout state changes. Escape cancels a drag.
An outside release, stale/incomplete frame, or closed source/destination/insertion
tab cannot silently move a tab to a different context. Refusals/cancelled releases
have a visible reason. File payloads are not tab payloads; no file drag/drop or
filesystem copy/move is enabled.

Use **Split left/right** or **Split top/bottom** for the focused pane. The new pane
starts with a fresh tab at the source tab's committed location, while the source
continues loading/browsing independently. Click anywhere in a pane or press **F6**
to focus it; cyan, a thicker border and an explicit Focused pane label identify it.
Pointer focus is resolved before keyboard dispatch. Navigation, tab shortcuts,
selection and activation target only the focused pane. Focus changes do not refresh,
cancel or overwrite another pane's state. **Close pane** removes all its tabs and
cancels their read-only requests; a captured file launch remains window-owned.
Closing the final pane replaces its tabs with one fresh home tab.

Drag a divider to resize both neighbors. Recursive subtree minimums clamp dividers;
each pane is at least 320 x 300 logical points. Splits that cannot fit in a fully
visible source pane are refused with feedback. If the window shrinks below a nested
layout's minimum, the workspace scrolls rather than overlapping controls; F6 reveals
the focused pane. Short panes separately scroll their controls/status to reach the
table, which retains its tab-owned row scroll. Narrow tables drop Modified, then
Type, then Size before sacrificing a 200-point Name column. When Type is absent,
plain [D]/[F]/[L]/[O]/[?] markers distinguish folders/files/links/other/unknown entries;
full metadata/path remain available on hover. No preview or preview divider exists
yet, and pane dividers do not share a control with any future preview divider.

Enter a folder path and press Enter/Go, double-click a folder row (or make it the
active row and press Enter), or use Back, Forward, Up and the breadcrumb buttons.
Double-click a regular file or make it the active row and press Enter to request
its Windows default application. Only that row opens, even with multiple selections;
selection, filtering, refresh and browsing never automatically launch a file.
Alt+Left/Right/Up navigate; Refresh/F5 reloads. These shortcuts are suspended while
the path or filter editor has focus (including its submitting frame). Relative typed
paths resolve against the **committed** location, including when a new destination
is still loading. Ambiguous drive-relative input (`C:folder` or `C:`) is rejected
with recovery feedback; use `C:\folder` or an ordinary relative path instead.
Absolute drive/UNC and native verbatim long-path prefixes are retained. Device
namespaces are not supported as typed browsing destinations. Enumeration workers
normalize ordinary Windows destinations with stdlib native full-path behavior before
opening them: `..` from `C:\fixture\a` commits `C:\fixture`, so the next Up goes to
`C:\`. Absolute dot-component inputs follow the same rule. Row paths come from that
same normalized destination; only a successful current-generation completion commits
it. Successful same-location aliases (`.` or `child\..`) refresh surviving selection
without adding history or clearing Forward. Failed/stale normalization of a new
destination retains the previous location/listing/history/selection, just like
failed destination loading. Explicit Refresh retains its existing loading/error
semantics.

This is not filesystem canonicalization or link-target resolution: ordinary paths
stay ordinary, while dot-free native UNC/verbatim and supported long paths keep their
prefixes and native name units. Explicit verbatim `.`/`..` components are rejected
on the worker with recovery feedback rather than guessed/stripped; this host's Windows
APIs report invalid-name errors for such inputs. Use Up/breadcrumbs or enter a
fully specified dot-free path. No UI filesystem I/O or process cwd changes are used.
Breadcrumbs identify
that committed location; an explicit Opening message identifies the pending
listing shown progressively below it. A new location/history commits only when
its enumeration succeeds. A failed destination restores the previous listing and
retains history, with a visible error and the path input available for retry.
Back/forward failures do not consume history; a successful new branch clears
forward history. Refresh/startup failures show the current location's error state,
including missing folders, with Up and Refresh still available.

Click Name, Type, Modified or Size to sort; click the active heading again to
reverse direction. ASCII `^` means ascending and `v` means descending, without
requiring optional symbol glyphs. Initially names are ascending. Folders stay first in both
directions. Unknown sizes/times follow known values within each group in both
directions. Equal values use ascending native name, then full native path, as the
deterministic tie-breaker. Sorting uses native filesystem name ordering, not
locale-aware or natural-number ordering. Sorting preserves selected paths, the
active row and range anchor; subsequent ranges use the currently displayed order.

Type in Filter to constrain immediate entry **names only**, using case-insensitive
fuzzy subsequences: `rdtxt` matches `ReadMe.txt`. This is not recursive search,
path/content search or fuzzy ranking; results retain the chosen column order.
Clear (or delete the query) restores entries without changing location/history.
Each tab retains its own filter through refresh and ordinary
navigation, including Back/Forward; each destination is shown with that tab's query. Counts show displayed /
total entries for a fresh projection, with an incomplete suffix during loading/errors.
A completed fresh listing with zero matches explicitly says so; an empty folder
remains distinct. While filter/sort changes are pending, old rows may stay visible,
but row clicks, Ctrl+A, movement and activation are paused. With a nonempty filter,
row commands also pause until enumeration succeeds and its complete projection is
accepted. A failed/incomplete filtered read retains selection identities but offers
retry/navigation/filter editing rather than commands on uncertain rows.

Matching uses stdlib Unicode lowercase, not locale-aware full case folding or
Unicode normalization; query whitespace is literal. Non-Unicode names match their
escaped diagnostic display, while selection/activation continue to use native paths.
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
paths remain selected. Fresh completed filter/visibility projections deselect hidden
native paths and remove hidden active/anchor identities. Still-visible selections
survive filter/sort/refresh changes;
clearing never resurrects hidden prior selections. Ctrl+A selects visible entries only.
Progressive snapshots do not prove that a selected path disappeared. Future file
commands must share `Navigation::rows_selectable` and capture only fresh visible
selection. File opening shares this boundary; no mutating-command abstraction is
introduced here.
Pending destination rows cannot be selected or activated until navigation commits
and its projection is fresh. Failed destinations preserve the committed selection;
successful navigation to another location clears it. Selection is scoped to its tab;
other tabs and panes do not overwrite it.

Identity lookup for keyboard movement and range endpoints is intentionally O(n) in
the displayed entry count; Ctrl+A/range updates and successful-refresh reconciliation
are also linear. No entry-count cap is enforced. The proposed 20,000-entry target has
not been performance-verified. If measured input latency exceeds its budget, build a
native path-to-index projection on the sorting worker and publish it with the existing
generation/revision key, rather than retaining row indices across sorting.

No preview or file-mutation commands are enabled. Workspace settings save and
restore are implemented. Browsing never modifies source files, but **externally launched applications may
edit their files or execute code**; open only files you intend to open.
Hidden and system entries are **hidden by default**. The explicit **Show
hidden/system entries** checkbox shows both attribute categories together, including
folders, in the immediate listing. It is retained through navigation/refresh in the
owning tab and is not persisted across restarts. Attributes are read
with non-following metadata on enumeration workers; visibility changes reuse the
projection worker without re-enumeration. Counts include hidden entries in the total;
an all-hidden folder is distinct from an empty folder. Attribute/metadata failures
remain visible as unknown rows with an error rather than silently disappearing.
A visibility change pauses row commands only until a fresh projection arrives;
empty-query progressive browsing is still interactive. Incomplete/failed projections
do not deselect absent paths; only successful complete projections are authoritative.
Show never resurrects selections previously hidden by a complete projection.

Paths used for folder/file activation, row identity, breadcrumbs and history stay
native/lossless. Valid Unicode display text is unchanged, including a literal `�`.
Malformed UTF-16 uses a marked `[non-Unicode: ...]` diagnostic with escaped units
(such as `\u{D800}`), not replacement-character text. This is diagnostic display,
not an editable path serialization: submitting the unchanged current diagnostic
retains its native path; edited diagnostic input is rejected with recovery guidance.
Use rows/breadcrumbs/Up or enter a Unicode folder path instead. No display text is
used as a row action source; long/UNC prefixes remain native.

Unicode rendering keeps egui's builtin Latin fonts first and appends a small fixed
installed-Windows fallback chain for both proportional and monospace text. One
startup load **before the native UI runs** gets the Fonts location using
`SHGetKnownFolderPath(FOLDERID_Fonts)`; no hard-coded drive, directory scan, registry
edits, downloads or proprietary font redistribution. It tries `msgothic.ttc` (or
`YuGothR.ttc` if unavailable), `malgun.ttf`, `msyh.ttc`, `seguisym.ttf` and
`seguihis.ttf`, using face zero for TTF/TTC. At most six fixed files are attempted,
five retained, each at most 24 MiB, with at most 64 MiB of retained optional font
payload. Reads are capped even if files grow. Parsing is validated on startup;
epaint's internal font copies/caches and glyph atlas are additional memory, not
included in that payload limit. No font-file I/O runs in UI updates.

Missing/unreadable, oversized or unparseable optional fonts leave the builtin fonts
and available fallbacks usable; a short visible warning offers hover details.
Absence of one Japanese candidate is not a limitation if its alternative loads.
Installed coverage varies by Windows edition/language assets. Tested on this host:
Japanese/kana, common Chinese, Korean Hangul, Latin accents, Greek, Cyrillic and a
supplementary Deseret letter (U+10437). This is not universal Unicode, every CJK
extension, script-shaping, or provider/font-version support. Native names/action
paths remain unchanged regardless of glyph coverage. The supervisor inspected an
own-window native capture showing readable Japanese names and ASCII sort markers;
other glyphs, scaling and Narrator remain pending. Headless charmap checks alone
are not a visual acceptance claim.

`task test` checks portable behavior, including graceful absence of optional fonts.
The complete installed-font coverage assertion is an explicitly opt-in reference-host
probe, `task test:fonts`; its assertions are unchanged. The default Rust run reports
that host-dependent probe as ignored, not passed. Run it on the prepared Windows
reference host before claiming its broader installed-font coverage.

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

All panes share fixed window-owned services: enumeration/metadata reads run on
**two workers**, projection/filter/sorting runs on **one worker**, and there is at
most **one explicit file-launch worker**. No per-pane or per-tab thread pools exist.
Each visible active tab has one enumeration cancellation token and at most one
coalesced pending request. Pending targets are FIFO; updating a queued target keeps
its place rather than evicting another pane. Switching tabs/closing panes cancels
only those targets, removes their pending requests and releases their target slots.
The enumeration snapshot/status transport is bounded to **eight messages window-wide**,
and UI catch-up drains at most eight per frame, including stale responses.
Enumeration uses batches of at most 256 entries; after the first batch, cumulative
snapshots are published at most every 50 ms plus the final snapshot. Snapshots share
entry allocations, but each snapshot holds its own vector of entry handles.

The projection worker processes targets fairly from a FIFO with at most one
coalesced pending request and one result per visible active tab. Publishing a result
checks that target's latest key; another pane's request does not cancel it. Switching
or closing removes that target's queued request, result and latest key. A running
projection may finish, but cannot publish after cancellation. UI catch-up takes at
most the pane count of results per frame. Query normalization, name matching,
entry-vector building, sorting and visible-path-set building happen on that worker,
not in UI update. These transport bounds scale only with live active pane targets,
not with the number of inactive tabs or the number of input events.
Projection results must match the visible listing's generation, data/completion
revision, query revision, visibility choice/revision and sort choice. Returning
quickly to the same query text still creates a newer query revision. Completion increments data revision so a
progressive projection cannot masquerade as authoritative final data. While updating,
the previous projection can stay visible with an Updating filter/sort/visibility
indicator.
Rows remain virtualized. Loaded listings and their snapshot vectors are not size-capped; no memory/performance acceptance claim has
been made.

Every activation/refresh receives a new window-wide enumeration generation. An already
active tab moved to another pane keeps its live target/generation and pending navigation;
only an activated inactive tab or newly exposed source tab needs a refresh. The replaced
destination active tab alone is cancelled/deactivated. Reorder does not restart reads,
and unrelated panes keep their work. Combined
with the existing data/query/visibility/sort guards, this rejects inactive/closed-tab
results even when two tabs share a folder. Responses route by unique generation to
the owning pane's active tab, never to whichever pane happens to be focused;
no per-tab workers are created. Fonts and the captured-path launcher stay window-owned;
a launch completion cannot change the active tab's navigation or selection.

Stale enumeration is cancelled between OS calls and rejected by navigation.
OS calls themselves cannot be interrupted. If both enumeration workers are stuck
on unavailable locations, new listings wait in the target-aware pending FIFO, but input remains responsive.
The upgrade path is cancellable platform I/O if measured workloads require it.
A running projection is not interrupted, but obsolete results are discarded and
the next queued target is processed with its latest request. If measured filter latency exceeds its
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
11. In a generated fixture with hidden/system files and folders, check the checkbox,
    counts and all-hidden state. Select hidden rows while shown, hide them, then show:
    completed projections remove hidden selection/active/anchor without resurrecting
    them. Toggle during refresh/filtering and verify stale rows cannot activate, while
    navigation/retry/filter/toggle remain usable. Check Unicode/supplementary glyphs,
    a verbatim path over 260 UTF-16 units and escaped malformed-name diagnostics.
    Submit the unchanged malformed current-path display and verify native identity;
    edit the diagnostic and verify rejection. Type `C:relative` and verify explanatory
    rejection without history/selection changes; ordinary relative paths still use
    the committed location. GUI glyph/scaling and actual long-path associations remain
    pending; automated Windows fixtures establish enumeration/native identity only.

12. In a disposable folder with enough entries to scroll, create two tabs at the same
    location. Give them different selections, filters, sort columns and visibility
    settings; navigate one to a child and Back. Switch using tab clicks and both cycle
    shortcuts: state must remain independent. Select an early row, then manually scroll
    elsewhere and switch away/back: refresh must restore the manual offset, not jump
    to the row. Keyboard arrows must still reveal the newly active row. Externally
    remove a selected disposable entry and return: only that tab's refreshed selection
    should reconcile. Create many tabs, narrow the window and use cycle/new/close:
    the active label and plus must remain reachable; hover shows the full location.
    Verify Ctrl+T/W/Tab/Shift+Tab while editing path/filter do not affect tabs, including
    Enter/Escape exit frames. Close the last tab and confirm a fresh home tab. Do not
    modify home entries; use only generated fixture entries for changes. This native
    interactive procedure is pending, not claimed from headless checks.

13. Split a loaded fixture left/right, then split one pane top/bottom. Give panes
    independent tabs, locations, filters and selections. Click blank pane space or
    controls and use F6, Ctrl+T/W/Tab, Alt navigation, F5, arrows/Ctrl+A and Enter:
    only the focused pane should change, including a click and shortcut in one frame.
    Check the cyan border plus Focused pane label. Resize both dividers to their
    limits, try a split too small to fit, then shrink the window: controls remain
    reachable through scrolling and F6 reveals offscreen panes. Check narrow Name
    columns and [D]/[F]/[L] markers. Close a pane with several tabs, and close the
    last tab of a non-final pane: siblings expand without refreshing or losing
    selection. Close the final pane/tab to get home. During a delayed explicit
    launch, close its source pane; captured-path status remains window-owned and
    cannot navigate/select in another pane. Do not open unknown documents or modify
    personal/home entries. Native interaction/DPI/Narrator/provider checks remain
    pending; the headless/background checks do not establish those results.

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
or programs and never alter file associations. S006 adds nine focused checks for
owned hidden/system attributes, visibility revisions/selection/progressive guards,
checkbox wiring, Unicode/malformed display identity, actual disposable malformed and
verbatim long-path enumeration, and typed-path rejection/recovery. These fixtures
passed on this Windows environment without ignored/unsupported cases; they do not
establish support on every filesystem or network provider. Missing-file preflight
returns before calling the shell. These are not a foreground-capture or interactive GUI acceptance claim.
S006a adds installed-font charmap/priority and missing-font/budget checks, plus
headless ASCII sort-direction feedback. Its first coverage check exposed an
epaint 0.36.2 `has_glyphs` false-negative: it compares resolved and replacement face
IDs, so supported characters on that same face can report missing. Coverage tests
use the public `Font::characters()` charmap instead, asserting every sample code
point in both families without treating replacement glyphs as name coverage.
S006b adds disposable background parent-then-Up, absolute-dot/same-alias history and
selection, native-prefix/malformed UTF-16 and failed/stale completion regressions.
Long-path enumeration also checks that ordinary paths are not converted to verbatim
paths and existing verbatim locations remain verbatim. Live UNC/provider-specific
normalization and native navigation interaction remain manual pending.
No destructive operations or personal-folder checks are needed for this slice.

S007 adds focused tab-state and headless egui checks for create/close/cycle/final-home,
independent history/filter/sort/visibility/native selection/editor state, same-location
widget IDs, overflow reachability and saved manual scroll after progressive refresh.
An actual shared-worker disposable fixture confirms per-tab refresh/reconciliation and
an injected final-home location, without changing the user's real home. The injected
launch-result regression also closes its original tab before completion; no documents
are executed. Native interactive tabs, hover/scaling and Narrator checks remain pending.

S008 adds a small binary split tree and focused state/headless/background regressions
for both axes/nesting, divider clamps/impossible splits, focus-before-keyboard routing,
independent tab/selection/navigation state, pane collapse/final-home, narrow columns,
shrink/F6 accessibility, simultaneous disposable pane loads and repeated projections,
closed/stale target rejection, per-target FIFO/coalescing/lifecycle bounds and captured
launch completion after pane close. The worker counts remain fixed. Mutating-operation
retention, previews and persistence are later slices.

S009 adds typed egui tab dragging, transactional reorder/transfer and four edge splits.
Ten focused state/headless/background checks cover real pointer reorder/cross-strip
transfer, retained state/active identities, source-last collapse/index shifts, all
four own-edge lone-tab replacements, minimum/offscreen/stale/closed refusal, Escape
and outside release, closed hovered destinations (no retarget to an expanded sibling),
foreign payload isolation, live target retention and generation-safe inactive-tab
refresh with manual scroll restoration. These are not native drag/DPI/Narrator checks.

14. In a disposable fixture, drag tabs forward/back within a strip and between panes
    with different histories, filters, selections and manual scroll positions. Verify
    the original state moves, active identities survive reorder, destination focus and
    active-label reveal, plus/x/plain clicks and overflow scrolling. Drag to all four
    edges, including the window's lone tab: the original moves and a fresh source tab
    retains the committed location. Move a last tab to an existing pane: its source
    collapses. Try a minimum-sized or partly offscreen edge and confirm orange feedback
    and no state loss. Cancel with Escape and release outside targets; neither should
    activate rows or launch files. Native pointer interaction, DPI and Narrator checks
    remain pending; do not use document launches or personal-folder mutations as tests.

## Folder finder (S010)

**Ctrl+O** opens a focused modal over the current layout. It captures the focused
pane's active tab ID and its **committed** native folder as a frozen relative base.
Opening, querying, expanding and Escape do not navigate or change history/selection.
Escape/Close dismiss and restore browsing focus. The finder exclusively owns text,
IME, clipboard and keyboard input; background pane/tab/row/divider/drag controls
are disabled, including the opening, dismissal and successful completion frames.

An empty query lists recent successful visits, bookmarks, logical storage roots,
and common places. Recent visits are recorded only after a directory load actually
commits, never from a failed request or path probe. Windows common places are the
current user's Home, Desktop, Documents, Downloads, Pictures, Music and Videos,
resolved on the finder worker through `SHGetKnownFolderPath` with default flags
(no default-path/create flags). This respects redirection; each returned allocation
is freed with `CoTaskMemFree`, including any allocation on failure. Logical drive
roots use `GetLogicalDrives`; they are not probed just by displaying them. Inventory
is a window-session snapshot initialized on first use, not a watcher; unavailable
known-folder APIs produce visible warnings. New/remapped drives require restarting
for inventory discovery; an explicit path can still be entered. This is Windows-first;
the non-Windows development fallback has only `/` and an explicit common-place warning.

Fuzzy **whitespace-separated terms** independently match case-insensitive
subsequences in each location's name or full diagnostic path: `mado exp` matches
`mado-explorer`. Terms need not be adjacent or ordered relative to each other.
Ranking first minimizes the number of path-only terms, then the sum of start-offset
and inter-character gap penalties, then compares case-sensitive display name and
native path, with original catalogue order for exact ties. Matching uses stdlib
Unicode lowercase, not locale-aware full case folding or normalization. Matched
characters are cyan and underlined. Empty/whitespace-only queries retain category
order (newest recents, bookmarks in addition order, common places, storage roots).
A location can appear in more than one category. Non-Unicode names use the existing
escaped diagnostics for matching; selected actions always retain the native path.
No matches and empty expansion views are explicit, not implicit navigation.

Enable **Path entry** to interpret the input as an absolute path or a path relative
to the captured base (including a single folder name, spaces or `..`). This explicit
mode avoids guessing whether a fuzzy query is a relative filename. Drive-relative,
device namespace, URL and NUL input is rejected; native UNC/verbatim prefixes and
Unicode follow existing browsing normalization. Diagnostic text is not editable
path serialization; use selected native results for non-Unicode child folders.

- Up/Down selects a result and reveals it; Enter opens it in the captured tab.
- Shift+Enter opens a fresh tab in the captured pane.
- Ctrl+Enter opens a fresh **left/right** split at that folder. The source must be
  fully visible and fit two 320 x 300 panes plus a six-point divider; refusal is
  visible and creates no partial pane/tab or focus change.
- Tab (or Children) probes/expands the selected folder, or the typed path in Path
  entry mode. It replaces the results with immediate non-link folder children and
  clears the query for further fuzzy narrowing. Repeated Tab expands a selected
  child; the underlying tab has not navigated. Known locations returns to inventory.
- Bookmark/Unbookmark and Forget recent manage up to 128 bookmarks and 128 newest
  recents. Adding a bookmark when full is visibly refused. Existing settings restore
  these lists; new changes remain session-only while production writes are gated.
  The finder and window visibly disclose that gate.

Actions perform a worker-side native normalization, folder-type and enumeration-
access probe. Missing, nonfolder, denied, unsupported and ordinary OS errors identify
the captured path and keep the modal usable. Probing can succeed and subsequent
browsing enumeration still fail (filesystem races remain possible); ordinary
transactional browsing then retains prior current-tab history/location. A captured
pane/tab that closed, moved, changed its active tab or began another navigation is
refused rather than retargeted. A pending background navigation committing a
different location also invalidates the target; it never changes the relative base.
Close and reopen to capture the new context.

One fixed window-owned finder worker is added to the existing services. It has one
running job, one replaceable pending job and one result slot, not a per-query/pane/tab
thread. All inventory calls, normalization, metadata, directory iteration, matching,
emphasis indices and ranking sorts run there. UI work is bounded catalogue copying,
state checks and virtualized result rendering, not filesystem I/O or ranking.
Known-location ranking has at most 289 candidates (128 recents + 128 bookmarks +
seven common places + 26 roots). Explicit expansion examines at most 8192 entries
(with one additional iterator step to detect truncation) and retains at most 4096
folders. It never recursively scans. Truncation is visibly labelled; use an explicit
path for omitted children. Child ordering is display-name/native-path deterministic after
that bounded enumeration subset. Expanded-query ranking has at most 4096 candidates.
Fuzzy queries above 4096 characters and explicit paths above 32766 UTF-16 units
produce visible errors. Input is retained in full, never silently truncated into
a different path. Supported long explicit paths do not inherit the fuzzy-query limit. Counts bound retained vectors/transport, not a
claimed process-memory byte budget or measured latency target. Names/paths retain
native OS length limits; the GUI's result list is virtualized and the whole modal
scrolls when the window is short.

Every query, catalogue change, probe/expansion, cancellation and reopen advances a
window-wide request revision. Publishing and accepting both check it. A catalogue
change during an action cancels that obsolete action and updates the view; choose
again after updating. Old results cannot be acted upon while a request is pending.
Cancellation is checked between ranking candidates and expansion entries, and at
publication; a running bounded sort and OS calls are not interruptible. Initial
inventory performs seven known-folder calls and one logical-drive call. A probe has
normalization, metadata, opening read_dir and at most one iterator step; expansion
then opens a second iterator and makes the bounded child steps/non-following metadata reads. One stuck
provider can occupy this worker, delaying finder work but not browsing/input or the
other services. Editing coalesces the next request; Escape discards pending/result
slots. Shutdown wakes the worker and joins only if already finished, otherwise
it detaches until return/process teardown. No filesystem writes, index, content
search, associations/registry changes or automatic launches are introduced.

S010 adds thirteen focused state/headless/background tests using disposable generated
folders and injected inventory. They cover target/base capture, native path modes,
empty categories/fuzzy ranking/emphasis/no-match, failed probes/recovery, expansion
without navigation, query/catalogue/modal/probe revisions, latest-request coalescing,
closed/moved/changed targets, atomic split refusal, all finder keys, modal text/paste
and background-shortcut isolation, same-frame ready-probe edit/dismissal cancellation,
and session management/success-only recent visits. Review regressions prove input
focus and typing after Tab expansion, lossless long-path paste and explicit oversize
refusal, pointer-before-Ctrl+O target capture, and a focused AccessKit text-input node.
The accessibility probe checks emitted metadata, not native Narrator output.
Native redirected places, unavailable/slow storage providers, real ACL denial,
interactive keyboard/pointer/clipboard, scaling and Narrator remain manual pending.
S011 implements reviewed save/restore for F05.6/F13.4-F13.5. Disposable native
final-save, normal/minimized-close and application-written-settings restart checks pass.
Narrator, mixed-DPI and disconnected-monitor checks remain pending.

15. Using only disposable folders, open Ctrl+O from each pane with different active
    tabs/filters/selections; verify the captured base and exclusive input. Try fuzzy
    terms/no-match, absolute and relative Path entry, repeated Tab, Up/Down and all
    three Enter actions. Confirm only explicit acceptance changes browsing, and a
    too-small split shows refusal without a new tab/pane. Test missing/nonfolder and
    a prepared denied fixture, then recover without reopening. Open during pending
    navigation: its later commit must not change the frozen base and should refuse
    acceptance until reopened. Escape/Close must not navigate or leak Ctrl+T/F6/row
    input. Check the actual redirected common places and logical roots without
    modifying personal entries. Bookmark/unbookmark/forget fixture locations and
    restart: changes survive when saving is available; corrupt settings remain untouched.
    Check narrow/short windows, 100/150/200%
    scaling and Narrator; these native/manual checks are not yet claimed as passed.

## Workspace settings

Production settings writes are enabled after independent backend and override-guard
safety reviews. Fourteen focused regressions cover schema/native paths, corrupt-file
preservation, atomic failure handling, coalescing and close behavior. Native checks
use generated disposable settings only; no personal settings or clipboard were accessed.

Startup reads `workspace.json` under the Windows LocalAppData known folder's
`Mado Explorer` directory. For disposable native checks or a separate settings
location, set `MADO_SETTINGS_PATH` to an absolute native path ending in
`workspace.json`. Invalid overrides visibly disable saving and never fall back to
personal settings. Named-pipe/device namespaces and ambiguous UNC prefix names are
refused before settings file access. Ordinary filesystem UNC paths remain supported.
Reads and serialization are capped at 2 MiB. Version 1
uses explicit serde JSON fields and native UTF-16 unit arrays on Windows, including
unpaired surrogates. Display strings never encode action paths. Validation bounds
64 panes, 256 total tabs, split depth 16, active/focus indices, tree membership,
ratios, geometry, native path lengths and 128 entries per catalogue list.

Saved fields are normal window bounds/maximized state, split tree/ratios, all tabs'
committed locations, active tabs, focused pane, bookmarks and recents. History,
selection, filters, scroll, drafts, clipboard and operations are not saved.
Unavailable folders use ordinary loading/error/Up handling. An explicit startup
folder replaces only the restored focused active tab's committed location. Without
an argument the workspace restores; without valid settings startup uses cwd.

Invalid, oversized, version-invalid or unreadable settings produce defaults and a
visible warning. The original is untouched and its save path stays disabled for
that session. Repair the file and restart to re-enable saving.
No backups or unrelated temps are deleted.

One fixed settings worker has one coalesced pending snapshot
and one report. Changes schedule a save after 500 ms; filesystem work and bounded
serialization run off the UI. A failure pauses saves until explicit Retry and
shows its error plus the full recovery-temp path on hover if cleanup failed.
The writer reserves a unique same-directory `create_new` temp, writes, flushes,
syncs, checks native close, then uses `MoveFileExW` replacement with write-through.
It never deletes the old settings before replacement.

Close freezes workspace handlers and waits for the latest snapshot. Failure or
unavailable saving offers Keep browsing, Retry save and close where available,
or explicit Close without saving. `on_exit` joins the worker rather than detaching
a last save; an unexpected final failure gets a native warning. An uninterruptible
filesystem call can delay final exit. Eframe's persistence feature/save hook is
not used. Monitor bounds from winit recover disconnected/offscreen positions;
physical outer position and logical normal inner size are retained. Disposable
native maximize/restore/minimized-close, final save and saved-workspace restart
checks pass. Mixed-DPI, disconnected monitors, Narrator and native save-error/retry
interaction remain pending. Run the owned-fixture smoke with
`pwsh -NoProfile -File handoff/s011-native-smoke.ps1 -ExpectWrites -MinimizedClose`.
