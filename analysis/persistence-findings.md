# Settings, persistence, and startup findings

## Evidence and coverage

Static interpretation only. No execution of `FPilot.exe`, REA calls, or runtime observations. Target SHA-256 `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e`. The binary declares product/file version `0.7.0.0`; that is not a persistence-format version.

All source paths below are under `C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/`.

| Reference | Saved evidence | Procedure |
| --- | --- | --- |
| S | `queries-03-settings-function.json`, `ev_20c3a026861ba06c6c0f1f04a8e4e13c08b5010e8c4ae86ec621a2ccde37fba5` | `0x14002f480`, body through `0x140032297` |
| R | `queries-03-session-read-function.json`, `ev_ada42c5a8b74072622034ed7962051e16f642b951941021a88c8487935b85c54` | `0x14012f4e0` |
| W | `queries-03-session-write-function.json`, `ev_625684f824cc7a93c7714c90771d61fb4b91ee08ec1d114047387d514044654e` | `0x14012e700` |
| D | `queries-03-data-location-function.json`, `ev_01e8ea4840e3fa6b5c9d7a7fa8f3711129c765e8f478637d89fbb98839e494c2` | Entry `0x140188fb0`, body through `0x14018a82b` |
| I | `queries-03-startup-function.json`, `ev_9c684b54d33804773349f6ab88428aade099549bbfd355aeeb52bf75552e0dbd` | Initialization `0x140129430`, body through `0x14012c1ca` |
| C | `queries-06-config-read.json`, `ev_20a600f3dc95ee795da3a80a4d767dd235d3f0ea150c6cde0f73aaf182021a23` | Config reader `0x140039820` |
| F | `queries-06-config-recovery.json`, `ev_d288753bc8c007bed15e270e7df590ffdeb6b8cecbb05bcad3c291efe4da1f08` | Deferred config writer `0x1400356f0` |
| E | `queries-06-executable-path.json`, `ev_ccb605df65f846f20ea36601daf340c8af6a237dc47ddff594ffac689cad4eed` | Executable-path helper `0x14019d900` |
| GR | `queries-06-storage-guid-roaming.json`, `ev_e2a86cb8e98e0fe74e9df96899556def435f56a49238fcb515cb1478481423c3` | Complete 16-byte read at `0x1401d6bf8` |
| GL | `queries-06-storage-guid-local.json`, `ev_3ef461ffca5f13fd44b5de42af620336055f5a835fab6da4e811eb26aa94fe47` | Complete 16-byte read at `0x1401d6c38` |

Read all extracted C for S, R, W, D, C, F, and E. Read complete GR/GL records and `C:/SANDBOX/REPOS/rev-filepilot/analysis/windows-sdk-mappings.txt`. C marks value-flow evidence truncated; F marks it untruncated. Startup I has 1,817 lines. Reviewed lines 1 through 419 and 1120 through 1817, plus bounded keyword searches across the file. Lines 420 through 1119 were not fully reviewed. Large function JSON dossiers were inspected selectively for identity, limitations, boundaries, and truncation, not fully read. S, R, I, and D mark high-pcode/value-flow evidence truncated; W marks it untruncated. These flags are not evidence that the extracted C itself stops early. Ghidra's types, helper meanings, indirect calls, and boundaries remain interpretations, not source declarations. Existing `C:/SANDBOX/REPOS/rev-filepilot/analysis/literal-findings.md` supplies string/resource corroboration only.

## Settings inventory and constructor defaults

S constructs a zeroed `0xc80`-byte descriptor collection. Forty registered descriptor pointers occupy indices `0x168` through `0x18f`. It copies names, constructs default values, and supplies choice lists. **This is a descriptor/default constructor, not the JSON config-file parser.** Its decimal loops convert built-in default strings, not user configuration input. Helper `0x1400349d0` constructs boolean-like descriptors; helper `0x140034d30` constructs choices. Integer descriptors explicitly use type tag 5 and signed decimal conversion; string descriptors use tag 1 and copied string data. Boolean/choice helper bodies were not inspected, so their tags and invalid-input behavior are unknown.

