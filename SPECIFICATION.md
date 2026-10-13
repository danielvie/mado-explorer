# File Pilot 0.7.0 application specification

## 1. Scope and evidence

This is a reverse-engineered specification of the supplied Windows application. It is not vendor documentation, recovered source code, or a claim of complete behavioral equivalence.

| Property | Supplied artifact |
| --- | --- |
| Executable | `C:\SANDBOX\REPOS\rev-filepilot\FPilot.exe` |
| Product and company | File Pilot, Voidstar |
| File/product version | `0.7.0.0` |
| Display strings | `v0.7.0`, `[Beta]`, `Feb 10 2026` |
| Format | Native Windows x86-64 PE32+ GUI executable |
| Size | 2,184,784 bytes |
| SHA-256 | `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e` |
| Analysis | REA 6.4.0 with Ghidra 12.1.4, static inspection only |

The date string is not independently verified as the release date. Identity comes from resource decoding and PE headers. See [literal findings](C:/SANDBOX/REPOS/rev-filepilot/analysis/literal-findings.md), [PE metadata](C:/SANDBOX/REPOS/rev-filepilot/analysis/pe-imports.txt), and E-resources.

Evidence classifications used throughout:

- **Code-backed**: recovered code, references, or typed bytes establish a static path. Execution and success are not implied.
- **Declared**: embedded labels, tooltips, or settings describe an intended feature. Its complete implementation is unverified.
- **Inferred**: interpretation from several observations, explicitly identified.
- **Unknown**: available evidence does not establish the behavior.

REA evidence aliases such as E-ops resolve to full Evidence IDs and saved JSON records in the [evidence index](C:/SANDBOX/REPOS/rev-filepilot/analysis/evidence-index.md). Supplementary shell extraction and Windows SDK interpretation are identified separately.

The executable was not launched, patched, or otherwise modified. No application file operations, registry changes, installation, or network requests were exercised. Ghidra's overview reports 1,553 analyzed procedures and 3,285 defined string objects. The nonempty regex inventory returned 3,282 matching objects. These are analysis counts, not feature counts. E-overview, E-strings.

## 2. Application purpose and state model

File Pilot is a Windows file manager organized around folder panels, tabs, a Quick Access area, command popups, and an optional inspector. This product model is inferred from command declarations, panel serialization, and tab/menu code. E-strings, E-panel-read, E-panel-write, E-tabs.

The recovered state model includes:

| Concept | Responsibility | Evidence strength |
| --- | --- | --- |
| Window/layout | Holds window geometry and arrangement of panels | Geometry field names declared; panel links code-backed |
| Panel | Holds a location, filter, sort order, columns, zoom, and inspector state | Code-backed serialization |
| Tab | Another panel-linked location using `NextTab` | Link serialization code-backed; interactions declared |
| Split container | Links parent/child panels and stores orientation and ratio | Code-backed serialization; exact tree rules unknown |
| Selection | Determines file-operation inputs and rename candidates | Code-backed operation/rename input handling |
| Inspector | Stores preview and metadata presentation state per panel | Code-backed persistence and image decode path |
| Command | Identified by strings such as `HK_SelectionCopy` | Identifiers declared; selected shell-command routing code-backed |
| Folder view | Saved sorting, columns, zoom, and visibility preferences | Declared; exact user-data schema incomplete |

`ID`, `Prev`, `Next`, `Parent`, `Child`, and `NextTab` are serialized links. The exact ownership rules, ordering, cycle rejection, and maximum nesting remain unknown. Do not interpret these fields as a verified original class hierarchy.

## 3. Navigation, panels, and tabs

The following is declared product behavior, not a tested interaction contract. E-strings and the literal inventory supply exact identifiers and addresses.

- Navigate backward, forward, to a parent, or to a containing location.
- Edit/copy a path; show breadcrumbs or shortened paths.
- Open a location in a new tab, new window, right split, or bottom split.
- Select previous/next/directional panels and equalize panel sizes.
- Create, duplicate, reorder, close, and reopen tabs; select numbered tabs; search tabs.
- Save/load window layouts and restore a startup location or previous session.
- Expand a folder into a flattened listing of descendant files and folders.
- Open selected folders from a multiselection in separate tabs, according to its activation tooltip.
- Close the containing panel when its final tab closes, according to its close-tab tooltip.

Tab rendering code at `0x1400fa420` traverses tab chains, computes widths bounded by UI-metric multiples 10 and 16, handles horizontal overflow controls, and calculates scroll adjustment for a marked tab. The panel composition body at `0x1400d20e0` selects tab state and calls tab/header and other UI children in separate phases. This supplies layout guidance, not the complete interaction contract. Exact drag targets, tab limits, focus transfer, inactive-tab retention, and close/reopen ordering remain unresolved. E-tabs, E-renderer-panel, E-renderer-metadata, E-header.

No inspected body establishes first/last-visible file indices, overscan, or offscreen file-row skipping. Scissor testing, shader rounding, tab overflow, and header-path scrolling do not establish file-list virtualization. Child `0x1400d9c20` is a focused follow-up candidate, not an identified file-list renderer.

The root location is treated as an empty-path target in inspected startup code. Its exact list of drives, network locations, and other virtual items is not established. E-init.

## 4. Folder listing, sorting, and filtering

### Declared views and metadata

