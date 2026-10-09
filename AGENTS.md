# Implementation loop

## Project contract

Implement the first release in SPEC.md using its existing acceptance IDs and the vocabulary in CONTEXT.md. Use the Windows-first, Rust/eframe/egui/wgpu baseline. The File Pilot-inspired design files are references, not an application implementation.

Do not shrink scope, change framework, or mark unavailable manual checks as passed to make progress. Bring consequential product/platform decisions to the supervisor. Never perform destructive development checks on personal folders.

## Roles and ownership

- Supervisor: select the next slice, assign its exclusive write boundary, classify blockers/review findings, inspect evidence, update acceptance status, and commit accepted progress.
- Worker: implement one usable behavior end-to-end, add the smallest useful regression check for non-trivial logic, run focused validation, and report limitations. No nested delegation or commits unless explicitly granted.
- Reviewer: read-only, fresh-context review of risky filesystem changes and milestone diffs. Report concrete correctness/security/data-loss findings with source evidence; style preferences and speculative improvements are not blockers.
- Scout/researcher: optional bounded discovery or external API/licensing work. Oracle: optional escalation for a consequential tradeoff or persistent disagreement.

Keep one writer per checkout. Initially work serially. Later parallel writers require independent ownership and isolated worktrees; integration stays supervisor-owned.

## Slice loop

1. Read SPEC.md, CONTEXT.md, and PROGRESS.md. Pick one ready behavior, normally a 20-45 minute slice. Time estimates are guidance, not automatic termination rules.
2. Record a slice ID, target acceptance IDs, write boundary, one useful check, and exclusions in PROGRESS.md before launch.
3. Implement the behavior through its UI/state/backend path as applicable. Do not build unrelated controls, introduce speculative abstractions, or refactor neighboring code.
4. Run checks after a coherent change, not after each edit. Inspect the final diff and report exact commands, outcomes, satisfied IDs, partial IDs, and remaining risks.
5. The supervisor consumes the result, resolves concrete findings, and commits only accepted, runnable progress. Preserve unfinished work rather than discarding it.
6. Update PROGRESS.md and immediately select the next ready slice. Continue across milestones until all required functionality is implemented, or a genuine decision/infrastructure blocker requires attention.

The native Pi supervisor owns continuation. PROGRESS.md makes the loop recoverable after interruption; no detached shell process or unattended script may approve changes or bypass safety review.

## Check policy

- Ordinary slices: formatting check, incremental compile, and one focused check for changed non-trivial behavior. A meaningful visual/manual check can cover simple UI wiring when available; record unavailable UI checks as pending.
- State/navigation/filter logic: a focused runnable regression check. Do not demand a whole UI automation framework.
- Filesystem mutations and settings replacement: focused safety tests covering the changed behavior and independent review before enabling the behavior on personal data.
- Milestones: full existing automated suite and a short end-to-end smoke check. A missing manual check does not block unrelated implementation, but the milestone is not fully verified until its required checks are completed.
- Release candidate: outstanding acceptance checks, packaging, accessibility, and performance measurements.

No coverage-percentage gate. No full suite, Clippy, screenshot pass, or benchmark after every edit. Rerun only affected checks after a fix. Bootstrap dependency downloads and initial compilation are not recurring iteration overhead; do not repeatedly restart a healthy build.

A compiling application is not proof that a feature works. A test failure caused by the current change cannot be called a passing check. Safety violations, data loss, security issues, and known corruption are hard blockers; the affected behavior remains disabled until resolved.

## Blocker policy

After two unsuccessful focused fix attempts for the same failure, report the exact failure and evidence to the supervisor. The supervisor classifies current-code bug, pre-existing failure/flake, environment/tooling failure, or missing decision, then chooses a smaller slice, dependency fix, or unrelated ready work. This is a diagnosis boundary, not permission to abandon correctness.

Do not repeatedly rerun identical failures, remove tests to make them pass, hide disabled functionality, silently switch execution tools/models, or treat quiet compilation as a failed run. Capture partial work before stopping. Keep main runnable; unresolved changes can remain on an explicitly identified work branch rather than contaminating accepted progress.

## Progress and evidence

PROGRESS.md contains one row per criterion with status and evidence. Valid states:

- Not started: no implementation claim.
- In progress: an active slice owns the work; list partial coverage in evidence.
- Implemented: the complete criterion has code, but required verification is incomplete. Requires supervisor inspection; a drawn control alone is insufficient.
- Verified: complete behavior with the criterion's required evidence. Record the check/procedure and relevant commit or slice.
- Blocked: an explicit dependency or decision prevents progress; include the next resolving action. Manual verification pending is not automatically a blocked implementation.

Workers may propose statuses in their handoff but do not edit criterion statuses unless assigned that authority. Never mark an entire feature verified from a check of one subcriterion. Deferred items already excluded by SPEC.md are not missing required features; new deferrals require user approval.

Tests use disposable directories with generated fixtures. Prefer Rust's built-in tests and standard library helpers; add dependencies only for a concrete requirement. Keep operation tests independent of the GUI.

## Task contract

Keep Taskfile.yml current. After application bootstrap, task run launches the explorer, task test runs its automated checks, task clean removes generated build output only, and task design opens the existing layout. Include descriptions and retain the default task listing available tasks. Track Cargo.lock for this application.
