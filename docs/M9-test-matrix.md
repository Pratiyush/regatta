# M9 — MCP Permission Server: 7-type test matrix

Recorded at the M9 close. M9 builds the MCP permission server that carries the approval round-trip to a
real `claude --permission-prompt-tool` session.

| # | Type | M9 evidence | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` lines 100%: parse_rpc, rpc_result/error, dispatch, approve_result, approve_server_config | ✅ |
| 2 | **Edge/boundary** | `adversarial_mcp`: empty / null / 400-deep / 1 MB command / method-not-string → no panic | ✅ |
| 3 | **Invalid/untrusted** | an injection-y approve request is surfaced as **inert data** (never executed); unknown method/tool → error Reply | ✅ |
| 4 | **Scenario/integration** | `serve_mcp` loop (initialize → reply, approve → decide → result); the approval round-trip composes | ✅ |
| 5 | **E2e-smoke** | the **`regatta mcp-approve` binary** serves MCP over stdio (real subprocess: initialize → serverInfo, tools/call → result) | ✅ |
| 6 | **Performance/scale** | 1 MB command parsed; `serve_mcp` streams line-by-line | ✅ |
| 7 | **UI/UX·a11y** | the dock's Approve/Deny (M1) is the human decision point; approvals are priority frames never coalesced away (M8) | ✅ |

**DoD:** the MCP permission server is a real, callable server (claude can spawn `regatta mcp-approve`),
and the approval round-trip is proven end-to-end in code. **Remaining:** the dock-click→decision socket
(the binary safe-defaults to **Deny** until then) + a real-`claude` lived run — the last runtime wiring.