View commands name Details, List, Columns, Medium Icons, Large Icons, and XL Icons. There are commands for hiding files/folders, changing folder-first ordering, changing content size, refreshing a folder, and calculating directory contents. E-strings.

Declared columns include name, type, size, folder/file/item counts, created/modified/accessed/deleted dates, original location, free space, filesystem, percent full, volume usage, and folder usage. Applicability varies by location; the labels do not prove that every column applies to ordinary folders.

Sorting commands correspond to these fields. The name-sort tooltip promises numeric treatment of adjacent digits. The comparator implementing that promise was not recovered, so exact natural-sort, locale, case, and tie-breaking rules remain unknown.

### Filters

The command vocabulary describes filename filtering, expanded search, file-type filtering, sticky filters, and apply/cancel/revert actions. Persisted panel records contain `Filter`, `FilterSticky`, `HideFiles`, `HideDirs`, and `FilterTypes`. E-strings, E-panel-read, E-panel-write.

Correction from the recovery follow-up: `0x14009de50`, saved under the misleading name `file-filter-ui`, initializes key/text tables. Its filter labels do not establish a file-list renderer or virtualization algorithm. E-text-table. Do not assume an indexed filesystem search, content search, regular expressions, glob syntax, or recursive search beyond the declared expanded-folder behavior.

The context-menu matcher at `0x140123060` accepts two candidate text spans, a parsed query-like structure, optional match-position outputs, and two integer outputs. It bounds long candidate spans around `0x400` bytes through a character-stepping helper, and chooses another matching path when its final parameter is nonzero. Context-menu search uses that alternate path. Complete query grammar, Unicode rules, ranking, and file-filter reuse remain unknown. E-matcher, E-matcher-alt, E-shell-search.

### Directory updates

The watcher procedure at `0x1401916a0` uses `ReadDirectoryChangesW` with subtree monitoring and notification mask `0x1f`. It selects synchronous handling for a drive classified as remote and overlapped handling for other drive types. It posts application message `0x8001` when its pending-change logic requests a main-thread wakeup. E-watcher.

The notification-read buffer is `0x8000` bytes. The code logs enumeration/notification failures and has explicit shutdown/handle cleanup. Directory load-result processing also exists at `0x14005b8e0`. These paths do not establish lossless monitoring, refresh latency, guaranteed recovery after disconnect, or folder-size accuracy. E-dir-load.

## 5. File operations

### Code-backed backend

The operation dispatcher at `0x1400c9cd0` copies destination, source paths, and optional per-item names into an owned operation record. Most operation kinds use a worker callback; kinds 8, 10, and 11 take a direct path, while kind 9 still uses the worker route. E-ops.

The Windows backend at `0x1400ca3b0` creates `IFileOperation`. Its embedded interface/class GUIDs match the Windows SDK. The table below combines that GUID match, x64 vtable positions, and the operation-name table. E-ops-nt, E-ops-guid, E-ops-labels, E-copy-label, E-move-label. SDK corroboration is in [Windows SDK mappings](C:/SANDBOX/REPOS/rev-filepilot/analysis/windows-sdk-mappings.txt).

| Internal kind | Name | Recovered backend action |
| --- | --- | --- |
| 0 | Copy | `IFileOperation::CopyItem` |
| 1 | Move | `IFileOperation::MoveItem` |
| 2 | Trash | `DeleteItem`, with `FOF_ALLOWUNDO` flag included |
| 3 | Delete | `DeleteItem`, without that allow-undo flag |
| 4 | Rename | `RenameItem`; a volume-label branch uses `SetVolumeLabelW` |
| 5 | CreateFolder | `NewItem` with directory attribute `0x10` |
| 6 | CreateFile | `NewItem` with normal-file attribute `0x80` |
| 7 | Shortcut | Shell-link construction and persistence to a `.lnk` path |
| 8 | TrashDelete | Recycle Bin shell context verb `delete` |
| 9 | TrashDeleteAll | `SHEmptyRecycleBinW` |
| 10 | TrashRestore | Recycle Bin shell context verb `undelete` |
| 11 | TrashRestoreAll | Same restore helper, with an empty selection record |

Queued `IFileOperation` work ends in `PerformOperations`. The code checks HRESULT failures and releases COM interfaces. It does not establish a transaction spanning all selected files, rollback, an application undo stack, cancellation reporting, or a guarantee that successful HRESULT means every requested item completed.

Same-folder copy has a name-generation branch containing ` - Copy`. Shortcut creation contains ` - Shortcut` and a bounded numbered-candidate loop. Complete collision naming, overwrite prompts, and error presentation are not specified by this pass.

### Declared user commands

Copy, cut, paste, paste shortcut, duplicate, rename, create file/folder, copy/move to neighboring panels, copy paths, trash, permanent delete, and restore are declared. Clipboard formats and selected copy/cut/paste paths now have code-backed evidence; the remaining command paths are not all traced. E-strings, E-copy-handler, E-cut-handler, E-paste.

### Recovered clipboard contract

The clipboard reader at `0x1400c36b0` optionally reads text and file paths. Text prefers `CF_UNICODETEXT`; the `CF_TEXT` fallback occurs when the Unicode format is unavailable, not merely when reading or locking its handle fails. File paths use `CF_HDROP`; this reader handles ANSI and UTF-16 lists according to the payload's wide-character flag. Registered format `Preferred DropEffect` sets its cut-state field when the stored value equals `DROPEFFECT_MOVE`, value 2. E-clipboard-read.

