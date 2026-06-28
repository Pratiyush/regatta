# Feature: JSON-RPC framing

> User story: As Regatta's MCP permission server, I need to parse the agent's JSON-RPC requests and build
> responses, so I can speak the protocol `claude --permission-prompt-tool` uses.
> Intake: M9 — the MCP server's wire format.

## Requirement R-RPC-01 — parse requests and build responses

WHEN parsing a JSON-RPC line THEN the system SHALL extract id / method / params (id and params default to null); no method or malformed input SHALL return None.

@check kind=unit ref=regatta_core::mcp::tests::parses_a_jsonrpc_request

WHEN building a response THEN the system SHALL produce a JSON-RPC 2.0 result or error envelope.

@check kind=unit ref=regatta_core::mcp::tests::rejects_non_requests_and_builds_responses
