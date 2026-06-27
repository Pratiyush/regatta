# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**8/8 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:44.354Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:44.500Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:44.643Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:44.794Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:45.392Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:45.536Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:44.949Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:45.096Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 8de15509* | 2026-06-27T17:58:45.247Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 8de15509* | 2026-06-27T17:58:46.024Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 8de15509* | 2026-06-27T17:58:46.341Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 8de15509* | 2026-06-27T17:58:42.661Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 8de15509* | 2026-06-27T17:58:43.177Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 17:58:44  ✅ check regatta_core::attention::tests::dock_orders_by_urgency @ 8de15509* → R-ATTN-02
- 2026-06-27 17:58:44  ✅ check regatta_core::attention::tests::dock_excludes_working_and_seen @ 8de15509* → R-ATTN-02
- 2026-06-27 17:58:44  ✅ check regatta_core::backend::tests::plans_a_fresh_session @ 8de15509* → R-LAUNCH-01
- 2026-06-27 17:58:45  ✅ check regatta_core::backend::tests::plans_a_resume @ 8de15509* → R-LAUNCH-01
- 2026-06-27 17:58:45  ✅ check regatta_core::backend::tests::tags_env_for_reaping @ 8de15509* → R-LAUNCH-01
- 2026-06-27 17:58:45  ✅ check regatta_core::stream::tests::parses_known_lines @ 8de15509* → R-PARSE-01
- 2026-06-27 17:58:45  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ 8de15509* → R-PARSE-01
- 2026-06-27 17:58:46  ✅ check regatta_supervisor::tests::shutdown_kills_the_whole_group @ 8de15509* → R-SUPERVISOR-01
- 2026-06-27 17:58:46  ✅ check regatta_supervisor::tests::shutdown_is_safe_when_already_exited @ 8de15509* → R-SUPERVISOR-01
- 2026-06-27 17:58:46  🧾 graph build  [Pratiyush]
