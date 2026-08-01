import type { ReactNode } from "react";
import { Image, Text, YStack, styled } from "tamagui";

const AlertFrame = styled(YStack, {
  borderWidth: 1,
  borderRadius: "$4",
  padding: "$4",
  gap: "$2",
  variants: {
    tone: {
      error: {
        backgroundColor: "$errorBackground",
        borderColor: "$error",
      },
      success: {
        backgroundColor: "$successBackground",
        borderColor: "$success",
      },
      info: {
        backgroundColor: "$backgroundStrong",
        borderColor: "$borderColor",
      },
    },
  } as const,
});

const AlertTitle = styled(Text, {
  fontWeight: "700",
  variants: {
    tone: {
      error: { color: "$error" },
      success: { color: "$success" },
      info: { color: "$color" },
    },
  } as const,
});

export interface AlertProps {
  tone: "error" | "success" | "info";
  title: string;
  children?: ReactNode;
}

export const Alert = ({ tone, title, children }: AlertProps) => (
  <AlertFrame role={tone === "error" ? "alert" : "status"} tone={tone}>
    <AlertTitle tone={tone}>{title}</AlertTitle>
    {children ? <Text color="$color">{children}</Text> : null}
  </AlertFrame>
);

const spinnerSizes = {
  sm: 24,
  md: 40,
  lg: 56,
} as const;

export interface CarrotSpinnerProps {
  "aria-label"?: string;
  size?: "sm" | "md" | "lg";
}

export const CarrotSpinner = ({
  "aria-label": ariaLabel = "Loading",
  size = "sm",
}: CarrotSpinnerProps) => {
  const dimension = spinnerSizes[size];

  return (
    <>
      <style>{`@keyframes carrot-spinner-rotation { to { transform: rotate(360deg); } }`}</style>
      <Image
        aria-label={ariaLabel}
        className="carrot-spinner"
        height={dimension}
        role="img"
        source={{ uri: "/carrot.png", width: dimension, height: dimension }}
        style={{ animation: "carrot-spinner-rotation 1.5s linear infinite" }}
        width={dimension}
      />
    </>
  );
};

export interface StatePanelProps {
  icon?: ReactNode;
  title: string;
  description?: string;
  action?: ReactNode;
}

export const StatePanel = ({
  icon,
  title,
  description,
  action,
}: StatePanelProps) => (
  <YStack alignItems="center" gap="$3" padding="$6">
    {icon}
    <Text color="$color" fontSize="$7" fontWeight="700" textAlign="center">
      {title}
    </Text>
    {description ? (
      <Text color="$muted" maxWidth={520} textAlign="center">
        {description}
      </Text>
    ) : null}
    {action}
  </YStack>
);
