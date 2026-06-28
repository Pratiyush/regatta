# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**41/41 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:39.286Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:39.465Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:39.636Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:39.810Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:51.328Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:51.498Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:41.553Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:41.720Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:41.886Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:52.001Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:52.336Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:53.860Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:54.036Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:53.522Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:53.691Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:54.203Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:54.373Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:50.986Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:51.158Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:52.846Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:53.015Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:52.508Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:52.680Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:49.747Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:49.916Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:50.601Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:50.795Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:47.678Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:47.860Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:49.399Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:49.573Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:42.228Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:42.396Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:45.322Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:45.501Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:42.902Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:43.068Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:42.564Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:42.733Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:44.976Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:45.144Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:53.179Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:53.346Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:40.845Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:41.036Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:46.872Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:47.034Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:45.677Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:45.855Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:48.035Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:48.206Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:46.526Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:46.701Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:50.090Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:50.260Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:44.632Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:44.805Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:48.377Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:48.548Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:48.720Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:48.889Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:44.291Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:44.460Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:38.926Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:39.096Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:50.430Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:43.946Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:44.118Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:43.236Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:43.415Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:41.208Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:41.381Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:43.589Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:43.770Z |

- ✅ **R-PROOF-01** Codex is indistinguishable through the pipeline 🟢
  📋 Evidence — R-PROOF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude` | integration | ✅ green | cb964df9* | 2026-06-28T08:41:49.223Z |

- ✅ **R-BADGE-01** parse a backend label 🟢
  📋 Evidence — R-BADGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_from_label_roundtrips` | unit | ✅ green | cb964df9* | 2026-06-28T08:41:42.061Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 08:41:52  ✅ check regatta_core::transcript::tests::parses_session_meta @ cb964df9* → R-TXMETA-01
- 2026-06-28 08:41:53  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ cb964df9* → R-TXMETA-01
- 2026-06-28 08:41:53  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ cb964df9* → R-USAGE-01
- 2026-06-28 08:41:53  ✅ check regatta_store::tests::rolls_up_spend_by_model @ cb964df9* → R-USAGE-01
- 2026-06-28 08:41:53  ✅ check regatta_core::view::tests::maps_each_event_kind @ cb964df9* → R-VIEW-01
- 2026-06-28 08:41:53  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ cb964df9* → R-VIEW-01
- 2026-06-28 08:41:53  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ cb964df9* → R-WIRE-01
- 2026-06-28 08:41:54  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ cb964df9* → R-WIRE-01
- 2026-06-28 08:41:54  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ cb964df9* → R-WORKTREE-01
- 2026-06-28 08:41:54  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ cb964df9* → R-WORKTREE-01
