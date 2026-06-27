# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**5/5 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | d8cb1302* | 2026-06-27T17:35:56.859Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | d8cb1302* | 2026-06-27T17:35:56.997Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | d8cb1302* | 2026-06-27T17:35:57.139Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | d8cb1302* | 2026-06-27T17:35:57.279Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | d8cb1302* | 2026-06-27T17:35:55.169Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | d8cb1302* | 2026-06-27T17:37:28.522Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 17:35:57  ✅ check regatta_core::attention::tests::dock_excludes_working_and_seen @ d8cb1302* → R-ATTN-02
- 2026-06-27 17:35:57  🧾 graph build  [Pratiyush]
- 2026-06-27 17:35:58  🧾 gate  [Pratiyush]
- 2026-06-27 17:36:36  🧾 drift  [Pratiyush]
- 2026-06-27 17:36:36  🧾 graph build  [Pratiyush]
- 2026-06-27 17:36:36  🧾 gate  [Pratiyush]
- 2026-06-27 17:37:27  🧾 spec tasks  [Pratiyush]
- 2026-06-27 17:37:27  🧾 check run R-PARSE-01 regatta_core::stream::tests::ignores_malformed_and_unknown  [Pratiyush]
- 2026-06-27 17:37:28  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ d8cb1302* → R-PARSE-01
- 2026-06-27 17:37:28  🧾 graph build  [Pratiyush]
