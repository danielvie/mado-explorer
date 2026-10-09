# Mado Explorer

S001: a Windows-first, read-only directory browser using Rust 1.95+, eframe/egui
and the wgpu renderer. This is the first runnable slice, not the full release in
SPEC.md. A working graphics adapter/driver is required.

```sh
task run                          # browse the working directory
task run -- "C:\path\to\folder"  # or: cargo run -- "C:\path\to\folder"
task test                         # cargo test; disposable generated fixtures
task clean                        # removes target only
task design                       # opens the Excalidraw reference
task design-test                  # validates the reference
```

Enter a folder path and press Enter/Go, double-click a folder name (or focus its
button and press Enter), or click Refresh to retry/reload. Relative typed paths
resolve against the displayed location. Navigation has no history yet. No file
opening, selection, filtering, sorting, previews, settings or mutations are
implemented. Hidden entries are included. Paths used for folder activation stay
native/lossless; display/path-editor text is lossy for non-Unicode names.

The table shows immediate entries only, modification timestamps in UTC, and
regular-file sizes in bytes. `—` means unknown/not applicable, never zero folder
size. Metadata errors remain attached to their rows and can be read by hovering
names. Links/reparse points are labelled and not followed for metadata or folder
activation. Listings retain filesystem enumeration order for now.

Enumeration and metadata reads run on two background workers. There is one
coalesced pending location and a bounded eight-message queue (at most 256 rows
per batch); stale generations are cancelled between OS calls and rejected by the
UI. OS calls themselves cannot be interrupted. If both workers are stuck on
unavailable locations, a new listing waits, but input remains responsive. The
upgrade path is cancellable platform I/O if measured workloads require it.
Complete loaded listings are not capped. Rows are virtualized; no benchmark or
memory/performance acceptance claim has been made.

## Pending Windows smoke procedure

Use only a disposable folder containing a child folder and a small file:

1. Launch with `task run -- "<fixture>"`; check dark layout, names/types, UTC times,
   file byte count and unknown folder size. Resize and scroll a larger fixture.
2. Open the child, enter the fixture path again, then Refresh. Check the empty
   child state; enter a missing path and recover with another path/Refresh.
3. Use a separately prepared access-denied fixture and an unavailable drive;
   check distinct errors and usable navigation. Permission error classification
   is unit-tested, but a real ACL-denial and unavailable-drive smoke is pending.
4. Navigate A to B while A is slow; verify only B entries appear. The generation
   guard has a deterministic automated regression, not a measured slow-drive run.

GUI appearance/interaction, display scaling and Windows Narrator checks still
require manual verification. A process-level Windows startup/close smoke created
the native window and exited with code 0; it did not verify the displayed states.
No destructive operations or personal-folder checks are needed for this slice.
