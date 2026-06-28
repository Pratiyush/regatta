# Feature: MCP dispatch

> User story: As Regatta's MCP permission server, I need to route the agent's requests — answer
> initialize/tools.list myself, and turn a tools/call(approve) into a dock approval — so a real session
> can ask for permission.
> Intake: M9 — the permission server's routing.

## Requirement R-MCP-DISPATCH-01 — route MCP requests

WHEN dispatching THEN `initialize` and `tools/list` SHALL get a ready Reply, and `tools/call("approve")` SHALL produce an Approve action carrying the parsed ApprovalRequested.

@check kind=unit ref=regatta_core::mcp::tests::dispatch_routes_mcp_methods

WHEN the tool is unknown, the approve arguments are invalid, or the method is unknown THEN the system SHALL reply with an error (and notifications SHALL be ignored).

@check kind=unit ref=regatta_core::mcp::tests::dispatch_handles_errors_and_notifications
