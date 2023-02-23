interface Props {
    size: 'sm' | 'md' | 'lg';
}

const getSize = (size: Props['size']) => {
    switch (size) {
        case "sm":
            return 'w-6';
        case "md":
            return 'w-10';
        case "lg":
            return 'w-14';
    }
}

const Spinner = ({ size = 'sm' }: Props) => {
    return <img className={`animate-[spin_1s_linear_infinite] ${getSize(size)}`} src="/carrot.png" alt="A carrot logo" />;
}

export default Spinner;