# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**37/37 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:36.238Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:36.403Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:36.569Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:36.735Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:46.542Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:46.718Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:38.087Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:38.253Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:38.418Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:47.233Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:47.561Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:49.079Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:49.252Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:48.735Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:48.909Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:49.417Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:49.593Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:46.210Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:46.374Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:48.066Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:48.233Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:47.732Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:47.902Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:45.037Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:45.206Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:45.877Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:46.045Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:43.328Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:43.501Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:44.695Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:44.872Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:38.583Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:38.747Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:40.958Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:41.121Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:39.251Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:39.418Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:38.919Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:39.085Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:40.622Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:40.795Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:48.396Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:48.566Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:37.741Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:37.920Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:42.455Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:42.620Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:41.287Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:41.452Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:43.679Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:43.850Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:42.116Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:42.289Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:45.366Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:45.534Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:40.268Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:40.455Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:44.012Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:44.178Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:44.347Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:44.511Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:39.929Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | 4f65646d* | 2026-06-28T08:14:40.096Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:35.899Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:36.069Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:45.704Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:39.585Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:39.752Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:33.982Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | 4f65646d* | 2026-06-28T08:14:34.547Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 08:14:48  ✅ check regatta_core::transcript::tests::parses_session_meta @ 4f65646d* → R-TXMETA-01
- 2026-06-28 08:14:48  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 4f65646d* → R-TXMETA-01
- 2026-06-28 08:14:48  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ 4f65646d* → R-USAGE-01
- 2026-06-28 08:14:48  ✅ check regatta_store::tests::rolls_up_spend_by_model @ 4f65646d* → R-USAGE-01
- 2026-06-28 08:14:48  ✅ check regatta_core::view::tests::maps_each_event_kind @ 4f65646d* → R-VIEW-01
- 2026-06-28 08:14:48  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 4f65646d* → R-VIEW-01
- 2026-06-28 08:14:49  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 4f65646d* → R-WIRE-01
- 2026-06-28 08:14:49  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 4f65646d* → R-WIRE-01
- 2026-06-28 08:14:49  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 4f65646d* → R-WORKTREE-01
- 2026-06-28 08:14:49  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 4f65646d* → R-WORKTREE-01
