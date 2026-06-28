# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**24/24 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:09.190Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:09.344Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:09.499Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:09.657Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:14.492Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:14.648Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:09.817Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:09.977Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:10.139Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:15.137Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:15.464Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:16.927Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:17.088Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:16.604Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:16.762Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:17.248Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:17.406Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:14.182Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:14.335Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:15.966Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:16.129Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:15.632Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:15.808Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:13.541Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:13.695Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:13.859Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:14.021Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:12.879Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:13.051Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:13.217Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:13.384Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:10.294Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:10.449Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:12.072Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:12.226Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:10.927Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:11.096Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:10.605Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:10.769Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:11.754Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:11.914Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 32f3a69c* | 2026-06-28T05:53:16.288Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:16.445Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:06.607Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | 32f3a69c* | 2026-06-28T05:53:07.151Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 05:53:15  ✅ check regatta_core::transcript::tests::parses_session_meta @ 32f3a69c* → R-TXMETA-01
- 2026-06-28 05:53:16  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 32f3a69c* → R-TXMETA-01
- 2026-06-28 05:53:16  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ 32f3a69c* → R-USAGE-01
- 2026-06-28 05:53:16  ✅ check regatta_store::tests::rolls_up_spend_by_model @ 32f3a69c* → R-USAGE-01
- 2026-06-28 05:53:16  ✅ check regatta_core::view::tests::maps_each_event_kind @ 32f3a69c* → R-VIEW-01
- 2026-06-28 05:53:16  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 32f3a69c* → R-VIEW-01
- 2026-06-28 05:53:16  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 32f3a69c* → R-WIRE-01
- 2026-06-28 05:53:17  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 32f3a69c* → R-WIRE-01
- 2026-06-28 05:53:17  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 32f3a69c* → R-WORKTREE-01
- 2026-06-28 05:53:17  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 32f3a69c* → R-WORKTREE-01
