import { RouteConfig, createBrowserRouter } from "found";
import { relaunch } from "@tauri-apps/plugin-process";
import { useState } from "react";
import AboutPage from "./AboutPage";
import NewPositionPage from "./NewPositionPage";
import IntroPage from "./IntroPage";
import ManagePositionsPage from "./ManagePositionsPage";
import { Button } from "./generic/button";
import { connectToDesk, removeConfig, resetDesk } from "./rustUtils";
import Spinner from "./generic/Spinner";

// This error will only happen for users with a desk already set up. Intro Page errors are be handled in Intro Page.
const ReturningUserErrorPage = () => {
  const [isLoading, setLoading] = useState(false);
  const [isResetting, setResetting] = useState(false);
  const [error, setError] = useState<string>("");
  const [resetError, setResetError] = useState<string>("");
  const errorState = (window as any)?.stateWorkaround;
  const canRetryDesk = Boolean(errorState?.desk_name);

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
      <div>{errorState?.title}</div>
      <div>{errorState?.description}</div>
      {canRetryDesk ? (
        <>
          <div>
            <Button
              onClick={async () => {
                setLoading(true);
                setError("");
                try {
                  await connectToDesk(errorState.desk_name);
                  await relaunch();
                } catch (retryError) {
                  setError(String(retryError));
                  setLoading(false);
                }
              }}
            >
              Try again
            </Button>
          </div>
          <div>or</div>
        </>
      ) : null}
      <div>
        <Button
          disabled={isResetting}
          onClick={async () => {
            setResetting(true);
            setResetError("");
            try {
              if (canRetryDesk) {
                await resetDesk();
              } else {
                await removeConfig();
              }
              await relaunch();
            } catch (actionError) {
              console.error(
                "Could not reset and relaunch Trayasen",
                actionError
              );
              setResetError(String(actionError));
              setResetting(false);
            }
          }}
        >
          {isResetting
            ? "Resetting..."
            : canRetryDesk
            ? "Reset app and desk name & open the connect intro menu"
            : "Reset config & restart the app"}
        </Button>
      </div>

      <div>Error content:</div>
      <div>{errorState?.error}</div>
      {error ? (
        <div className="text-red-500">Action failed: {error}</div>
      ) : null}
      {resetError ? (
        <div className="text-red-500">Reset failed: {resetError}</div>
      ) : null}
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
