# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**38/38 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:45.719Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:45.890Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:46.061Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:46.234Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:56.649Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:56.821Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:47.651Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:47.821Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:47.993Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:57.335Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:57.681Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:59.376Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:59.575Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:58.972Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:59.187Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:59.751Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:59.926Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:56.308Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:56.480Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:58.267Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:58.442Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:57.875Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:58.072Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:55.106Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:55.275Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:55.958Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:56.130Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:53.299Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:53.492Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:54.736Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:54.930Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:48.171Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:48.342Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:50.872Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:51.038Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:48.854Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:49.014Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:48.513Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:48.683Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:50.543Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:50.709Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:58.620Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:58.796Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:47.298Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:47.483Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:52.429Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:52.598Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:51.215Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:51.383Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:53.674Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:53.845Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:52.074Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:52.254Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:55.438Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:55.616Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:50.203Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:50.372Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:54.020Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:54.191Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:54.375Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:54.553Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:49.866Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | 0383c3b5* | 2026-06-28T08:17:50.034Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:45.382Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:45.547Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:55.784Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:49.524Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:49.697Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:49.185Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:49.357Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:20:28.261Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | 0383c3b5* | 2026-06-28T08:17:44.011Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 08:17:59  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ 0383c3b5* → R-WIRE-01
- 2026-06-28 08:17:59  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ 0383c3b5* → R-WORKTREE-01
- 2026-06-28 08:17:59  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ 0383c3b5* → R-WORKTREE-01
- 2026-06-28 08:18:00  🧾 gate  [Pratiyush]
- 2026-06-28 08:19:25  🧾 check run R-BACKEND-01 regatta_core::backend::tests::backend_dispatches_label_and_launch  [Pratiyush]
- 2026-06-28 08:19:26  ✅ check regatta_core::backend::tests::backend_dispatches_label_and_launch @ 0383c3b5* → R-BACKEND-01
- 2026-06-28 08:19:27  🧾 drift  [Pratiyush]
- 2026-06-28 08:19:27  🧾 gate  [Pratiyush]
- 2026-06-28 08:20:27  🧾 check run R-BACKEND-01 regatta_core::backend::tests::backend_dispatches_label_and_launch  [Pratiyush]
- 2026-06-28 08:20:28  ✅ check regatta_core::backend::tests::backend_dispatches_label_and_launch @ 0383c3b5* → R-BACKEND-01
