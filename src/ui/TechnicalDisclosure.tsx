import { Text, YStack } from "tamagui";

interface TechnicalDisclosureProps {
  label: string;
  children: string;
}

export const TechnicalDisclosure = ({
  label,
  children,
}: TechnicalDisclosureProps) => (
  <YStack
    aria-label={label}
    asChild
    borderColor="$borderColor"
    borderTopWidth={1}
    maxWidth="100%"
    paddingTop="$3"
    width="100%"
  >
    <details>
      <Text
        asChild
        color="$muted"
        cursor="pointer"
        focusStyle={{ outlineColor: "$accent" }}
        fontWeight="600"
        outlineColor="transparent"
        outlineOffset={2}
        outlineStyle="solid"
        outlineWidth={2}
      >
        <summary>{label}</summary>
      </Text>
      <Text
        asChild
        color="$color"
        fontFamily="$mono"
        fontSize="$3"
        marginBottom={0}
        maxWidth="100%"
        style={{ overflowWrap: "anywhere" } as never}
        whiteSpace="pre-wrap"
      >
        <pre>{children}</pre>
      </Text>
    </details>
  </YStack>
);
