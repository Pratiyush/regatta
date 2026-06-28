# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**32/32 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:36.225Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:36.386Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:36.551Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:36.714Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:45.178Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:45.343Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:38.077Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:38.238Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:38.400Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:45.851Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:46.187Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.662Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.830Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.331Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.495Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.995Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:48.157Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:44.847Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:45.012Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:46.678Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:46.839Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:46.351Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:46.516Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:43.867Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:44.034Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:44.518Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:44.681Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:42.523Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:42.695Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:43.517Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:43.697Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:38.563Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:38.726Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:40.204Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:40.366Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:39.225Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:39.386Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:38.894Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:39.061Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:39.875Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:40.040Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.004Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:47.170Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:37.741Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:37.910Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:41.677Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:41.838Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:40.524Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:40.685Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:42.855Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:43.018Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:41.348Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | 3ec447c6* | 2026-06-28T06:57:41.515Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:44.196Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:44.358Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:39.549Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:39.712Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:43.187Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:43.350Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:34.347Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | 3ec447c6* | 2026-06-28T06:57:34.908Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 06:57:46  ✅ check regatta_core::transcript::tests::parses_session_meta @ 3ec447c6* → R-TXMETA-01
- 2026-06-28 06:57:46  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 3ec447c6* → R-TXMETA-01
- 2026-06-28 06:57:47  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ 3ec447c6* → R-USAGE-01
- 2026-06-28 06:57:47  ✅ check regatta_store::tests::rolls_up_spend_by_model @ 3ec447c6* → R-USAGE-01
- 2026-06-28 06:57:47  ✅ check regatta_core::view::tests::maps_each_event_kind @ 3ec447c6* → R-VIEW-01
- 2026-06-28 06:57:47  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 3ec447c6* → R-VIEW-01
- 2026-06-28 06:57:47  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 3ec447c6* → R-WIRE-01
- 2026-06-28 06:57:47  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 3ec447c6* → R-WIRE-01
- 2026-06-28 06:57:48  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 3ec447c6* → R-WORKTREE-01
- 2026-06-28 06:57:48  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 3ec447c6* → R-WORKTREE-01
