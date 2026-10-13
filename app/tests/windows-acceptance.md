# Windows implementation and acceptance matrix

This is a coverage map, not a pass certificate. It separates implemented surfaces, automated checks present in source, and deferred user-dependent acceptance. Actual command results, including the verified full chain and separate post-guard rerun status, belong in [tasks/todo.md](../../tasks/todo.md). Exact deferred procedures, devices, expected results and build fields are retained in [tasks/deferred-tests.md](../../tasks/deferred-tests.md). Every manual scenario below is **DEFERRED / NOT VERIFIED**, unless later replaced by dated direct evidence.

Baseline: Windows 11 x64, Rust 1.95, Wayfarer Files 0.1.0. **Final verification PASS:** format, strict Clippy, 71 automatic tests (64 unit + 7 integration; 9 ignored), debug/release builds, ten five-frame debug probes and one release probe. Explicit native suite: 9/9 passed before the final root-only concurrency guard; native adapters were unchanged and that native run was not repeated. Release artifact: 16,210,944 bytes; SHA-256 `ECA98B6213FBB9DF2C9ADE3FA5593A393E261613F778BD4057F736EA54906787`. Earlier failures and fixes are preserved chronologically in tasks/todo.md. All user-dependent manual acceptance remains DEFERRED. No File Pilot execution or equivalence claim; named test fragments below locate coverage, and empty filtered runs are not evidence.