Every row below is a constructor default from S, not a verified effective setting after file loading. Types describe the constructed values, not a fully established JSON wire contract.

| Setting | Constructed type | Default |
| --- | --- | --- |
| FontName | String | `segoeui.ttf` |
| FontSize | Integer | `9` |
| InspectorFontName | String | `consola.ttf` |
| InspectorFontSize | Integer | `10` |
| Spacing | Integer | `4` |
| Animations | Boolean-like | `true` |
| RoundCorners | Boolean-like | `true` |
| SystemColorScheme | Boolean-like | `false` |
| ColorScheme | Choice | `Voidstar` |
| ColorSchemeLight | Choice | `Voidstar` |
| ColorSchemeDark | Choice | `Voidstar` |
| AlternatingRowColors | Boolean-like | `false` |
| InactiveOpacity | Integer | `60` |
| ShowFileExtension | Boolean-like | `true` |
| LongPathSupport | Boolean-like | `false` |
| ShowShortenedPaths | Boolean-like | `true` |
| ShowBreadcrumbs | Boolean-like | `true` |
| ShowHiddenFiles | Boolean-like | `false` |
| ShowSystemFiles | Boolean-like | `false` |
| LoadThumbnails | Choice | `LocalDisk` |
| StoreRecents | Choice | `Persistent` |
| ShowRecents | Boolean-like | `false` |
| SortRecents | Boolean-like | `false` |
| CalcFolderSizes | Boolean-like | `true` |
| CalcFolderSizesSSD | Boolean-like | `true` |
| CalcFolderSizesHDD | Boolean-like | `false` |
| CalcFolderSizesNetwork | Boolean-like | `false` |
| ActOnPress | Boolean-like | `true` |
| ShowCheckboxes | Boolean-like | `true` |
| SingleClick | Boolean-like | `false` |
| SingleClickFoldersOnly | Boolean-like | `false` |
| HoverToSelect | Boolean-like | `false` |
| UnderlineOnHover | Boolean-like | `true` |
| DoubleClickToParent | Boolean-like | `false` |
| Overscroll | Boolean-like | `true` |
| ReverseScroll | Boolean-like | `false` |
| CharacterKeyAction | Choice | `TypeInFilter` |
| InspectorLayout | Choice | `Auto` |
| LoadOnStartup | Choice | `LastSession` |
| NewTabShow | Choice | `Current` |

Choice lists, in constructor order:

- Three color-scheme descriptors: `Sirius`, `Pollux`, `Eridanus`, `Voidstar`, `Regulus`, `Altair`.
- `LoadThumbnails`: `LocalDisk`, `Always`, `Never`.
- `StoreRecents`: `Persistent`, `Session`, `Never`.
- `CharacterKeyAction`: `TriggerHotkey`, `TypeInFilter`, `SelectTypedItem`.
- `InspectorLayout`: `Auto`, `Hor`, `Vert`.
- `LoadOnStartup`: `LastSession`, `Root`, `Home`.
- `NewTabShow`: `Current`, `Root`, `Home`.

The config-reader section below establishes four loaded numeric clamps, top-level ASCII key normalization, and named color-scheme records. Remaining unknowns include units, other ranges, option-name matching, boolean/choice helper validation, and how every option affects behavior. `VSync` is a separate literal candidate, not one of these forty descriptors. Do not derive UI controls, tabs, accessibility, or effective long-path support from these names.

## Storage selection and installation state

D creates three named mutexes: `FilePilot_Config_Mutex`, `FilePilot_UserData_Mutex`, and `FilePilot_Session_Mutex`. Creation alone does not establish where reads/writes acquire them or prove atomic replacement.

