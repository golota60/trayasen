import { useEffect, useState } from "react";
import { relaunch } from "@tauri-apps/plugin-process";
import useSimpleAsync from "use-simple-async";
import { enable, isEnabled, disable } from "@tauri-apps/plugin-autostart";
import { HelpCircle } from "lucide-react";
import { removeConfig } from "./rustUtils";
import { Button } from "./generic/button";
import { Checkbox } from "./generic/checkbox";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "./generic/tooltip";
import Href from "./generic/Href";

const AboutPage = () => {
  const [isAutostartEnabled, setAutostartEnabled] = useState<
    boolean | undefined
  >();
  const [isUpdatingAutostart, setUpdatingAutostart] = useState(false);
  const [isResetting, setResetting] = useState(false);
  const [actionError, setActionError] = useState<string>();

  const [upstreamAutostart, { error: autostartReadError }] =
    useSimpleAsync(isEnabled);

  useEffect(() => {
    if (isAutostartEnabled === undefined) {
      setAutostartEnabled(upstreamAutostart);
    }
  }, [isAutostartEnabled, upstreamAutostart]);

  useEffect(() => {
    if (autostartReadError) {
      console.error("Could not read autostart state", autostartReadError);
      setActionError(
        `Could not read autostart state: ${String(autostartReadError)}`
      );
    }
  }, [autostartReadError]);

  return (
    <>
      <div className="flex-col flex justify-center items-center">
        <img src="/carrot.png" alt="A carrot logo" />
        <h1
          className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
 mt-2 mb-5"
        >
          Options
        </h1>
        <div className="items-top flex space-x-2 mb-3">
          <Checkbox
            onCheckedChange={async () => {
              setUpdatingAutostart(true);
              setActionError(undefined);
              try {
                if (isAutostartEnabled) {
                  await disable();
                  setAutostartEnabled(false);
                } else {
                  await enable();
                  setAutostartEnabled(true);
                }
              } catch (error) {
                console.error("Could not update autostart state", error);
                setActionError(`Could not update autostart: ${String(error)}`);
              } finally {
                setUpdatingAutostart(false);
              }
            }}
            checked={isAutostartEnabled || false}
            disabled={isUpdatingAutostart}
            id="autostart-toggle"
          />
          <div className="grid gap-1.5 leading-none">
            <label
              className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
              htmlFor="autostart-toggle"
            >
              Open on system startup
            </label>
            <p className="text-sm text-muted-foreground">
              Once checked, the app will attempt to start up with the system
            </p>
          </div>
        </div>

        <div className="flex-col flex justify-center items-center">
          <h1
            className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
 mt-2 mb-5"
          >
            Advanced Options
          </h1>
          <div className="flex justify-between mb-3">
            <Button
              className="mr-2"
              disabled={isResetting}
              onClick={async () => {
                setResetting(true);
                setActionError(undefined);
                try {
                  await removeConfig();
                  await relaunch();
                } catch (error) {
                  console.error("Could not reset and relaunch Trayasen", error);
                  setActionError(`Could not reset config: ${String(error)}`);
                  setResetting(false);
                }
              }}
            >
              {isResetting ? "Resetting..." : "Reset config & restart the app"}
            </Button>
            <TooltipProvider>
              <Tooltip delayDuration={100}>
                <TooltipTrigger>
                  <HelpCircle className="text-muted-foreground" />
                </TooltipTrigger>
                <TooltipContent side="bottom">
                  <p>Returns your app to default settings</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
          {actionError ? (
            <div className="mb-3 text-red-500">{actionError}</div>
          ) : null}
        </div>
        <h1
          className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
 mt-2 mb-3"
        >
          About
        </h1>
        <p>
          This lovely little app has been created by Szymon Wiszczuk(
          <Href
            target="_blank"
            href="https://github.com/golota60"
            rel="noreferrer"
          >
            github
          </Href>
          ,
          <Href target="_blank" href="https://szymon.codes" rel="noreferrer">
            my website
          </Href>
          )
        </p>
        <p>
          If something doesn't work for you - please file an issue under{" "}
          <Href
            target="_blank"
            href="https://github.com/golota60/trayasen/issues/new"
            rel="noreferrer"
          >
            this link
          </Href>
          .
        </p>
      </div>
    </>
  );
};

export default AboutPage;
