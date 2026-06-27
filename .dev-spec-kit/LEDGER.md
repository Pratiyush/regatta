# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**7/7 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:30.985Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:31.144Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:31.299Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:31.447Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:32.033Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:32.179Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:31.598Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:31.747Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | c78266e9* | 2026-06-27T17:51:31.889Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | c78266e9* | 2026-06-27T17:51:28.709Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | c78266e9* | 2026-06-27T17:51:29.380Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 17:51:31  ✅ check regatta_core::attention::tests::ranks_blocked_sessions @ c78266e9* → R-ATTN-01
- 2026-06-27 17:51:31  ✅ check regatta_core::attention::tests::working_sessions_score_zero @ c78266e9* → R-ATTN-01
- 2026-06-27 17:51:31  ✅ check regatta_core::attention::tests::dock_orders_by_urgency @ c78266e9* → R-ATTN-02
- 2026-06-27 17:51:31  ✅ check regatta_core::attention::tests::dock_excludes_working_and_seen @ c78266e9* → R-ATTN-02
- 2026-06-27 17:51:31  ✅ check regatta_core::backend::tests::plans_a_fresh_session @ c78266e9* → R-LAUNCH-01
- 2026-06-27 17:51:31  ✅ check regatta_core::backend::tests::plans_a_resume @ c78266e9* → R-LAUNCH-01
- 2026-06-27 17:51:31  ✅ check regatta_core::backend::tests::tags_env_for_reaping @ c78266e9* → R-LAUNCH-01
- 2026-06-27 17:51:32  ✅ check regatta_core::stream::tests::parses_known_lines @ c78266e9* → R-PARSE-01
- 2026-06-27 17:51:32  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ c78266e9* → R-PARSE-01
- 2026-06-27 17:51:32  🧾 graph build  [Pratiyush]
