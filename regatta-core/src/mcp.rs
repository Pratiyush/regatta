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
}
