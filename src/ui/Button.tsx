import type { ComponentProps } from "react";
import { Button, Spinner, Text, XStack, styled } from "tamagui";

const StyledButton = styled(Button, {
  borderWidth: 1,
  borderColor: "transparent",
  borderRadius: "$4",
  fontWeight: "600",
  pressStyle: {
    scale: 0.98,
  },
  focusStyle: {
    outlineColor: "$accent",
    outlineStyle: "solid",
    outlineWidth: 2,
    outlineOffset: 2,
  },
  variants: {
    variant: {
      primary: {
        backgroundColor: "$accent",
        borderColor: "$accent",
        color: "$accentColor",
        hoverStyle: { backgroundColor: "$accentHover" },
        pressStyle: { backgroundColor: "$accentPress", scale: 0.98 },
      },
      secondary: {
        backgroundColor: "$backgroundStrong",
        borderColor: "$borderColor",
        color: "$color",
        hoverStyle: {
          backgroundColor: "$backgroundHover",
          borderColor: "$borderColorHover",
        },
        pressStyle: {
          backgroundColor: "$backgroundPress",
          borderColor: "$borderColorPress",
          scale: 0.98,
        },
      },
      ghost: {
        backgroundColor: "$backgroundTransparent",
        borderColor: "transparent",
        color: "$color",
        hoverStyle: { backgroundColor: "$backgroundHover" },
        pressStyle: { backgroundColor: "$backgroundPress", scale: 0.98 },
      },
      destructive: {
        backgroundColor: "$errorBackground",
        borderColor: "$error",
        color: "$error",
        hoverStyle: { opacity: 0.9 },
        pressStyle: { opacity: 0.8, scale: 0.98 },
      },
    },
  } as const,
  defaultVariants: {
    variant: "primary",
  },
});

type StyledButtonProps = ComponentProps<typeof StyledButton>;

export interface AppButtonProps
  extends Omit<StyledButtonProps, "variant" | "disabled"> {
  variant?: "primary" | "secondary" | "ghost" | "destructive";
  disabled?: boolean;
  loading?: boolean;
  loadingLabel?: string;
}

export const AppButton = ({
  children,
  disabled,
  loading = false,
  loadingLabel,
  variant = "primary",
  "aria-label": ariaLabel,
  ...props
}: AppButtonProps) => (
  <StyledButton
    {...props}
    aria-label={loading ? loadingLabel ?? ariaLabel ?? "Loading" : ariaLabel}
    disabled={disabled || loading}
    opacity={disabled || loading ? 0.55 : 1}
    variant={variant}
  >
    {loading ? (
      <XStack alignItems="center" gap="$2">
        <Spinner size="small" color="currentColor" />
        {loadingLabel ? <Text color="inherit">{loadingLabel}</Text> : null}
      </XStack>
    ) : (
      children
    )}
  </StyledButton>
);
