# M8 — Real-Time Focus + Approvals: payload-executing adversarial review

M8 adds the approval surface (`parse_approval_request` ingests an agent's untrusted tool-use request;
`approval_response` emits what the agent receives) plus the real-time Channel. Both were attacked —
see `regatta-core/tests/adversarial_approval.rs`.

## Attacks executed & results
| Angle | Payload | Result |
|-------|---------|--------|
| Parser DoS / overflow | empty, null bytes, 400-deep nesting, 1 MB command | → Some/None, no panic or stack overflow |
| Type confusion | command as array, input as int, tool_name with metacharacters | `as_str` rejects → detail falls back / None; never panics |
| **Command injection** | `tool_name = "Bash; rm -rf / #"`, `command = "$(whoami)\`id\`"` | parsed as **inert data** — Regatta never executes it; it only surfaces it to the dock for a human decision |
| JSON integrity | hostile `original_input` (malformed/huge), reason with quotes/newlines/`;` | `approval_response` always emits valid JSON; the reason is JSON-escaped, inert |
| Priority suppression | a burst with an `ApprovalRequested` among text deltas | `coalesce` never merges/drops the approval — it stays a discrete frame the human sees |
| Real-time spawn | (found via render-and-look) a **sync** launch command spawning a tokio process | panicked "no reactor running" → **FIXED**: launch commands are now `async` |

## Findings
**One real bug, fixed** (found by the render-and-look, not by reading): the sync launch command could
not spawn a `tokio::process::Child` ("no reactor running") — fixed by making `launch_session` /
`launch_demo_live` `async`. **No security findings:** the approval parser treats a hostile request as
data (Regatta never executes the requested command — it asks a human), and `approval_response` always
emits valid, escaped JSON.

## The trust boundary
The whole point of approvals is that the agent's requested action is shown to a human *before* it runs.
Regatta parses the request (never executes it), surfaces it as a **priority frame** (never coalesced
away), and only the human's Approve/Deny produces the response that lets the agent proceed.
