# FPilot.exe static literal findings

## Scope and evidence

Inspected only local bytes, existing PE imports, and the existing REA resource inventory. No target execution, web access, decompilation, or REA lifecycle calls.

- Target: `C:/SANDBOX/REPOS/rev-filepilot/FPilot.exe`
- SHA-256: `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e`
- Size: 2,184,784 bytes.
- Full extraction: `C:/SANDBOX/REPOS/rev-filepilot/analysis/literal-inventory.txt`
- Existing resources: `C:/SANDBOX/REPOS/rev-filepilot/analysis/rea-pe-resources.json`, REA evidence `ev_02edf4ca7cc68b0c75516ac8af184540cc9ee808e0d3a3999f9ee4ee0974a8f6`.
- Existing import metadata: `C:/SANDBOX/REPOS/rev-filepilot/analysis/pe-imports.txt`.

All offsets below are hexadecimal file offsets, not virtual addresses. For `.rdata`, RVA equals file offset plus `0x1400`; preferred VA equals `0x140000000` plus RVA. The inventory includes both addresses per string.

The inventory uses GNU strings 2.42, ASCII in `.rdata` and `.data`, and printable ASCII UTF-16LE across the entire file, with a minimum length of two. A focused NUL-terminated ASCII scan includes single-character key tokens. The PE version tree and manifest were decoded directly from resource byte ranges. The broad extraction retains incidental printable bytes and must not be treated as a curated command table.

## Exact identity and version

These are declared resource values, not inferred application behavior.

| Field | Value | Value file offset |
| --- | --- | --- |
| Fixed file version | `0.7.0.0` | Fixed structure at `0x211ebc` |
| Fixed product version | `0.7.0.0` | Fixed structure at `0x211ebc` |
| FileVersion string | `0.7.0.0` | `0x211fc0` |
| ProductVersion string | `0.7.0.0` | `0x2120a4` |
| CompanyName | `Voidstar` | `0x211f4c` |
| FileDescription | `File Pilot` | `0x211f88` |
| ProductName | `File Pilot` | `0x212068` |
| OriginalFilename | `FPilot.exe` | `0x212030` |
| LegalCopyright | `Voidstar` | `0x211ff4` |

Version translation bytes `09 04 b0 04` declare language `0x0409`, US English, and code page `0x04b0`, 1200. The resource tree uses language ID 1033 and directory code page 1252. These are different metadata fields, not a contradiction.

Separate ASCII display literals: `v0.7.0` at `0x1cdf1c`, `Feb 10 2026` at `0x1cdf78`, and `[Beta]` at `0x1cdf84`. PE timestamp in the existing import report reads February 10, 2026. The date literals and timestamp do not independently establish a release date.

## Resource declarations

REA lists nine resources: five RT_ICON payloads, one RT_RCDATA, one RT_GROUP_ICON, one RT_VERSION, and one RT_MANIFEST. There are no RT_MENU, RT_DIALOG, RT_STRING, or RT_ACCELERATOR entries in this resource tree. User-facing English literals instead occur in `.rdata`.

- Icon group 101, `0x211e48`, references five 32-bit images sized 16, 32, 48, 64, and 128 pixels. No runtime use was observed.
- RCDATA 102, `0x207d40`, 41,224 bytes, remains opaque. No payload-purpose claim is made.
- Version resource 1, `0x211e94`, 612 bytes.
- Manifest resource 1, `0x2120f8`, 864 bytes, declares `requestedExecutionLevel="asInvoker"`, `uiAccess="false"`, `longPathAware=true`, and an amd64 Microsoft.Windows.Common-Controls dependency version `6.0.0.0`.

The manifest does not establish effective privileges, long-path behavior, or whether the dependency is used.

## User-facing command literal groups

Found 263 distinct `HK_` identifier literals after excluding `_Tooltip` and `Short` suffixes. The main block contains 250 identifiers between `0x1c58b8` and `0x1cd2f0`. Thirteen more appear between `0x1d0538` and `0x1d0638`. Every identifier and offset is listed in the inventory.

These labels and tooltips describe candidate specification topics. They do not prove the command runs, establish its binding, or define verified interactions.

