# S016 promoted workspace validation

## Source and automated checks

Reviewed terminal d8ffb317be036bf683ef147bf512839beb8b2ab6 is preserved at archive/s016-reviewed. Independent reviewer23dff4a1 found no concrete blocker. Parent inspected actual committed source/test diff and reran nine focused checks before promotion.

Clean terminal rebase/fixup-squash and fast-forward promotion3ada2f30d4277ca0c3aa0c3b950320d0d4db1e68 preserved complete src tree df7113c9e2a72e15d41dc5078baf8e098c2e4225 and handoff blob31b9269de96cb0a28c29aa24a1b62c75e938a388 exactly. No changed integration adapter or conflict resolution.

On promoted main: cargo fmt --check, cargo check --locked, cargo test s016, cargo test --locked, cargo build --locked and git diff --check passed. Focused9 passed; full root119 passed, one existing optional installed-font probe ignored. No preview, filesystem command or app clipboard activation.

## Executed native checks

`pwsh -NoProfile -ExecutionPolicy Bypass -File handoff/s011-native-smoke.ps1 -ExpectWrites` passed on the new binary: launched own process only, generated settings/folders, restored Unicode/missing tabs, final save/normal close and restart from app-written settings. No personal settings, clipboard or app input/capture outside the launched process.

`pwsh -NoProfile -ExecutionPolicy Bypass -File handoff/s016-owned-capture.ps1` passed: retained V001 fixture validated before launch and after normal close, exact launched PID/window verified, PrintWindow used only that HWND with measured client crop. No desktop capture, File Pilot launch/input, global clipboard or screenshot scaling. Output retained and copied byte-identically to tracked evidence:

- C:/SANDBOX/REPOS/mado-explorer.git/validation/file-pilot/evidence/s016-owned-listing-125.png
- C:/SANDBOX/REPOS/mado-explorer.git/validation/file-pilot/evidence/s016-owned-listing-125.json

Source3ada2f3; binary SHA2568303b4c4c792f8aa042c49f0a154f7c9b3be3cd344d4928e0ca2b3b2c00d6efb. Capture2026-10-10T09:56:52.5279866Z; client1500x1000physical pixels, measured DPI120, nominal125%. PNG SHA256fd2414334721122253ae6e74193ffca585e7ae7e884e6e0595078c7c29df9188.

Parent inspected the actual image. Compact tab/navigation/breadcrumb bands, table and filter/count footer appear within the normal client. All displayed locations are generated fixture paths. Actual cursor hover highlights readme.txt and shows its own-path tooltip over several rows. No pointer was moved/injected; this is a documented hovered state, not a clean no-hover L1 baseline pair.

## Limits

This is Mado-only evidence. No matched File Pilot image or user visual approval. It does not pass the full100/150/200% DPI gate, small-window/native hit-testing, Narrator speech, physical-input routing, system clipboard or performance targets. Rectangle-based headless checks remain distinct from actual paint/hit/accessibility verification. Historical S016 early root-enumeration fixture violation and correction remain in the reviewed worker handoff.

Captured own fixture, script and records survive source recovery. Generated target output can be removed by task clean; tracked evidence and handoffs remain. Capture task requires the explicitly retained original raster-worktree V001 fixture. Final File Pilot comparison remains pending and must use matched state/client/theme/scaling without relabeling this image as approval.
