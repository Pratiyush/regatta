# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**28/28 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:28.224Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:28.391Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:28.549Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:28.720Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:35.131Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:35.289Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.022Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.177Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.344Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:35.786Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:36.114Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:37.575Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:37.740Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:37.239Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:37.411Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:37.898Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:38.060Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.814Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.970Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:36.597Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:36.758Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:36.281Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:36.440Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.175Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.331Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.490Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.652Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:33.188Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:33.359Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:33.846Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:34.013Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.503Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.664Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:31.799Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:31.959Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:31.151Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:31.320Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.825Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:30.987Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:31.479Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:31.638Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:36.918Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:37.083Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:29.696Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:29.864Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:32.434Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:32.595Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:32.116Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:32.278Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:33.516Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | 7ec706c8* | 2026-06-28T06:23:33.676Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:25.523Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | 7ec706c8* | 2026-06-28T06:23:26.065Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 06:23:36  ✅ check regatta_core::transcript::tests::parses_session_meta @ 7ec706c8* → R-TXMETA-01
- 2026-06-28 06:23:36  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 7ec706c8* → R-TXMETA-01
- 2026-06-28 06:23:36  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ 7ec706c8* → R-USAGE-01
- 2026-06-28 06:23:37  ✅ check regatta_store::tests::rolls_up_spend_by_model @ 7ec706c8* → R-USAGE-01
- 2026-06-28 06:23:37  ✅ check regatta_core::view::tests::maps_each_event_kind @ 7ec706c8* → R-VIEW-01
- 2026-06-28 06:23:37  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 7ec706c8* → R-VIEW-01
- 2026-06-28 06:23:37  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 7ec706c8* → R-WIRE-01
- 2026-06-28 06:23:37  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 7ec706c8* → R-WIRE-01
- 2026-06-28 06:23:37  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 7ec706c8* → R-WORKTREE-01
- 2026-06-28 06:23:38  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 7ec706c8* → R-WORKTREE-01
