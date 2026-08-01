import { render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import App, { appRoutes } from "./App";

const mocks = vi.hoisted(() => ({
  connectToDesk: vi.fn(),
  relaunch: vi.fn(),
  removeConfig: vi.fn(),
  resetDesk: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-process", () => ({
  relaunch: mocks.relaunch,
}));

vi.mock("./rustUtils", () => ({
  connectToDesk: mocks.connectToDesk,
  removeConfig: mocks.removeConfig,
  resetDesk: mocks.resetDesk,
}));

vi.mock("./AboutPage", () => ({ default: () => null }));
vi.mock("./IntroPage", () => ({ default: () => null }));
vi.mock("./ManagePositionsPage", () => ({ default: () => null }));
vi.mock("./NewPositionPage", () => ({ default: () => null }));

type RecoveryState = {
  title: string;
  description: string;
  error: string;
  desk_name?: string;
};

const setRecoveryState = (state: RecoveryState) => {
  Object.defineProperty(window, "stateWorkaround", {
    configurable: true,
    value: state,
    writable: true,
  });
};

const renderApp = () =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <App />
    </TamaguiProvider>
  );

describe("application recovery route", () => {
  beforeEach(() => {
    window.history.replaceState({}, "", "/error");
    mocks.connectToDesk.mockReset();
    mocks.relaunch.mockReset();
    mocks.removeConfig.mockReset();
    mocks.resetDesk.mockReset();
    mocks.relaunch.mockResolvedValue(undefined);
    mocks.removeConfig.mockResolvedValue(undefined);
    mocks.resetDesk.mockResolvedValue(undefined);
    setRecoveryState({
      title: "Could not connect",
      description: "Trayasen could not reconnect to your saved desk.",
      error: "Adapter did not find the desk",
      desk_name: "Desk 1234",
    });
  });

  it("keeps all six application routes and renders the configured recovery route", () => {
    renderApp();

    expect(appRoutes.map(({ path }) => path)).toEqual([
      "/error",
      "/about",
      "/new-position",
      "/manage-positions",
      "/intro",
      "/*",
    ]);
    expect(screen.getByText("Could not connect")).toBeInTheDocument();
  });

  it("names and contains long technical recovery details", () => {
    const longError = `Native adapter failure: ${"A".repeat(500)}`;
    setRecoveryState({
      title: "Could not connect",
      description: "Trayasen could not reconnect to your saved desk.",
      error: longError,
      desk_name: "Desk 1234",
    });

    renderApp();

    const disclosure = screen.getByText("Technical details").closest("details");
    expect(disclosure).toHaveAccessibleName("Technical details");
    expect(disclosure?.querySelector("pre")).toHaveTextContent(longError);
  });

  it("retries a saved desk and relaunches only after connection succeeds", async () => {
    mocks.connectToDesk.mockResolvedValue(undefined);
    renderApp();

    await userEvent.click(screen.getByRole("button", { name: "Try again" }));

    expect(mocks.connectToDesk).toHaveBeenCalledWith("Desk 1234");
    expect(mocks.relaunch).toHaveBeenCalledOnce();
  });

  it("shows retry failure and does not relaunch", async () => {
    mocks.connectToDesk.mockRejectedValue(new Error("Bluetooth unavailable"));
    renderApp();

    await userEvent.click(screen.getByRole("button", { name: "Try again" }));

    expect(screen.getByRole("alert")).toHaveTextContent(
      "Bluetooth unavailable"
    );
    expect(mocks.relaunch).not.toHaveBeenCalled();
  });

  it("forgets a saved desk before relaunching setup", async () => {
    renderApp();

    await userEvent.click(
      screen.getByRole("button", { name: "Forget desk and restart setup" })
    );

    expect(mocks.resetDesk).toHaveBeenCalledOnce();
    expect(mocks.removeConfig).not.toHaveBeenCalled();
    expect(mocks.relaunch).toHaveBeenCalledOnce();
  });

  it("omits retry and resets config when no saved desk is available", async () => {
    setRecoveryState({
      title: "Configuration error",
      description: "Trayasen could not load its configuration.",
      error: "Invalid configuration",
    });
    renderApp();

    expect(
      screen.queryByRole("button", { name: "Try again" })
    ).not.toBeInTheDocument();

    await userEvent.click(
      screen.getByRole("button", { name: "Reset config and restart" })
    );

    expect(mocks.removeConfig).toHaveBeenCalledOnce();
    expect(mocks.resetDesk).not.toHaveBeenCalled();
    expect(mocks.relaunch).toHaveBeenCalledOnce();
  });

  it("shows reset failure without relaunching", async () => {
    mocks.resetDesk.mockRejectedValue(new Error("Reset unavailable"));
    const consoleError = vi
      .spyOn(console, "error")
      .mockImplementation(() => undefined);
    renderApp();

    await userEvent.click(
      screen.getByRole("button", { name: "Forget desk and restart setup" })
    );

    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent("Reset unavailable")
    );
    expect(mocks.relaunch).not.toHaveBeenCalled();
    expect(consoleError).toHaveBeenCalledWith(
      "Could not reset and relaunch Trayasen",
      expect.any(Error)
    );
  });
});
