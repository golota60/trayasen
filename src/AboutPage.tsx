import clsx from "clsx";
import { useEffect, useState } from "react";
import { relaunch } from "@tauri-apps/api/process";
import useSimpleAsync from "use-simple-async";
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import Button from "./generic/Button";
import { removeConfig } from "./rustUtils";

const AboutPage = () => {
  const [prompt] = useState<string | undefined>();
  const [isAutostartEnabled, setAutostartEnabled] = useState<
    boolean | undefined
  >();

  const [upstreamAutostart] = useSimpleAsync(isEnabled);

  useEffect(() => {
    if (isAutostartEnabled === undefined) {
      setAutostartEnabled(upstreamAutostart);
    }
  }, [isAutostartEnabled, upstreamAutostart]);

  return (
    <>
      <div className="flex-col flex justify-center items-center">
        <img src="/carrot.png" alt="A carrot logo" />
        <h1 className="text-4xl mt-2 mb-3">Options</h1>
        <Button
          onClick={() => {
            removeConfig();
            relaunch();
            // setPrompt(
            //   "Config removed! Restart the app for changes to take place"
            // );
          }}
        >
          Reset config & restart the app
        </Button>
        <div className="flex items-center mt-3">
          <input
            className="w-5 h-5 mr-2 ring-black checked:bg-blue-500"
            type="checkbox"
            name="autostart-toggle"
            checked={isAutostartEnabled || false}
            value={isAutostartEnabled as any}
            onChange={() => {
              if (isAutostartEnabled) {
                disable();
                setAutostartEnabled(false);
              } else {
                enable();
                setAutostartEnabled(true);
              }
            }}
          />
          <label htmlFor="autostart-toggle">Open on system startup</label>
        </div>
        <p className={clsx("h-12 my-2", prompt ? "visible" : "invisible")}>
          {prompt}
        </p>
        <img src="/carrot.png" alt="A carrot logo" />
        <h1 className="text-4xl mt-2 mb-3">About</h1>
        <p>
          This lovely little app has been created by Szymon Wiszczuk(
          <a
            target="_blank"
            href="https://github.com/golota60"
            rel="noreferrer"
          >
            github
          </a>
          ,
          <a
            target="_blank"
            href="https://twitter.com/SzymonWiszczuk"
            rel="noreferrer"
          >
            twitter
          </a>
          )
        </p>
        <p>
          If something doesn't work for you - please file an issue under{" "}
          <a
            target="_blank"
            href="https://github.com/golota60/trayasen/issues/new"
            rel="noreferrer"
          >
            this link
          </a>
          .
        </p>
      </div>
    </>
  );
};

export default AboutPage;
