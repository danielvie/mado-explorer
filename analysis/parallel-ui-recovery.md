# UI rendering recovery for Rust/Windows

## Scope and evidence

Static review of retained evidence for `C:/SANDBOX/REPOS/rev-filepilot/FPilot.exe`, SHA-256 `07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e`. No REA calls, lifecycle changes, target execution, shaders, or application code written. Only this report was written.

All evidence files below are under `C:/SANDBOX/REPOS/rev-filepilot/analysis/rea/`.

| Alias | Address | Exact Evidence ID | Record |
| --- | --- | --- | --- |
| P | `0x1400d20e0` | `ev_c571f2845b4ac5fd76a9b32bfa971bb01e51eb58ce544dbf17e8aadff2284801` | `queries-11-panel-renderer-pseudocode.json` |
| PM | `0x1400d20e0` | `ev_80219f768c3ecf8e772cb4fbfa23159e2a5dd0eefadd99754b9077ad0c107bf3` | `queries-11-panel-renderer-metadata.json` |
| T | `0x1400fa420` | `ev_610efcb0ace1adf00ce0a6e6cb5bed7e75e25e387041c69546e3819770b66407` | `queries-03-panel-tabs-function.json` |
| H | `0x1400f3360` | `ev_50f1f10c61fbe8d42a0ddd53645fc1258c1ae26d4b1e72897d8d846244397552` | `queries-10-file-header-function.json` |
| I | `0x1400666b0` | `ev_c9c7e1338de5edcbb2b1f019d8e0e3a07a2afc64e7983a5fcbaefec0ccf35399` | `queries-10-clipboard-state-update.json` |
| F | `0x1401d1b70` | `ev_73810092362e4370dad89f4d609cb069a7b6dda1f33b876cbd885fb0bee41d4b` | `queries-11-embedded-fragment-shader.json` |
| V | `0x1401d30c0` | `ev_8efad9242ff9041b9946139e1789a37c7bd534554bceed0410e919cdc3c18f60` | `queries-11-embedded-vertex-shader.json` |
| S | strings listed below | `ev_b8ccd43ce2fb3f117d9b6a84187f1948af64e0daa3f0bc987b0ceef1e35c7701` | `queries-09-file-row-seeds.json` |

## Complete shader text, not complete application source

F and V have `complete: true`, requested bytes equal returned bytes, and identical raw/normalized byte-read objects. Decoded bytes are ASCII, with exactly one NUL at the end. Removing only that terminator gives the complete retained GLSL text, including comments and whitespace.

| Shader | Bytes including NUL | Source bytes | I string-reference instruction addresses | Exact comparison |
| --- | --- | --- | --- | --- |
| Fragment, F | 5441 | 5440 | `0x14006675f`, `0x140066768` | Both `referenced_strings.value` entries equal decoded F without NUL |
| Vertex, V | 1823 | 1822 | `0x140066774`, `0x140066781` | Both `referenced_strings.value` entries equal decoded V without NUL |

SHA-256 of source bytes without NUL:
- F: `99969305793c366ffdccd7c1305ae98e3321f59dce7ff3797517461cf6d068bc`.
- V: `c061d572bc75cbe9a74b0f3ea28f02b4107699b06d1247c2be723965d9bacb14`.

The `.c` initializer's fragment literal ends with `/* TRUNCATED STRING LITERAL */`; extracting that display would lose source. Its vertex literal is complete, but the raw-byte records are the stronger extraction inputs. Preserve the final LF and omit only the final NUL when creating GLSL files.

I and H explicitly classify host pseudocode as `decompiler-generated-non-source`, `compilable: false`; T does likewise with low-confidence types. P also contains unresolved types, omitted call arguments and removed-unreachable-block warnings. Recovering embedded shader text is different from recovering original C/Rust host source. Byte reads describe Ghidra initialized memory, including possible loader fixups, not a fresh runtime observation. No shader compilation was performed.

## Renderer and shader algorithms

