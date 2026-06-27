# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**13/13 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:48.607Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:48.763Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:48.919Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:49.081Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:50.012Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:50.162Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:49.239Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:49.395Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:49.550Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:50.646Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:50.963Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:51.732Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:51.897Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:51.421Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:51.574Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:52.049Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:52.201Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:49.703Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:49.856Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:51.117Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 64ff8dd7* | 2026-06-27T21:28:51.268Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:46.191Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 64ff8dd7* | 2026-06-27T21:28:46.722Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 21:28:50  ✅ check regatta_supervisor::tests::shutdown_is_safe_when_already_exited @ 64ff8dd7* → R-SUPERVISOR-01
- 2026-06-27 21:28:51  ✅ check regatta_core::transcript::tests::parses_session_meta @ 64ff8dd7* → R-TXMETA-01
- 2026-06-27 21:28:51  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ 64ff8dd7* → R-TXMETA-01
- 2026-06-27 21:28:51  ✅ check regatta_core::view::tests::maps_each_event_kind @ 64ff8dd7* → R-VIEW-01
- 2026-06-27 21:28:51  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ 64ff8dd7* → R-VIEW-01
- 2026-06-27 21:28:51  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 64ff8dd7* → R-WIRE-01
- 2026-06-27 21:28:51  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 64ff8dd7* → R-WIRE-01
- 2026-06-27 21:28:52  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 64ff8dd7* → R-WORKTREE-01
- 2026-06-27 21:28:52  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 64ff8dd7* → R-WORKTREE-01
- 2026-06-27 21:28:52  🧾 graph build  [Pratiyush]
