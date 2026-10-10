# P18 create-only backend

## Candidate and boundary

- Checkout `C:/SANDBOX/WORKTREES/mado-explorer/fs-create`, branch `lane/fs-create`.
- Starting HEAD `29ad63aea356ad8d31ee6ce7a06ea7fe37cdb4a6` and clean checkout confirmed before writes.
- Owned paths only `components/fs-create/**`. No root, P02, source-snapshot, UI, queue, clipboard, settings or design edits. No delegation, integration or production enablement.
- Targets F09.1/F09.3/F09.5 backend prerequisites only. Full app criteria remain incomplete. Fresh independent filesystem-safety review of the frozen owned commit is required before promotion or production integration.

## API and mutation boundary

`CapturedDirectory::capture(PathBuf)` retains the exact original native path, native volume/file ID and an owned handle for the root and every directory component. `original_path()` returns the unchanged spelling. `identity()` describes the captured target object. Explicit callers invoke `create_empty_file(&OsStr)` or `create_directory(&OsStr)` on that capture. Returned/error paths preserve the native original spelling, never display-text conversion. Capture and create are synchronous and belong on a background caller, not the UI thread.

Capture opens ancestors one component at a time relative to the previous retained handle. Non-disk objects, unsupported native identity, non-directories and reparse entries refuse capture. Every directory handle denies data-write/delete sharing through capture lifetime, protecting names against rename/removal/replacement. Partial capture errors and Drop close all handles without mutation.

Creation validates a single bounded lossless UTF-16 basename before I/O. It rejects empty/dot names, separators, NUL, C0/C1 controls, colon and other Windows illegal characters, trailing dots/spaces, DOS-device stems, and names exceeding 255 UTF-16 units. The conservative policy also reserves COM0/LPT0, CLOCK$, CONIN$ and CONOUT$. Supported unpaired surrogates remain unchanged.

The only mutation is synchronous `NtCreateFile` with `FILE_CREATE`, one relative basename, the **captured target handle** as `RootDirectory`, `OBJ_DONT_REPARSE`, and file-versus-directory options. It atomically refuses existing names. There is no existence preflight, overwrite, replacement, recursive traversal, temporary entry, rename, rollback, or path-based mutation fallback. Native failure codes remain in `Error::Io` with action and native path.

## Initial support ceiling

The supervisor authorized a LOCAL NTFS-only fail-closed guard for this initial backend, not a product-scope deferral. UNC spellings are validated but refuse as `UnsupportedProvider` before contacting a provider. Drive roots are queried by their retained handle with `FileFsDeviceInformation`: require disk device and neither REMOTE_DEVICE nor REMOTE_DEVICE_VSMB. `GetVolumeInformationByHandleW` must return NTFS. The captured target is queried again before mutation. Unknown/remote/non-NTFS/query-failing providers refuse, never fall back.

Ordinary absolute drive paths, forward/mixed ordinary separators, their verbatim equivalents, and supported long paths work internally without lossy normalization. No relative paths, dot components, ADS, DOS/device/pipe/GLOBALROOT/volume-GUID namespaces, ambiguous empty components or unsafe trailing aliases. A single trailing separator is supported. Paths are bounded to 32,760 input UTF-16 units; native API constraints still apply. Native basename/path admission uses bounded temporary allocations. Reparse ancestors and conflicting directory handles are deliberately unsupported. O(depth) directory handles remain live until capture drops. Capture success does not promise later permission, volume availability or crash durability.

Other required filesystem/provider support remains pending for later owned implementation and verification. No universal filesystem, filter-driver, remote/removable-provider, ACL, or privileged-reparse guarantee is claimed.

## Reparse findings and containment evidence

Share modes alone do **not** prevent metadata reparse conversion. A real `FILE_WRITE_ATTRIBUTES`-only handle successfully installed an owned junction on a pinned empty target using `FSCTL_SET_REPARSE_POINT`, despite no write/delete sharing. This disproved the initial test assumption that share denial would prohibit conversion. The diagnostic is retained as `native_relative_create_refuses_target_converted_to_reparse_after_capture`, which requires actual conversion, observed refusal for both create kinds, and unchanged owned outside sentinel names/bytes.

