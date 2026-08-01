import { Trash2 } from "lucide-react";
import { ScrollView, Text, XStack, YStack } from "tamagui";
import type { Config } from "../../rustUtils";
import { AppButton } from "../../ui/Button";
import { SurfaceCard } from "../../ui/Card";
import { CarrotSpinner, StatePanel } from "../../ui/Feedback";

export interface PositionListProps {
  positions: Config["saved_positions"] | undefined;
  loading: boolean;
  removingName?: string;
  onRemove(name: string): void;
}

const columnLabelProps = {
  color: "$muted",
  fontSize: "$3",
  fontWeight: "700",
} as const;

const PositionHeader = () => (
  <XStack
    alignItems="center"
    gap="$3"
    paddingHorizontal="$3"
    $sm={{ alignItems: "stretch", flexDirection: "column", gap: "$2" }}
  >
    <Text
      {...columnLabelProps}
      flex={2}
      minWidth={0}
      $sm={{ flex: 0, width: "100%" }}
    >
      Name
    </Text>
    <XStack
      alignItems="center"
      flex={3}
      gap="$3"
      $sm={{ flex: 0, width: "100%" }}
    >
      <Text {...columnLabelProps} flex={1} minWidth={0}>
        Height
      </Text>
      <Text {...columnLabelProps} flex={2} minWidth={0}>
        Shortcut
      </Text>
      <YStack aria-hidden width={40} />
    </XStack>
  </XStack>
);

export const PositionList = ({
  positions,
  loading,
  removingName,
  onRemove,
}: PositionListProps) => {
  if (loading) {
    return (
      <SurfaceCard role="status">
        <StatePanel
          icon={<CarrotSpinner decorative />}
          title="Loading positions"
        />
      </SurfaceCard>
    );
  }

  if (!positions || positions.length === 0) {
    return (
      <SurfaceCard>
        <StatePanel
          title="No saved positions yet"
          description="Add a position to quickly return your desk to a saved height."
        />
      </SurfaceCard>
    );
  }

  return (
    <SurfaceCard padding="$3">
      <PositionHeader />
      <ScrollView maxHeight={320}>
        <YStack gap="$2">
          {positions.map(({ name, value, shortcut }) => {
            const isRemoving = removingName === name;

            return (
              <XStack
                alignItems="center"
                borderColor="$borderColor"
                borderTopWidth={1}
                gap="$3"
                key={name}
                padding="$3"
                $sm={{
                  alignItems: "stretch",
                  flexDirection: "column",
                  gap: "$2",
                }}
              >
                <Text
                  {...{ title: name }}
                  color="$color"
                  flex={2}
                  fontWeight="600"
                  minWidth={0}
                  numberOfLines={1}
                  overflow="hidden"
                  textOverflow="ellipsis"
                  $sm={{ flex: 0, width: "100%" }}
                >
                  {name}
                </Text>
                <XStack
                  alignItems="center"
                  flex={3}
                  gap="$3"
                  $sm={{ flex: 0, width: "100%" }}
                >
                  <Text color="$color" flex={1} minWidth={0}>
                    {value}
                  </Text>
                  <Text color="$muted" flex={2} minWidth={0} numberOfLines={1}>
                    {shortcut || "—"}
                  </Text>
                  <AppButton
                    aria-label={
                      isRemoving ? `Removing ${name}` : `Remove ${name}`
                    }
                    circular
                    disabled={removingName !== undefined}
                    loading={isRemoving}
                    onPress={() => onRemove(name)}
                    size="$3"
                    type="button"
                    variant="destructive"
                  >
                    <Trash2 aria-hidden size={18} />
                  </AppButton>
                </XStack>
              </XStack>
            );
          })}
        </YStack>
      </ScrollView>
    </SurfaceCard>
  );
};