| Literal group | Examples of identifiers and associated textual topics | File offset range |
| --- | --- | --- |
| Selection and file operations | `HK_SelectionActivate`, `HK_SelectionCopy`, `HK_SelectionCut`, `HK_SelectionPaste`, `HK_SelectionPasteLink`, `HK_SelectionTrash`, `HK_SelectionDelete`, `HK_SelectionDuplicate`, `HK_SelectionRename`, `HK_SelectionInspect`, `HK_SelectionRestore`, `HK_SelectionCopyAsPathSmart`; previous/next panel copy and move | `0x1c58b8` to `0x1c64e0` |
| Creation and folder listing | `HK_CreateFile`, `HK_CreateFolder`, `HK_ToggleExpandFolder`, relative/containing-path commands, folder/file visibility, folder sort order, content zoom | `0x1c6530` to `0x1c6ca0` |
| Views, filters, navigation | Details, List, Columns, Medium/Large/XL Icons; `HK_StartFileFilter`, `HK_StartExpandedSearch`, `HK_CalcDirsContents`, `HK_RefreshFolder`, back/forward/parent navigation; trash all/restore all; saved folder views and bookmarks | `0x1c6ce0` to `0x1c7810` |
| Dropdowns and sorting | Folder/type dropdowns; `HK_SortByName`, Size, DateCreated, DateModified, DateAccessed, DateDeleted, ItemCount, Type, OriginalLocation, FreeSpace, FileSystem, PercentFull, VolumeUsage, FolderUsage | `0x1c7858` to `0x1c84d8` |
| Inspector | `HK_InspectorToggle`, details/layout/contents-size controls, word wrap, line numbers, whitespace, text/folder filter, image grid/minimap/scale | `0x1c8518` to `0x1c8e80` |
| Panels and tabs | Right/bottom splits; next/previous/directional panel selection; equal sizing; new/duplicate/reorder/close/reopen tabs; numbered tab selection 1 through 8 and last tab; tab menu/search | `0x1c8ea0` to `0x1c9e80` |
| GoTo and command windows | GoTo navigation and selection, extra details, activate/cancel/exit, config create/open, Quick Access and group options, main menu, command palette, options window | `0x1c9ea0` to `0x1ca6c0` |
| Popup and context commands | Search Tabs, Context Menu, folder/type popup activate/cancel/exit, toggle/untoggle all types, Quick Commands | `0x1ca708` to `0x1ca970` |
| State, layouts, application controls | Clear bookmarks/recents/folder views; save/load layout; open native File Explorer or console; minimize/fullscreen/setup/close; font-size/spacing/extension/hidden/system/recents toggles | `0x1ca9c0` to `0x1cb460` |
| Rename insertion | `HK_RenameInsertIncrementalID`, CurrentDate, CreateDate, ModifiedDate; previous/next file rename | `0x1cb4a8` to `0x1cb800` |
| Text editing | Caret/selection by character or word; home/end; directional deletion; select/copy/cut/paste; uppercase/lowercase; autocomplete; undo/redo | `0x1cb848` to `0x1cc350` |
| List cursor and selection | `HK_ScrollSelect*`, `HK_ScrollMoveCursor*`, `HK_ScrollExtendSelect*`, page selection, centering, select all | `0x1cc378` to `0x1ccee0` |
| Panel/inspector filter controls | Revert/cancel/apply filter; checkbox cancel; viewport sizes; inspector left/right selection | `0x1ccf00` to `0x1cd318` |

Additional exact identifiers: `HK_SelectionCancel`, `HK_Popup_Apply`, `HK_PopupClose`, `HK_PopupButton1`, `HK_PopupButton2`, `HK_ContextMenuExpandGroup`, `HK_ContextMenuCollapseGroup`, `HK_CmdWindowExpandGroup`, `HK_CmdWindowCollapseGroup`, `HK_TextApply`, `HK_TextApply2`, `HK_TextCancel`, `HK_TextCancel2`.

Textual details worth checking against code:

- `0x1c5900`: selection activation tooltip says selected folders in a multiselection open in separate tabs.
- `0x1c66b0`: expand-folder tooltip says recursively flatten all subfiles and subfolders into one listing.
- `0x1c7b70`: sort-by-name tooltip says neighboring digits are treated as numbers.
- `0x1c96a0`: close-tab tooltip says the last tab closes its panel.
- `0x1c6308` and `0x1c6330`: `HK_SelectionInspect` has the label `Inspect`, but its tooltip says system properties window. This must not be silently equated with `HK_InspectorToggle`.
- `0x1c19c0`, `0x1c1988`, `0x1c1a28`: tooltips mention restoring defaults and unchangeable mouse hotkeys. They do not prove default values or restrictions.

Labels preserve search-like suffix text, for example `Filter^search`, `Trash^delete,remove`, and `Toggle Expand Folder^unfold,reveal,deep,hierarchy,directory`. Their parsing semantics remain unknown.

## Configuration-name literals

Exactly 40 adjacent option-name literals occur at `0x1cd338` through `0x1cd610`. No value, type, accepted range, or default is established by adjacency.

