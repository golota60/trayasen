import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import App from "./App";
import "./style.css";
import { hasCustomDecorations } from "./rustUtils";

const appWindow = getCurrentWindow();

(async () => {
  const customDecorations = await hasCustomDecorations();

  if (customDecorations) {
    document.documentElement.dataset.customDecorations = "true";
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
    document.documentElement.dataset.customDecorations = "false";
    // if window doesn't have custom decorations, remove the titlebar altogether
    document?.querySelector(".titlebar")?.remove();
  }
})();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <App />
    </TamaguiProvider>
  </React.StrictMode>
);
