# Commands, hotkeys, and search recovery

## Scope and result

Read-only static investigation of saved REA evidence and selected raw PE bytes. No REA calls, native REA tools, or target execution. Only this report was written.

The selected constructor path associates `HK_SelectionCopy` with handler `0x1400739b0` and default text `Ctrl+C`, `Ctrl+Insert`, and unmodified `Y`. This is not a complete default map or proof of effective runtime bindings.

The Shell-menu search path sorts retained results by descending aggregate score. Equal scores compare equal, with no secondary comparison. Its sort helper is not stable, so neither alphabetical ties nor preserved enumeration order is established.

## Evidence ledger

All filenames in this ledger are under `C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/`, except E-strings under `C:/SANDBOX/REPOS/rev-filepilot/analysis/`.

| Alias | Saved evidence | Address | Exact Evidence ID |
| --- | --- | --- | --- |
| E-registration | `queries-08-command-registration-assembly.json/.asm` | `0x1400258e0` | `ev_55ae7be3239235ddc3b4c28a9240ef3b08e715a4a1176a2bb4da379f504ca1aa` |
| E-builder | `queries-09-command-record-builder.json/.c` | `0x140025540` | `ev_8a432e7cdd2f7bfaa51f5eeef31c18db54fc2a0504e1a926c1e0c94bd1e1740a` |
| E-config | `queries-07-hotkey-parser.json/.c` | `0x140035100` | `ev_8d36029546c7fcf2ee239f89d3a9de2ea3cb005a50cee5600736dacea7e8d4ea` |
| E-chord | `queries-08-key-chord-parser.json/.c` | `0x140034f60` | `ev_0f1ea656d25feb0cebad3021c133736830fa44a20e03f635f7fe5004fa0bd1c3` |
| E-key-name | `queries-09-key-name-lookup.json/.c` | `0x14003c940` | `ev_7228ff3f9ce67581e1c91d8af9c47a69827327c19b67542ddd3bc23d0b6402f3` |
| E-key-text | `queries-09-default-key-text.json` | `0x1401cec74`, 52 bytes | `ev_9263598a97c6bdd43014f6ca3244205091be24b0d83ff473ab29bd70b2285701` |
| E-strings | `rea-strings.json` | `0x1401c7060` | `ev_453bf0abd31ef801caf790e89cbf12389bd80693356bb5d887a1e2ef07ff4b32` |
| E-copy | `queries-10-copy-handler.json/.c` | `0x1400739b0` | `ev_ec5b8b6388dd85c1cbf1584087b486f9e1a43bb948efdea71caebc80bee97d4d` |
| E-cut | `queries-10-cut-handler.json/.c` | `0x140073b80` | `ev_555fa06a93254f622021d22d5e67667e2bf025f385567923d2bad7cebdb736a7` |
| E-matcher | `filter-algorithm.c`, prior recovery evidence | `0x140123060` | `ev_4b786f7f26851765897b320658a520943f137b516e906b434ed9f2ed1aaf2fd2` |
| E-case-search | `queries-06-case-search.json/.c` | `0x140124360` | `ev_28208ba77a6ec96ce61fb67207cf9ff7f75e052d8e5d10897c4dccf580066ae3` |
| E-shell-search | `queries-01-shell-search.json`, `shell-search.c` | `0x140040560` | `ev_2671f325485b8692f2f8a5355a270f0885fb41f7d21ada00e6921d9ee7d9b092` |
| E-compare-load | `queries-07-shell-comparator-instruction.json` | `0x1400178a0` | `ev_cec91681530652c5aa349d6e2ce2af855cf736e4c4b52ee468ced58ca402d651` |
| E-compare-sub | `queries-08-shell-comparator-subtract.json` | `0x1400178a3` | `ev_a31812e1080d5a3870fbb65dd7d1b2fa4a74b8c39223301b92acff28a700ebcf` |
| E-compare-ret | `queries-08-shell-comparator-return.json` | `0x1400178a6` | `ev_843d6771f07252004db339dca7846740a0f2873fece54cbccb002bb945cf1185` |
| E-compare-bytes | `queries-08-shell-comparator-bytes.json` | `0x1400178a0`, 16 bytes | `ev_ba340512e8e1b30b7b13673f3f48df5c19388674490fb44d2cdfb0a1e35f6cab` |
| E-sort | `queries-10-sort-helper.json/.c` | `0x140165a20` | `ev_c4bf8029e199687b322189bf706dd04ab3d7df411faff6b691e65528ef991bbc` |

