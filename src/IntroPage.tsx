import { relaunch } from "@tauri-apps/api/process";
import { Link } from "found";
import { HelpCircle } from "lucide-react";
import { useEffect, useState } from "react";
import useSimpleAsync from "use-simple-async";
import {
  TooltipProvider,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "./generic/tooltip";
import DeskElement from "./DeskElement";
import { Button } from "./generic/button";
import Spinner from "./generic/Spinner";
import { connectToDesk, getConnectionDesk, removeConfig } from "./rustUtils";

const IntroPage = () => {
  const [data, { error, loading }] = useSimpleAsync(getConnectionDesk, {
    useLayout: true,
  });
  const [isConnecting, setIsConnecting] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [connectedNewDesk, setConnectedNewDesk] = useState<string>();
  const [showAll, setShowAll] = useState(false);

  // workaround for checking whether the desk is saved. TODO: make this nicer
  // If it's saved, autoconnect
  const isSaved = data?.[0].status === "saved";

  console.log(data);

  useEffect(() => {
    if (!isConnecting && isSaved && !isConnected) {
      setIsConnecting(true);
      connectToDesk(data?.[0].name);
      setIsConnecting(false);
      setIsConnected(true);
    }
  }, [isSaved, data, isConnecting, isConnected]);

  if (loading) {
    return (
      <div>
        <Spinner size="lg" />
        Loading...
      </div>
    );
  }

  if (error) {
    return (
      <div>
        Something went wrong.{" "}
        <div>
          <Button
            onClick={() => {
              removeConfig();
              relaunch();
            }}
          >
            Reset config & restart the app
          </Button>
        </div>
        <div>
          Error contents: <p>{String(error as any)}</p>
        </div>
      </div>
    );
  }

  const dataToDisplay = showAll
    ? data
    : data?.filter((e) => e.name.includes("Desk"));

  return (
    <div className="w-full h-full flex flex-col justify-center items-center">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1
        className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
 mt-2 mb-3"
      >
        Welcome to Trayasen!
      </h1>
      <p>
        This app will help you to interact with your IKEA Idasen Desk from the
        system tray.
      </p>
      <div className="flex flex-col justify-center items-center my-4">
        {isSaved ? (
          <>
            <p>Connecting to saved desk...</p>
            {isConnecting ? <Spinner size="sm" /> : "Connected!"}
          </>
        ) : (
          <>
            <p>No saved desk found. Connect to one of desks listed below:</p>
            <div className="w-64 overflow-x-auto p-2 h-64">
              {dataToDisplay?.map((e, i) => (
                <DeskElement
                  key={i}
                  deskName={e.name}
                  onConnect={() => {
                    setIsConnected(true);
                    setConnectedNewDesk(e.name);
                  }}
                  isConnected={e.name === connectedNewDesk}
                />
              ))}
            </div>
            If your desk has a different name from "Desk XXXX", click the button
            below to expand the list
            <Button onClick={() => setShowAll(true)}>Show all devices</Button>
          </>
        )}
      </div>
      <p>Then, add a new postition!</p>
      <TooltipProvider>
        <Tooltip disableHoverableContent={!isConnected} delayDuration={100}>
          <TooltipTrigger>
            <Button className="mt-4" disabled={!isConnected}>
              <Link to="/new-position">Add a new position!</Link>
            </Button>
          </TooltipTrigger>
          <TooltipContent side="bottom">
            <p>
              You have to be connected to a desk to start adding new positions
            </p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  );
};

export default IntroPage;
