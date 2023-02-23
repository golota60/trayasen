import { invoke } from "@tauri-apps/api";
import React, { useMemo, useState } from "react";
import useSimpleAsync from "use-simple-async";
import Button from "./generic/Button";


async function getDeskNames()  {
  return await invoke("get_avail_desks") as string[];
}

const IntroPage = () => {
  const [data] = useSimpleAsync(getDeskNames);

  return (
    <div className="w-full h-full flex flex-col justify-center items-center bg-slate-800">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1 className="text-4xl mt-2 mb-3">Welcome to Idasen Tray!</h1>
      <p>
        This app will help you to interact with your IKEA Idasen Desk from the
        system tray.
      </p>
      <p>Start by connecting your desk</p>
      {data?.map(e => <li>{e}</li>)}

      <p>Start by adding a new postition!</p>
      <a href="/new-position">
        <Button className="mt-3">Add a new position!</Button>
      </a>
    </div>
  );
};

export default IntroPage;
