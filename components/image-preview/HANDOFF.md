# P01 image decoder handoff

## Candidate and gate

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/image-preview`.
- Branch: `lane/image-preview`. Candidate is the commit containing this handoff.
- Owned paths only: `components/image-preview/**`. Root manifest, lock, trackers, application source and other lanes are unchanged.
- Backend candidate is partial, not ready to claim the full P01 memory contract. Hard decoder-working-memory enforcement is unresolved. Do not enable it as a fully bounded production preview backend until that blocker is resolved and independently reviewed.
- No GUI criterion is accepted by this work. No merges, pushes, nested delegation, personal-data probes or source-file writes were performed.

## API

`mado_image_preview::decode(&Path, DecodeLimits) -> Result<Preview, PreviewError>` is synchronous. The integration owner calls it on its bounded worker and rejects stale results. It returns owned, tightly packed, straight-alpha CPU RGBA8, corrected width/height, encoded dimensions, detected format, file byte length and the applied Exif orientation number.

Native paths stay native through file opening and error reporting. File content signatures precede extension detection; a supported extension is a fallback for malformed input without a recognizable signature. Unsupported formats/features, malformed input, unavailable files, oversize input and allocation failure have distinct public error kinds. `PreviewError` retains the original `PathBuf` and implements Display/Error. Source files open read-only and handles close before returning.

PNG, JPEG and WebP use pinned `image = 0.25.10`, with only those codec features enabled. No native renderer, egui, GPU, worker pool, cache or shared application type is introduced. PNG/WebP return the dependency's default/first still image; animation playback is not implemented. No resize or ICC color-management promise is made. Invalid/missing orientation falls back to orientation 1, as the dependency specifies. Valid orientations 1 through 8 are applied for all three formats.

Caller supplies every limit; there are no implicit defaults:

| Field | Enforcement |
| --- | --- |
| `max_input_bytes` | File length checked before allocation. Read exactly the admitted length and probe one extra byte without growing the Vec. Observed growth is rejected; shrinking/read failure is unavailable. |
| `max_width`, `max_height` | Encoded and corrected dimensions checked before output decode/allocation. Also passed to the dependency's strict dimension controls. |
| `max_output_bytes` | Checked RGBA8 byte count and addressability before output decode/allocation. |
| `max_buffer_bytes` | Conservative admission for input + native decoded pixels + two RGBA8 buffers. Covers conversion/rotation overlap. Excludes codec internals, allocator overhead and small metadata/path values. Returned pixels consume only their RGBA byte length; dropping them releases the buffer. |
| `max_decoder_bytes` | Passed to `image::Limits::max_alloc`. Native output alone must fit it even for adapters that ignore it. Codec internal working-memory enforcement is best-effort, NOT a hard bound. |

Allocation arithmetic uses checked operations. Input allocation uses `try_reserve_exact`. Dependency conversion/rotation allocation is infallible; admitted sizes do not guarantee the operating system will satisfy allocations under unrelated memory pressure. No process-wide OOM recovery is promised. Concurrent writer consistency, prompt interruption of a slow file open/read, cancellation and total window CPU/GPU/cache budgets belong to later supervisor-owned integration.

## Working-memory blocker evidence

Installed version-specific source inspected:

- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/io/limits.rs`, lines 12-48: `max_alloc` is explicitly non-strict and some decoders may ignore it. Only dimensions are strict.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/codecs/jpeg/decoder.rs`: construction copies bounded file input into another Vec. `set_limits` checks dimensions, while `new_zune_decoder` forwards width/height only, not `max_alloc`. JPEG codec working allocations are not capped by that field.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/codecs/webp/decoder.rs`: no `set_limits` implementation forwarding an internal allocation budget; inherited implementation checks dimensions only.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-webp-0.2.4/src/decoder.rs`, lines 550-555: even its direct `set_memory_limit` documents that some allocations ignore the limit. Switching to that adapter alone does not fix the contract.
- `C:/Users/daniel/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/codecs/png.rs`: construction forwards `max_alloc` to `png::Limits`, checks header dimensions before `read_info`, but the image API still describes its allocation limit as best-effort. Post-construction `set_limits` has a TODO for forwarding changed allocation limits.

The passing admission-limit tests do not prove codec-internal memory stays within `max_decoder_bytes`. This is a dependency capability blocker, not an ignored failing regression. Supervisor must choose a verified hard-bounded dependency/allocation strategy or a separately scoped isolation design. A mere larger estimate or a silent relaxation to best-effort does not satisfy the assigned hard memory contract. No such product/platform change was made here.

## Evidence

All commands ran with explicit `cd C:/SANDBOX/WORKTREES/mado-explorer/image-preview`. Build output stayed in ignored `target/p01`. Component-local Cargo.lock is tracked; no root lock edit.

Latest coherent validation:

```text
cargo fmt --manifest-path components/image-preview/Cargo.toml --check
cargo check --locked --manifest-path components/image-preview/Cargo.toml --target-dir target/p01 --all-targets
cargo test --locked --manifest-path components/image-preview/Cargo.toml --target-dir target/p01
task --taskfile components/image-preview/Taskfile.yml
task --taskfile components/image-preview/Taskfile.yml run
task --taskfile components/image-preview/Taskfile.yml clean
git diff --check
git diff --cached --check
```

Outcomes: formatting and all-target compilation passed. Seven integration tests passed, zero failed/ignored. Library and doc-test targets have no additional tests. Task default listed all four described tasks; run built/launched the CPU probe and displayed usage without touching any input file. Task clean succeeded and removed only component build output from `target/p01`, reporting 678 files and 234.5 MiB. Working and staged diff whitespace checks passed.

Tests create their own uniquely named disposable directories and remove only those fixtures:

1. PNG/JPEG/WebP RGBA length, alpha, dimensions, detected format/file length, Unicode path, misleading extension, unchanged source bytes and released file handle.
2. Exact pixel order for all eight orientations in every format, including encoded versus corrected dimensions and unchanged source bytes.
3. Corrupt/truncated input in all three formats, malformed supported-extension input, unsupported GIF and unavailable native path.
4. Input, width, height, output, component peak and decoder-native-output admission limits, plus exact boundary success for each format.
5. JPEG SOF declaring 65535 x 65535 rejected before pixel decoding, without allocating its declared image.
6. Orientation-corrected dimensions rejected when the rotated width cannot fit.
7. Windows filename containing an unpaired UTF-16 surrogate decodes without lossy conversion; missing-file error retains that exact native path.

No GUI, stale-result, window-budget, cache eviction, GPU, zoom/pan, scaling, Narrator or responsiveness check ran. No hard internal-memory measurement or guarantee was obtained. Task clean was executed successfully against only the generated `target/p01` build directory. Rebuilding is required for subsequent checks.

## Acceptance coverage and next work

- F07.1 partial: supported-format backend decoding and unsupported fallback errors, no pane/selection/preview UI.
- F07.2 partial: orientation and dimensions/file-size metadata only. Fit/100%/zoom/pan remain integration work.
- F07.3 pending: synchronous backend is ready for a caller-owned worker, but no background scheduling, resize or stale-result routing is implemented.
- F07.4 partial: malformed/unavailable errors and strict dimension/output/component-buffer admission pass checks. Hard decoder working budget and visible state integration remain unresolved.
- F07.5 pending: selection/folder-summary behavior is not part of this component.
- F07.6 partial: per-call buffer admission, read-only access and drop-based resource release. Window-wide cache/CPU/GPU reservation and eviction are supervisor-owned.

First resolve the codec memory decision. Then review this exact candidate before promotion. GUI scheduling, pane following, stale-result guards, shared budgets, resizing and texture ownership remain outside this lane.
