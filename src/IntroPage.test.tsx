import { render, screen, within } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../tamagui.config";
import IntroPage from "./IntroPage";

const mocks = vi.hoisted(() => ({
  getAvailableDesks: vi.fn(),
  relaunch: vi.fn(),
  removeConfig: vi.fn(),
  retry: vi.fn(),
  useSimpleAsync: vi.fn(),
}));

vi.mock("use-simple-async", () => ({
  default: mocks.useSimpleAsync,
}));

vi.mock("@tauri-apps/plugin-process", () => ({
  relaunch: mocks.relaunch,
}));

vi.mock("./rustUtils", () => ({
  getAvailableDesks: mocks.getAvailableDesks,
  removeConfig: mocks.removeConfig,
}));

const renderIntroPage = () =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <IntroPage />
    </TamaguiProvider>
  );

describe("IntroPage discovery status", () => {
  beforeEach(() => {
    mocks.retry.mockReset();
    mocks.useSimpleAsync.mockReset();
  });

  it("announces scanning as a busy polite status without announcing the spinner", () => {
    mocks.useSimpleAsync.mockReturnValue([
      undefined,
      { error: undefined, loading: true, retry: mocks.retry },
    ]);

    const { container } = renderIntroPage();
    const status = screen.getByRole("status");

    expect(status).toHaveAttribute("aria-live", "polite");
    expect(status).toHaveAttribute("aria-busy", "true");
    expect(status).toHaveTextContent("Searching for Bluetooth devices");
    expect(within(status).queryByRole("img")).not.toBeInTheDocument();
    expect(container.querySelector(".carrot-spinner")).toHaveAttribute(
      "aria-hidden",
      "true"
    );
  });

  it("announces the completed no-results message as a polite status", () => {
    mocks.useSimpleAsync.mockReturnValue([
      [],
      { error: undefined, loading: false, retry: mocks.retry },
    ]);

    renderIntroPage();
    const status = screen.getByRole("status");

    expect(status).toHaveAttribute("aria-live", "polite");
    expect(status).not.toHaveAttribute("aria-busy");
    expect(status).toHaveTextContent("No matching desks found");
  });
});
