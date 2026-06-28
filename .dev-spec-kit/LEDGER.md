# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**23/23 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:08.464Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:08.646Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:08.822Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:08.998Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:13.809Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:13.983Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:09.176Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:09.355Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:09.536Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:14.959Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:15.294Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:16.536Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:16.717Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:16.181Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:16.358Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:16.893Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:17.067Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:13.456Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:13.632Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:15.831Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:16.005Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:15.471Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:15.652Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:12.747Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:12.922Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:13.102Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:13.279Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:12.020Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:12.205Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:12.387Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:12.570Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:09.710Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:09.888Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:11.128Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:11.300Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:10.414Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:10.596Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:10.065Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:10.239Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:10.775Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:10.952Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 9d2b0e60* | 2026-06-28T05:45:05.959Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 9d2b0e60* | 2026-06-28T05:45:07.056Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 05:45:15  ✅ check regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset @ 9d2b0e60* → R-TAIL-01
- 2026-06-28 05:45:15  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ 9d2b0e60* → R-TAIL-01
- 2026-06-28 05:45:15  ✅ check regatta_core::transcript::tests::parses_session_meta @ 9d2b0e60* → R-TXMETA-01
- 2026-06-28 05:45:16  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 9d2b0e60* → R-TXMETA-01
- 2026-06-28 05:45:16  ✅ check regatta_core::view::tests::maps_each_event_kind @ 9d2b0e60* → R-VIEW-01
- 2026-06-28 05:45:16  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 9d2b0e60* → R-VIEW-01
- 2026-06-28 05:45:16  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 9d2b0e60* → R-WIRE-01
- 2026-06-28 05:45:16  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 9d2b0e60* → R-WIRE-01
- 2026-06-28 05:45:16  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 9d2b0e60* → R-WORKTREE-01
- 2026-06-28 05:45:17  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 9d2b0e60* → R-WORKTREE-01
