import { describe, expect, it } from "vitest";
import {
  getBrowserProcessEnvironment,
  resolveViteNodeEnvironment,
} from "../viteBrowserEnvironment.js";

describe("Vite browser environment", () => {
  it("replaces Tamagui process.env reads with browser-safe values", () => {
    const browserEnvironment = getBrowserProcessEnvironment("development");

    expect(JSON.parse(browserEnvironment)).toEqual({
      NODE_ENV: "development",
      TAMAGUI_TARGET: "web",
    });
  });

  it("derives NODE_ENV from the Vite command rather than its custom mode", () => {
    expect(resolveViteNodeEnvironment("serve", undefined)).toBe("development");
    expect(resolveViteNodeEnvironment("build", undefined)).toBe("production");
    expect(resolveViteNodeEnvironment("serve", "production")).toBe(
      "production"
    );
  });
});
