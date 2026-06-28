# M8 — Real-Time Focus + Approvals: 7-type test matrix

Recorded at the M8 close. M8 adds real-time Focus streaming (per-session Channel + coalescing) and the
inline-approval round-trip (parse → dock event → decision → agent response).

| # | Type | M8 evidence | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` lines 100%: `parse_approval_request`, `approval_response`, `coalesce` | ✅ |
| 2 | **Edge/boundary** | `adversarial_approval`: 1 MB command, 400-deep nesting, type confusion, unicode/RTL → no panic | ✅ |
| 3 | **Invalid/untrusted** | hostile `tool_name`/`command` are parsed as **inert data** (never executed); `approval_response` always valid JSON (reason escaped); `coalesce` never drops a priority approval | ✅ |
| 4 | **Scenario/integration** | allow/deny round-trip (parse→event→decision→response); the real-time Focus Channel streams a live session | ✅ |
| 5 | **E2e-smoke** | the Focus pane streams a live session over a Channel — render-and-look (`scratchpad/m8-rtfocus2.png`) | ✅ |
| 6 | **Performance/scale** | `coalesce` merges a token burst into one frame; 1 MB command parsed; the render-and-look **caught + fixed** a real sync-spawn panic | ✅ |
| 7 | **UI/UX·a11y** | streamed lines use the role-colored `EventLine`; an approval is a **priority frame** (never coalesced away from the human) | ✅ |

**DoD:** a session streams token-by-token into Focus live (proven, `m8-rtfocus2.png`); the approval
round-trip logic is proven (Approve echoes the input → proceed; Deny → stop). **Remaining for the full
DoD:** the in-process MCP permission server that carries the round-trip over stdio to a real `claude`
session (the *transport*; the *logic* is done and tested).
