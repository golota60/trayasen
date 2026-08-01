import { useState } from "react";
import useSimpleAsync from "use-simple-async";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Link } from "found";
import { Button } from "./generic/button";
import removeIcon from "./assets/cross.svg";
import { getPositions, removePosition } from "./rustUtils";

const appWindow = getCurrentWindow();

const ManagePositionsPage = () => {
  const [data, { error: loadError, loading, retry }] =
    useSimpleAsync(getPositions);
  const [removingName, setRemovingName] = useState<string>();
  const [actionError, setActionError] = useState<string>();

  console.log(data);
  return (
    <div className="flex flex-col items-center">
      <img src="/carrot.png" alt="A carrot logo" />
      <h1
        className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0
 mt-2 mb-3"
      >
        Manage positions
      </h1>
      <div className="h-80 max-w-md overflow-scroll  overflow-x-hidden">
        <table className="grid grid-cols-[2fr_2fr_2fr_1fr] text-left gap-x-3">
          <thead className="contents">
            <th className="sticky top-0 bg-slate-800">Name</th>
            <th className="sticky top-0 bg-slate-800">Height</th>
            <th className="sticky top-0 bg-slate-800">Shortcut</th>
            <th className="sticky top-0 bg-slate-800">Actions</th>
          </thead>
          {data?.saved_positions
            ? data?.saved_positions.map(({ name, value, shortcut }) => (
                <tbody className="contents" key={name}>
                  {/**
                   * TODO: Add a tooltip or some shit
                   */}
                  <td className="h-8 text-ellipsis overflow-hidden inline-block whitespace-nowrap max-w-xs">
                    {name}
                  </td>
                  <td className="h-8">{value}</td>
                  <td className="h-8">{shortcut}</td>
                  <td className="h-8 flex flex-row-reverse">
                    <button
                      type="button"
                      disabled={removingName !== undefined}
                      onClick={async () => {
                        setRemovingName(name);
                        setActionError(undefined);
                        try {
                          await removePosition(name);
                          retry();
                        } catch (removeError) {
                          setActionError(
                            `Could not remove position: ${String(removeError)}`
                          );
                        } finally {
                          setRemovingName(undefined);
                        }
                      }}
                      className="disabled:cursor-not-allowed disabled:opacity-50"
                    >
                      <img
                        className="cursor-pointer"
                        src={removeIcon}
                        alt={`Remove ${name}`}
                      />
                    </button>
                  </td>
                </tbody>
              ))
            : loading
            ? "Loading..."
            : null}
        </table>
      </div>
      {loadError ? (
        <div className="my-2 text-red-500">
          Could not load positions: {String(loadError)}
        </div>
      ) : null}
      {actionError ? (
        <div className="my-2 text-red-500">{actionError}</div>
      ) : null}
      <div className="w-full flex justify-between mt-3">
        <Link to="/new-position">
          <Button>New position</Button>
        </Link>
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
