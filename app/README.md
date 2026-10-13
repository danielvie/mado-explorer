# Wayfarer Files

Independent Rust desktop file manager targeting Windows 11 x64. The application has its own identity and state format. It does not execute File Pilot, import its configuration, or claim behavioral/pixel equivalence. Original analysis/reference files remain separate.

## Build and run

Use the pinned Cargo.lock with Rust 1.95 or newer and the Windows MSVC toolchain/Windows SDK. Run these commands from the repository root:

```powershell
cargo build --locked --manifest-path app/Cargo.toml
cargo run --locked --manifest-path app/Cargo.toml -- C:\Your\Folder
cargo build --locked --release --manifest-path app/Cargo.toml
task -d app --list
task -d app release
```

The release executable is `app/target/release/wayfarer-files.exe`. A single optional positional directory selects startup navigation; saved session restoration may supersede it. The executable can be copied to a user-chosen location. No installer, automatic elevation, registry/default-folder registration, or PATH change is performed. The embedded MSVC manifest requests `asInvoker`, PerMonitorV2 DPI awareness, and long-path awareness; these declarations do not prove every Shell API/path or display configuration works. Windows 10, ARM, and non-Windows builds are not supported claims.

## Implemented surfaces

The code includes asynchronous lossless-path directory browsing, virtualized rows, filtering/sorting and hidden/system controls, selection/history, tabs and resizable splits. It includes command/palette actions, Shell menus, clipboard/OLE adapters, file operations with result reporting, text/image previews, watcher refresh, and session restoration. Extended surfaces include descendant listings, folder totals, batch rename preview/commit, Recycle Bin browsing/actions, layout presets, bookmarks/recents, appearance/input/hotkey settings, external tools, shortcuts, and separately confirmed permanent deletion.

These are implementation surfaces, not a claim that all acceptance scenarios passed. Consult [the canonical checklist](../tasks/todo.md) for integrated task status and actual command results. [Deferred tests](../tasks/deferred-tests.md) retain user-dependent scenarios, prerequisites and expected outcomes. Native GUI, Explorer interoperability, Narrator, IME, high contrast and mixed-DPI acceptance must be recorded independently from compilation/unit tests. Never mark deferred checks passed merely because a release executable builds.

## State and limits

State is independent JSON under `%LOCALAPPDATA%\WayfarerFiles`, protected by an exclusive writer lease. Filesystem paths retain Windows UTF-16 code units. Recycle Bin tabs use an explicit location kind with no filesystem path; no live handles or PIDLs are serialized. A corrupt/unsupported session is preserved; startup uses safe defaults and disables saving after restore failure. Saves synchronize a same-directory temporary file and use Windows replacement without deleting prior state as a fallback. Named layouts are embedded in the session rather than arbitrary user-chosen files.

The codec limits sessions to 4 MiB, 128 tabs, and 32 named layouts. Preset names are unique and at most 128 UTF-8 bytes. Layout ratios must be finite and within 0.05–0.95. The codec and UI restoration limit layout depth to 12 to bound egui_dock's sparse tree allocation. Missing/unavailable locations report errors. Window position, size and maximized state are stored in logical points and validated/clamped to the current monitor work area during restoration. Per-tab view, icon size, inspector visibility/zoom/wrap/line numbers round-trip; full navigation history and every transient UI detail are not serialized.

Clipboard/mutation operations are not transactional; inspect per-item outcomes after failure or cancellation. Trash must never silently fall back to permanent deletion. Incoming drops have one transfer authority. Link drag effects remain disabled pending their separate interoperability/approval gate. Virtual-file streams, automatic elevation, global Recycle Bin emptying, original application config import, proprietary assets and exact original option parity are excluded. Network and long-path behavior depend on the particular Windows APIs and environment; unsupported cases must remain visible.

The [Windows acceptance matrix](tests/windows-acceptance.md) maps all 45 task IDs to source checks and deferred scenarios. Image zoom is 0.01–4x; icon size is 48–192 logical points. Typography uses bundled proportional/monospace fonts, with size 10–32 and density 0.75–2. Arbitrary external font loading is unsupported. Recents use exact-path MRU deduplication with a 32-entry limit; bookmarks are capped at 256. Searchable Shell menus initialize a bounded projection on the first nonempty query and retain native fallback; installed-extension compatibility remains subject to verification.

Synthetic diagnostic budgets are 250 ms for 100,000-entry construction plus three headless frames and 128 MiB peak for the headless suite. These do not promise filesystem/GPU/network performance; measured baseline and deferred real-folder checks are in the acceptance matrix.

## Verification

```powershell
cargo fmt --manifest-path app/Cargo.toml -- --check
cargo clippy --locked --manifest-path app/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path app/Cargo.toml
cargo test --locked --manifest-path app/Cargo.toml native_ -- --ignored --test-threads=1 --nocapture
cargo test --locked --manifest-path app/Cargo.toml native_runtime -- --ignored --test-threads=1
cargo test --locked --manifest-path app/Cargo.toml native_image_decode -- --ignored --test-threads=1
cargo test --locked --manifest-path app/Cargo.toml native_operations -- --ignored --test-threads=1
```

Native tests are opt-in and serialized. The full `native_` filter includes WIC/OLE/read-only Bin diagnostics and owned copy/create/rename/move/Trash/exact-restore, shortcut and confirmed permanent-delete fixtures; it performs real mutations only inside its disposable fixture scope. Read their fixture behavior before running; mutation tests must act only on newly created disposable fixtures. Do not run them against user files or change system/security settings to manufacture fixtures. The application-owned `--window-boot-probe` mode opens a short native window lifecycle probe and bypasses normal session loading/saving; it is an automated check, not manual accessibility or drag/drop validation. Root `task test` concerns reference analysis and is separate from application tests.

`task -d app clean` calls Cargo clean for this application's build output. It does not remove session data, reference analysis, or user documents. A release build is an artifact for testing; publishing/distribution and verified release acceptance remain separate actions.
