import clsx from "clsx";
import { openUrl } from "@tauri-apps/plugin-opener";
import { HTMLProps, MouseEvent } from "react";

interface Props extends HTMLProps<HTMLAnchorElement> {}

const Href = ({ children, className, href, onClick, ...props }: Props) => {
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
      className={clsx(
        className,
        "font-medium text-primary underline underline-offset-4"
      )}
      href={href}
      onClick={openExternalUrl}
      {...props}
    >
      {children}
    </a>
  );
};

export default Href;
