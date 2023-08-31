import { useState } from "react";

const AutoconnectPage = () => {
  const [isConnecting, setIsConnecting] = useState(false);

  return (
    <div className="w-full h-full flex flex-col justify-center items-center">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1
        className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
mt-2 mb-3"
      >
        Connecting to your desk...
      </h1>
    </div>
  );
};

export default AutoconnectPage;
