import type { Component } from "solid-js";

// M0 shell only — the real cockpit (sidebar + Attention Dock + Focus/Grid) lands in M1.
const App: Component = () => {
  return (
    <main class="shell">
      <h1>Regatta</h1>
      <p>The cockpit shell — UI lands in M1.</p>
    </main>
  );
};

export default App;
