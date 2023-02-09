import { invoke } from "@tauri-apps/api";
import React, { useState } from "react";
import useSimpleAsync from "use-simple-async";
import Button from "./generic/Button";
import removeIcon from "./assets/cross.svg";
import { appWindow } from "@tauri-apps/api/window";

interface Config {
  local_name: string;
  saved_positions: Array<{ name: string; value: number }>;
}

async function getPositions(): Promise<Config> {
  return await invoke("get_config");
}

async function removePosition(positionName: string): Promise<Config> {
  return await invoke("remove_position", { posName: positionName });
}

const ManagePositionsPage = () => {
  const [data, { loading, error, retry }] = useSimpleAsync(getPositions);

  console.log(data);
  return (
    <div className="flex flex-col items-center">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1 className="text-4xl mt-2 mb-3">Manage positions</h1>
      <div className="h-80 max-w-md overflow-scroll  overflow-x-hidden">
        <table className="grid grid-cols-[2fr_2fr_1fr] text-left gap-x-3">
          <thead className="contents">
            <th className="sticky top-0 bg-slate-800">Name</th>
            <th className="sticky top-0 bg-slate-800">Height</th>
            <th className="sticky top-0 bg-slate-800">Actions</th>
          </thead>
          {data?.saved_positions
            ? data?.saved_positions.map(({ name, value }) => (
                <tbody className="contents">
                  {/**
                   * TODO: Add a tooltip or some shit
                   */}
                  <td className="h-8 text-ellipsis overflow-hidden inline-block whitespace-nowrap max-w-xs">
                    {name}
                  </td>
                  <td className="h-8">{value}</td>
                  <td className="h-8 flex flex-row-reverse">
                    <img
                      onClick={async () => {
                        // TODO: use return value instead of retry
                        await removePosition(name);
                        retry();
                      }}
                      className="cursor-pointer"
                      src={removeIcon}
                      alt="Remove position icon"
                    />
                  </td>
                </tbody>
              ))
            : "Loading..."}
        </table>
      </div>
      <div className="w-full flex justify-between mt-3">
        <a href="/new-position">
          <Button>New position</Button>
        </a>
        <Button
          onClick={() => {
            appWindow.close();
          }}
        >
          Close
        </Button>
      </div>
    </div>
  );
};

export default ManagePositionsPage;
