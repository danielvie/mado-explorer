# Rust UI recovery from saved REA evidence

## Scope and answer

Static interpretation of saved File Pilot PE evidence only. No target execution, REA calls, or lifecycle changes in this pass. Only this report was written.

REA recovers useful control flow, constants, field relationships, and candidate algorithms for a new Rust/Windows implementation. It does not recover original Rust or original compilable application source. E-tabs explicitly classifies its pseudocode as `decompiler-generated-non-source`, `compilable: false`, with low-confidence recovered types. `FUN_*`, `undefined*`, raw offsets, unresolved indirect calls, and reconstructed stack values require interpretation, not mechanical translation.

No Rust output, compilation, or runtime fidelity has been established. Reimplement observed behavior and validate it independently; do not port inferred layouts as a trustworthy ABI.

## Evidence references

Aliases from `C:/SANDBOX/REPOS/rev-filepilot/analysis/evidence-index.json`; E-filter-text is a report-local alias, not an index edit.

| Alias | Procedure | Exact Evidence ID |
| --- | --- | --- |
| E-tabs | `0x1400fa420` | `ev_610efcb0ace1adf00ce0a6e6cb5bed7e75e25e387041c69546e3819770b66407` |
| E-filter-text | `0x14009de50` | `ev_b634e2c98ff68407d18459720df3817e8d2d647ec53ce746baf25327ba522d96` |
| E-matcher | `0x140123060` | `ev_4b786f7f26851765897b320658a520943f137b516e906b434ed9f2ed1aaf2fd2` |
| E-matcher-alt | `0x140124360` | `ev_28208ba77a6ec96ce61fb67207cf9ff7f75e052d8e5d10897c4dccf580066ae3` |
| E-shell-search | `0x140040560` | `ev_2671f325485b8692f2f8a5355a270f0885fb41f7d21ada00e6921d9ee7d9b092` |
| E-comparator-gap | unresolved `0x1400178a0` | `ev_a91305f4bdadfcaec0192e1f9d1b6af2d702c496671f09c1608b18d3ec378e42` |
| E-rename | `0x140169320` | `ev_fa9843735cd571a9f9e86e700a302811969eed219dbd4d26d59b5b034a87d902` |
| E-panel-read | `0x14012f4e0` | `ev_ada42c5a8b74072622034ed7962051e16f642b951941021a88c8487935b85c54` |
| E-panel-write | `0x14012e700` | `ev_625684f824cc7a93c7714c90771d61fb4b91ee08ec1d114047387d514044654e` |

All `.c` paths below are under `C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/` and are saved decompiler pseudocode, not original source.

## Code-backed observations

### Tab layout and interactions, partially recovered

- E-tabs, `queries-03-panel-tabs.c:1032-1282`: a rendering phase branches on UI state at `+0x272`. It walks the panel's tab chain from `param_2[1]` through tab offset `+0x540`, counts entries, and clears a per-tab `+0x678` flag while remembering its index. An allocation diagnostic names `Panel_RenderTabs`.
- E-tabs, lines 1168-1221: tab width uses available horizontal extent minus per-tab overhead and an extra control allowance, divides by tab count, and bounds the result with rounded multiples of the current UI metric, 10 and 16. Helper `0x140065160` participates, so exact rounding semantics remain uninspected.
- E-tabs, lines 1282-1300: helper `0x1400fd210` receives panel/context, width/height-like values, and an array. The caller copies each returned object's `+0x278/+0x280` geometry into 16-byte records. This is concrete layout flow, not proof of that helper's event semantics.
- E-tabs, lines 1409-1498: `ScrollLeft`/`ScrollRight` control results produce repeated records with opposite signed deltas sent to `0x140185710`. Left passes a zero-position flag; right passes a maximum-offset flag calculated from content and viewport extent. The control helper's interpretation of these flags is uninspected.
- E-tabs, lines 1553-1585: three decoded event cases invoke command record `0x22`; the middle case also invokes `0x58` with two input-state values. Lines 919-950 route another event through `0x22` then `0x59` with geometry. Do not name these commands as select/close/new-tab without inspecting their dispatch mapping.
- E-tabs, lines 1618-1685: the remembered flagged tab index drives a new scroll-position calculation using its rectangle, neighboring rectangles, viewport width, current scroll, and maximum scroll. The caller submits the resulting position through `0x140185710`.

