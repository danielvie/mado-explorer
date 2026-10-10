# C001 shared browsing extraction

Source baseline `1ecd1d8b32743b13eb54bd936c6e59ed46b03a95`. Candidate source commit `02881b34fbebce1fba2adef9f1bca2769c0f5376` on `lane/shared-core`. The following evidence-only commit changes no application or test source. This is a frozen candidate for supervisor review, not approval.

Checkout `C:/SANDBOX/WORKTREES/mado-explorer/shared-core`. Authorized current SPEC F15/F16, CONTEXT, AGENTS and PROGRESS were read from `C:/SANDBOX/REPOS/mado-explorer.git` without writes.

## Implementation and write boundary

Moved the existing modules into one independent library. The directory, normalization, native-display, fuzzy-filter, sort, selection, finder ranking/probe/expansion/catalogue and navigation algorithms were not rewritten. Comparison against baseline source found only the intended activation extraction, IDs import, focus-field relocation and test-support adaptations.

Owned changed paths:

- `crates/mado-core/.gitignore`, `Cargo.toml`, `Cargo.lock`, `Taskfile.yml`, `src/lib.rs`, `src/ids.rs`, `src/directory.rs`, `src/navigation.rs`, `src/native.rs`, `src/selection.rs`, `src/sorting.rs`, `src/finder.rs`, `src/test_support.rs`, `examples/browsing.rs`, `tests/browsing.rs`.
- Root `Cargo.toml`, `Cargo.lock`; `src/directory.rs`, `src/navigation.rs`, `src/native.rs`, `src/selection.rs`, `src/sorting.rs`, `src/finder.rs`, `src/main.rs`, `src/tabs.rs`, `src/panes.rs`.
- Supervisor-authorized expansion `src/create_tests.rs`, fixture naming and one parallel uniqueness check only.
- `handoff/c001-shared-browsing.md`.

No root Taskfile, tracker, backend, operation, watcher, settings, preview, design or gate edits. Root lock changes only add the local core package and move the existing time dependency behind it. Both locks are tracked. Core uses installed time 0.3.55 and windows-sys 0.61.2, with no new external library.

GUI compatibility modules import the real core types/functions. GUI Tab widget IDs, editor focus/drafts and pixel scrolling remain local. Pane geometry/layout remain local. Finder Modal wraps core state solely for `focus_input`; matching expansion responses restore GUI input focus as before. Pure native activation capture lives in core; GUI wrappers retain editor suppression. Original GUI activation/input assertions remain intact.

The shared `test-support` feature is enabled by the GUI dev-dependency only. Its generated row helper moved from selection tests, and synchronous projection helpers invoke the same core algorithm. No GUI dependency enters core, including test-support.

## Public API for the TUI

| Module | API and caller responsibilities |
| --- | --- |
| `ids` | `PaneId`, `TabId` are plain stable u64 identities. GUI reexports preserve existing call sites and widget identity. |
| `directory` | `Entry`, `EntryKind`, `Failure`, `FailureKind`, `Status`, `DirectoryState`, `Update`, `Response`; `DirectoryLoader::new`, `request_for`, `cancel`, `drain`. Allocate one window-owned loader. Use `TabId.0` as target; retain returned generation and route responses through Navigation. |
| `navigation` | `Navigation` owns committed/pending locations, history, selection, filter, sort, visibility and projection freshness. `begin`, `apply`, `apply_sorted`, `resolve`, `up`, `deactivate`, `key`, `rows_selectable` retain original failure/stale semantics. `Activation`, `capture_activation`, `active_activation` capture complete native paths from fresh visible rows. Frontend suppresses commands while an editor owns input. |
| `sorting` | `Column`, `Sort`, `Key`, `Sorted`, `Sorter::new`, `request_for`, `cancel`, `take`. Send current native entries, query and key to the shared worker. Route returned globally unique listing-generation keys to their contexts; `Navigation::apply_sorted` refuses stale data/query/sort/visibility results. Do not implement frontend filtering or sorting. |
| `selection` | `Selection`, `Movement`; click/range/toggle/all/movement/reconciliation are path-based. Pass the fresh displayed order. Frontend owns viewport movement and input mapping. |
| `finder` | `Location`, `Match`, `Catalogue`, `Target`, `Modal`, `Action`, `Work`, `Output`, `Response`, `Service`, `rank`, existing bounds. `Service::new` supplies normal Windows known places; `with_inventory` supports bounded owned checks without personal inventory. Core Modal owns query/path mode/results/revisions/validation; frontend owns focus and must revalidate captured pane/tab/base/generation before applying an accepted action. Workspace ownership is not extracted yet. |
| `native` | `display`, `is_diagnostic` keep diagnostic text distinct from raw native paths. Never reconstruct action paths from display text. |

