# M6 — Codex Backend: payload-executing adversarial review

Per the constitution, every release tag is preceded by a multi-angle adversarial review that **executes
payloads**, not just reads code. M6 adds one untrusted-input surface (the Codex transcript parsers) and
one argv builder (the Codex launch plan). Both were attacked — see `regatta-core/tests/adversarial_codex.rs`.

## Attack surface (new in M6)
1. **Codex transcript parsers** — `parse_codex_line` / `parse_codex_meta` ingest JSON from
   `~/.codex/sessions/**/rollout-*.jsonl`, which Regatta does not author and must treat as hostile.
2. **Codex launch plan** — `plan_codex_launch` builds argv + env from a model + session id that may
   originate from an indexed rollout (i.e. attacker-influenced).
3. **Backend dispatch** — adds no I/O; it only routes to the above.

## Attacks executed & results

| Angle | Payload | Result |
|-------|---------|--------|
| Malformed / DoS | empty, null bytes, bare scalar, array-not-object | → `None`, no panic |
| Parser stack overflow | 500-deep `{` nesting | serde recursion limit → `Err` → `None` (no stack overflow) |
| Integer overflow | `input_tokens` = `u64::MAX` | parsed as `u64`, no wrap/panic |
| Type confusion | negative / float token counts, `id` as array, `cwd` as object, `content` as string | `as_u64`/`as_str` reject → `0` / `None`, no panic |
| Memory / quadratic | 1 MB assistant-text content block | linear concat, parses fine |
| Unicode / spoofing | math-bold glyphs + 🚀 + RTL override (U+202E) | passed through as opaque text, no panic |
| **Command injection** | `session_id = "abc; rm -rf / # $(whoami) \`id\` && curl evil"`, `model = "$(touch /tmp/pwned)"` | each survives as exactly ONE argv element; program fixed to `codex`; no shell involved |

## Findings
**None.** The Codex parsers are total functions over arbitrary bytes (serde + `Option` combinators; no
`unwrap`/`expect`/indexing on untrusted data), so a hostile rollout yields `None`, never a crash or an
invalid state. The launch plan is an argv/env builder that the supervisor execs **directly** (no shell),
so shell metacharacters in a session id/model are inert data — they cannot inject a command.

## Standing controls (unchanged, still hold for Codex)
- Process-group spawn + graded SIGTERM→SIGKILL teardown reaps the Codex process tree too
  (proved by `codex_runs_through_the_same_pipeline_as_claude`).
- The Tauri command boundary still validates inputs; M6 added no new commands beyond a backend field
  on the existing `dock_view`.
