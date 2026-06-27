import { createResource, createSignal, For, Show, type Component } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

type SessionView = {
  id: string; name: string; project: string; branch: string;
  status: string; status_label: string; priority: number;
  action: string; cost: string; reason: string;
};
type DockView = { sessions: SessionView[]; order: number[] };
type EventLine = { role: string; text: string };

const STATUS_COLOR: Record<string, string> = {
  running: "#5fb98a", "needs-input": "#e0b15e", "waiting-approval": "#e0b15e",
  error: "#e07a6e", done: "#6e9fe0", starting: "#8a887f",
  compacting: "#b08ee0", "rate-limited": "#e0b15e", paused: "#8a887f",
};
const color = (s: string) => STATUS_COLOR[s] ?? "#8a887f";

const ROLE_COLOR: Record<string, string> = {
  claude: "#e0925e", system: "#8a887f", usage: "#5fb98a", you: "#6e9fe0",
};

async function fetchDock(): Promise<DockView> {
  try { return await invoke<DockView>("dock_view"); } catch { return { sessions: [], order: [] }; }
}
async function runDemo(): Promise<EventLine[]> {
  try { return await invoke<EventLine[]>("run_demo_session"); } catch { return []; }
}

const App: Component = () => {
  const [data] = createResource(fetchDock);
  const [session, { refetch }] = createResource(runDemo); // a real session, driven by the pipeline
  const [busy, setBusy] = createSignal(false);
  const runAgain = async () => { setBusy(true); await refetch(); setBusy(false); };

  const groups = () => {
    const sessions = data()?.sessions ?? [];
    const m = new Map<string, SessionView[]>();
    for (const s of sessions) {
      if (!m.has(s.project)) m.set(s.project, []);
      m.get(s.project)!.push(s);
    }
    return [...m.entries()];
  };
  const dock = () => {
    const d = data();
    return d ? d.order.map((i) => d.sessions[i]) : [];
  };
  const live = () => busy() || session.loading;

  return (
    <div class="app">
      <header class="topbar">
        <div class="brand"><span class="logo">R</span> Regatta</div>
        <div class="spacer" />
        <div class="usage">
          <b>$9.12</b> today <span class="sep">·</span> <b>1.2M</b> tok
          <span class="sep">·</span> <span class="live">● {dock().length} need you</span>
        </div>
        <div class="views">
          <button class="seg active">Focus</button>
          <button class="seg">Grid</button>
          <button class="seg">Sessions</button>
        </div>
      </header>

      <div class="body">
        {/* Left: project-grouped session sidebar */}
        <aside class="sidebar">
          <div class="panel-h">Projects <button class="newbtn" title="New session (⌘T)">+</button></div>
          <div class="scroll">
            <For each={groups()}>{([project, sessions]) => (
              <div class="group">
                <div class="group-h">{project}<span class="count">{sessions.length}</span></div>
                <For each={sessions}>{(s) => (
                  <div class="row">
                    <span class="dot" style={{ background: color(s.status) }} />
                    <div class="row-main">
                      <div class="row-name">{s.name}</div>
                      <div class="row-meta">{s.action || s.branch}</div>
                    </div>
                    <span class="row-status" style={{ color: color(s.status) }}>{s.status_label}</span>
                  </div>
                )}</For>
              </div>
            )}</For>
          </div>
        </aside>

        {/* Center: the active session — driven live by the real supervisor→parser→view pipeline */}
        <main class="focus">
          <div class="focus-h">
            <span class="dot" style={{ background: live() ? "#5fb98a" : "#6e9fe0" }} />
            <span class="focus-title">payments-svc / fix-idempotency</span>
            <span class="pill" style={{ color: live() ? "#5fb98a" : "#6e9fe0", "border-color": live() ? "#5fb98a" : "#6e9fe0" }}>
              {live() ? "Working" : "Done"}
            </span>
            <span class="spacer" />
            <button class="run" onClick={runAgain} disabled={live()}>▶ Run test session</button>
          </div>
          <div class="stream">
            <Show when={!session.loading} fallback={<div class="empty">Starting session…</div>}>
              <For each={session()} fallback={<div class="empty">Press “Run test session” to drive a live session.</div>}>{(ln) => (
                <div class="msg">
                  <span class="role" style={{ color: ROLE_COLOR[ln.role] ?? "#8a887f" }}>{ln.role}</span>
                  <div class="msg-body">{ln.text}</div>
                </div>
              )}</For>
            </Show>
          </div>
          <div class="composer"><span class="caret">›</span> Message the agent, or type / for commands…</div>
        </main>

        {/* Right: the Attention Dock — Regatta's headline */}
        <aside class="dock">
          <div class="panel-h">Attention <span class="badge">{dock().length}</span> <span class="hint">⌘⇧U</span></div>
          <div class="scroll">
            <For each={dock()} fallback={<div class="empty">All clear 🎉</div>}>{(s) => (
              <div class="card" style={{ "border-left-color": color(s.status) }}>
                <div class="card-top">
                  <span class="pri" style={{ background: color(s.status) }}>{s.priority}</span>
                  <span class="card-name">{s.project} / {s.name}</span>
                </div>
                <div class="card-status" style={{ color: color(s.status) }}>{s.status_label}</div>
                <div class="card-reason">{s.reason || s.action}</div>
                <Show when={s.status === "waiting-approval" || s.status === "needs-input"}>
                  <div class="card-actions">
                    <button class="approve">Approve</button>
                    <button class="deny">Deny</button>
                  </div>
                </Show>
              </div>
            )}</For>
          </div>
        </aside>
      </div>
    </div>
  );
};

export default App;