| Task | Implemented surface | Automated coverage present / verification boundary | Deferred manual acceptance |
| --- | --- | --- | --- |
| T01 | Window, pinned dependencies, application tasks | Executable `--window-boot-probe`; prior ten-cycle foundation record | Launch/close, visible window behavior |
| T02 | HWND/OLE ownership, callbacks, pre-destruction detach | `native_runtime_balances_nested_ole_initialization`; executable probe | Keyboard/mouse continuity, Narrator, DPI |
| T03 | Bounded async directory generations, virtualized details | `browsing_owned_async_filter_generation_and_navigation`; `browsing_virtualization_ten_and_hundred_thousand_rows` | Large real directory responsiveness, inaccessible directory |
| T04 | Authoritative selection/range/cursor, activation | `selection_toggle_range_and_prune`; `selection_range_uses_current_order_and_control_moves_focus` | Keyboard/IME/focus and vanished-file activation |
| T05 | Native Shell menu, IContextMenu2/3 forwarding | `shell_menu_selection_rejects_cross_folder_and_empty`; no claim of extension invocation coverage | Shift+F10, dynamic submenus, read-only Properties |
| T06 | STA file-operation worker, outcomes/cancellation/close coordination | `operations_copy_*`, opt-in `native_operations_owned_fixture` / `native_operations_owned_workflow` | Shell dialogs, locks/collisions, cancellation and closing mid-copy |
| T07 | Exclusive OLE target, bounded extraction, read-only probe mode | Drop-target routing tests in `platform/drag_drop.rs`; executable smoke probe enables read-only mode | Explorer probe drops across panes and DPI |
| T08 | Shell outgoing data/source lifetime, copy/move effects | Path validation/routing tests; live negotiation is not simulated proof | Explorer copy/move modifiers and cancellation |
| T09 | Substring/logical sorting/visibility/navigation | Directory tests; 10k/100k viewport-row tests | Real-folder timing/memory baseline and responsiveness; record measured budgets |
| T10 | Independent new/duplicate/reopen tabs and final Home safeguard | `browsing_duplicate_and_reopen_preserve_independent_owned_state`; `browsing_close_reconciles_active_and_retains_home_tab`; `tabs_keyboard_switch_stays_within_active_pane` | Overflow reachability, rapid switching/closing |
| T11 | Split/resize/equalize/pane focus, bounded depth | `browsing_pane_switch_and_equalize_preserve_locations`; layout codec validation | Keyboard pane targeting and live resize |
| T12 | Typed action eligibility, searchable palette, focus-aware routing | `command_palette_context_eligibility`; hotkey validation | Text editor/IME precedence, full keyboard command flow |
| T13 | CF_HDROP/text publication, bounds and owned memory | `clipboard_copy_round_trip_lossless_unicode`; malformed-offset/terminator test | Explorer round-trip, external clipboard replacement |
| T14 | Captured cut/paste request, conditional completed-item removal | Clipboard parser and operation outcome tests; native move fixture where applicable | Cross-volume partial/cancel outcomes and clipboard races |
| T15 | Separate Trash, recycle-only callback guard | `trash_denies_non_recycle_callback`; opt-in owned workflow | Recycle unavailable/cancel/partial scenarios; unrelated Bin unchanged |
| T16 | Validated file/folder creation and result refresh/reconciliation | `create_entry_rejects_invalid_and_reserved_names`; native owned workflow | Dialog cancellation, destination disappearance, result selection |
| T17 | Single rename, full name editing and explicit outcomes | `rename_entry_accepts_unicode_and_extensions`; native owned workflow | F2/editor precedence, collision and vanished source |
| T18 | Bounded read-only UTF-8 inspector, cancellation/metadata | `text_preview_limits_and_encoding`, bound/clear/cancellation tests | Binary/inaccessible UI fallback, text selection/clipboard/focus |
| T19 | WIC owned premultiplied pixels and checked limits | `image_decode_bounds`; opt-in corrupt/BMP/alpha-PNG native tests | Broader installed codec/error cases |
| T45 | Image texture lifecycle, zoom/fit, per-tab inspector options | WIC alpha test, preview generation tests, inspector DTO round-trip | Displayed alpha/channel correctness, both panes, texture lifetime under rapid changes |
| T20 | Watcher invalidation/coalescing/retry and visible failures | `watch_refresh_retains_visible_selection_and_prunes_confirmed_outcomes`; directory tests | Explorer event bursts and authorized share reconnect |
| T21 | Independent lossless typed session/location/layout/preferences codec | `session_codec_*`, `recycle_bin_location_codec`, preset/geometry validation | Review representative files and warnings |
| T43 | Synced same-directory temp plus Windows replacement | Locked replacement/corrupt/future preservation; abandoned-temp reopen simulation | Real interrupted-process coverage remains unverified; no power-loss guarantee |
| T44 | Writer lease, startup/close save, monitor-clamped geometry | `session_lifecycle_*`; `session_window_geometry_clamps_and_rejects_invalid_input` | Restart, missing paths, second process and changed-monitor restoration |
| T22 | Pane-targeted synchronous Shell transfer authority | Routing/validation tests; executable native lifetime probe | Real Explorer copy/move/cancel/collision and mixed-DPI coordinates |
| T23 | AccessKit-enabled toolkit, labels and keyboard paths | Headless interaction/routing tests cover portions only | Full Narrator/IME/high-contrast/100–150–200% DPI workflow |
| T24 | Release manifest/build target, independent executable/docs | Manifest XML parsed; final release/link/build PASS recorded in canonical ledger | Run release outside source tree with fresh state; support matrix acceptance |
| T25 | Owned searchable Shell command projection and native fallback | `shell_search_labels_preserve_literal_ampersands`; source eligibility checks | Installed extension/dynamic menu invocation and native fallback; query-lazy bounded projection starts on the first nonempty query; per-submenu compatibility remains unverified |
| T26 | Typed Recycle Bin pane, checked properties and persisted kind | `bin_listing_*`, `bin_location_never_exposes_filesystem_actions_or_drop_targets`, codec test | Owned Bin entries/property fallbacks and restart |
| T27 | Selected revalidated Shell restore/delete identities | Selection validation tests; no automated global-bin mutation | Collision/cancel/disappeared item; unrelated Bin untouched |
| T28 | Descendant paths/depth, no reparse traversal, cancellation | `expanded_listing_duplicate_names_depth_and_cancellation`; cancellation-publication test | Junction/access-denied hierarchy and navigation during scan |
| T29 | Requested bounded folder totals; explicit network policy | Statistics fixture/cancellation tests | Device/network policy and denied-subtree observation |
| T30 | Visible-row icon grid, bounded thumbnail cache | `icon_view_virtualizes_hundred_thousand_items`; `thumbnails_downsample_bounds_and_content` | Large image scrolling/cache eviction/DPI/zoom |
| T31 | Mutation-free rename proposal, explicit numbering/date policy | `rename_plan_*`, owned preview/revalidation fixture | Preview every path/options/selection without mutation |
| T32 | Revalidated ordered rename operations and per-item outcomes | Rename identity/conflict tests; native operation workflow | Partial commit/cancel/races and final-path selection |
| T33 | Named layout validation, overwrite/delete confirmation, session persistence | `layout_presets_codec_and_validation`; split mapping tests | Save/load/cancel, missing locations and preference independence |
| T34 | Lossless bookmark groups/reorder/filter/open/new tab | `bookmarks_lossless_roundtrip_and_reorder`; `bookmarks_filter_matches_labels_groups_and_paths` | Keyboard/accessibility, unavailable UNC and restart |
| T35 | Exact-path bounded MRU; Never/Session/Persistent/clear | `recents_bounds_privacy_and_independent_clear` | UI filtering/order/mode restart and independence from layouts/bookmarks |
| T36 | Bundled proportional/monospace, theme/size/density | `appearance_settings_bounds` | Contrast/DPI/focus; arbitrary external font loading is unsupported rather than pending |
| T37 | Double/single-folder/single-all and checkbox modes | `input_settings_editor_precedence` | Mouse/keyboard/IME each exposed mode |
| T38 | Checked Explorer/terminal launch and quoting | External-tool quoting/validation checks where present; inspect source test inventory | Launch actual tools in spaced/metacharacter owned paths |
| T39 | Explicit non-replacing .lnk creation, copy/move-only drag | Opt-in `native_shortcut_owned_fixture`; link drag remains disabled | Explorer shortcut open/collision/cancel; link effects require separate approval/interop |
| T40 | Separate captured-selection permanent-delete confirmation | `permanent_delete_requires_confirmation`; opt-in `native_permanent_delete_owned_fixture` | Exact owned-target confirmation/cancel; never a Trash fallback |
| T41 | Validated single-chord editing/reset and persisted bindings | `hotkeys_conflict_is_transactional_and_text_reserved` | Rebinding/conflicts while text/IME editors own focus |
| T42 | Combined implementation coverage and this support matrix | Final aggregate suite/native/release commands attached in canonical ledger | Extended workflow/accessibility/resource/interop release acceptance |

