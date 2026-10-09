# S012-R01 settings override guard

Ready for fresh review. Target F13.5; no acceptance/status update or production enablement.

## Changed paths

- `C:/SANDBOX/REPOS/mado-explorer.git/src/settings.rs`
- `C:/SANDBOX/REPOS/mado-explorer.git/src/settings_tests.rs`
- `C:/SANDBOX/REPOS/mado-explorer.git/handoff/s012-r01-settings.md`

Existing uncommitted S011 work retained. No main.rs, settings_workspace.rs, manifest, root-document or component edits; no commits or delegation. Production settings writes remain disabled in settings_workspace.rs, also asserted by the passing S011 close test.

## Change

`override_path` now inspects `Prefix::UNC` and `Prefix::VerbatimUNC` before returning an admitted path. It validates server/share names and rejects the share `pipe` using an ASCII case-insensitive, non-lossy comparison. Prefix names reject empty names, trailing dots/spaces, controls, forbidden characters and overlong components. This covers ambiguous names rather than trimming them into an accepted path.

The existing normal-component validator moved into a settings-only helper. DOS device-stem checks still apply to normal components, but not to UNC server/share names, where names such as con/NUL can identify filesystem shares. Device namespace admission and basename/absolute-path checks remain unchanged. NativePath restoration and browsing-domain paths are untouched. Accepted paths are returned with their original OsStr identity.

Inspected installed Rust 1.95 sources at `C:/Users/daniel/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/std/src/sys/path/windows_prefix.rs` and `C:/Users/daniel/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/std/src/sys/path/windows.rs`. Ordinary UNC parsing recognizes both separators; verbatim names retain forward slashes inside names. Server/share belong to the prefix and never reach the old Normal-component loop. Native file opening can pass short absolute paths straight to Win32, so validation must precede opening rather than rely on Rust normalization.

## Regression coverage

Added one Windows-only, entirely lexical test, with no fixture creation or filesystem/network calls:

- Ordinary/verbatim UNC pipe shares, local/remote server spellings and mixed case.
- Forward/mixed separators, trailing dot/space aliases, ambiguous dot-server names.
- Invalid prefix controls, colon/stream forms, embedded verbatim forward slashes and dot-dot prefix names.
- Ordinary/verbatim filesystem UNC, forward-separator UNC, pipe-like filesystem shares, a server named pipe, DOS-like host/share names and local disk paths remain accepted unchanged.
- Unicode and unpaired UTF-16 units in ordinary/verbatim server/share names preserve exact identity.

Existing S011 device/DOS-name, absolute-path, basename, native-path, atomic-save and production-gate checks passed unchanged.

## Commands and outcomes

Working directory for every command: `C:/SANDBOX/REPOS/mado-explorer.git`. Existing main target used.

1. Before the implementation fix: `cargo test s011_settings_override_unc_namespace_guard_is_lexical_and_lossless`
   - Expected regression failure, exit 101. One test failed at settings_tests.rs:135 because `\\localhost\pipe\workspace.json` was admitted. No I/O was attempted on that path.
2. After the coherent fix: `rustfmt --edition 2024 src/settings.rs src/settings_tests.rs && cargo fmt --check && cargo check --locked && cargo test s011 && git diff --check`
   - Exit 0. Formatting passed; locked incremental compilation passed; 14 S011 tests passed, 0 failed/ignored, 97 filtered out; tracked diff whitespace check passed. Git emitted only existing LF/CRLF conversion warnings.
3. Because the two pre-existing S011 source files are untracked, additionally inspected their changed regions and ran `git diff --no-index --check -- /dev/null src/settings.rs` and `git diff --no-index --check -- /dev/null src/settings_tests.rs`.
   - Both emitted only LF/CRLF warnings, no whitespace defects. Both returned 1 for the new-file differences. The first was in an && chain, so the second and status command were subsequently run separately.
4. `git status --short`
   - Same pre-existing modified/untracked path set as at entry; the handoff is inside the already-untracked handoff directory.

## Limits

No app launch, personal-settings access, global clipboard, elevation, real named-pipe/UNC I/O, full suite or Clippy. Existing S011 tests use generated disposable directories; the new guard test performs no I/O. This fixes lexical override admission, not general network-provider latency, reparse resolution or startup-read threading. Independent review remains required before enabling production writes.
