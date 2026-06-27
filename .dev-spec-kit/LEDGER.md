# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**16/16 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:22.725Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:22.878Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.033Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.204Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:25.191Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:25.345Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.361Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.525Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.676Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:25.845Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:26.170Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:27.288Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:27.449Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:26.968Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:27.128Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:27.604Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:27.761Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:24.859Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:25.023Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:26.655Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:26.811Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:26.330Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:26.491Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.843Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 3d490593* | 2026-06-27T21:41:23.999Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:24.528Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:24.696Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:20.889Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 3d490593* | 2026-06-27T21:41:21.457Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 21:41:26  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ 3d490593* → R-TAIL-01
- 2026-06-27 21:41:26  ✅ check regatta_core::transcript::tests::parses_session_meta @ 3d490593* → R-TXMETA-01
- 2026-06-27 21:41:26  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 3d490593* → R-TXMETA-01
- 2026-06-27 21:41:26  ✅ check regatta_core::view::tests::maps_each_event_kind @ 3d490593* → R-VIEW-01
- 2026-06-27 21:41:27  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 3d490593* → R-VIEW-01
- 2026-06-27 21:41:27  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 3d490593* → R-WIRE-01
- 2026-06-27 21:41:27  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 3d490593* → R-WIRE-01
- 2026-06-27 21:41:27  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 3d490593* → R-WORKTREE-01
- 2026-06-27 21:41:27  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 3d490593* → R-WORKTREE-01
- 2026-06-27 21:41:28  🧾 graph build  [Pratiyush]
