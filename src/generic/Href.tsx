import clsx from "clsx";
import { HTMLProps } from "react";

interface Props extends HTMLProps<HTMLAnchorElement> {}

const Href = ({ children, className, ...props }: Props) => {
  return (
    <a
      className={clsx(
        className,
        "font-medium text-primary underline underline-offset-4"
      )}
      {...props}
    >
      {children}
    </a>
  );
};

export default Href;
