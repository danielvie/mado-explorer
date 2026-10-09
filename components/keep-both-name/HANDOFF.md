# P13 Keep both name planning

## Candidate and boundary

- Checkout: `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`.
- Branch: `lane/selection-summary`.
- Clean starting baseline: `1c50734fa578cec7f2067bab39d54094d81f4a1b`.
- Exclusive changes: `components/keep-both-name/**`. Accepted `components/fs-source-snapshot` is a read-only path dependency, unchanged. No root/shared/sibling/dependency edits, production integration, GUI, clipboard, source-content reads, transfer, reservation or enablement.
- Target: F10.5 unused-name planning portion only. No complete F10.5 claim, no acceptance-status edits, and no P02 rename correction claim.
- Terminal SHA is reported in the worker result, also available through `git rev-parse HEAD`. Fresh read-only exact-head safety review, parent inspection/approval, one-at-a-time integration and validation on updated main must precede any next task. This component commit is not its own approval.

## Exact API

Windows-only standalone `mado-keep-both-name`, Rust 1.95, std plus accepted `mado-fs-source-snapshot` path dependency. No added registry package. Its standalone lock includes only that dependency and its existing windows-sys/windows-link packages.

- `Planner::capture(original_basename: OsString, kind: EntryKind, destination: PathBuf) -> Result<Planner, PlanError>` consumes immutable native inputs. `EntryKind` is `File` or `Folder`. Basename validation and the first suffix's fit check finish before any destination I/O.
- Destination capture uses accepted `Snapshot::capture`, preserving its exact native path and native volume/file ID. Only an observed ordinary directory is admitted. A final file, reparse entry or other kind is refused. Snapshot namespace/provider/spelling restrictions remain in effect.
- Read-only getters: `original_basename() -> &OsStr`, `kind() -> EntryKind`, `destination() -> &Path`. No setter, raw handle, retargeting, source-content reader or mutation API.
- `Planner::plan() -> Result<Candidate, PlanError>` revalidates the captured directory name before candidate search. Every candidate is checked with `std::fs::symlink_metadata`, which does not follow the final link/reparse entry. All successful metadata kinds count as occupied, including dangling junctions.
- Only `io::ErrorKind::NotFound` means observed absence. Before returning that candidate, plan revalidates the original directory name against the same retained Snapshot. PermissionDenied, sharing, provider, malformed-path and other metadata failures remain `CandidateIo` with full native candidate path and the actual `io::Error`. Directory failures retain `SnapshotError`, including missing/replaced/unsupported.
- `Candidate` has private native basename/path/number and read-only `basename() -> &OsStr`, `path() -> &Path`, `number() -> u32`. Display formatting of errors is diagnostic only, never an input to commands.
- Public errors: `InvalidName(NameRejection)`, `CannotFit { number, extension_units }`, `Destination(SnapshotError)`, `NotDirectory(observed_kind)`, `CandidateIo { path, error }`, `Exhausted { checked }`. They implement Display and Error.

Capture and plan are synchronous metadata I/O and must run on a future operation worker away from the UI. No worker, GUI adapter, background latency bound or cancellation service is added here.

## Naming policy and limits

Names start at number 2 and use one ASCII space before the parenthesized decimal number: `report (2).txt`. The unsuffixed original is never selected, even when absent. Each call restarts at 2, so unchanged observations give deterministic results.

For files, extension means the last suffix, matching `Path::extension` on an admitted native basename. The entire final dot and suffix remain unchanged. Examples:

| Input and kind | First candidate |
| --- | --- |
| `report.txt`, File | `report (2).txt` |
| `archive.tar.gz`, File | `archive.tar (2).gz` |
| `.profile`, File | `.profile (2)` |
| `.profile.conf`, File | `.profile (2).conf` |
| `folder.txt`, Folder | `folder.txt (2)` |

