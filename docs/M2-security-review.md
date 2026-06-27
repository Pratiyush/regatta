# M2 — Resume & Persistence: Adversarial Security Review

Recorded at the M2 close (2026-06-28). M2 added real attack surface: parsing **untrusted transcript
JSON** from disk, **walking the filesystem** to index, and a **free-text search** into SQLite. Per the
method, this review *executes* hostile payloads against the real functions — it does not just read code.

## New attack surface
- `regatta_core::transcript::parse_session_meta` — parses arbitrary transcript lines.
- `regatta_supervisor::tail_transcript` — reads arbitrary/partial files by byte offset.
- `regatta_indexer::index_dir` — walks a directory tree and reads files.
- `regatta_store::board_query` — a free-text search into SQLite.
- `regatta_core::board::recency_group` — arithmetic on a stored integer.
- Tauri `board_list` — the command boundary (a `query: String`).

## Findings & fixes (2 hardenings, both proven by executed tests)
1. **Integer overflow in `recency_group`.** `now - last_activity` overflowed on i64 extremes (a corrupt
   mtime or stored value) → debug panic. **Fixed:** `now.saturating_sub(last_activity)`. Proven by
   `regatta_core::adversarial::recency_group_never_overflows` (every pair of i64 extremes).
2. **Symlink-cycle infinite recursion in `index_dir`.** `path.is_dir()` follows symlinks; a cycle
   (`sub/loop → parent`) would recurse forever — a DoS / stack overflow. **Fixed:** skip symlink entries
   (`entry.file_type().is_symlink()`). Proven by `regatta_indexer::tests::does_not_follow_symlink_cycles`.

## Verified safe (executed payloads, no change required)
- **No SQL injection.** `board_query("x'; DROP TABLE transcript_index; --")` is bound via a `?1`
  parameter → treated as a literal → matches nothing, table intact. (`board_query_resists_sql_injection`)
- **No parser panic / DoS.** `parse_session_meta` survives deep nesting (serde's recursion limit),
  2 MB ids, wrong types, and null bytes — it returns `Some`/`None`, never panics.
- **No path traversal in labels.** A `cwd` of `../../../…` yields the project label `session` (the slug
  of an empty leaf), never a `..`-bearing string. (`transcript_meta_never_panics_on_hostile_input`)
- **No traversal via tail.** Offsets always land on newline boundaries (a single ASCII byte), so reads
  stay UTF-8-valid; a binary or rotated file errors cleanly and reattach skips it.
- **Command boundary.** `board_list` passes its query straight to the parameterized `board_query`,
  reads only `~/.claude/projects`, and returns metadata + a resume command — never transcript content.

## Standing properties (unchanged from M1)
- The pure core stays I/O-free and deterministic; glue is thin and integration-tested.
- `cargo deny` (advisories / licenses / bans) green. M2 added no third-party dependencies — only the
  internal `regatta_indexer` crate over `regatta_core` + `regatta_store` + `regatta_supervisor`.
