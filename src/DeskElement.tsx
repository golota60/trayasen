import Button from "./generic/Button";

interface Props {
  deskName: string;
  handleConnect?: () => void;
}

const DeskElement = ({ deskName, handleConnect }: Props) => {
  return <div className="flex justify-between items-center my-4">
    <span>{deskName}</span>
    <Button onClick={handleConnect}>Connect</Button>
  </div>
}

export default DeskElement;