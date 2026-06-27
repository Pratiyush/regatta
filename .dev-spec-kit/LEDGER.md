# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**6/6 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:03.933Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:04.088Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:04.238Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:04.384Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:04.529Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:04.676Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:01.549Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:02.102Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 5d83261d* | 2026-06-27T17:41:02.631Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 17:41:03  🧾 task done R-LAUNCH-01  [Pratiyush]
- 2026-06-27 17:41:03  🏁 task R-LAUNCH-01 → done
- 2026-06-27 17:41:03  🧾 drift  [Pratiyush]
- 2026-06-27 17:41:03  ✅ check regatta_core::attention::tests::ranks_blocked_sessions @ 5d83261d* → R-ATTN-01
- 2026-06-27 17:41:04  ✅ check regatta_core::attention::tests::working_sessions_score_zero @ 5d83261d* → R-ATTN-01
- 2026-06-27 17:41:04  ✅ check regatta_core::attention::tests::dock_orders_by_urgency @ 5d83261d* → R-ATTN-02
- 2026-06-27 17:41:04  ✅ check regatta_core::attention::tests::dock_excludes_working_and_seen @ 5d83261d* → R-ATTN-02
- 2026-06-27 17:41:04  ✅ check regatta_core::stream::tests::parses_known_lines @ 5d83261d* → R-PARSE-01
- 2026-06-27 17:41:04  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ 5d83261d* → R-PARSE-01
- 2026-06-27 17:41:05  🧾 graph build  [Pratiyush]
