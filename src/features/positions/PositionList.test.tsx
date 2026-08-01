import { render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../../../tamagui.config";
import type { PositionListProps } from "./PositionList";
import { PositionList } from "./PositionList";

const renderList = (props: Partial<PositionListProps> = {}) =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      <PositionList
        loading={false}
        onRemove={vi.fn()}
        positions={[]}
        {...props}
      />
    </TamaguiProvider>
  );

it("shows a loading state", () => {
  renderList({ positions: undefined, loading: true });
  expect(screen.getByRole("status")).toHaveTextContent("Loading positions");
});

it("shows an empty state", () => {
  renderList({ positions: [], loading: false });
  expect(screen.getByText("No saved positions yet")).toBeInTheDocument();
});

it("renders position data and requests removal by name", async () => {
  const onRemove = vi.fn();
  renderList({
    positions: [{ name: "Standing", value: 11200, shortcut: "CmdOrCtrl+1" }],
    loading: false,
    onRemove,
  });

  expect(screen.getByText("11200")).toBeInTheDocument();
  await userEvent.click(
    screen.getByRole("button", { name: "Remove Standing" })
  );
  expect(onRemove).toHaveBeenCalledWith("Standing");
});

it("labels the affected row and disables every removal while removing", () => {
  renderList({
    positions: [
      { name: "Standing", value: 11200, shortcut: "CmdOrCtrl+1" },
      { name: "Sitting", value: 7200 },
    ],
    removingName: "Standing",
  });

  expect(
    screen.getByRole("button", { name: "Removing Standing" })
  ).toHaveAttribute("aria-disabled", "true");
  expect(
    screen.getByRole("button", { name: "Remove Sitting" })
  ).toHaveAttribute("aria-disabled", "true");
  expect(screen.getByText("—")).toBeInTheDocument();
  expect(screen.getByText("Standing")).toHaveAttribute("title", "Standing");
});
