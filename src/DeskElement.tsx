import { useState } from "react";
import { Button } from "./generic/button";
import Spinner from "./generic/Spinner";
import { connectToDesk } from "./rustUtils";

interface Props {
  deskName: string;
  onConnect?: () => void;
  onError: (err: string) => void;
  isConnected?: boolean;
}

const DeskElement = ({
  deskName,
  onConnect,
  onError,
  isConnected = false,
}: Props) => {
  const [loading, setLoading] = useState(false);

  const text = isConnected ? "Connected!" : "Connect";

  return (
    <div className="flex justify-between items-center my-4">
      <span>{deskName}</span>
      <Button
        className="flex justify-center items-center"
        onClick={async () => {
          setLoading(true);
          try {
            const result = await connectToDesk(deskName);
            console.log("result", result);
          } catch (e) {
            console.log("error", e);
            onError(e as string);
          }
          setLoading(false);
          onConnect?.();
        }}
      >
        {loading ? <Spinner size="sm" /> : text}
      </Button>
    </div>
  );
};

export default DeskElement;