D populates candidate global paths using `SHGetKnownFolderPath` and `Voidstar\\FilePilot`. GR bytes `db85b63ef965f64ca03ae3ef65729f3d` decode to `3EB685DB-65F9-4CF6-A03A-E3EF65729F3D`, `FOLDERID_RoamingAppData`. GL bytes `8527b3f1ba6fcf4f9d557b8e7f157091` decode to `F1B32785-6FBA-4FCF-9D55-7B8E7F157091`, `FOLDERID_LocalAppData`. The saved Windows SDK mappings corroborate both identities.

At indices `0x15` and `0x16`, D calls the roaming known-folder API and appends `Voidstar\\FilePilot`. These two decompiled branches lose arguments to `FUN_1401b7d60`, so their complete default filenames cannot be reconstructed here. Their global positions correspond to config and user-data paths. At index `0x17`, D explicitly appends `\\Voidstar\\FilePilot\\FPilot-Session.json` to LocalAppData. Thus the ordinary roots are roaming application data for config/user data and local application data for session. These are known-folder API results, not evidence that the app reads environment variables. Logs also use LocalAppData in D.

E establishes that `0x14019d900` calls `GetModuleFileNameW` with a null module handle and capacity `0x800` UTF-16 code units. It returns pointer/length state and leaves an empty result for the explicit insufficient-buffer case. D transforms its nonempty executable-path result through still-unresolved helpers `0x140188d00` and `0x140067a00`, then constructs three candidate paths:

- `%S\\FPilot-Config.json`
- `%S\\FPilot-UserData.json`
- `%S\\FPilot-Session.json`

Each candidate is queried separately through `0x1400cbd50`. A nonzero byte at result offset 1 replaces only that candidate's global path: config at `0x1401e8d48`, user data at `0x1401e8d58`, session at `0x1401e8d68`. This supports independent per-file overrides, not an all-or-nothing portable directory switch. The candidate base is now established as executable-path-derived. Executable-adjacent storage remains the interpretation of the intermediate transformations; their bodies were not read. Do not collapse those unresolved steps into a verified parent-directory algorithm.

Standalone versus installed detection is unresolved. The sole explicit installation-related gate inspected is the one-argument `--uninstall` branch: result field at offset `0x30` from `0x1400c4370` must equal 3 before it proceeds. That helper's enum and detection rules are unknown. Installer label literals do not prove that persistence selection follows installation state.

## Persisted panel record

W writes one JSON-shaped panel object, not the entire session file. R reads corresponding tokenized records. This supports the following record fields:

| Fields | Stored form and conditions from W |
| --- | --- |
| `ID` | Integer, always emitted |
| `Prev`, `Next`, `Parent`, `Child`, `NextTab` | Integer references, omitted when zero |
| `SplitRatio` | Floating-point number, always emitted |
| `Vertical` | Boolean, emitted only when path length is zero |
| `Path` | String, emitted only when path length is nonzero; output backslashes become `/` |
| `Expanded`, `Hidden` | Booleans, path-bearing records only |
| `SortedBy` | Integer, path-bearing records only |
| `FolderSort` | String from `FoldersFirst`, `FoldersLast`, `Mixed` table |
| `Filter` | String, omitted when empty |
| `FilterSticky`, `HideFiles`, `HideDirs` | Booleans |
| `ZoomLevel` | Floating-point number |
| `Columns` | Optional array of objects containing integer `Kind` and floating-point `SplitRatio` |
| `FilterTypes` | Optional collection whose reader consumes strings; writer element format remains behind `DAT_1401cff84` |
| `InspectorSplitRatio`, `InspectorLayout` | Floating-point number and integer |
| `InspectorDetails`, `InspectorDetailsExtra`, `InspectorWordWrap`, `InspectorHideLineNumbers`, `InspectorShowWhitespace`, `InspectorHideGrid`, `InspectorHideMinimap` | Booleans |
| `InspectorSortedBy`, `InspectorFolderSort`, `InspectorZoomLevel`, `InspectorColumns` | Integer, folder-order string, floating-point number, column-object array |

For path-bearing records, W omits every inspector field when internal layout value equals 4. It emits inspector sorting/zoom/columns only when inspector sort is nonzero and inspector column count is nonzero. These are serialization conditions, not UI availability claims.

