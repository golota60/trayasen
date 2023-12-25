import React from "react";
import ReactDOM from "react-dom/client";
import { appWindow } from "@tauri-apps/api/window";
import App from "./App";
import "./style.css";
import { hasCustomDecorations } from "./rustUtils";

const customDecorations = await hasCustomDecorations();

if (customDecorations) {
  document
    ?.getElementById("titlebar-minimize")
    ?.addEventListener("click", () => appWindow.minimize());
  document
    ?.getElementById("titlebar-maximize")
    ?.addEventListener("click", () => appWindow.toggleMaximize());
  document
    ?.getElementById("titlebar-close")
    ?.addEventListener("click", () => appWindow.close());
} else {
  // if window doesn't have custom decorations, remove the titlebar altogether
  document?.querySelector(".titlebar")?.remove();
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
