# P05 file-clipboard backend

## Boundary and scope

Checkout `C:/SANDBOX/WORKTREES/mado-explorer/file-clipboard`, branch `lane/file-clipboard`, baseline `36798781b4f9bf8701c54197f90e80d39c1d9cb6`.

Only `components/file-clipboard/**` source/configuration/documentation changes. Standalone Windows Rust library, not connected to the application. No root manifests, GUI, settings, sibling components, operation engine or acceptance tracker changes. No filesystem access, source-file mutations, threads, window creation, automatic reads, polling or retries.

This prepares the backend portions of F13.6/F10.1 and native immutable path snapshots for F02.4. It does not complete these criteria or establish Explorer interoperability. No acceptance/status authority exercised.

## API

- `NativePathBuf` is Windows `PathBuf`; `OsString` plus `encode_wide`/`from_wide` preserve native UTF-16 identity, including unpaired surrogates. No lossy string conversion or normalization.
- `FilePacket::snapshot(&[NativePathBuf], Intent)` clones an immutable selection. Read-only `paths()`, `intent()` and `warning()` accessors expose it. No file IDs or destination are captured.
- `FilePacket::encode_hdrop()` produces Unicode DROPFILES/CF_HDROP bytes. `encode_preferred_effect()` produces the DWORD Copy 1 or Move 2.
- `decode_packet(hdrop, preferred_effect)` validates the entire input before returning an owned packet. `decode_preferred_effect` also works independently.
- `read_clipboard()` performs one explicit global clipboard read. `write_clipboard(owner: HWND, &FilePacket)` performs one explicit replacement. Only these two functions access global clipboard contents. They must run on a caller-owned background service after the corresponding user action, never in the UI update path.
- Errors are `CodecError` or `TransportError`. Transport errors include the failing native API and Win32 code where available, publication state, and an additional close error without discarding the original failure.

The registered format name is exactly `Preferred DropEffect`. Missing intent means Copy with `MissingPreferredEffect`. Only DWORD 2 means Move. Link, mixed Copy/Move, zero and unknown flags mean Copy with `AmbiguousPreferredEffect(raw)`. Malformed effect storage is refused. Integration must surface these warnings, not silently turn them into Move. Republishing a decoded packet publishes the resolved intent, not its original ambiguous metadata.

## Untrusted input and limits

- HDROP input/native allocation and canonical Unicode packet each at most 8 MiB.
- At most 4096 paths; each at most 32767 UTF-16 units excluding NUL.
- Preferred DropEffect allocation at most 64 bytes, containing a DWORD followed only by zero allocation padding.
- Check 20-byte DROPFILES header, offset range, Unicode offset alignment, strict 0/1 fNC/fWide, even UTF-16 length, every path terminator, final list terminator, count, path length and checked allocation arithmetic.
- Empty lists, truncated lists, and nonzero data after the final terminator are rejected, never returned as a guessed prefix. Zero allocation padding is accepted.
- ANSI fWide=0 uses native ACP `MultiByteToWideChar` with `MB_ERR_INVALID_CHARS`. It never assumes UTF-8. Native conversion failures are typed errors; no replacement-byte fallback.
- Allow absolute drive paths, UNC, verbatim drive/UNC and syntactically valid verbatim Volume GUID paths. Preserve accepted representation exactly.
- Refuse relative/drive-relative/root-relative paths, shell/URL strings, device/pipe/GLOBALROOT namespaces, ADS/invalid characters, dot/parent components and DOS device names. Refuse trailing dot/space in ordinary paths; allow literal trailing dot/space in verbatim paths. These are lexical admission rules, not filesystem validation. Ordinary forward slashes are retained; verbatim forward slashes are refused.

Application-controlled buffers are bounded; this is not a total-process memory/OOM guarantee. Native path construction uses the standard library's allocation behavior. Actual component-name/filesystem limits, aliases, permissions, reparse entries, source identity and conflicts remain the operation owner's responsibility.

## Production I/O and publication safety

The transport is implemented but has not been called against the interactive clipboard. No global native probe exists. Default tests and `task run` use generated payloads and owned GlobalAlloc buffers only. The invalid-NULL-owner test returns before any global clipboard access.

Writing requires a live caller-owned, current-process HWND. NULL, invalid and foreign-process HWNDs are rejected. Caller retains the window for the entire synchronous call, including when called from a background service. No foreign or hidden window is acquired automatically. Owner validation runs before preparation and again after opening, before clearing.

