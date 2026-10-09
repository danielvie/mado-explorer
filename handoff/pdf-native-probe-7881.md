# PDFium 7881 native probe

Supervisor evidence for candidate `418b02b5b90284bbfe06d3a186518bcfe7632283` on `lane/pdf-preview`. This is backend verification, not GUI acceptance or redistribution approval.

Subsequent independent review found a Windows x64 memory-loader ABI mismatch and unrestricted dependent-DLL search. This original passing fixture does not disprove either defect. P03b corrected both at f4d4d6c; its HANDOFF.md records corrected native/loader checks. Fresh independent review found no concrete blocker, and the unchanged reviewed component was promoted at 78eefc3. This document records the original probe only, not new native verification or GUI/redistribution approval.

## Artifact

Downloaded only to the lane's ignored `target/native-probe-pdfium-7881` directory:

- URL: https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F7881/pdfium-win-x64.tgz
- SHA-256 verified against the worker's documented release digest: `73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08`.
- Archive member names inspected before extraction; no absolute or parent-traversal members.
- Extracted `VERSION` reports 151.0.7881.0.
- Extracted `args.gn` reports x64 Windows, `pdf_enable_v8 = false`, `pdf_enable_xfa = false`, standalone release build.
- Top-level license and complete third-party `licenses` directory retained with the downloaded artifact. Full release notice review and security-update policy remain pending. No binary added to Git or application packaging.

## Commands and results

All commands start in `C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview`.

```sh
cargo fmt --manifest-path components/pdf-preview/Cargo.toml --check
cargo check --locked --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
cargo test --locked --manifest-path components/pdf-preview/Cargo.toml --target-dir target/p03
```

Passed formatting and compile. Seven default checks passed; native probe explicitly ignored in this default run.

```sh
mkdir -p target/native-probe-pdfium-7881
curl --fail --location --proto '=https' --tlsv1.2 --max-time 120 \
  --output target/native-probe-pdfium-7881/pdfium-win-x64.tgz \
  'https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F7881/pdfium-win-x64.tgz'
printf '73cc0de638ac2095e7445bf56a38200a5b7c7ca0e9f4ba144598f2457377ac08  target/native-probe-pdfium-7881/pdfium-win-x64.tgz\n' | sha256sum --check
tar -tzf target/native-probe-pdfium-7881/pdfium-win-x64.tgz
# Inspect member names before extraction.
tar -xzf target/native-probe-pdfium-7881/pdfium-win-x64.tgz -C target/native-probe-pdfium-7881
PDFIUM_LIBRARY_PATH='C:/SANDBOX/WORKTREES/mado-explorer/pdf-preview/target/native-probe-pdfium-7881/bin/pdfium.dll' \
  cargo test --locked --manifest-path components/pdf-preview/Cargo.toml \
  --target-dir target/p03 --test backend native_fixture_probe -- --ignored --nocapture
```

Hash check passed. Native probe actually ran: one passed, zero failed, zero ignored. Output:

```text
Native PDFium probe passed: RGBA channel order, metadata, resize, page bounds, corrupt/password input, limits and unchanged sources.
```

Generated fixtures only. No personal files or arbitrary external documents opened. The probe does not prove native-engine memory/time bounds, malicious-input sandboxing, pane/page/zoom UI, stale-result routing, shared budgets, display scaling, Narrator, clean-machine installation or final packaging. Fresh source safety review and serial supervisor promotion remain required.
