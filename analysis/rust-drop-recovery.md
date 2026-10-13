# Incoming OLE drop and Recycle Bin recovery

## Scope and evidence

Saved static Ghidra 12.1.4 evidence only. No REA native/lifecycle calls or application execution occurred in this pass.
Target `FPilot.exe`, SHA-256 `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e`.
This supplies reconstruction observations, not original Rust, compiled Rust, external-drag compatibility, or complete behavior recovery.
Pseudocode below was read from each JSON's `normalized_result`. The Recycle Bin `.c` matches its normalized pseudocode after trimming.

Evidence aliases used throughout:
- E = `ev_baee6cb2d13c76b6bbfa8f9a1157dda50d2f2883f56ccd2ea821f6495ab633f8`, `queries-09-drop-enter.json`, `0x140195a70`.
- O = `ev_7df53f53f33e65363ed24daf3037159b5ec9403bb72b73b27d52a2389a30d29b`, `queries-09-drop-over.json`, `0x140195c90`.
- L = `ev_283303a8f8bd3b208fa945c608b924dfb9c27dd4ca71ff294d3271337d5c4a61`, `queries-09-drop-leave.json`, `0x140195bd0`.
- D = `ev_2dd3f7d6becd89ded56f060f9e9b442d553e7889863a94a0a02ffc0be9bbc7ef`, `queries-09-drop-commit.json`, `0x140196070`.
- R = `ev_39066d9daa85548651bb4dc4d5824c553222ddda77767f7b9079072ac0016beb`, `queries-08-recycle-enumeration-function.json/.c`, `0x1401a86b0`.
- I = `ev_25501f6e8b3bbdc6296f97aee4b3c40e1c75e6d8c0fc6e27734503d8a6d050d1`, `queries-03-data-location.json/.c`.
All evidence filenames above are under `C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/`.

## Callback identity and state

I, `.c:940-954`, stores seven function pointers at application-state offsets `+0x8a0..+0x8d0`.
Their order is `0x1401961c0`, `0x140195a60`, `0x140196220`, E, O, L, D.
Under the supplied IDropTarget interpretation these are QueryInterface/AddRef/Release/DragEnter/DragOver/DragLeave/Drop.
The first three bodies were not inspected here. E/O/L/D signatures and indirect-call offsets corroborate the four callback roles.
Use `S = DAT_1401e8b90` for the application-state pointer, not the global's address itself.
I stores the window value from `S+0x9c8` at `S+0x8f0` and the vtable pointer at `S+0x8d8`.
E/O/D consult an optional interface at `this+0x10`; its calls match an IDropTargetHelper-shaped sequence, but its IID/creation was not inspected.
The interface at `S+0x920` has IDropTarget-shaped DragOver/DragLeave/Drop slots. Its acquisition belongs to uninspected helper code.

## Enter, modifiers, formats and coordinates

E, `0x140195aaf..0x140195b12`, sets byte `S+0x9a8=1`, effect-state DWORD `S+0x8fc=7`, and caller `*pdwEffect=7`.
It stores the entering data object at `S+0x958` and calls its vtable `+8`, consistent with AddRef.
The input effect mask is overwritten rather than intersected in this wrapper. Later helpers may change `S+0x8fc`; that is unresolved.
E stores `S+0x9a9 = ((keyState & 3) == 2)`, consistent with right-button down and left-button up, not a recovered right-drop menu policy.
E and O compute `S+0x9aa = ((keyState & 4) ? 1 : 0) | ((keyState >> 2) & 2) | ((keyState & 0x20) ? 4 : 0)`.
By Windows OLE bit conventions this compactly records Shift=1, Control=2, Alt=4. It does not establish copy/move/link precedence.
E, `0x140195b1f..0x140195b47`, conditionally installs hook type `0xe` with `0x140194fa0`, null module and thread zero, then calls `0x1401949d0(S+0x968,dataObject)`.
Type `0xe` corresponds to WH_MOUSE_LL; installation success and hook behavior are not observed.
No QueryGetData/GetData, FORMATETC, clipboard-format ID, CF_HDROP parser, or virtual-file decoder appears directly in these four bodies.
Incoming format acceptance/extraction remains unresolved behind `0x1401949d0` and downstream helpers. Do not import outgoing/clipboard findings as incoming acceptance proof.
E zeroes `0x1b8` bytes and calls `0x1401a9d60(S,2,payload)` before the optional helper-shaped DragEnter.
At `0x140195b79..0x140195b9d` that helper receives the window, data object, address of a copied packed point, and current output effect.
O and D likewise pass the packed point by value to delegated IDropTarget-shaped calls and by address to helper-shaped calls.
None of E/O/L/D directly calls ScreenToClient or otherwise visibly converts the point.
Therefore screen-to-client conversion and UI hit-testing are unresolved inside `0x1401a9d60`, `0x140194d20`, or related code; do not assert they happen here.
Ghidra incorrectly-looking pointer types for POINTL-bearing parameters are ABI/decompiler artifacts, not proof of pointer-valued OLE coordinates.

