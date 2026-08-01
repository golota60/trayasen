import type { ReactNode } from "react";
import { H1, Image, ScrollView, Text, XStack, YStack } from "tamagui";

export interface PageShellProps {
  title: string;
  description?: string;
  children: ReactNode;
  actions?: ReactNode;
}

export const PageShell = ({
  title,
  description,
  children,
  actions,
}: PageShellProps) => (
  <ScrollView backgroundColor="$background" height="100%" width="100%">
    <YStack
      alignSelf="center"
      minHeight="100vh"
      maxWidth={960}
      padding="$7"
      width="100%"
      $sm={{ padding: "$4" }}
    >
      <header>
        <XStack
          alignItems="flex-start"
          borderBottomColor="$borderColor"
          borderBottomWidth={1}
          gap="$4"
          justifyContent="space-between"
          paddingBottom="$5"
          $sm={{
            alignItems: "stretch",
            flexDirection: "column",
            paddingBottom: "$4",
          }}
        >
          <XStack alignItems="flex-start" flex={1} gap="$3">
            <Image
              aria-hidden
              height={32}
              source={{ uri: "/carrot.png", width: 32, height: 32 }}
              width={32}
            />
            <YStack flex={1} gap="$2">
              <H1
                color="$color"
                fontSize="$9"
                fontWeight="700"
                lineHeight="$9"
                margin={0}
              >
                {title}
              </H1>
              {description ? (
                <Text color="$muted" fontSize="$4">
                  {description}
                </Text>
              ) : null}
            </YStack>
          </XStack>
          {actions ? (
            <XStack
              alignItems="center"
              flexWrap="wrap"
              gap="$3"
              $sm={{ flexDirection: "column", alignItems: "stretch" }}
            >
              {actions}
            </XStack>
          ) : null}
        </XStack>
      </header>
      <main>
        <YStack gap="$6" paddingTop="$6" $sm={{ paddingTop: "$5" }}>
          {children}
        </YStack>
      </main>
    </YStack>
  </ScrollView>
);
