import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import AboutPage from "./AboutPage";
import NewPositionPage from "./NewPositionPage";
import IntroPage from "./IntroPage";
import SetupPage from "./SetupPage";
import ManagePositionsPage from "./ManagePositionsPage";

const PageContent = () => {
  const path = window.location.pathname;

  // Do not rely on router for simplicity
  if (path === "/about") {
    // return <SetupPage />;
    return <AboutPage />;
  }

  if (path === "/new-position") {
    return <NewPositionPage />;
  }

  if (path === "/manage-positions") {
    return <ManagePositionsPage />;
  }

  if (path === "/intro") {
    return <IntroPage />;
  }

  // Return intro page as a fallback; this should never happen but yeah fuck me if it does
  return <IntroPage />;
};

function App() {
  return (
    <div className="flex-col bg-slate-800 h-full flex justify-center items-center font-sans text-slate-100">
      <PageContent />
    </div>
  );
}

export default App;
