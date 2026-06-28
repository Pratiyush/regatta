# LEDGER — generated from the journal; do not edit

> Legend: ✅ done · 🔨 in progress · 🚧 blocked · ⬜ pending — proofs: 🟢 green · 🔴 red · 🟣 stale · ⚪ unproven

## Progress board

**39/39 done (100%)**

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
  | `regatta_core::attention::tests::ranks_blocked_sessions` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:00.602Z |
  | `regatta_core::attention::tests::working_sessions_score_zero` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:00.781Z |

- ✅ **R-ATTN-02** order the Attention Dock 🟢🟢
  📋 Evidence — R-ATTN-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::attention::tests::dock_orders_by_urgency` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:00.957Z |
  | `regatta_core::attention::tests::dock_excludes_working_and_seen` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:01.133Z |

- ✅ **R-PARSE-01** normalize Claude stream-json lines 🟢🟢
  📋 Evidence — R-PARSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_known_lines` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:11.820Z |
  | `regatta_core::stream::tests::ignores_malformed_and_unknown` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:11.990Z |

- ✅ **R-LAUNCH-01** plan a Claude Code session 🟢🟢🟢
  📋 Evidence — R-LAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_fresh_session` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:02.830Z |
  | `regatta_core::backend::tests::plans_a_resume` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:03.003Z |
  | `regatta_core::backend::tests::tags_env_for_reaping` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:03.169Z |

- ✅ **R-SUPERVISOR-01** terminate the whole process group 🟢🟢
  📋 Evidence — R-SUPERVISOR-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::shutdown_kills_the_whole_group` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:12.497Z |
  | `regatta_supervisor::tests::shutdown_is_safe_when_already_exited` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:12.825Z |

- ✅ **R-WIRE-01** collect normalized events from a session's stdout 🟢🟢
  📋 Evidence — R-WIRE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::collects_parsed_events_from_stdout` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:14.329Z |
  | `regatta_supervisor::tests::skips_unparseable_lines` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:14.499Z |

- ✅ **R-VIEW-01** render an event as a display line 🟢🟢
  📋 Evidence — R-VIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::view::tests::maps_each_event_kind` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:13.990Z |
  | `regatta_core::view::tests::preserves_empty_assistant_text` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:14.159Z |

- ✅ **R-WORKTREE-01** plan an isolated worktree 🟢🟢
  📋 Evidence — R-WORKTREE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::worktree::tests::plans_a_unique_worktree` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:14.664Z |
  | `regatta_core::worktree::tests::handles_a_short_uuid` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:14.830Z |

- ✅ **R-STORE-01** persist and list sessions 🟢🟢
  📋 Evidence — R-STORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_and_lists_sessions` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:11.487Z |
  | `regatta_store::tests::upsert_updates_existing_session` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:11.657Z |

- ✅ **R-TXMETA-01** parse Claude transcript session metadata 🟢🟢
  📋 Evidence — R-TXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_session_meta` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:13.326Z |
  | `regatta_core::transcript::tests::rejects_malformed_or_idless` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:13.488Z |

- ✅ **R-TAIL-01** tail a transcript from an offset 🟢🟢
  📋 Evidence — R-TAIL-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:12.995Z |
  | `regatta_supervisor::tests::tail_transcript_restarts_after_truncation` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:13.162Z |

- ✅ **R-RESUME-01** resume an existing session 🟢🟢
  📋 Evidence — R-RESUME-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_resume_and_command` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:10.310Z |
  | `regatta_core::backend::tests::resume_command_handles_empty_id` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:10.484Z |

- ✅ **R-STORE-02** persist and search transcripts 🟢🟢
  📋 Evidence — R-STORE-02
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::transcript_index_persists_and_searches` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:11.151Z |
  | `regatta_store::tests::upsert_transcript_updates_in_place` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:11.316Z |

- ✅ **R-INDEX-01** index a transcript directory 🟢🟢
  📋 Evidence — R-INDEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::indexes_a_transcript_dir` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:08.588Z |
  | `regatta_indexer::tests::missing_dir_yields_nothing` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:08.762Z |

