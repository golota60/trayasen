import { render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import DeskElement from "./DeskElement";

const { connectToDesk } = vi.hoisted(() => ({
  connectToDesk: vi.fn(),
}));

vi.mock("./rustUtils", () => ({ connectToDesk }));

const renderDesk = (
  overrides: Partial<React.ComponentProps<typeof DeskElement>> = {}
) => {
  const props = {
    deskName: "Desk 1234",
    onConnect: vi.fn(),
    onError: vi.fn(),
    onLoadStart: vi.fn(),
    onLoadEnd: vi.fn(),
    ...overrides,
  };

  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <DeskElement {...props} />
    </TamaguiProvider>
  );

  return props;
};

describe("DeskElement", () => {
  beforeEach(() => {
    connectToDesk.mockReset();
  });

  it("reports connection only after the desk connection resolves", async () => {
    let resolveConnection = () => {};
    const connection = new Promise<void>((resolve) => {
      resolveConnection = resolve;
    });
    connectToDesk.mockReturnValue(connection);
    const props = renderDesk();

    await userEvent.click(screen.getByRole("button", { name: "Connect" }));

    expect(connectToDesk).toHaveBeenCalledWith("Desk 1234");
    expect(props.onLoadStart).toHaveBeenCalledOnce();
    expect(props.onConnect).not.toHaveBeenCalled();
    expect(props.onLoadEnd).not.toHaveBeenCalled();

    resolveConnection();

    await waitFor(() => expect(props.onConnect).toHaveBeenCalledOnce());
    expect(props.onError).not.toHaveBeenCalled();
    expect(props.onLoadStart).toHaveBeenCalledOnce();
    expect(props.onLoadEnd).toHaveBeenCalledOnce();
  });

  it("reports an error without reporting a connection when connection fails", async () => {
    connectToDesk.mockRejectedValue(new Error("Bluetooth unavailable"));
    const props = renderDesk();

    await userEvent.click(screen.getByRole("button", { name: "Connect" }));

    await waitFor(() =>
      expect(props.onError).toHaveBeenCalledWith("Error: Bluetooth unavailable")
    );
    expect(props.onConnect).not.toHaveBeenCalled();
    expect(props.onLoadStart).toHaveBeenCalledOnce();
    expect(props.onLoadEnd).toHaveBeenCalledOnce();
  });
});
