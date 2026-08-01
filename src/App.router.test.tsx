import { render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import App from "./App";

const mocks = vi.hoisted(() => ({
  close: vi.fn(),
  useSimpleAsync: vi.fn(),
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ close: mocks.close }),
}));

vi.mock("use-simple-async", () => ({
  default: mocks.useSimpleAsync,
}));

vi.mock("./rustUtils", () => ({
  connectToDesk: vi.fn(),
  createNewElem: vi.fn(),
  getAvailableDesks: vi.fn(),
  getPositions: vi.fn(),
  removeConfig: vi.fn(),
  removePosition: vi.fn(),
  resetDesk: vi.fn(),
}));

const renderApp = () =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <App />
    </TamaguiProvider>
  );

describe("real Wouter browser routing under React 19", () => {
  beforeEach(() => {
    window.history.replaceState({}, "", "/manage-positions");
    mocks.useSimpleAsync.mockReset();
    mocks.useSimpleAsync.mockReturnValue([
      { saved_positions: [] },
      { error: undefined, loading: false, retry: vi.fn() },
    ]);
  });

  it("responds to popstate and navigates a route link with browser history", async () => {
    renderApp();

    expect(
      screen.getByRole("heading", { level: 1, name: "Manage positions" })
    ).toBeInTheDocument();

    window.history.pushState({}, "", "/about");
    window.dispatchEvent(new PopStateEvent("popstate"));
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { level: 1, name: "Options & About" })
      ).toBeInTheDocument()
    );

    window.history.pushState({}, "", "/manage-positions");
    window.dispatchEvent(new PopStateEvent("popstate"));
    const routeLink = await screen.findByRole("link", { name: "New position" });
    await userEvent.click(routeLink);

    await waitFor(() => {
      expect(window.location.pathname).toBe("/new-position");
      expect(
        screen.getByRole("heading", { level: 1, name: "Add a new position" })
      ).toBeInTheDocument();
    });
  });
});
