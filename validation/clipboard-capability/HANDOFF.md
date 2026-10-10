# P22 stage 1 private clipboard capability probe

## Frozen boundary

Checkout `C:/SANDBOX/WORKTREES/mado-explorer/clipboard-capability`, branch `lane/clipboard-capability`. Initial HEAD `251f7019cdd050277be360316bf2819e6bbc8126`, clean. Only `validation/clipboard-capability/**` changed; generated builds use `target/p22`. Accepted P05/P19 backend is a read-only path dependency, tree `08f208b4aec1763013af2d65344ef94837ec27a9`. No root tracker, backend or application edits. No production gate changes.

## Actual native result

One actual native setup-only attempt, 2026-10-10, Windows build 26200, own console PID 59128. `CreateWindowStationW(NULL, CWF_CREATE_ONLY=0x1, access=0x2e, explicit protected current-user+SYSTEM DACL)` returned NULL, `GetLastError=183`, `ERROR_ALREADY_EXISTS`. No station name/flags could be queried; process assignment, desktop creation and HWND creation were NOT reached. Token query and descriptor conversion succeeded. Cleanup completed with no newly created station/desktop/window handles. Backend writes=0, reads=0, clipboard calls=0. Process exit=1, task reports failure honestly.

Safe transcript `evidence/setuponly-20261010T1111086979947Z.txt` records own SID, host, exact masks/flags/raw return, source bundle SHA256 `E06F9BE154E3129C993014F463F83FDA6E13BE196A2E30C72E29220A0B41D207`, binary SHA256 `9649E6D34BCEC53FD4344096842106FAE99F012608A09C438FEED6F0E4FBA92C`, individual source hashes and exit. No native retry, named station, existing station reuse or rights escalation.

This is a new, correctly bounded attempt. It does not establish the cause/masks of historical Win32 5, or prove an unavoidable environment blocker. Current setup is blocked on the returned existing-object error. Supervisor must classify it and authorize any next change. Station isolation, desktop/window setup and clipboard capability remain unverified.

## Validation

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

Stage 2 `--roundtrip` source is compiled, NOT RUN, and NOT approved. It creates bounded, create-new marked Unicode fixtures only under owned target/p22 and tests Copy+Move packets. Unpaired surrogates remain codec-only. Fixture cleanup uses retained original file/directory handles, not reopened paths or recursive removal. Failed runs retain generated leftovers. Ancestor preflight is not a hostile filesystem sandbox; stage 2 fixture lifecycle and successful native cleanup need review and actual validation. Native clipboard internals are not instrumented after backend admission; future output reports backend invocation counts, never invents exact internal counts.

No personal clipboard checks, backup, sequence queries, format enumeration, Explorer/File Pilot launch, desktop capture/input, elevation, personal-data tests, clean task or stage 2 occurred. Private OS packet capability would not establish public Explorer interoperability, frontend/eframe/Narrator integration, full F13.6 or permission to clear the production gate.

## Primary sources read

- [CreateWindowStationW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationw): only administrators specify a name; NULL chooses logon-session-generated name; CWF_CREATE_ONLY refuses existing objects. Explicit security avoids documented default-descriptor ambiguity.
- [Window stations](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations): each owns clipboard/atoms/desktops; only WinSta0 is interactive.
- [SetProcessWindowStation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocesswindowstation): caller station assignment controls access to its clipboard and atoms.
- [Station rights](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights): exact clipboard, atoms, desktop creation and read-attribute rights.
- [CreateDesktopW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopw) and [desktop rights](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights): CREATEWINDOW required; no hook/switch/input rights requested.
