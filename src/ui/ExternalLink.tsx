import { openUrl } from "@tauri-apps/plugin-opener";
import type { AnchorHTMLAttributes, MouseEvent } from "react";

export interface ExternalLinkProps
  extends AnchorHTMLAttributes<HTMLAnchorElement> {}

export const ExternalLink = ({
  children,
  href,
  onClick,
  style,
  ...props
}: ExternalLinkProps) => {
  const openExternalUrl = async (event: MouseEvent<HTMLAnchorElement>) => {
    onClick?.(event);
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
    <a
      {...props}
      href={href}
      onClick={openExternalUrl}
      style={{
        color: "var(--accent)",
        fontWeight: 600,
        textDecorationLine: "underline",
        textUnderlineOffset: 4,
        ...style,
      }}
    >
      {children}
    </a>
  );
};