- ✅ **R-REATTACH-01** reattach persisted sessions 🟢🟢
  📋 Evidence — R-REATTACH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_indexer::tests::reattaches_and_advances_offset` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:09.949Z |
  | `regatta_indexer::tests::skips_a_missing_transcript` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:10.136Z |

- ✅ **R-BOARD-01** group sessions by recency 🟢🟢
  📋 Evidence — R-BOARD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::board::tests::buckets_by_recency` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:03.334Z |
  | `regatta_core::board::tests::future_or_zero_diff_is_today` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:03.506Z |

- ✅ **R-COST-01** price token usage and pick the effective cost 🟢🟢
  📋 Evidence — R-COST-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::prices_tokens_by_model` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:06.081Z |
  | `regatta_core::cost::tests::effective_cost_prefers_authoritative_usd` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:06.243Z |

- ✅ **R-BURN-01** burn rate and time-to-ceiling 🟢🟢
  📋 Evidence — R-BURN-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::computes_burn_rate` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:04.022Z |
  | `regatta_core::cost::tests::predicts_time_to_ceiling` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:04.190Z |

- ✅ **R-BUDGET-01** classify spend and decide auto-pause 🟢🟢
  📋 Evidence — R-BUDGET-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::budget::tests::classifies_spend_against_budget` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:03.681Z |
  | `regatta_core::budget::tests::pauses_only_when_exceeded_and_block` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:03.846Z |

- ✅ **R-COSTSTORE-01** record and roll up cost 🟢🟢
  📋 Evidence — R-COSTSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::records_and_rolls_up_cost` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:05.733Z |
  | `regatta_store::tests::empty_cost_rollups_are_zero` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:05.904Z |

- ✅ **R-USAGE-01** budget percent and spend by model 🟢🟢
  📋 Evidence — R-USAGE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::cost::tests::budget_pct_clamps_0_to_100` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:13.654Z |
  | `regatta_store::tests::rolls_up_spend_by_model` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:13.819Z |

- ✅ **R-AUTOPAUSE-01** auto-pause a runaway session 🟢🟢
  📋 Evidence — R-AUTOPAUSE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:02.140Z |
  | `regatta_supervisor::tests::does_not_autopause_when_action_is_not_block` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:02.315Z |

- ✅ **R-GITSTATUS-01** parse git status --porcelain 🟢🟢
  📋 Evidence — R-GITSTATUS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_status_porcelain` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:07.665Z |
  | `regatta_core::git::tests::skips_blank_and_pathless_lines` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:07.834Z |

- ✅ **R-DIFF-01** parse git diff --numstat 🟢🟢
  📋 Evidence — R-DIFF-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::parses_git_numstat` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:06.415Z |
  | `regatta_core::git::tests::numstat_skips_malformed_lines` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:06.585Z |

- ✅ **R-LAYOUT-01** grid layout and pane assignment 🟢🟢
  📋 Evidence — R-LAYOUT-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::layout::tests::layout_from_count` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:08.928Z |
  | `regatta_core::layout::tests::fills_panes_with_sessions` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:09.089Z |

- ✅ **R-GITREAD-01** read a repo's status and diffstat 🟢🟢
  📋 Evidence — R-GITREAD-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_git::tests::reads_status_and_diffstat_from_a_repo` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:07.325Z |
  | `regatta_git::tests::non_repo_yields_empty_status` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:07.500Z |

- ✅ **R-REVIEW-01** summarize a diff for the Review Inbox 🟢🟢
  📋 Evidence — R-REVIEW-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::git::tests::sums_diff_stats` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:10.651Z |
  | `regatta_core::git::tests::empty_diff_is_zero` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:10.814Z |

- ✅ **R-CONFIG-01** resolve layered config 🟢🟢
  📋 Evidence — R-CONFIG-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::later_layers_override_earlier` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:05.391Z |
  | `regatta_core::config::tests::empty_layers_resolve_empty` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:05.561Z |

- ✅ **R-MASK-01** mask secret config values 🟢🟢
  📋 Evidence — R-MASK-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::flags_secret_keys` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:09.250Z |
  | `regatta_core::config::tests::masks_secret_values` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:09.423Z |

