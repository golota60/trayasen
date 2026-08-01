import { render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { Input, TamaguiProvider } from "tamagui";
import appConfig from "../../tamagui.config";
import { AppButton } from "./Button";
import { Alert } from "./Feedback";
import { FormField } from "./FormField";

const renderUi = (node: React.ReactNode) =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      {node}
    </TamaguiProvider>
  );

describe("shared UI", () => {
  it("blocks a loading button and exposes its loading label", async () => {
    const onPress = vi.fn();
    renderUi(
      <AppButton loading loadingLabel="Saving" onPress={onPress}>
        Save
      </AppButton>
    );
    await userEvent.click(screen.getByRole("button", { name: "Saving" }));
    expect(onPress).not.toHaveBeenCalled();
  });

  it("renders errors as alerts", () => {
    renderUi(
      <Alert tone="error" title="Connection failed">
        Try again.
      </Alert>
    );
    expect(screen.getByRole("alert")).toHaveTextContent("Connection failed");
    expect(screen.getByRole("alert")).toHaveTextContent("Try again.");
  });

  it("associates a form error with its input", () => {
    renderUi(
      <FormField id="name" label="Position name" error="Name cannot be empty">
        <Input id="name" />
      </FormField>
    );
    expect(screen.getByLabelText("Position name")).toHaveAccessibleDescription(
      "Name cannot be empty"
    );
  });
});