The writer at `0x1400ccf10` publishes optional Unicode text and `CF_HDROP`. Its file-list builder at `0x14018c3e0` creates movable zero-filled global memory with a 20-byte `DROPFILES` header, wide-character flag, and NUL-separated UTF-16 paths with a final terminator. A nonzero cut-state requests `Preferred DropEffect=2`. E-clipboard-write, E-clipboard-dropfiles.

The selected copy/cut handlers obtain panel selection data and call this writer. Cut sets the writer's leading cut-state field to 1. The paste handler reads file payloads, chooses an operation kind using cut state and a destination comparison, and enqueues work through the established operation dispatcher. It can clear a cut clipboard immediately after enqueueing, without a visible operation-completion check in this handler. This is not a guarantee of successful transfer or safe clipboard clearing after failure. E-copy-handler, E-cut-handler, E-paste.

A Rust implementation can follow the formats and call ordering, but must implement its own validated memory/encoding boundaries and clipboard ownership. The recovered reader's raw scans and the writer's failure paths are not a safe Rust contract.

Keep these distinctions:

- Trash and permanent Delete are separate commands and backend kinds.
- The command labelled `Inspect` describes opening the system properties window. It is distinct from toggling File Pilot's inspector.
- Shell extensions may expose their own rename or other commands. Their presence does not make them built-in File Pilot operations.

### Rename

`0x140169320` creates rename state from panel/list selection and installs a text-edit event callback. This proves a selection-aware rename UI path, not a complete batch-renaming algorithm. E-rename.

Incremental ID, current date, creation date, modification date, and previous/next-file rename actions are declared. Their formatting, numbering origin, duplicate-name detection, extension-selection policy, and preview/commit rules remain unknown. E-strings.

## 6. Windows shell context menu

The application asks a Windows `IContextMenu` object to populate an `HMENU`, then converts the items into its own linked records. It queries optional `IContextMenu2` and `IContextMenu3` interfaces. E-shell-query, E-shell-build.

Code-backed behavior:

1. Enumerate items, IDs, state, labels, submenus, and bitmap information.
2. Obtain canonical Unicode verbs, with an ANSI fallback where needed.
3. Remove the first mnemonic `&` and separate tab-delimited text.
4. Keep parent/child/sibling links, grouping state, and disabled state.
5. Filter selected root verbs and apply application clipboard checks to paste entries.
6. Prepare dynamic submenus through helper `0x1400c7550`.
7. Route common file-panel commands to internal handlers; route eligible other commands to the shell invocation helper.

Observed root filtering includes `groupby`, `pintohome`, `viewcustomwizard`, `undo`, and `redo`. `windows.share` has a selection-dependent skip. `view` and `arrange` receive special treatment; their full replacement-submenu contract is unknown.

The dispatch function rejects disabled records and handles submenu records separately. Common command kinds 1 through 9 use internal handlers when a file-panel context exists. Contextless handling differs, and some commands do nothing. E-shell-dispatch.

The shell fallback builds a structure consistent with `CMINVOKECOMMANDINFOEX` and invokes `IContextMenu::InvokeCommand`. E-shell-invoke.

### Search

`0x140040560` clears its result list. Empty input follows normal menu-building paths. Nonempty input prepares submenus once, visits non-submenu records, matches primary and auxiliary text, aggregates positive integer outputs, and sorts 16-byte result records. Disabled records are not excluded from matching, but remain non-invocable through the dispatch check. E-shell-search.

Ghidra still does not recognize `0x1400178a0` as a procedure, but REA decoded its complete seven-byte comparator through individual instruction and byte reads. It returns the second record's 32-bit field at `+8` minus the first record's field at `+8`. Together with the recovered sorting helper and score-record construction, this supports descending aggregate-score ordering for the Shell-menu result list. Equal scores compare as zero, with no secondary key. The inspected partitioning sort can swap equal-score elements, so global stability is not guaranteed. A deterministic name-based tiebreaker in Rust would be a new design choice. E-comparator-gap, E-comparator-load, E-comparator-sub, E-comparator-ret, E-comparator-bytes, E-sort.

The alternate matcher also exposes scoring constants. A visible full-match branch starts with `100 + 3 * (matched_count - candidate_length) - first_position`; adjacent matches add 200, a match at position zero adds 50, and gaps longer than five subtract `50 * (gap - 5)`. Preceding underscore/space adds 25; explicit letter-range boundary tests add 30. Primary scores accumulate directly; each secondary token score receives a rounded `0.75` weight before accumulation. Positions and lengths here are in converted-element units, not a proven byte or Unicode-character contract. These observations cover the inspected branches, not a complete query-language or Unicode contract. E-matcher-alt. Quoting, negation, wildcards, case mapping, alternate match enumeration, and global filename-filter reuse remain unresolved.

Application-owned menu presentation is inferred from these records and search state. The popup's pixels, key badges, search grammar, extension compatibility, and handling of owner-drawn menus were not verified. Installed third-party commands are system-dependent, not a fixed list of supported applications.

## 7. Inspector and preview

### Code-backed image decoder

The preview function at `0x1401a0640` uses Windows Imaging Component. Its GUIDs identify `CLSID_WICImagingFactory2`, `IWICImagingFactory`, and `GUID_WICPixelFormat32bppPRGBA`. SDK bytes corroborate this ABI interpretation. E-preview, E-preview-guid.

