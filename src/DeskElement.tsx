import { useState } from "react";
import Button from "./generic/Button";
import Spinner from "./generic/Spinner";
import { connectToDesk } from "./rustUtils";

interface Props {
  deskName: string;
  onConnect?: () => void;
  isConnected?: boolean;
}

const DeskElement = ({ deskName, onConnect, isConnected = false }: Props) => {
  const [loading, setLoading] = useState(false);

  const text = isConnected ? "Connected!" : "Connect";

  return (
    <div className="flex justify-between items-center my-4">
      <span>{deskName}</span>
      <Button
        className="w-16 h-6 flex justify-center items-center"
        onClick={async () => {
          setLoading(true);
          await connectToDesk(deskName);
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
