# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**18/18 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:57.880Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:58.046Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:58.209Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:58.375Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:01.016Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:01.179Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:58.539Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:58.699Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:58.859Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:02.091Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:02.445Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:03.637Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:03.806Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:03.310Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:03.466Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:03.982Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:04.146Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:00.692Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:00.854Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:02.986Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:03.150Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:02.640Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:02.819Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:00.016Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 1672ab00* | 2026-06-27T22:06:00.194Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:00.359Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 1672ab00* | 2026-06-27T22:06:00.530Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 1672ab00* | 2026-06-27T22:05:59.347Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 1672ab00* | 2026-06-27T22:05:59.516Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 1672ab00* | 2026-06-27T22:05:59.684Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 1672ab00* | 2026-06-27T22:05:59.851Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:59.019Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 1672ab00* | 2026-06-27T22:05:59.182Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 22:06:02  ✅ check regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset @ 1672ab00* → R-TAIL-01
- 2026-06-27 22:06:02  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ 1672ab00* → R-TAIL-01
- 2026-06-27 22:06:03  ✅ check regatta_core::transcript::tests::parses_session_meta @ 1672ab00* → R-TXMETA-01
- 2026-06-27 22:06:03  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 1672ab00* → R-TXMETA-01
- 2026-06-27 22:06:03  ✅ check regatta_core::view::tests::maps_each_event_kind @ 1672ab00* → R-VIEW-01
- 2026-06-27 22:06:03  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 1672ab00* → R-VIEW-01
- 2026-06-27 22:06:03  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 1672ab00* → R-WIRE-01
- 2026-06-27 22:06:03  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 1672ab00* → R-WIRE-01
- 2026-06-27 22:06:04  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 1672ab00* → R-WORKTREE-01
- 2026-06-27 22:06:04  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 1672ab00* → R-WORKTREE-01
