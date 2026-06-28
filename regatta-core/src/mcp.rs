//! Minimal JSON-RPC 2.0 framing for the MCP permission server — pure parse/build, no I/O.

use serde_json::{json, Value};

/// A parsed JSON-RPC request.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcRequest {
    pub id: Value, // number, string, or null (notification)
    pub method: String,
    pub params: Value, // the params object, or Null
}

/// Parse one JSON-RPC request line. Returns None if it isn't a valid request (no method) or malformed.
pub fn parse_rpc(line: &str) -> Option<RpcRequest> {
    let v: Value = serde_json::from_str(line).ok()?;
    let method = v.get("method")?.as_str()?.to_string();
    Some(RpcRequest {
        id: v.get("id").cloned().unwrap_or(Value::Null),
        method,
        params: v.get("params").cloned().unwrap_or(Value::Null),
    })
}

/// Build a JSON-RPC success response.
pub fn rpc_result(id: &Value, result: Value) -> String {
    json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string()
}

/// Build a JSON-RPC error response.
pub fn rpc_error(id: &Value, code: i64, message: &str) -> String {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } }).to_string()
}

/// What the server should do with a parsed request — computed purely; the glue performs the I/O.
#[derive(Debug, Clone, PartialEq)]
pub enum McpAction {
    /// A ready JSON-RPC response to write back immediately.
    Reply(String),
    /// A `tools/call("approve")`: surface this request to the dock, then reply once the human decides.
    Approve {
        id: Value,
        request: crate::stream::NormalizedEvent,
        raw_input: String,
    },
    /// Nothing to send (e.g. a notification).
    Ignore,
}

/// Route a parsed MCP request to an action.
pub fn dispatch(req: &RpcRequest) -> McpAction {
    match req.method.as_str() {
        "initialize" => McpAction::Reply(rpc_result(&req.id, initialize_result())),
        "tools/list" => McpAction::Reply(rpc_result(&req.id, tools_list_result())),
        "tools/call" => {
            let name = req
                .params
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if name != "approve" {
                return McpAction::Reply(rpc_error(&req.id, -32601, "unknown tool"));
            }
            let raw = req
                .params
                .get("arguments")
                .cloned()
                .unwrap_or(Value::Null)
                .to_string();
            match crate::stream::parse_approval_request(&raw) {
                Some(request) => McpAction::Approve {
                    id: req.id.clone(),
                    request,
                    raw_input: raw,
                },
                None => McpAction::Reply(rpc_error(&req.id, -32602, "invalid approve arguments")),
            }
        }
        m if m.starts_with("notifications/") => McpAction::Ignore,
        _ => McpAction::Reply(rpc_error(&req.id, -32601, "method not found")),
    }
}

fn initialize_result() -> Value {
    json!({
        "protocolVersion": "2024-11-05",
        "serverInfo": { "name": "regatta", "version": "1" },
        "capabilities": { "tools": {} }
    })
}

fn tools_list_result() -> Value {
    json!({
        "tools": [{
            "name": "approve",
            "description": "Ask the human to approve a tool use",
            "inputSchema": { "type": "object" }
        }]
    })
}

/// Build the JSON-RPC response for a `tools/call("approve")` once the human has decided: the MCP
/// tools/call result wraps the permission `approval_response` as text content.
pub fn approve_result(
    id: &Value,
    decision: crate::approval::Decision,
    input: &str,
    reason: &str,
) -> String {
    let body = crate::approval::approval_response(decision, input, reason);
    rpc_result(id, json!({ "content": [{ "type": "text", "text": body }] }))
}

