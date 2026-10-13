# File Pilot context-menu analysis

## Result

File Pilot uses Windows shell context-menu providers as its command source, converts the resulting native menu into an application-owned tree, and dispatches common commands to its own handlers. Unhandled shell commands generally fall back to `IContextMenu::InvokeCommand`; additional branches handle special menu records internally.

The searchable, styled popup in the supplied screenshot is therefore best understood as File Pilot's presentation of shell commands, not a fixed list of integrations hardcoded for VS Code, Git, PowerRename, and similar applications. This presentation conclusion is an inference from the screenshot, internal menu records, text-edit identifier, and command-dispatch code. The actual drawing routine and search-matching algorithm were not fully traced.

## Target and analysis boundary

- Target: `C:\SANDBOX\REPOS\rev-filepilot\FPilot.exe`
- Size: 2,184,784 bytes. Native x86-64 PE32+; no CLR runtime header.
- SHA-256: `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e`
- Static inspection only. The executable was not launched or modified.
- REA opened the target and inspected its complete bounded PE resource tree. Native REA analysis failed with `provider_unavailable`: `GHIDRA_INSTALL_DIR` is unset, Hopper does not support this host, and IDA is not configured. A full JDK 21 was detected.
- Subsequent code evidence came from the already-installed LLVM disassembler and GNU strings, not from REA decompilation. Function labels below are identified from embedded diagnostics or assigned by the analyst; they are not recovered public symbols.
- Addresses are preferred-image virtual addresses with image base `0x140000000`, not observed runtime addresses.

## Observed flow

### 1. Obtain a shell context-menu object

`0x1400c3e40` references the diagnostic name `P_GetContextMenu` and the message `P_GetContextMenu for dir path '%S', file paths count: %llu`.

Its branches distinguish an empty selection from a file selection. The empty-selection route requests `IShellView`, then calls its `GetItemObject` slot with selector zero and `IID_IContextMenu`, consistent with a folder-background menu. A selected-item route calls helper `0x1401a16e0`, which parses child names and calls the `IShellFolder::GetUIObjectOf` slot using the requested interface IID and item list. This is static interface/ABI identification, not an observed Windows execution.

The binary contains `IID_IContextMenu` at `0x1401d6aa8`, with bytes `e414020000000000c000000000000046`.

### 2. Ask the shell to populate an HMENU

At `0x1401a45f0`:

- `0x1401a45ff` calls imported `CreatePopupMenu`.
- `0x1401a466e` calls the stored context-menu object's vtable slot `+0x18`, matching `IContextMenu::QueryContextMenu`.
- Its arguments set insertion index zero, first command ID zero, and last command ID `0x7fff`. Flags depend on stored state.
- It queries `IContextMenu2` and `IContextMenu3`, storing optional interfaces.
- `0x1401a47c5` calls the application menu builder at `0x14018e300`.

Installed shell extensions can contribute commands through this interface. Which extensions actually supplied the screenshot's individual rows was not observed at runtime.

### 3. Convert native items into application menu records

`0x14018e300` references the diagnostic name `Win_ContextMenuBuild`.

- `0x14018e330`: `GetMenuItemCount`.
- `0x14018e402`: `GetMenuItemInfoW`, with a mask requesting item type, state, ID, submenu, and bitmap.
- `0x14018e528`: `GetMenuStringW` by position.
- It processes the Windows mnemonic marker `&` and separates tab-delimited label text.
- `0x14018e88c`: `IContextMenu::GetCommandString` with `GCS_VERBW`, value 4. If this fails or yields an empty string, `0x14018ea5d` tries `GCS_VERBA`, value 0.
- It stores command IDs, label/verb data, disabled state, parent/child/sibling links, submenu information, and bitmap-related data in application-owned records.
- The bitmap path calls `GetObjectW` at `0x14018eff0` and an internal conversion helper at `0x14018f115`. Exact texture-upload and draw behavior remains untraced.

At the root level, identified verbs `groupby`, `pintohome`, `viewcustomwizard`, `undo`, and `redo` take a skip branch. `windows.share` is skipped when the stored selection pointer is null. The verbs `view` and `arrange` are specially marked; do not infer their complete custom submenu behavior from those markers alone.

Paste and paste-link also receive application clipboard-state checks. This can affect their disabled state beyond the shell-provided state.

