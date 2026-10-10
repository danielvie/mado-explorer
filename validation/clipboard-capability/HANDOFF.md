# P22 stage 1 private clipboard capability probe

## Frozen boundary, historical stage 1

Checkout `C:/SANDBOX/WORKTREES/mado-explorer/clipboard-capability`, branch `lane/clipboard-capability`. Initial HEAD `251f7019cdd050277be360316bf2819e6bbc8126`, clean. Only `validation/clipboard-capability/**` changed; generated builds use `target/p22`. Accepted P05/P19 backend is a read-only path dependency, tree `08f208b4aec1763013af2d65344ef94837ec27a9`. No root tracker, backend or application edits. No production gate changes.

## P22-R01 filesystem retention correction

Starting HEAD `0c7bc46768c0ff7167ca4f07249d2afdb5b603be`, clean on `lane/clipboard-capability`. Root AGENTS.md and PROGRESS.md read only. This correction changes only `src/native.rs`, `run.ps1`, `Taskfile.yml`, and this handoff within `validation/clipboard-capability`. `src/main.rs`, its three original pure tests, manifests/lock, backend, original evidence and production gates are unchanged.

Review `bd0f3727` identified a directory replacement race between `create_dir` and opening the root handle. That handle does not prove identity of the created directory. All fixture deletion, disposition FFI and DELETE desired access are removed. Explicit success release and error-path Drop release handles without deleting bytes. Logs identify the retained relative path and this ownership uncertainty, including partial creation failures. Creation still uses `create_new` and refuses existing names. Preflight remains limited to trusted owned fixtures, not an adversarial ancestor or filesystem security sandbox. Native private window/station cleanup is unchanged.

Build/run/test/check/clean now target `target/p22/build`; fixtures use `target/p22/fixtures`. The clean command targets only the build directory and was NOT executed. The wrapper header records the actual build/binary paths and computes current source/binary hashes if a future authorized invocation runs it. The wrapper was parsed only, never executed here.

One real filesystem regression checks explicit successful release, an injected early error with automatic release, exclusive directory/file reopening, retained marker/Unicode file bytes, and create-new collision refusal without changing existing bytes. Deletion of the directory or either file would fail its assertions. No destructive old-code red run occurred. The first focused run failed at file creation before retention assertions, leaving an empty generated directory. Adding the missing `.write(true)` creation option corrected that existing compile-only fixture defect. No assertion was weakened. Final focused and full runs passed.

Exact successful commands from this checkout:

- `cargo fmt --manifest-path validation/clipboard-capability/Cargo.toml -- --check`
- `cargo check --locked --offline --all-targets --manifest-path validation/clipboard-capability/Cargo.toml --target-dir target/p22/build`
- `cargo test --locked --offline --manifest-path validation/clipboard-capability/Cargo.toml --target-dir target/p22/build native::tests::fixture_release_retains_success_and_failure_paths -- --exact --nocapture` passed 1.
- `cargo test --locked --offline --manifest-path validation/clipboard-capability/Cargo.toml --target-dir target/p22/build -- --nocapture` passed all 4, including the unchanged 3 pure tests.
- `task --taskfile validation/clipboard-capability/Taskfile.yml --list` and `git diff --check` passed.
- `powershell -NoProfile -Command '$tokens = $null; $errors = $null; [System.Management.Automation.Language.Parser]::ParseFile("C:/SANDBOX/WORKTREES/mado-explorer/clipboard-capability/validation/clipboard-capability/run.ps1", [ref]$tokens, [ref]$errors) | Out-Null; if ($errors.Count) { $errors; exit 1 }; "run.ps1 parse-only passed; wrapper not executed"'` passed without executing the wrapper.

Intentionally retained roots under `target/p22/fixtures/`, never manually removed:

- `fixture-10664-1791632762703068900`, initial failed creation.
- `fixture-54232-1791632783897831600` and `fixture-54232-1791632783902542300`, focused success/error cases.
- `fixture-48500-1791632783961579400` and `fixture-48500-1791632783963536800`, full-suite success/error cases.

No native station/desktop/window attempt, backend call, clipboard access, cleanup task or personal-data probe occurred. New source cannot claim a passed native runtime gate here. Stage 2 remains NOT RUN and NOT authorized. Fresh privacy-delta review is required; no station/clipboard capability, production enablement or full F13 criterion is claimed. The historical result and hashes below remain historical and unchanged.

## Actual native result, historical stage 1

One actual native setup-only attempt, 2026-10-10, Windows build 26200, own console PID 59128. `CreateWindowStationW(NULL, CWF_CREATE_ONLY=0x1, access=0x2e, explicit protected current-user+SYSTEM DACL)` returned NULL, `GetLastError=183`, `ERROR_ALREADY_EXISTS`. No station name/flags could be queried; process assignment, desktop creation and HWND creation were NOT reached. Token query and descriptor conversion succeeded. Cleanup completed with no newly created station/desktop/window handles. Backend writes=0, reads=0, clipboard calls=0. Process exit=1, task reports failure honestly.

