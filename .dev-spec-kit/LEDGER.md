# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**10/10 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:13.687Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:13.837Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:13.990Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:14.147Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:14.756Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:14.912Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:14.298Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:14.450Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:14.600Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | f0f5be61* | 2026-06-27T18:19:15.886Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | f0f5be61* | 2026-06-27T18:19:16.206Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | f0f5be61* | 2026-06-27T18:19:16.673Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | f0f5be61* | 2026-06-27T18:19:16.836Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:16.358Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:16.517Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:11.869Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | f0f5be61* | 2026-06-27T18:19:12.419Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 18:19:14  ✅ check regatta_core::backend::tests::tags_env_for_reaping @ f0f5be61* → R-LAUNCH-01
- 2026-06-27 18:19:14  ✅ check regatta_core::stream::tests::parses_known_lines @ f0f5be61* → R-PARSE-01
- 2026-06-27 18:19:14  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ f0f5be61* → R-PARSE-01
- 2026-06-27 18:19:15  ✅ check regatta_supervisor::tests::shutdown_kills_the_whole_group @ f0f5be61* → R-SUPERVISOR-01
- 2026-06-27 18:19:16  ✅ check regatta_supervisor::tests::shutdown_is_safe_when_already_exited @ f0f5be61* → R-SUPERVISOR-01
- 2026-06-27 18:19:16  ✅ check regatta_core::view::tests::maps_each_event_kind @ f0f5be61* → R-VIEW-01
- 2026-06-27 18:19:16  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ f0f5be61* → R-VIEW-01
- 2026-06-27 18:19:16  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ f0f5be61* → R-WIRE-01
- 2026-06-27 18:19:16  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ f0f5be61* → R-WIRE-01
- 2026-06-27 18:19:17  🧾 graph build  [Pratiyush]