The path opens a decoder from a filename, requests frame zero, checks or converts the pixel format, queries dimensions, computes a nominal four-byte-per-pixel buffer using 32-bit multiplication, and copies pixels into application memory. It checks several HRESULTs and releases interfaces, but does not check the dimension-query HRESULT. This is a static qualification, not proof of a runtime crash or exploitable failure.

This establishes an image-preview backend, not a list of guaranteed formats. Available codecs, corrupt-image behavior, animated-image playback, orientation, color management, maximum dimensions, and memory limits are unknown.

### Declared inspector controls

- Show/hide inspector and additional metadata; choose right/below/automatic placement.
- Show path, types, size totals, image dimensions, subfolder/subfile counts, filesystem/free space, and date metadata.
- Preview text with configurable font, word wrap, line numbers, whitespace display, and filtering.
- Zoom images; switch fit/original scale; show a grid or minimap.
- Filter a folder preview and change its sorting/columns.

These controls are declared by labels/tooltips and partially corroborated by persisted inspector fields. E-strings, E-panel-read, E-panel-write. Text encoding detection, syntax highlighting, editing, media playback, archive browsing, PDF preview, and support for every embedded extension token are not established.

## 8. Commands, Quick Access, and customization

The executable contains 263 distinct `HK_` identifier candidates after the literal inventory excludes tooltip and short-label suffixes. This is not a verified count of reachable commands. The large registration function at `0x1400258e0` exceeded Ghidra's decompiler response buffer, but REA subsequently returned its assembly and metadata. Selected registration paths are therefore inspectable despite the pseudocode failure. E-command-assembly, E-command-metadata, E-strings and [literal inventory](C:/SANDBOX/REPOS/rev-filepilot/analysis/literal-inventory.txt).

Declared controls include a command palette, main menu, Quick Commands, Quick Access, GoTo, saved layouts, bookmarks, recents, group expansion/collapse, custom hotkeys, and color schemes.

The declarations also include console/File Explorer launch, fullscreen, minimize, setup, options, and application-close commands. Their process arguments, console choice, elevation rules, and complete dispatch were not traced.

Embedded key chords include familiar combinations such as `Ctrl+C`, `Ctrl+V`, `Shift+Delete`, `Ctrl+T`, and `Shift+F10`. Their presence alone does not establish default bindings. The follow-up combines registration assembly, original key-string bytes, record-builder behavior, and a matching handler to establish the following selected constructor mapping. The complete default map and effective bindings after user configuration remain unresolved. E-command-assembly, E-command-record, E-default-copy-keys, E-copy-handler.

| Command | Constructor key text | Handler |
| --- | --- | --- |
| `HK_SelectionCopy` | `Ctrl+C`, `Ctrl+Insert`, `Y` without a modifier prefix | `0x1400739b0` |

These texts are passed to the chord parser on the returned command records. They are stronger evidence than adjacent literals, but are not live-binding observations. The builder can append records sharing a command name; this does not establish a one-command/one-binding data model.

The config hotkey loader recognizes command-name records and a `Keys` field, looks up registered command records, and parses at most two array entries into key slots. The slot interaction and every conflict rule are not established. The chord parser separates nonempty components at `+`, uses earlier components as modifiers and the final component as a key, and calls a lookup that lowercases ASCII names before table lookup. This is implementation guidance, not a verified complete grammar. E-hotkey-config, E-hotkey-keys, E-hotkey-chord, E-hotkey-keynames.

Text command labels contain suffixes such as `Filter^search` and `Trash^delete,remove`. These look like search synonyms, but their parsing rules are not established.

## 9. Settings

The constructor at `0x14002f480` registers 40 setting descriptors and their initial values. These are built-in constructor defaults, not necessarily effective values after config loading. Integer units, most allowed ranges, UI controls, and application of each option are not fully established. E-settings.

| Group | Setting | Constructor default |
| --- | --- | --- |
| Fonts | `FontName` | `segoeui.ttf` |
| Fonts | `FontSize` | `9` |
| Fonts | `InspectorFontName` | `consola.ttf` |
| Fonts | `InspectorFontSize` | `10` |
| Appearance | `Spacing` | `4` |
| Appearance | `Animations` | `true` |
| Appearance | `RoundCorners` | `true` |
| Appearance | `SystemColorScheme` | `false` |
| Appearance | `ColorScheme` | `Voidstar` |
| Appearance | `ColorSchemeLight` | `Voidstar` |
| Appearance | `ColorSchemeDark` | `Voidstar` |
| Appearance | `AlternatingRowColors` | `false` |
| Appearance | `InactiveOpacity` | `60` |
| Paths | `ShowFileExtension` | `true` |
| Paths | `LongPathSupport` | `false` |
| Paths | `ShowShortenedPaths` | `true` |
| Paths | `ShowBreadcrumbs` | `true` |
| Visibility | `ShowHiddenFiles` | `false` |
| Visibility | `ShowSystemFiles` | `false` |
| Visibility | `LoadThumbnails` | `LocalDisk` |
| History | `StoreRecents` | `Persistent` |
| History | `ShowRecents` | `false` |
| History | `SortRecents` | `false` |
| Statistics | `CalcFolderSizes` | `true` |
| Statistics | `CalcFolderSizesSSD` | `true` |
| Statistics | `CalcFolderSizesHDD` | `false` |
| Statistics | `CalcFolderSizesNetwork` | `false` |
| Input | `ActOnPress` | `true` |
| Input | `ShowCheckboxes` | `true` |
| Input | `SingleClick` | `false` |
| Input | `SingleClickFoldersOnly` | `false` |
| Input | `HoverToSelect` | `false` |
| Input | `UnderlineOnHover` | `true` |
| Input | `DoubleClickToParent` | `false` |
| Scrolling | `Overscroll` | `true` |
| Scrolling | `ReverseScroll` | `false` |
| Input | `CharacterKeyAction` | `TypeInFilter` |
| Layout | `InspectorLayout` | `Auto` |
| Startup | `LoadOnStartup` | `LastSession` |
| Startup | `NewTabShow` | `Current` |

