# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**21/21 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:47.059Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:47.226Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:47.390Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:47.555Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:51.445Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:51.604Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:47.716Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:47.885Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:48.042Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:52.549Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:52.879Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:54.021Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:54.192Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:53.699Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:53.863Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:54.360Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:54.529Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:51.126Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:51.285Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:53.368Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:53.538Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:53.040Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:53.210Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:50.465Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:50.634Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:50.800Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:50.963Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:49.789Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:49.960Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:50.128Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 4b5d092e* | 2026-06-28T05:36:50.301Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:48.205Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:48.368Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:48.858Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:49.017Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:48.529Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:48.689Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:45.045Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 4b5d092e* | 2026-06-28T05:36:45.587Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 05:36:53  ✅ check regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset @ 4b5d092e* → R-TAIL-01
- 2026-06-28 05:36:53  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ 4b5d092e* → R-TAIL-01
- 2026-06-28 05:36:53  ✅ check regatta_core::transcript::tests::parses_session_meta @ 4b5d092e* → R-TXMETA-01
- 2026-06-28 05:36:53  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 4b5d092e* → R-TXMETA-01
- 2026-06-28 05:36:53  ✅ check regatta_core::view::tests::maps_each_event_kind @ 4b5d092e* → R-VIEW-01
- 2026-06-28 05:36:53  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 4b5d092e* → R-VIEW-01
- 2026-06-28 05:36:54  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 4b5d092e* → R-WIRE-01
- 2026-06-28 05:36:54  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 4b5d092e* → R-WIRE-01
- 2026-06-28 05:36:54  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 4b5d092e* → R-WORKTREE-01
- 2026-06-28 05:36:54  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 4b5d092e* → R-WORKTREE-01
