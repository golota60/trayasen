import { invoke } from "@tauri-apps/api";
import React, { useMemo, useState } from "react";
import useSimpleAsync from "use-simple-async";
import DeskElement from "./DeskElement";
import Button from "./generic/Button";
import Spinner from "./generic/Spinner";

interface ConnectionDesk {
  name: string;
  status: "new" | 'saved';
}

async function getConnectionDesk() {
  return await invoke("get_desk_to_connect") as Array<ConnectionDesk>;
}

const IntroPage = () => {
  const [data, { error, loading }] = useSimpleAsync(getConnectionDesk, { useLayout: true });

  // workaround for checking whether the desk is saved. TODO: make this nicer
  // If it's saved, it's going to be only 1 element
  const isSaved = data?.[0].status === 'saved';

  if (loading) {
    return <div><Spinner size="lg" />Loading...</div>
  }

  if (error) {
    return <div>Something went wrong.</div>
  }

  return (
    <div className="w-full h-full flex flex-col justify-center items-center bg-slate-800">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1 className="text-4xl mt-2 mb-3">Welcome to Idasen Tray!</h1>
      <p>
        This app will help you to interact with your IKEA Idasen Desk from the
        system tray.
      </p>
      <div className="flex flex-col justify-center items-center my-4">
        {isSaved ?
          <p>Connecting to saved desk...</p> : <p>No saved desk found. Connect to one of desks listed below:</p>}
        <div className="w-64">{data?.map(e => <DeskElement deskName={e.name} />)}</div>

        <Spinner size="sm" />
      </div>
      <p>Start by adding a new postition!</p>
      <a href="/new-position">
        <Button className="mt-3">Add a new position!</Button>
      </a>
    </div >
  );
};

export default IntroPage;
