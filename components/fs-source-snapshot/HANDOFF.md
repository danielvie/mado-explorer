# P12 read-only native source snapshots

## Candidate and boundary

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`.
- Branch: `lane/selection-summary`.
- Clean starting baseline: `c8b162e8bcedc60aa8e00c696be884bc7aa34e61`.
- Exclusive changes: `components/fs-source-snapshot/**`. No root/shared/sibling edits, production integration, enablement, acceptance-status changes or P02 rename changes.
- Target: native-source-identity backend portion of F02.4 only. Not whole-criterion implementation or acceptance.
- Terminal candidate SHA is reported with the worker result and can be obtained with `git rev-parse HEAD`. Fresh exact-head safety review and supervisor approval are required before promotion. The worker does not self-approve or select another task.

## API and observation boundaries

Windows-only standalone library, using installed `windows-sys = 0.61.2` and Rust's `OwnedHandle`.

- `Snapshot::capture(PathBuf)` consumes and retains the exact native original path. No display conversion, canonicalization, case folding, separator replacement or native-path reconstruction occurs.
- Read-only getters: `original_path() -> &Path`, `identity() -> Identity`, `kind() -> EntryKind`. Snapshot fields and its handle are private; it is not cloneable or serializable. No raw handle escapes the public API.
- Identity is the native `FILE_ID_INFO` volume serial and all 128 file-ID bits. Identity getters expose those values, not mutation rights. No path/hash/text-derived fallback exists.
- Kind comes from handle `FILE_ATTRIBUTE_TAG_INFO`: file, directory, reparse point with tag, or other. A final reparse entry wins over its directory attribute. Capture does not fetch reparse target data or read its target.
- `revalidate_original_name()` admits and opens the original name again. It returns `Ok(())` only for equal native volume/file ID and kind. Errors distinguish invalid native path, missing name, replaced object/kind, unsupported native metadata and I/O failure.
- The original handle remains owned across rename, unlink and replacement. Revalidation does not replace the captured identity or retarget the snapshot. Temporary revalidation handles and error-path handles close through RAII.

This is an observation, not destructive authorization. It captures no contents, sizes, times or ancestor identity. Metadata requests themselves are separate observations, not a transactional metadata snapshot. Ancestor reparse points may resolve normally; lexical admission is not ancestor pinning or a guarantee about drive mappings/providers. Reparse data can change on the same object. Contents may change without changing identity. Volume serial/file ID semantics depend on the native provider.

Names can change immediately after revalidation. A future executor must bind/revalidate handles through its actual commit and separately enforce destination, collision, permission, root/protected-location and link policies. This helper does not close a check-then-use race, prove mutation atomicity, or make the blocked P02 rename primitive acceptable. Keep existing mutation gates closed.

Calls are synchronous metadata I/O. No worker/thread/UI service is added. A future application adapter must invoke them off the UI thread and handle slow/unavailable providers; no cancellation or latency guarantee is claimed.

## Rights, sharing and FFI

Every production open uses `CreateFileW` with:

- `FILE_READ_ATTRIBUTES` only. No content-read, write, DELETE, create or truncate access is requested.
- `OPEN_EXISTING`.
- `FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE`. This grants no lock or concurrent-writer consistency.
- `FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS`. The first prevents following the final entry; the second permits directory handles. No backup privilege, elevation or permission bypass is enabled.
- Null security attributes and template handle.

`GetFileType` requires `FILE_TYPE_DISK` before metadata queries. Unknown type with a native error remains an I/O error; non-disk handles are unsupported. Explicit device/pipe names are rejected lexically before even this open.

`GetFileInformationByHandleEx` uses aligned typed `MaybeUninit<FILE_ID_INFO>` and `MaybeUninit<FILE_ATTRIBUTE_TAG_INFO>` buffers with exact native sizes. Values are used only after successful calls. Provider unsupported errors are explicit; an all-zero file ID is refused. `OwnedHandle::from_raw_handle` is called once per successful fresh open; no duplicate ownership or custom close wrapper exists. Production raw handles are used only inside the native calls while their owner is live.

## Namespace admission

Admission examines native UTF-16 before any I/O and appends only the required terminating NUL after validation. Unpaired UTF-16 units remain lossless.

Admitted exact namespace shapes:

- Absolute drive paths, including drive roots.
- Ordinary UNC server/share paths and share roots.
- Verbatim absolute drive paths.
- Verbatim UNC server/share paths, with case-insensitive ASCII `UNC` recognition.
- Verbatim `Volume{GUID}\\` roots and descendants with a strict GUID spelling.

No namespace is guessed or rewritten. Unsupported spellings fail with typed `PathRejection`, even if a platform could interpret them. Conservative limits are 259 UTF-16 units for ordinary paths, 32766 for verbatim paths, and 255 per named component. Ordinary long paths must be supplied in an admitted native verbatim form; capture never adds a prefix.

Rejections include NUL; relative, drive-relative and root-relative paths; NT-device, Win32 device, GLOBALROOT, pipe and unknown verbatim prefixes; forward slashes; dot/parent/empty components; trailing dots/spaces; trailing separators on non-root entries; controls, wildcard/device punctuation and colon/alternate-stream components; reserved DOS aliases, including extensions, case variants, spaces before extensions, CONIN$/CONOUT$, and COM/LPT superscript aliases. UNC server/share components use the same strict checks. A `pipe` share is rejected in any ASCII case in both ordinary and verbatim UNC forms. Trailing-dot/space/prefix aliases fail instead of being normalized.

The lexical tests call the real capture entry point with rejected names and prove zero calls to the sole production open helper. Valid UNC/volume spellings are checked only lexically; no server, drive root, device or real volume is probed.

## Fixture safety and evidence

Built-in Rust tests only. Each native test creates a unique disposable temporary root with `create_dir`, never adopts an existing directory, and installs a `create_new` ownership marker. Mutation helpers check the marker and restrict paths to that owned root; cleanup refuses a replaced/reparse root. The only content reads are these generated ownership markers. Source file contents are never read, including in the same-bytes replacement test; both files receive the same generated literal.

Default automated evidence:

- File and directory IDs/kinds remain unchanged at their original names.
- Rename plus same-bytes replacement yields a different object ID, rejects revalidation, and preserves the captured original ID.
- Missing file/directory names return typed missing.
- Hard-link aliases share native identity but preserve distinct original native paths and independent name revalidation.
- Japanese, accented, supplementary non-emoji Unicode and unpaired UTF-16 filenames round-trip natively.
- A generated verbatim path longer than 260 UTF-16 units captures/revalidates without truncation.
- Invalid namespace/name paths perform zero opens. Supported namespace spellings retain exact units.
- Native owned-handle lifetime is verified with test-only private `GetHandleInformation` before and immediately after Drop. The documented suite runs serially to prevent unrelated fixture-handle reuse during this close probe. No production raw-handle access API was added.
- Attribute-only opens make no exclusive-lock claim. I/O and unsupported-provider error classification checks retain the native error codes where applicable.
- A nonprivileged owned junction captures the leaf's ID/tag, not its target ID, and still captures/revalidates after its owned empty target is removed. Removing the owned junction reports its original name missing. Independent target-root ownership and its generated sibling sentinel remain intact.

Test-only `DeviceIoControl(FSCTL_SET_REPARSE_POINT)` creates the junction on an owned empty fixture directory. Its write-access handle and bounded aligned reparse buffer exist only in `#[cfg(test)]` code. Extra windows-sys IO/Ioctl features are dev-dependencies. No elevation, process/shell helper, privileged symlink attempt or real-path probe is used.

Pending: privileged symlink check is explicitly ignored and requires separate authorization; no privilege bypass. Real unsupported-ID providers, live UNC/volume roots, denied ACLs, unavailable devices, and concurrent filesystem stress were not probed. Synthetic error classification is not native provider/ACL evidence.

## Commands and results

Run from `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`:

```text
cargo generate-lockfile --manifest-path components/fs-source-snapshot/Cargo.toml --offline
cargo fmt --manifest-path components/fs-source-snapshot/Cargo.toml --check
cargo check --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline --all-targets
cargo test --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline -- --test-threads=1
task -t components/fs-source-snapshot/Taskfile.yml --list
task -t components/fs-source-snapshot/Taskfile.yml run
git diff --cached --check
```

Lock generation uses only the two already-installed packages, windows-sys/windows-link. Formatting and all-target compilation pass. The final focused suite passes 12 tests, with one explicitly ignored privileged-symlink test and zero failures. Safe task run passes its one generated file/directory self-check; task listing passes. Diff whitespace check passes. `task clean` is restricted to the worktree's generated `target/p12`; it was not executed. No root build/test/Clippy was run.

The first coherent suite run passed 9 tests, failed 2, and ignored 1. The two failures came from an incorrect fixture assumption that attribute-only opens participate in exclusive sharing locks. Production rights were not broadened. The lifetime test now directly proves native closure, error classification retains explicit checks, and a new native test documents the no-lock behavior. After this one correction, all 12 default checks pass. No failing behavior was marked passed or permission/provider evidence invented.
