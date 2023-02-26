import { invoke } from "@tauri-apps/api";
import Button from "./generic/Button";

interface Props {
  deskName: string;
}

const handleConnect = async (name: string) => {
  return await invoke('connect_to_desk_by_name', { name });
}

const DeskElement = ({ deskName }: Props) => {
  return <div className="flex justify-between items-center my-4">
    <span>{deskName}</span>
    <Button onClick={() => handleConnect(deskName)}>Connect</Button>
  </div >
}

export default DeskElement;