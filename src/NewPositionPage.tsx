import React, { useState } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { Button } from "./generic/button";
import { Input } from "./generic/input";
import { MAX_HEIGHT, MIN_HEIGHT } from "./utils";
import { createNewElem } from "./rustUtils";
import { Label } from "./generic/label";

const modifierMap = new Map<string, string>([
  ["Command", "Cmd"],
  ["Control", "Ctrl"],
  ["Alt", "Alt"],
  ["Option", "Option"],
  ["AltGraph", "AltGr"],
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
  const [shortcutValue, setShortcutValue] = useState<string | undefined>("");
  console.log(shortcutValue);
  const [keystrokeText, setKeystrokeText] = useState<
    string | KeystoreStatusTexts
  >(KeystoreStatusTexts.init);

  const keystrokeHandler = (e: KeyboardEvent) => {
    if (e.defaultPrevented) {
      return;
    }
    const key = lowercaseMap.get(e.key) || e.key;
    const keyCode = e.code;
    console.log(key, keyCode);

    const isModifierKey = modifierMap.get(key);
    if (isModifierKey) {
      setShortcutValue((prevValue) => {
        const newVal = prevValue ? `${prevValue}+${key}` : key;
        return newVal;
      });
      return;
    }
    // Is normal key
    // If previous value(modifier) exists, append, otherwise take as-is
    setShortcutValue((prevValue) => {
      const newVal = prevValue ? `${prevValue}+${key}` : key;
      setKeystrokeText(newVal);
      return newVal;
    });

    window.removeEventListener("keydown", keystrokeHandler, true);

    e.preventDefault();
  };

  const handleRegisterKeyClick = () => {
    setShortcutValue("");
    setKeystrokeText(KeystoreStatusTexts.capturing);

    window.addEventListener("keydown", keystrokeHandler, true);
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