## Support boundaries and completion rules

Code existence and source test names do not complete an acceptance task. Record actual final command output, test counts, skipped tests, build/version and failures in the canonical checklist before marking automated checks passed. Keep each deferred manual checkbox unchecked. Technical defects and missing implementable behavior must be fixed or explicitly listed; they are not transformed into manual deferrals.

Use owned disposable fixtures. Never run FPilot.exe, empty the global Recycle Bin, change registry/PATH/default handlers or use real-user destructive test targets. Native mutation checks remain explicit opt-in and serialized. Permission to continue implementation is not permission for an unrelated external action.

The independent application supports ordinary Windows filesystem paths and a typed Recycle Bin view. General virtual-file streams, original app configuration, exact original visuals/options, link drag effects, arbitrary external font loading, installation/default-handler registration, Windows 10/ARM/non-Windows support and transactional rollback are not claimed. Searchable Shell menus initialize their bounded projection on the first nonempty query, with native fallback; extension compatibility must be verified per installed extension.

Performance distinction: synthetic 10k/100k tests establish viewport-bounded row construction; they do not establish physical directory enumeration speed, peak working set, network behavior or user-perceived responsiveness. Measured intermediate serialized baseline: details 10k/100k 58.1513/87.1859 ms and 25 constructed rows each; icons 60.9779/85.4647 ms and 5 rows each. Entire seven-test process PeakWorkingSet64 was 48,504,832 bytes (about 46.3 MiB), sampled every 10 ms. Struct-header estimates 960,000/9,600,000 bytes exclude heap buffers. Diagnostic thresholds: 250 ms for 100k construction plus three headless frames, and 128 MiB for the full headless test process. These are regression budgets, not user-facing speed guarantees. Full chronological evidence, including resolved native failures, is recorded in tasks/todo.md.


Final native command: `cargo test --manifest-path app/Cargo.toml --offline native_ -- --ignored --test-threads=1 --nocapture` — 9 passed, 0 failed, 3.31 s. Scope and failure/fix history remain in the canonical ledger; no manual acceptance is implied.