R clamps panel and inspector split ratios and zoom to `[0,1]`; clamps panel and inspector `SortedBy` to `[3,30]`; clamps loaded `InspectorLayout` to `[0,2]`. It converts path `/` characters to the global separator set to `\\` by D. R also accepts `SepFilesFolders` and `InspectorSepFilesFolders` booleans, mapping true to folder-order enum 0 and false to 2. For `FolderSort`, recognized `FoldersLast` maps to 1 and `Mixed` to 2. W emits the folder-order strings rather than those alternative booleans.

R initializes five columns and a first ratio bit pattern `0x3ee66666`, about 0.45, but complete default column kinds and other ratios are not established. Inspector defaults come from helper `0x14007b390` and are unresolved. Unknown fields call `0x14003bc90`, apparently a token-skipping helper; its nested-value behavior is unverified. Record-link validation, cycle handling, nonexistent paths, and complete escaping behavior are unknown.

## Config reader, validation, and status

C reads the selected config global through `0x1400cbdb0` and calculates a content fingerprint through `0x1401b40a0`. It compares that fingerprint with its supplied prior value before parsing. The hash algorithm and file-read error enum are unresolved.

C indexes top-level string keys shorter than 128 bytes after explicitly lowercasing ASCII `A` through `Z`. It recognizes `options`, `colors`, `hotkeys`, and `version`. Top-level section indexing accepts object or array token kinds; that alone does not establish every section's accepted payload. Token kinds are object 1, array 2, string 4, and primitive 8. Primitive tokens group numbers, booleans, and null rather than proving a numeric type. The scanner/tokenizer checks allowed initial characters, printable primitive bytes, matching bracket kinds, and incomplete containers. String scanning/escapes are delegated to `0x1401baac0`. Do not specify strict RFC JSON validation or exact malformed-input status mapping from this decompilation.

C constructs defaults via `0x140036170`, then looks up the forty known option descriptors. Unknown option names do not receive descriptors in this loop. Values require a single value token under the matched property:

| Descriptor type | Reader behavior |
| --- | --- |
| String, tag 1 | Requires a string token and nonzero length before replacement. The copy call loses its length argument in the decompilation. |
| Boolean, tag 2 | Extracts a primitive token and delegates conversion to `0x1400349d0`. Exact accepted spellings and missing/wrong-type behavior remain unknown. |
| Choice, tag 4 | Delegates to `0x14003bf80`; accepted input forms and invalid-choice behavior remain unknown. |
| Integer, tag 5 | Requires a nonempty primitive token; consumes an optional leading minus and decimal digits until the first nondigit. No whole-token numeric validation is visible. |

When an Options section is indexed, C clamps loaded `FontSize` and `InspectorFontSize` to `[8,16]`, `Spacing` to `[1,10]`, and `InactiveOpacity` to `[0,100]`. These names follow the descriptor/value offsets already established by S: `0x60`, `0xf0`, `0x138`, and `0x378`. No units or other integer clamps are established. Missing or malformed boolean/choice values still reach their helpers, so do not assume all wrong-type values preserve defaults.

Colors processing recognizes a string scheme name with one object value. It constructs a scheme via `0x140024540`, stores the name, iterates 33 known color descriptors, extracts string values, and calls `0x140034a90`. This establishes named scheme objects with recognized string-valued color properties. Color syntax, missing-color behavior, and duplicate-name handling remain unresolved.

A present indexed Hotkeys section triggers `0x140035100`. **No key-chord parsing, binding validation, or `Keys`/`Command` entry parsing is visible inside C.** Those fields remain literal candidates until the delegated helper is inspected.

The reader initializes the format-version field to 1. A top-level primitive `version` value is parsed as signed decimal prefix and clamped to at least 1. A missing, string-valued, zero, or negative version does not establish rejection. No upper supported-version comparison or migration dispatch appears in C.

The result status resides at offset `0x30`, separate from the version field at offset 0:

