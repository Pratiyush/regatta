import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

// Vite + Solid, tuned for Tauri (fixed dev port, no screen-clearing).
export default defineConfig({
  plugins: [solid()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  build: { target: "es2022", outDir: "dist", emptyOutDir: true },
});
