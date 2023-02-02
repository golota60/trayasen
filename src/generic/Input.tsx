interface Props extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input = ({ className, ...rest }: Props) => (
  <input
    className={`rounded px-2 py-1 text-slate-800 ${className}`}
    {...rest}
  />
);

export default Input;
