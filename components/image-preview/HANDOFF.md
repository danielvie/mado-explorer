# P01b image decoder handoff

## Candidate and gate

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/image-preview`.
- Branch: `lane/image-preview`. Base: `491733082afbbaafe84f81919c8ff169378e53f3`. Candidate is the commit containing this handoff.
- Exclusive writes: `components/image-preview/**`. No root/main/sibling writes, delegation, merges or pushes.
- Targeted correction is partial. Keep the production candidate unapproved and unpromoted until supervisor fresh review. No acceptance statuses were changed.
- No total-process memory or OOM-survival guarantee is required or claimed. Dependency-allocated bulk image buffers count toward the 256 MiB application CPU budget. Current accounting still does not prove that complete bound.

## Changes

1. JPEG constructor admission runs before `ImageReader::into_decoder`. It charges the actual input capacity, the constructor's copied input capacity, and bounded header metadata payloads. A safe malformed four-byte JPEG test distinguishes constructor refusal from subsequent header parsing.
2. JPEG decode admission now includes conservative pinned coefficient and row-buffer ceilings. It uses separate constructor, orientation, decode, conversion and rotation phases instead of the previous `F + N + 2R` expression.
3. Allocation-free WebP preflight checks the RIFF size against the actual snapshot, every top-level chunk extent/pad, and the single ANMF nesting level. It checks VP8/VP8L short headers and dimensions against the canvas or animation frame before constructing the dependency decoder. The VP8 first-partition extent is also checked before allocation. Unknown chunk payloads are not parsed or copied.
4. Exact dependency constraints now pin `image-webp = 0.2.4`, `png = 0.18.1` and `zune-jpeg = 0.5.15`, alongside `image = 0.25.10`. The component lock remains tracked. No new package was downloaded or added to the resolved graph.
5. Tests retain three formats, all eight orientations, corrupt/unavailable files, unchanged sources, Unicode/native Windows paths and dimension/output limits. New tests cover constructor ordering, accounted phase boundaries, forged metadata extents, embedded huge/mismatched VP8 dimensions and animation subchunks.

The API remains synchronous, read-only `decode(&Path, DecodeLimits) -> Result<Preview, PreviewError>`. A worker, stale-result rejection, resize, window cache/CPU reservations and GPU ownership remain integration-owned. PNG/WebP still return the dependency's default/first still image, not animation playback. No required format or orientation support was removed. Preflight deliberately refuses RIFF trailing bytes and validates all ANMF frame headers, even though only the first frame is decoded.

## Limits actually enforced

- `max_input_bytes`: checked before allocating the snapshot. Exact-length read plus one-byte growth probe never grows the input Vec.
- `max_width`, `max_height`: encoded and orientation-corrected dimensions; WebP preflight also checks embedded frame dimensions before decode.
- `max_output_bytes`: checked/addressable RGBA byte count before native pixel allocation.
- `max_buffer_bytes`: currently accounted buffer phases only, not a complete CPU bound. JPEG includes audited input/payload/coefficient/row buffers. PNG/WebP include input/native/conversion/rotation pixels; WebP separately admits input plus an Exif payload up to the actual file length.
- `max_decoder_bytes`: still passed to image's best-effort `max_alloc`; native output must fit even when the adapter ignores it. It does not cap JPEG/WebP internals and does not prove PNG's complete working peak.

Input capacity is counted rather than just length. Checked arithmetic rejects overflow. Dependency allocations remain infallible; ordinary unrelated memory pressure can still cause an allocation failure outside this component's fallible input allocation.

## Pinned allocation audit

Source root for dependency paths below is `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`. These are installed sources, not vendored files. Rust std evidence is under `C:/Users/daniel/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/`.

Notation: `F` is bounded file length, `P` is actual snapshot Vec capacity, `N` is native pixels, `R = 4*w*h`. `L` is the PNG constructor allocation limit. Quantities below are bytes, excluding allocator bookkeeping and small path/error values.

### JPEG phases implemented

- `image-0.25.10/src/codecs/jpeg/decoder.rs:29-40` calls `read_to_end` into an empty Vec before headers/dimension checks. Std `std/src/io/cursor.rs:381-388` reserves the complete slice once, then copies it. Std `alloc/src/raw_vec/mod.rs` uses minimum capacity 8 for bytes. The copied capacity is `D = max(F, 8)`. There is no old growing allocation to overlap in this specific Cursor path. This conclusion is toolchain-specific, not a claim about arbitrary Read implementations.
- `zune-jpeg-0.5.15/src/headers.rs:540-693` copies APP metadata payloads bounded by actual input through `peek_at(...).to_vec()`. Aggregate payload copies during one header parse are conservatively charged `F`. Container capacities are a remaining gap, listed below.
- Constructor charge is `P + D + F`, admitted before dependency construction. Orientation reparses headers and clones Exif in `image-0.25.10/src/codecs/jpeg/decoder.rs:104-127`; charge `P + D + 2F` for simultaneous payload and clone.
- `zune-jpeg-0.5.15/src/mcu.rs:174-183` and `mcu_prog.rs:99-105` allocate `128*MCU_x*MCU_y*h_sample*v_sample` bytes per component. `components.rs:105-149` permits at most four components with sampling factors at most four. Set `C = ceil(w/8)+3`, `H = ceil(h/8)+3`. Since `ceil(w/(8*h_max))*h_sample <= C`, and likewise for height, four components need at most `512*C*H`. The exceptional MCU-height multiplier `v_max/h_max` in `mcu.rs:108-124` and `mcu_prog.rs:83-97` is at most four. The implemented coefficient ceiling is `2048*C*H` for baseline separated scans and progressive decoding alike.
- `misc.rs:240-288` establishes `width_stride <= 8*C` and the workaround multiplier at most two. `components.rs:190-196`, `mcu.rs:160-191`, `mcu_prog.rs:509-530` and `worker.rs:509-511` allocate native row coefficients, saved rows, upsample destinations and scratch. Per component in i16 elements: raw rows <= `32*stride`, two saved rows <= `8*stride`, first upsampled row <= `64*stride`, destination <= `256*stride`. Four components total <= `2880*stride` bytes. The implemented `8192*stride` ceiling also charges replacement/growth overlap and <= `16*stride` bytes of upsampler scratch.
- Decode charge is `P + D + F + N + 2048*C*H + 8192*8*C`.
- Decoder working buffers and its copied input drop before conversion/rotation. Those phases are `P + N + R` and `P + 2R`. The maximum of all phases is admitted before orientation/native decoding.

These are conservative charges for the listed allocations, not a total JPEG allocator proof. The large row ceiling can reject images that a tighter sampling-aware calculation could admit. It does not raise or waive any budget.

### PNG derived bulk ceilings, not yet integrated

- `image-0.25.10/src/codecs/png.rs:45-70` forwards `L` during construction, but enables text metadata. Later `set_limits` does not forward changed allocation limits.
- `png-0.18.1/src/decoder/stream.rs:603, 783-794` maintains and grows raw chunk storage. `1726-1729` decompresses ICC data bounded by remaining `L`; `1843-1894` stores text metadata. `fdeflate-0.3.7/src/decompress.rs:1063-1090` resizes ICC output with logical length <= `L`, but Vec capacity can exceed logical length. An individual ICC Vec growth can have old/new capacity overlap <= `3L`; retained charged payload capacities also need accounting. `max_alloc = L` alone is not a peak bound.
- `png-0.18.1/src/decoder/unfiltering_buffer.rs:65-100, 186-195, 306` allocates an uncharged inflate/unfilter buffer and row scratch. For supported <=16-bit RGBA, a whole-frame raw-stream ceiling is `B = 8*w*h + 7*h`, including Adam7 filter bytes. A conservative data-stream capacity/replacement ceiling is `3*(B + 128 KiB)`. The initial buffer is at most 128 KiB; later growth uses 32 KiB increments. Row scratch/replacement is <= `3*(8*w+1)`, plus output-line scratch <= `8*w` from `decoder/mod.rs:529`.
- Exif clone used for orientation overlaps the stored Exif payload in `image-0.25.10/src/codecs/png.rs:217-225` and needs up to another `F`. PNG decode therefore needs `P + N + metadata/capacity peak + data-stream/row scratch`, not merely `P + N + 2R`.
- Remaining proof obligations: aggregate raw/text/ICC storage and container growth, inflater state, and constructor-stage scratch before the image adapter returns. A direct `png::Decoder` can disable unused ICC/text parsing while retaining Exif, but integrating it and auditing its output/conversion path is a separate bounded slice. No metadata feature was silently disabled here.

### WebP derived bulk ceilings and remaining decoder paths

- `image-webp-0.2.4/src/decoder.rs:439-444, 622-635` stores declared ranges, then allocates Exif from those lengths before reads. Preflight now requires real in-bounds ranges, including ANMF subranges. Orientation payload is at most `F`, admitted with `P` before construction.
- `src/vp8.rs:1194-1215` allocates padded YUV planes `Y = 384*ceil(w/16)*ceil(h/16)` bytes and width-dependent macroblock/border storage. Outer and embedded dimensions are now checked before this path, including the animation ALPH+VP8 branch that previously lacked a matching-dimensions check. The first partition declared by the ten-byte VP8 header is also checked against its real payload before `vp8.rs:1219` allocates it.
- `src/vp8.rs:1031-1054` reads internal partition sizes only after arithmetic-decoding the first partition. Each 24-bit size can request up to 16 MiB rounded to four-byte storage before `read_exact`. Earlier successful partitions remain live; the last partition additionally overlaps a growing read-to-end Vec and a four-byte chunk copy. RIFF preflight cannot validate these internal lengths without decoding that bitstream prefix. These allocations are NOT charged by current code.
- `src/lossless.rs:191, 222-239, 255, 313-328` allocates a color cache <=8192 bytes, two transform images each <= `4*ceil(w/4)*ceil(h/4)`, a palette <=1024 bytes and entropy image/copy <= `6*ceil(w/4)*ceil(h/4)`. Recursive auxiliary streams do not read another meta-Huffman image, but their single-group scratch still overlaps transform or entropy data.
- `src/lossless.rs:315-349` sets Huffman group count from the maximum 16-bit entropy-image value plus one, NOT from its pixel count. A tiny canvas can select 65,536 groups. `src/huffman.rs:14-28, 73-84` allocates each group's five tables and tree nodes. On the x64 lane, a node is 16 bytes. Maximum alphabets are `2328, 256, 256, 256, 40`, totaling 3136. A conservative per-group heap ceiling is `32*3136 + 5*4096 = 120832` bytes, before group-container growth and temporary code lengths. 65,536 such groups alone exceed 7 GiB. A file-bit-length ceiling can tighten this: every completed group requires at least 20 bits for five single-symbol tree descriptions, plus one potentially partial group. This remains very pessimistic for useful images and is NOT implemented as a blanket admission rule.
- `src/decoder.rs:687, 809-816, 840, 870` adds an opaque lossless RGBA temporary, first-frame RGB/RGBA temporary and animation canvas, each <= `R`. `src/extended.rs:306-315` compressed alpha overlaps RGBA <=`R` with alpha <=`R/4` and lossless scratch. These coexist with native output and sometimes YUV planes; conversion/rotation occurs only after the decoder drops them.
- WebP working peak therefore requires `P + N + YUV/borders + partitions + transforms/entropy/cache/Huffman + applicable RGBA/alpha/frame/canvas overlap`. Current code does NOT admit that complete expression. Direct `WebPDecoder::set_memory_limit` is insufficient: its documented TODO and these allocations bypass it.

## Remaining blocker and next decision

The dangerous outer-metadata/embedded-dimension guards are preserved. A complete useful bound now needs PNG metadata/scratch accounting and WebP internal partition/Huffman admission. JPEG still needs metadata container capacities and fixed table/state audit. None are excluded merely because a dependency allocates them.

A pure worst-case WebP Huffman ceiling cannot admit ordinary useful images under 256 MiB without substantial pessimistic rejection. A tighter count requires internal bitstream inspection or an audited dependency change that admits Huffman groups/partition storage before allocation. I stopped rather than build a second substantial WebP parser, vendor the codec, introduce processes, or silently waive these buffers. Supervisor should choose the next API/dependency slice or explicitly assess a conservative rejection policy. Strict allocator APIs are not presumed mandatory; audited useful admission remains acceptable if completed.

## Exact validation evidence

All commands ran with explicit `cd C:/SANDBOX/WORKTREES/mado-explorer/image-preview`.

```text
cargo generate-lockfile --offline --manifest-path components/image-preview/Cargo.toml
cargo fmt --manifest-path components/image-preview/Cargo.toml
cargo fmt --manifest-path components/image-preview/Cargo.toml --check
cargo check --locked --manifest-path components/image-preview/Cargo.toml --target-dir target/p01 --all-targets
cargo test --locked --manifest-path components/image-preview/Cargo.toml --target-dir target/p01
cargo test --locked --manifest-path components/image-preview/Cargo.toml --target-dir target/p01 input_dimension_output_peak_and_decoder_limits
git diff --check
git diff --cached --check
```

- Offline lock generation succeeded; resolved dependency versions did not change.
- Initial all-target check failed on a missing slice-return lifetime in the new preflight helper. Corrected with the lifetime tied only to input; subsequent fmt/all-target check passed.
- Coherent test run passed all four unit checks and seven of eight integration checks. The exact WebP boundary fixture omitted the newly admitted `P + F` orientation phase and failed, not counted as passing. Corrected its boundary to the maximum of that phase and its pixel phase, preserving the exact success/one-byte-less refusal assertions.
- After that test-only correction, fmt check and locked all-target check passed. The affected boundary integration check passed, one passed/zero failed. Combined unchanged and affected evidence is twelve distinct passing tests, none ignored. No tests were removed or weakened.
- Forged multi-GiB lengths and huge embedded VP8 fixtures run ONLY through the allocation-free private preflight seam, never through the vulnerable decoder. Constructor ordering uses four-byte or ordinary small generated JPEGs. All file tests use disposable generated fixtures, never personal folders.
- Build output stays in ignored `target/p01`. Taskfile remains current; test runs the new checks and run/clean contracts are unchanged. No build cleanup was needed in this slice.
- No GUI, lossy WebP end-to-end fixture, full decoder-memory instrumentation, window/GPU/cache, Narrator, stale-result or performance check ran. Their absence is not a pass.

## Coverage

F07.4/F07.6 backend portions gain early constructor/range/dimension guards and audited JPEG bulk-buffer admission. Both remain partial due to the allocations above and missing integration. F07.1/F07.2 retain P01 backend format/orientation/metadata behavior, not GUI acceptance. F07.3/F07.5 and full window cache/CPU/GPU ownership remain outside this component.
