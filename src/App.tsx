import { RouteConfig, createBrowserRouter } from "found";
import { relaunch } from "@tauri-apps/api/process";
import { useState } from "react";
import AboutPage from "./AboutPage";
import NewPositionPage from "./NewPositionPage";
import IntroPage from "./IntroPage";
import ManagePositionsPage from "./ManagePositionsPage";
import { Button } from "./generic/button";
import { connectToDesk, resetDesk } from "./rustUtils";
import Spinner from "./generic/Spinner";

const ErrorPage = () => {
  const [isLoading, setLoading] = useState(false);

  if (isLoading) {
    return (
      <div>
        <Spinner size="md" />
      </div>
    );
  }

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
          onClick={async () => {
            setLoading(true);
            await connectToDesk((window as any)?.stateWorkaround?.desk_name);
            setLoading(false);
          }}
        >
          Try again
        </Button>
      </div>
      <div>or</div>
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