All worker constructors accept `Arc<dyn Fn() + Send + Sync>` wake signals. Core itself captures no egui Context, terminal renderer or GPU resource. Existing ownership/bounds stay unchanged: two enumeration workers and eight transport responses, per-target FIFO/coalescing and cancellation, one sorter worker, one finder thread with one pending/result slot. Drops request stop without joining unfinished read-only workers; an uninterruptible OS call may outlive the frontend. This slice adds no stronger cancellation deadline or filesystem snapshot guarantee.

## Validation commands and outcomes

Commands ran from the candidate checkout using the installed tools. Root Cargo commands used `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target`, exclusively reserved for this worker. Initial standalone core builds/tests used their own `crates/mado-core/target`; no GUI window or terminal frontend session was used.

| Exact command | Outcome |
| --- | --- |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo test --locked --offline -- --list` before extraction | Actual baseline 160 identities captured. |
| `cargo generate-lockfile --offline` and `cargo generate-lockfile --offline --manifest-path crates/mado-core/Cargo.toml` | Tracked root/core locks generated; existing external versions unchanged. |
| `cargo fmt -- --check` | Pass, including authorized fixture fix. |
| `cargo fmt --manifest-path crates/mado-core/Cargo.toml -- --check` | Pass. |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo check --locked --offline --all-targets` | Pass before and after fixture fix. |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo build --locked --offline` | Native GUI binary builds; no window launched. |
| `cargo tree --locked --offline --manifest-path crates/mado-core/Cargo.toml` | Only time/windows-sys and their normal transitives; no eframe, egui, wgpu, crossterm, ratatui or other frontend dependency. |
| `cargo build --locked --offline --manifest-path crates/mado-core/Cargo.toml` | Independent core build passes. |
| `cargo check --locked --offline --all-targets --manifest-path crates/mado-core/Cargo.toml` | Independent core library, tests and example compile. |
| `cargo test --locked --offline --manifest-path crates/mado-core/Cargo.toml` | 37 migrated unit checks and one generated end-to-end check pass, zero ignored. |
| `cargo run --locked --offline --manifest-path crates/mado-core/Cargo.toml --example browsing` | Actual generated-fixture demo passes. |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo test --locked --offline finder_tests::` | All 13 existing finder/UI/routing checks pass. |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo test --locked --offline create_tests::s019_affected_active_inactive_tabs_filter_and_alias_reconciliation -- --exact` | Isolated pre-fix check passes; not a passing full-suite claim. |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target cargo test --locked --offline create_tests::s019_fixture_parallel_names_are_unique -- --exact` | New 16-thread generated-fixture uniqueness check passes. |
| `CARGO_TARGET_DIR=C:/SANDBOX/REPOS/mado-explorer.git/target task test` after fixture fix | Complete existing task chain passes. Root uses default parallel execution: 123 pass, optional font 1 ignored. All accepted backend checks, workflow guard and disposable build-clean guard pass. No serial-only workaround. |
| `task --dir crates/mado-core --list` | Described default/run/test/check/build-only-clean tasks parse and list correctly. Core clean was not run against a real checkout. |
| `git diff --check` and `git diff --cached --check` | Pass. |

Standalone end-to-end proof runs real directory, sorter and finder workers against uniquely generated A/B folders, Unicode native paths and known file bytes. It holds actual A worker messages and delivers them after B is requested, proving stale refusal deterministically without claiming an OS stall. It checks committed B rows/history, folders-first metadata and unknown folder size, size sort retaining selections, fuzzy filtering pruning hidden selections without resurrection, stale finder revision refusal, injected owned catalogue ranking/emphasis, relative native probe and Back history. It never inventories a home folder or drive root.

