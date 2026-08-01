import { describe, expect, it } from "vitest";
import { filterDevices } from "./deviceFilters";

const devices = [
  { name: "Desk 1234", status: "new" as const },
  { name: "Headphones", status: "new" as const },
];

describe("filterDevices", () => {
  it("shows only desk-named devices by default", () => {
    expect(filterDevices(devices, false).map(({ name }) => name)).toEqual([
      "Desk 1234",
    ]);
  });

  it("shows every device when requested", () => {
    expect(filterDevices(devices, true)).toEqual(devices);
  });

  it("returns an empty list before discovery resolves", () => {
    expect(filterDevices(undefined, false)).toEqual([]);
  });
});