| Status | Exact producer condition in C |
| --- | --- |
| 0 | File-read helper returns its status 1. Meaning of that helper status remains unresolved. |
| 1 | File-read helper returns another nonzero status. |
| 2 | Assigned after a successful changed-content read; retained on early scan/count failure. |
| 3 | Content fingerprint equals the supplied prior value; immediate return without parsing. |
| 4 | Final expression when version is nonpositive. No normal route to this is established because initialization and parsed-version clamping make it positive. |
| 5 | Final expression when version is positive. This is I's accepted result, not a specific format revision. |

I accepts status 5 and otherwise substitutes defaults. It requests warning/deferred-write handling for nonzero unsuccessful status and dispatches distinct command IDs for statuses 2 and 4. That does not prove an old-format threshold. Intermediate tokenizer errors have separate negative codes; their full propagation through decompiled pointer-typed token-count variables remains uncertain.

## Deferred config writing and backup limits

Despite its saved filename `config-recovery`, F at `0x1400356f0` is a deferred/throttled config-write path, not an established backup routine. It sets a pending field at `param_1 + 0x15c`. Its work requires elapsed microsecond arithmetic above 99,999 and a global counter difference above 9. The counter's unit is unknown. It transforms the current config pathname via `0x140067a00`, calls `0x1400c2db0`, serializes config through `0x140032680`, updates the content fingerprint, and passes the selected config path plus serialized data to `0x1400cd6e0`. It then clears pending state and updates its throttle marker through `0x1400018e0`. The lower helper bodies, actual write-success checks, mutex acquisition, atomicity, and directory creation are not established here. No alternate backup destination or backup-copy operation appears in F.

Corruption text at VA `0x1401c4db0` says a copy has been preserved in the original location. Older-format text at `0x1401c4f78` says a copy will be preserved. Neither message verifies backup creation, naming, retention, migration, rollback, or failure safety. Actual format revisions and backup behavior remain unknown.

## Remaining whole-file schema gaps

The panel schema and config-reader behaviors above are code-supported. Broader user-data/session structures remain literal candidates, not validated whole-file schemas:

- User data: layouts, saved folder views, recents, path/command/context-menu bookmarks, installer-presentation state, group state. Candidate names include `Layouts`, `Folders`, `Recents`, `PathBookmarks`, `CommandBookmarks`, `ContextMenuBookmarks`, `InstallerPresentedVersion`, `Score`, `Decay`, `Ordinal`, `QuickAccessGroups`, `GotoGroups`, and `ClosedCmdGroups`.
- Session/layout: window position/size/maximization, quick-access and details state, `Panels`, `SelectedPanel`, and a `Layout` wrapper.

I calls `0x140140030` while selecting startup state; its full schema/status semantics are unresolved. User-data/session format versions, serializers, and recovery rules were not established by the new config evidence.

## Startup arguments and `--root`

D uses `GetCommandLineW` and `CommandLineToArgvW`. Special handling occurs only when argument count is exactly 2, including the executable. It compares the single argument with `--root` through `0x1401b9fa0`. A match stores the string `Root` from VA `0x1401c5f10` as a startup sentinel. **It does not consume a following root-directory argument.** In this branch, `FPilot.exe --root PATH` has no recognized three-argument handling in the inspected entry function.

If `--root` does not match, D checks `--uninstall`, then `--uninstallex`, then processes the same argument as a path candidate. It queries that candidate through `0x1400cbd50` and stores startup path/token state only when its result byte at offset 1 is nonzero. A flag-dependent branch transforms the path with `0x140067a00` and derives a residual token. Directory-versus-file interpretation is inferred until those helpers are inspected.

I later recognizes the `Root` sentinel and substitutes a zero-length path before requesting navigation. This supports a virtual/root target rather than a configurable filesystem root. Matching case sensitivity, relative-path resolution, malformed argument behavior, and the exact contents of the root listing remain unknown. No claims about startup window appearance follow from this evidence.
