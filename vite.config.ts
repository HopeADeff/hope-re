import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

// @ts-expect-error process is a nodejs global
// eslint-disable-next-line node/no-process-env
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    sveltekit(),
    tailwindcss(),
  ],

  clearScreen: false,

  server: {
    port: 3000,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 3000,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