## Selected constructor mapping

E-registration supplies assembly despite the earlier pseudocode failure. E-builder shows the two text spans copied to record `+0x10/+0x20`, callback stored at `+0x30`, and the sixth argument at `+0x38`. Assembly stores the latter two at `0x140025729/0x140025731`. These are observed offsets, not recovered domain types.

At `0x140025e63`, registration loads `0x1401c7060` into R12. At `0x140025e6a`, it loads callback `0x1400739b0` into R13. It passes the command span through R8 and callback through `[RSP+0x20]`; `0x140025e99` sets command length `0x10`. Subsequent selected calls reuse R12/R13.

| Builder call | Default-text address | Copy then chord-parser call | Interpretation |
| --- | --- | --- | --- |
| `0x140025ea1` | `0x1401cec74`, loaded at `0x140025ea9` | `0x140025f02`, `0x140025f1e` | `Ctrl+C` |
| `0x140025f5a` | `0x1401cec80`, loaded at `0x140025f62` | `0x140025fbf`, `0x140025fdb` | `Ctrl+Insert` |
| `0x140026017` | `0x1401cec8c`, loaded at `0x14002601f` | `0x14002607f`, `0x14002609b` | `Y`, no modifier prefix |

Each parser call has RDX set to the returned record and R8D zero, selecting its first chord slot. The copied text is parsed, not merely displayed. E-builder can reuse an existing unbound record with the same name/callback or append another record; do not collapse this into a proven one-command/one-binding layout.

E-key-text raw bytes independently decode the three NUL-terminated literals. E-strings identifies the command name at `0x1401c7060`. A local standard-library PE read also verified the following original file bytes, without executing the file or creating REA evidence:

| VA | File offset | Bytes including NUL | Text |
| --- | --- | --- | --- |
| `0x1401c7060` | `0x1c5c60` | `484b5f53656c656374696f6e436f707900` | `HK_SelectionCopy` |
| `0x1401cec74` | `0x1cd874` | `4374726c2b4300` | `Ctrl+C` |
| `0x1401cec80` | `0x1cd880` | `4374726c2b496e7365727400` | `Ctrl+Insert` |
| `0x1401cec8c` | `0x1cd88c` | `5900` | `Y` |

The local PE SHA-256 matches every cited artifact: `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e`.

## Hotkey parsing and handler limits

- E-chord `0x140034fb5-0x140035060` splits on `+`, skips empty pieces, resolves the final piece as a key with lookup mode 2, and preceding pieces as modifiers with mode 1. Modifier results are ORed. `0x14003508e/0x140035092` store modifier/key bytes at slot index and slot index plus 2.
- E-key-name `0x14003c970-0x14003c98b` lowercases ASCII A-Z in the supplied scratch text before table lookup. Mode 1 accepts table codes 1-4; mode 2 excludes those modifier codes. This does not recover the complete key vocabulary or modifier-bit assignments.
- E-chord has special rejection branches and only four locally retained pieces. Its pseudocode is not a safe parser specification for oversized input; exact invalid-input behavior was not reconstructed. Missing modifier lookups are ignored in the visible loop, rather than proving strict validation.
- E-config `0x140035100` clears binding words while walking registered chains, looks up `Command` and the `Keys` field, then resolves existing command records. `Keys` is confirmed locally as `4b65797300` at `0x1401d1a90`, file offset `0x1d0690`.
- E-config's array branch consumes at most two chord entries and only registers when its counted parses succeed. `0x1400354c5` also leads to a candidate conflict-check helper `0x140035fd0`, then E-builder and a four-byte binding copy. Token kinds, conflict resolution, precedence, and runtime dispatch remain incomplete.
- E-copy and E-cut fall back to application `+0x920` when their first context value is zero. Both call `0x1400cfff0`, with `0xc0` for copy and `0x80` for cut. Both act only when its returned first value is nonzero and the third argument equals 2, then call `0x1400cdd40` and `0x1400ccf10`. Cut explicitly forces the outgoing first low 32-bit field to 1. Both return whether the first value was zero. Those values are not independently proven as execution phases, booleans with domain names, or complete clipboard payload types.

Constructor text therefore supports defaults for reconstruction, not currently effective shortcuts. Configuration, conflict checks, command context, and dispatch can change their effect.

