# S016 compact enabled workspace

## Candidate and ownership

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace`.
- Branch: `lane/compact-workspace`.
- Verified starting HEAD: `29ad63aea356ad8d31ee6ce7a06ea7fe37cdb4a6`; starting `git status --porcelain` was empty.
- Frozen source commit: `35757f22bf0ea4654974c6b70405b35f644f401a`.
- The following handoff-only commit is the terminal candidate. Its full SHA is returned in the native worker completion receipt. Source is unchanged from the frozen source commit above.
- Commission: authoritative main `PROGRESS.md` S016, native supervisor `01a12207`. Main, trackers, other checkouts and components were not edited. No merge, rebase, reset, push, target cleanup or delegation.

Only changed paths:

- `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace/src/main.rs`
- `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace/src/workspace.rs`
- `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace/src/compact_ui_tests.rs`
- `C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace/handoff/s016-compact-workspace.md`

## Runnable application changes

Existing checkpoint-2 functionality now uses compact dark bands:

- Tab band includes the explicit `Focused` label, tab overflow, plus and a working pane-local Layout menu. Split left/right, split top/bottom and Close pane invoke the existing layout/state operations. Focus retains the cyan border and no longer depends on color alone.
- Navigation buttons and editable Location share a band above 660 logical points. Narrow panes put Location on its own band. Existing full path text, native breadcrumb payloads and horizontal breadcrumb scrolling remain intact.
- Filter, Clear, displayed/total counts and selection move to the footer. Narrow panes use two footer lines. View opens the actual hidden/system checkbox before the lower-priority counter overflow, so counter scrolling cannot hide that action.
- Normal counters no longer consume status lines above the table. Errors, loading, incomplete listings, metadata warnings and no-match states remain visible. The warning/table body scrolls within its own clipped region rather than painting over the footer. Status is evaluated after current-frame editors, including Clear, so an old no-match warning cannot survive that frame's filter change.
- Removed the redundant Entries label. A stable, labelled table interaction receives focus instead. Row height remains 24 points, rows remain virtualized and metadata-drop priority is unchanged.
- Footer input is processed before explorer shortcuts. Opening-frame editor clicks, held focus, submission/exit frames and open/closing menus block background navigation/tab/selection shortcuts. Existing pane pointer focus, F6, finder ownership, tab drag geometry and state routing are retained.
- Path and filter use egui's `labelled_by`, preserving built-in editable text accessibility data. Plus and close-tab buttons have descriptive accessible names with actual enablement.

No previews, mutations, clipboard integration, dependencies, settings schema changes, downloaded fonts or unimplemented controls were added. Root Taskfile was already current and was outside the write boundary.

References inspected: `design/01-browse-preview.png`, `design/03-split-panes.png` and editable layout structure. These are design references, not an actual File Pilot comparison or pixel-perfect evidence.

## Final validation

All commands ran from the exclusive checkout, with:

```sh
cd C:/SANDBOX/WORKTREES/mado-explorer/compact-workspace
export CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target
cargo fmt
cargo fmt --check
cargo check --locked
cargo test s016
cargo test --locked
git diff --check
```

Final outcomes:

- `cargo fmt --check`: pass.
- `cargo check --locked`: pass.
- `cargo test s016`: 9 passed, 0 failed, 111 filtered out.
- `cargo test --locked`: 119 passed, 0 failed, 1 existing optional installed-font probe ignored. This includes all existing root UI/navigation/filter/finder/tab/split/drag/settings checks; no standalone component source changed.
- `git diff --check` and `git diff --cached --check`: pass. Git printed normal LF-to-CRLF working-copy warnings, not whitespace failures.
- Final owned diff inspected before source commit; staged paths were exactly the three commissioned Rust files.

Focused S016 checks exercise actual egui widgets and pointer/key events:

1. Normal 1050 x 700 header/table/footer geometry. Name heading above 130 points, file-row viewport above 480 points, navigation/path share a band and footer counters share its filter band.
2. 320 x 300, short 480 x 280 and minimum-width two-pane 646 x 340 geometry. Separate narrow path band, ordered non-overlapping bands, reachable action labels and usable row viewport.
3. First-click and held filter/path ownership against Ctrl+T, Ctrl+W, F5, Ctrl+A and row movement.
4. Inactive-pane filter click, independent text routing and F6 selection routing.
5. Actual View checkbox change and visible split refusal at narrow size.
6. Open/exit menu keyboard ownership, successful split and actual Close pane routing.
7. Clear invalidates a no-match projection in the same frame; updating status replaces the old warning.
8. Long warning and short-body scrolling preserve the error and reach footer controls.
9. Long/Unicode path remains complete, the narrow editor stays inside its pane, and an actual native breadcrumb press/release produces the exact payload without submitting it to the loader.

The existing `s006_visibility_control_updates_freshness_without_navigation_at_narrow_viewport` now opens View before clicking the checkbox. Original freshness, native location/history and hidden-listing assertions are retained. Test-only toolbar/status wrappers include the relocated footer/counters; the new suite also renders the actual workspace, so those wrappers are not the geometry evidence.

## Iteration diagnoses and fixture-boundary violation

- The first full root run failed `s007_same_location_ids_and_overflow_keep_active_tab_and_plus_reachable`: active label geometry was `Rect::NOTHING` after overflow. Smaller tab-strip width exposed delayed animated reveal. Setting only that horizontal strip to `animated(false)` corrected it. The unchanged focused original test passed, and final full runs pass. No assertion was weakened.
- An early version of the synthetic long-path test clicked `C:\` through `Explorer::workspace`, scheduling a read-only directory-loader request for a non-owned drive root. This was a test fixture-boundary violation, not a data-loss event. No file-content read, mutation, native app launch, clipboard or settings probe was requested. Do not characterize all historical runs as owned-fixture-only. The supervisor was notified and classified it explicitly. It was not repeated after recognition.
- The corrected test captures `Tab::toolbar`'s returned command without calling `Explorer::navigate_in` or the loader. Exact payload, unchanged current native location, unchanged navigation key and no pending navigation are asserted. It prevents request submission rather than cancelling a request afterward.
- The initial direct `run_ui/Tab::toolbar` replacement failed to click despite point containment: point `[12,59]`, response `[0,50]-[23.9,68]`, clip `[0,0]-[320,400]`, enabled background layer, correct pointer, hover/click false. Settling frames and adding TabId did not fix that harness. After two unsuccessful focused fixes, evidence was escalated. Supervisor provisionally classified layer/layout fidelity, authorized one production-equivalent CentralPanel/style/TabId correction, and that real press/release check passed. No production breadcrumb patch was made.
- Early new-test compilation also required correcting `Option<&Path>` comparison and the installed egui MouseWheel `phase` field. These were test-source errors; final checks above are green.

## Criterion proposal and pending evidence

S016 supports the existing implemented F01.1/F01.3/F01.4/F04.4/F04.6 behaviors with compact presentation and preserved headless regression evidence. F14.4 remains partial: logical-point normal/narrow/short/split geometry is checked, but native Windows 100%, 150% and 200% display scaling is not. Worker proposes no full-criterion Verified upgrade. Supervisor alone updates acceptance status.

No native app launch/capture/input, File Pilot comparison, Narrator check, personal clipboard/settings probe or user visual approval was performed for final validation. Actual DPI, font rendering, accessibility announcement order, visual/usability approval and performance measurements remain pending. New labelled controls are not a claim of Narrator verification. Accepted S011 persistence remains unchanged; no beyond-checkpoint-2 production gate was enabled.

## Next usable integration

Fresh read-only review should inspect the terminal candidate against baseline `29ad63aea356ad8d31ee6ce7a06ea7fe37cdb4a6`, especially real workspace geometry, footer-first input ownership, clipped warning scrolling, tab drag/overflow, menu routing and the safe native-command test. If accepted, the supervisor can preserve the reviewed head and promote the bounded source plus this handoff under the existing serial integration policy, then rerun root fmt/locked compile/focused and affected root tests. Native visual/DPI/Narrator approval stays pending independently.