Safe transcript `evidence/setuponly-20261010T1111086979947Z.txt` records own SID, host, exact masks/flags/raw return, source bundle SHA256 `E06F9BE154E3129C993014F463F83FDA6E13BE196A2E30C72E29220A0B41D207`, binary SHA256 `9649E6D34BCEC53FD4344096842106FAE99F012608A09C438FEED6F0E4FBA92C`, individual source hashes and exit. No native retry, named station, existing station reuse or rights escalation.

This is a new, correctly bounded attempt. It does not establish the cause/masks of historical Win32 5, or prove an unavoidable environment blocker. Current setup is blocked on the returned existing-object error. Supervisor must classify it and authorize any next change. Station isolation, desktop/window setup and clipboard capability remain unverified.

## Validation, historical stage 1

From the checkout above:

- `cargo generate-lockfile --offline --manifest-path validation/clipboard-capability/Cargo.toml` passed.
- `cargo fmt --manifest-path validation/clipboard-capability/Cargo.toml -- --check` passed.
- `cargo check --locked --offline --all-targets --manifest-path validation/clipboard-capability/Cargo.toml --target-dir target/p22` passed.
- `cargo test --locked --offline --manifest-path validation/clipboard-capability/Cargo.toml --target-dir target/p22` passed all 3 pure tests: 18 isolation-proof refusals, explicit CLI mode selection, Copy/Move Unicode/unpaired UTF-16 codec identity. No native setup or clipboard in tests.
- `task --taskfile validation/clipboard-capability/Taskfile.yml --list` passed.
- `task --taskfile validation/clipboard-capability/Taskfile.yml run` built successfully, then failed on native 183 as recorded above.
- `git diff --cached --check` and final staged delta inspected before commit.

Initial fixture-FFI compile exposed missing constants and BOOLEAN-vs-bool mismatch; fixed, final locked all-target compile passed. First task run failed before build/native launch because installed PowerShell lacked `Get-FileHash`; wrapper now uses .NET SHA256 with disposed streams. That wrapper failure was not a native attempt.

## Privacy proof and pending review

Only a fresh console child calls USER32 setup. Creation is unnamed and creation-only. No OpenWindowStation, WinSta0 fallback, hook/input/screen/exit rights, token adjustment, ACL change on existing objects or GUI startup. Descriptors are protected, non-inheritable and specify only exact station 0x2e / desktop 0x83 rights to own user and SYSTEM. Native name queries are bounded. A hidden owned HWND has a valid window procedure. Borrowed original handles exist only for child cleanup, never clipboard inspection or closing.

Immediately before each future backend entry, the same thread rechecks newly-created status, explicit security, station handle/name/nonvisible flags/current identity, desktop handle/name and live hidden HWND PID/thread/desktop. Failures stop backend admission. Cleanup destroys HWND/class before restoring contexts, and refuses closing assigned contexts. Cleanup failure remains a failure; process exit releases remaining private resources.

Stage 2 `--roundtrip` source is compiled, NOT RUN, and NOT approved. The original stage 1 source created bounded, create-new marked Unicode fixtures under target/p22 and attempted deletion through retained file/directory handles. Review `bd0f3727` found that the opened directory handle need not identify the created directory. P22-R01 above removes that deletion path entirely and retains fixtures under target/p22/fixtures. Unpaired surrogates remain codec-only. Ancestor preflight is not a hostile filesystem sandbox; stage 2 and successful native cleanup still need separate review, authorization and actual validation. Native clipboard internals are not instrumented after backend admission; future output reports backend invocation counts, never invents exact internal counts.

No personal clipboard checks, backup, sequence queries, format enumeration, Explorer/File Pilot launch, desktop capture/input, elevation, personal-data tests, clean task or stage 2 occurred. Private OS packet capability would not establish public Explorer interoperability, frontend/eframe/Narrator integration, full F13.6 or permission to clear the production gate.

## Primary sources read

- [CreateWindowStationW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationw): only administrators specify a name; NULL chooses logon-session-generated name; CWF_CREATE_ONLY refuses existing objects. Explicit security avoids documented default-descriptor ambiguity.
- [Window stations](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations): each owns clipboard/atoms/desktops; only WinSta0 is interactive.
- [SetProcessWindowStation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocesswindowstation): caller station assignment controls access to its clipboard and atoms.
- [Station rights](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights): exact clipboard, atoms, desktop creation and read-attribute rights.
- [CreateDesktopW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopw) and [desktop rights](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights): CREATEWINDOW required; no hook/switch/input rights requested.