## Search scoring

E-shell-search `shell-search.c:87-182` sends nonempty input through parser `0x1401b9110` with configuration byte `0x20`, then matches two candidate spans and auxiliary strings through E-matcher in mode 1. E-matcher's nonzero-mode branch calls E-case-search at `filter-algorithm.c:420`.

E-case-search processes converted candidate spans and query tokens through `0x14013f5d0`. It compares 32-bit elements after `0x140169dc0` mapping and records ordered positions. This supports an ordered/fuzzy interpretation, not a fully recovered Unicode or case-folding contract.

The visible complete-token score, in converted-element units, starts at `100 + 3 * (matched_count - candidate_length) - first_position`. Both candidate branches then apply:

- Adjacent matched positions add 200. Primary assembly includes `0x140124922`.
- A match at position zero adds 50, including `0x140124ac6`.
- A position gap greater than five subtracts `50 * (gap - 5)`, including `0x140124913`.
- A preceding underscore or space adds 25.
- Recognized lower-to-upper boundaries add 30. Explicit ranges include ASCII and selected non-ASCII ranges, not proof of a standard Unicode category implementation.

Alternate-search helper `0x140065880` can replace the current candidate when its signed score is larger. Its search limits and full matching algorithm remain uninspected. Primary token scores accumulate directly. Secondary token scores are individually multiplied by 0.75, adjusted by plus/minus 0.5 according to sign, truncated to integer, then accumulated. E-case-search `0x1401254f1/0x14012551d` corroborates multiplication/conversion; `.c:746-774` supplies the rounding flow. The returned match flag records a match, not a requirement that every token matched.

E-shell-search `.c:183-259` adds both score outputs and scores from matching auxiliary strings. Only a positive signed aggregate is retained. Each 16-byte result stores the candidate pointer at `+0` and score in the low 32 bits at `+8`. This recovery applies to Shell-menu search, not automatically filename filters or a global command palette.

## Ordering and ties

E-shell-search `0x140040a7f-0x140040a93` supplies comparator `0x1400178a0`, stride `0x10`, and sort helper `0x140165a20`.

E-compare-load/sub/ret decode the entire comparator as `MOV EAX,[RDX+8]` at `0x1400178a0`, `SUB EAX,[RCX+8]` at `0x1400178a3`, then `RET` at `0x1400178a6`. E-compare-bytes corroborates `8b42082b4108c3`, followed by `cc` padding. Under the Windows x64 argument convention it returns `score(second) - score(first)` using 32-bit arithmetic. For the retained positive signed scores, subtraction cannot overflow.

E-sort `0x140165aa0-0x140165ab1` compares the later element with its predecessor and swaps only for a negative comparator result. Thus a higher score moves earlier. Descending aggregate scores are established by the caller, comparator, and sort sign convention together.

Equal scores return zero with no name, path, ID, or pointer tiebreaker. E-sort's partition branch begins when the index span exceeds ten, at `0x140165a70-0x140165a77`. Its scans stop on nonnegative comparisons at `0x140165c58/0x140165c95`; `0x140165cc1-0x140165cfb` swaps distinct positions even when their scores are equal. This proves the sort is unstable. For example, twelve equal-score records reach a swap of indices 1 and 11 on the first partition. Small insertion-only inputs preserve equality order, but there is no global stability guarantee. Exact tie permutations depend on input order/count and this implementation, not a separate semantic tie policy.

## Rust reconstruction boundary

These are compiled artifact observations and behavioral guidance, not original Rust, original C/C++, or recovered compilable application source. E-builder explicitly marks its pseudocode `decompiler-generated-non-source`, `compilable: false`, with low-confidence recovered types. `undefined*`, `FUN_*`, inferred arrays, stack fragments, and raw offsets must not become domain types or an ABI by transcription.

A Rust implementation can model named commands, parsed chord slots, two score channels, and descending Shell results using its own safe types. Choosing a stable or alphabetical tiebreaker would be a new design choice, not recovered behavior. No Rust code, compilation, runtime binding verification, or complete matcher grammar was produced here.

Coverage is selected registration assembly through `0x14002609b`, full saved builder/hotkey/key-name/copy/cut/sort pseudocode, full Shell-search pseudocode, selected matcher scoring blocks and instruction records. Query parsing, character conversion/mapping, alternate-search internals, effective key dispatch, and all remaining defaults are unresolved.