I is renderer initialization despite its clipboard filename. `queries-10-clipboard-state-update.c` shows `glEnable(0xbe2)`, `glEnable(0xc11)`, `glBlendFunc(1,0x303)`. These are blending, scissor testing, and ONE/ONE_MINUS_SRC_ALPHA blending. This supports a premultiplied-alpha-compatible blend configuration, not proof of every texture's alpha encoding.

I's indirect-call sequence uses vertex/fragment shader types `0x8b31/0x8b30`, compile status `0x8b81`, link status `0x8b82`, and 512-byte log buffers on failure. Names of these indirect APIs are interpretations of their arguments and sequence, not resolved imported callees. Texture sampler is assigned unit zero.

I assembly at `0x1400668fa..0x140066b45` corroborates a 72-byte attribute stride. Pseudocode's first stride includes a reconstructed stack artifact; assembly writes literal `0x48`.

| Locations | Components and GL storage type | Byte offsets |
| --- | --- | --- |
| 0..3, corners | 2 INT each | `0x00,0x08,0x10,0x18` |
| 4..5, UV limits | 2 FLOAT each | `0x20,0x28` |
| 6, color; 7, gradient | 4 normalized UNSIGNED_BYTE each | `0x30,0x38` |
| 8, corner radii; 9, excluded edges | 4 unnormalized UNSIGNED_BYTE each | `0x34,0x3c` |
| 10, texture layer | 1 SHORT | `0x40` |
| 11..15, thickness/softness/dash/gap/dash radius | 1 UNSIGNED_BYTE each | `0x42..0x46` |

I then enables 16 attributes and submits divisor 1 through `DAT_1401e8a08`. Together with V this supports instanced quad records. Draw topology, submission batching, texture uploads and resource lifetime are not recovered here.

V declares GLSL `#version 330 core`, selects four positions and UVs by `gl_VertexID`, projects with `proj`, and passes shape position `p0` and dimension `abs(p3-p0)` to F. `fragDepth` is the texture-array layer, not geometric depth; projected Z is zero.

F samples `sampler2DArray`; monochrome mode replicates the red channel to RGBA before tint multiplication. Dashed outlines use modulo dash-plus-gap length on the edge axis and discard gaps. RoundedBoxSDF selects corner radius by quadrant, computes rounded-box distance, then smoothstep handles fill or outline coverage. Edge masks have corner corrections. Center coordinates use side-dependent ceil/floor; softness adds a four-times-softness pad. Four corner gradient values are bilinearly interpolated and multiply the whole pixel value. A source comment limits extra shape parameters to unrotated shapes. Shader ceil/floor operations are shape rasterization, not file-row virtualization evidence.

## Panel composition and tab geometry, partially recovered

P `.c:83-145` walks tabs by `+0x540`, selecting the first with `+0x548 == 0`. Global UI phase `+0x272` separates branches 0, 1, 2. Phase 1 calls T, `0x1400edb40`, H, and `0x1400d9c20`; phase 2 calls T and conditionally `0x1400d1ff0`. Naming phases as layout/input/paint remains inference.

P `.c:180-245,363-449` obtains a rectangle through `0x1400cf8d0`, passes dimension-derived values through `0x140176440`, and rounds outputs by sign-dependent +/-0.5 before integer conversion. Helper internals and animation timing remain unknown. The orientation selector at tab-associated `+0x680 -> +0x48` uses available height less application `+0x848` versus width when zero; nonzero selects by equality to 1. A ratio at `+0x40` participates in a half-scaled rounded extent stored at `+0x50`.

P `.c:363-545` creates IDs using `Panel`, `Canvas`, `TopBlock`, pushes geometry/style records, calls T, then `0x1400edb40`, H, `0x1400ea040`, `0x1400d9c20`. An additional `0x14007c570` call is gated by the extent at `+0x50 > 1`. This is composition evidence, not a row renderer body.

