# Feature: MCP server loop

> User story: As Regatta, I want a server loop that reads an agent's JSON-RPC over a stream, dispatches
> it, and writes responses — surfacing approvals to a decision callback — so a real `claude
> --permission-prompt-tool` session can ask for permission.
> Intake: M9 — the permission server transport.

## Requirement R-MCP-SERVE-01 — run the MCP server loop

WHEN the server processes a session's requests THEN `initialize` SHALL get a reply, and a `tools/call("approve")` SHALL surface the parsed request to the `decide` callback and write the resulting approve response.

@check kind=integration ref=regatta_supervisor::tests::serve_mcp_handles_a_session
