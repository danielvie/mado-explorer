# P21 capped image backend

## Frozen candidate boundary

Checkout `C:/SANDBOX/WORKTREES/mado-explorer/image-capped-service`, branch `lane/image-capped-service`. Starting HEAD `a88943dc077179e0f70f04e0b0fe939df0b15112` and tracked/untracked status were clean. The candidate is the commit containing this document. Only `components/image-preview-service/**` changes. Root AGENTS/SPEC/CONTEXT/PROGRESS S016 P21/LANES and accepted PDF service were read-only. No approval, integration or full F07 claim.

## Implemented backend

- One process-wide service admission gate, one fixed window worker, 16 opaque never-reused targets, fresh generations, one coalesced FIFO request per target and one result mailbox per target. Active replacement joins the tail; stale/cancelled/closed results cannot publish. Public request/cancel/close/take-current/stop/poll-join do no filesystem/process/provider waits. Start resolves only our installed helper sibling, never a caller-selected executable. Cargo's `deps` adjustment exists only in the library test build.
- PNG/JPEG/WebP decode occurs only in private `src/helper/decoder.rs`, compiled into OUR `mado-image-helper` binary. Our preserved P01b decoder was narrowly adapted to accept the already-admitted read-only File, not reopen a path. Existing input/dimension/area, JPEG phase, WebP range/frame/partition, native-output, conversion and all orientation guards remain. The parent has no decoder API or decoder module.
- The private duplex named pipe uses a random anonymous-to-child name, first-instance and remote-client refusal. Parent creates both ends before launch. The only inherited handle is the connected private client, also explicitly used for stdin/stdout/stderr. Empty child environment, no shell, no GUI/settings/font startup and no source path in argv. Both parent transfers and helper pixel writes are at most 16 KiB per operation.
- Version 1 fixed request/response headers carry target/generation, native UTF-16 units including unpaired surrogates, status, format, orientation, encoded/corrected dimensions, file length, exact payload/error length. Parent checks arithmetic, 8192-per-axis, 32 MiB RGBA area, 16 MiB encoded length, 1024-byte UTF-8 errors, version/identity/orientation, truncation, extra frames and EOF plus actual process exit before acceptance. Child error strings are not retained in the parent.
- Lexical source admission precedes I/O. Absolute drive/UNC and verbatim filesystem spellings are admitted losslessly; devices, pipes, DOS filename aliases, NUL, dot components, alternate streams, slash/trailing aliases and unsupported namespaces refuse. Conservative spelling limits are explicit, not lossy normalization. Source `NtCreateFile` uses read-only FILE_OPEN, OBJ_DONT_REPARSE, FILE_OPEN_REPARSE_POINT and non-directory/synchronous options. Disk/regular/non-reparse attributes are checked on that same handle. No following reopen fallback. Source handles deny write/delete sharing during snapshot reading.

## Hard native containment and lifetime

Windows-sys `=0.61.2`, installed local features only. Job hard process AND job commit limits are 128 MiB, active process count 1, kill-on-close, no breakaway flags. STARTUPINFOEX JOB_LIST assigns the job DURING CreateProcessW, with CREATE_SUSPENDED. Own Job membership and exact native limits are queried before ResumeThread. Every error after creation terminates the owned process/job, waits for real exit and observes Job ActiveProcesses == 0 before refund. Direct process termination also covers an unconfirmed-membership suspended-child error. No running-then-assign path.

Worker-side overlapped reads/writes wait on their private completion event plus the service control wake. Cancellation can reach a helper while IPC is pending. Every submitted OVERLAPPED, event and borrowed payload survives CancelIoEx and the actual GetOverlappedResult completion acknowledgement, including unwind. A closed pipe is NOT process termination: exit waiting also listens to the control wake. Child Drop terminates/reaps before another request can launch. No per-query production threads or retiree list. Native termination/acknowledgement has no deadline promise.

