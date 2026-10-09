# V001 File Pilot comparison

Visual acceptance: PENDING. Preparation only. No application screenshot, native interaction, or File Pilot comparison was performed. User visual approval is required, both for this early checkpoint and again at final acceptance.

## Scope and safety

Baseline `bb57d30c0d9904e9cff22796f72c1743ac856903`, branch `lane/raster-viewport`, checkout `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport`. Writes belong to `validation/file-pilot/**` and this generator's disposable outputs under that checkout's ignored `target/visual`. No application, design, framework, dependency, production gate or acceptance tracker changes.

The fixture is a shared location, not a copy of personal files. Use the same absolute listing path in both applications. Do not rename, copy, move, trash, permanently delete, open files in external applications, use the clipboard, or browse outside the fixture during this procedure. Never close Mado's final tab, which can navigate to the personal home location. Do not use Up above `listing`.

There is no connected dedicated desktop controller. PowerShell input, HWND selection, PrintWindow capture and DPI measurement were NOT validated in this commission. Do not run unverified helpers or capture the foreground/full desktop. Existing historical captures are not V001 evidence. This preparation includes no automation for native input or capture.

File Pilot's CLI, restored locations, window ownership, settings isolation, theme controls and supported finder/preview actions remain unverified. Neither the executable nor `/?` was launched. Its user data/configuration was not read or rewritten. Use the manual fallback below, not guessed arguments. No external CLI research was needed.

## Generated fixture and checks

Requires installed Node and Task only. Scoped `.gitattributes` preserves LF generator hashes on Windows checkouts. Commands work from any current directory:

```powershell
task --taskfile C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/Taskfile.yml
task --taskfile C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/Taskfile.yml test
task --taskfile C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/Taskfile.yml run
task --taskfile C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/Taskfile.yml validate
```

`run` claims one fixed unique directory exclusively. A second run refuses the existing directory; use `validate` instead. No operator path arguments are accepted. Tests claim fresh UUID-suffixed roots in the same namespace and remove only their generated outputs. No arbitrary cleanup path is supported.

Retained root:

`C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/target/visual/v001-file-pilot-26cbf184-176d-4a38-b08d-2cd84a920c20`

Browse this folder in both apps:

`C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/target/visual/v001-file-pilot-26cbf184-176d-4a38-b08d-2cd84a920c20/listing`

Its sibling `.owner.json` establishes the claim. `manifest.json` records absolute paths, generator SHA-256, expected entry kinds, byte sizes, file hashes and UTC modification times. Folder sizes are unknown. `workspace.json` is an actual version-1 Mado settings snapshot with one fixture tab, fixture bookmarks/recents and requested 1200 x 800 logical client size. It may change on reviewed native settings saves. Validation rejects saved tab/bookmark/recent paths outside this fixture's listing before a later launch. It is never a source of cleanup paths.

Listing contents are four folders, including `Empty`, and eight immediate files. Three folders contain a generated text/CSV child. Fixtures cover a zero-byte file, 1024/65536-byte files, `café-東京-𐐀.txt`, a long truncated-name candidate, fixed dates, a 64 x 48 RGB checkerboard PNG and a two-page text-only PDF. PNG chunks/CRC/inflated length and PDF object/xref offsets are checked structurally. This is not PDF renderer evidence. No hidden attributes, links, ACL denial, path-length stress or performance dataset is claimed. The test creates and removes its own nonprivileged junction solely to check cleanup refusal.

Preparation ran at 2026-10-09 UTC with Node v24.15.0 and Task 3.50.0+49932d0. `test`, `run`, `validate` and default task listing passed. Test roots were removed; the comparison root remains. Manifest SHA-256, updated during preparation to record the final generator, with listing data retained unchanged:

`c7e2f861e49d59411cae615a77488c115ba3614de1b7cd25ee53bc7363e2af6f`

Generator SHA-256:

`64874669155af4b4fc7458b20a450591816f58d79dfe4722580a5e2cb274af93`

## Operator launch and reference fallback

