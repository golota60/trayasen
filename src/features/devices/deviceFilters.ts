import type { ConnectionDesk } from "../../rustUtils";

export const filterDevices = (
  devices: ConnectionDesk[] | undefined,
  showAll: boolean
): ConnectionDesk[] =>
  showAll
    ? devices ?? []
    : (devices ?? []).filter(({ name }) => name.includes("Desk"));
