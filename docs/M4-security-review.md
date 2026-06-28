# M4 — Review & Multi-pane Orchestration: Adversarial Security Review

Recorded at the M4 close (2026-06-28). M4 parses git CLI output (untrusted if a repo is hostile) and
runs git in a working directory. This review *executes* hostile payloads against the parsers.

## New attack surface
- `regatta_core::git` — `parse_status`, `parse_numstat`, `summarize_diff`.
- `regatta_core::layout` — `GridLayout` / `panes`.
- `regatta_git` — runs `git status` / `git diff` in a caller-supplied cwd.

## Findings & fixes (1 hardening, proven by an executed test)
1. **u64 overflow in `summarize_diff`.** Summing per-file added/removed could overflow u64 on a giant
   or crafted diff → debug panic. **Fixed:** `saturating_add`. Proven by
   `git_parsers_and_summary_never_panic` (two files at u64::MAX → saturates to u64::MAX).

## Verified safe (executed payloads)
- **No parser panic.** `parse_status` / `parse_numstat` survive traversal paths, null bytes, control
  chars, 1 MB paths, garbage, and odd renames — they always return, never panic.
- **Path traversal is display-only.** A `../../etc/passwd` status path becomes *data* in a
  `FileChange`; the pure core never performs a filesystem op on it, and the reader runs git in a fixed,
  caller-supplied cwd — it never opens a parsed path.
- **The git reader is contained.** It invokes `git` with a fixed argument list (no shell, no
  interpolation of untrusted input) in the given cwd; a non-repo yields empty, not an error.

## Standing properties
- The pure core stays I/O-free and deterministic; glue is thin and integration-tested. `cargo deny`
  green. M4 added one internal crate (`regatta_git`) and **no third-party dependencies**.
