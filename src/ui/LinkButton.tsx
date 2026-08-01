import { Link } from "found";
import type { ReactNode } from "react";
import { Text, XStack, styled } from "tamagui";

const LinkButtonFrame = styled(XStack, {
  alignItems: "center",
  borderRadius: "$4",
  borderWidth: 1,
  justifyContent: "center",
  minHeight: 42,
  paddingHorizontal: "$4",
  pointerEvents: "none",
  variants: {
    variant: {
      primary: {
        backgroundColor: "$accent",
        borderColor: "$accent",
      },
      secondary: {
        backgroundColor: "$backgroundStrong",
        borderColor: "$borderColor",
      },
    },
  } as const,
});

const LinkButtonText = styled(Text, {
  fontWeight: "600",
  variants: {
    variant: {
      primary: { color: "$accentColor" },
      secondary: { color: "$color" },
    },
  } as const,
});

export interface LinkButtonProps {
  to: string;
  children: ReactNode;
  variant?: "primary" | "secondary";
}

export const LinkButton = ({
  to,
  children,
  variant = "primary",
}: LinkButtonProps) => (
  <Link
    to={to}
    style={{
      borderRadius: 8,
      display: "inline-flex",
      outlineColor: "#F28C28",
      outlineOffset: 2,
      textDecoration: "none",
    }}
  >
    <LinkButtonFrame variant={variant}>
      <LinkButtonText variant={variant}>{children}</LinkButtonText>
    </LinkButtonFrame>
  </Link>
);
