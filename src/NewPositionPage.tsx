import React, { useCallback, useEffect, useMemo, useState } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { Button } from "./generic/button";
import { Input } from "./generic/input";
import { MAX_HEIGHT, MIN_HEIGHT } from "./utils";
import { createNewElem } from "./rustUtils";
import { Label } from "./generic/label";

// Maps browser keys into accelerator keys
const modifierMap = new Map<string, string>([
  ["Command", "CmdOrCtrl"],
  ["Control", "CmdOrCtrl"],
  ["Alt", "Alt"],
  ["Option", "Option"],
  ["Shift", "Shift"],
  ["Super", "Super"],
  ["Meta", "Meta"],
]);

// Utility to change SUPER+value into just value. Useful for creating SHIFT+<key> shortcuts
// Cause if you really think about it, when you input Shift+1, you're just typing "!" and we need to avoid you doing that.
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

enum KeystoreStatusTexts {
  init = "Click to register shortcut",
  capturing = "Listening for input...",
}

enum ErrorCodes {
  no_name = "Name cannot be empty",
  wrong_value = "Value has to be between 6200 and 12700", // MIN_HEIGHT and MAX_HEIGHT
  value_string = "Value has to be a number",
  duplicate = "A position with that name already exists",
}

const NewPositionPage = () => {
  const [name, setName] = useState<string>("");
  const [value, setValue] = useState<string>("7200");
  const [error, setError] = useState<string | undefined>();
  const [shortcutValue, setShortcutValue] = useState<string>("");
  console.log(shortcutValue);
  const [keystrokeText, setKeystrokeText] = useState<
    string | KeystoreStatusTexts
  >(KeystoreStatusTexts.init);

  const keystrokeHandler = useCallback(
    (e: KeyboardEvent) => {
      console.log("key keyed");
      if (e.defaultPrevented) {
        return;
      }
      console.log(keystrokeText);
      if (keystrokeText !== KeystoreStatusTexts.capturing) {
        return;
      }
      console.log("raw key:", e.key, e.code);
      const clickedKey = lowercaseMap.get(e.key) || e.key;

      const isModifierKey = modifierMap.get(clickedKey);
      if (isModifierKey) {
        const existingModifiers = shortcutValue.split("+");
        // We want to avoid "Shift + Shift" modifiers
        const isTwoSameModifiers =
          existingModifiers.length === 1 && existingModifiers[0] === clickedKey;
        // Max 2 modifiers supported
        const isMaxTwoModifiers = existingModifiers.length > 1;

        console.log(
          isTwoSameModifiers,
          isMaxTwoModifiers,
          existingModifiers,
          clickedKey
        );
        console.log("end click");
        if (isTwoSameModifiers || isMaxTwoModifiers) {
          // Restart the shortcut
          setShortcutValue(clickedKey);

          return;
        }
        setShortcutValue((prevValue) => {
          const newVal = prevValue ? `${prevValue}+${clickedKey}` : clickedKey;
          return newVal;
        });
        return;
      }
      // Is normal key
      // If previous value(modifier) exists, append, otherwise take as-is
      setShortcutValue((prevValue) => {
        const newVal = prevValue ? `${prevValue}+${clickedKey}` : clickedKey;
        setKeystrokeText(newVal);
        return newVal;
      });

      e.preventDefault();
    },
    [keystrokeText, shortcutValue]
  );

  useEffect(() => {
    document.addEventListener("keydown", keystrokeHandler);

    return () => document.removeEventListener("keydown", keystrokeHandler);
  }, [keystrokeHandler]);

  const handleRegisterKeyClick = () => {
    setShortcutValue("");
    setKeystrokeText(KeystoreStatusTexts.capturing);
  };

  const handleChangeName = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newVal = e.target.value;

    setName(newVal);
  };

  const handleChangeValue = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newVal = e.target.value;

    setValue(newVal);
  };

  return (
    <>
      <img src="/carrot.png" alt="A carrot logo" />
      <div className="flex justify-center flex-col">
        <h1
          className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
 mt-2 mb-3"
        >
          Add a new position
        </h1>
        <div className="flex flex-col">
          <Label className="mt-2 mb-2" htmlFor="nameInput">
            Position name
          </Label>
          <Input value={name} id="nameInput" onChange={handleChangeName} />
          <Label className="mt-4 mb-2" htmlFor="valueInput">
            Position height (between {MIN_HEIGHT} and {MAX_HEIGHT})
          </Label>
          <Input value={value} id="valueInput" onChange={handleChangeValue} />{" "}
          <Label className="mt-4 mb-2" htmlFor="shortcutInput">
            Shortcut(optional)
          </Label>
          <div className="flex">
            <Button
              className="mr-2"
              id="shortcutInput"
              onClick={handleRegisterKeyClick}
            >
              {keystrokeText}
            </Button>
            <Button
              onClick={() => {
                setShortcutValue("");
                setKeystrokeText(KeystoreStatusTexts.init);
              }}
            >
              Clear shortcut
            </Button>
          </div>
        </div>
        <div className={`h-4 my-3 text-red-500 ${!error && "invisible"}`}>
          {error || ""}
        </div>

        <Button
          className="mt-2"
          onClick={async () => {
            let valAsNum = Number(value);
            let locErr: ErrorCodes | undefined;
            if (!name) {
              locErr = ErrorCodes.no_name;
            } else if (isNaN(valAsNum) || !valAsNum) {
              locErr = ErrorCodes.value_string;
            } else if (valAsNum < MIN_HEIGHT || valAsNum > MAX_HEIGHT) {
              locErr = ErrorCodes.wrong_value;
            } else {
              locErr = undefined;
              setError(undefined);
            }

            if (locErr) {
              setError(locErr);
            } else {
              // try to create an elem
              let resp = await createNewElem(name, value, shortcutValue);

              if (resp === "duplicate") {
                setError(ErrorCodes.duplicate);
              } else {
                // exit cause shits been created
                console.log("closing...");
                appWindow.close();
              }
            }
          }}
        >
          Add
        </Button>
      </div>
    </>
  );
};

export default NewPositionPage;
