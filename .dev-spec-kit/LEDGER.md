# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**26/26 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | ac628887* | 2026-06-28T06:16:59.485Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | ac628887* | 2026-06-28T06:16:59.644Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | ac628887* | 2026-06-28T06:16:59.805Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | ac628887* | 2026-06-28T06:16:59.968Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | ac628887* | 2026-06-28T06:17:05.964Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | ac628887* | 2026-06-28T06:17:06.124Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | ac628887* | 2026-06-28T06:17:01.294Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | ac628887* | 2026-06-28T06:17:01.455Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | ac628887* | 2026-06-28T06:17:01.618Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | ac628887* | 2026-06-28T06:17:06.617Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | ac628887* | 2026-06-28T06:17:06.948Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | ac628887* | 2026-06-28T06:17:08.421Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | ac628887* | 2026-06-28T06:17:08.587Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | ac628887* | 2026-06-28T06:17:08.098Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | ac628887* | 2026-06-28T06:17:08.254Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | ac628887* | 2026-06-28T06:17:08.768Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | ac628887* | 2026-06-28T06:17:08.952Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | ac628887* | 2026-06-28T06:17:05.638Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | ac628887* | 2026-06-28T06:17:05.798Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | ac628887* | 2026-06-28T06:17:07.452Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | ac628887* | 2026-06-28T06:17:07.614Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | ac628887* | 2026-06-28T06:17:07.113Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | ac628887* | 2026-06-28T06:17:07.285Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | ac628887* | 2026-06-28T06:17:04.937Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | ac628887* | 2026-06-28T06:17:05.119Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | ac628887* | 2026-06-28T06:17:05.304Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | ac628887* | 2026-06-28T06:17:05.477Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | ac628887* | 2026-06-28T06:17:04.232Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | ac628887* | 2026-06-28T06:17:04.402Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | ac628887* | 2026-06-28T06:17:04.572Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | ac628887* | 2026-06-28T06:17:04.750Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | ac628887* | 2026-06-28T06:17:01.782Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | ac628887* | 2026-06-28T06:17:01.944Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | ac628887* | 2026-06-28T06:17:03.074Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | ac628887* | 2026-06-28T06:17:03.237Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | ac628887* | 2026-06-28T06:17:02.429Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | ac628887* | 2026-06-28T06:17:02.589Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | ac628887* | 2026-06-28T06:17:02.109Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | ac628887* | 2026-06-28T06:17:02.272Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | ac628887* | 2026-06-28T06:17:02.749Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | ac628887* | 2026-06-28T06:17:02.908Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | ac628887* | 2026-06-28T06:17:07.774Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | ac628887* | 2026-06-28T06:17:07.934Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | ac628887* | 2026-06-28T06:17:00.963Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | ac628887* | 2026-06-28T06:17:01.137Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | ac628887* | 2026-06-28T06:17:03.407Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | ac628887* | 2026-06-28T06:17:03.570Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | ac628887* | 2026-06-28T06:16:57.630Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | ac628887* | 2026-06-28T06:16:58.198Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 06:17:07  ✅ check regatta_core::transcript::tests::parses_session_meta @ ac628887* → R-TXMETA-01
- 2026-06-28 06:17:07  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ ac628887* → R-TXMETA-01
- 2026-06-28 06:17:07  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ ac628887* → R-USAGE-01
- 2026-06-28 06:17:07  ✅ check regatta_store::tests::rolls_up_spend_by_model @ ac628887* → R-USAGE-01
- 2026-06-28 06:17:08  ✅ check regatta_core::view::tests::maps_each_event_kind @ ac628887* → R-VIEW-01
- 2026-06-28 06:17:08  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ ac628887* → R-VIEW-01
- 2026-06-28 06:17:08  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ ac628887* → R-WIRE-01
- 2026-06-28 06:17:08  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ ac628887* → R-WIRE-01
- 2026-06-28 06:17:08  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ ac628887* → R-WORKTREE-01
- 2026-06-28 06:17:08  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ ac628887* → R-WORKTREE-01
