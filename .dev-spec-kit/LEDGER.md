# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**20/20 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:07.269Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:07.439Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:07.601Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:07.774Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:11.335Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:11.491Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:07.933Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:08.089Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:08.245Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:12.464Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:12.802Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:14.071Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:14.270Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:13.719Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:13.893Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:14.459Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:14.641Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:11.008Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:11.178Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:13.369Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:13.541Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:12.989Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:13.175Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:10.355Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:10.517Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:10.676Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:10.837Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:09.657Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:09.841Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:10.017Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 9b0cf29d* | 2026-06-28T05:33:10.187Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:08.404Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:08.560Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:08.716Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:08.883Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:05.405Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 9b0cf29d* | 2026-06-28T05:33:05.956Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 05:33:13  ✅ check regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset @ 9b0cf29d* → R-TAIL-01
- 2026-06-28 05:33:13  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ 9b0cf29d* → R-TAIL-01
- 2026-06-28 05:33:13  ✅ check regatta_core::transcript::tests::parses_session_meta @ 9b0cf29d* → R-TXMETA-01
- 2026-06-28 05:33:13  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 9b0cf29d* → R-TXMETA-01
- 2026-06-28 05:33:13  ✅ check regatta_core::view::tests::maps_each_event_kind @ 9b0cf29d* → R-VIEW-01
- 2026-06-28 05:33:13  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 9b0cf29d* → R-VIEW-01
- 2026-06-28 05:33:14  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 9b0cf29d* → R-WIRE-01
- 2026-06-28 05:33:14  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 9b0cf29d* → R-WIRE-01
- 2026-06-28 05:33:14  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 9b0cf29d* → R-WORKTREE-01
- 2026-06-28 05:33:14  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 9b0cf29d* → R-WORKTREE-01
