import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import App from "./App";
import "./style.css";
import { hasCustomDecorations } from "./rustUtils";

(async () => {
  const customDecorations = await hasCustomDecorations();

  if (customDecorations) {
    document
      ?.getElementById("titlebar-minimize")
      ?.addEventListener("click", () => getCurrentWindow().minimize());
    document
      ?.getElementById("titlebar-maximize")
      ?.addEventListener("click", () => getCurrentWindow().toggleMaximize());
    document
      ?.getElementById("titlebar-close")
      ?.addEventListener("click", () => getCurrentWindow().close());
  } else {
    // if window doesn't have custom decorations, remove the titlebar altogether
    document?.querySelector(".titlebar")?.remove();
  }
})();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
