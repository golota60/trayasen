interface Props {
    size: 'sm' | 'md' | 'lg';
}

const Spinner = ({ size = 'sm' }: Props) => {
    return <img className={`spinner spinner-${size}`} src="/carrot.png" alt="A carrot logo" />;
}

export default Spinner;