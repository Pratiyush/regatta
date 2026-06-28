# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**44/44 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | c712609f* | 2026-06-28T10:22:43.776Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | c712609f* | 2026-06-28T10:22:43.939Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | c712609f* | 2026-06-28T10:22:44.108Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | c712609f* | 2026-06-28T10:22:44.269Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | c712609f* | 2026-06-28T10:22:55.465Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | c712609f* | 2026-06-28T10:22:55.629Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | c712609f* | 2026-06-28T10:22:45.376Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | c712609f* | 2026-06-28T10:22:45.538Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | c712609f* | 2026-06-28T10:22:45.702Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | c712609f* | 2026-06-28T10:22:56.117Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | c712609f* | 2026-06-28T10:22:56.454Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | c712609f* | 2026-06-28T10:22:57.961Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | c712609f* | 2026-06-28T10:22:58.139Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | c712609f* | 2026-06-28T10:22:57.631Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | c712609f* | 2026-06-28T10:22:57.795Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | c712609f* | 2026-06-28T10:22:58.303Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | c712609f* | 2026-06-28T10:22:58.462Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | c712609f* | 2026-06-28T10:22:55.147Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | c712609f* | 2026-06-28T10:22:55.305Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | c712609f* | 2026-06-28T10:22:56.977Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | c712609f* | 2026-06-28T10:22:57.143Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | c712609f* | 2026-06-28T10:22:56.643Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | c712609f* | 2026-06-28T10:22:56.810Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | c712609f* | 2026-06-28T10:22:53.647Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | c712609f* | 2026-06-28T10:22:53.817Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | c712609f* | 2026-06-28T10:22:54.814Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | c712609f* | 2026-06-28T10:22:54.985Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | c712609f* | 2026-06-28T10:22:51.559Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | c712609f* | 2026-06-28T10:22:51.738Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | c712609f* | 2026-06-28T10:22:52.944Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | c712609f* | 2026-06-28T10:22:53.143Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | c712609f* | 2026-06-28T10:22:46.031Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | c712609f* | 2026-06-28T10:22:46.196Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | c712609f* | 2026-06-28T10:22:49.015Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | c712609f* | 2026-06-28T10:22:49.183Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | c712609f* | 2026-06-28T10:22:46.678Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | c712609f* | 2026-06-28T10:22:46.840Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | c712609f* | 2026-06-28T10:22:46.359Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | c712609f* | 2026-06-28T10:22:46.518Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | c712609f* | 2026-06-28T10:22:48.686Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | c712609f* | 2026-06-28T10:22:48.852Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | c712609f* | 2026-06-28T10:22:57.305Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | c712609f* | 2026-06-28T10:22:57.468Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | c712609f* | 2026-06-28T10:22:44.718Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | c712609f* | 2026-06-28T10:22:44.886Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | c712609f* | 2026-06-28T10:22:50.577Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | c712609f* | 2026-06-28T10:22:50.741Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | c712609f* | 2026-06-28T10:22:49.347Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | c712609f* | 2026-06-28T10:22:49.567Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | c712609f* | 2026-06-28T10:22:51.911Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | c712609f* | 2026-06-28T10:22:52.082Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | c712609f* | 2026-06-28T10:22:50.249Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | c712609f* | 2026-06-28T10:22:50.414Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | c712609f* | 2026-06-28T10:22:53.987Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | c712609f* | 2026-06-28T10:22:54.154Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | c712609f* | 2026-06-28T10:22:48.354Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | c712609f* | 2026-06-28T10:22:48.518Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | c712609f* | 2026-06-28T10:22:52.254Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | c712609f* | 2026-06-28T10:22:52.418Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | c712609f* | 2026-06-28T10:22:52.581Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | c712609f* | 2026-06-28T10:22:52.747Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | c712609f* | 2026-06-28T10:22:48.013Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | c712609f* | 2026-06-28T10:22:48.189Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | c712609f* | 2026-06-28T10:22:43.442Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | c712609f* | 2026-06-28T10:22:43.609Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | c712609f* | 2026-06-28T10:22:54.650Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | c712609f* | 2026-06-28T10:22:47.679Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | c712609f* | 2026-06-28T10:22:47.845Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | c712609f* | 2026-06-28T10:22:47.014Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | c712609f* | 2026-06-28T10:22:47.179Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | c712609f* | 2026-06-28T10:22:45.050Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | c712609f* | 2026-06-28T10:22:45.214Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | c712609f* | 2026-06-28T10:22:47.346Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | c712609f* | 2026-06-28T10:22:47.511Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | c712609f* | 2026-06-28T10:22:41.049Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | c712609f* | 2026-06-28T10:22:45.865Z |

- ✅ **R-RUNTIME-01** fold the event stream into live state 🟢🟢
  📋 Evidence — R-RUNTIME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::folds_events_into_live_state` | unit | ✅ green | c712609f* | 2026-06-28T10:22:54.318Z |
  | `regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd` | unit | ✅ green | c712609f* | 2026-06-28T10:22:54.484Z |

- ✅ **R-REGISTRY-01** track live sessions by id 🟢🟢
  📋 Evidence — R-REGISTRY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::runtime::tests::registry_tracks_sessions_by_id` | unit | ✅ green | c712609f* | 2026-06-28T10:22:53.313Z |
  | `regatta_core::runtime::tests::snapshot_is_id_sorted` | unit | ✅ green | c712609f* | 2026-06-28T10:22:53.483Z |

- ✅ **R-LIVE-LAUNCH-01** pump a live session into the registry 🟢
  📋 Evidence — R-LIVE-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::pump_feeds_the_registry_live` | integration | ✅ green | c712609f* | 2026-06-28T10:22:39.658Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 10:22:57  ✅ check regatta_core::transcript::tests::parses_session_meta @ c712609f* → R-TXMETA-01
- 2026-06-28 10:22:57  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ c712609f* → R-TXMETA-01
- 2026-06-28 10:22:57  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ c712609f* → R-USAGE-01
- 2026-06-28 10:22:57  ✅ check regatta_store::tests::rolls_up_spend_by_model @ c712609f* → R-USAGE-01
- 2026-06-28 10:22:57  ✅ check regatta_core::view::tests::maps_each_event_kind @ c712609f* → R-VIEW-01
- 2026-06-28 10:22:57  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ c712609f* → R-VIEW-01
- 2026-06-28 10:22:57  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ c712609f* → R-WIRE-01
- 2026-06-28 10:22:58  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ c712609f* → R-WIRE-01
- 2026-06-28 10:22:58  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ c712609f* → R-WORKTREE-01
- 2026-06-28 10:22:58  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ c712609f* → R-WORKTREE-01
