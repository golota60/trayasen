import { useState } from "react";
import { Text, XStack, YStack } from "tamagui";
import { connectToDesk } from "./rustUtils";
import { AppButton } from "./ui/Button";

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

  const connect = async () => {
    setLoading(true);
    onLoadStart?.();
    try {
      await connectToDesk(deskName);
      onConnect?.();
    } catch (error) {
      onError(String(error));
    } finally {
      setLoading(false);
      onLoadEnd?.();
    }
  };

  return (
    <XStack
      alignItems="center"
      borderColor={isConnected ? "$success" : "$borderColor"}
      borderRadius="$4"
      borderWidth={1}
      gap="$4"
      justifyContent="space-between"
      padding="$3"
    >
      <YStack flex={1} gap="$1">
        <Text color="$color" fontWeight="600">
          {deskName}
        </Text>
        <Text color={isConnected ? "$success" : "$muted"} fontSize="$3">
          {isConnected ? "Connected" : "Available"}
        </Text>
      </YStack>
      <AppButton
        disabled={disabled || isConnected}
        loading={loading}
        loadingLabel="Connecting"
        onPress={connect}
      >
        {isConnected ? "Connected" : "Connect"}
      </AppButton>
    </XStack>
  );
};

export default DeskElement;
