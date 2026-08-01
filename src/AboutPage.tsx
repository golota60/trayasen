import { useEffect, useState, type ReactNode } from "react";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { relaunch } from "@tauri-apps/plugin-process";
import { Info, RotateCcw, Settings } from "lucide-react";
import { Label, Switch, Text, XStack, YStack } from "tamagui";
import useSimpleAsync from "use-simple-async";
import { removeConfig } from "./rustUtils";
import { AppButton } from "./ui/Button";
import { SurfaceCard } from "./ui/Card";
import { ExternalLink } from "./ui/ExternalLink";
import { Alert } from "./ui/Feedback";
import { PageShell } from "./ui/PageShell";

interface SectionHeadingProps {
  icon: ReactNode;
  title: string;
  description: string;
}

const SectionHeading = ({ icon, title, description }: SectionHeadingProps) => (
  <XStack alignItems="flex-start" gap="$3">
    {icon}
    <YStack flex={1} gap="$1">
      <Text color="$color" fontSize="$6" fontWeight="700">
        {title}
      </Text>
      <Text color="$muted" fontSize="$4">
        {description}
      </Text>
    </YStack>
  </XStack>
);

const AboutPage = () => {
  const [isAutostartEnabled, setAutostartEnabled] = useState<boolean>();
  const [isUpdatingAutostart, setUpdatingAutostart] = useState(false);
  const [isResetting, setResetting] = useState(false);
  const [autostartError, setAutostartError] = useState<string>();
  const [resetError, setResetError] = useState<string>();

  const [upstreamAutostart, { error: autostartReadError, loading }] =
    useSimpleAsync(isEnabled);

  useEffect(() => {
    if (
      isAutostartEnabled === undefined &&
      typeof upstreamAutostart === "boolean"
    ) {
      setAutostartEnabled(upstreamAutostart);
    }
  }, [isAutostartEnabled, upstreamAutostart]);

  useEffect(() => {
    if (autostartReadError) {
      console.error("Could not read autostart state", autostartReadError);
      setAutostartError(
        `Could not read autostart state: ${String(autostartReadError)}`
      );
    }
  }, [autostartReadError]);

  const updateAutostart = async (checked: boolean) => {
    setUpdatingAutostart(true);
    setAutostartError(undefined);
    try {
      if (checked) {
        await enable();
      } else {
        await disable();
      }
      setAutostartEnabled(checked);
    } catch (error) {
      console.error("Could not update autostart state", error);
      setAutostartError(`Could not update autostart: ${String(error)}`);
    } finally {
      setUpdatingAutostart(false);
    }
  };

  const resetConfig = async () => {
    setResetting(true);
    setResetError(undefined);
    try {
      await removeConfig();
      await relaunch();
    } catch (error) {
      console.error("Could not reset and relaunch Trayasen", error);
      setResetError(`Could not reset config: ${String(error)}`);
    } finally {
      setResetting(false);
    }
  };

  const autostartUnavailable =
    loading || isAutostartEnabled === undefined || isUpdatingAutostart;

  return (
    <PageShell
      title="Options & About"
      description="Manage how Trayasen starts, reset local settings, or get support."
    >
      <SurfaceCard>
        <SectionHeading
          icon={<Settings aria-hidden color="#F28C28" size={22} />}
          title="Startup"
          description="Choose whether Trayasen opens automatically when you sign in."
        />
        <XStack
          alignItems="center"
          gap="$4"
          justifyContent="space-between"
          $sm={{ alignItems: "flex-start", flexDirection: "column" }}
        >
          <YStack flex={1} gap="$1">
            <Label
              color="$color"
              fontSize="$4"
              fontWeight="600"
              htmlFor="autostart-toggle"
            >
              Open Trayasen when the system starts
            </Label>
            <Text color="$muted" fontSize="$3">
              Trayasen will stay available from the system tray after login.
            </Text>
          </YStack>
          <Switch
            aria-label="Open Trayasen when the system starts"
            checked={Boolean(isAutostartEnabled)}
            disabled={autostartUnavailable}
            id="autostart-toggle"
            onCheckedChange={updateAutostart}
            size="$4"
          >
            <Switch.Thumb />
          </Switch>
        </XStack>
        {autostartError ? (
          <Alert tone="error" title="Autostart unavailable">
            {autostartError}
          </Alert>
        ) : null}
      </SurfaceCard>

      <SurfaceCard>
        <SectionHeading
          icon={<RotateCcw aria-hidden color="#F28C28" size={22} />}
          title="Advanced"
          description="Remove saved settings and restart Trayasen in its initial setup state."
        />
        <Text color="$muted" fontSize="$3">
          This clears your saved desk and positions. You will need to connect
          and configure them again after restart.
        </Text>
        <XStack>
          <AppButton
            loading={isResetting}
            loadingLabel="Resetting"
            onPress={resetConfig}
            variant="destructive"
          >
            Reset config and restart
          </AppButton>
        </XStack>
        {resetError ? (
          <Alert tone="error" title="Reset failed">
            {resetError}
          </Alert>
        ) : null}
      </SurfaceCard>

      <SurfaceCard>
        <SectionHeading
          icon={<Info aria-hidden color="#F28C28" size={22} />}
          title="About Trayasen"
          description="A small desktop companion for controlling an IKEA Idasen desk."
        />
        <Text color="$color" fontSize="$4">
          Trayasen is created by Szymon Wiszczuk. View the project on{" "}
          <ExternalLink href="https://github.com/golota60/trayasen">
            GitHub
          </ExternalLink>{" "}
          or visit{" "}
          <ExternalLink href="https://szymon.codes">
            Szymon&apos;s website
          </ExternalLink>
          .
        </Text>
        <Text color="$color" fontSize="$4">
          Found a problem or have a suggestion?{" "}
          <ExternalLink href="https://github.com/golota60/trayasen/issues/new">
            Create an issue
          </ExternalLink>
          .
        </Text>
      </SurfaceCard>
    </PageShell>
  );
};

export default AboutPage;
