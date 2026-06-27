# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**17/17 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:47.196Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:47.349Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:47.502Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:47.658Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:49.508Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:49.662Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:47.808Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:47.958Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:48.110Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:50.160Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:50.476Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:51.558Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:51.717Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:51.249Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:51.401Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:51.867Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:52.019Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:49.206Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:49.357Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:50.944Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:51.095Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:50.636Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:50.792Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:48.592Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | bd27085a* | 2026-06-27T21:47:48.745Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:48.899Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:49.050Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:48.271Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:48.433Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:45.415Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | bd27085a* | 2026-06-27T21:47:45.947Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 21:47:50  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ bd27085a* → R-TAIL-01
- 2026-06-27 21:47:50  ✅ check regatta_core::transcript::tests::parses_session_meta @ bd27085a* → R-TXMETA-01
- 2026-06-27 21:47:51  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ bd27085a* → R-TXMETA-01
- 2026-06-27 21:47:51  ✅ check regatta_core::view::tests::maps_each_event_kind @ bd27085a* → R-VIEW-01
- 2026-06-27 21:47:51  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ bd27085a* → R-VIEW-01
- 2026-06-27 21:47:51  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ bd27085a* → R-WIRE-01
- 2026-06-27 21:47:51  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ bd27085a* → R-WIRE-01
- 2026-06-27 21:47:51  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ bd27085a* → R-WORKTREE-01
- 2026-06-27 21:47:52  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ bd27085a* → R-WORKTREE-01
- 2026-06-27 21:47:52  🧾 graph build  [Pratiyush]
