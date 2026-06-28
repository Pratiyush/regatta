# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**54/54 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:02.355Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:02.527Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:02.694Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:02.860Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:16.926Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:17.095Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:03.973Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:04.141Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:04.308Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:17.610Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:17.949Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:19.459Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:19.628Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:19.122Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:19.289Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:19.799Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:19.970Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:16.593Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:16.755Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:18.451Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:18.621Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:18.121Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:18.286Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:14.756Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:14.925Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:16.270Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:16.431Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:10.917Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:11.091Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:14.087Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:14.257Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:04.643Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:04.812Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:07.641Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:07.807Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:05.319Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:05.488Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:04.983Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:05.152Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:07.311Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:07.477Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:18.787Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:18.956Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:03.304Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:03.473Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:09.598Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:09.764Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:08.345Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:08.510Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:11.259Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:11.429Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:09.247Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:09.425Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:15.096Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:15.262Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:06.980Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:07.148Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:11.932Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:12.100Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:12.266Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:12.432Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:06.650Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:06.811Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:01.009Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:01.175Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:16.106Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:06.324Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:06.488Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:05.655Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:05.822Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:03.637Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:03.805Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:05.993Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:06.159Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:13.914Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:04.476Z |

- ✅ **R-RUNTIME-01** fold the event stream into live state 🟢🟢
  📋 Evidence — R-RUNTIME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::folds_events_into_live_state` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:15.763Z |
  | `regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:15.936Z |

- ✅ **R-REGISTRY-01** track live sessions by id 🟢🟢
  📋 Evidence — R-REGISTRY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::registry_tracks_sessions_by_id` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:14.427Z |
  | `regatta_core::runtime::tests::snapshot_is_id_sorted` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:14.590Z |

- ✅ **R-LIVE-LAUNCH-01** pump a live session into the registry 🟢
  📋 Evidence — R-LIVE-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::pump_feeds_the_registry_live` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:11.595Z |

- ✅ **R-LIVEVIEW-01** summarize a live session 🟢
  📋 Evidence — R-LIVEVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::summary_is_a_compact_dock_line` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:11.762Z |

- ✅ **R-GOLIVE-01** a Claude session runs live through the pipeline 🟢
  📋 Evidence — R-GOLIVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::claude_session_runs_live_through_the_pipeline` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:10.103Z |

- ✅ **R-APPROVAL-PARSE-01** parse an approval request 🟢🟢
  📋 Evidence — R-APPROVAL-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_an_approval_request` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:01.344Z |
  | `regatta_core::stream::tests::rejects_non_approval_input` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:01.512Z |

- ✅ **R-APPROVAL-RESP-01** build the approve-tool response 🟢🟢
  📋 Evidence — R-APPROVAL-RESP-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_echoes_input_as_updated` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:01.678Z |
  | `regatta_core::approval::tests::deny_carries_the_reason` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:01.850Z |

- ✅ **R-FRAME-01** coalesce a burst, preserving priority 🟢🟢
  📋 Evidence — R-FRAME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::coalesces_adjacent_text_only` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:08.006Z |
  | `regatta_core::stream::tests::never_coalesces_an_approval` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:08.178Z |

- ✅ **R-APPROVAL-RT-01** the round-trip drives the agent 🟢🟢
  📋 Evidence — R-APPROVAL-RT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::approval::tests::allow_round_trip_drives_the_agent` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:02.019Z |
  | `regatta_core::approval::tests::deny_round_trip_stops_the_agent` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:51:02.189Z |

- ✅ **R-RPC-01** parse requests and build responses 🟢🟢
  📋 Evidence — R-RPC-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::parses_a_jsonrpc_request` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:15.430Z |
  | `regatta_core::mcp::tests::rejects_non_requests_and_builds_responses` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:15.596Z |

- ✅ **R-MCP-DISPATCH-01** route MCP requests 🟢🟢
  📋 Evidence — R-MCP-DISPATCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::dispatch_routes_mcp_methods` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:12.769Z |
  | `regatta_core::mcp::tests::dispatch_handles_errors_and_notifications` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:12.936Z |

- ✅ **R-MCP-APPROVE-01** wrap the decision as a tools/call result 🟢
  📋 Evidence — R-MCP-APPROVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::mcp::tests::approve_result_wraps_the_decision` | unit | ✅ green | 4707eb7a* | 2026-06-28T14:51:12.601Z |

- ✅ **R-MCP-SERVE-01** run the MCP server loop 🟢
  📋 Evidence — R-MCP-SERVE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::serve_mcp_handles_a_session` | integration | ✅ green | 4707eb7a* | 2026-06-28T14:52:31.232Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 14:51:19  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 4707eb7a* → R-VIEW-01
- 2026-06-28 14:51:19  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 4707eb7a* → R-WIRE-01
- 2026-06-28 14:51:19  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 4707eb7a* → R-WIRE-01
- 2026-06-28 14:51:19  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 4707eb7a* → R-WORKTREE-01
- 2026-06-28 14:51:19  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 4707eb7a* → R-WORKTREE-01
- 2026-06-28 14:51:20  🧾 gate  [Pratiyush]
- 2026-06-28 14:52:30  🧾 check run R-MCP-SERVE-01 regatta_supervisor::tests::serve_mcp_handles_a_session  [Pratiyush]
- 2026-06-28 14:52:31  ✅ check regatta_supervisor::tests::serve_mcp_handles_a_session @ 4707eb7a* → R-MCP-SERVE-01
- 2026-06-28 14:52:31  🧾 task done R-MCP-SERVE-01  [Pratiyush]
- 2026-06-28 14:52:31  🏁 task R-MCP-SERVE-01 → done
