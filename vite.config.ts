import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import {
  getBrowserProcessEnvironment,
  resolveViteNodeEnvironment,
} from "./src/viteBrowserEnvironment.js";

// https://vitejs.dev/config/
export default defineConfig(({ command }) => ({
  plugins: [react()],

  // Tamagui reads process.env in browser modules. Replace it at build time so
  // the WebView never needs a Node.js `process` global.
  define: {
    "process.env": getBrowserProcessEnvironment(
      resolveViteNodeEnvironment(command, process.env.NODE_ENV)
    ),
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "oxc" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
}));
