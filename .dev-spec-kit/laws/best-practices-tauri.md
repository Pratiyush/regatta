---
inclusion: always
---
# Best practices — Tauri (the trust boundary)

- **The Tauri command boundary IS the trust boundary.** Validate and scope every `#[command]` input; never pass a user/agent-supplied string to a shell or path API unsanitized.
- **Least-privilege.** Capabilities grant only the exact commands/paths a window needs — no blanket `fs`/`shell`/`http`/`path` scopes. Strict CSP. Never enable `dangerousRemoteDomainIpcAccess` or `withGlobalTauri` without a written reason here.
- High-frequency output uses `tauri::ipc::Channel`, not global events: coalesce (~16 ms), apply backpressure, and downgrade hidden panes to status-only.
- Every spawned agent process is a process-group / Job-Object leader, tagged `HELM_SESSION_ID`, reaped on `Drop`. **Zero leaked helpers is a release gate.**
- `cargo audit` + `cargo deny` pass in CI. A **payload-executing adversarial review** precedes every release tag (command-injection, path traversal, IPC abuse, DoS, deserialization).
