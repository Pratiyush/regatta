# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**12/12 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | d031d860* | 2026-06-27T21:24:27.416Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | d031d860* | 2026-06-27T21:24:27.575Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | d031d860* | 2026-06-27T21:24:27.731Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | d031d860* | 2026-06-27T21:24:27.907Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | d031d860* | 2026-06-27T21:24:28.881Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | d031d860* | 2026-06-27T21:24:29.045Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | d031d860* | 2026-06-27T21:24:28.080Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | d031d860* | 2026-06-27T21:24:28.242Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | d031d860* | 2026-06-27T21:24:28.403Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | d031d860* | 2026-06-27T21:24:30.069Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | d031d860* | 2026-06-27T21:24:30.397Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | d031d860* | 2026-06-27T21:24:30.871Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | d031d860* | 2026-06-27T21:24:31.047Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | d031d860* | 2026-06-27T21:24:30.557Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | d031d860* | 2026-06-27T21:24:30.710Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | d031d860* | 2026-06-27T21:24:31.210Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | d031d860* | 2026-06-27T21:24:31.383Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | d031d860* | 2026-06-27T21:24:28.569Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | d031d860* | 2026-06-27T21:24:28.725Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | d031d860* | 2026-06-27T21:24:25.503Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | d031d860* | 2026-06-27T21:24:26.035Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-27 21:24:29  ✅ check regatta_core::stream::tests::ignores_malformed_and_unknown @ d031d860* → R-PARSE-01
- 2026-06-27 21:24:30  ✅ check regatta_supervisor::tests::shutdown_kills_the_whole_group @ d031d860* → R-SUPERVISOR-01
- 2026-06-27 21:24:30  ✅ check regatta_supervisor::tests::shutdown_is_safe_when_already_exited @ d031d860* → R-SUPERVISOR-01
- 2026-06-27 21:24:30  ✅ check regatta_core::view::tests::maps_each_event_kind @ d031d860* → R-VIEW-01
- 2026-06-27 21:24:30  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ d031d860* → R-VIEW-01
- 2026-06-27 21:24:30  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ d031d860* → R-WIRE-01
- 2026-06-27 21:24:31  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ d031d860* → R-WIRE-01
- 2026-06-27 21:24:31  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ d031d860* → R-WORKTREE-01
- 2026-06-27 21:24:31  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ d031d860* → R-WORKTREE-01
- 2026-06-27 21:24:31  🧾 graph build  [Pratiyush]