Constructed types are strings for font filenames, integers for sizes/spacing/opacity, choices for named modes/schemes, and boolean-like descriptors for the remaining toggles. This does not by itself establish the JSON encoding of every descriptor.

| Setting | Built-in choice list |
| --- | --- |
| Color scheme descriptors | `Sirius`, `Pollux`, `Eridanus`, `Voidstar`, `Regulus`, `Altair` |
| `LoadThumbnails` | `LocalDisk`, `Always`, `Never` |
| `StoreRecents` | `Persistent`, `Session`, `Never` |
| `CharacterKeyAction` | `TriggerHotkey`, `TypeInFilter`, `SelectTypedItem` |
| `InspectorLayout` | `Auto`, `Hor`, `Vert` |
| `LoadOnStartup` | `LastSession`, `Root`, `Home` |
| `NewTabShow` | `Current`, `Root`, `Home` |

The config reader supports named color-scheme objects. Their complete acceptance rules and any extension of other choice lists remain unverified. `VSync` appears as a label but is not among these 40 descriptors.

The manifest declares `longPathAware=true`, while the setting constructor initializes `LongPathSupport=false`. These are separate mechanisms, not evidence of a contradiction or effective long-path support.

## 10. Persistence and startup

### Storage files

The binary uses three separate filenames and named mutexes. E-storage, E-strings.

| File | Observed/declared role |
| --- | --- |
| `FPilot-Config.json` | Option/color/hotkey configuration; reader code recovered |
| `FPilot-UserData.json` | Labels/templates name layouts, folder views, recents, bookmarks, and group state; complete serializer unverified |
| `FPilot-Session.json` | Session/layout state; panel record reader/writer recovered |

Known-folder GUIDs match Roaming AppData for config/user-data roots and Local AppData for the session root. The inspected entry explicitly constructs the session suffix `\Voidstar\FilePilot\FPilot-Session.json`. Decompiler argument loss prevents establishing the complete default config/user-data filenames from that path-construction branch alone. E-storage, E-roaming, E-local; Windows SDK mappings.

The entry also constructs three alternative filename candidates from the executable-path flow and checks each independently. A qualifying candidate replaces only its corresponding storage path. `GetModuleFileNameW` is established, but intermediate transformation and existence-test helper semantics are not fully traced. Executable-adjacent portable storage is therefore inferred, not a complete verified portable-mode contract. E-exepath, E-storage.

Named mutex creation does not prove atomic writes, lock acquisition order, crash safety, or multi-instance consistency.

### Persisted panel schema

The panel writer emits JSON-shaped records; the matching reader consumes the fields below. This is a partial schema of a panel record, not a complete session schema. E-panel-read, E-panel-write.

| Fields | Recovered form |
| --- | --- |
| `ID` | Integer |
| `Prev`, `Next`, `Parent`, `Child`, `NextTab` | Integer links, omitted when zero |
| `SplitRatio` | Floating-point number |
| `Vertical` | Boolean for records without a path |
| `Path` | String for location-bearing records; serialized separators become `/` |
| `Expanded`, `Hidden` | Booleans |
| `SortedBy` | Integer |
| `FolderSort` | `FoldersFirst`, `FoldersLast`, or `Mixed` |
| `Filter` | String, omitted when empty |
| `FilterSticky`, `HideFiles`, `HideDirs` | Booleans |
| `ZoomLevel` | Floating-point number |
| `Columns` | Optional objects with `Kind` and `SplitRatio` |
| `FilterTypes` | Optional collection; reader consumes strings, writer element format unresolved |
| `InspectorSplitRatio`, `InspectorZoomLevel` | Floating-point numbers |
| `InspectorLayout`, `InspectorSortedBy` | Integers |
| `InspectorDetails`, `InspectorDetailsExtra`, `InspectorWordWrap` | Booleans |
| `InspectorHideLineNumbers`, `InspectorShowWhitespace`, `InspectorHideGrid`, `InspectorHideMinimap` | Booleans |
| `InspectorFolderSort`, `InspectorColumns` | Folder-order string and column objects |

The writer omits inspector fields when its layout value equals 4. Inspector sorting/zoom/columns have additional omission conditions. The reader clamps split ratios and zoom to `[0,1]`, sorting values to `[3,30]`, and loaded inspector layout to `[0,2]`. These internal limits do not establish displayed units or every valid runtime state.

The reader also accepts `SepFilesFolders` and `InspectorSepFilesFolders`, while the inspected writer emits folder-order strings. Complete format migration and compatibility are unknown.

### Configuration and failure handling