T `.c:1055-1078,1168-1221` counts tabs, clears/retains the index of `+0x678`, and computes shared tab width from available width minus per-tab overhead and an extra-control allowance. Width is bounded by rounded UI-metric multiples 10 and 16. Some transient state uses tab count plus one. `0x140065160` rounding and unsigned division artifacts need corroboration before a literal Rust port.

T `.c:1282-1299,1409-1498,1618-1685` gets tab nodes through `0x1400fd210`, copies their `+0x278/+0x280` rectangles into 16-byte records, supplies opposite signed deltas to `0x140185710` for horizontal controls, and adjusts scroll for the remembered tab using its rectangle, neighboring rectangles, viewport width and maximum overflow. Supported target behavior is adaptive widths and keeping a marked tab visible. Exact tab interaction semantics are outside this pass.

H `.c:298-340,1605-1703,1740-1840,1850-1890,2080-2164` is `Panel_RenderHeader`, supported by diagnostics, with `DirsViewport`/`DirsArea` IDs. It measures and renders header elements through `0x1400f6ec0`, saves their rectangles, handles horizontal overflow controls, and uses flag `+0x67a` or an external requested index for keep-in-view calculations. Header-path overflow is not file-list culling. Header child rendering and early controls remain partial.

## File-list virtualization remains unresolved

No inspected body establishes first-visible/last-visible file indices, overscan, row-height-to-scroll division, skipped offscreen item construction, or a file-list complexity bound. Scissor testing, shader floor/ceil, header overflow and tab scrolling do not establish virtualization.

The actual file-rendering child's identity is not proved by P's call sites alone. `0x1400d9c20` is a focused first candidate because it follows H in both phase 0 and 1. `0x1400edb40` is another repeated child; `0x14007c570` is gated by the extra extent. Do not label any of them definitively from address/order alone.

Recommended focused follow-up, not performed: `procedure_pseudo_code` for `0x1400d9c20`, then its metadata/selected child if it delegates. Ask specifically for listing-array access, row/grid extent, visible-index bounds and scroll-state use. If it is not the file-list body, inspect `0x1400edb40` next. Keep `0x1400f6ec0` separate as header element rendering.

S's saved regex search returned `Panel_RenderTabs` at `0x1401d0c28`, `Panel_RenderHeader` at `0x1401d0c98`, and `AT_FileName` at `0x1401c02b0`, but no file-row diagnostic matching that pattern. Its coverage is Ghidra-defined strings only, not proof that other strings or row rendering are absent.

## Read/search coverage and limits

- Read the old `C:/SANDBOX/REPOS/rev-filepilot/analysis/rust-ui-recovery.md` and relevant evidence-index section. Reused only its UI scope; did not reinterpret hotkeys, search matching, clipboard or drop behavior.
- Read all 564 lines of P `.c`; parsed P/PM JSON for identity and metadata. Read all I `.c`; parsed full I JSON, inspected reference strings and selected assembly `0x140066719..0x140066bd2`, not all value-flow/basic-block sections.
- Parsed complete F/V JSON, decoded every byte, compared both copies and every matching I referenced-string entry, and read complete decoded GLSL. No shader files written.
- H `.c` has 2176 lines. Read 1590-2176 plus grep results for literals, arithmetic and local child calls; earlier control implementations were not fully read. Parsed full H JSON for identity, callees, strings and pseudocode classification, not complete assembly/value flow.
- T `.c` has 1708 lines. Read 1032-1301,1400-1510,1618-1708. Parsed full T dossier for identity/pseudocode classification. Earlier interactions were not reread. `queries-03-panel-tabs.json` is a containing-procedure resolution, not T's function dossier.
- S was read completely. Local full-file searches under `analysis` for `Panel_Render` hit a 35-result cap; follow-up used S and the known dossiers. Searches for child names in saved `.c` returned only P; JSON searches for literal child addresses returned no hits. These are bounded local searches, not whole-binary call-graph coverage.
- Recovery is partial host behavior plus complete embedded shader text. Accessibility and IME support were not investigated; no absence claim follows. No pixel-equivalence, runtime correctness, or performance guarantee is established.
