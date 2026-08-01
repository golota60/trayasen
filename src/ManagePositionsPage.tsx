import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState } from "react";
import useSimpleAsync from "use-simple-async";
import { PositionList } from "./features/positions/PositionList";
import { getPositions, removePosition } from "./rustUtils";
import { AppButton } from "./ui/Button";
import { Alert } from "./ui/Feedback";
import { LinkButton } from "./ui/LinkButton";
import { PageShell } from "./ui/PageShell";

const appWindow = getCurrentWindow();

const ManagePositionsPage = () => {
  const [data, { error: loadError, loading, retry }] =
    useSimpleAsync(getPositions);
  const [removingName, setRemovingName] = useState<string>();
  const [actionError, setActionError] = useState<string>();

  const handleRemove = async (name: string) => {
    setActionError(undefined);
    setRemovingName(name);

    try {
      await removePosition(name);
      retry();
    } catch (removeError) {
      setActionError(String(removeError));
    } finally {
      setRemovingName(undefined);
    }
  };

  return (
    <PageShell
      actions={
        <>
          <LinkButton to="/new-position">New position</LinkButton>
          <AppButton
            onPress={() => {
              appWindow.close();
            }}
            type="button"
            variant="secondary"
          >
            Close
          </AppButton>
        </>
      }
      title="Manage positions"
    >
      {loadError ? (
        <Alert tone="error" title="Could not load positions">
          {String(loadError)}
        </Alert>
      ) : null}
      {actionError ? (
        <Alert tone="error" title="Could not remove position">
          {actionError}
        </Alert>
      ) : null}
      {!loadError ? (
        <PositionList
          loading={loading}
          onRemove={handleRemove}
          positions={data?.saved_positions}
          removingName={removingName}
        />
      ) : null}
    </PageShell>
  );
};

export default ManagePositionsPage;
