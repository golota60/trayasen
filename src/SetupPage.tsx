import React, { useState } from "react";
import Button from "./generic/Button";
import { invoke } from "@tauri-apps/api";
import useSimpleAsync from 'use-simple-async';

const getShit = async (): Promise<any> => {
    return await invoke("get_test");
}

const SetupPage = () => {
    const [data] = useSimpleAsync(getShit);
    
    console.log(data);
    
  return (
    <div className="w-full h-full flex flex-col justify-center items-center bg-slate-800">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1 className="text-4xl mt-2 mb-3">Welcome to Idasen Tray!</h1>
      <p>
        This app will help you to interact with your IKEA Idasen Desk from the
        system tray.
      </p>
      <p>Let's start by connecting your desk</p>
    </div>
  );
};

export default SetupPage;
