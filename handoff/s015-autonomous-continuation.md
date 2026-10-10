# Autonomous implementation continuation

Human instruction relayed 2026-10-10: finish remaining approved Mado Explorer implementation, integration, fresh reviews, tests and verification autonomously. Visual approval is deferred until the user returns, not a blocker and never passed. Do not stop at another visual preparation checkpoint. Preserve existing safety/probe/history boundaries and report real blockers.

## Current baseline

Accepted application/component baseline: 86e2390 on main. New continuation policy commit: 0bbcb93. Tracked tree otherwise clean; preserve untracked .pi/workflows/project-review.js and handoff/handoff-1-rust-explorer-implementation.md. No active project workers/reviewers were present at recovery.

Canonical requirements SPEC.md; language CONTEXT.md; acceptance/evidence PROGRESS.md; scheduling LANES.md/AGENTS.md. Five shared implementation/fresh-rework writer slots, three fresh read-only reviewer slots, one supervisor-owned serial integration slot. One writer/checkout. Each lane implement -> review -> supervisor approval -> serial integration -> affected validation -> next task against updated main. No rounds, no automated approval. Use native Agent/notifications; do not poll. Global reusable workflow is optional, not currently executing the project.

Runnable root UI: S001-S010 browser, navigation, filtering/selection, OS open, independent tabs/splits/finder, plus reviewed S011 atomic workspace/bookmark/recents save/restore enabled. Root previously passed 110 checks, optional font probe ignored; owned native save/normal/minimized close/restart passed. Task run launches app. Do not mistake accepted components for app functionality.

Accepted standalone components in main:
- PDF backend, trusted Windows x64 PDFium 7881 load/usize ABI repair, secure initial DLL search. Real owned native backend/service checks passed; no binary redistribution approval.
- Selection summary, raster Fit/100%/zoom/pan, conflict decisions, operation drawer, PDF page controls, leased/coalesced one-worker PDF service.
- Read-only source identity capture and path compatibility, read-only Keep-both name planner, immutable Trash/permanent-delete confirmation intent.
All these are largely disconnected from root application; no preview/file-mutation commands enabled. App adapters, operation executors, watcher/clipboard/drop/context wiring and release validation remain missing.

## Ready work

V001 preparation candidate dfcd52153b5ecc0e1a792fd12faf4440b1e2057c, branch lane/raster-viewport, clean managed worktree C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport. Changes only validation/file-pilot/**. Parent read complete 269-line fixture generator and reran syntax/fixture ownership/determinism/retained validation. Fresh reviewer 88572e13-e7d6-4f4 found no concrete prep blocker. Original fixture retained in that worktree target/visual/v001-file-pilot-26cbf184-176d-4a38-b08d-2cd84a920c20. No apps/screenshots/comparison performed; visual acceptance pending. Promote serially preserving reviewed head/subtree, then validate setup without claiming actual comparison. Reference executable C:/Users/daniel/AppData/Local/Voidstar/FilePilot/FPilot.exe; CLI/startup restored personal locations unverified. Manual user screenshots remain valid later fallback.

Prefer next slices that connect usable root UI/backend behavior, not more disconnected controls. Explicitly scope ownership/checks/exclusions before dispatch. App/GUI integrations require their own fresh safety review when filesystem/settings behavior changes. Keep main runnable and commit accepted progress. Continue unaffected implementation despite deferred visuals.

## Real unresolved gates

- Interactive clipboard development/probe access not explicitly approved. Private noninteractive station creation failed Win32 5. Do not replace personal clipboard under generic continuation permission; do not promise universal backup/restore. Can implement code and owned-buffer tests; actual interop remains pending permission.
- Image lane beae376: pinned JPEG/WebP/PNG codec allocations not fully bounded by outer parsing; helper isolation or codec-internal admission decision required. Supervisor may resolve within approved Windows/Rust stack, no budget waiver or fake CPU bound.
- Notifications c39e273: notify 8.2.0 loses native overflow without rescan/error, retained regression fails, no service/API. Need explicit narrow provider repair/native API decision, not polling-only substitution.
- Rename 76325ff: legacy/Ex native diagnostics and hard-link collision coalescing; production rename fail-closed. Do not weaken no-overwrite/source-name contract, delete failure tests or claim preflight atomicity. Other safe operations can be implemented independently using owned fixtures and fresh review.
- Shared CPU preview limit 256 MiB, GPU preview textures 128 MiB. Service leases source reservation 2*MAX_INPUT_BYTES+16 KiB and returned Vec capacity. Native PDFium internal memory/process memory exempt; GUI conversion/staging/cache/GPU integration still unimplemented.
- Packaging/PDF notice inventory, clean-machine support, 20k-directory performance, DPI/keyboard/Narrator, remaining acceptance manual checks incomplete. Required outcomes must be recorded, not inferred from compile or a drawn control.

## Context recovery

Old supervisor context exceeds 830k/900k and has become inefficient. A compacted/fresh native supervisor should continue from these files and actual Git state. Never edit session JSONL, silently change model/execution tools, use a detached shell approval loop or discard partial work. Preserved worktrees/branches remain; rebase/squash only terminal clean approved candidates, never accepted-main history. Archived reviewed branches allow subtree equality verification.
