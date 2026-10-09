# P12/P16 read-only native source snapshots

## P16 compatibility correction

- Checkout and branch remain `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`, `lane/selection-summary`. Clean P16 starting baseline: `37e9a32b2f48173f2260ef5a0fd5d4f9a0f15e73`.
- P16 exclusive changes: `components/fs-source-snapshot/src/lib.rs`, `src/tests.rs` and this `HANDOFF.md`. No root/shared/sibling/dependency/Taskfile edits. Existing standalone Taskfile remains current.
- Targets: supported selected-path representations in F01.6/F02.4 backend only. No whole-criterion acceptance, production adapter/enablement or destructive capability.
- The public `Snapshot.original_path` remains the exact caller-owned `PathBuf`/`OsString`. Only a private, validated, terminated UTF-16 Win32 opening buffer changes. Revalidation repeats admission/conversion from that same untouched original. No canonicalization, case folding, display conversion, public path reconstruction or path-derived identity occurs.
- Ordinary absolute drive/UNC paths now accept backslash, forward and mixed separators. After exact lexical admission, the opening buffer replaces only ordinary separators with backslashes. Above 259 original UTF-16 units, drive names receive `\\?\` and UNC names replace their two leading separators with `\\?\UNC\`. No prefix is added to short or already-verbatim names.
- The internal buffer, including prefix and terminating NUL, must fit 32767 UTF-16 units. Maximum original lengths are 32762 for long ordinary drive paths, 32760 for long ordinary UNC and 32766 for verbatim forms. Components still cap at 255 units. Oversized forms return typed `InvalidPath(Length)` before any open, with no truncation.
- UNC server/share validation now skips only the local DOS-device-stem check. DOS-like hosts/shares such as `NUL`, `COM1`, `CONIN$` and `CONOUT$` are lexical positives. Empty/dot/parent/trailing-dot/space/control/invalid-punctuation aliases and case-insensitive `pipe` shares remain rejected. Descendant components retain the full DOS-device check.
- Verbatim names retain explicit literal rules: any forward slash remains unsupported, not interpreted as a separator. The existing Volume GUID guard is unchanged. Relative roots, NUL, device/pipe/GLOBALROOT namespaces, ADS, unsafe normalization aliases and DOS-like local/descendant basenames still fail before I/O. This is not blind prefixing of arbitrary Windows paths.
- Production rights, RAII, sharing, final-leaf nonfollowing, error classification and `FILE_ID_INFO` comparison are unchanged. This is still an observation, not ancestor pinning, content consistency or TOCTOU-safe mutation authority.

### P16 regression evidence

Four new tests failed before the production correction:

- `ordinary_separator_spellings_capture_and_revalidate_same_file_and_directory`: `InvalidPath(UnsupportedSpelling)` on an owned ordinary forward spelling.
- `ordinary_long_spellings_open_without_truncating_original`: `InvalidPath(Length)` on an owned ordinary long spelling.
- `ordinary_buffers_normalize_only_separators_and_unc_root_dos_stems_are_supported`: `InvalidPath(UnsupportedSpelling)` first, then a separate exact UNC DOS-host/share run produced `InvalidPath(DeviceAlias)` after placing that lexical case first.
- `opening_prefix_and_nul_are_included_in_utf16_bounds`: `InvalidPath(Length)` on the newly supported 260-unit ordinary path.

All four pass after correction. Native checks compare actual volume/file IDs and kinds for the same owned file/directory across ordinary backslash/forward/mixed and verbatim spellings. They compare complete original UTF-16 units, including Japanese/accented/supplementary Unicode and unpaired surrogates. The ordinary long fixture also verifies missing then same-name replacement through the unchanged forward original. Owned junction checks now repeat those spellings both before and after removing the target, proving final-leaf nonfollowing remains intact.

All twelve P12 default behavior checks remain. Only the obsolete `C:/x` negative was moved to a supported lexical positive; no safety assertion was removed. Added mixed/forward and verbatim-literal alias/endpoint negatives prove zero calls to the production open helper. Supported UNC DOS-like names, long UNC conversion, exact prefix/NUL boundaries, and one-unit-over-limit refusals are lexical only. No real UNC, drive/volume root or device was opened.

### Installed API evidence

Installed definitions inspected locally, without network/probes:

- Rust 1.95.0, host `x86_64-pc-windows-msvc`, sysroot `C:/Users/daniel/.rustup/toolchains/stable-x86_64-pc-windows-msvc`. Installed `lib/rustlib/src/rust/library/std/src/sys/path/windows.rs` distinguishes ordinary `/`/`\` from verbatim `\`, describes the 260-unit legacy limit including NUL, and converts drive/UNC prefixes only after obtaining an absolute normalized spelling. Its `GetFullPathNameW` implementation is not called here; exact lexical admission avoids silently admitting its normalization aliases.
- Installed `std/src/os/windows/ffi.rs` specifies lossless `OsStringExt::from_wide`/`OsStrExt::encode_wide` roundtrips, including ill-formed UTF-16, without an appended terminator. Installed `std/src/path.rs` documents normalization in components/equality and the literal verbatim slash rule. Neither normalized Path equality nor text is used as identity.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows-sys-0.61.2/src/Windows/Win32/Storage/FileSystem/mod.rs` defines the typed `CreateFileW`, `GetFileInformationByHandleEx`, `FILE_ID_INFO`, metadata rights and nonfollowing flag used unchanged.
- Installed SDK `C:/Program Files (x86)/Windows Kits/10/Include/10.0.26100.0/um/fileapi.h` defines `CreateFileW` with `LPCWSTR`/DWORD arguments. `um/winbase.h` defines `GetFileInformationByHandleEx`, `FILE_ATTRIBUTE_TAG_INFO` and the 64-bit volume serial plus `FILE_ID_128`. No native ABI/rights changes were needed.

