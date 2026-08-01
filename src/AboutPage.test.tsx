import { act, render, screen, waitFor } from "@testing-library/react";
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

const deferred = <T,>() => {
  let resolveDeferred!: (value: T | PromiseLike<T>) => void;
  let rejectDeferred!: (reason?: unknown) => void;
  const promise = new Promise<T>((resolve, reject) => {
    resolveDeferred = resolve;
    rejectDeferred = reject;
  });
  return {
    promise,
    reject: rejectDeferred,
    resolve: resolveDeferred,
  };
};

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

const waitForAutostart = async (checked: boolean) => {
  await waitFor(() => {
    expect(autostartSwitch()).toBeEnabled();
    if (checked) {
      expect(autostartSwitch()).toBeChecked();
    } else {
      expect(autostartSwitch()).not.toBeChecked();
    }
  });
};

describe("AboutPage settings", () => {
  beforeEach(() => {
    mocks.disable.mockReset();
    mocks.enable.mockReset();
    mocks.isEnabled.mockReset();
    mocks.openUrl.mockReset();
    mocks.relaunch.mockReset();
    mocks.removeConfig.mockReset();

    mocks.disable.mockResolvedValue(undefined);
    mocks.enable.mockResolvedValue(undefined);
    mocks.isEnabled.mockResolvedValue(false);
    mocks.openUrl.mockResolvedValue(undefined);
    mocks.relaunch.mockResolvedValue(undefined);
    mocks.removeConfig.mockResolvedValue(undefined);
  });

  it("shows a polite visible status while the initial setting read is pending", async () => {
    const read = deferred<boolean>();
    mocks.isEnabled.mockReturnValueOnce(read.promise);

    renderAboutPage();

    expect(screen.getByRole("status")).toHaveTextContent(
      "Reading startup setting…"
    );
    expect(screen.getByRole("status")).toHaveAttribute("aria-live", "polite");
    expect(autostartSwitch()).toBeDisabled();

    await act(async () => read.resolve(false));
    await waitForAutostart(false);
    expect(
      screen.queryByText("Reading startup setting…")
    ).not.toBeInTheDocument();
  });

  it("enables autostart only after a pending native update succeeds", async () => {
    const update = deferred<void>();
    mocks.enable.mockReturnValueOnce(update.promise);
    renderAboutPage();
    await waitForAutostart(false);

    await userEvent.click(autostartSwitch());

    expect(mocks.enable).toHaveBeenCalledOnce();
    expect(autostartSwitch()).toBeDisabled();
    expect(autostartSwitch()).not.toBeChecked();
    expect(screen.getByRole("status")).toHaveTextContent(
      "Updating startup setting…"
    );

    await act(async () => update.resolve());
    await waitForAutostart(true);
    expect(
      screen.queryByText("Updating startup setting…")
    ).not.toBeInTheDocument();
  });

  it("disables autostart only after a pending native update succeeds", async () => {
    const update = deferred<void>();
    mocks.isEnabled.mockResolvedValueOnce(true);
    mocks.disable.mockReturnValueOnce(update.promise);
    renderAboutPage();
    await waitForAutostart(true);

    await userEvent.click(autostartSwitch());

    expect(mocks.disable).toHaveBeenCalledOnce();
    expect(autostartSwitch()).toBeDisabled();
    expect(autostartSwitch()).toBeChecked();
    expect(screen.getByRole("status")).toHaveTextContent(
      "Updating startup setting…"
    );

    await act(async () => update.resolve());
    await waitForAutostart(false);
  });

  it("shows an update error and preserves the prior checked state", async () => {
    const update = deferred<void>();
    mocks.enable.mockReturnValueOnce(update.promise);
    vi.spyOn(console, "error").mockImplementation(() => undefined);
    renderAboutPage();
    await waitForAutostart(false);

    await userEvent.click(autostartSwitch());
    expect(autostartSwitch()).not.toBeChecked();

    await act(async () => update.reject(new Error("permission denied")));
    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent("permission denied")
    );
    expect(autostartSwitch()).toBeEnabled();
    expect(autostartSwitch()).not.toBeChecked();
  });

  it("renders the page and card titles with ordered heading semantics", async () => {
    renderAboutPage();
    await waitForAutostart(false);

    expect(
      screen.getByRole("heading", { level: 1, name: "Options & About" })
    ).toBeInTheDocument();
    expect(
      screen
        .getAllByRole("heading", { level: 2 })
        .map(({ textContent }) => textContent?.trim())
    ).toEqual(["Startup", "Advanced", "About Trayasen"]);
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
