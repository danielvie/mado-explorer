# Shell-menu findings

File Pilot 0.7.0, static Ghidra evidence, not execution. B/D/S cite exact records:

- B: `ev_a1236e9a8d634cdb625e45ee44e82c3c2fe869771269b1abccf1354e1ac700e5`, [builder](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/queries-01-shell-builder.json).
- D: `ev_77131e1e38fd8c999b90d629c119f88cffe4fa24f7bc840fb1ba6608e7e10b57`, [dispatch](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/queries-01-shell-dispatch.json).
- S: `ev_2671f325485b8692f2f8a5355a270f0885fb41f7d21ada00e6921d9ee7d9b092`, [search](C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/queries-01-shell-search.json).

## Observed specification

1. B `0x14018e300` enumerates an existing HMENU into linked application records. Separators increment grouping state. Labels lose only the first `&` and end before the first tab. IDs below `0x7fff` request Unicode verbs, then ANSI fallback. Root verb filtering, clipboard-based paste disabling, parent/sibling links, and 32-bit bitmap conversion are present. Submenu preparation depends on flags/bookmark comparisons, calling `0x1400c7550`.
2. D `0x14003efc0` rejects disabled records and expands submenu records. With panel context, kinds 1–9 use internal handlers; special-parent children take another internal route. Otherwise eligible commands call shell-invocation helper `0x1400c2bf0` at `0x14003f2ad`/`0x14003f64c`. Several contextless kinds do nothing.
3. S `0x140040560` clears results. Empty input calls `0x14003f880` twice. Nonempty input prepares submenus once, traverses non-submenu records, including disabled ones, and matches label/path plus auxiliary strings through `FUN_140123060` at `0x140040815`/`0x140040888`. Positive aggregate outputs become 16-byte results, sorted at `0x140040a93` using comparator `0x1400178a0`.

## Interpretation and limits

Application-owned presentation is inferred; rendering, matcher algorithm, scoring meaning, and ordering remain unknown. B value-flow is truncated. Ghidra marks `0x14018e365` terminal despite callee `hasNoReturn=false`; this does not establish nonreturning execution. Targetless indirect edges remain unresolved.

No behavioral contradiction found with [context-menu.md](C:/SANDBOX/REPOS/rev-filepilot/analysis/context-menu.md). Its search-candidate uncertainty is reduced; its historical provider failure is superseded by these dossiers.