Whole-chain pre-operation attribute/identity queries refuse observed conversions. They are **not** the atomic containment guard. `reparse_conversion_after_last_observation_is_refused_by_native_create` injects actual target conversion after all provider/chain queries, immediately before `NtCreateFile`. File and folder creation both return real Win32 **1921**, and the separate generated sentinel directory retains exactly its original name and bytes. The native `OBJ_DONT_REPARSE` call supplies this refusal, not a query-to-use interval. An earlier bounded native follow-up also returned 1921 before adding chain queries.

Converted-ancestor assessment: creation never re-resolves the original path or ancestor spellings. `RootDirectory` is the retained captured target object, not a path reopened after validation. Ancestor rename/removal cannot swap it because pins deny delete sharing. Changing an ancestor's spelling resolution cannot replace that handle with another directory. Observed reparse ancestors refuse. The actual local NTFS junction-conversion attempt on a nonempty pinned ancestor returned Win32 **145**, DirectoryNotEmpty, even through an attribute-only handle. The test retains that result and verifies original-target creation plus untouched separate sentinel. This is **not** empirical coverage of a successfully converted ancestor. Other tags/providers and privileged conversions remain unverified and are not enabled or probed here. Fresh safety review must inspect the handle-binding argument and these limits.

## Checks and outcomes

Run from `C:/SANDBOX/WORKTREES/mado-explorer/fs-create`:

```text
cargo generate-lockfile --offline --manifest-path components/fs-create/Cargo.toml
cargo fmt --check --manifest-path components/fs-create/Cargo.toml
cargo check --locked --all-targets --manifest-path components/fs-create/Cargo.toml --target-dir target/p18
cargo test --locked --manifest-path components/fs-create/Cargo.toml --target-dir target/p18 reparse -- --test-threads=1 --nocapture
cargo test --locked --manifest-path components/fs-create/Cargo.toml --target-dir target/p18 nonempty_pinned_ancestor -- --test-threads=1 --nocapture
cargo test --locked --manifest-path components/fs-create/Cargo.toml --target-dir target/p18 local_ntfs_admission -- --test-threads=1
cargo test --locked --manifest-path components/fs-create/Cargo.toml --target-dir target/p18 -- --test-threads=1
task --taskfile components/fs-create/Taskfile.yml --list
task --taskfile components/fs-create/Taskfile.yml run
git diff --check
git diff --cached --check
```

Final formatting and all-target compile pass. Focused reparse checks: four pass, with post-query file/folder 1921 evidence. Ancestor check: one pass with explicit 145 evidence. Provider admission check: one pass. Full component suite: **14 passed, zero failed, one ignored**. Task listing and run pass; run executes component fixture checks, not an application. Staged and unstaged whitespace checks pass.

Tests reserve uniquely generated owned directories under this checkout's `target/p18/fixtures`. Cleanup reads non-following metadata and never traverses a reparse entry. Sentinel targets are separate generated owned directories. Coverage includes actual empty file/folder creation, file/folder/hard-link/dangling-junction/case collisions, eight competing mixed creators with exactly one success, captured identity/pinning/replacement after Drop, real sharing violations, disappearance, zero-I/O invalid names/namespaces, Unicode/unpaired surrogates, 255-unit names, ordinary/verbatim long paths and nonprivileged junction refusal.

Initial full runs exposed false fixture assumptions: `PathBuf::join(".")` had normalized away the intended raw-dot input, and native type-mismatch collisions returned code 5 rather than always 80/183. Fixtures now construct exact dot spellings and retain failure, names, bytes and exactly-one-winner assertions while admitting actual NTFS collision codes. The real attribute-only conversion finding and failed nonempty ancestor setup are recorded above, not counted as successful prevention or converted-ancestor coverage.

The privileged file-symlink/dangling-symlink fixture check is explicitly ignored and pending. Real ACL denial, non-NTFS/remote/removable provider behavior, successfully converted ancestors, application editors/worker/queue/status/tab reconciliation and release acceptance remain pending. No personal-folder, elevation, clipboard, GUI or privileged provider probes were performed. No `clean` task was executed. `clean` targets only this checkout's generated `target/p18` output.
