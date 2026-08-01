import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import ManagePositionsPage from "./ManagePositionsPage";

const mocks = vi.hoisted(() => ({
  close: vi.fn(),
  getPositions: vi.fn(),
  removePosition: vi.fn(),
  retry: vi.fn(),
  useSimpleAsync: vi.fn(),
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ close: mocks.close }),
}));

vi.mock("use-simple-async", () => ({
  default: mocks.useSimpleAsync,
}));

vi.mock("./rustUtils", () => ({
  getPositions: mocks.getPositions,
  removePosition: mocks.removePosition,
}));

const renderPage = () =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <ManagePositionsPage />
    </TamaguiProvider>
  );

describe("ManagePositionsPage", () => {
  beforeEach(() => {
    mocks.useSimpleAsync.mockReset();
    mocks.useSimpleAsync.mockReturnValue([
      undefined,
      {
        error: new Error("configuration unavailable"),
        loading: false,
        retry: mocks.retry,
      },
    ]);
  });

  it("shows load rejection feedback without a misleading empty state", () => {
    renderPage();

    expect(screen.getByRole("alert")).toHaveTextContent(
      "configuration unavailable"
    );
    expect(
      screen.queryByText("No saved positions yet")
    ).not.toBeInTheDocument();
  });

  it("renders the page title as a level-one heading", () => {
    renderPage();

    expect(
      screen.getByRole("heading", { level: 1, name: "Manage positions" })
    ).toBeInTheDocument();
  });
});
