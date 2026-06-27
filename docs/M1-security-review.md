# M1 — adversarial security review (milestone close)

Regatta is unusually security-sensitive: it **spawns agent processes that run arbitrary commands**, **parses untrusted process output**, builds **git branch/filesystem names from user session names**, and runs a **Tauri webview**. This review *executes* payloads (see `regatta-core/tests/adversarial.rs`) rather than only reading code.

## Threats reviewed & findings

| # | Threat | Finding | Mitigation (executed where noted) |
|---|--------|---------|-----------------------------------|
| 1 | **Command injection** via session name → branch/path | ✅ Mitigated | `slugify` reduces names to `[a-z0-9-]`; the supervisor passes argv as a `Vec` (no shell string); worktree names never interpolate into a shell. Test: `slugify_neutralizes_shell_and_path_payloads`, `worktree_plan_resists_traversal_and_injection`. |
| 2 | **Path traversal** via session name → worktree path | ✅ Mitigated | Slug sanitized; path stays under the worktrees dir; no `..`. Test: `worktree_plan_resists_traversal_and_injection`. |
| 3 | **Parser DoS / panic** via hostile agent stdout | ✅ Mitigated | `parse_claude_line` never panics on 2 MB lines, 20k content blocks, overflow numbers, null bytes, or malformed JSON — it returns `None`. Test: `parser_never_panics_on_hostile_input`. |
| 4 | **Process leak / runaway agent** | ✅ Mitigated | Process-group spawn + graded `SIGTERM`→`SIGKILL` of the whole group + `Drop` reaper; integration test asserts a spawned grandchild dies. |
| 5 | **Tauri trust boundary** | ✅ Mitigated | Command inputs are typed; least-privilege capability (`core:default` only — no blanket fs/shell/http); strict CSP (no external origins — why we bundle, not CDN, fonts); `dangerousRemoteDomainIpcAccess`/`withGlobalTauri` off. |
| 6 | **Supply chain** | ✅ Mitigated | `cargo-deny` (advisories + licenses + bans + sources) green in CI; internal crates marked `publish = false`. |

## Residual risk (by design, gated later)
- Real `claude` will execute actual arbitrary commands — that *is* the product. The **permission/approval** mechanism (MCP permission-prompt tool + the inline approval UI) gates risky tool calls; the inline-approval UX exists, the enforcement binding lands in a later slice.
- The hook **control socket** (loopback + per-launch token) lands with the hooks integration.
- **Secret/env masking** is scheduled for M5 (config layer).

## Verdict
No blocking findings for M1. The untrusted-input surfaces are sanitized and panic-safe, process teardown is leak-free, and the Tauri boundary is locked down. Residual items are tracked and gated to their milestones.
