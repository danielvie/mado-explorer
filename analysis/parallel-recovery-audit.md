# Recovery and retained-evidence audit

## Scope and identity

Read-only audit of saved metadata, limitations, logs, exports, and snapshot. No native tools or target execution. Domain algorithms were not independently interpreted.

Original FPilot.exe remains 2,184,784 bytes. SHA-256 independently recomputed as 07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e.

## Fresh recovery counts

These counts exclude inherited records in cumulative exports. All 68 fresh records have unique Evidence IDs, the expected subject hash, raw_result, normalized_result, Ghidra 12.1.4 provenance, and profile digest 249564812a8d83b9b3f461098fc485ccde60c516d4497547a124081c91d654bf.

| Batch | Successful fresh records |
| --- | ---: |
| queries-07 | 13 |
| queries-08 | 26 |
| queries-09 | 14 |
| queries-10 | 9 |
| queries-11 | 6 |

Operation totals: 30 analyze_function dossiers; 1 procedure_pseudo_code; 9 read_bytes; 3 inspect_native_instruction; 1 procedure_assembly; 2 procedure_info; 9 resolve_containing_procedure; 8 xrefs; 3 procedure_callers; 1 search_procedures; 1 search_strings.

All 30 dossiers retain nonempty pseudocode and assembly. The separate panel pseudocode is also nonempty. Zero saved error records or unresolved containment results in these five batches. Logs report successful exports and close_binary calls for all five sessions. This does not erase older failures.

## Retention integrity

All 68 fresh JSON records deep-match their exported counterparts and the latest snapshot evidence_bundle. Object-key ordering differs in exports; this is not content corruption. Intermediate exports contain 54, 80, 94, 103, and 109 cumulative records. Their fresh-record coverage is 13, 39, 53, 62, and 68.

The latest bundle has 109 records and zero capture, scenario, or environment entries. The snapshot binds the same target hash, provider, and profile, with 42 primitive entries and one workflow entry. Bundle unknowns=0 does not mean indirect calls or semantics are resolved.

31 directly corresponding .c files match normalized pseudocode after trimming; the command-registration .asm matches its saved Listing text. File extensions do not turn these outputs into original source.

The transport export retains ev_50f1f10c61fbe8d42a0ddd53645fc1258c1ae26d4b1e72897d8d846244397552. Its complete dossier survives a large-response transport limit. Complete retention preserves the producer result, including its own truncation flags.

## Separate kinds of incompleteness

- Recycle enumeration, queries-08-recycle-enumeration-function.json, ev_39066d9daa85548651bb4dc4d5824c553222ddda77767f7b9079072ac0016beb: native_value_flow retains 3000 operations, truncated=true, omitted-operation lower bound 1, known omitted inputs/edges 0. Zero known omitted edges does not bound edges inside omitted operations. Pseudocode retains 17,212 characters and assembly 627 instructions.
- File header, queries-10-file-header-function.json, ev_50f1f10c61fbe8d42a0ddd53645fc1258c1ae26d4b1e72897d8d846244397552: native_value_flow retains 3000 operations, truncated=true, omitted-operation lower bound 1, known omitted inputs 0 and edges 351. Pseudocode retains 85,074 characters and assembly 3003 instructions.
- The other 28 fresh dossiers report untruncated native_value_flow. Even these def-use links do not cross functions and can omit aliases, call side effects, and unresolved indirect flow.
- All 9 raw reads report complete=true and returned_bytes=requested_bytes. Both shader reads independently equal original file bytes, including terminating NULs. Their 5440/1822-character source text is embedded material, not decompiled application source.
- The existing defined-string inventory retains complete shader text. No ASCII prefix-clipping was found by comparing inventory values to original NUL-terminated bytes at mapped addresses. This check does not establish exhaustive string discovery. Ghidra-defined inventory can omit undefined or packed text. Tool previews can also truncate display without truncating saved JSON.

## Recovery limits and decompiler assumptions

The comparator at 0x1400178a0 remains unrecognized as a function, but three decoded instruction records and a complete 16-byte read exist. Missing function recognition is not missing machine code. The return record has procedure=null, status=decoded.

The older queries-03 command-registration attempt failed with Ghidra decompile_failed, response buffer size exceeded. Fresh queries-07 metadata establishes a 39,449-byte body, and queries-08 retains 252,461 characters of assembly. Neither proves full command pseudocode or verified command bindings.

Fresh warnings include unrecovered jump tables and indirect-jump-as-call treatment in the operation worker and callback; unsettled type propagation in the window procedure; overlapping global symbols; and 42 unreachable-block-removal warnings in the file header. The earlier menu-builder terminal-flow assumption at 0x14018e365 remains a separate caveat. The profile disables no-return repair.

Native API metadata labels pseudocode decompiler-generated-non-source, compilable=false, with low-confidence recovered types and default signature provenance. Registers, reconstructed stack variables, pointer arithmetic, and compiler-generated control flow are observations, not original declarations.

REA supplies reconstruction evidence. It does not supply the original Rust/source language, names, structs/enums, generics, traits, modules, comments, ownership/lifetimes, dependency versions, Cargo project, build flags, or original project organization from these records. A native Windows x64 PE does not itself establish Rust provenance. Analysts must design a new Rust state/ownership model rather than claim it was recovered.

No Rust implementation, compilation, runtime fidelity, UI interaction, external drag compatibility, operation success, safety, or complete behavioral equivalence is proved. Static call/reference and body coverage must not be described as execution evidence.

## Checks and resolving action

Existing verify-evidence.mjs and rea-batch.mjs --check passed without native analysis or application execution. The evidence index was being updated concurrently; this audit does not judge its intentionally pending aliases as a defect.

Use these cautions in SPECIFICATION.md, preserve function-specific truncation and assumptions, and distinguish byte-exact embedded shader text from decompiler pseudocode. No retained-evidence integrity failure found. Whole-domain semantics and runtime/build verification remain outside this audit.

All cited filenames are under C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/. Logs are under C:/SANDBOX/REPOS/rev-filepilot/analysis/.