Full existing task backend counts were PDF 9 pass/2 ignored, selection summary 7, conflict 22, raster 20, drawer 23, source identities 16 pass/1 ignored, PDF service 25 plus 2 compile-fail doctests pass/1 ignored, PDF controls 28, Keep both 15, confirmation 18, owned clipboard memory/codec 17, safe create 14 pass/1 ignored, native watcher 20, blocked-probe pure/retention checks 4, capped image service/helper 29. Ignored native PDF/privileged symlink probes remain pending. The known-failing legacy notify diagnostic is still excluded by the unchanged existing task, not relabeled passing. Workflow ownership checks and generated build-clean preservation also pass. The existing probe retention test intentionally retains its tiny owned fixtures, as before.

## Preserved first failure and diagnosis

The first default-parallel `task test` stopped before backend tasks. It produced 121 pass, 1 fail, 1 ignored in the then-123-test GUI inventory:

```text
---- create_tests::s019_affected_active_inactive_tabs_filter_and_alias_reconciliation stdout ----
thread 'create_tests::s019_affected_active_inactive_tabs_filter_and_alias_reconciliation' (13548) panicked at src\create_tests.rs:29:31:
called `Result::unwrap()` on an `Err` value: Os { code: 183, kind: AlreadyExists, message: "Cannot create a file when that file already exists." }
test result: FAILED. 121 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 1.48s
error: test failed, to rerun pass `--bin mado-explorer`
task: Failed to run task "test": exit status 101
```

The unchanged fixture name used PID and SystemTime nanoseconds, which are not a uniqueness guarantee under Windows clock granularity. Failure occurred at create-new fixture root construction, before any App/core path. The isolated exact check passed. Supervisor classified a pre-existing fixture-name collision and authorized only adding a process-local AtomicU64 suffix and a small parallel check in `src/create_tests.rs`. Create-new refusal, all existing test assertions and cleanup policy remain unchanged. No retry, overwrite, deletion of a collision, personal-data cleanup or serialized root test workaround was added. Subsequent default-parallel full task passes.

## Remaining work and acceptance limits

This supplies F15.1 implementation evidence and the browsing portion of F15.2, F15.5 and F15.6, preserving existing F01/F02/F05/F06 behavior. It does not complete all F15, does not implement F16 and does not self-approve criteria.

Next slices still need shared operation requests/immutable snapshots/models/runtime/service/queue/conflict/cancellation/report APIs; reusable workspace ownership; operation completion reconciliation; persistence/watcher/preview-service extraction and their GUI adapters. The TUI frontend must use these APIs, preserve context validation and retain the reviewed native safety backends. GUI layouts/focus/GPU textures remain outside core. Root Taskfile wiring for core/TUI belongs to supervisor integration and was not changed here.

Production capability gates remain false and unchanged. No mutations or previews newly enabled, no personal clipboard or native capability probe, no worker-owned app window. Supervisor-owned generated-window save/close/restart smoke, fresh review, optional fonts, DPI/Narrator/performance/packaging and other outstanding native/manual evidence remain pending. No criterion is fully verified merely from this shared library or its adapters.

## Test inventory migration

Actual Cargo inventories were collected before and after extraction and compared by name. Every baseline name survives unchanged. Baseline 160 equals GUI 123 original checks plus core 37 migrated. The final candidate adds the core end-to-end check and authorized GUI fixture uniqueness check, so GUI has 124 total and core 38 total. No assertion or test removed. The optional installed-font check remains ignored in GUI.

