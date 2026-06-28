# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**49/49 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:32.042Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:32.223Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:32.395Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:32.563Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:46.060Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:46.230Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:34.394Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:34.563Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:34.732Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:46.748Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:47.091Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:48.669Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:48.852Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:48.304Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:48.472Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:49.028Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:49.213Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:45.718Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:45.891Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:47.623Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:47.800Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:47.272Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:47.446Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:44.192Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:44.358Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:45.388Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:45.553Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:41.215Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:41.416Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:43.497Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:43.677Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:35.075Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:35.241Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:38.138Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:38.305Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:35.748Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:35.920Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:35.411Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:35.580Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:37.789Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:37.965Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:47.971Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:48.140Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:33.686Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:33.863Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:39.841Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:40.034Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:38.492Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:38.682Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:41.624Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:41.817Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:39.440Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:39.645Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:44.523Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:44.695Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:37.448Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:37.619Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:42.435Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:42.614Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:42.796Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:42.972Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:37.101Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:37.268Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:31.009Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:31.175Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:45.215Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:36.761Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:36.930Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:36.092Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:36.258Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:34.040Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:34.221Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:36.427Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:36.594Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:43.309Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:34.902Z |

- ✅ **R-RUNTIME-01** fold the event stream into live state 🟢🟢
  📋 Evidence — R-RUNTIME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::folds_events_into_live_state` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:44.871Z |
  | `regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:45.043Z |

- ✅ **R-REGISTRY-01** track live sessions by id 🟢🟢
  📋 Evidence — R-REGISTRY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::registry_tracks_sessions_by_id` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:43.852Z |
  | `regatta_core::runtime::tests::snapshot_is_id_sorted` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:44.023Z |

- ✅ **R-LIVE-LAUNCH-01** pump a live session into the registry 🟢
  📋 Evidence — R-LIVE-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::pump_feeds_the_registry_live` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:42.012Z |

- ✅ **R-LIVEVIEW-01** summarize a live session 🟢
  📋 Evidence — R-LIVEVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::summary_is_a_compact_dock_line` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:42.212Z |

- ✅ **R-GOLIVE-01** a Claude session runs live through the pipeline 🟢
  📋 Evidence — R-GOLIVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::claude_session_runs_live_through_the_pipeline` | integration | ✅ green | ffe159d0* | 2026-06-28T14:10:40.384Z |

- ✅ **R-APPROVAL-PARSE-01** parse an approval request 🟢🟢
  📋 Evidence — R-APPROVAL-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_an_approval_request` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:31.351Z |
  | `regatta_core::stream::tests::rejects_non_approval_input` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:31.521Z |

- ✅ **R-APPROVAL-RESP-01** build the approve-tool response 🟢🟢
  📋 Evidence — R-APPROVAL-RESP-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_echoes_input_as_updated` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:31.690Z |
  | `regatta_core::approval::tests::deny_carries_the_reason` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:31.873Z |

- ✅ **R-FRAME-01** coalesce a burst, preserving priority 🟢🟢
  📋 Evidence — R-FRAME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::coalesces_adjacent_text_only` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:29.024Z |
  | `regatta_core::stream::tests::never_coalesces_an_approval` | unit | ✅ green | ffe159d0* | 2026-06-28T14:10:29.606Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 14:10:47  ✅ check regatta_core::transcript::tests::parses_session_meta @ ffe159d0* → R-TXMETA-01
- 2026-06-28 14:10:47  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ ffe159d0* → R-TXMETA-01
- 2026-06-28 14:10:47  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ ffe159d0* → R-USAGE-01
- 2026-06-28 14:10:48  ✅ check regatta_store::tests::rolls_up_spend_by_model @ ffe159d0* → R-USAGE-01
- 2026-06-28 14:10:48  ✅ check regatta_core::view::tests::maps_each_event_kind @ ffe159d0* → R-VIEW-01
- 2026-06-28 14:10:48  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ ffe159d0* → R-VIEW-01
- 2026-06-28 14:10:48  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ ffe159d0* → R-WIRE-01
- 2026-06-28 14:10:48  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ ffe159d0* → R-WIRE-01
- 2026-06-28 14:10:49  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ ffe159d0* → R-WORKTREE-01
- 2026-06-28 14:10:49  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ ffe159d0* → R-WORKTREE-01
