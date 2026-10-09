# Mado Explorer

S002: a Windows-first, read-only directory browser using Rust 1.95+, eframe/egui
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

Enter a folder path and press Enter/Go, double-click a folder name (or focus its
button and press Enter), or use Back, Forward, Up and the breadcrumb buttons.
Alt+Left/Right/Up navigate; Refresh/F5 reloads. These shortcuts are suspended while
the path editor has focus. Relative typed paths resolve against the **committed**
location, including when a new destination is still loading. Breadcrumbs identify
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
locale-aware or natural-number ordering. No selection exists yet, so selection
preservation under sorting remains unimplemented.

No regular-file opening, selection, filtering, previews, settings, tabs/splits or
mutations are implemented. Hidden entries are included. Paths used for folder
activation, row identity, breadcrumbs and history stay native/lossless;
display/path-editor text is lossy for non-Unicode names. Submitting the unchanged
displayed current path retains its native path instead of reparsing lossy text.

The table shows immediate entries only, modification timestamps in UTC, and
regular-file sizes in bytes. `—` means unknown/not applicable, never zero folder
size. Metadata errors remain attached to their rows and can be read by hovering
names. Links/reparse points are labelled and not followed for metadata or folder
activation.

Enumeration and metadata reads run on two background workers, with one coalesced
pending location and a bounded eight-message snapshot/status queue. Enumeration
still uses batches of at most 256 entries; after the first batch, cumulative
snapshots are published at most every 50 ms plus the final snapshot. Snapshots
share entry allocations, but each snapshot holds its own vector of entry handles.
A separate sorting worker has one active sort, one coalesced pending request and
one result slot. Cloning entry-handle vectors and sorting happen on workers, not
in UI update. Sort results must match the visible listing's generation, data
revision and sort choice. While sorting, the previous projection stays visible
with a Sorting indicator. Rows remain virtualized. Loaded listings and their
snapshot vectors are not size-capped; no memory/performance acceptance claim has
been made.

Stale enumeration is cancelled between OS calls and rejected by navigation.
OS calls themselves cannot be interrupted. If both enumeration workers are stuck
on unavailable locations, a new listing waits, but input remains responsive.
The upgrade path is cancellable platform I/O if measured workloads require it.
A running sort is not interrupted, but obsolete results are discarded and only
the latest pending request is processed next.

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
6. Navigate A to B while A is slow; change sort while loading and navigate again.
   Verify late entries/sorts never populate the wrong listing and history changes
   only on successful latest navigation. Automated state/worker regressions do
   not substitute for a measured slow-drive interaction check.

GUI appearance/interaction, display scaling and Windows Narrator checks still
require manual verification. S001's process-level startup/close smoke created the
native window and exited with code 0; it did not verify displayed states. S002
has automated sorting, navigation, background pipeline and headless shortcut
checks, but no foreground-capture or interactive GUI acceptance claim.
No destructive operations or personal-folder checks are needed for this slice.
