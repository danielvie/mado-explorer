# Mado Explorer

A file explorer with graphical and terminal interfaces for navigating locations, inspecting files, and managing filesystem entries. Both interfaces use the same location, selection and operation vocabulary.

## Language

**Location**:
A folder or system place that can be opened for browsing.
_Avoid_: Page, screen

**Pane**:
An independently navigable region containing tabs and the contents of its active tab.
_Avoid_: Panel, window

**Tab**:
A browsing context with its own location, navigation history, selection, filter, and scroll position.

**Focused pane**:
The pane receiving browsing commands and keyboard shortcuts. Its active tab is the target of navigation.
_Avoid_: Selected pane

**Folder finder**:
The location-navigation interface opened with Ctrl+O. It searches known locations and accepts paths, rather than searching file contents.
_Avoid_: Command palette, file search

**Filter**:
A constraint on the entries shown in the current folder. It does not change the folder or recursively search its descendants.

**Preview**:
A read-only view of a selected file's contents inside its pane.
_Avoid_: Editor, document tab

**Operation**:
A requested filesystem change with fixed sources and, where applicable, a destination. It may contain several items and may complete only partially.

**Conflict**:
A destination entry that prevents an operation from creating or replacing the requested entry without a user decision.

**Cancellation**:
A request to stop an operation before further changes. It does not reverse items already completed.
_Avoid_: Rollback, undo

**Trash**:
Recoverable deletion through the operating system's recycle facility. It is distinct from permanent deletion.
_Avoid_: Permanent delete
