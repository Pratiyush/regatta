# Feature: MCP approve result

> User story: As Regatta's MCP permission server, once the human decides I must return the decision to
> the agent as a proper MCP tools/call result, so the agent proceeds or stops.
> Intake: M9 — the approve response envelope.

## Requirement R-MCP-APPROVE-01 — wrap the decision as a tools/call result

WHEN building the approve `tools/call` response THEN the system SHALL wrap `approval_response` (allow echoes the input, deny carries the reason) as MCP text content inside a JSON-RPC result for the request id.

@check kind=unit ref=regatta_core::mcp::tests::approve_result_wraps_the_decision
