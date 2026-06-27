# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**9/9 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:52.477Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:52.636Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:52.814Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:52.969Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:53.586Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:53.737Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:53.128Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:53.282Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:53.437Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 08678f82* | 2026-06-27T18:11:54.813Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 08678f82* | 2026-06-27T18:11:55.129Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 08678f82* | 2026-06-27T18:11:55.298Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 08678f82* | 2026-06-27T18:11:55.454Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:50.607Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 08678f82* | 2026-06-27T18:11:51.162Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 18:11:53  ✅ check regatta_core::backend::tests::plans_a_fresh_session @ 08678f82* → R-LAUNCH-01
- 2026-06-27 18:11:53  ✅ check regatta_core::backend::tests::plans_a_resume @ 08678f82* → R-LAUNCH-01
- 2026-06-27 18:11:53  ✅ check regatta_core::backend::tests::tags_env_for_reaping @ 08678f82* → R-LAUNCH-01
- 2026-06-27 18:11:53  ✅ check regatta_core::stream::tests::parses_known_lines @ 08678f82* → R-PARSE-01
- 2026-06-27 18:11:53  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ 08678f82* → R-PARSE-01
- 2026-06-27 18:11:54  ✅ check regatta_supervisor::tests::shutdown_kills_the_whole_group @ 08678f82* → R-SUPERVISOR-01
- 2026-06-27 18:11:55  ✅ check regatta_supervisor::tests::shutdown_is_safe_when_already_exited @ 08678f82* → R-SUPERVISOR-01
- 2026-06-27 18:11:55  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ 08678f82* → R-WIRE-01
- 2026-06-27 18:11:55  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 08678f82* → R-WIRE-01
- 2026-06-27 18:11:55  🧾 graph build  [Pratiyush]