Both encoded buffers are allocated with GMEM_MOVEABLE/GMEM_ZEROINIT, filled and unlocked before opening/EmptyClipboard. Open is attempted once. Clipboard open/close stay on the calling thread; the private guard is not Send. No busy loop, automatic retry, automatic reread or attempt to restore arbitrary unknown formats.

Preferred intent publishes first. Each successful SetClipboardData transfers exactly that allocation to Windows. Untransferred allocations are freed on failure. Error publication states distinguish:

- `Untouched`: EmptyClipboard was not successfully called. This is not a promise that another process retained previous contents.
- `Cleared`: previous contents removed, no format published.
- `IntentOnly`: intent published but HDROP failed; no usable file list published.
- `Complete`: both published, even if CloseClipboard subsequently fails.

Closing failures are reported. Ordinary close is attempted once; unwind cleanup closes an open guard. Native delayed rendering can block inside GetClipboardData and Windows calls have no hard deadline here. Callers must keep this work away from the UI; cancellation cannot interrupt these synchronous native calls.

Ctrl+X is metadata publication only. This library never removes, moves, copies, renames or inspects source files. The later operation owner must capture the fixed Paste destination when invoked, revalidate native source identities, deduplicate/admit sources, and perform the existing confirmation/conflict workflow. Neither clipboard packet nor Move intent authorizes destructive processing by itself.

## Checks and commands

All commands ran from `C:/SANDBOX/WORKTREES/mado-explorer/file-clipboard` with explicit `cd`. Native checks ran on Windows with Cargo 1.95.0 and Task 3.50.0. Build output is confined to ignored `target/p05`.

```text
cargo generate-lockfile --offline --manifest-path components/file-clipboard/Cargo.toml
cargo fmt --manifest-path components/file-clipboard/Cargo.toml
cargo fmt --check --manifest-path components/file-clipboard/Cargo.toml
cargo check --locked --offline --manifest-path components/file-clipboard/Cargo.toml --target-dir target/p05 --all-targets
cargo test --locked --offline --manifest-path components/file-clipboard/Cargo.toml --target-dir target/p05
cargo test --locked --offline --manifest-path components/file-clipboard/Cargo.toml --target-dir target/p05 native::tests
task --taskfile components/file-clipboard/Taskfile.yml
task --taskfile components/file-clipboard/Taskfile.yml run
task --taskfile components/file-clipboard/Taskfile.yml test
git diff --check
```

Outcomes: formatting and locked all-target compile pass. Eleven regression tests pass, zero ignored; three affected native tests also pass after strengthening owned-buffer coverage. Default task lists tasks. Safe run parses two generated paths through Windows Shell, without clipboard access. Task test passes all eleven checks. Diff whitespace check passes.

Retained tests cover Unicode, supplementary non-emoji letters, unpaired surrogates, UNC/verbatim/long/Volume GUID paths; immutable packet roundtrip; Copy/Move/missing/ambiguous intent; malformed header/offset/BOOL/termination/odd UTF-16/trailing data; count/path/aggregate size and arithmetic overflow; invalid namespaces/device aliases; native ACP; and bounded generated mutations/truncations without panics. Owned HDROP `DragQueryFileW` confirms Unicode/unpaired-surrogate/UNC/long-verbatim path identity and native parser compatibility without publishing. Owned GlobalSize tests verify allocation bounds. Injected publication failures verify every state, freeing all unsuccessful handles and retaining successful recipient-owned handles.

One initial `task run` failed because a top-level Taskfile `dir` did not change command working directory. Fixed by setting per-task `dir`; subsequent run/test pass. No repeated identical failure or ignored assertion.

The `clean` task uses `cargo clean --manifest-path components/file-clipboard/Cargo.toml --target-dir target/p05` only. It was inspected, not run, to retain build evidence.

## Pending gates

- Fresh independent review of exact candidate before promotion/enablement.
- GUI shortcut/editor gating, visible immutable selection capture, warning/error rendering and current-process HWND lifetime adapter.
- Caller-owned background-service routing, queue integration, source identity revalidation, fixed Paste destination and conflicts/cancellation/partial operations.
- Permission-gated actual Explorer copy/cut/paste interoperability, real busy/delayed-rendering/close failures, cross-process publication and cancellation/conflict manual checks. None passed or skipped as evidence. Do not enable a probe without explicit operator approval to access/replace the personal clipboard.
- No generic clipboard backup/restore guarantee, physical filesystem identity guarantee, outbound drag/drop or non-Windows support.
