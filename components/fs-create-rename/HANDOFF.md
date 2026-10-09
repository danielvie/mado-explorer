# P02 create and rename backend handoff

## State and ownership

Partial, NOT ready for promotion or application enablement. Rename is disabled by default and returns `Error::RenameSafetyBlocked`. The `unreviewed-native-rename` feature exists only to reproduce disposable native probes, including the retained failing collision regression. Do not enable it on personal data.

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename`
- Branch: `lane/fs-create-rename`
- Baseline: `6b363f9`
- Exact implementation commit: `4c9b1f6ce4357b6f7589a0aa0b680ebbde9818b4`. Checkout was clean after that commit. This handoff is a subsequent documentation-only commit; its exact final HEAD is reported to the supervisor with the completion message.
- Exclusive paths: `components/fs-create-rename/**`. No root manifest, lock, tracker, GUI, sibling, or main-branch changes.
- Mandatory independent safety review remains outstanding. No delegation, merge, push, or personal-folder mutation occurred.

Changed paths, relative to the checkout above:

- `components/fs-create-rename/Cargo.toml`
- `components/fs-create-rename/Cargo.lock`
- `components/fs-create-rename/Taskfile.yml`
- `components/fs-create-rename/src/lib.rs`
- `components/fs-create-rename/src/windows.rs`
- `components/fs-create-rename/tests/safety.rs`
- `components/fs-create-rename/HANDOFF.md`

## API

Windows exports:

```rust
validate_name(name: &OsStr) -> Result<(), Error>
create_empty_file(parent: &Path, name: &OsStr) -> Result<PathBuf, Error>
create_directory(parent: &Path, name: &OsStr) -> Result<PathBuf, Error>
CapturedRename::capture(source: &Path) -> Result<CapturedRename, Error>
CapturedRename::source(&self) -> &Path
CapturedRename::rename(self, name: &OsStr) -> Result<PathBuf, Error>
```

All work is synchronous. Integration must call it on the operation worker, not the UI thread. There is no GUI, queue, copy, move, deletion, batch rename, automatic retry, or root application integration.

Create opens the absolute parent one component at a time and uses handle-relative `NtCreateFile` with `FILE_CREATE`. Existing files, directories, and other entries are never reused or truncated. Success returns a native verbatim absolute path. No recursive creation, file contents writes, or temporary names.

Capture retains the original native handle and its volume serial plus 128-bit file ID. The retained handle prevents identity reuse. Ancestor directory handles deny delete sharing until capture completion/drop, so ancestor rename/removal cannot retarget the operation. The source itself may disappear or be replaced. The experimental commit reopens the original name with DELETE access, denies delete sharing, compares native identity, rejects reparse sources, and commits using that same reopened handle. It uses `NtSetInformationFile` with replacement false and a relative destination under the retained parent handle.

Names stay UTF-16, including native unpaired surrogates. Validation rejects empty/dot/dot-dot names, separators, NUL/control/illegal Windows characters, alternate stream syntax, reserved DOS names with extensions and superscript digits, CONIN$/CONOUT$/CLOCK$, trailing dots/spaces, and more than 255 UTF-16 code units. The OS remains authoritative for tighter filesystem limits.

Paths must be absolute drive or UNC paths, or their ordinary verbatim equivalents. Relative/drive-relative/root-relative paths, dot components, device/GLOBALROOT/volume-GUID namespaces, alternate streams, invalid components, and reparse ancestors/sources are rejected. Existing unusual names outside this conservative policy cannot be renamed. No lossy conversions are used for operations; display formatting is diagnostic only. Non-Windows builds expose no mutating entry points.

Errors preserve the action, native path, and OS error where applicable. The experimental `CommitUnverified` error means a successful native call could not have its final spelling confirmed. It names both paths and forbids treating the outcome as an unchanged-source failure or automatically retrying it. Default rename does not reach a mutating call.

## Blocker and diagnosis boundary

The retained `late_file_directory_and_hardlink_collisions_preserve_every_entry` regression fails for a destination hard link to the captured source. On this Windows NTFS host, replacement false still returns success and coalesces the names:

```text
collision 0: error 183, source_exists=true, destination_exists=true
collision 1: error 183, source_exists=true, destination_exists=true
collision 2: Ok(destination), source_exists=false, destination_exists=true
assertion failed: collision kind 2
```

No content bytes were lost, but the existing-entry collision removed the source name instead of refusing the operation. This is not claimed as satisfying the strict no-overwrite/collision contract.

Two focused strategies were unsuccessful, then implementation stopped at the diagnosis boundary:

1. Query the source handle's native name before and after commit. This does not detect coalescing: the handle reflects the destination after the source name is removed.
2. Deny write sharing in addition to delete sharing. A disposable native probe still created a new hard link successfully. Sharing locks do not protect against this alias-creation race. The probe now asserts the observed behavior rather than the disproved assumption.

The collision regression remains failing behind the explicit diagnostic feature. The default API is fail-closed, and the default regression confirms no rename mutation. `task test` includes the diagnostic suite and therefore remains failing. Passing default checks do NOT establish a complete rename implementation.

Next supervisor action: classify the native same-file alias behavior against the contract, then commission a bounded resolution or bring the consequential semantics decision to the user. A preflight collision check alone cannot establish race safety because new hard links are possible while the source is locked. Do not remove the failing regression or enable rename to make progress.

## Exact checks and outcomes

Each command below was run from the authorized checkout. Toolchain: Windows host, Rust/Cargo 1.95.0. Fixtures were uniquely reserved directories under the OS temporary directory, with generated contents and owned cleanup only.

Formatting and compile, passed without warnings in final state:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && cargo fmt --manifest-path components/fs-create-rename/Cargo.toml --check
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && cargo check --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 --tests
```

Default disposable suite, passed: one unit name-rule test and seven safety tests, one explicitly ignored symlink probe. Covers create file/directory, file/directory collisions and content preservation, invalid names and namespaces, eight simultaneous creators with exactly one winner, immutable capture/drop and ancestor pinning, explicit rename disablement, and native hard-link lock behavior.

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && cargo test --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 -- --nocapture
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && task --taskfile components/fs-create-rename/Taskfile.yml run
```

Experimental native suite, FAILED, exit 101: one unit test passed; safety suite 12 passed, one failed, one ignored. The failing hard-link collision is the blocker above. Other experimental probes passed for file bytes, non-empty directories, case-only rename with actual spelling, same-name rename, disappearance, replacement identity rejection, no-delete-share locks, native Unicode and an unpaired surrogate, and paths over 260 UTF-16 units. These are experimental backend evidence only, not acceptance claims.

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && cargo test --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 --features unreviewed-native-rename -- --nocapture
```

Explicit reparse probe, unavailable: FAILED before exercising the backend, exit 101, Windows error 1314 on symlink fixture creation. No reparse-guard pass is claimed. It is ignored by normal suites with an explicit reason and has a dedicated task.

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && cargo test --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 --test safety reparse_source_and_ancestors_are_rejected_without_touching_target -- --ignored --exact --nocapture
```

Task listing and staged whitespace inspection passed:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && task --taskfile components/fs-create-rename/Taskfile.yml --list
cd C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename && git diff --cached --check
```

`task run` is this library's safe disposable check, not a GUI. `task test` runs both default and experimental suites, retaining the known failure. `task clean` only targets generated `target/p02` output; it was not run. `task test:reparse` is the explicit privilege-dependent probe.

## Coverage and remaining limits

Backend-only partial criteria, no acceptance statuses edited:

- F09.1: isolated create behavior and collision tests implemented. Required safety review, reparse and ACL/provider checks remain.
- F09.3: name/path guards implemented and checked. No editor/error UI.
- F02.4: retained immutable path/native identity and experimental revalidation evidence. Rename is disabled, with no selection/operation-worker integration.
- F09.4: native case-only success observed without temporary names. Hard-link collision safety is unresolved. Case-sensitive directory/provider behavior is untested.
- F09.5: disposable create contents, experimental rename contents/disappearance/locking/replacement checks. No real permission-denial evidence or tab reconciliation.
- F09.2/F09.6: UI and single-selection enablement are outside this component. No full F09 criterion is claimed verified.

Further limits:

- Real ACL denial, symlinks/junctions/mount points, UNC/removable/provider filesystems, case-sensitive directories, short-name aliases, mapped/SUBST root changes, and concurrent reparse transformations need checks/review. OS drive/share root resolution is used; no physical-root mapping guarantee is claimed.
- Unsupported native file IDs fail closed; there is no weaker metadata-identity fallback.
- Captures retain O(path depth) handles and can prevent other processes from renaming/removing ancestors until dropped. Keep captures short-lived in future integration.
- Retained identity is not a content snapshot and does not stop concurrent writers.
- Success paths use verbatim native absolute representation. Integration must reconcile affected locations without assuming textual equality with ordinary tab paths.
- Fresh independent filesystem safety review is mandatory before promotion or personal-data use, including creation. No manual or skipped check is marked passed.

## P02b bounded diagnostics

Completed diagnostic work only. No production fix or enablement. Starting clean HEAD was `41cb9f72d9a0e292dbfdbd6b8b41b35a39e64d7d` on `lane/fs-create-rename` in the checkout named above. The supervisor recorded this slice in main PROGRESS.md before launch. Targets are F09.4 primitive diagnosis and static F09.1/F09.5 reparse evidence, not complete criteria.

Exclusive changed paths:

- `components/fs-create-rename/tests/diagnostics.rs`
- `components/fs-create-rename/Taskfile.yml`
- `components/fs-create-rename/HANDOFF.md`

`src/`, manifest, lockfile, default rename gate, and `tests/safety.rs` are unchanged. No root/main/sibling edit, delegation, merge, push, personal-data test, elevation, or Developer Mode request. The new matrix is explicitly ignored and invokes native calls only within its private test helper. `task test` still exposes the original failing regression.

The supervisor's independent review of the original implementation confirmed the collision blocker without another concrete FFI/data-loss defect. Any later production change still needs independent review before enablement.

### Native comparison result

One nine-call matrix ran on Windows build `10.0.26200.0`, `x86_64-pc-windows-msvc`, C: NTFS, Rust/Cargo 1.95.0. Each call used a separately reserved flat `mado-p02b-<pid>-<counter>` directory under `C:/Users/daniel/AppData/Local/Temp`. Sources contain exactly `b"source bytes"`; ordinary destinations contain exactly `b"destination bytes"`. Before and after directory enumeration recorded exact names, volume serial, all 128 file-ID bits, and bytes. Case-only results use exact enumeration spelling, not case-insensitive path existence.

The fixture uses backend capture to pin/reject ancestors and retain source identity. It separately captures a native source ID, reopens the fixed name with DELETE and no delete sharing, compares IDs, checks non-reparse attributes, and mutates only through that reopened handle under the retained parent. Destinations for ordinary/hard-link cases are created after revalidation. All buffers are aligned and zero-filled. Replacement bit 1 is never set. Legacy uses class 10/flags 0; Ex variants use class 65/flags 0 or 2. No additional flag search was performed.

`C` below is `NTSTATUS 0xc0000035`, Win32 183. `S` is `NTSTATUS 0x00000000`, Win32 0.

| Variant | Existing ordinary file | Existing source hard link | Case-only `Case.txt` to `case.TXT` |
| --- | --- | --- | --- |
| Legacy, class 10, flags 0 | C; both names, IDs and bytes unchanged | S; source name removed, destination ID/bytes retained | S; one entry with requested spelling, same ID/bytes |
| Ex-zero, class 65, flags 0 | C; both names, IDs and bytes unchanged | S; source name removed, destination ID/bytes retained | S; one entry with requested spelling, same ID/bytes |
| Ex-POSIX-only, class 65, flags 2 | C; both names, IDs and bytes unchanged | C; both names, same ID and bytes unchanged | C; original spelling, ID and bytes unchanged |

Exact observed IDs below are the 16-byte identifiers as hexadecimal in returned byte order. All use volume serial `3172367324309295054`. A hard-link destination starts with the same ID as its source. Case-only starts with one entry. After-state identity mappings are described in the matrix above; no remaining entry's ID or bytes changed.

| Variant/scenario | Source ID before | Destination ID before |
| --- | --- | --- |
| Legacy/ordinary | `fcea1c00000019000000000000000000` | `94f21c0000001b000000000000000000` |
| Legacy/hardlink | `fcea1c0000001a000000000000000000` | same as source |
| Legacy/case-only | `fcea1c0000001b000000000000000000` | no separate entry |
| Ex-zero/ordinary | `fcea1c0000001c000000000000000000` | `94f21c0000001c000000000000000000` |
| Ex-zero/hardlink | `fcea1c0000001d000000000000000000` | same as source |
| Ex-zero/case-only | `fcea1c0000001e000000000000000000` | no separate entry |
| Ex-POSIX-only/ordinary | `fcea1c0000001f000000000000000000` | `94f21c0000001d000000000000000000` |
| Ex-POSIX-only/hardlink | `fcea1c00000020000000000000000000` | same as source |
| Ex-POSIX-only/case-only | `fcea1c00000021000000000000000000` | no separate entry |

Ex-POSIX-only protects the static alias in this observation but refuses case-only rename. No candidate both protects aliases and permits case-only rename. Consequently no conditional concurrent-alias follow-up was added. Stop this flag-comparison avenue here and return the tradeoff to the supervisor. Do not combine a preflight guess with these outcomes and call it race-safe.

The primary [FILE_RENAME_INFORMATION documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_file_rename_information) was read. It describes POSIX semantics in conjunction with the replacement flag for replacing an open target. It provides no separate same-file collision guarantee for POSIX-only. This table reports observed host behavior, not a documented guarantee. The diagnostic test's passing result means it completed its observations and remaining-entry integrity checks. It explicitly does not assert that the no-overwrite criterion passed.

### Static junction sentinel

The new default test invokes the OS `cmd.exe /D /C mklink /J` facility between two generated owned fixtures. No privilege change was needed. File creation, directory creation, capture of the junction itself, and capture of a source through the junction all returned `Error::ReparsePoint`. The target retained its sole `sentinel` entry with identical native identity and exact `b"untouched"` bytes. No `escape` entry was created. The junction was removed with nonrecursive `remove_dir`, then target identity/bytes were checked again before fixture cleanup.

A junction guard removes the link itself before target fixture cleanup, including assertion unwinding. All new fixtures use nonrecursive cleanup only. No cleanup error was emitted during these runs.

This establishes a static junction check only. Concurrent reparse transformations, symlink creation with the unavailable Win32 1314 privilege, mount points, root mappings, provider/UNC/removable/case-sensitive filesystems, real ACL denial and GUI/tab reconciliation remain pending. The original privilege-dependent symlink test remains unchanged and ignored; it was not retried.

### P02b commands and outcomes

Commands ran from `C:/SANDBOX/WORKTREES/mado-explorer/fs-create-rename`:

```sh
rustfmt --edition 2024 components/fs-create-rename/tests/diagnostics.rs
cargo fmt --manifest-path components/fs-create-rename/Cargo.toml --check
cargo test --locked --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 --no-run
cargo test --locked --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 -- --nocapture
cargo test --locked --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 --test diagnostics native_no_replace_variant_matrix -- --ignored --exact --nocapture
cargo test --locked --manifest-path components/fs-create-rename/Cargo.toml --target-dir target/p02 --features unreviewed-native-rename --no-run
task --taskfile components/fs-create-rename/Taskfile.yml test
task --taskfile components/fs-create-rename/Taskfile.yml --list
git diff --check
git diff --exit-code HEAD -- components/fs-create-rename/src components/fs-create-rename/Cargo.toml components/fs-create-rename/Cargo.lock components/fs-create-rename/tests/safety.rs
```

- Formatting and both locked test-compilation modes passed. The first compile emitted two constant-pattern naming warnings in the new test; replacing the pattern with constant comparisons removed both. Final compilations were warning-free.
- Default suite passed: 9 tests across unit/safety/diagnostics, 2 explicitly ignored probes. The added static junction test passed.
- Exact ignored matrix ran once: 1 diagnostic test completed, all nine native outcomes recorded above. No rename acceptance claim.
- `task test` FAILED as intended: the unchanged classic hard-link collision regression still reports `Ok(destination)`, `source_exists=false`, `destination_exists=true`, then fails at `tests/safety.rs:204`, collision kind 2. Experimental safety suite 12 passed, 1 failed, 1 ignored; unit and static junction checks passed, matrix ignored. Cargo exit 101; Task wrapper exit 201. This is a known unresolved production blocker, not a passing test run.
- Task listing and whitespace checks passed. Read-only diff of source, manifest, lockfile and original safety test was empty. Git reported only its LF-to-CRLF notice for edited Markdown/YAML, not a whitespace defect.

New `diagnostic:rename` task runs the exact ignored matrix with `--locked`. New `test:junction` task runs the static sentinel test with `--locked`. Existing `run`, `test` and `test:reparse` behavior is retained. Build-output cleanup was not run.

No full F09/F02.4 or GUI criterion is satisfied by this slice. F09.4 remains blocked, rename remains disabled, and creation remains unpromoted. Next action is supervisor selection of a separately scoped approach or consequential decision, with the strict collision/case-only contract unchanged. Any proposed production fix requires focused race evidence and independent safety review before enablement.
