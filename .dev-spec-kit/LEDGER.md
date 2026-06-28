# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**55/55 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:07.215Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:07.386Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:07.557Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:07.729Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:22.570Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:22.747Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:09.539Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:09.714Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:09.888Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:23.274Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:23.615Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:25.149Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:25.322Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:24.811Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:24.981Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:25.497Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:25.667Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:22.229Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:22.396Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:24.129Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:24.300Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:23.790Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:23.959Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:20.331Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:20.502Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:21.892Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:22.061Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:16.790Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:16.967Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:19.641Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:19.819Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:10.234Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:10.406Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:13.304Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:13.477Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:10.925Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:11.096Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:10.580Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:10.753Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:12.965Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:13.134Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:24.469Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:24.636Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:08.856Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:09.027Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:15.489Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:15.657Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:13.994Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:14.168Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:17.157Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:17.334Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:15.141Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:15.314Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:20.677Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:20.865Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:12.625Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:12.794Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:17.855Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:18.032Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:18.205Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:18.376Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:12.288Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:12.456Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:05.848Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:06.014Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:21.724Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:11.952Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:12.120Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:11.268Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:11.442Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:09.198Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:09.367Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:11.613Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:11.784Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:19.443Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:10.059Z |

- ✅ **R-RUNTIME-01** fold the event stream into live state 🟢🟢
  📋 Evidence — R-RUNTIME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::folds_events_into_live_state` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:21.388Z |
  | `regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:21.556Z |

- ✅ **R-REGISTRY-01** track live sessions by id 🟢🟢
  📋 Evidence — R-REGISTRY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::registry_tracks_sessions_by_id` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:19.990Z |
  | `regatta_core::runtime::tests::snapshot_is_id_sorted` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:20.158Z |

- ✅ **R-LIVE-LAUNCH-01** pump a live session into the registry 🟢
  📋 Evidence — R-LIVE-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::pump_feeds_the_registry_live` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:17.508Z |

- ✅ **R-LIVEVIEW-01** summarize a live session 🟢
  📋 Evidence — R-LIVEVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::summary_is_a_compact_dock_line` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:17.683Z |

- ✅ **R-GOLIVE-01** a Claude session runs live through the pipeline 🟢
  📋 Evidence — R-GOLIVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::claude_session_runs_live_through_the_pipeline` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:16.004Z |

- ✅ **R-APPROVAL-PARSE-01** parse an approval request 🟢🟢
  📋 Evidence — R-APPROVAL-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_an_approval_request` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:06.187Z |
  | `regatta_core::stream::tests::rejects_non_approval_input` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:06.356Z |

- ✅ **R-APPROVAL-RESP-01** build the approve-tool response 🟢🟢
  📋 Evidence — R-APPROVAL-RESP-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_echoes_input_as_updated` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:06.527Z |
  | `regatta_core::approval::tests::deny_carries_the_reason` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:06.697Z |

- ✅ **R-FRAME-01** coalesce a burst, preserving priority 🟢🟢
  📋 Evidence — R-FRAME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::coalesces_adjacent_text_only` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:13.647Z |
  | `regatta_core::stream::tests::never_coalesces_an_approval` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:13.819Z |

- ✅ **R-APPROVAL-RT-01** the round-trip drives the agent 🟢🟢
  📋 Evidence — R-APPROVAL-RT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_round_trip_drives_the_agent` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:06.872Z |
  | `regatta_core::approval::tests::deny_round_trip_stops_the_agent` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:07.044Z |

- ✅ **R-RPC-01** parse requests and build responses 🟢🟢
  📋 Evidence — R-RPC-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::parses_a_jsonrpc_request` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:21.047Z |
  | `regatta_core::mcp::tests::rejects_non_requests_and_builds_responses` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:21.218Z |

- ✅ **R-MCP-DISPATCH-01** route MCP requests 🟢🟢
  📋 Evidence — R-MCP-DISPATCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::dispatch_routes_mcp_methods` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:18.724Z |
  | `regatta_core::mcp::tests::dispatch_handles_errors_and_notifications` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:18.896Z |

- ✅ **R-MCP-APPROVE-01** wrap the decision as a tools/call result 🟢
  📋 Evidence — R-MCP-APPROVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::approve_result_wraps_the_decision` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:18.550Z |

- ✅ **R-MCP-SERVE-01** run the MCP server loop 🟢
  📋 Evidence — R-MCP-SERVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::serve_mcp_handles_a_session` | integration | ✅ green | a6fe9fae* | 2026-06-28T14:57:19.090Z |

- ✅ **R-APPROVE-WIRE-01** build the approve MCP server config 🟢
  📋 Evidence — R-APPROVE-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::builds_the_approve_server_config` | unit | ✅ green | a6fe9fae* | 2026-06-28T14:57:04.421Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 14:57:24  ✅ check regatta_core::transcript::tests::parses_session_meta @ a6fe9fae* → R-TXMETA-01
- 2026-06-28 14:57:24  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ a6fe9fae* → R-TXMETA-01
- 2026-06-28 14:57:24  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ a6fe9fae* → R-USAGE-01
- 2026-06-28 14:57:24  ✅ check regatta_store::tests::rolls_up_spend_by_model @ a6fe9fae* → R-USAGE-01
- 2026-06-28 14:57:24  ✅ check regatta_core::view::tests::maps_each_event_kind @ a6fe9fae* → R-VIEW-01
- 2026-06-28 14:57:25  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ a6fe9fae* → R-VIEW-01
- 2026-06-28 14:57:25  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ a6fe9fae* → R-WIRE-01
- 2026-06-28 14:57:25  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ a6fe9fae* → R-WIRE-01
- 2026-06-28 14:57:25  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ a6fe9fae* → R-WORKTREE-01
- 2026-06-28 14:57:25  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ a6fe9fae* → R-WORKTREE-01
