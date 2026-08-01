import { relaunch } from "@tauri-apps/plugin-process";
import { RefreshCw } from "lucide-react";
import { useState } from "react";
import { Text, XStack, YStack } from "tamagui";
import useSimpleAsync from "use-simple-async";
import DeskElement from "./DeskElement";
import { filterDevices } from "./features/devices/deviceFilters";
import { getAvailableDesks, removeConfig } from "./rustUtils";
import { AppButton } from "./ui/Button";
import { SurfaceCard } from "./ui/Card";
import { Alert, CarrotSpinner, StatePanel } from "./ui/Feedback";
import { LinkButton } from "./ui/LinkButton";
import { PageShell } from "./ui/PageShell";

const IntroPage = () => {
  const [data, { error, loading: devicesLoading, retry }] = useSimpleAsync(
    getAvailableDesks,
    {
      useLayout: true,
    }
  );
  const [connectingLoading, setConnectingLoading] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [connectedNewDesk, setConnectedNewDesk] = useState<string>();
  const [showAll, setShowAll] = useState(false);
  const [deskError, setDeskError] = useState<string>();
  const [resetError, setResetError] = useState<string>();
  const [isResetting, setResetting] = useState(false);

  const actualError = error || deskError;

  const resetAndRestart = async () => {
    setResetting(true);
    setResetError(undefined);
    try {
      await removeConfig();
      await relaunch();
    } catch (actionError) {
      console.error("Could not reset and relaunch Trayasen", actionError);
      setResetError(String(actionError));
      setResetting(false);
    }
  };

  if (actualError) {
    return (
      <PageShell title="Connection recovery">
        <SurfaceCard>
          <Alert tone="error" title="Something went wrong">
            Trayasen could not discover or connect to a desk.
          </Alert>
          <YStack alignItems="flex-start" gap="$3">
            <AppButton
              loading={isResetting}
              loadingLabel="Resetting"
              onPress={resetAndRestart}
              variant="destructive"
            >
              Reset config and restart the app
            </AppButton>
          </YStack>
          {resetError ? (
            <Alert tone="error" title="Reset failed">
              {resetError}
            </Alert>
          ) : null}
          <details>
            <summary>Error details</summary>
            <pre>{String(actualError)}</pre>
          </details>
        </SurfaceCard>
      </PageShell>
    );
  }

  const devices = filterDevices(data, showAll);
  const controlsDisabled = devicesLoading || connectingLoading || isConnected;

  return (
    <PageShell
      description="Find a nearby IKEA Idasen desk and connect it to Trayasen."
      title="Connect your desk"
    >
      <XStack alignItems="stretch" gap="$5" $sm={{ flexDirection: "column" }}>
        <YStack flex={0.7} gap="$3" justifyContent="center">
          <Text color="$color" fontSize="$7" fontWeight="700">
            Bluetooth setup
          </Text>
          <Text color="$muted" lineHeight="$5">
            Trayasen lets you control your desk from the system tray. Choose a
            nearby desk to get started.
          </Text>
        </YStack>

        <SurfaceCard flex={1.3}>
          <YStack gap="$1">
            <Text color="$color" fontSize="$7" fontWeight="700">
              Nearby devices
            </Text>
            <Text color="$muted">Select the desk you want to use.</Text>
          </YStack>

          {devicesLoading ? (
            <StatePanel
              description="This can take a few seconds."
              icon={
                <CarrotSpinner
                  aria-label="Searching for Bluetooth devices"
                  size="md"
                />
              }
              title="Searching for Bluetooth devices"
            />
          ) : devices.length === 0 ? (
            <StatePanel
              description="Refresh the scan or show all Bluetooth devices."
              title="No matching desks found"
            />
          ) : (
            <YStack gap="$3">
              {devices.map((device) => (
                <DeskElement
                  key={device.name}
                  deskName={device.name}
                  disabled={connectingLoading || isConnected}
                  isConnected={device.name === connectedNewDesk}
                  onConnect={() => {
                    setIsConnected(true);
                    setConnectedNewDesk(device.name);
                  }}
                  onError={setDeskError}
                  onLoadEnd={() => setConnectingLoading(false)}
                  onLoadStart={() => setConnectingLoading(true)}
                />
              ))}
            </YStack>
          )}

          <Text color="$muted" fontSize="$3">
            If your desk has a different name from &quot;Desk XXXX&quot;, show
            all devices to find it by its alternate name.
          </Text>

          <XStack flexWrap="wrap" gap="$3" $sm={{ flexDirection: "column" }}>
            <AppButton
              disabled={controlsDisabled}
              onPress={retry}
              variant="secondary"
            >
              <XStack alignItems="center" gap="$2">
                <RefreshCw aria-hidden size={16} />
                <Text color="inherit">Refresh</Text>
              </XStack>
            </AppButton>
            <AppButton
              disabled={controlsDisabled}
              onPress={() => setShowAll((current) => !current)}
              variant="secondary"
            >
              {showAll ? "Show desks only" : "Show all devices"}
            </AppButton>
          </XStack>

          {isConnected ? (
            <YStack alignItems="flex-start" gap="$3">
              <Alert tone="success" title="Desk connected">
                You can now save your first standing or sitting position.
              </Alert>
              <LinkButton to="/new-position">Add first position</LinkButton>
            </YStack>
          ) : (
            <Text color="$muted">
              Connect to a desk before adding your first position.
            </Text>
          )}
        </SurfaceCard>
      </XStack>
    </PageShell>
  );
};

export default IntroPage;
