/**
 * Functions defined in rust
 */

import { invoke } from "@tauri-apps/api";

export const connectToDesk = async (name: string) => {
  return await invoke("connect_to_desk_by_name", { name });
};

export interface ConnectionDesk {
  name: string;
  status: "new" | "saved";
}

export const getConnectionDesk = async () => {
  return (await invoke("get_desk_to_connect")) as Array<ConnectionDesk>;
};

export interface Config {
  local_name: string;
  saved_positions: Array<{ name: string; value: number; shortcut?: string }>;
}

export const getPositions = async (): Promise<Config> => {
  return await invoke("get_config");
};

export const removePosition = async (positionName: string): Promise<Config> => {
  return await invoke("remove_position", { posName: positionName });
};

export const createNewElem = async (
  name: string,
  value: string | number,
  shortcutvalue?: string
): Promise<"duplicate" | "success"> => {
  console.log(shortcutvalue);
  return await invoke("create_new_elem", {
    name,
    value: Number(value),
    shortcutvalue: shortcutvalue !== "" ? shortcutvalue : undefined,
  });
};

export const removeConfig = async () => {
  return await invoke("remove_config");
};

export const resetDesk = async () => {
  return await invoke("reset_desk");
};