## DragOver effect selection and handoff

O, `0x140195d19..0x140195d2a`, zeroes `0x1b8` bytes and calls `0x1401a9d60(S,3,payload)`.
It then reads a selector named `local_e8` in pseudocode. These reads support a request/output handoff, but do not prove a thread switch or async message.
If a cached target exists, selector 1 plus nonzero `0x1401b87a0` comparison preserves it.
Otherwise `0x140195d81..0x140195da8` invokes cached target vtable `+0x28` then `+0x10`, consistent with DragLeave and Release, and clears target/associated 16-byte data.
A transition away from previous selector 2 calls `0x1401950b0(dataObject,0,zero16,zero16)` at `0x140195de2`.
O records the new selector at `S+0x950` and caches an ID at `S+0x92c` plus two 16-byte values at `S+0x930` and `S+0x940`.
`0x1401b87a0` compares those values; `0x1401b8530` copies/rebuilds them using `S+0x968`. Their actual types and comparison semantics are not recovered here.
Selector 1 stores key state at `S+0x928`. If payload `iStack_a4 == 1`, it clears bit 0 and sets bit 1, consistent with synthesizing right-button state.
At `0x140195f67`, it calls `0x140194d20(S+0x8a0,1,value16,keyState,packedPoint)`.
If a target is then present, `0x140195f98` invokes its vtable `+0x20` with key state, point, and effect initialized from `S+0x8fc`.
Only a nonnegative HRESULT admits that target's output effect; a missing target or negative HRESULT leaves effect zero.
Selector 2 instead computes exactly `payload.local_a8 & 7`. On a changed cached ID/value pair it also calls `0x1401950b0` with data object, ID and two values.
Other selectors leave effect zero. No explicit default move/copy choice or modifier priority is visible.
At `0x140196035` O writes the chosen effect, then optionally calls helper-shaped vtable `+0x28` with point-address/effect. O returns zero regardless of delegated HRESULT.

## Leave and commit

L, `0x140195c02..0x140195c12`, sends cached-target DragLeave/Release-shaped calls and clears `S+0x920`.
It calls optional helper-shaped `+0x20`, unhooks and clears `S+0x960`, calls `0x140194940(S+0x8a0)`, then dispatches zeroed payload event 4 at `0x140195c70`. L returns zero.
Data-object release/reset on Leave may occur in `0x140194940`; L does not show it directly.
D first unhooks, sets `S+0x9ab=1`, and calls optional helper-shaped Leave before delegating Drop.
At `0x140196108` cached-target vtable `+0x30` receives the incoming data object, cached key state `S+0x928`, packed point, and effect initialized from `S+0x8fc`.
The Drop callback's current key-state parameter is unused in the displayed body. Last DragOver-derived state, including its possible button rewrite, supplies the delegated key state.
D returns the delegated HRESULT, retaining effect only for nonnegative HRESULT; absent target produces HRESULT zero/effect zero.
It releases/clears the cached target, writes the effect, then optionally calls helper-shaped Drop with object/point-address/effect at `0x14019615b`.
It unconditionally Release-calls stored `S+0x958`, clears it, and dispatches zeroed payload event 5 at `0x140196195` after releasing the data object.
No direct file copy/move/link implementation or selector-2 transfer appears in D. Actual transfer/UI completion requires inspecting the event dispatcher and helpers.