| Baseline test | Candidate owner |
| --- | --- |
| `compact_ui_tests::s016_footer_and_path_editors_block_shortcuts_including_the_first_click_frame` | GUI |
| `compact_ui_tests::s016_footer_clear_does_not_leave_a_stale_no_match_warning_in_the_same_frame` | GUI |
| `compact_ui_tests::s016_footer_click_and_f6_route_only_the_focused_pane` | GUI |
| `compact_ui_tests::s016_layout_and_visibility_menus_perform_real_actions_at_minimum_width` | GUI |
| `compact_ui_tests::s016_long_unicode_paths_keep_editor_and_breadcrumb_native_targets` | GUI |
| `compact_ui_tests::s016_long_warning_remains_truthful_and_short_body_scroll_reaches_footer` | GUI |
| `compact_ui_tests::s016_menu_keyboard_ownership_split_and_close_route_to_the_invoking_pane` | GUI |
| `compact_ui_tests::s016_narrow_short_and_split_geometry_has_reachable_actions_and_no_band_overlap` | GUI |
| `compact_ui_tests::s016_normal_header_table_footer_geometry_keeps_the_file_list_primary` | GUI |
| `create_tests::s019_actual_file_folder_collision_and_fresh_selection_without_watcher` | GUI |
| `create_tests::s019_actual_new_menu_and_focused_confirm_enter_reach_worker_once` | GUI |
| `create_tests::s019_affected_active_inactive_tabs_filter_and_alias_reconciliation` | GUI |
| `create_tests::s019_bounded_fifo_queued_cancel_and_active_acknowledgement` | GUI |
| `create_tests::s019_capture_capacity_includes_unacknowledged_retiring_pins_and_retry_keeps_draft` | GUI |
| `create_tests::s019_close_wait_counts_actual_success_after_cancel_and_preserves_settings_flow` | GUI |
| `create_tests::s019_drawer_layout_retry_does_not_leak_enter_but_global_finder_stays_usable` | GUI |
| `create_tests::s019_editor_capture_survives_navigation_and_origin_close_drawer_survives` | GUI |
| `create_tests::s019_editor_owner_rejects_inherited_enter_space_and_pointer_release` | GUI |
| `create_tests::s019_explicit_refresh_during_create_does_not_reauthorize_old_selection_marker` | GUI |
| `create_tests::s019_failed_only_explicit_retry_retains_identity_changed_and_missing_target_refuse` | GUI |
| `create_tests::s019_failed_retry_ui_retains_original_target_after_origin_tab_closes` | GUI |
| `create_tests::s019_history_bound_refusal_keeps_editor_and_acknowledgement_frees_metadata` | GUI |
| `create_tests::s019_invalid_names_and_valid_enter_once_escape_owns_submission_frame` | GUI |
| `create_tests::s019_native_close_request_routes_prompt_buttons_and_waits_before_settings` | GUI |
| `create_tests::s019_origin_pane_close_does_not_cancel_active_creation_or_lose_drawer` | GUI |
| `create_tests::s019_pending_b_failure_reconciles_a_and_selects_captured_entry` | GUI |
| `create_tests::s019_pending_b_success_defers_a_refresh_and_suppresses_stale_selection` | GUI |
| `create_tests::s019_preparation_failure_keeps_draft_target_retry_cancel_no_mutation` | GUI |
| `create_tests::s019_production_gate_false_and_explicit_ui_commands_explain_unavailable` | GUI |
| `create_tests::s019_r01_normalized_new_navigation_rejects_deferred_old_generation_selection` | GUI |
| `create_tests::s019_r01_progressive_fresh_rows_do_not_consume_creation_selection` | GUI |
| `create_tests::s019_worker_refuses_injected_drawer_payload_mismatch_before_any_mutation` | GUI |
| `directory::target_tests::s008_enumeration_pending_is_target_coalesced_fifo_and_lifecycle_bounded` | core |
| `directory::tests::background_listing_sort_and_failed_navigation_recover_together` | core |
| `directory::tests::background_loader_delivers_latest_request` | core |
| `directory::tests::cancellation_stops_scheduling_after_a_batch` | core |
| `directory::tests::empty_missing_and_not_folder_are_distinct` | core |
| `directory::tests::immediate_entries_have_metadata_and_unknown_folder_size` | core |
| `directory::tests::late_batches_and_errors_cannot_replace_new_location_even_on_refresh` | core |
| `directory::tests::modified_timestamp_handles_pre_epoch_and_out_of_range_values` | core |
| `directory::tests::s006_long_verbatim_unicode_fixture_enumerates_native_paths` | core |
| `directory::tests::s006_malformed_fixture_names_keep_native_actions_and_distinct_display` | core |
| `directory::tests::s006_owned_hidden_system_attributes_project_without_touching_identity` | core |
| `directory::tests::s006b_background_parent_up_absolute_dots_and_same_location_aliases` | core |
| `directory::tests::s006b_native_prefixes_unicode_and_verbatim_dots_are_not_reinterpreted` | core |
| `directory::tests::unavailable_windows_device_errors_are_not_missing_folders` | core |
| `finder_tests::s010_fuzzy_terms_ranking_marks_no_match_and_empty_categories` | GUI |
| `finder_tests::s010_modal_headless_all_key_actions_and_selection` | GUI |
| `finder_tests::s010_modal_headless_keyboard_text_clipboard_background_isolation_and_escape` | GUI |
| `finder_tests::s010_open_dismiss_freezes_committed_native_base_during_pending_navigation` | GUI |
| `finder_tests::s010_query_catalogue_probe_modal_revisions_reject_late_responses` | GUI |
| `finder_tests::s010_ready_probe_cannot_beat_same_frame_query_edit_or_dismissal` | GUI |
| `finder_tests::s010_review_expansion_keeps_input_focus_for_keyboard_narrowing` | GUI |
| `finder_tests::s010_review_finder_emits_focused_accessibility_input` | GUI |
| `finder_tests::s010_review_opening_pointer_focus_precedes_capture` | GUI |
| `finder_tests::s010_review_pasted_paths_are_never_silently_truncated` | GUI |
| `finder_tests::s010_session_bookmark_recent_management_only_successful_commits_are_visits` | GUI |
| `finder_tests::s010_stale_moved_closed_changed_active_targets_and_split_refusal_do_not_retarget` | GUI |
| `finder_tests::s010_worker_expansion_never_navigates_and_failed_path_recovers` | GUI |
| `font_fallback::tests::s006a_installed_windows_loader_covers_japanese_and_common_unicode` | GUI |
| `font_fallback::tests::s006a_missing_optional_fonts_keep_builtin_latin_and_report_limits` | GUI |
| `native::tests::malformed_native_units_are_distinct_diagnostics_not_replacement_text` | core |
| `native::tests::unicode_display_is_unmodified_including_literal_replacement_character` | core |
| `navigation::tests::failed_new_or_history_destination_keeps_location_listing_and_history` | core |
| `navigation::tests::filter_prunes_hidden_identities_only_after_complete_fresh_projection_and_clear_does_not_restore_them` | core |
| `navigation::tests::filter_revisions_reject_old_query_data_navigation_and_completion_projections` | core |
| `navigation::tests::s006_typed_paths_reject_drive_relative_and_preserve_absolute_prefixes` | core |
| `navigation::tests::s006_visibility_freshness_pruning_and_progressive_command_boundary` | core |
| `navigation::tests::s006b_normalized_completion_is_transactional_and_generation_guarded` | core |
| `navigation::tests::selection_reconciles_only_successful_current_refresh_and_survives_failed_navigation` | core |
| `navigation::tests::stale_navigation_and_sort_cannot_commit_and_relative_input_uses_committed_location` | core |
| `navigation::tests::successful_back_forward_and_new_branch_preserve_history` | core |
| `navigation::tests::vanished_current_location_has_missing_state_and_usable_parent_without_history_change` | core |
| `open::tests::open_native_preflight_rejects_missing_file_without_shell_execution` | GUI |
| `open::tests::open_shell_path_preserves_utf16_and_rejects_argument_namespace_edges` | GUI |
| `open::tests::open_worker_is_bounded_captures_path_and_reports_controlled_os_failure` | GUI |
| `pane_tests::s008_both_axes_nested_minimums_divider_clamps_and_collapse` | GUI |
| `pane_tests::s008_captured_launcher_survives_source_pane_close_without_retargeting` | GUI |
| `pane_tests::s008_committed_source_focus_tab_commands_close_and_final_home_are_isolated` | GUI |
| `pane_tests::s008_concurrent_load_projection_switch_close_and_stale_rejection` | GUI |
| `pane_tests::s008_first_click_in_inactive_pane_selects_its_row_without_a_second_click` | GUI |
| `pane_tests::s008_narrow_name_first_layout_and_shrink_keep_panes_accessible` | GUI |
| `pane_tests::s008_pointer_focus_precedes_shortcuts_f6_and_navigation_target_only_one_pane` | GUI |
| `selection::tests::click_toggle_range_and_select_all_use_native_displayed_paths` | core |
| `selection::tests::distinct_non_unicode_native_paths_do_not_collapse_selection` | core |
| `selection::tests::keyboard_boundaries_pages_and_shift_extension` | core |
| `selection::tests::reordered_projection_keeps_paths_and_changes_new_range_order` | core |
| `settings_workspace::tests::s011_atomic_success_replaces_old_bytes_and_all_precommit_failures_preserve_old_bytes` | GUI |
| `settings_workspace::tests::s011_bounded_serialization_preserves_old_file_without_temp_creation` | GUI |
| `settings_workspace::tests::s011_close_waits_for_latest_snapshot_or_explicit_failure_gate_prompt` | GUI |
| `settings_workspace::tests::s011_coalescing_worker_shutdown_saves_latest_and_join_reports_failure` | GUI |
| `settings_workspace::tests::s011_corrupt_oversized_version_invalid_and_unreadable_settings_preserved_and_disable_adapter` | GUI |
| `settings_workspace::tests::s011_gate_corruption_and_failed_close_controls_are_visible_at_minimum_window_size` | GUI |
| `settings_workspace::tests::s011_geometry_offscreen_recovery_and_normal_bounds_survive_maximize_minimize` | GUI |
| `settings_workspace::tests::s011_invalid_schema_tree_dimensions_indices_counts_and_encodings_are_rejected` | GUI |
| `settings_workspace::tests::s011_native_commit_failure_and_cleanup_failure_report_owned_temp_without_deleting_others` | GUI |
| `settings_workspace::tests::s011_native_path_roundtrip_including_malformed_and_long_prefix` | GUI |
| `settings_workspace::tests::s011_nested_restore_missing_location_up_and_explicit_folder_override` | GUI |
| `settings_workspace::tests::s011_settings_override_is_native_absolute_and_rejects_device_names_without_fallback` | GUI |
| `settings_workspace::tests::s011_settings_override_unc_namespace_guard_is_lexical_and_lossless` | GUI |
| `settings_workspace::tests::s011_worker_error_cannot_be_hidden_by_coalescing_or_consuming_report` | GUI |
| `sorting::target_tests::s008_projection_pending_and_results_are_bounded_per_target_not_global` | core |
| `sorting::tests::background_projection_coalesces_fast_query_and_data_edits` | core |
| `sorting::tests::background_sorter_delivers_latest_data_and_sort_without_filesystem_reads` | core |
| `sorting::tests::folders_first_in_every_column_and_direction_with_deterministic_ties` | core |
| `sorting::tests::fuzzy_filter_is_case_insensitive_name_only_and_keeps_column_order` | core |
| `sorting::tests::native_names_break_ties_even_when_display_strings_match` | core |
| `sorting::tests::unknown_sizes_and_times_stay_last_and_known_values_toggle` | core |
| `tab_drag_tests::s009_active_source_replacement_and_reorder_of_inactive_tab_do_not_retarget_neighbors` | GUI |
| `tab_drag_tests::s009_actual_cross_strip_transfer_keeps_full_context_and_source_active_id` | GUI |
| `tab_drag_tests::s009_actual_pointer_strip_reorder_preserves_active_identity_and_click_distinction` | GUI |
| `tab_drag_tests::s009_all_four_pointer_edge_splits_replenish_lone_own_source` | GUI |
| `tab_drag_tests::s009_closed_hover_destination_cannot_retarget_release_to_expanded_sibling` | GUI |
| `tab_drag_tests::s009_edge_transfer_from_other_last_source_collapses_only_source` | GUI |
| `tab_drag_tests::s009_escape_outside_release_closed_source_and_foreign_payload_do_not_move_tabs` | GUI |
| `tab_drag_tests::s009_minimum_offscreen_stale_closed_drops_are_atomic` | GUI |
| `tab_drag_tests::s009_real_worker_generation_routing_and_manual_scroll_restoration_after_move` | GUI |
| `tab_drag_tests::s009_source_last_collapse_resolves_destination_after_index_shift_and_preserves_live_target` | GUI |
| `tab_tests::s007_native_selection_and_late_closed_tab_responses_cannot_cross_contexts` | GUI |
| `tab_tests::s007_new_close_cycle_and_final_home_use_fresh_non_reused_ids` | GUI |
| `tab_tests::s007_plus_and_close_controls_create_tabs_and_replace_the_final_tab` | GUI |
| `tab_tests::s007_return_restores_independent_history_filter_sort_selection_and_editors` | GUI |
| `tab_tests::s007_same_location_ids_and_overflow_keep_active_tab_and_plus_reachable` | GUI |
| `tab_tests::s007_saved_manual_scroll_waits_for_complete_fresh_projection_not_active_row` | GUI |
| `tab_tests::s007_shared_background_refresh_reconciles_only_active_tab_and_final_fixture_home` | GUI |
| `tab_tests::s007_shortcuts_obey_focused_editors_submission_and_exact_modifiers` | GUI |
| `tests::filter_clear_control_restores_rows_without_navigation_at_narrow_viewport` | GUI |
| `tests::filter_status_counts_and_no_match_are_never_from_an_old_projection` | GUI |
| `tests::filter_table_commands_use_only_fresh_complete_visible_rows` | GUI |
| `tests::filter_toolbar_owns_editing_keys_and_submission_frame` | GUI |
| `tests::navigation_shortcuts_are_suspended_for_focused_path_editing` | GUI |
| `tests::open_activation_requires_a_fresh_visible_file_or_folder` | GUI |
| `tests::open_captured_native_file_survives_selection_navigation_and_failure_reporting` | GUI |
| `tests::open_double_click_routes_visible_file_and_folder_without_launching` | GUI |
| `tests::open_file_enter_obeys_editor_focus_repeat_and_pending_projection_guards` | GUI |
| `tests::row_scroll_offset_keeps_first_last_and_page_targets_inside_viewport` | GUI |
| `tests::s006_path_submission_reports_ambiguous_drive_relative_without_navigation` | GUI |
| `tests::s006_visibility_control_updates_freshness_without_navigation_at_narrow_viewport` | GUI |
| `tests::s006a_sort_direction_feedback_uses_builtin_ascii_glyphs` | GUI |
| `tests::selection_shortcuts_respect_editor_ownership_and_exact_modifiers` | GUI |
| `tests::table_cell_keeps_columns_aligned_for_short_and_truncated_names` | GUI |
| `tests::virtualized_table_keyboard_keeps_active_visible_and_pauses_pending_rows` | GUI |
| `tests::whole_row_metadata_and_gaps_receive_clicks_without_changing_virtual_pitch` | GUI |
| `watch_tests::s018_actual_app_exit_requests_nonblocking_stop_and_retains_cleanup_ownership` | GUI |
| `watch_tests::s018_actual_initial_missing_target_error_and_retry_are_not_mock_registration` | GUI |
| `watch_tests::s018_actual_tab_move_and_pane_close_reject_old_context_keep_loader_and_other_target` | GUI |
| `watch_tests::s018_capacity_refuses_all_contexts_and_recovers_without_arbitrary_subset` | GUI |
| `watch_tests::s018_headless_actual_retry_button_routes_native_retry_and_preserves_generation` | GUI |
| `watch_tests::s018_inactive_location_has_no_target_and_refreshes_on_activation` | GUI |
| `watch_tests::s018_injected_overflow_failure_busy_warning_survives_listing_refresh_until_retry_report` | GUI |
| `watch_tests::s018_native_create_rename_delete_reconcile_visible_rows_and_path_selection` | GUI |
| `watch_tests::s018_native_dirty_a_does_not_cancel_pending_b_success` | GUI |
| `watch_tests::s018_native_dirty_a_recovers_after_controlled_pending_b_failure` | GUI |
| `watch_tests::s018_native_root_delete_recreate_missing_retry_and_usable_up` | GUI |
| `watch_tests::s018_production_gate_truthful_footer_zero_service_targets_ticks_and_manual_f5` | GUI |
| `watch_tests::s018_r01_controlled_startup_failure_before_admission_retains_cause_and_stopped` | GUI |
| `watch_tests::s018_r01_disabled_and_intentionally_closing_adapters_reject_service_injection` | GUI |
| `watch_tests::s018_r01_stale_report_keeps_service_errors_but_rejects_target_payload` | GUI |
| `watch_tests::s018_refresh_filter_sort_do_not_reregister_or_repeat_cancel_inflight_load` | GUI |
| `watch_tests::s018_switch_close_stale_revision_generation_and_complete_native_path` | GUI |

New core integration check `c001_generated_listing_stale_navigation_filter_sort_selection_and_owned_finder`. New GUI fixture check `create_tests::s019_fixture_parallel_names_are_unique`.
