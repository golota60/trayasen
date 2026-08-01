import { describe, expect, it } from "vitest";
import {
  applyShortcutKey,
  beginShortcutCapture,
  clearShortcutCapture,
  validatePosition,
} from "./positionForm";

describe("validatePosition", () => {
  it("returns field-specific validation errors", () => {
    expect(validatePosition("", "7200")).toEqual({
      name: "Name cannot be empty",
    });
    expect(validatePosition("Sit", "desk")).toEqual({
      value: "Height must be a number",
    });
    expect(validatePosition("Sit", "6199")).toEqual({
      value: "Height must be between 6200 and 12700",
    });
    expect(validatePosition("Sit", "12701")).toEqual({
      value: "Height must be between 6200 and 12700",
    });
    expect(validatePosition("Sit", "7200")).toEqual({});
  });
});

describe("shortcut capture transitions", () => {
  it("resets idle and captured shortcuts when capture begins", () => {
    expect(clearShortcutCapture()).toEqual({ status: "idle", value: "" });
    expect(beginShortcutCapture()).toEqual({
      status: "capturing",
      value: "",
    });

    const captured = applyShortcutKey(beginShortcutCapture(), { key: "s" });
    expect(captured).toEqual({ status: "captured", value: "s" });
    expect(beginShortcutCapture()).toEqual({
      status: "capturing",
      value: "",
    });
  });

  it("maps Control to CmdOrCtrl", () => {
    expect(
      applyShortcutKey(beginShortcutCapture(), { key: "Control" })
    ).toEqual({ status: "capturing", value: "CmdOrCtrl" });
  });

  it("appends a second distinct modifier", () => {
    const first = applyShortcutKey(beginShortcutCapture(), { key: "Control" });

    expect(applyShortcutKey(first, { key: "Shift" })).toEqual({
      status: "capturing",
      value: "CmdOrCtrl+Shift",
    });
  });

  it("restarts capture from a third modifier", () => {
    const first = applyShortcutKey(beginShortcutCapture(), { key: "Control" });
    const second = applyShortcutKey(first, { key: "Shift" });

    expect(applyShortcutKey(second, { key: "Alt" })).toEqual({
      status: "capturing",
      value: "Alt",
    });
  });

  it("maps a shifted exclamation mark to 1", () => {
    const shifted = applyShortcutKey(beginShortcutCapture(), { key: "Shift" });

    expect(applyShortcutKey(shifted, { key: "!" })).toEqual({
      status: "captured",
      value: "Shift+1",
    });
  });

  it("completes capture after a normal key", () => {
    expect(applyShortcutKey(beginShortcutCapture(), { key: "k" })).toEqual({
      status: "captured",
      value: "k",
    });
  });

  it("restarts rather than duplicating the same first modifier", () => {
    const first = applyShortcutKey(beginShortcutCapture(), { key: "Control" });

    expect(applyShortcutKey(first, { key: "Control" })).toEqual({
      status: "capturing",
      value: "CmdOrCtrl",
    });
  });
});
