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
