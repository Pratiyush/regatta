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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:21.701Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:21.870Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:22.041Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:22.212Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:35.332Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:35.510Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:23.946Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:24.115Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:24.282Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:36.030Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:36.366Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:37.897Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:38.066Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:37.559Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:37.725Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:38.237Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:38.407Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:34.998Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:35.163Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:36.868Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:37.036Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:36.536Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:36.703Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:33.480Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:33.655Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:34.664Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:34.830Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:30.774Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:30.948Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:32.795Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:32.968Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:24.624Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:24.794Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:27.661Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:27.835Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:25.299Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:25.469Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:24.962Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:25.132Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:27.319Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:27.484Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:37.203Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:37.390Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:23.263Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:23.433Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:29.580Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:29.753Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:28.333Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:28.497Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:31.114Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:31.281Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:29.235Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:29.411Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:33.821Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:33.988Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:26.985Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:27.152Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:31.796Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:31.963Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:32.130Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:32.297Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:26.651Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:26.819Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:20.338Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:20.508Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:34.497Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:26.315Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:26.484Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:25.641Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:25.819Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:23.604Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:23.773Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:25.985Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:26.151Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:32.622Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:24.448Z |

- ✅ **R-RUNTIME-01** fold the event stream into live state 🟢🟢
  📋 Evidence — R-RUNTIME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::folds_events_into_live_state` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:34.154Z |
  | `regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:34.323Z |

- ✅ **R-REGISTRY-01** track live sessions by id 🟢🟢
  📋 Evidence — R-REGISTRY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::registry_tracks_sessions_by_id` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:33.139Z |
  | `regatta_core::runtime::tests::snapshot_is_id_sorted` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:33.309Z |

- ✅ **R-LIVE-LAUNCH-01** pump a live session into the registry 🟢
  📋 Evidence — R-LIVE-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::pump_feeds_the_registry_live` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:31.454Z |

- ✅ **R-LIVEVIEW-01** summarize a live session 🟢
  📋 Evidence — R-LIVEVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::summary_is_a_compact_dock_line` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:31.628Z |

- ✅ **R-GOLIVE-01** a Claude session runs live through the pipeline 🟢
  📋 Evidence — R-GOLIVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::claude_session_runs_live_through_the_pipeline` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:30.091Z |

- ✅ **R-APPROVAL-PARSE-01** parse an approval request 🟢🟢
  📋 Evidence — R-APPROVAL-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_an_approval_request` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:20.676Z |
  | `regatta_core::stream::tests::rejects_non_approval_input` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:20.845Z |

- ✅ **R-APPROVAL-RESP-01** build the approve-tool response 🟢🟢
  📋 Evidence — R-APPROVAL-RESP-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_echoes_input_as_updated` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:21.021Z |
  | `regatta_core::approval::tests::deny_carries_the_reason` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:21.189Z |

- ✅ **R-FRAME-01** coalesce a burst, preserving priority 🟢🟢
  📋 Evidence — R-FRAME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::coalesces_adjacent_text_only` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:28.001Z |
  | `regatta_core::stream::tests::never_coalesces_an_approval` | unit | ✅ green | cbc5187f* | 2026-06-28T14:25:28.167Z |

- ✅ **R-APPROVAL-RT-01** the round-trip drives the agent 🟢🟢
  📋 Evidence — R-APPROVAL-RT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_round_trip_drives_the_agent` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:21.360Z |
  | `regatta_core::approval::tests::deny_round_trip_stops_the_agent` | integration | ✅ green | cbc5187f* | 2026-06-28T14:25:21.534Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 14:25:36  ✅ check regatta_core::transcript::tests::parses_session_meta @ cbc5187f* → R-TXMETA-01
- 2026-06-28 14:25:37  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ cbc5187f* → R-TXMETA-01
- 2026-06-28 14:25:37  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ cbc5187f* → R-USAGE-01
- 2026-06-28 14:25:37  ✅ check regatta_store::tests::rolls_up_spend_by_model @ cbc5187f* → R-USAGE-01
- 2026-06-28 14:25:37  ✅ check regatta_core::view::tests::maps_each_event_kind @ cbc5187f* → R-VIEW-01
- 2026-06-28 14:25:37  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ cbc5187f* → R-VIEW-01
- 2026-06-28 14:25:37  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ cbc5187f* → R-WIRE-01
- 2026-06-28 14:25:38  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ cbc5187f* → R-WIRE-01
- 2026-06-28 14:25:38  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ cbc5187f* → R-WORKTREE-01
- 2026-06-28 14:25:38  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ cbc5187f* → R-WORKTREE-01
