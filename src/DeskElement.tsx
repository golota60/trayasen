import { useState } from "react";
import { Button } from "./generic/button";
import Spinner from "./generic/Spinner";
import { connectToDesk } from "./rustUtils";

interface Props {
  deskName: string;
  onConnect?: () => void;
  onError: (err: string) => void;
  onLoadStart?: () => void;
  onLoadEnd?: () => void;
  isConnected?: boolean;
  disabled?: boolean;
}

const DeskElement = ({
  deskName,
  onConnect,
  onError,
  isConnected = false,
  disabled,
  onLoadStart,
  onLoadEnd,
}: Props) => {
  const [loading, setLoading] = useState(false);

  const text = isConnected ? "Connected!" : "Connect";

  return (
    <div className="flex justify-between items-center my-4">
      <span>{deskName}</span>
      <Button
        disabled={disabled || loading}
        className="flex justify-center items-center"
        onClick={async () => {
          setLoading(true);
          onLoadStart?.();
          try {
            await connectToDesk(deskName);
          } catch (e) {
            onError(e as string);
          }
          setLoading(false);
          onLoadEnd?.();
          onConnect?.();
        }}
      >
        {loading ? <Spinner size="sm" /> : text}
      </Button>
    </div>
  );
};

export default DeskElement;
