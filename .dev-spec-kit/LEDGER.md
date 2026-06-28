# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**19/19 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:49.045Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:49.204Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:49.358Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:49.524Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:53.590Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:53.758Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:49.691Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:49.860Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:50.031Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:54.686Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:55.027Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:56.203Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:56.362Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:55.875Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:56.040Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:56.524Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:56.686Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:53.252Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:53.420Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:55.547Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:55.713Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:55.200Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:55.383Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:52.209Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:52.376Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:52.921Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:53.086Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:51.512Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:51.687Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:51.868Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 839082f2* | 2026-06-28T05:27:52.043Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:50.198Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:50.364Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:47.007Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 839082f2* | 2026-06-28T05:27:47.568Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 05:27:55  ✅ check regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset @ 839082f2* → R-TAIL-01
- 2026-06-28 05:27:55  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ 839082f2* → R-TAIL-01
- 2026-06-28 05:27:55  ✅ check regatta_core::transcript::tests::parses_session_meta @ 839082f2* → R-TXMETA-01
- 2026-06-28 05:27:55  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 839082f2* → R-TXMETA-01
- 2026-06-28 05:27:55  ✅ check regatta_core::view::tests::maps_each_event_kind @ 839082f2* → R-VIEW-01
- 2026-06-28 05:27:56  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 839082f2* → R-VIEW-01
- 2026-06-28 05:27:56  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 839082f2* → R-WIRE-01
- 2026-06-28 05:27:56  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 839082f2* → R-WIRE-01
- 2026-06-28 05:27:56  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 839082f2* → R-WORKTREE-01
- 2026-06-28 05:27:56  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 839082f2* → R-WORKTREE-01
