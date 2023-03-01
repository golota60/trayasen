import { useEffect, useState } from "react";
import useSimpleAsync from "use-simple-async";
import DeskElement from "./DeskElement";
import Button from "./generic/Button";
import Spinner from "./generic/Spinner";
import { connectToDesk, getConnectionDesk } from "./rustUtils";

const IntroPage = () => {
  const [data, { error, loading }] = useSimpleAsync(getConnectionDesk, {
    useLayout: true,
  });
  const [isConnecting, setIsConnecting] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [connectedNewDesk, setConnectedNewDesk] = useState<string>();

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
    return <div>Something went wrong.</div>;
  }

  return (
    <div className="w-full h-full flex flex-col justify-center items-center bg-slate-800">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1 className="text-4xl mt-2 mb-3">Welcome to Idasen Tray!</h1>
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
            <div className="w-64">
              {data?.map((e, i) => (
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
          </>
        )}
      </div>
      <p>Then, add a new postition!</p>
      <a href="/new-position">
        <Button className="mt-3" disabled={!isConnected}>
          Add a new position!
        </Button>
      </a>
    </div>
  );
};

export default IntroPage;
