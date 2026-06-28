import { createResource, createSignal, For, Show, type Component } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

type SessionView = {
  id: string; name: string; project: string; branch: string;
  status: string; status_label: string; priority: number;
  action: string; cost: string; reason: string; backend: string;
};
type DockView = { sessions: SessionView[]; order: number[] };
type EventLine = { role: string; text: string };
type BoardRow = { session_id: string; project: string; title: string; group: string; resume_cmd: string };
type Spend = { name: string; usd: number };
type UsageView = { today_usd: number; burn_per_hr: number; budget_usd: number; budget_pct: number; by_project: Spend[]; by_model: Spend[] };
type ReviewItem = { session: string; project: string; branch: string; files: number; added: number; removed: number };
type FileEntry = { path: string; status: string; added: number | null; removed: number | null };
type Cfg = { key: string; value: string; secret: boolean };
type Toggle = { name: string; enabled: boolean };
type Mcp = { name: string; tools: number; enabled: boolean };
type SettingsView = { config: Cfg[]; mcp_servers: Mcp[]; skills: Toggle[]; commands: string[] };

const STATUS_COLOR: Record<string, string> = {
  running: "#5fb98a", "needs-input": "#e0b15e", "waiting-approval": "#e0b15e",
  error: "#e07a6e", done: "#6e9fe0", starting: "#8a887f",
  compacting: "#b08ee0", "rate-limited": "#e0b15e", paused: "#8a887f",
};
const color = (s: string) => STATUS_COLOR[s] ?? "#8a887f";

const ROLE_COLOR: Record<string, string> = {
  claude: "#e0925e", system: "#8a887f", usage: "#5fb98a", you: "#6e9fe0",
};

const RECENCY_ORDER = ["Today", "Yesterday", "Earlier this week", "Last two weeks", "Older"];

async function fetchDock(): Promise<DockView> {
  try { return await invoke<DockView>("dock_view"); } catch { return { sessions: [], order: [] }; }
}
async function runDemo(): Promise<EventLine[]> {
  try { return await invoke<EventLine[]>("run_demo_session"); } catch { return []; }
}
async function fetchBoard(query: string): Promise<BoardRow[]> {
  try { return await invoke<BoardRow[]>("board_list", { query }); } catch { return []; }
}
async function fetchUsage(): Promise<UsageView> {
  try { return await invoke<UsageView>("usage_view"); }
  catch { return { today_usd: 0, burn_per_hr: 0, budget_usd: 0, budget_pct: 0, by_project: [], by_model: [] }; }
}
async function fetchInbox(): Promise<ReviewItem[]> {
  try { return await invoke<ReviewItem[]>("review_inbox"); } catch { return []; }
}
async function fetchDiff(session: string): Promise<FileEntry[]> {
  try { return await invoke<FileEntry[]>("diff_view", { session }); } catch { return []; }
}
async function fetchSettings(): Promise<SettingsView> {
  try { return await invoke<SettingsView>("settings_view"); }
  catch { return { config: [], mcp_servers: [], skills: [], commands: [] }; }
}