Inference: adaptive tab widths, horizontal overflow controls, and keeping a marked tab in view are supported reconstruction targets. Exact clicks, drag-copy/move/reorder rules, close behavior, and command IDs remain gaps because `0x1400fd210`, `0x140183800`, `0x14010e440`, and their handlers were not interpreted here.

### The file-filter UI filename is misleading

E-filter-text, `queries-03-file-filter-ui.c`, is `FUN_14009de50`, not a recovered file-list renderer. Lines 1913-1971 reset a structure and insert paired keys/text, such as `ThisPC` and `This PC`, into 32-byte entries. Lines 3002-3083 and 18530-18623 repeat this pattern for tab/filter tooltip and command descriptions. Its saved callee list contains only `0x1401b6980`, `0x1401b6ec0`, `0x1401b8530`, and `0x1401b8770`; its saved caller is startup `0x140129430`.

Inference: a text/translation table initializer. Its labels are not implementations of the actions they describe. No visible-file-row culling formula was recovered from this function or the reviewed tab caller. This is a bounded gap, not evidence that File Pilot lacks culling. Tab geometry/scroll management does not establish file-list virtualization.

### Search scoring, partially recovered

- E-matcher, `filter-algorithm.c:73-146`: both candidate string spans above `0x400` enter a shortening loop involving `0x140187e70`. Exact character-boundary behavior is not established without that helper.
- E-matcher, lines 147-423: final mode argument zero uses `0x140065230`; nonzero delegates to E-matcher-alt `0x140124360`. Both expose separate primary/secondary scores and optional matched-position arrays. A helper name alone does not identify the zero-mode matching rules.
- E-matcher-alt, `queries-06-case-search.c:64-145,184-390,495-683`: strings/tokens go through `0x14013f5d0`; equality compares outputs of `0x140169dc0` on each 32-bit element. The visible scan records ordered candidate positions. Alternate matches call `0x140065880` and replace the current choice when their score is higher.
- E-matcher-alt's visible full-match scoring starts at `100 + 3 * (matched_count - candidate_length) - first_position`. Adjacent positions add 200; position zero adds 50; gaps over five deduct `50 * (gap - 5)`; preceding underscore/space adds 25. Explicit letter-range tests add 30 for recognized lower-to-upper boundaries. These constants appear independently in the primary and secondary candidate branches.
- E-matcher-alt, lines 440-445,746-774: per-token primary scores accumulate directly. Secondary scores accumulate after multiplying by `0.75`, adding/subtracting `0.5` by sign, and converting to integer. The return tracks whether a match was recorded, not proof that every query token matched.
- E-shell-search, `shell-search.c:87-110,137-182,183-259`: nonempty input passes a configuration with byte `0x20` to `0x1401b9110`. The caller matches two candidate spans plus auxiliary strings through E-matcher with mode `1`; it retains only positive aggregate scores. Results are 16-byte candidate/score records. A sort call uses comparator `0x1400178a0`.

Inference: an ordered/fuzzy matcher and weighted secondary text are supported. This caller establishes use for searchable Shell-menu commands; it does not establish identical global command-palette or filename-filter behavior.

Gaps: `0x1401b9110` query parser, `0x14013f5d0` conversion, `0x140169dc0` character mapping, `0x140065880` alternate-search implementation, and comparator body are uninspected. Space splitting is plausible but not a recovered grammar. Quoting, negation, wildcards, regex, case rules, ordering direction, and tie-breaking must not be specified from these records alone.

### Rename setup and editing handoff, partially recovered