### P16 commands and results

Run in the assigned worktree:

```text
rustc -Vv
rustc --print sysroot
rustup component list --installed
cargo test --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline ordinary -- --test-threads=1
cargo test --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline ordinary_buffers_normalize_only_separators_and_unc_root_dos_stems_are_supported -- --test-threads=1
cargo test --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline opening_prefix_and_nul_are_included_in_utf16_bounds -- --test-threads=1
cargo fmt --manifest-path components/fs-source-snapshot/Cargo.toml
cargo fmt --manifest-path components/fs-source-snapshot/Cargo.toml --check
cargo check --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline --all-targets
cargo test --manifest-path components/fs-source-snapshot/Cargo.toml --target-dir target/p12 --locked --offline -- --test-threads=1
git diff --check
```

Red commands exited 101 for the errors listed above. Green focused runs pass three ordinary tests and one bounds test. Final formatting, locked/offline all-target compilation, serial full component suite and diff checks pass: sixteen passing tests, zero failures, one explicitly ignored privileged-symlink check. No dependency download, root suite, GUI, clipboard, source-content read, personal/network/device/volume probe, elevation or production mutation was performed. All test mutations remain generated owned fixtures; ownership-marker reads are unchanged from P12.

Limits remain: no live UNC/provider/ACL evidence, privileged symlink pending, no guarantee every filesystem supports native identity or the lexical maximum length. Typed provider/I/O errors remain authoritative. Fresh exact-terminal-head safety review is required before supervisor approval and serial parent integration/validation. Do not start a sibling task or self-promote.

## Original P12 candidate and boundary

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`.
- Branch: `lane/selection-summary`.
- Clean starting baseline: `c8b162e8bcedc60aa8e00c696be884bc7aa34e61`.
- Exclusive changes: `components/fs-source-snapshot/**`. No root/shared/sibling edits, production integration, enablement, acceptance-status changes or P02 rename changes.
- Target: native-source-identity backend portion of F02.4 only. Not whole-criterion implementation or acceptance.
- Terminal candidate SHA is reported with the worker result and can be obtained with `git rev-parse HEAD`. Fresh exact-head safety review and supervisor approval are required before promotion. The worker does not self-approve or select another task.

## API and observation boundaries

Windows-only standalone library, using installed `windows-sys = 0.61.2` and Rust's `OwnedHandle`.

- `Snapshot::capture(PathBuf)` consumes and retains the exact native original path. No display conversion, canonicalization, case folding or public native-path reconstruction occurs. P16 changes only the private Win32 opening spelling described above.
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

Admission examines exact native UTF-16 before any I/O. P16 then constructs the private opening buffer as described above and appends the required terminating NUL. Unpaired UTF-16 units remain lossless.

Admitted exact namespace shapes:

- Absolute drive paths, including drive roots.
- Ordinary UNC server/share paths and share roots.
- Verbatim absolute drive paths.
- Verbatim UNC server/share paths, with case-insensitive ASCII `UNC` recognition.
- Verbatim `Volume{GUID}\\` roots and descendants with a strict GUID spelling.

Unsupported spellings fail with typed `PathRejection`, even if a platform could interpret them. P16 supersedes the original 259-unit ordinary-path limit with validated internal prefix conversion and prefix-inclusive bounds. No unknown namespace is guessed. The public original is never rewritten.

Rejections include NUL; relative, drive-relative and root-relative paths; NT-device, Win32 device, GLOBALROOT, pipe and unknown verbatim prefixes; forward slashes in verbatim names; dot/parent/empty components; trailing dots/spaces; trailing separators on non-root entries; controls, wildcard/device punctuation and colon/alternate-stream components; reserved DOS aliases in local/descendant basenames, including extensions, case variants, spaces before extensions, CONIN$/CONOUT$, and COM/LPT superscript aliases. UNC server/share components skip only that DOS-stem restriction. A `pipe` share is rejected in any ASCII case in both ordinary and verbatim UNC forms. Trailing-dot/space/prefix aliases fail instead of being normalized.

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
- Invalid namespace/name paths perform zero opens. The public original retains exact units; P16 internal-buffer checks distinguish unchanged verbatim from validated ordinary separator/prefix conversion.
- Native owned-handle lifetime is verified with test-only private `GetHandleInformation` before and immediately after Drop. The documented suite runs serially to prevent unrelated fixture-handle reuse during this close probe. No production raw-handle access API was added.
- Attribute-only opens make no exclusive-lock claim. I/O and unsupported-provider error classification checks retain the native error codes where applicable.
- A nonprivileged owned junction captures the leaf's ID/tag, not its target ID, and still captures/revalidates after its owned empty target is removed. Removing the owned junction reports its original name missing. Independent target-root ownership and its generated sibling sentinel remain intact.

Test-only `DeviceIoControl(FSCTL_SET_REPARSE_POINT)` creates the junction on an owned empty fixture directory. Its write-access handle and bounded aligned reparse buffer exist only in `#[cfg(test)]` code. Extra windows-sys IO/Ioctl features are dev-dependencies. No elevation, process/shell helper, privileged symlink attempt or real-path probe is used.

Pending: privileged symlink check is explicitly ignored and requires separate authorization; no privilege bypass. Real unsupported-ID providers, live UNC/volume roots, denied ACLs, unavailable devices, and concurrent filesystem stress were not probed. Synthetic error classification is not native provider/ACL evidence.

## Original P12 commands and results

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