## Recycle Bin enumeration pattern

R identifies itself with the diagnostic string `Win_LoadRecycleBinProc`; this establishes the feature context without resolving data GUID bytes.
At `0x1401a87d1`, SHGetKnownFolderItem receives GUID address `0x1401d6bb8`, flags/token zero, IID address `0x1401d6d18`.
The returned object's `+0x18` call at `0x1401a8807` uses handler address `0x1401d6bc8` and IID `0x1401d6d38`, matching BindToHandler-shaped enumeration setup.
Enumerator `+0x18` calls request one item with null fetched-count; the loop continues only while HRESULT equals zero. Item QueryInterface uses IID address `0x1401d6d28`.
These are IShellItem/IEnumShellItems/IShellItem2-shaped patterns, not independently resolved interface GUID identities.
Per-item `+0x30` at `0x1401a8a97` requests attributes mask `0x20091000`; returned bits populate custom flags and separate counters by custom bit 2.
`0x140197060` receives display-name-like flags `0x80080001` and `0x80058000`; its conversion and fallback logic remain uninspected.
Property-shaped `+0x68` calls use key addresses `0x1401d6e38`, `0x1401d6e50`, `0x1401d6e20`, and `0x1401d6fb8`.
Three successful property payloads populate record offsets `+0x30/+0x38/+0x40`; PropVariantClear follows successful reads. Another payload goes through `0x1401b8680` into `+0x48/+0x50`.
Two stack property keys use FMTID `{9b174b33-40ff-11d2-a27e-00c04fc30871}` with PIDs 2 and 3, passed through `0x1401971a0`.
The PID-3 helper result is parsed at fixed date/time positions into SYSTEMTIME, then SystemTimeToFileTime conditionally stores a FILETIME. Semantic property names and timezone policy are not established here.
A separate helper result using key address `0x1401d7000` is parsed as leading decimal digits into record `+0x28`; do not label it size without resolving the key/helper.
R allocates linked, 8-byte-aligned `0x78`-byte records using application arenas and increments counters; this is custom storage, not evidence of a filesystem-only listing.
No inspected GUID bytes, item-action dispatch, restore/delete behavior, persistence, or broad virtual-folder support was recovered. Display formatting and malformed-property handling remain helper-dependent.

## Coverage limits and next inspection targets

All signatures have low-confidence decompiler parameter/return types, default signature provenance, and non-compilable non-source pseudocode.
Ghidra body ranges end in RET for E/O/L/D/R at `0x140195bc0/0x140196067/0x140195c8b/0x1401961b9/0x1401a90be` respectively.
No terminal reference edge appears in these dossiers. The profile retains `ghidra-terminal-call-limitations-v1` and `no_return_repair=disabled-windows-p0`; RET/body coverage is not proof of semantic completeness or callees' return behavior.
E/O/L/D value flow is untruncated, with 101/743/62/130 retained operations and zero reported omitted operations/inputs/edges.
R retains 3000 operations, `truncated=true`, omitted-operation lower bound 1, known omitted inputs/edges zero. Those zero counts do not bound edges hidden in omitted operations.
High-p-code def-use does not cross functions and can omit aliases, call side effects and unresolved indirect flow. `is_dead` is an AST flag, not execution reachability; memory effects include temporaries, not business semantics.
Unresolved computed/indirect targetless flows are absent from reference edges. Inferred COM roles must remain separate from resolved interface identity.
Priority incoming helpers still uninspected here: `0x1401949d0` format/data setup, `0x140194d20` target acquisition, `0x1401950b0` selector-2 data notification, `0x140194940` cleanup, `0x140194fa0` mouse hook, `0x1401a9d60` event/UI dispatch, `0x1401b8530/0x1401b87a0` value copy/comparison.
Recycle follow-ups are GUID/property-key bytes and `0x140197060/0x1401971a0/0x1401b8680` conversions. No parent-owned clipboard/outgoing-drag/command/menu/COM-worker bodies were inspected.
