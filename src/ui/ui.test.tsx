import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import type { AnchorHTMLAttributes, ReactNode } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { Input, TamaguiProvider } from "tamagui";
import appConfig from "../../tamagui.config";
import { AppButton } from "./Button";
import { ExternalLink } from "./ExternalLink";
import { Alert } from "./Feedback";
import { FormField } from "./FormField";
import { LinkButton } from "./LinkButton";

const { openUrlMock } = vi.hoisted(() => ({
  openUrlMock: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: openUrlMock,
}));

vi.mock("found", () => ({
  Link: ({
    to,
    children,
    ...props
  }: AnchorHTMLAttributes<HTMLAnchorElement> & {
    to: string;
    children: ReactNode;
  }) => (
    <a {...props} href={to}>
      {children}
    </a>
  ),
}));

const renderUi = (node: React.ReactNode) =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      {node}
    </TamaguiProvider>
  );

describe("shared UI", () => {
  beforeEach(() => {
    openUrlMock.mockReset();
    openUrlMock.mockResolvedValue(undefined);
  });

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

  it("keeps loading buttons accessible without a loading label", () => {
    const { rerender } = renderUi(
      <AppButton aria-label="Save changes" loading>
        Save
      </AppButton>
    );
    expect(
      screen.getByRole("button", { name: "Save changes" })
    ).toHaveAttribute("aria-disabled", "true");

    rerender(
      <TamaguiProvider config={appConfig} defaultTheme="dark">
        <AppButton loading>Save</AppButton>
      </TamaguiProvider>
    );
    expect(screen.getByRole("button", { name: "Loading" })).toHaveAttribute(
      "aria-disabled",
      "true"
    );
  });

  it("uses alert only for errors and status for other feedback", () => {
    renderUi(
      <>
        <Alert tone="error" title="Connection failed">
          Try again.
        </Alert>
        <Alert tone="success" title="Saved" />
        <Alert tone="info" title="Working" />
      </>
    );
    expect(screen.getByRole("alert")).toHaveTextContent("Connection failed");
    expect(screen.getByRole("alert")).toHaveTextContent("Try again.");
    expect(screen.getAllByRole("status")).toHaveLength(2);
    expect(screen.getByText("Saved").closest("[role=status]")).not.toBeNull();
    expect(screen.getByText("Working").closest("[role=status]")).not.toBeNull();
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

  it("lets an ExternalLink caller cancel opening", () => {
    const onClick = vi.fn((event: React.MouseEvent<HTMLAnchorElement>) => {
      event.preventDefault();
    });
    renderUi(
      <ExternalLink href="https://example.com" onClick={onClick}>
        Documentation
      </ExternalLink>
    );

    fireEvent.click(screen.getByRole("link", { name: "Documentation" }));

    expect(onClick).toHaveBeenCalledOnce();
    expect(openUrlMock).not.toHaveBeenCalled();
  });

  it("leaves non-HTTP ExternalLink URLs to the browser", () => {
    renderUi(<ExternalLink href="#details">Details</ExternalLink>);
    const link = screen.getByRole("link", { name: "Details" });
    const event = new MouseEvent("click", { bubbles: true, cancelable: true });

    expect(link.dispatchEvent(event)).toBe(true);
    expect(event.defaultPrevented).toBe(false);
    expect(openUrlMock).not.toHaveBeenCalled();
  });

  it("prevents HTTP navigation and delegates to the external opener", async () => {
    renderUi(
      <ExternalLink href="https://example.com/docs">Documentation</ExternalLink>
    );
    const link = screen.getByRole("link", { name: "Documentation" });
    const event = new MouseEvent("click", { bubbles: true, cancelable: true });

    expect(link.dispatchEvent(event)).toBe(false);
    expect(event.defaultPrevented).toBe(true);
    expect(openUrlMock).toHaveBeenCalledWith("https://example.com/docs");
    await waitFor(() => expect(openUrlMock).toHaveBeenCalledOnce());
  });

  it("logs ExternalLink opener failures", async () => {
    const error = new Error("opener unavailable");
    const consoleError = vi
      .spyOn(console, "error")
      .mockImplementation(() => {});
    openUrlMock.mockRejectedValueOnce(error);
    renderUi(<ExternalLink href="http://example.com">Example</ExternalLink>);

    fireEvent.click(screen.getByRole("link", { name: "Example" }));

    await waitFor(() =>
      expect(consoleError).toHaveBeenCalledWith(
        "Could not open external URL http://example.com",
        error
      )
    );
    consoleError.mockRestore();
  });

  it("attaches explicit focus outline styles to ExternalLink", () => {
    renderUi(<ExternalLink href="#details">Details</ExternalLink>);
    const className = screen.getByRole("link", { name: "Details" }).className;

    expect(className).toContain("_outlineColor-0focus-accent");
    expect(className).toContain("_outlineStyle-0focus-solid");
    expect(className).toContain("_outlineWidth-0focus-2px");
    expect(className).toContain("_outlineOffset-0focus-2px");
  });

  it("applies a deterministic orange focus outline to LinkButton", () => {
    renderUi(<LinkButton to="/settings">Settings</LinkButton>);
    const link = screen.getByRole("link", { name: "Settings" });

    expect(link.style.outlineColor).toBe("transparent");
    expect(link.style.outlineOffset).toBe("2px");
    expect(link.style.outlineStyle).toBe("solid");
    expect(link.style.outlineWidth).toBe("2px");

    fireEvent.focus(link);
    expect(link.style.outlineColor).toBe("rgb(242, 140, 40)");
    expect(link.style.outlineOffset).toBe("2px");
    expect(link.style.outlineStyle).toBe("solid");
    expect(link.style.outlineWidth).toBe("2px");
  });
});
