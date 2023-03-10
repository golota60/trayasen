import React from "react";

interface Props extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  className?: string;
  onClick?: (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void;
  children: React.ReactNode;
}
const Button = ({ children, onClick, className, ...rest }: Props) => {
  return (
    <button
      className={`rounded border-2 px-2 py-1 border-slate-600 bg-slate-900 ${className} disabled:bg-slate-500 disabled:cursor-not-allowed`}
      onClick={onClick}
      {...rest}
    >
      {children}
    </button>
  );
};

export default Button;
