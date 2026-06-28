# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**50/50 done (100%)**

- ✅ **R-SLUG-01** slugify a session name 🟢🟢
  📋 Evidence — R-SLUG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::slug::tests::slugifies_a_name` | unit | ✅ green | — | 2026-06-27T16:40:58.735Z |
  | `regatta_core::slug::tests::empty_slug_falls_back` | unit | ✅ green | — | 2026-06-27T16:40:59.157Z |

- ✅ **R-SLUG-02** fallback for empty slugs 🟢
  📋 Evidence — R-SLUG-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `helm_core::slug::tests::empty_slug_falls_back` | unit | ✅ green | — | 2026-06-27T16:27:26.316Z |

- ✅ **R-ATTN-01** rank sessions that need the human 🟢🟢
  📋 Evidence — R-ATTN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:47.225Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:47.401Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:47.569Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:47.739Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:01.090Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:01.255Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:49.540Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:49.708Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:49.871Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:01.771Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:02.108Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:03.636Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:03.804Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:03.293Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:03.468Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:03.974Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:04.145Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:00.755Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:00.920Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:02.620Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:02.787Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:02.278Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:02.448Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:59.228Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:59.401Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:00.419Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:00.590Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:56.496Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:56.674Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:58.539Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:58.718Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:50.212Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:50.384Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:53.269Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:53.442Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:50.887Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:51.063Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:50.554Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:50.721Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:52.931Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:53.097Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:02.958Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | fb464e26* | 2026-06-28T14:23:03.124Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:48.855Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:49.028Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:55.218Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:55.394Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:53.954Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:54.122Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:56.848Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:57.020Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:54.879Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:55.051Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:59.572Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:59.743Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:52.593Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:52.759Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:57.528Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:57.698Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:57.867Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:58.035Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:52.252Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:52.422Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:46.204Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:46.375Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:00.250Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:51.918Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:52.087Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:51.233Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:51.408Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:49.197Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:49.370Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:51.577Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:51.746Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:58.365Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:50.043Z |

- ✅ **R-RUNTIME-01** fold the event stream into live state 🟢🟢
  📋 Evidence — R-RUNTIME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::folds_events_into_live_state` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:59.912Z |
  | `regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd` | unit | ✅ green | fb464e26* | 2026-06-28T14:23:00.082Z |

- ✅ **R-REGISTRY-01** track live sessions by id 🟢🟢
  📋 Evidence — R-REGISTRY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::registry_tracks_sessions_by_id` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:58.891Z |
  | `regatta_core::runtime::tests::snapshot_is_id_sorted` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:59.062Z |

- ✅ **R-LIVE-LAUNCH-01** pump a live session into the registry 🟢
  📋 Evidence — R-LIVE-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::pump_feeds_the_registry_live` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:57.188Z |

- ✅ **R-LIVEVIEW-01** summarize a live session 🟢
  📋 Evidence — R-LIVEVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::summary_is_a_compact_dock_line` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:57.359Z |

- ✅ **R-GOLIVE-01** a Claude session runs live through the pipeline 🟢
  📋 Evidence — R-GOLIVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::claude_session_runs_live_through_the_pipeline` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:55.730Z |

- ✅ **R-APPROVAL-PARSE-01** parse an approval request 🟢🟢
  📋 Evidence — R-APPROVAL-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_an_approval_request` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:46.544Z |
  | `regatta_core::stream::tests::rejects_non_approval_input` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:46.711Z |

- ✅ **R-APPROVAL-RESP-01** build the approve-tool response 🟢🟢
  📋 Evidence — R-APPROVAL-RESP-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_echoes_input_as_updated` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:46.879Z |
  | `regatta_core::approval::tests::deny_carries_the_reason` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:47.048Z |

- ✅ **R-FRAME-01** coalesce a burst, preserving priority 🟢🟢
  📋 Evidence — R-FRAME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::coalesces_adjacent_text_only` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:53.608Z |
  | `regatta_core::stream::tests::never_coalesces_an_approval` | unit | ✅ green | fb464e26* | 2026-06-28T14:22:53.776Z |

- ✅ **R-APPROVAL-RT-01** the round-trip drives the agent 🟢🟢
  📋 Evidence — R-APPROVAL-RT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_round_trip_drives_the_agent` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:44.187Z |
  | `regatta_core::approval::tests::deny_round_trip_stops_the_agent` | integration | ✅ green | fb464e26* | 2026-06-28T14:22:44.775Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 14:23:02  ✅ check regatta_core::transcript::tests::parses_session_meta @ fb464e26* → R-TXMETA-01
- 2026-06-28 14:23:02  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ fb464e26* → R-TXMETA-01
- 2026-06-28 14:23:02  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ fb464e26* → R-USAGE-01
- 2026-06-28 14:23:03  ✅ check regatta_store::tests::rolls_up_spend_by_model @ fb464e26* → R-USAGE-01
- 2026-06-28 14:23:03  ✅ check regatta_core::view::tests::maps_each_event_kind @ fb464e26* → R-VIEW-01
- 2026-06-28 14:23:03  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ fb464e26* → R-VIEW-01
- 2026-06-28 14:23:03  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ fb464e26* → R-WIRE-01
- 2026-06-28 14:23:03  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ fb464e26* → R-WIRE-01
- 2026-06-28 14:23:03  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ fb464e26* → R-WORKTREE-01
- 2026-06-28 14:23:04  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ fb464e26* → R-WORKTREE-01
