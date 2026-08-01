import { relaunch } from "@tauri-apps/plugin-process";
import { createBrowserRouter } from "found";
import type { RouteConfig } from "found";
import { useState } from "react";
import { YStack } from "tamagui";
import AboutPage from "./AboutPage";
import IntroPage from "./IntroPage";
import ManagePositionsPage from "./ManagePositionsPage";
import NewPositionPage from "./NewPositionPage";
import { connectToDesk, removeConfig, resetDesk } from "./rustUtils";
import { AppButton } from "./ui/Button";
import { SurfaceCard } from "./ui/Card";
import { Alert, CarrotSpinner } from "./ui/Feedback";
import { PageShell } from "./ui/PageShell";

interface ReturningUserErrorState {
  title?: string;
  description?: string;
  error?: unknown;
  desk_name?: string;
}

// This error only happens for users with an existing configuration. Intro errors are handled by IntroPage.
const ReturningUserErrorPage = () => {
  const [isLoading, setLoading] = useState(false);
  const [isResetting, setResetting] = useState(false);
  const [error, setError] = useState("");
  const [resetError, setResetError] = useState("");
  const errorState = (
    window as Window & { stateWorkaround?: ReturningUserErrorState }
  ).stateWorkaround;
  const canRetryDesk = Boolean(errorState?.desk_name);

  if (isLoading) {
    return (
      <PageShell title="Reconnecting to desk">
        <YStack
          alignItems="center"
          flex={1}
          justifyContent="center"
          minHeight={320}
        >
          <CarrotSpinner aria-label="Reconnecting to saved desk" size="lg" />
        </YStack>
      </PageShell>
    );
  }

  const feedback = [
    errorState?.description,
    error ? `Action failed: ${error}` : "",
    resetError ? `Reset failed: ${resetError}` : "",
  ]
    .filter(Boolean)
    .join(" ");

  const retry = async () => {
    setLoading(true);
    setError("");

    try {
      await connectToDesk(errorState?.desk_name as string);
      await relaunch();
    } catch (retryError) {
      setError(String(retryError));
      setLoading(false);
    }
  };

  const reset = async () => {
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
      console.error("Could not reset and relaunch Trayasen", actionError);
      setResetError(String(actionError));
      setResetting(false);
    }
  };

  return (
    <PageShell title="Connection recovery">
      <SurfaceCard>
        <Alert
          tone="error"
          title={errorState?.title ?? "Trayasen needs attention"}
        >
          {feedback}
        </Alert>

        <YStack alignItems="flex-start" gap="$3">
          {canRetryDesk ? (
            <AppButton onPress={retry}>Try again</AppButton>
          ) : null}
          <AppButton
            loading={isResetting}
            loadingLabel="Resetting"
            onPress={reset}
            variant="destructive"
          >
            {canRetryDesk
              ? "Forget desk and restart setup"
              : "Reset config and restart"}
          </AppButton>
        </YStack>

        <details>
          <summary>Technical details</summary>
          <pre>{String(errorState?.error ?? "")}</pre>
        </details>
      </SurfaceCard>
    </PageShell>
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
  return <BrowserRouter />;
}

export default App;
