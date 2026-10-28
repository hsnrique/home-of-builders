import { defineConfig } from "vite";

// Local servers stay on the loopback interface.
export default defineConfig({
  server: { host: "127.0.0.1", port: 4173, strictPort: true },
  preview: { host: "127.0.0.1", port: 4174, strictPort: true },
});