- Typography and appearance: `FontName`, `FontSize`, `InspectorFontName`, `InspectorFontSize`, `Spacing`, `Animations`, `RoundCorners`, `SystemColorScheme`, `ColorScheme`, `ColorSchemeLight`, `ColorSchemeDark`, `AlternatingRowColors`, `InactiveOpacity`.
- Paths and visibility: `ShowFileExtension`, `LongPathSupport`, `ShowShortenedPaths`, `ShowBreadcrumbs`, `ShowHiddenFiles`, `ShowSystemFiles`, `LoadThumbnails`.
- Recents and folder statistics: `StoreRecents`, `ShowRecents`, `SortRecents`, `CalcFolderSizes`, `CalcFolderSizesSSD`, `CalcFolderSizesHDD`, `CalcFolderSizesNetwork`.
- Input and scrolling: `ActOnPress`, `ShowCheckboxes`, `SingleClick`, `SingleClickFoldersOnly`, `HoverToSelect`, `UnderlineOnHover`, `DoubleClickToParent`, `Overscroll`, `ReverseScroll`, `CharacterKeyAction`.
- Layout and startup: `InspectorLayout`, `LoadOnStartup`, `NewTabShow`.

`Setting_VSync` and label `VSync` also occur at `0x1c4b60` and `0x1c4b70`, with tooltip literals at `0x1c1670` through `0x1c16e0`. `VSync` is absent from the adjacent 40-name block. Do not assume it is a persisted setting from that label alone.

Enum-like words include `true`, `false`, `Always`, `Never`, `LocalDisk`, `TriggerHotkey`, `TypeInFilter`, `SelectTypedItem`, `Auto`, `Hor`, `Vert`, `Root`, `Home`, `Current`, `LastSession`, `persistent`, `session`, `never`, `always`, and `localdisk`. Case and relationships require code verification.

Theme-name literals are Sirius, Pollux, Eridanus, Voidstar, Regulus, and Altair, with lowercase forms elsewhere. Color identifiers and tooltip descriptions include background, caption, border, foreground, text, folder/file, selection, match, hover, warning, disabled states, and hidden opacity. Hex color literals occur at `0x1d0084` through `0x1d0444`; they do not establish the selected/default scheme.

## JSON-like persistence text

The following are literal names and formatting templates, not a validated schema.

- Filenames: `\FPilot-Config.json` at `0x1d2720`, `\FPilot-UserData.json` at `0x1d2738`, `\FPilot-Session.json` at `0x1d2750`; related path templates at `0x1d27a0` through `0x1d27d0`.
- Root/path templates: `%S\Voidstar\FilePilot` at `0x1ce040`, `Voidstar\FilePilot` at `0x1d2408`. No storage root is inferred.
- Config-shaped text at `0x1d0488` through `0x1d0520`: `options`, `colors`, `hotkeys`, `version`, and capitalized output strings `"Options"`, `"Colors"`, `"Hotkeys"`; hotkey entry text `{ "Keys": [` and `], "Command": "%S" }`.
- User-data-shaped text at `0x1ce571` through `0x1ce898`: `Version`, `Layouts`, `Folders`, `Path`, `DriveFixed`, `DriveSerial`, `UncRoot`, `ZoomLevel`, `SortBy`, `FolderSort`, `Columns`, `Kind`, `SplitRatio`, `Recents`, `PathBookmarks`, `CommandBookmarks`, `ContextMenuBookmarks`, `InstallerPresented`, `InstallerPresentedVersion`, `Score`, `Decay`, `Ordinal`, `QuickAccessGroups`, `GotoGroups`, `ClosedCmdGroups`. `SepFilesFolders` is another standalone literal.
- Window/layout-shaped text at `0x1ce8b9` through `0x1cea31`: `Name`, `WindowX`, `WindowY`, `WindowWidth`, `WindowHeight`, `WindowMaximized`, `QuickAccessWidth`, `QuickAccessClosed`, `QuickAccessShowPaths`, `CommandsHideDetails`, `OptionsHideDetails`, `ShowAllRecents`, `Panels`, `SelectedPanel`.
- Panel-shaped text at `0x1cea48` through `0x1ced28`: `ID`, `Prev`, `Next`, `Parent`, `Child`, `NextTab`, `SplitRatio`, `Path`, `Expanded`, `Hidden`, `SortedBy`, `Filter`, `FilterSticky`, `HideFiles`, `HideDirs`, `FilterTypes`, `InspectorSplitRatio`, `InspectorLayout`, `InspectorDetails`, `InspectorDetailsExtra`, `InspectorWordWrap`, `InspectorHideLineNumbers`, `InspectorShowWhitespace`, `InspectorHideGrid`, `InspectorHideMinimap`, `InspectorSortedBy`, `InspectorFolderSort`, `InspectorZoomLevel`, `InspectorColumns`, `Vertical`.
- Group/session-shaped text: `Name`, `IsClosed`, `SortIndex` at `0x1ced51`; `InspectorSepFilesFolders` at `0x1cf018`; `Panel` at `0x1cf084`; `"Layout": {` at `0x1cf091`.

