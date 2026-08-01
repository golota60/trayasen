import { MAX_HEIGHT, MIN_HEIGHT } from "../../utils";

export type PositionFieldErrors = { name?: string; value?: string };

export type ShortcutCaptureState =
  | { status: "idle"; value: "" }
  | { status: "capturing"; value: string }
  | { status: "captured"; value: string };

const modifierMap = new Map<string, string>([
  ["Command", "CmdOrCtrl"],
  ["Control", "CmdOrCtrl"],
  ["Alt", "Alt"],
  ["Option", "Option"],
  ["Shift", "Shift"],
  ["Super", "Super"],
  ["Meta", "Meta"],
]);

const lowercaseMap = new Map<string, string>([
  ["!", "1"],
  ["@", "2"],
  ["#", "3"],
  ["$", "4"],
  ["%", "5"],
  ["^", "6"],
  ["&", "7"],
  ["*", "8"],
  ["(", "9"],
  [")", "0"],
  ["_", "-"],
  ["+", "="],
  ["{", "["],
  ["}", "]"],
  [":", ";"],
  ['"', "'"],
  ["<", ","],
  [">", "."],
  ["?", "/"],
  ["|", "\\"],
  ["~", "`"],
]);

export const validatePosition = (
  name: string,
  value: string
): PositionFieldErrors => {
  if (!name) {
    return { name: "Name cannot be empty" };
  }

  const numericValue = Number(value);
  if (Number.isNaN(numericValue) || !numericValue) {
    return { value: "Height must be a number" };
  }
  if (numericValue < MIN_HEIGHT || numericValue > MAX_HEIGHT) {
    return {
      value: `Height must be between ${MIN_HEIGHT} and ${MAX_HEIGHT}`,
    };
  }

  return {};
};

export const beginShortcutCapture = (): ShortcutCaptureState => ({
  status: "capturing",
  value: "",
});

export const clearShortcutCapture = (): ShortcutCaptureState => ({
  status: "idle",
  value: "",
});

export const applyShortcutKey = (
  state: ShortcutCaptureState,
  event: Pick<KeyboardEvent, "key">
): ShortcutCaptureState => {
  if (state.status !== "capturing") {
    return state;
  }

  const key = lowercaseMap.get(event.key) ?? event.key;
  const modifier = modifierMap.get(key);

  if (modifier) {
    const existingModifiers = state.value ? state.value.split("+") : [];
    if (existingModifiers.length >= 2 || existingModifiers.includes(modifier)) {
      return { status: "capturing", value: modifier };
    }

    return {
      status: "capturing",
      value: state.value ? `${state.value}+${modifier}` : modifier,
    };
  }

  return {
    status: "captured",
    value: state.value ? `${state.value}+${key}` : key,
  };
};