### 4. Initialize dynamic submenus

Helper `0x1400c7550` obtains the submenu with `GetSubMenu` at `0x1400c75a2`.

It then forwards message `0x117`, `WM_INITMENUPOPUP`, to:

- `IContextMenu3::HandleMenuMsg2` at `0x1400c75e2`, when available; otherwise
- `IContextMenu2::HandleMenuMsg` at `0x1400c75ff`.

It calls `Win_ContextMenuBuild` on that submenu at `0x1400c7617`. Records have a built flag and the helper can recurse through already-built children. This supports delayed shell submenu population. Exact expansion timing depends on the caller and was not fully mapped.

### 5. Dispatch the selected command

The classifier at `0x14018f230` assigns internal command kinds by canonical verb:

| Kind | Verb |
| --- | --- |
| 1 | refresh |
| 2 | open |
| 3 | opencontaining |
| 4 | copy |
| 5 | cut |
| 6 | paste |
| 7 | pastelink |
| 8 | delete |
| 9 | rename |
| 11 | newlink |

`0x14003efc0` first checks the disabled flag. For an active file-panel context, kinds 1 through 9 branch to distinct internal File Pilot routines rather than directly invoking the shell. For example, kind 1 calls `0x140071bd0` and kind 6 calls `0x1400741d0`. These identifications establish routing, not the complete implementation of each file operation.

Children of records specially marked `view` or `arrange` take another internal branch at `0x14003f2bf`, rather than shell invocation. Without an active file-panel context, some command kinds also exit without invoking the shell.

The general shell fallback calls `0x1400c2bf0` at `0x14003f2ad`. That function references the diagnostic `P_ContextMenuInvoke` and builds a 104-byte structure matching `CMINVOKECOMMANDINFOEX`, including the owner window, numeric command selector, and directory-related strings. At `0x1400c2d59`, it calls the stored context-menu interface's `+0x20` slot, matching `IContextMenu::InvokeCommand`.

Consequently, third-party commands can use their Windows shell implementations while common file-management actions use File Pilot's own behavior.

## Search and shortcut presentation

The binary contains `Select a system command...`, `ContextMenu_TextEdit`, context-menu activation/cancellation/expansion hotkey identifiers, and `ContextMenuBookmarks`. The text-edit identifier has a reference at `0x140046221`. Together with the screenshot, these support an application-owned searchable menu with extra keyboard UI.

A useful next search/UI entry is `0x140040560`. It clears a displayed-result list, has an empty versus nonempty text branch, invokes recursive submenu preparation in the nonempty branch, and eventually calls a sort-like helper with 16-byte records and comparator `0x1400178a0`. This identifies a candidate result-building path, not the complete search algorithm.

Not yet established: search scoring, exact key-badge assignment, full popup rendering, the reason PowerRename appears twice, owner-drawn extension compatibility, or the specific registered extensions contributing each row.

## Evidence files

- [REA resource Evidence](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea-pe-resources.json), ID `ev_02edf4ca7cc68b0c75516ac8af184540cc9ee808e0d3a3999f9ee4ee0974a8f6`. Complete resource inventory contains icons, RCDATA, group icon, version, and manifest resources; no RT_MENU entry. This does not by itself prove custom rendering.
- [PE headers and imports](C:/SANDBOX/REPOS/rev-filepilot/analysis/pe-imports.txt).
- [Focused LLVM disassembly](C:/SANDBOX/REPOS/rev-filepilot/analysis/context-menu-assembly.txt).
- [Relevant strings with hexadecimal file offsets](C:/SANDBOX/REPOS/rev-filepilot/analysis/context-menu-strings.txt).
- [Verified interface GUID bytes and canonical verb locations](C:/SANDBOX/REPOS/rev-filepilot/analysis/context-menu-identities.txt).

The imports include shell-object APIs and menu enumeration APIs. `TrackPopupMenu` is not statically imported. That absence is bounded evidence only; dynamically resolved APIs have not been exhaustively ruled out.

## Next step for REA decompilation

Configure an existing Ghidra 12.1.x installation through `GHIDRA_INSTALL_DIR`, then restart the REA MCP server so it receives that environment variable. The detected full JDK 21 already satisfies the Java prerequisite. Resume at the addresses above to recover pseudocode and trace rendering/search without rediscovering the menu flow.
