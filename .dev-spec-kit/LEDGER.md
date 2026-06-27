# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**4/4 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | eed81c76* | 2026-06-27T17:12:29.955Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | eed81c76* | 2026-06-27T17:12:30.110Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | eed81c76* | 2026-06-27T17:12:05.927Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | eed81c76* | 2026-06-27T17:12:06.454Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 17:12:07  🧾 graph build  [Pratiyush]
- 2026-06-27 17:12:12  🧾 gate  [Pratiyush]
- 2026-06-27 17:12:29  🧾 drift  [Pratiyush]
- 2026-06-27 17:12:29  ✅ check regatta_core::attention::tests::ranks_blocked_sessions @ eed81c76* → R-ATTN-01
- 2026-06-27 17:12:30  ✅ check regatta_core::attention::tests::working_sessions_score_zero @ eed81c76* → R-ATTN-01
- 2026-06-27 17:12:30  🧾 graph build  [Pratiyush]
- 2026-06-27 17:12:31  🧾 gate  [Pratiyush]
- 2026-06-27 17:15:13  🧾 graph build  [Pratiyush]
- 2026-06-27 17:15:14  🧾 gate  [Pratiyush]
- 2026-06-27 17:15:33  🧾 graph build  [Pratiyush]
