# Feature: Wire the approve MCP server into a claude launch

> User story: As Regatta, I want a claude session launched with my permission server registered, so its
> tool requests reach the dock.
> Intake: M9 — wiring claude to the MCP permission server.

## Requirement R-APPROVE-WIRE-01 — build the approve MCP server config

WHEN building the approve MCP config THEN the system SHALL produce a `.mcp.json` registering `<exe> mcp-approve` as the `regatta` server (so `--permission-prompt-tool mcp__regatta__approve` resolves to it).

@check kind=unit ref=regatta_core::mcp::tests::builds_the_approve_server_config