The config reader at `0x140039820` normalizes top-level ASCII keys to lowercase and recognizes `options`, `colors`, `hotkeys`, and `version`. It constructs defaults before applying known option descriptors, supports named color-scheme objects, and delegates hotkey parsing to the partially recovered loader described in section 8. E-hotkey-config. It compares a content fingerprint with the prior value before parsing. Exact string escapes, option-name matching, boolean/choice conversion, and strict JSON conformance remain unknown. E-config-read.

When an Options section is indexed, loaded `FontSize` and `InspectorFontSize` are clamped to `[8,16]`, `Spacing` to `[1,10]`, and `InactiveOpacity` to `[0,100]`. Integer parsing consumes a signed decimal prefix; whole-token numeric validation is not established. The config format-version field starts at 1 and parsed primitive values are clamped to at least 1. An upper supported-version or migration threshold was not established.

A deferred config-save path at `0x1400356f0` waits for elapsed-time and counter thresholds, constructs serialized text, records a hash, and passes it to a write helper. The counter's unit and the write helper's durability/replacement strategy are not verified. This is not an established backup routine, despite the saved evidence filename `config-recovery`. E-config-write.

Startup code accepts one config-loader status and falls back to constructed defaults for other statuses. Nonzero unsuccessful statuses trigger warning/save paths. These numeric statuses must not be mistaken for product or persistence-version numbers. E-init.

Labels describe corrupt/older config, user-data, and session files and copies being preserved. Backup creation, backup filenames, retention, migration, and rollback remain unknown. No corruption scenario was executed.

### Command line

The inspected entry uses `GetCommandLineW` and `CommandLineToArgvW`. Special processing occurs with exactly one argument after the executable. E-storage.

- `FPilot.exe --root` stores a `Root` startup sentinel, later translated to an empty-path target.
- `--root` does not consume a following directory argument in this branch. It is not established as `--root PATH`.
- `--uninstall` and `--uninstallex` have special branches. Their effects were not exercised.
- Another single argument is tested as a path candidate and can populate startup location/token state.

Multiple arguments, option case sensitivity, quoting edge cases, relative-path resolution, and file-versus-directory launch details are not fully established.

## 11. Platform and integration boundary

This supplied artifact is a native Windows x86-64 application. Its static dependencies include Windows file/process APIs, shell/COM APIs, registry APIs, GDI/OpenGL, DirectWrite, DWM, and network-share APIs. Dependency presence is not proof of a reachable feature. PE metadata, E-strings.

The follow-up now establishes a custom OpenGL initialization path at `0x1400666b0`, including blend/scissor configuration, shader creation/linking-shaped calls, attribute setup, and referenced GLSL 330 source. The evidence filename `clipboard-state-update` is misleading: this is renderer initialization, not clipboard logic. The full widget/layout/render loop is not reconstructed. E-renderer-init.

REA byte reads recovered complete embedded fragment and vertex shader text, including their original NUL terminators. Exact extracted text is retained as [fragment shader](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/filepilot-shape.frag.glsl) and [vertex shader](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/filepilot-shape.vert.glsl). These are shipped source strings, unlike decompiled native pseudocode. They describe textured quads, rounded-shape distance calculations, outline/dash masks, gradients, and alpha smoothing. No GPU compilation, visual fidelity, or permission to reuse proprietary source has been established. E-shader-fragment, E-shader-vertex.

The manifest requests `asInvoker`, disables `uiAccess`, and names Windows Common Controls v6. It does not promise administrative operation success, accessibility coverage, or a tested minimum Windows version. E-resources; literal findings.

Installation labels/registry strings describe desktop shortcuts, shell context entries, default folder handling, PATH registration, installation replacement, and uninstallation. Their implementation and rollback are not specified by this pass. No registry mutations were performed.

Do not infer plugins, cloud synchronization, FTP/SFTP, archive extraction, application updates, or a licensing protocol from this investigation. Their support or absence was not established.

## 12. Verification scenarios for a later runtime pass

These scenarios have not been run. Use an isolated test account or VM and disposable files before verifying mutations.

| Scenario | Expected subject to verify | Current evidence |
| --- | --- | --- |
| Start with empty settings | Compare effective defaults to descriptor defaults | Code-backed constructor only |
| Split a panel, create/reorder/close tabs | Arrangement, focus, last-tab close, serialized links | Declared interactions; code-backed links |
| Filter and expand a folder | Search scope, matching, sticky state, cancel/revert | Declared; persisted fields |
| Copy/move/duplicate with collisions | Naming, overwrite choices, partial failure, cancellation | Backend code-backed; UI contract unknown |
| Trash/delete/restore | Recycle versus permanent behavior, prompts, permissions | Distinct code-backed backend paths |
| Rename several files | Number/date insertion, extensions, preview/commit | Rename setup code; declared actions |
| Open/search a shell menu | Third-party commands, disabled state, dynamic submenus | Code-backed construction/search/dispatch |
| Preview images and text | Formats, limits, encoding, zoom/wrap, failure states | Image decoder code; declared controls |
| Restart after layout changes | Session restoration and per-folder preferences | Partial schema code-backed |
| Supply corrupt/old data | Backup, warning, fallback, migration, data preservation | Warning declarations and config control flow |
| Use local, removable, and network folders | Watcher behavior, disconnect/reconnect, stale metadata | Watcher paths code-backed |
| Run standalone and installed copies | Actual storage selection and integration changes | Partial storage code; installer declarations |

## 13. Coverage and remaining work

