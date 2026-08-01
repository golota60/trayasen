import { openUrl } from "@tauri-apps/plugin-opener";
import type { AnchorHTMLAttributes, ComponentProps, MouseEvent } from "react";
import { Anchor, styled } from "tamagui";

const StyledExternalLink = styled(Anchor, {
  color: "$accent",
  fontWeight: "600",
  textDecorationLine: "underline",
  focusStyle: {
    outlineColor: "$accent",
    outlineOffset: 2,
    outlineStyle: "solid",
    outlineWidth: 2,
  },
  focusVisibleStyle: {
    outlineColor: "$accent",
    outlineOffset: 2,
    outlineStyle: "solid",
    outlineWidth: 2,
  },
});

export interface ExternalLinkProps
  extends AnchorHTMLAttributes<HTMLAnchorElement> {}

export const ExternalLink = ({
  children,
  href,
  onClick,
  ...props
}: ExternalLinkProps) => {
  const openExternalUrl = async (event: MouseEvent<HTMLElement>) => {
    onClick?.(event as MouseEvent<HTMLAnchorElement>);
    if (event.defaultPrevented || !href || !/^https?:\/\//.test(href)) {
      return;
    }

    event.preventDefault();
    try {
      await openUrl(href);
    } catch (error) {
      console.error(`Could not open external URL ${href}`, error);
    }
  };

  return (
    <StyledExternalLink
      {...(props as ComponentProps<typeof StyledExternalLink>)}
      href={href}
      onClick={openExternalUrl}
    >
      {children}
    </StyledExternalLink>
  );
};
