import { defineConfig } from "vite";

// Tauri expects a fixed port and loads the UI from the loopback interface. Mobile dev
// (`tauri android|ios dev`) passes TAURI_DEV_HOST so the device can reach the server.
// @ts-expect-error process is a Node global; this config is not part of the browser build.
const mobileHost: string | undefined = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: mobileHost ?? "127.0.0.1",
    hmr: mobileHost ? { protocol: "ws", host: mobileHost, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
});
