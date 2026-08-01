import { describe, expect, it } from "vitest";
import { darkTheme } from "../../tamagui.config";

describe("dark Tamagui theme", () => {
  it("uses the approved dark palette and carrot accent", () => {
    expect(darkTheme.background).toBe("#0B0F14");
    expect(darkTheme.backgroundStrong).toBe("#111823");
    expect(darkTheme.color).toBe("#F5F7FA");
    expect(darkTheme.accent).toBe("#F28C28");
    expect(darkTheme.error).toBe("#F97066");
  });
});