The implementation inspects exact UTF-16 units and returns `OsString::from_wide`. Accents, supplementary letters, unpaired high/low surrogate units and complete extensions survive without display-string reconstruction. No Unicode normalization, case folding or guessed target case policy. Ordinary candidate spellings are resolved by actual native metadata observations in the fixed destination.

Supported component maximum is `MAX_COMPONENT_UNITS = 255` UTF-16 units. When suffix insertion exceeds it, only the right end of the stem is shortened. A valid high/low surrogate pair is never split at that boundary. Already-unpaired retained units stay unchanged. Stem may shorten to empty when the suffix and whole extension exactly occupy the component limit. If those two alone exceed 255, `CannotFit` refuses rather than truncate the extension. Suffix growth, such as 9 to 10, recomputes room and can refuse partway through search.

Names are rejected before I/O when empty, over 255 units, containing NUL/control units or Windows forbidden punctuation/separators/colon, or ending in dot/space. Reserved DOS checks match accepted Snapshot's conservative policy, including ASCII case variants, first-dot extensions, spaces before that dot, CON/PRN/AUX/NUL/CLOCK$/CONIN$/CONOUT$, COM/LPT 0 through 9, and superscript 1/2/3 aliases. Dot and parent names fail the trailing-dot rule. Conservative DOS ASCII comparisons only recognize reserved aliases; they do not rewrite input or decide collision equality.

`MAX_CANDIDATES = 1000`, numbers 2 through 1001. All occupied returns visible `Exhausted { checked: 1000 }`. No unbounded loop, alternate naming scheme, fallback overwrite or hidden retry. Destination paths inherit accepted Snapshot's strict absolute filesystem admission, component limits and provider-ID requirement. Long destination paths need a supported verbatim spelling. Full child-path/provider limits can still produce an explicit metadata I/O error.

## Observational, not atomic

A returned name is an absence observation followed by a matching directory identity observation. These calls are separate, not a transaction. Another process can create the candidate immediately after metadata or return. The Snapshot retains a handle but does not pin ancestors, prevent directory swaps, bind the child lookup to a directory handle, reserve the candidate or lock names. No content snapshot or filesystem-wide case/identity guarantee is claimed.

A future reviewed executor must enforce non-overwrite behavior at actual destination creation/commit, bind/revalidate its destination and sources, and treat later destination creation as a NEW conflict requiring a fresh choice. Reusing this candidate must never authorize silent replacement. This library neither supplies atomic reservation nor fixes the known P02 rename primitive. Existing operation/checkpoint/clipboard/production gates remain closed.

## Fixture safety and evidence

Fifteen built-in std tests, no framework. Every native check creates a uniquely named disposable root under the local temporary directory using `create_dir`, never adopting an existing name. It uses a retained native Snapshot to guard the root's identity and ordinary-directory kind before test mutations and cleanup. Generated paths stay below that owned root; dot/parent input is refused. Roots use verbatim native paths to exercise 255-unit components without losing full-path support. Fixture files are newly created empty files; no source or marker contents are read. Cleanup uses std Windows non-following remove_dir_all, guarded against replaced/reparse roots.

Passing coverage:

- Original occupied plus 99 numbered collisions selects 101. Native absence and unchanged before/after entry names prove no planner filesystem writes.
- Last-extension, compound-last-suffix, leading-dot, extensionless, repeated-dot and folder naming cases. Unused originals still receive suffixes.
- Japanese, accents, supplementary non-emoji letters and unpaired UTF-16 units in both stem and extension retain full native identity through capture/plan. Creating the returned owned name causes the next plan to select another number.
- 255-unit stem clipping, unchanged complete extension, empty-stem fit, valid surrogate-pair boundary and retained unpaired units. Suffix growth recomputes clipping or visibly refuses when the full extension cannot fit.
- Rejected basename entry points and impossible first-suffix fit perform zero destination capture calls and zero production metadata calls. Rejected planner inputs never reach native opens.
- Native case occupancy is compared against a separate metadata observation of this owned directory's actual behavior, rather than assuming lowercase equality. Distinct normalization spellings remain unchanged input.
- Synthetic native errors 5, 32 and 1117 remain CandidateIo, one attempted candidate each, retaining error code/path. A private metadata seam injects a real owned overlong-component native error, observed code 123/InvalidFilename, which also remains CandidateIo. This is not real ACL/provider-denial evidence.
- Exhaustion checks exactly 1000 numbered paths and preserves owned entries. A controlled private metadata closure supplies an occupied real fixture's metadata; no generic filesystem layer or production injection API is added. Directory capture/revalidation always remains real accepted Snapshot I/O.
- Destination disappearance/replacement before search makes zero candidate calls. Deterministic disappearance/replacement between absent metadata and final directory revalidation refuses the candidate. Captured destination path remains unchanged.
- Non-directory and unsupported path admission are refused.
- A nonprivileged owned junction with an independently owned target is genuinely dangling after target removal. Following metadata reports missing, non-following metadata reports its reparse entry, and the production planner selects the next candidate without writes. Final reparse destinations are refused. Independent target-root identity and an unread empty sibling sentinel remain intact.
- Repeated plans return the same unreserved candidate. Test-only creation of that candidate afterward leaves it intact and makes the next production plan choose a different name. This demonstrates the race is not solved by planning; no transfer/executor is implemented.

Junction creation is test-only. std OpenOptions owns a handle with GENERIC_WRITE, share mode 7 and OPEN_REPARSE_POINT/BACKUP_SEMANTICS flags on an owned empty leaf. One kernel32 DeviceIoControl call installs an aligned, bounded mount-point reparse buffer using FSCTL_SET_REPARSE_POINT. The handle closes through std File. No added windows-sys dependency, shell subprocess, privileged symlink attempt, elevation or production mutation code. The test removes its owned junction leaf before fixture cleanup.

Pending limits: privileged symbolic-link subtype, real ACL-denied/unavailable providers, case-sensitive-directory configuration and concurrent stress were not probed. The nonprivileged dangling-junction case did run and pass. No real directory, network, volume-root, device, personal path, GUI or clipboard probe. No performance/latency or mutation guarantee.

## Exact commands and results

Run from `C:/SANDBOX/WORKTREES/mado-explorer/selection-summary`:

```text
cargo generate-lockfile --manifest-path components/keep-both-name/Cargo.toml --offline
cargo fmt --manifest-path components/keep-both-name/Cargo.toml
cargo fmt --manifest-path components/keep-both-name/Cargo.toml --check
cargo check --manifest-path components/keep-both-name/Cargo.toml --target-dir target/p13 --locked --offline --all-targets
cargo test --manifest-path components/keep-both-name/Cargo.toml --target-dir target/p13 --locked --offline tests::ordinary_io_error_and_synthetic_denied_errors_are_not_absence -- --exact
cargo test --manifest-path components/keep-both-name/Cargo.toml --target-dir target/p13 --locked --offline -- --test-threads=1
task -t components/keep-both-name/Taskfile.yml --list
task -t components/keep-both-name/Taskfile.yml run
git diff --cached --check
```

Final formatting and locked offline all-target compilation pass. Focused error regression passes. Full owned-component suite passes 15 tests, zero failures and zero ignored. Task listing and run pass; run executes the one owned collision fixture self-check. Staged whitespace check passes. No full root suite, Clippy or GUI check. `task clean` was not executed; it is restricted to generated worktree-root `target/p13`.

Initial all-target compile found two test-only trait mismatches, corrected once by joining the borrowed OsStr and comparing to borrowed string text. First coherent native run passed 14 and failed one actual-error fixture assertion. Windows returned NotFound for file-as-ancestor, so that probe was replaced by owned overlong-component metadata. First focused fix expected native 206 incorrectly; a diagnostic-only run showed the actual preserved CandidateIo code 123/InvalidFilename. The second focused correction asserted that observed error exactly and passed. No production change, weaker error behavior, deleted regression, tool/model fallback or hidden passing claim. The final 15-test rerun passes.
