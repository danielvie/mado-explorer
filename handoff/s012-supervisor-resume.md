# S012 supervisor checkpoint

## Accepted main

- d5a4681 is current main. Tracked working tree is clean.
- 5aae419 records the approved topology: three feature workers, two fresh fix slots, one separate serial integration lane. Five applies to feature/fix workers only. Parent alone accepts/integrates; read-only reviewers are separate. Ready integration backlog cap three.
- 54136b3 accepts reviewed S011 workspace/settings persistence with production saving enabled. Fourteen focused tests and full root suite pass: 110 passing Rust tests, one optional font probe ignored; workflow guard check passes. Disposable native normal/minimized close, final save, normal geometry and cold restart from application-written settings pass. No personal settings or interactive clipboard development access.
- 78eefc3 promotes the reviewed standalone PDF backend without GUI enablement. Reviewed f4d4d6c preserved at archive/p03b-reviewed; component subtree 9ec7aac20df6dab05e6c1a0d028e2c5d33ccaceb stayed identical through rebase/squash. Nine backend checks pass on main. task test now includes test:pdf and workflow:test.
- F05.6/F13.4/F13.5 are Implemented, not Verified. Broader native/manual checks remain pending. PROGRESS.md and LANES.md are canonical; the older handoff-1 is historical.

## Work awaiting notifications

- Workflow wf_82fe0bbdd7ff, rolling-feature-wave: P06 selection-summary, P07 raster-viewport, P08 conflict-dialog. Three independent worktrees under C:/SANDBOX/WORKTREES/mado-explorer, component-only writes. Baseline 3679878. Each terminal candidate gets an immediate fresh read-only review; workflow never accepts/promotes/enables. No final workflow result received yet. Do not assume all workers are still implementing, or restart/overlap their checkouts.
- Running workflow source/journal are under C:/Users/daniel/AppData/Local/Temp/pi-subagents-0/SANDBOX-REPOS-mado-explorer.git/01a121a0-76a2-74e0-b550-d6747768b809/tasks/wf_82fe0bbdd7ff.workflow.js and adjacent .workflow.jsonl. Saved future-use script .pi/workflows/rolling-feature-wave.js adds tested capacity/checkout/review-head guards; it is not an instruction to restart the healthy current run.
- Clipboard correction 8ae99adb51ce39c06d7b0c3208c6e0d3161a88ae on lane/file-clipboard: four-line UNC pipe-share rejection plus snapshot/raw UTF-16/native-ACP tests. Parent inspected full correction and reran fmt/locked compile/fifteen passing tests. Fresh reviewer bb4b5d89-c0c1-488, handle clipboard-fix-review, is awaiting completion notification. Do not promote until that result is consumed. Original whole-backend review found this namespace blocker, no other concrete defect. No clipboard I/O permitted.

## Next supervisor actions

1. Compact native Pi context before further large reads. Context last measured about 220k input tokens with no compaction, approaching the previous failed session's roughly 329k. Do not switch model/tools or edit session JSONL.
2. Consume completion notifications, retrieve actual results and inspect changed code/heads. Classify concrete findings, assign fresh fix workers as needed, keep unrelated ready work moving. Do not poll or fabricate statuses.
3. Before a clean-lane rebase, transfer then remove only the supervisor-created commission note at the top of that lane's PROGRESS.md. These intentional notes were left unstaged by workers. Preserve original reviewed heads, verify identical component subtree or review changed integration, rebase/squash only terminal clean accepted branches, then fast-forward main serially.
4. Do not mark backend/control preparation as complete GUI criteria. Record and implement each production adapter after its prerequisites; do not enable later-checkpoint GUI features while required checkpoint probes are unresolved.

## Gates and preserved work

- Interactive clipboard development probe needs explicit operator permission; private noninteractive station creation failed Win32 5. Never replace personal clipboard as an unapproved test or promise generic backup/restore.
- Image beae376 remains unpromoted: codec allocation bounds need a helper-isolation or codec-internal decision. No budget waiver authorized.
- Filesystem 76325ff remains unpromoted: native hard-link collision issue, production rename fail-closed. Preserve diagnostics and source-name safety tests; do not infer a passing primitive or weaken semantics.
- Notifications c39e273 is dependency diagnosis only: notify 8.2.0 loses overflow without rescan/error; retained test fails, no service/API. Provider decision precedes fresh implementation.
- Untracked .pi/workflows/project-review.js and handoff/handoff-1-rust-explorer-implementation.md are preserved prior-session artifacts. No destructive reset, branch/worktree cleanup, force push, elevation, personal-folder mutation or CLI/model fallback.
