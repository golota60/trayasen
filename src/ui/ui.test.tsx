import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { Input, TamaguiProvider } from "tamagui";
import appConfig from "../../tamagui.config";
import { AppButton } from "./Button";
import { ExternalLink } from "./ExternalLink";
import { Alert, CarrotSpinner } from "./Feedback";
import { FormField } from "./FormField";
import { LinkButton } from "./LinkButton";
import { PageShell } from "./PageShell";
import { TechnicalDisclosure } from "./TechnicalDisclosure";

const { openUrlMock } = vi.hoisted(() => ({
  openUrlMock: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: openUrlMock,
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

  it("keeps the default spinner accessible", () => {
    renderUi(<CarrotSpinner />);

    expect(screen.getByRole("img", { name: "Loading" })).toBeInTheDocument();
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

  it("renders page titles as level-one headings", () => {
    renderUi(<PageShell title="Manage positions">Content</PageShell>);

    expect(
      screen.getByRole("heading", { level: 1, name: "Manage positions" })
    ).toBeInTheDocument();
  });

  it("separates the page header from its content", () => {
    renderUi(
      <PageShell title="Manage positions">
        <p>Page content</p>
      </PageShell>
    );

    expect(
      screen.getByRole("heading", { name: "Manage positions" }).closest("header")
    ).not.toBeNull();
    expect(screen.getByText("Page content").closest("main")).not.toBeNull();
  });

  it("constrains and wraps a themed technical disclosure", () => {
    const technicalText = `Native failure: ${"x".repeat(500)}`;
    renderUi(
      <TechnicalDisclosure label="Technical details">
        {technicalText}
      </TechnicalDisclosure>
    );

    const summary = screen.getByText("Technical details");
    const details = summary.closest("details");
    const pre = details?.querySelector("pre");
    expect(details).toHaveAccessibleName("Technical details");
    expect(summary.className).toContain("_col-muted");
    expect(summary.className).toContain("_outlineColor-0focus-accent");
    expect(pre).toHaveTextContent(technicalText);
    expect(pre?.className).toContain("_col-color");
    expect(getComputedStyle(details as HTMLElement).maxWidth).toBe("100%");
    expect(getComputedStyle(pre as HTMLElement).maxWidth).toBe("100%");
    expect(getComputedStyle(pre as HTMLElement).whiteSpace).toBe("pre-wrap");
    expect(getComputedStyle(pre as HTMLElement).overflowWrap).toBe("anywhere");
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
