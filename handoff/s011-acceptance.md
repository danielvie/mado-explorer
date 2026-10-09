# S011 settings implementation acceptance

Supervisor accepted the recovered S011 implementation after the whole-backend review and fresh S012-R01 UNC-override correction review found no concrete blockers. The latter matched settings.rs 034bb7c12787b316d9bdf3d1274b021b12ffcbbf and the frozen test/adapter hashes recorded in PROGRESS.md. The supervisor subsequently enabled production saves and retained explicit disabled-save coverage; enabled-close coverage uses the actual production gate.

Targets F05.6/F13.4/F13.5 are Implemented, not Verified. Native bookmark interactions, corrupt-file/save-error/retry interaction, mixed DPI, disconnected monitors and Narrator remain pending. Later checkpoint clipboard gates remain unchanged. No source-file mutation commands are enabled.

Checks from C:/SANDBOX/REPOS/mado-explorer.git:

- cargo fmt --check, cargo check --locked, cargo test s011: pass; fourteen focused tests, no failures or ignored checks.
- cargo build --locked: pass.
- task test: 110 Rust checks pass, zero fail, one explicitly optional installed-font probe ignored; workflow ownership/capacity/review-head checks pass.
- pwsh -NoProfile -ExecutionPolicy Bypass -File handoff/s011-native-smoke.ps1 -ExpectWrites: own-window Unicode/missing-tab restore, final save and normal close pass.
- Same command with -MinimizedClose: maximize/restore/minimized-close preserves normal bounds; final save and cold restart from application-written settings pass. The retained helper now includes that restart check.
- git diff --check: pass; existing LF/CRLF conversion warnings only.

Native checks created GUID-named disposable directories, supplied validated MADO_SETTINGS_PATH overrides, queried only their own window, waited for owned processes and removed only generated fixtures. No personal settings, global clipboard, foreground capture, elevation or Narrator probe. Original worker evidence is archived in handoff/s011-settings-original.md; correction evidence in handoff/s012-r01-settings.md.
