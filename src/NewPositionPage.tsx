import React, { useState } from "react";
import { appWindow } from "@tauri-apps/api/window";
import Button from "./generic/Button";
import Input from "./generic/Input";
import { MAX_HEIGHT, MIN_HEIGHT } from "./utils";
import { createNewElem } from "./rustUtils";

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
        <h1 className="text-4xl mt-2 mb-3">Create a new position</h1>

        <div className="flex flex-col">
          <label htmlFor="nameInput">Position name</label>
          <Input value={name} id="nameInput" onChange={handleChangeName} />
          <label className="mt-2" htmlFor="valueInput">
            Position height(between {MIN_HEIGHT} and {MAX_HEIGHT})
          </label>
          <Input value={value} id="valueInput" onChange={handleChangeValue} />
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
              let resp = await createNewElem(name, value);

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