1. Obtain a supervisor-approved Mado binary and record its source commit and binary hash. The existing `C:/SANDBOX/REPOS/mado-explorer.git/target/debug/mado-explorer.exe` was found, but its build provenance was not verified and it was not launched. Do not infer its source version from its path. Building the app is parent-owned work, outside this preparation.
2. Validate the fixture. After the parent confirms that exact binary, launch Mado with the owned settings override below. This is a PLANNED operator command, not an executed check. It saves/restores the invoking process's environment and waits for this Mado instance to exit.

```powershell
$fixture = 'C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/target/visual/v001-file-pilot-26cbf184-176d-4a38-b08d-2cd84a920c20'
$exe = 'C:/SANDBOX/REPOS/mado-explorer.git/target/debug/mado-explorer.exe'
task --taskfile C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/Taskfile.yml validate
if ($LASTEXITCODE -ne 0) { throw 'Fixture validation failed; do not launch' }
$previous = $env:MADO_SETTINGS_PATH
try {
    $env:MADO_SETTINGS_PATH = "$fixture/workspace.json"
    Start-Process -FilePath $exe -ArgumentList ('"' + $fixture + '/listing"') -Wait
} finally {
    $env:MADO_SETTINGS_PATH = $previous
}
```

3. The user opens `C:/Users/daniel/AppData/Local/Voidstar/FilePilot/FPilot.exe` interactively. It may restore personal folders. No agent launches, inspects or captures that state. The user manually navigates to the exact fixture listing using File Pilot's visible UI, types rather than clipboard-injects the path, and closes/hides unrelated tabs/panes/sidebars. Do not rewrite File Pilot's configuration to arrange the comparison.
4. The user supplies window-only/client-only images that contain only fixture locations. Use a trusted user-operated window capture method that saves directly to PNG without replacing clipboard contents. If none is available, screenshot evidence stays pending. Do not submit a desktop screenshot or one showing personal restored tabs, recents, tooltips or other apps. Do not relabel crops of personal captures as owned evidence.
5. For Mado finder captures, first type `Projects` so fixture results alone remain, or enable Path entry and expand the fixture listing's children. Its empty known-location finder can include personal common-place labels despite isolated settings. Do not capture that empty state automatically. The operator must inspect and withhold any image with non-fixture results.
6. Keep fixture files unchanged. Record File Pilot version/theme from user-visible UI and Mado source/binary provenance. No behavior absent from the actual File Pilot build should be invented to match Mado. If equivalent controls are unavailable, record that mismatch.

## Matching conditions and evidence record

Requested starting Mado client size is 1200 x 800 logical units, not an observed window size. Actual pixel size, outer dimensions, monitor, DPI, Windows scale, fonts, File Pilot theme/version and GPU are UNKNOWN. No native measurement was performed.

For each pair, use the same monitor, display scale, fixture, dark theme, sort direction, selected paths, filter, scroll position, pane ratio and preview visibility. Match measured client pixel dimensions, not outer window borders. Prefer 1200 x 800 logical units if the monitor permits it. Record what actually fits. Never stretch screenshots to make them match. Derive each client pixel size from a lossless client-only image with a documented crop boundary or a separately validated own-window measurement. Record outer size separately if known. If exact matching cannot be established, label the pair unmatched and keep its visual decision pending.

Mado currently forces dark theme. Light-theme comparison is unavailable, not passed. Repeat at 100%, 150%, 200% scaling when an operator can safely arrange each. Physical size is NOT assumed to equal logical size times scaling until measured. Do not change system scaling automatically. If a monitor cannot accommodate the planned size, record the actual smaller matching dimensions and minimum-pane refusals.

Save supplied images under `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/evidence/` with names such as `early-100-listing-mado.png` and `early-100-listing-file-pilot.png`. The folder does not yet exist. The operator creates it only for supplied evidence. Keep originals; record any crops and their original hashes. Do not edit the fixture manifest to insert screenshot results.

Complete one record per application/state/image, then one finding record per difference:

| Field | Required evidence |
| --- | --- |
| Identity | Phase early/final, state ID, capture UTC, operator, application version, Mado source SHA/binary SHA or File Pilot executable version/hash if supplied |
| Dataset | Absolute listing path, fixture manifest SHA, successful validate command/time |
| Display | Monitor, OS, Windows scaling %, measured DPI if available, client width/height in pixels, logical size if known, outer size if known, measurement method |
| State | Theme, sorting, selected complete paths, active pane/tab, pane ratio, filter/query, scroll position, preview visibility/page/fit |
| Image | Absolute PNG path, SHA-256, capture method, original/crop coordinates, confirmation no personal content |
| Interaction | Exact pointer/keyboard steps, target pane/tab, observed result, latency only if measured |
| Finding | Both source image paths/hashes and state IDs, observed difference, design reference, usability impact, user decision |

For image hashes after the user supplies them:

```powershell
Get-ChildItem -LiteralPath 'C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/evidence' -Filter '*.png' -File | Get-FileHash -Algorithm SHA256
```

No evidence images or completed environment/state records exist yet.

## State checklist

All states below are PENDING visual comparison. Source availability is not a passing visual check. Reference actions use its actual UI; its shortcuts/capabilities are not assumed identical.

| ID | Matched state and operator steps | Current Mado availability |
| --- | --- | --- |
| L1 | Name ascending, folders first, no filter/selection, top scroll. Compare headers, timestamps, sizes, truncation and Unicode. | Native listing UI in source |
| L2 | Click `research-notes.pdf`, Ctrl+click `capture.png`, then a Shift range in displayed order. Capture single and multiple selection separately. Use arrows/Home/End/Page keys and inspect visible active row/focus. Do not press Enter on files. | Native selection/keyboard UI in source |
| T1 | Make two fixture tabs using Ctrl+T. Navigate the second to `Projects` via editable path. Set first filter `size`; second selects `readme.txt`. Switch back, check selection/filter/scroll isolation. Keep at least one fixture tab. | Native tabs/filter UI in source |
| S1 | Split left/right using visible layout action, start at listing in each pane, navigate right to `Reference`. Focus right then F6; confirm border/label and keyboard target. Resize divider and narrow panes; inspect hidden metadata before unusable names. | Native split/focus/dividers in source |
| S2 | Reorder two tabs, move a tab to the other pane and, where space allows, drop a tab on an edge. Record drop/focus feedback and retained location/filter/selection. | Native tab drag in source; actual interaction untested here |
| O1 | On a fixture tab Ctrl+O, query `Projects`; capture only fixture results, ranking emphasis, query focus and captured target. Escape must leave location unchanged. | Native finder in source; empty query may disclose common-place labels |
| O2 | Path entry with relative `Projects`; Tab expands children without navigating. Escape. Repeat Enter into Projects, Shift+Enter into a fixture tab, Ctrl+Enter into a split only if room. Check captured pane target. A nonexistent relative `missing-owned-folder` gives recoverable error. | Native explicit path/expansion/actions in source |
| E1 | Navigate to `Empty`, capture explicit empty state, then Back to listing. Filter `zzzz-no-match`, capture no-match/count, clear without navigation. | Native empty/filter states in source |
| I1 | Select `capture.png`. In File Pilot enable an actual supported preview, if available. Record fit, 100%, pan/zoom, dimensions and preview divider. | Mado native preview unavailable. Raster/summary components are not a preview integration |
| P1 | Select `research-notes.pdf`. In File Pilot use actual supported PDF preview, if available; record page 1/2 and page controls/resize. | Mado native preview unavailable. PDF backend/service/controls evidence is component-only |

Do not enable previews or fabricate a Mado panel for I1/P1. Reference-only images may document File Pilot capability, but cannot pass a paired comparison. Final acceptance must revisit these states after production integration and include remaining required UI/OS/accessibility/performance checks.

For every matched state inspect:

- [ ] Hierarchy: primary listing versus toolbars/finder, collapsed sidebar, active tab and focused pane.
- [ ] Spacing: top chrome, navigation/editor/filter placement, headers, cell padding, divider width and modal margins.
- [ ] Typography: actual font, size/weight, metadata legibility, Unicode glyphs and truncation/hover.
- [ ] Row density: measured row pitch and visible row count at matching client size. Do not use screenshot scaling as a measurement shortcut.
- [ ] Colors: background levels, cyan selection, contrast and inactive/hover states.
- [ ] Focus feedback: pane indicator, active row versus selected paths, text caret, keyboard focus visible without color alone.
- [ ] Interactions: selection modifiers, tab/split drag feedback, divider resizing, finder targeting/dismissal and filter isolation.
- [ ] User visual approval: approved, changes requested or pending, with paired image/state evidence and date.

## Existing design and source observations

These are inspected references, NOT observations of actual File Pilot or Mado native windows:

| Source | Observation / question for comparison |
| --- | --- |
| `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/design/01-browse-preview.png` | File list is primary; dark separated tab/navigation/table/filter bands, cyan outlined selected row, metadata columns, optional right PDF preview. SHA-256 `10a9746270e3ccfa86294cd1ef0dd8b6330f5e42c2d8265db0acb201de33f368` |
| `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/design/02-folder-finder.png` | Dimmed browsing behind rounded centered finder, large query, category chips, two-line path results, cyan selected result and shortcut hints. SHA-256 `45df065f8c229ca1ac90579325a9a968425be8dfd00e482580b8e3d669990aa0` |
| `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/design/03-split-panes.png` | Independent tab/navigation/filter bands, cyan right-pane boundary, narrow-pane name priority and separate image preview divider. SHA-256 `d77e2c136280ecccfc37a15a14fd7e185a1de4e0a01eaebfae2083be78eaa97f` |
| `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/src/main.rs` at baseline | Dark theme, item spacing 8 x 3, selection fill RGB 18/93/111 and stroke 89/215/235, 24-logical-unit table row plus spacing. Text buttons/path editor, UTC and byte-size headings differ from the drawn design. Actual rendered fonts/density remain unobserved |
| `C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/src/finder_ui.rs` at baseline | Finder includes heading, captured-base/status text, Path entry checkbox and action buttons. Design's category chips are not the same source UI. Inspect actual modal hierarchy before judging whether this is too busy |

Actual File Pilot findings: NONE. Actual paired-image findings: NONE. Source/design differences above identify things to inspect, not requests to refactor application style.

## Retention and cleanup

Retain the fixture until the operator completes comparison. The root Taskfile's `clean` uses Cargo clean and can remove all target output, including this fixture; do not run root cleanup while V001 evidence is pending. This preparation does not change that shared task.

After both apps close and evidence is retained outside the fixture, the only allowed cleanup command is:

```powershell
task --taskfile C:/SANDBOX/WORKTREES/mado-explorer/raster-viewport/validation/file-pilot/Taskfile.yml clean
```

This was NOT run on the retained comparison root. The same cleanup implementation passed on generated test roots. It checks fixed namespace, ordinary ancestors, exact marker, generated allowlist, unchanged manifest, fixture bytes/times and admitted settings locations before deletion. It refuses links/junctions, unknown files including screenshots/settings temporaries, missing outputs and altered ownership. On refusal, keep data and report it. Do not bypass checks or manually delete an operator-supplied path. If an app still writes, stop and close it first. The helper is for trusted single-operator fixtures, not a hostile concurrent-filesystem sandbox.

## Next actions

1. Fresh read-only review of the terminal candidate's fixture safety, settings setup and evidence claims.
2. Parent integration/validation, no criterion acceptance from preparation.
3. User-operated File Pilot fixture navigation, confirmed Mado binary launch, matched-state window images and completed provenance records.
4. Record concrete image-backed findings here, obtain user visual approval. Repeat at final acceptance; previews, light theme and unmeasured scaling stay explicitly pending until actually available and checked.
