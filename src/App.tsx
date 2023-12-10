import { RouteConfig, createBrowserRouter } from "found";
import { relaunch } from "@tauri-apps/api/process";
import AboutPage from "./AboutPage";
import NewPositionPage from "./NewPositionPage";
import IntroPage from "./IntroPage";
import ManagePositionsPage from "./ManagePositionsPage";
import { Button } from "./generic/button";
import { resetDesk } from "./rustUtils";

const ErrorPage = () => {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <div>{(window as any)?.stateWorkaround?.title}</div>
      <div>{(window as any)?.stateWorkaround?.description}</div>
      <div>
        <Button
          onClick={() => {
            resetDesk();
            relaunch();
          }}
        >
          Reset app and desk name & open the connect intro menu
        </Button>
      </div>
    </div>
  );
};

const routeConfig: RouteConfig = [
  { path: "/error", Component: ErrorPage },
  { path: "/about", Component: AboutPage },
  { path: "/new-position", Component: NewPositionPage },
  { path: "/manage-positions", Component: ManagePositionsPage },
  { path: "/intro", Component: IntroPage },
  { path: "/*", Component: IntroPage },
];

const BrowserRouter = createBrowserRouter({ routeConfig });

function App() {
  return (
    <div className="flex-col h-full flex justify-center items-center font-sans bg-background">
      <BrowserRouter />
    </div>
  );
}

export default App;
