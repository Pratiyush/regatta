# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**14/14 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:21.124Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:21.283Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:21.441Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:21.600Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:22.533Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:22.688Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:21.752Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:21.906Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:22.064Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:23.738Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.066Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:25.152Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:25.307Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.838Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.992Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:25.461Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:25.615Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:22.221Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:22.377Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.536Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.686Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.223Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | d4df3bf0* | 2026-06-27T21:33:24.379Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:19.374Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | d4df3bf0* | 2026-06-27T21:33:19.895Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 21:33:24  ✅ check regatta_supervisor::tests::tail_transcript_restarts_after_truncation @ d4df3bf0* → R-TAIL-01
- 2026-06-27 21:33:24  ✅ check regatta_core::transcript::tests::parses_session_meta @ d4df3bf0* → R-TXMETA-01
- 2026-06-27 21:33:24  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ d4df3bf0* → R-TXMETA-01
- 2026-06-27 21:33:24  ✅ check regatta_core::view::tests::maps_each_event_kind @ d4df3bf0* → R-VIEW-01
- 2026-06-27 21:33:25  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ d4df3bf0* → R-VIEW-01
- 2026-06-27 21:33:25  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ d4df3bf0* → R-WIRE-01
- 2026-06-27 21:33:25  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ d4df3bf0* → R-WIRE-01
- 2026-06-27 21:33:25  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ d4df3bf0* → R-WORKTREE-01
- 2026-06-27 21:33:25  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ d4df3bf0* → R-WORKTREE-01
- 2026-06-27 21:33:25  🧾 graph build  [Pratiyush]
