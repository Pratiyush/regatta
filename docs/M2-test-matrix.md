# M2 — Resume & Persistence: 7-type Test Matrix

Recorded at the M2 close (2026-06-28). M2 made session history durable, searchable, resumable, and
crash-recoverable. Every row passes or its gap is logged.

| # | Type | M2 coverage | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` (`transcript`, `board`, `backend::plan_resume`/`resume_command`) at 100% line+branch via `cargo-llvm-cov` (563 regions). | ✅ |
| 2 | **Edge/boundary** | tail-from-offset holding a partial line + truncation restart; recency at day boundaries + future timestamps; `board_query` empty/no-match; indexer scans a file's head only. | ✅ |
| 3 | **Invalid/untrusted** | `transcript_meta_never_panics_on_hostile_input` (deep nesting, 2 MB id, wrong types, null bytes, traversal cwd); `recency_group_never_overflows` (i64 extremes); `board_query_resists_sql_injection`; `does_not_follow_symlink_cycles`. | ✅ |
| 4 | **Scenario/integration** | reattach across a simulated restart (replay → 0-new → append → 1-new); index a fixtures dir; incremental tail; **lived test: 4,872 real transcripts → 44 distinct sessions** indexed. | ✅ |
| 5 | **E2e-smoke** | app builds + launches; the Sessions board renders real history and offers `claude --resume` per row. (render-and-look) | ✅ |
| 6 | **Performance/scale** | indexer reads only each file's **head (64 KB)**, never the 44 MB whole; board search is index-backed (`idx_transcript_activity`); 4,872 files indexed at startup in ~1–2 s. | ✅ |
| 7 | **UI/UX·a11y** | render-and-look of the Sessions board: recency groups, project tags, mono resume commands, search, Resume/Copy, Re-index; dark-theme consistent; controls keyboard-reachable. | ✅ |

Artifact: `scratchpad/m2-board3.png` — the board on real history.