- E-rename, `queries-03-rename.c:167-247`: `Rename_Start` allocates a private arena/context stored at application `+0xcf8`, retains its panel, reserves a selection-sized mapping, and chooses between two listing arrays using listing state. Its diagnostic records selection count and expanded state.
- E-rename, lines 242-309: each listing entry is checked against the panel's selected-entry hash table. Only found entries continue into edit-record creation. Thus the visible path is selection-driven, not an unconditional rename of all rows.
- E-rename, lines 309-343: branch conditions use listing flags and a setting. One path takes text from linked metadata; another invokes `0x14016da00`. A 16-bit value derived from the text length is kept, sometimes from a separately transformed span. It is not yet a proven initial selection range.
- E-rename, lines 379-424: each accepted entry gets a copied text span, per-item ID generated from `Rename%llu` plus entry identity, calls to `0x14017e400` with `0x100`, `0x14017dab0` with the initial text, and `0x14017df60`. Records form a doubly linked edit chain and an entry-to-record mapping.
- E-rename, lines 429-444: application action state receives the first edit record, panel/context, and callback `0x140169c00`.

Inference: per-selected-item edit state and an explicit editing/completion handoff can guide a Rust state model. Extension stripping/selection, initial caret, keyboard traversal, validation, commit/cancel, filesystem rename, and undo are not recovered by the start function. Do not infer these from uninspected editing helpers.

### Persisted panel state, stronger behavioral guidance

- E-panel-write, `queries-03-session-write.c:54-154,155-333`: JSON output follows explicit branches. Nonzero relationship IDs emit `Prev`, `Next`, `Parent`, `Child`, `NextTab`; `ID` and `SplitRatio` are emitted unconditionally. A pathless record emits `Vertical` then skips leaf-only fields. A leaf emits `Path`, `Expanded`, `Hidden`, sorting, optional nonempty `Filter`, `FilterSticky`, file/directory visibility, zoom, optional `Columns` and `FilterTypes`.
- E-panel-write, lines 99-109: every backslash in the newly formatted path fragment becomes `/`. Lines 333-425 omit inspector fields for layout value `4`; otherwise emit inspector settings, with inspector sorting/zoom/columns conditional on nonzero sort and column count.
- E-panel-read, `queries-03-session-read.c:294-427,628-767,875-1010`: recognized keys feed destination fields, not just string literals. `SplitRatio`, `InspectorSplitRatio`, and `InspectorZoomLevel` clamp to `[0,1]`; `SortedBy` and `InspectorSortedBy` clamp to `[3,30]`; `InspectorLayout` clamps to `[0,2]`. Path `/` bytes become the global separator byte. `FolderSort` maps `FoldersLast` to `1`, `Mixed` to `2`; legacy `SepFilesFolders` also writes that field.

Inference: typed panel/tree/tab state and explicit load-time bounds are recoverable specification inputs. Reader and writer are object-level routines, not proof of whole-session graph restoration or crash-safe file saving. Helpers, token grammar, malformed-input handling, unknown keys, column validation, and complete default values remain unverified.

## Read/search coverage

- Read all of `filter-algorithm.c`, `queries-06-case-search.c`, `queries-03-rename.c`, `queries-03-session-write.c`, and `shell-search.c`.
- Read E-tabs executable-body pseudocode lines 230-1708; searched the full `.c` for literals, divisions, selected call sites, and state offsets. Inspected saved type/pseudocode classification and callee-address metadata, not every assembly/value-flow section.
- E-filter-text has 18,624 lines. Read header, lines 510-559,1800-1869,1910-1971,18530-18624; searched the full file for control branches, filter/search/tab text, matcher calls, and division/floor/ceil patterns. Read saved callee/caller metadata. The first broad arithmetic search saturated on table indexing; narrowed searches did not establish row culling.
- E-panel-read has 1,013 lines. Read lines 249-427,628-767,875-1013; searched full file for keys, parsing calls, constants, and returns. Middle column/filter-type parsing and earlier defaults were not fully read.
- Full-file searches across saved `.c` files for `floor`, `ceil`, visible/render/rename/search identifiers yielded no additional culling implementation. This pass did not interpret all saved functions or all indirect callees.
