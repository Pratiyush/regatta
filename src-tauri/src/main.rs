// Prevents an extra console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // MCP permission-server mode: a `claude --permission-prompt-tool mcp__regatta__approve` session
    // spawns `regatta mcp-approve`; we serve JSON-RPC over stdin/stdout. The dock-decision socket is the
    // remaining wiring, so until it lands we safely default to Deny with a clear message.
    if std::env::args().nth(1).as_deref() == Some("mcp-approve") {
        let _ = regatta_supervisor::serve_mcp(
            std::io::stdin().lock(),
            std::io::stdout().lock(),
            |_req| {
                (
                    regatta_core::approval::Decision::Deny,
                    "Regatta approval UI is not connected to this session".to_string(),
                )
            },
        );
        return;
    }
    regatta_lib::run();
}
