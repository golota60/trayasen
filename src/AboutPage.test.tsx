import { render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import AboutPage from "./AboutPage";

const mocks = vi.hoisted(() => ({
  disable: vi.fn(),
  enable: vi.fn(),
  isEnabled: vi.fn(),
  openUrl: vi.fn(),
  relaunch: vi.fn(),
  removeConfig: vi.fn(),
  useSimpleAsync: vi.fn(),
}));

vi.mock("use-simple-async", () => ({
  default: mocks.useSimpleAsync,
}));

vi.mock("@tauri-apps/plugin-autostart", () => ({
  disable: mocks.disable,
  enable: mocks.enable,
  isEnabled: mocks.isEnabled,
}));

vi.mock("@tauri-apps/plugin-process", () => ({
  relaunch: mocks.relaunch,
}));

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: mocks.openUrl,
}));

vi.mock("./rustUtils", () => ({
  removeConfig: mocks.removeConfig,
}));

const renderAboutPage = () =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <AboutPage />
    </TamaguiProvider>
  );

const autostartSwitch = () =>
  screen.getByRole("switch", {
    name: "Open Trayasen when the system starts",
  });

describe("AboutPage settings", () => {
  beforeEach(() => {
    mocks.disable.mockReset();
    mocks.enable.mockReset();
    mocks.isEnabled.mockReset();
    mocks.openUrl.mockReset();
    mocks.relaunch.mockReset();
    mocks.removeConfig.mockReset();
    mocks.useSimpleAsync.mockReset();

    mocks.disable.mockResolvedValue(undefined);
    mocks.enable.mockResolvedValue(undefined);
    mocks.openUrl.mockResolvedValue(undefined);
    mocks.relaunch.mockResolvedValue(undefined);
    mocks.removeConfig.mockResolvedValue(undefined);
    mocks.useSimpleAsync.mockReturnValue([
      false,
      { error: undefined, loading: false, retry: vi.fn() },
    ]);
  });

  it("renders a checked switch when autostart is enabled upstream", async () => {
    mocks.useSimpleAsync.mockReturnValue([
      true,
      { error: undefined, loading: false, retry: vi.fn() },
    ]);

    renderAboutPage();

    await waitFor(() => expect(autostartSwitch()).toBeChecked());
  });

  it("disables autostart and updates the checked state", async () => {
    mocks.useSimpleAsync.mockReturnValue([
      true,
      { error: undefined, loading: false, retry: vi.fn() },
    ]);
    renderAboutPage();

    await waitFor(() => expect(autostartSwitch()).toBeChecked());
    await userEvent.click(autostartSwitch());

    expect(mocks.disable).toHaveBeenCalledOnce();
    await waitFor(() => expect(autostartSwitch()).not.toBeChecked());
  });

  it("enables autostart and updates the checked state", async () => {
    renderAboutPage();

    await waitFor(() => expect(autostartSwitch()).not.toBeChecked());
    await userEvent.click(autostartSwitch());

    expect(mocks.enable).toHaveBeenCalledOnce();
    await waitFor(() => expect(autostartSwitch()).toBeChecked());
  });

  it("shows an autostart error and re-enables the switch", async () => {
    let rejectEnable: (error: Error) => void = () => undefined;
    mocks.enable.mockReturnValueOnce(
      new Promise<void>((_resolve, reject) => {
        rejectEnable = reject;
      })
    );
    const consoleError = vi
      .spyOn(console, "error")
      .mockImplementation(() => undefined);
    renderAboutPage();

    await userEvent.click(autostartSwitch());

    expect(autostartSwitch()).toBeDisabled();
    rejectEnable(new Error("permission denied"));
    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent("permission denied")
    );
    expect(autostartSwitch()).toBeEnabled();
    expect(autostartSwitch()).not.toBeChecked();
    consoleError.mockRestore();
  });

  it("removes config before relaunching", async () => {
    renderAboutPage();

    await userEvent.click(
      screen.getByRole("button", { name: "Reset config and restart" })
    );

    await waitFor(() => expect(mocks.relaunch).toHaveBeenCalledOnce());
    expect(mocks.removeConfig).toHaveBeenCalledOnce();
    expect(mocks.removeConfig.mock.invocationCallOrder[0]).toBeLessThan(
      mocks.relaunch.mock.invocationCallOrder[0]
    );
  });

  it("opens the HTTP project link with the Tauri opener", async () => {
    renderAboutPage();

    await userEvent.click(screen.getByRole("link", { name: "GitHub" }));

    expect(mocks.openUrl).toHaveBeenCalledWith(
      "https://github.com/golota60/trayasen"
    );
  });
});
