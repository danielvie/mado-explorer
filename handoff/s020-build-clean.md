# S020 root build cleanup

## Candidate and boundary

Checkout `C:/SANDBOX/WORKTREES/mado-explorer/build-clean`, branch `lane/build-clean`, unchanged baseline `6a5348c6fbfa661fb511f25beafd85b1be4a1dd1`. Clean implementation commit `e6497ad984c593fcc0ac5fffd8c2e9765adbc283`; this handoff is a subsequent documentation-only commit. Final frozen head is in the worker receipt.

Only `Taskfile.yml`, `validation/build-clean.test.cjs` and this handoff changed. Existing run/test/design/default commands and descriptions remain intact; `test:clean` is appended to default tests. No Rust, dependency, source-gate, private-capability or release-criterion changes.

## Behavior

Root clean now runs three native offline Cargo commands:

```text
cargo clean --target-dir target --profile dev --offline
cargo clean --target-dir target --profile release --offline
cargo clean --target-dir target --doc --offline
```

Scope is root app dev/debug, release and documentation output. Auxiliary component builds, retained fixtures/comparison data and other target siblings remain. No component discovery, custom-profile cleanup or deletion fallback added.

The standard-library Node test copies the actual current Taskfile into a create-new unique temporary Rust project. It guards absolute owned paths, sets Task's explicit directory/taskfile and child cwd to that fixture, invokes installed Task offline, checks retained bytes before artifact removal assertions, then removes only its established owned fixture after child completion. Uncertain ownership/completion retains and reports it.

## Evidence

All commands ran from the owned checkout above. Node `v24.15.0`, Task `3.50.0+49932d0`, Cargo `1.95.0`.

- RED before Taskfile edit: `node validation/build-clean.test.cjs`, exit 1. Actual copied command `cargo clean --target-dir target` removed 11 generated files, 585 bytes. Assertion failed: `Retained file missing: target/p22/fixtures/owned-data/sentinel.bin`. Task ran with explicit `--dir` and `--taskfile` in `C:/Users/daniel/AppData/Local/Temp/mado-s020-clean-so1RqV`.
- GREEN after edit: `node validation/build-clean.test.cjs` and `task test:clean`, both exit 0. Fixtures `mado-s020-clean-casywJ` and `mado-s020-clean-YnjsYa` under the same temp parent. Each actual copied Taskfile invocation removed six fake build artifacts, 180 bytes; eight retained files were byte-identical. These cover target/p22/fixtures, target/visual, auxiliary target/p22/build, another target sibling, manifest, Taskfile, source and design markers. All three unique fixtures were removed after normal child completion and ownership recheck.
- `task --list`, `task --dry clean`, `git diff --check`, staged diff check: pass. Dry-run printed only the three commands, with no cleanup execution. Final diff inspected. No full suite or compile needed for this Task-only correction.

## Promotion limits

Supervisor separately disabled MAIN clean at `a4c97fe` with a no-I/O warning/exit 1. That historical protection was not merged into this baseline; integration conflict is expected. Fresh independent cleanup review and supervisor disposable validation are required before replacing it with this candidate. No actual repository/worktree target, personal folder or retained evidence was deleted. Fake-artifact tests do not claim runtime activation, native capability or application acceptance.
