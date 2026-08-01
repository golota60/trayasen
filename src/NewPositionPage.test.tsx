import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import NewPositionPage from "./NewPositionPage";

const mocks = vi.hoisted(() => ({
  close: vi.fn(),
  createNewElem: vi.fn(),
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ close: mocks.close }),
}));

vi.mock("./rustUtils", () => ({
  createNewElem: mocks.createNewElem,
}));

const renderPage = () =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <NewPositionPage />
    </TamaguiProvider>
  );

const enterValidPosition = async () => {
  await userEvent.type(screen.getByLabelText("Position name"), "Standing");
};

describe("NewPositionPage native workflow", () => {
  beforeEach(() => {
    mocks.close.mockReset();
    mocks.createNewElem.mockReset();
    mocks.close.mockResolvedValue(undefined);
  });

  it("suppresses the native call when submit validation fails", async () => {
    renderPage();

    await userEvent.click(screen.getByRole("button", { name: "Add position" }));

    expect(mocks.createNewElem).not.toHaveBeenCalled();
    expect(screen.getByLabelText("Position name")).toHaveAccessibleDescription(
      "Name cannot be empty"
    );
    expect(mocks.close).not.toHaveBeenCalled();
  });

  it("shows duplicate feedback on the name field without closing", async () => {
    mocks.createNewElem.mockResolvedValueOnce("duplicate");
    renderPage();
    await enterValidPosition();

    await userEvent.click(screen.getByRole("button", { name: "Add position" }));

    await waitFor(() =>
      expect(
        screen.getByLabelText("Position name")
      ).toHaveAccessibleDescription("A position with that name already exists")
    );
    expect(mocks.createNewElem).toHaveBeenCalledWith("Standing", "7200", "");
    expect(mocks.close).not.toHaveBeenCalled();
  });

  it("shows native rejection feedback without closing", async () => {
    mocks.createNewElem.mockRejectedValueOnce(new Error("native write failed"));
    renderPage();
    await enterValidPosition();

    await userEvent.click(screen.getByRole("button", { name: "Add position" }));

    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent("native write failed")
    );
    expect(mocks.close).not.toHaveBeenCalled();
  });

  it("closes only for the explicit native success response", async () => {
    mocks.createNewElem.mockResolvedValueOnce("unexpected");
    renderPage();
    await enterValidPosition();

    await userEvent.click(screen.getByRole("button", { name: "Add position" }));
    await waitFor(() => expect(mocks.createNewElem).toHaveBeenCalledOnce());
    expect(mocks.close).not.toHaveBeenCalled();

    mocks.createNewElem.mockResolvedValueOnce("success");
    await userEvent.click(screen.getByRole("button", { name: "Add position" }));

    await waitFor(() => expect(mocks.close).toHaveBeenCalledOnce());
  });

  it("installs a shortcut listener, prevents handled keys, and removes it after capture", async () => {
    const addListener = vi.spyOn(document, "addEventListener");
    const removeListener = vi.spyOn(document, "removeEventListener");
    renderPage();

    await userEvent.click(
      screen.getByRole("button", { name: "Keyboard shortcut (optional)" })
    );

    const keydownCall = addListener.mock.calls.find(
      ([type]) => type === "keydown"
    );
    expect(keydownCall).toBeDefined();
    const listener = keydownCall?.[1];
    const event = new KeyboardEvent("keydown", {
      bubbles: true,
      cancelable: true,
      key: "1",
    });

    fireEvent(document, event);

    expect(event.defaultPrevented).toBe(true);
    await waitFor(() => expect(screen.getByText("1")).toBeInTheDocument());
    expect(removeListener).toHaveBeenCalledWith("keydown", listener);
  });

  it("ignores shortcut events already prevented by another listener", async () => {
    renderPage();
    await userEvent.click(
      screen.getByRole("button", { name: "Keyboard shortcut (optional)" })
    );
    const event = new KeyboardEvent("keydown", {
      bubbles: true,
      cancelable: true,
      key: "1",
    });
    event.preventDefault();

    fireEvent(document, event);

    expect(screen.getByText("Listening for keys…")).toBeInTheDocument();
  });
});