| Investigation question | Status |
| --- | --- |
| Artifact identity/version/resources | Answered statically |
| Functional vocabulary and 40 built-in setting defaults | Answered as declarations/default construction |
| Shell-menu construction, search pipeline, dispatch | Static pipeline, selected scoring branches, and score ordering recovered; query grammar/compatibility/rendering gaps |
| File-operation backend and operation-kind map | Answered statically; end-to-end UI/safety gaps |
| Directory notifications | Partially answered; update correctness and recovery gaps |
| Image-preview backend | Answered statically; supported formats/UI gaps |
| Panel persistence | Partially answered; whole-session/user-data schemas missing |
| Startup arguments/storage | Partially answered; path helpers/portable behavior gaps |
| Default hotkeys, full command registration | Assembly and selected constructor mappings recovered; complete map/effective runtime bindings unresolved |
| Clipboard and external drag/drop | Formats and selected handlers/callbacks recovered; complete UI action/virtual-file/runtime compatibility gaps |
| Custom renderer | Initialization and full embedded shader text recovered; complete file-list/widget behavior and pixel fidelity unresolved |
| Installer, accessibility, performance, runtime fidelity | Unresolved; no runtime pass |

REA records retain provider limitations, unresolved indirect calls, and value-flow truncation where reported. Pseudocode is decompiler output, not compilable original source. The menu builder also has a reported terminal-flow assumption at `0x14018e365`; do not infer nonreturning execution from it. Full executable behavior has not been reconstructed.

## 14. Reproducing and checking the analysis

The [batch runner](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea-batch.mjs) uses the existing REA registration with per-process Ghidra/JDK overrides. It admits only selected static query tools, writes complete records, exports successful sessions, and closes the owned native target. It never launches `FPilot.exe`.

The runner's paths reflect this workstation. Existing result files are protected from overwrite. To repeat a batch, copy its query JSON to a new basename. `queries-02`, `queries-03`, and `queries-04` encountered documented transport/decompiler/query failures; successful individual records remain usable. Later batches completed and closed their sessions. Transport-sized completed records were exported rather than silently truncated.

Available tasks in [Taskfile.yml](C:/SANDBOX/REPOS/rev-filepilot/Taskfile.yml):

```text
task test
task run BATCH=analysis/queries-06-copy.json
task clean
```

`run` runs analysis, not the application. The example assumes a copied query file with that new basename. `clean` removes only the regenerable REA snapshot, not evidence, specification, or the executable.

## 15. Recoverability for a Rust/Windows implementation

### What can be obtained

REA can recover assembly, C-like behavioral pseudocode, API call sequences, constants, branches, serialized field names, and relationships between routines. This supplies implementation guidance for an independent Rust version. It does not produce original native application source, a compilable Rust port, reliable original types, or the original build system.

The embedded GLSL shaders are a narrower exception: their text was shipped in the executable and was read directly, not reconstructed by a decompiler. Source recovery does not itself grant a reuse license. Native pseudocode still contains generated names, raw offsets, uncertain signatures, and unresolved helpers; copying it mechanically would preserve both decompiler mistakes and unchecked original behavior.

### Recovery matrix

| Required custom work | Obtained from this artifact | Remaining Rust work / evidence gap |
| --- | --- | --- |
| Shell context-menu adapter | Query/populate/convert/verb-route/invoke sequence; dynamic submenu messages | HWND/toolkit integration, extension compatibility, accessibility and runtime tests |
| File-operation backend | Operation kinds, `IFileOperation` queue/execute calls, recycle/restore routes | Typed COM ownership, cancellation/partial-failure policy, reliable outcome reporting |
| Clipboard adapter | Unicode text, ANSI fallback, `CF_HDROP`, `DROPFILES` packing, preferred move effect, selected copy/cut/paste handlers | Bounds/encoding validation, ownership/failure cleanup, completion-aware cut behavior |
| Outbound file dragging | Shell-supplied `IDataObject` paths and `SHDoDragDrop` with copy/move/link mask 7 | Toolkit mouse lifecycle, Shell object ownership, cross-application tests |
| Incoming file dropping | `IDropTarget` registration/callbacks, `CF_HDROP` decode helper, target acquisition, effect/state handoffs | Complete UI hit-testing/action policy, long paths, virtual-file formats, compatibility tests |
| Worker/COM setup | Threadpool scheduling and a callback that calls `OleInitialize` | Do not infer message pumping or safe cross-thread interface transfer; use an explicit supported COM lifetime/thread design |
| Command registry/hotkeys | Record construction, loader, chord/name parsing, registration assembly and selected mappings | Complete map, collision and multi-slot rules, effective user-config behavior |
| Searchable menu | Candidate collection, match-position/score paths, selected bonus/penalty constants, comparator and sort helper | Complete grammar/Unicode rules, equivalent matching in other search widgets |
| Tabs/panels | Tab-chain traversal, bounded tab-width calculations, overflow controls and marked-tab scroll adjustment; panel JSON fields | Exact click/drag/close/focus behavior and complete layout-graph restoration |
| Image preview/icons | WIC frame-to-pixel path and image-list/icon/bitmap conversion pattern | Codec coverage, caching, memory limits, text previews, DPI and malformed-input tests |
| Recycle Bin listing | Shell item enumeration and property-fetch patterns | Property identities/conversion rules and full virtual-folder model |
| Rendering | OpenGL initialization and complete embedded GLSL source | Rebuild the UI with the chosen Rust toolkit; full widget behavior, typography, accessibility and pixel fidelity are not recovered |
| Folder updates/settings | Directory watcher paths, 40 descriptor defaults, config/panel reading and writing | Notification recovery, full user-data/session schema, durable saving and migration |

