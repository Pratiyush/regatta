# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**11/11 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:38.478Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:38.637Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:38.797Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:38.952Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:39.581Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:39.733Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:39.113Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:39.269Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:39.428Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | ece4d8cc* | 2026-06-27T18:22:40.706Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.032Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.509Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.676Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.191Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.346Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.829Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | ece4d8cc* | 2026-06-27T18:22:41.980Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | ece4d8cc* | 2026-06-27T18:22:36.224Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | ece4d8cc* | 2026-06-27T18:22:36.746Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 18:22:39  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ ece4d8cc* → R-PARSE-01
- 2026-06-27 18:22:40  ✅ check regatta_supervisor::tests::shutdown_kills_the_whole_group @ ece4d8cc* → R-SUPERVISOR-01
- 2026-06-27 18:22:41  ✅ check regatta_supervisor::tests::shutdown_is_safe_when_already_exited @ ece4d8cc* → R-SUPERVISOR-01
- 2026-06-27 18:22:41  ✅ check regatta_core::view::tests::maps_each_event_kind @ ece4d8cc* → R-VIEW-01
- 2026-06-27 18:22:41  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ ece4d8cc* → R-VIEW-01
- 2026-06-27 18:22:41  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ ece4d8cc* → R-WIRE-01
- 2026-06-27 18:22:41  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ ece4d8cc* → R-WIRE-01
- 2026-06-27 18:22:41  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ ece4d8cc* → R-WORKTREE-01
- 2026-06-27 18:22:41  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ ece4d8cc* → R-WORKTREE-01
- 2026-06-27 18:22:42  🧾 graph build  [Pratiyush]