Stop signals only; poll-join requires Rust completion AND a zero-time native worker-thread termination probe, after child/IPC cleanup and mailbox/queue release. Service Drop is a final blocking ownership safety net. A future UI owner must stop/poll before Drop, not drop synchronously during an ordinary update. A panic cleans bounded state, preserves native/lease ownership through unwind, and is observable through the join result. No early cleanup acknowledgement.

Job containment is a RESOURCE limit, not a filesystem security sandbox. It does not restrict the helper's user token or promise total Windows/kernel/system memory containment. Trusted deployment of the own helper sibling remains an installation requirement.

## Shared CPU accounting and concrete control bound

The ONLY counter is a clone of the accepted `mado_pdf_preview_service::CpuBudget`, with its window limit of 256 MiB. No second budget and no dummy helper-sized Vec. The service reserves 8 MiB control capacity before state/path/wake/worker allocation. The single private native launch function reserves the full 128 MiB helper cap and full 32 MiB output ceiling on that SAME counter before any process setup/creation, including every native diagnostic launch. The unused output reservation stays in Child until validated-header consumption or real reap. A failed admission never launches. Existing PDF allocations and caller-held image frames remain counted.

The 8 MiB envelope covers the whole service, not each request. On the supported Windows/Rust Global Vec backend, the conservative component-owned peak below is less than 6 MiB:

| Live storage | Conservative ceiling |
| --- | --- |
| 16 pending native paths, 1 active path, 1 transient replacement path; each <=32766 UTF-16 units | 1.125 MiB |
| At most one UI lexical admission and one worker executable admission simultaneously: native-name growth and split-slice Vec growth/replacement | 1.875 MiB |
| Stored deployment path, current-exe/startup temporaries, executable UTF-16, command line, attributes <=64 KiB, handle arrays | 0.75 MiB |
| Active request wire <=65572 bytes, headers/errors, slots/deque/Arc/thread/control records and formatting temporaries | 0.25 MiB |
| Fixed 512 KiB worker stack plus guard/thread bookkeeping allowance | 0.625 MiB |
| Requested duplex IPC buffers 64 KiB each, one <=16 KiB native transfer, stable OVERLAPPED/event/native structs and remaining bounded IPC allowance | 0.5 MiB |

UI admission is serialized by try-lock BEFORE path allocation, so concurrent callers cannot multiply validation copies. Queue/slot cardinalities never grow beyond 16. There is only one child/IPC operation at a time. Native attribute-list sizes above 64 KiB refuse. No dynamic parent error strings or returned path copies. The table charges requested IPC storage and component-owned native buffers/records, not undocumented Windows kernel/loader implementation memory or all allocator bookkeeping. The remaining >2 MiB envelope is not another available request/output allocation. Native path construction may refuse near-maximum spellings when the NT prefix itself would exceed the native length ceiling.

Control storage/native handles remain owned under the control reservation until destruction. Helper reservation is the Child's last field, after native termination/reap and pipe/process/job handle destruction. Validated payload size consumes the output reservation into accepted P20 exact-capacity CpuBuffer, then ReadFile writes DIRECTLY into it. No intermediate pixels or double charge. Output ownership can escape only as noncloneable Frame with a leased buffer, independent of the service/target lifetime. Payload memory frees before its budget refund. GUI/GPU/staging/caller-created unleased copies are not claimed here.

## Actual validation

All commands used explicit `cd C:/SANDBOX/WORKTREES/mado-explorer/image-capped-service`:

```text
cargo generate-lockfile --offline --manifest-path components/image-preview-service/Cargo.toml
cargo fmt --manifest-path components/image-preview-service/Cargo.toml
cargo fmt --manifest-path components/image-preview-service/Cargo.toml --check
cargo check --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --all-targets
cargo build --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --bins
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 -- --nocapture
target/p21/debug/p21-selfcheck.exe
git diff --check
git diff --cached --check
task --taskfile components/image-preview-service/Taskfile.yml --list
task --taskfile components/image-preview-service/Taskfile.yml run
task --taskfile components/image-preview-service/Taskfile.yml test
```