The matrix is a qualitative assessment, not a percentage of recovered source or a proof that these areas are ready to compile. E-shell-query, E-shell-build, E-shell-submenus, E-shell-invoke, E-ops, E-ops-nt, E-clipboard-read, E-clipboard-write, E-clipboard-dropfiles, E-drag-register, E-drag-start, E-drop-data, E-drop-target, E-drop-enter, E-drop-over, E-drop-leave, E-drop-commit, E-worker-schedule, E-worker-com, E-command-record, E-command-assembly, E-hotkey-config, E-hotkey-chord, E-matcher-alt, E-sort, E-tabs, E-panel-read, E-panel-write, E-preview, E-shell-icon, E-bin-list, E-renderer-init, E-shader-fragment, E-shader-vertex, E-watcher, E-settings.

### Recovered Windows call patterns

Outbound dragging obtains `IDataObject` through Shell object interfaces rather than constructing a complete custom data object in the inspected start routine. It calls `SHDoDragDrop` with allowed effect mask 7 and releases the object afterward. Incoming registration attaches an application `IDropTarget` to the HWND and creates a Shell drag-image helper; helper-creation failure revokes registration. GUID bytes identify `IDataObject`, `IDropTarget`, and the drag helper interfaces. E-drag-start, E-drag-register, E-dataobject-guid, E-drop-target-guid, E-drophelper-guid, E-drop-class-guid.

The incoming reader requests `CF_HDROP` with global-memory storage, uses `DragQueryFileW`, converts paths, and releases `STGMEDIUM`. Its per-path buffer is fixed at `0x104` UTF-16 units in this routine. This proves a file-path reader, not support for every virtual-file/stream format. A Rust version should use bounded, length-aware extraction rather than adopt this fixed buffer. E-drop-data.

The target-acquisition helper parses a destination with `SHParseDisplayName`, binds its parent, requests a Shell drop-target interface, calls its DragEnter slot, and caches a successful target. DragOver and Drop can delegate to that target. An alternate UI event branch supplies an effect mask and drop-description data. Actual file-transfer correctness and every application-local drop action remain unverified. E-drop-target, E-drop-over, E-drop-commit, E-drag-description, E-drop-ui.

The worker callback calls `OleInitialize` before the operation callback, ignores its HRESULT in the inspected body, and has a tail resolved as `CoUninitialize`. Its displayed decompiler arguments are not a valid API signature. A message pump around arbitrary jobs and every apartment-bound interface lifetime are not established. It is evidence of the original pattern, not permission to pass arbitrary COM objects between Rust worker threads. E-worker-com.

The drag/UI dispatcher is an inline call, not a recovered cross-thread queue. Drag events 2 through 5 pass payloads synchronously through application hooks, including `0x1401afbc0` and `0x1401ad170`. Their destination hit-testing and transfer actions remain uninspected. The Drop callback passes the cached last-DragOver key state to a delegated Shell target rather than using its current key-state parameter. Exact modifier precedence, right-button policy, coordinate conversion, and application-local drop completion remain unresolved. E-drop-ui, E-drop-over, E-drop-commit.

### Implementation boundaries

- Reconstruct a typed Rust state model; do not use recovered memory offsets as public data structures.
- Reuse Windows Shell/WIC services through documented bindings where the evidence shows platform delegation.
- Rebuild widget presentation with the selected Rust UI toolkit rather than porting the entire original renderer.
- Add explicit memory/path/preview limits and failure handling, even where the inspected native code omits checks.
- Treat spec unknowns as design choices or questions requiring a controlled runtime pass, not recovered requirements.

No Rust implementation, Rust compilation, original-app execution, or reconstruction-equivalence test was performed in this follow-up. Evidence integrity and exact extracted shader bytes are checked by `task test`; those checks do not validate application behavior.

### Follow-up coverage and reports

Batches `queries-07` through `queries-11` retained 68 fresh successful observations, including 30 native function dossiers, one standalone panel pseudocode record, command-registration assembly, and raw-byte/instruction evidence. This follow-up had no saved query failures. All five owned sessions exported evidence and closed; older decompiler failures remain documented rather than erased.

Four parallel read-only reviews cover [Windows integration](C:/SANDBOX/REPOS/rev-filepilot/analysis/parallel-windows-recovery.md), [UI and shader recovery](C:/SANDBOX/REPOS/rev-filepilot/analysis/parallel-ui-recovery.md), [commands and search](C:/SANDBOX/REPOS/rev-filepilot/analysis/parallel-commands-recovery.md), and [retained-evidence limits](C:/SANDBOX/REPOS/rev-filepilot/analysis/parallel-recovery-audit.md). Each names its inspected bodies, bounded searches, exact Evidence IDs, and uninspected helpers.

Complete retention does not mean complete analysis. Recycle enumeration and the header dossier report truncated native value flow. Header/worker/window code also retains decompiler warnings about unreachable blocks, indirect-jump treatment, or unsettled types. The fragment shader's pseudocode display is clipped even though its separately read bytes and extracted text are complete. These limitations must survive any Rust reconstruction plan.