- ✅ **R-MATERIALIZE-01** materialize config into session env 🟢🟢
  📋 Evidence — R-MATERIALIZE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::materializes_env_with_local_model_path` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:09.597Z |
  | `regatta_core::config::tests::no_local_model_means_no_base_url` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:09.771Z |

- ✅ **R-CONFIGSTORE-01** persist config layers by scope 🟢🟢
  📋 Evidence — R-CONFIGSTORE-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_store::tests::persists_config_layers_by_scope` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:05.049Z |
  | `regatta_store::tests::set_config_updates_in_place` | integration | ✅ green | c7a535b5* | 2026-06-28T08:27:05.222Z |

- ✅ **R-APPLY-01** merge config env into a launch plan 🟢🟢
  📋 Evidence — R-APPLY-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::with_env_merges_overriding_existing` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:00.262Z |
  | `regatta_core::backend::tests::with_env_empty_is_unchanged` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:00.433Z |

- ✅ **R-SETTINGS-01** effective config display 🟢
  📋 Evidence — R-SETTINGS-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::config::tests::effective_masked_resolves_and_masks` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:10.981Z |

- ✅ **R-CODEX-01** parse Codex events into the normalized shape 🟢🟢
  📋 Evidence — R-CODEX-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::stream::tests::parses_codex_events_into_normalized` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:04.704Z |
  | `regatta_core::stream::tests::codex_skips_meta_and_unknown` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:04.874Z |

- ✅ **R-CODEXLAUNCH-01** plan a Codex launch 🟢🟢
  📋 Evidence — R-CODEXLAUNCH-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::plans_a_codex_session` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:04.365Z |
  | `regatta_core::backend::tests::plans_a_codex_resume` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:04.535Z |

- ✅ **R-BACKEND-01** dispatch by backend 🟢🟢
  📋 Evidence — R-BACKEND-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::backend::tests::backend_dispatches_label_and_launch` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:02.487Z |
  | `regatta_core::backend::tests::backend_parses_its_own_format` | unit | ✅ green | c7a535b5* | 2026-06-28T08:27:02.658Z |

- ✅ **R-CODEXMETA-01** parse a Codex session_meta line 🟢🟢
  📋 Evidence — R-CODEXMETA-01
  | Check | Kind | State | Proof | Proven at |
  |---|---|---|---|---|
  | `regatta_core::transcript::tests::parses_codex_session_meta` | unit | ✅ green | c7a535b5* | 2026-06-28T08:26:58.282Z |
  | `regatta_core::transcript::tests::rejects_non_codex_meta` | unit | ✅ green | c7a535b5* | 2026-06-28T08:26:58.877Z |


## Approvals & governance

- _none recorded yet_

## Recent activity

- 2026-06-28 08:27:13  ✅ check regatta_core::transcript::tests::parses_session_meta @ c7a535b5* → R-TXMETA-01
- 2026-06-28 08:27:13  ✅ check regatta_core::transcript::tests::rejects_malformed_or_idless @ c7a535b5* → R-TXMETA-01
- 2026-06-28 08:27:13  ✅ check regatta_core::cost::tests::budget_pct_clamps_0_to_100 @ c7a535b5* → R-USAGE-01
- 2026-06-28 08:27:13  ✅ check regatta_store::tests::rolls_up_spend_by_model @ c7a535b5* → R-USAGE-01
- 2026-06-28 08:27:14  ✅ check regatta_core::view::tests::maps_each_event_kind @ c7a535b5* → R-VIEW-01
- 2026-06-28 08:27:14  ✅ check regatta_core::view::tests::preserves_empty_assistant_text @ c7a535b5* → R-VIEW-01
- 2026-06-28 08:27:14  ✅ check regatta_supervisor::tests::collects_parsed_events_from_stdout @ c7a535b5* → R-WIRE-01
- 2026-06-28 08:27:14  ✅ check regatta_supervisor::tests::skips_unparseable_lines @ c7a535b5* → R-WIRE-01
- 2026-06-28 08:27:14  ✅ check regatta_core::worktree::tests::plans_a_unique_worktree @ c7a535b5* → R-WORKTREE-01
- 2026-06-28 08:27:14  ✅ check regatta_core::worktree::tests::handles_a_short_uuid @ c7a535b5* → R-WORKTREE-01