Coherent final run: format/check/build pass without warnings; 17 library checks + 6 helper guard/protocol checks pass, zero ignored. Own self-check passes malformed-source fallback, exit/reap, exact source bytes and budget release. Expected caught unwind prints one panic line but that test passes. Generated fixtures remain under ignored `target/p21/fixtures`; no clean task ran. No personal source/clipboard, elevation, network, external executable, desktop/UI, redistribution or GPU probe occurred.

Actual native evidence, distinct from compile/mocks:

- Job cap probe: contained helper committed 64 MiB successfully, freed it, then one bounded 144 MiB VirtualAlloc attempt was refused with Win32 **1455**. Last coherent full-run Job peak **69,033,984 bytes**, below 134,217,728. Exact limits and membership were observed before run.
- Setup-failure probe measured actual primary suspend count **1** before run, verified own Job membership, injected an error before ResumeThread, then observed the retained process handle signaled with exit **995** before budget refund.
- Genuine child access-violation exception exited **0xc0000005**; parent continued. A separate actual 144 MiB Rust allocator attempt produced the exact fatal allocation diagnostic and natural native exit **0xc0000409**. Parent/source survived. Production rejects fatal stderr as invalid protocol and may terminate before abort completes; no arbitrary exit or parser failure is labeled MemoryLimit.
- Real window-worker pending IPC was cancelled/replaced/closed/stopped; completion/reap/budget counters drained and observed peak active children was exactly **1**, including retirement. EOF-before-exit cancellation and pending-operation unwind also pass.
- Nonallowlisted inheritable PRIVATE event: child SetEvent could not signal that exact parent kernel object. Handle-slot numeric reuse is not treated as inheritance. The allowlist's sole IPC entry is also inspected in source.
- A functioning generated ancestor junction has a positive ordinary-read control; nonfollowing native source admission refuses it with **4395** on this host. Same regular read-only handle denies a generated writer with **32**. Bytes/hash and modified-time checks pass for generated format fixtures.

Focused commands also passed for native_job_cap_enforcement_assignment_before_run_and_reap, actual_worker_cancel_close_stop_and_fair_replacement_never_overlap_helpers, eof_before_process_exit_remains_cancellable, actual_child_allocator_failure_survival_has_native_exit_evidence, native_request_header_oversize_missing_helper_and_nonfollowing_sources, nonallowlisted_inheritable_private_event_does_not_reach_child and bounded_coalescing_fifo_close_cancel_stop_and_recreation.

Historical failed checks retained honestly:

1. First full run rejected generated fixture paths containing `../..`; fixture-only canonicalization corrected the harness, without relaxing source admission.
2. Numeric GetHandleInformation-only inheritance assertion encountered a child-local reused slot. Replaced by the stronger exact parent-event state probe, not a weaker numeric assumption.
3. Initial allocator probe used the production parser and raced abort with prompt Job termination, giving MalformedProtocol. The stronger separate probe now requires exact fatal stderr AND natural native exit; the actual worker separately proves its honest fallback and next successful decode.
4. The junction test initially expected relative-open code 1921. Actual absolute NtCreateFile returned 4395. Positive ordinary-read control plus exact native refusal now prove the intended nonfollowing behavior. No native cap/creation/association failure was waived or skipped.

## Attribution, pins and remaining gates

Original old checkout was separately rechecked at clean `beae3769906bcec2d7309dd09e88d4d02892708c`. Its decoder/admission/WebP audit and handoff remain untouched at [P01b](C:/SANDBOX/WORKTREES/mado-explorer/image-preview/components/image-preview/HANDOFF.md), preserved `beae376` / `archive/p01-unbounded-legacy`. That old public decoder must never execute in the parent/root app. New private decoder comments retain its attribution and best-effort allocation caveat. `image=0.25.10`, `image-webp=0.2.4`, `png=0.18.1`, `zune-jpeg=0.5.15` remain pinned. No codec fork/vendor copy or foreign binary was added. Upstream codec/transitive notices and packaged-helper licensing/redistribution checks remain pending packaging work; do not discard their notices. The accepted PDF reservation API is an unchanged read-only dependency; no PDFium renderer is initialized by this backend.