/// The `.mcp.json` that registers Regatta's permission server with a `claude` session, so
/// `--permission-prompt-tool mcp__regatta__approve` resolves to `<exe> mcp-approve`.
pub fn approve_server_config(exe: &str) -> String {
    json!({
        "mcpServers": { "regatta": { "command": exe, "args": ["mcp-approve"] } }
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_jsonrpc_request() {
        assert_eq!(
            parse_rpc(
                r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"approve"}}"#
            ),
            Some(RpcRequest {
                id: json!(1),
                method: "tools/call".into(),
                params: json!({"name":"approve"})
            })
        );
        // a notification: no id → Null, no params → Null
        assert_eq!(
            parse_rpc(r#"{"jsonrpc":"2.0","method":"initialized"}"#),
            Some(RpcRequest {
                id: Value::Null,
                method: "initialized".into(),
                params: Value::Null
            })
        );
    }

    #[test]
    fn rejects_non_requests_and_builds_responses() {
        assert_eq!(parse_rpc("not json"), None);
        assert_eq!(parse_rpc(r#"{"id":1}"#), None); // no method
        assert_eq!(parse_rpc(r#"{"method":5}"#), None); // method not a string
        assert_eq!(
            rpc_result(&json!(1), json!({"ok": true})),
            r#"{"id":1,"jsonrpc":"2.0","result":{"ok":true}}"#
        );
        assert_eq!(
            rpc_error(&json!(2), -32601, "method not found"),
            r#"{"error":{"code":-32601,"message":"method not found"},"id":2,"jsonrpc":"2.0"}"#
        );
    }

    fn req(method: &str, params: Value) -> RpcRequest {
        RpcRequest {
            id: json!(1),
            method: method.into(),
            params,
        }
    }

    #[test]
    fn dispatch_routes_mcp_methods() {
        use crate::stream::NormalizedEvent;
        assert!(matches!(
            dispatch(&req("initialize", Value::Null)),
            McpAction::Reply(ref s) if s.contains("serverInfo") && s.contains("regatta")
        ));
        assert!(matches!(
            dispatch(&req("tools/list", Value::Null)),
            McpAction::Reply(ref s) if s.contains("\"approve\"")
        ));
        let call = req(
            "tools/call",
            json!({"name":"approve","arguments":{"tool_name":"Bash","input":{"command":"ls"}}}),
        );
        let expected = NormalizedEvent::ApprovalRequested {
            tool: "Bash".into(),
            detail: "ls".into(),
        };
        assert!(matches!(
            dispatch(&call),
            McpAction::Approve { ref request, .. } if *request == expected
        ));
    }

    #[test]
    fn dispatch_handles_errors_and_notifications() {
        assert!(matches!(
            dispatch(&req("tools/call", json!({"name":"frobnicate"}))),
            McpAction::Reply(ref s) if s.contains("unknown tool")
        ));
        assert!(matches!(
            dispatch(&req("tools/call", json!({"name":"approve","arguments":{"no":"tool_name"}}))),
            McpAction::Reply(ref s) if s.contains("invalid approve")
        ));
        assert_eq!(
            dispatch(&req("notifications/initialized", Value::Null)),
            McpAction::Ignore
        );
        assert!(matches!(
            dispatch(&req("nonsense", Value::Null)),
            McpAction::Reply(ref s) if s.contains("method not found")
        ));
    }

    #[test]
    fn approve_result_wraps_the_decision() {
        use crate::approval::Decision;
        let allow = approve_result(&json!(7), Decision::Allow, r#"{"command":"ls"}"#, "");
        let v: Value = serde_json::from_str(&allow).unwrap();
        assert_eq!(v["id"], json!(7));
        assert_eq!(v["result"]["content"][0]["type"], "text");
        let body: Value =
            serde_json::from_str(v["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(body["behavior"], "allow");
        assert_eq!(body["updatedInput"], json!({"command":"ls"}));

        let deny = approve_result(&json!(8), Decision::Deny, "{}", "nope");
        let dv: Value = serde_json::from_str(&deny).unwrap();
        let dbody: Value =
            serde_json::from_str(dv["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(dbody["behavior"], "deny");
        assert_eq!(dbody["message"], "nope");
    }

    #[test]
    fn builds_the_approve_server_config() {
        let cfg = approve_server_config("/usr/local/bin/regatta");
        let v: Value = serde_json::from_str(&cfg).unwrap();
        assert_eq!(
            v["mcpServers"]["regatta"]["command"],
            "/usr/local/bin/regatta"
        );
        assert_eq!(v["mcpServers"]["regatta"]["args"][0], "mcp-approve");
    }
}