const App: Component = () => {
  const [data] = createResource(fetchDock);
  const [session, { refetch }] = createResource(runDemo); // a real session, driven by the pipeline
  const [busy, setBusy] = createSignal(false);
  const runAgain = async () => { setBusy(true); await refetch(); setBusy(false); };

  const [view, setView] = createSignal<"focus" | "usage" | "review" | "sessions" | "settings">("focus");
  const [query, setQuery] = createSignal("");
  const [board, { refetch: refetchBoard }] = createResource(query, fetchBoard);
  const [usage] = createResource(fetchUsage);
  const [inbox] = createResource(fetchInbox);
  const [selected, setSelected] = createSignal("s1");
  const [diff] = createResource(selected, fetchDiff);
  const [settings] = createResource(fetchSettings);
  const money = (n: number) => `$${(n ?? 0).toFixed(2)}`;
  const pctOf = (usd: number, list?: Spend[]) => {
    const max = Math.max(1e-9, ...(list ?? []).map((s) => s.usd));
    return Math.round((usd / max) * 100);
  };
  const [copied, setCopied] = createSignal("");
  const copyResume = async (cmd: string, id: string) => {
    try { await navigator.clipboard.writeText(cmd); } catch { /* clipboard may be blocked */ }
    setCopied(id);
    setTimeout(() => setCopied(""), 1500);
  };
  const reindex = async () => { try { await invoke("board_reindex"); } catch { /* ignore */ } await refetchBoard(); };

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

  const boardGroups = () => {
    const rows = board() ?? [];
    const m = new Map<string, BoardRow[]>();
    for (const r of rows) {
      if (!m.has(r.group)) m.set(r.group, []);
      m.get(r.group)!.push(r);
    }
    return RECENCY_ORDER.filter((g) => m.has(g)).map((g) => [g, m.get(g)!] as const);
  };

  return (
    <div class="app">
      <header class="topbar">
        <div class="brand"><span class="logo">R</span> Regatta</div>
        <div class="spacer" />
        <div class="usage clickable" onClick={() => setView("usage")} title="Open Usage">
          <b>{money(usage()?.today_usd ?? 0)}</b> today <span class="sep">·</span>
          <span class="live">● {dock().length} need you</span>
        </div>
        <div class="views">
          <button class={`seg ${view() === "focus" ? "active" : ""}`} onClick={() => setView("focus")}>Focus</button>
          <button class={`seg ${view() === "usage" ? "active" : ""}`} onClick={() => setView("usage")}>Usage</button>
          <button class={`seg ${view() === "review" ? "active" : ""}`} onClick={() => setView("review")}>Review</button>
          <button class={`seg ${view() === "sessions" ? "active" : ""}`} onClick={() => setView("sessions")}>Sessions</button>
          <button class={`seg gear ${view() === "settings" ? "active" : ""}`} onClick={() => setView("settings")} title="Settings & Extensions">⚙</button>
        </div>
      </header>

      {/* ───────── Focus / cockpit view ───────── */}
      <Show when={view() === "focus"}>
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
                        <div class="row-name">{s.name} <span class="bk" classList={{ codex: s.backend === "Codex" }}>{s.backend}</span></div>
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
                    <span class="bk" classList={{ codex: s.backend === "Codex" }}>{s.backend}</span>
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
      </Show>

      {/* ───────── Resume board (Sessions) — M2 ───────── */}
      <Show when={view() === "sessions"}>
        <div class="board">
          <div class="board-h">
            <input
              class="search"
              placeholder="Search your history by title or project…"
              value={query()}
              onInput={(e) => setQuery(e.currentTarget.value)}
            />
            <span class="board-count">{board()?.length ?? 0} shown</span>
            <button class="reindex" onClick={reindex}>↻ Re-index</button>
          </div>
          <div class="board-scroll">
            <Show when={!board.loading} fallback={<div class="empty">Indexing your sessions…</div>}>
              <For each={boardGroups()} fallback={<div class="empty">No sessions match “{query()}”.</div>}>{([g, rows]) => (
                <div class="rgroup">
                  <div class="rgroup-h">{g}<span class="count">{rows.length}</span></div>
                  <For each={rows}>{(r) => (
                    <div class="brow">
                      <span class="ptag">{r.project}</span>
                      <div class="brow-main">
                        <div class="brow-title">{r.title}</div>
                        <div class="brow-cmd">{r.resume_cmd}</div>
                      </div>
                      <button class="resume" classList={{ ok: copied() === r.session_id }} onClick={() => copyResume(r.resume_cmd, r.session_id)}>
                        {copied() === r.session_id ? "✓ copied" : "⏎ Resume"}
                      </button>
                    </div>
                  )}</For>
                </div>
              )}</For>
            </Show>
          </div>
        </div>
      </Show>

      {/* ───────── Usage view — M3 ───────── */}
      <Show when={view() === "usage"}>
        <div class="usage-view">
          <div class="uv-top">
            <div class="uv-metric">
              <div class="uv-label">Spent today</div>
              <div class="uv-big">{money(usage()?.today_usd ?? 0)}</div>
            </div>
            <div class="uv-metric">
              <div class="uv-label">Burn rate</div>
              <div class="uv-big">{money(usage()?.burn_per_hr ?? 0)}<span class="uv-unit">/hr</span></div>
            </div>
            <div class="uv-metric uv-budget">
              <div class="uv-label">Daily budget · {money(usage()?.budget_usd ?? 0)}</div>
              <div class="uv-bar">
                <div class="uv-bar-fill" style={{
                  width: `${usage()?.budget_pct ?? 0}%`,
                  background: (usage()?.budget_pct ?? 0) >= 100 ? "#e07a6e" : (usage()?.budget_pct ?? 0) >= 80 ? "#e0b15e" : "#5fb98a",
                }} />
              </div>
              <div class="uv-pct">{usage()?.budget_pct ?? 0}% used</div>
            </div>
          </div>
          <div class="uv-cols">
            <div class="uv-panel">
              <div class="uv-panel-h">By project</div>
              <For each={usage()?.by_project ?? []}>{(s) => (
                <div class="uv-row">
                  <span class="uv-row-name">{s.name}</span>
                  <div class="uv-row-bar"><div class="uv-row-fill" style={{ width: `${pctOf(s.usd, usage()?.by_project)}%` }} /></div>
                  <span class="uv-row-usd">{money(s.usd)}</span>
                </div>
              )}</For>
            </div>
            <div class="uv-panel">
              <div class="uv-panel-h">By model</div>
              <For each={usage()?.by_model ?? []}>{(s) => (
                <div class="uv-row">
                  <span class="uv-row-name">{s.name}</span>
                  <div class="uv-row-bar"><div class="uv-row-fill" style={{ width: `${pctOf(s.usd, usage()?.by_model)}%` }} /></div>
                  <span class="uv-row-usd">{money(s.usd)}</span>
                </div>
              )}</For>
            </div>
          </div>
        </div>
      </Show>

      {/* ───────── Review & Orchestration (Grid) — M4 ───────── */}
      <Show when={view() === "review"}>
        <div class="review">
          <div class="grid4">
            <For each={(data()?.sessions ?? []).slice(0, 4)}>{(s) => (
              <div class="pane">
                <div class="pane-h">
                  <span class="dot" style={{ background: color(s.status) }} />
                  <span class="pane-title">{s.project} / {s.name}</span>
                  <span class="pane-status" style={{ color: color(s.status) }}>{s.status_label}</span>
                </div>
                <div class="pane-body">{s.action || s.branch}</div>
              </div>
            )}</For>
          </div>
          <div class="review-cols">
            <div class="inbox">
              <div class="panel-h">Review Inbox <span class="badge">{inbox()?.length ?? 0}</span></div>
              <div class="scroll">
                <For each={inbox() ?? []}>{(it) => (
                  <div class="inbox-row" classList={{ sel: selected() === it.session }} onClick={() => setSelected(it.session)}>
                    <div class="inbox-main">
                      <div class="inbox-proj">{it.project}</div>
                      <div class="inbox-branch">{it.branch}</div>
                    </div>
                    <div class="inbox-stat">
                      <span class="files">{it.files} files</span>
                      <span class="add">+{it.added}</span> <span class="rem">−{it.removed}</span>
                    </div>
                  </div>
                )}</For>
              </div>
            </div>
            <div class="diffp">
              <div class="panel-h">Diff <span class="hint">{selected()}</span></div>
              <div class="scroll">
                <For each={diff() ?? []} fallback={<div class="empty">Select a session.</div>}>{(f) => (
                  <div class="file-row">
                    <span class="fbadge" classList={{ added: f.status === "A", mod: f.status === "M", del: f.status === "D" }}>{f.status}</span>
                    <span class="fpath">{f.path}</span>
                    <span class="fstat">
                      <Show when={f.added !== null} fallback={<span class="bin">binary</span>}>
                        <span class="add">+{f.added}</span> <span class="rem">−{f.removed}</span>
                      </Show>
                    </span>
                  </div>
                )}</For>
              </div>
            </div>
          </div>
        </div>
      </Show>

      {/* ───────── Settings + Extensions (One System, One Team) — M5 ───────── */}
      <Show when={view() === "settings"}>
        <div class="settings">
          <div class="set-cols">
            <div class="set-panel">
              <div class="panel-h">Settings <span class="muted">effective config (global → project → session)</span></div>
              <div class="scroll">
                <For each={settings()?.config ?? []}>{(c) => (
                  <div class="cfg-row">
                    <span class="cfg-key">{c.key}</span>
                    <span class="cfg-val" classList={{ secret: c.secret }}>{c.value}</span>
                  </div>
                )}</For>
              </div>
            </div>
            <div class="set-panel">
              <div class="panel-h">Extensions <span class="muted">MCP · skills · commands</span></div>
              <div class="scroll">
                <div class="ext-h">MCP servers</div>
                <For each={settings()?.mcp_servers ?? []}>{(s) => (
                  <div class="ext-row">
                    <span class="ext-name">{s.name}</span>
                    <span class="ext-tools">{s.tools} tools</span>
                    <span class="sw" classList={{ on: s.enabled }}><span class="knob" /></span>
                  </div>
                )}</For>
                <div class="ext-h">Skills</div>
                <For each={settings()?.skills ?? []}>{(s) => (
                  <div class="ext-row">
                    <span class="ext-name">{s.name}</span>
                    <span class="spacer" />
                    <span class="sw" classList={{ on: s.enabled }}><span class="knob" /></span>
                  </div>
                )}</For>
                <div class="ext-h">Commands</div>
                <div class="cmd-grid">
                  <For each={settings()?.commands ?? []}>{(c) => <span class="cmd">{c}</span>}</For>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default App;