Warning labels at `0x1c3930` through `0x1c3c98` refer to corrupted or older-format config, user-data, and session files and copies being preserved. These are messages only. Backup creation, migrations, overwrite safety, and actual format versions remain unverified.

## Hotkey and CLI text

Chord literals occupy `0x1cd828` through `0x1cde98`, including Enter/KeypadEnter, Ctrl+C/X/V, Ctrl+Insert, Shift+Insert, Shift+Delete, Alt+Enter, Ctrl+Shift+C/N, Alt+Left/Right/Up/Down, Ctrl+Shift+1 through 6, Ctrl+T/N/W, Ctrl+Tab, Ctrl+PageUp/PageDown, Ctrl+1 through 9 and keypad variants, Shift+F10, Ctrl+Z, Ctrl+Shift+Z, and many directional selection chords. Single-letter candidates are retained in the focused scan. The inventory does not associate chords with commands.

Lowercase key-name literals at `0x1ce070` through `0x1ce2a0` include modifiers, navigation keys, punctuation, keypad0 through keypad9, keypad operators, mouse buttons, mousex1/mousex2, mouse wheel directions, menu, f1 through f24, and `mousedouble`. Punctuation includes a two-backslash literal at `0x1ce110`; no escape syntax is inferred.

UTF-16LE CLI switch literals: `--root` at `0x1d2420`, `--uninstall` at `0x1d2430`, `--uninstallex` at `0x1d2448`. Additional uninstall command fragments appear at `0x1d47d8` and `0x1d4810`. Switch parsing and effects remain unknown.

## Other user-facing literals

- Panel columns at `0x1bf4c0` through `0x1bf780`: name/type/size, folder/file counts, total items, created/modified/accessed/deleted dates, original location, free space, file system, percent full, volume usage, folder usage.
- Inspector labels at `0x1bf7c8` through `0x1bfb10`: path/type/size sums/dimensions/subfolders/subfiles/date metadata/line and type counts; encoding label `UTF-8`.
- Path options at `0x1bfb18` through `0x1bfcf0`: open, tab/window/right/bottom split, edit/copy path, compact paths, breadcrumbs.
- Confirmation/warning messages at `0x1c3438` through `0x1c3c98`: layout deletion/overwrite, quick-command restore/add/remove, pinned commands, clear recents/bookmarks/folder layouts, scheme/hotkey deletion, missing/offline/mismatched saved paths, corrupt/old data.
- Installer text at `0x1c4f90` through `0x1c5870`: standalone/installed version, install/replace/modify/uninstall, desktop shortcut, right-click entries, default folder handler, PATH, install location, close-other-instances, failed folder creation/copy. The label claims about Win+E and PATH are not proven integrations.
- Windows registration strings at `0x1d3df8` through `0x1d4860`: uninstall metadata, `Open in File Pilot`, `File Pilot here`, `Software\Classes\Directory\shell\OpenInFilePilot`, desktop/directory-background entries, Drive/Directory open commands, CLSID `{52205fd8-5dfb-447d-801a-d0b52f2e83e1}`, PATH/Environment, and theme keys. No registry mutations were observed.
- Context verb literals at `0x1d3bf0` through `0x1d3cb8`: `groupby`, `pintohome`, `viewcustomwizard`, `undo`, `redo`, `windows.share`, `newfolder`, `view`, `arrange`, `paste`, `pastelink`, `refresh`, `opencontaining`, `copy`, `rename`, `newlink`. These do not establish invocation or filtering policy.
- File-extension/name token block at `0x1bea50` through `0x1bedf4`: image, text/source/config, audio/video, executable/archive/database/document extensions. Image examples include png/jpeg/jpg/gif/tiff/tif/bmp/ico. SVG appears among text/XML-like tokens. Extension presence does not prove rendering, playback, editing, or archive support. One-character entries excluded by GNU minimum length are `c` at `0x1beb18`, `h` at `0x1beb1c`, `r` at `0x1beb58`, `m` at `0x1beb5c`, and `f` at `0x1beb60`.
- Debug/user messages mention log and dump filenames, fatal exceptions, directory watchers, unreadable folder counts, and incomplete size/count data. They do not establish recovery behavior or performance guarantees.

## Imports and remaining unknowns

The import table names KERNEL32, USER32, OPENGL32, GDI32, SHELL32, ole32, ADVAPI32, dwmapi, DWrite, COMCTL32, SHLWAPI, PROPSYS, NETAPI32, and MPR. Import names support only dependencies/API availability. They do not show a particular feature path executing. Several ordinal-only shell/common-control imports remain unnamed in the supplied report.

Still unknown from this pass: actual command dispatch and availability, command-to-hotkey bindings, setting defaults/types/ranges, schema version numbers and migration rules, filesystem safety and concurrency, search matching rules, effective startup/storage paths, installer/registry semantics, preview format support, and every runtime interaction. Parent decompilation should establish those separately.
