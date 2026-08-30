import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from '@tailwindcss/vite'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [solid(), tailwindcss()],

  // Matches package.json's `browserslist` (the oldest confirmed-live target:
  // Chromium 109 on an Android API 33 emulator's system WebView) so esbuild
  // doesn't emit JS syntax those engines can't run.
  build: {
    target: ["chrome109", "edge109", "safari15", "ios15", "firefox100"],
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