Backend portions of F07.1/F07.2/F07.3/F07.4/F07.6 have end-to-end evidence here. Full criteria remain incomplete. Fresh unsafe-code/protocol/shared-budget review is required before promotion. Actual GUI selection/pane routing, fit/100%/zoom/pan integration, cache eviction, GPU 128 MiB and staging accounting, DPI/Narrator/File Pilot, deployment/packaging and personal-data enablement remain pending. WebP fixtures are lossless; lossy WebP and animated PNG/WebP default-frame end-to-end fixtures remain unexecuted, as in the retained legacy notice. UNC/network/provider, maximum long-path and hostile deployment checks were not executed; conservative path policies and fail-closed native setup remain visible limits. No supervisor status/tracker edits or self-approval.

## P21-R01 relative UNC admission correction

Started clean on `lane/image-capped-service` at `1f00d57483f10683779d6bca44e89d87859f8bfb` in the exact checkout above. Only `src/source.rs` and this handoff changed. Root P21-R01 commission, AGENTS, SPEC and CONTEXT were read-only.

Accounting review `186df890` found that `admit` stripped optional `\\?\` without retaining its presence. Static relative input `UNC\server\share\x.png` then matched the case-insensitive `UNC\` branch and became absolute NT `\??\UNC\server\share\x.png`. `Service::request` used this guard, so the original admission could select an unrelated network source. No network access was used to reproduce the defect.

The guard now remembers the actual verbatim prefix and requires it for the `UNC\` branch. Ordinary double-backslash UNC and verbatim UNC remain admitted. No normalization, spelling compatibility expansion, API, decoder export, process/controller, budget, IPC or native open change. Parent and private helper still import the same `source.rs`. Private helper main/decoder/probe code and all 23 original test executions are unchanged. Existing device/pipe/ADS/dot/trailing/DOS, bounds, non-directory and nonfollowing guards remain unchanged.

Two inline pure tests cover bare uppercase/lowercase/mixed-case `UNC\` rejection and exact ordinary/verbatim UNC output, including mixed-case verbatim prefixes and separated unpaired high/low UTF-16 units. They call only `admit`, never `Service`, `open` or any filesystem/network function. Existing namespace/native-unit assertions remain intact.

Commands ran from `C:/SANDBOX/WORKTREES/mado-explorer/image-capped-service`:

```text
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 relative_unc_is_rejected_before_io -- --nocapture
cargo fmt --manifest-path components/image-preview-service/Cargo.toml --check
cargo check --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --all-targets
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 source::tests -- --nocapture
cargo build --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --bins
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 -- --nocapture
git diff --check
git diff --cached --check
```

Red on original guard: exit 101, one failed library test at the bare uppercase input, no native I/O. Green after correction: six focused source executions pass, three per parent/helper. Formatting, locked offline all-target compile and binary build pass. One coherent final full run passes 19 library + 8 helper tests, 27 total, zero ignored. This retains the original 17 + 6 and adds two tests compiled in each binary. Expected caught pending-unwind panic remains a passing check. Full diff inspected; no other source/config/lock/build-task changes.

Native evidence comes only from the unchanged owned tests, not from the lexical UNC tests. The existing hard-capped helper refused its bounded 144 MiB VirtualAlloc attempt with 1455 under the unchanged 128 MiB Job cap; this run's Job peak was 69,038,080 bytes. Existing setup-before-resume, crash/allocator-exit, cancellation/reap, private-handle and read-only/nonfollowing checks pass. No new native case or larger allocation was introduced; the standalone self-check was not rerun.

SourceNS limits remain conservative: <=32766 native path units including resulting NT spelling, no live UNC/provider/maximum-path compatibility evidence. Source namespaces are admission policy, not a user-token security sandbox. No personal/network/UI/clipboard/elevation/foreign-executable probe, tracker change, app-preview enablement or full F07 claim. Corrected-head namespace review, supervisor approval and promotion remain pending. Original native review `529b25b5` does not waive that source gate.

## P21-R02 malformed verbatim namespace correction

Started clean at `16a4ba176a069080c95c429b657dc70e1b66e3fa` on `lane/image-capped-service` in the exact checkout above. Read root AGENTS and supervisor PROGRESS P21-R02 read-only. Only `src/source.rs` and this handoff changed. This candidate requires a new fresh namespace review, not self-approval.

Independent namespace review `8fbc52f0` and parent inspection confirmed a pre-existing residual P2, not a regression from the bare-relative UNC fix. Exact original UTF-16 units:

```text
[92,92,63,92,92,92,115,101,114,118,101,114,92,115,104,97,114,101,92,120,46,112,110,103]
```

After stripping the verbatim prefix, the ordinary double-backslash branch accepted the remaining spelling and invented NT `\??\UNC\server\share\x.png`. That network object was not identified by the input's verbatim namespace. The branch now requires `!verbatim`; the verbatim UNC branch still requires the actual prefix plus case-insensitive `UNC\`. No normalization or namespace expansion was added.

One inline exact-unit regression calls only `admit`. It never calls Service, source open, or any filesystem/network operation. With only the test added to the original guard, both parent and helper failed its rejection assertion; the red command sequence exited 101. After the one-condition correction, all eight focused source executions pass, four per parent/helper. Existing bare-relative uppercase/lowercase/mixed-case UNC negatives, ordinary UNC, mixed-case verbatim UNC, absolute drives and unpaired native units remain green. Device/pipe/dot/ADS/trailing/DOS, length and nonfollowing guards are unchanged. Parent admission, helper admission and executable pinning still share this source guard.

Commands ran from `C:/SANDBOX/WORKTREES/mado-explorer/image-capped-service`:

```text
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --lib verbatim_double_backslash_is_rejected_before_io -- --nocapture
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --bin mado-image-helper verbatim_double_backslash_is_rejected_before_io -- --nocapture
cargo fmt --manifest-path components/image-preview-service/Cargo.toml --check
cargo check --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --all-targets
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 source::tests -- --nocapture
cargo build --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 --bins
cargo test --locked --offline --manifest-path components/image-preview-service/Cargo.toml --target-dir target/p21 -- --nocapture
git diff --check
git diff --cached --check
```

Format, locked offline all-target compile and owned binary build pass without warnings. One coherent final full run passes 20 library plus 9 helper tests, 29 total, zero ignored. All 27 existing executions are retained unchanged; the new test runs in both binaries. The expected caught pending-unwind panic prints and passes. The standalone self-check was not rerun. Final diff inspection covers only the one guard condition, one test and this appendix; no dependency, Taskfile, lockfile, service, protocol, decoder, native or retained legacy changes.

Native evidence is from the unchanged owned suite only, distinct from the pure lexical regression. Existing 128 MiB Job containment admitted the 64 MiB commit and refused the bounded 144 MiB VirtualAlloc attempt with 1455; Job peak was 69,009,408 bytes. Suspended setup failure observed count 1 and exit 995 before refund. Natural allocator fatal exit `0xc0000409`, access violation `0xc0000005`, cancellation/reap/EOF/unwind, private-handle isolation, shared accounting, codec and source-preservation checks pass. The functioning generated ancestor junction still refuses with 4395. No new native cases or allocation probes were added.

No remote open/probe, personal-data, clipboard, elevation, UI, network compatibility or maximum-path experiment occurred. API, budgets, Job, cancellation/runtime, native opens and nonfollowing policy are unchanged. Original native `529b25b5` and accounting `186df890` reviews of other areas do not approve this corrected namespace. Source admission remains policy, not a user-token security sandbox. Fresh corrected-head namespace review, supervisor approval, promotion, deployment/licensing, GUI/GPU/staging and all outstanding full F07 criteria remain pending; no production gate or acceptance status changed.
