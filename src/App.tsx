import { RouteConfig, createBrowserRouter } from "found";
import { relaunch } from "@tauri-apps/plugin-process";
import { useState } from "react";
import AboutPage from "./AboutPage";
import NewPositionPage from "./NewPositionPage";
import IntroPage from "./IntroPage";
import ManagePositionsPage from "./ManagePositionsPage";
import { Button } from "./generic/button";
import { connectToDesk, resetDesk } from "./rustUtils";
import Spinner from "./generic/Spinner";

// This error will only happen for users with a desk already set up. Intro Page errors are be handled in Intro Page.
const ReturningUserErrorPage = () => {
  const [isLoading, setLoading] = useState(false);
  const [error, setError] = useState<string>("");

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
            try {
              await connectToDesk((window as any)?.stateWorkaround?.desk_name);
            } catch (e) {
              setError(e as string);
            }
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

      <div>Error content:</div>
      <div>{(window as any)?.stateWorkaround?.error || error}</div>
    </div>
  );
};

const routeConfig: RouteConfig = [
  { path: "/error", Component: ReturningUserErrorPage },
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
