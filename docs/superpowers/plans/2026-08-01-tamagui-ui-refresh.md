# Tamagui UI Refresh Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace Trayasen's Tailwind/shadcn/Radix interface with a polished, dark-only Tamagui UI and simplify the existing workflows without changing native command contracts or persisted data.

**Architecture:** A root `TamaguiProvider` supplies one dark theme and shared tokens. Small components under `src/ui/` define the page shell, buttons, cards, form feedback, links, and status displays; page components retain ownership of Tauri calls and local async state. Pure helpers and prop-driven view components isolate shortcut, validation, device-filtering, and position-list behavior for focused tests.

**Tech Stack:** React 19, TypeScript 5, Vite 8, Tamagui 2.6.2, Tauri 2, Found router, Vitest 4, Testing Library, Lucide React

## Global Constraints

- The application is dark-only; do not add light-theme switching.
- Use deep charcoal/navy surfaces, restrained borders, off-white text, carrot-orange accents, and red only for destructive/error treatment.
- Tamagui fully replaces Tailwind, shadcn-style primitives, Radix UI, class-variance-authority, clsx, tailwind-merge, and Tailwind build configuration.
- Keep Lucide React for icons.
- Preserve routes, persisted-data formats, Tauri command signatures, desk-control behavior, and successful position creation closing the window.
- Change the native default window from 1280×720 to exactly 800×600 on every platform.
- Keep and restyle the 30px Windows custom title bar; retain native decorations on macOS and Linux.
- Async actions must clear stale action errors, expose progress, block duplicate/conflicting actions, update success only after resolution, and retain actionable failure text.
- Make the smallest reasonable autonomous decision when a local assumption is incorrect; consult the user only for conflicts involving scope, user-visible behavior, persisted data, native contracts, platform support, destructive behavior, or the approved architecture/visual direction.
- Use the exact existing height bounds: `MIN_HEIGHT = 6200` and `MAX_HEIGHT = 12700`.

---

## File Structure

### New files

- `tamagui.config.ts` — tokens, media configuration, and the single `dark` theme.
- `vitest.config.ts` — jsdom test configuration.
- `src/test/setup.ts` — Testing Library matchers and test cleanup.
- `src/ui/Button.tsx` — primary, secondary, ghost, and destructive button treatments with loading behavior.
- `src/ui/Card.tsx` — reusable elevated content surface.
- `src/ui/PageShell.tsx` — responsive viewport, scrolling, branding, title, description, and action layout.
- `src/ui/Feedback.tsx` — spinner, alert, and loading/empty state presentation.
- `src/ui/FormField.tsx` — accessible label, helper text, and field error composition.
- `src/ui/ExternalLink.tsx` — Tamagui-styled external URL handling through Tauri opener.
- `src/ui/LinkButton.tsx` — accessible Found-router link styled as an application action.
- `src/ui/ui.test.tsx` — shared UI behavior and semantics.
- `src/features/devices/deviceFilters.ts` — pure filtered/all device selection.
- `src/features/devices/deviceFilters.test.ts` — device filtering tests.
- `src/features/positions/positionForm.ts` — pure position validation and shortcut-state transitions.
- `src/features/positions/positionForm.test.ts` — validation and shortcut tests.
- `src/features/positions/PositionList.tsx` — prop-driven loading/empty/list/error rendering.
- `src/features/positions/PositionList.test.tsx` — position-list state tests.

### Modified files

- `package.json`, `package-lock.json` — Tamagui/test dependencies, scripts, and legacy dependency removal.
- `src/main.tsx` — root provider and custom-decoration document state.
- `src/style.css` — minimal browser reset, scrollbars, and matching Windows title bar only.
- `index.html` — Trayasen metadata, dark loading fallback, and accessible title-bar buttons.
- `src/App.tsx` — page root and consistent returning-user recovery UI.
- `src/IntroPage.tsx`, `src/DeskElement.tsx` — discovery workflow and successful-connection semantics.
- `src/NewPositionPage.tsx` — structured form and shortcut recorder.
- `src/ManagePositionsPage.tsx` — shared list and fixed page actions.
- `src/AboutPage.tsx` — settings, destructive reset, and project-information cards.
- `src-tauri/src/main.rs` — default 800×600 window size and its unit assertion.
- `README.md` — replace the shadcn frontend note with Tamagui.

### Removed files

- `postcss.config.cjs`, `tailwind.config.cjs`.
- `src/generic/button.tsx`, `checkbox.tsx`, `input.tsx`, `label.tsx`, `tooltip.tsx`, `Spinner.tsx`, `Href.tsx`, and `lib/utils.ts`.
- `src/assets/cross.svg` after Lucide's `Trash2` replaces it.

---

### Task 1: Establish Tamagui and the frontend test harness

**Files:**
- Create: `tamagui.config.ts`
- Create: `vitest.config.ts`
- Create: `src/test/setup.ts`
- Create: `src/test/tamagui-config.test.ts`
- Modify: `package.json`
- Modify: `package-lock.json`
- Modify: `src/main.tsx`
- Modify: `src/style.css`
- Modify: `index.html`

**Interfaces:**
- Produces: default export `appConfig` from `tamagui.config.ts`; module augmentation `TamaguiCustomConfig`; `npm test` command; `TamaguiProvider` around `<App />`.
- Consumes: existing React root and `hasCustomDecorations(): Promise<unknown>`.

- [ ] **Step 1: Install the compatible Tamagui and test dependencies**

Run:

```bash
npm install react@19.2.8 react-dom@19.2.8 lucide-react@1.28.0
npm install --save-dev @types/react@19.2.18 @types/react-dom@19.2.4
npm install tamagui@2.6.2 @tamagui/config@2.6.2
npm install --save-dev vitest@4.0.18 jsdom@26.1.0 @testing-library/dom@10.4.1 @testing-library/react@16.3.2 @testing-library/jest-dom@6.9.1 @testing-library/user-event@14.6.1
npm pkg set scripts.test="vitest run"
```

Expected: `package.json` and `package-lock.json` contain current pinned Tamagui packages and a `test` script. Tamagui 2 requires React 19, so React, React DOM, their type packages, and Lucide are upgraded together. Found and `use-simple-async` both declare support for React versions `>=16.8.0`; route and async behavior are covered by later tests and the production build.

- [ ] **Step 2: Add the test harness and write the failing theme contract test**

Create `vitest.config.ts`:

```ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    restoreMocks: true,
  },
});
```

Create `src/test/setup.ts`:

```ts
import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach, vi } from "vitest";

afterEach(cleanup);

Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

class TestResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
}

globalThis.ResizeObserver = TestResizeObserver as unknown as typeof ResizeObserver;
```

Create `src/test/tamagui-config.test.ts`:

```ts
import { describe, expect, it } from "vitest";
import { darkTheme } from "../../tamagui.config";

describe("dark Tamagui theme", () => {
  it("uses the approved dark palette and carrot accent", () => {
    expect(darkTheme.background).toBe("#0B0F14");
    expect(darkTheme.backgroundStrong).toBe("#111823");
    expect(darkTheme.color).toBe("#F5F7FA");
    expect(darkTheme.accent).toBe("#F28C28");
    expect(darkTheme.error).toBe("#F97066");
  });
});
```

- [ ] **Step 3: Run the theme test and confirm the missing-config failure**

Run: `npm test -- src/test/tamagui-config.test.ts`

Expected: FAIL because `tamagui.config.ts` does not exist.

- [ ] **Step 4: Create the Tamagui configuration**

Create `tamagui.config.ts` with a plain exported theme contract and Tamagui configuration:

```ts
import { config as baseConfig } from "@tamagui/config/v3";
import { createTamagui } from "tamagui";

export const darkTheme = {
  background: "#0B0F14",
  backgroundHover: "#121A25",
  backgroundPress: "#182231",
  backgroundFocus: "#182231",
  backgroundStrong: "#111823",
  backgroundTransparent: "rgba(11, 15, 20, 0)",
  color: "#F5F7FA",
  colorHover: "#FFFFFF",
  colorPress: "#FFFFFF",
  colorFocus: "#FFFFFF",
  colorTransparent: "rgba(245, 247, 250, 0)",
  borderColor: "#263241",
  borderColorHover: "#344458",
  borderColorPress: "#F28C28",
  borderColorFocus: "#F28C28",
  accent: "#F28C28",
  accentHover: "#FFA447",
  accentPress: "#D97716",
  accentColor: "#17100A",
  muted: "#98A5B5",
  error: "#F97066",
  errorBackground: "#321B1D",
  success: "#67D59C",
  successBackground: "#123024",
};

const appConfig = createTamagui({
  ...baseConfig,
  themes: { dark: darkTheme },
  settings: {
    ...baseConfig.settings,
    defaultFont: "body",
  },
});

export type AppConfig = typeof appConfig;

declare module "tamagui" {
  interface TamaguiCustomConfig extends AppConfig {}
}

export default appConfig;
```

If Tamagui's current type requires additional theme keys from `baseConfig.themes.dark`, spread that theme first and overwrite the exact keys above; this is a local compatibility decision and must not alter the asserted palette.

- [ ] **Step 5: Wrap the application and expose custom-decoration state**

In `src/main.tsx`, import `TamaguiProvider` and `appConfig`, set `document.documentElement.dataset.customDecorations` after `hasCustomDecorations()` resolves, and render:

```tsx
<TamaguiProvider config={appConfig} defaultTheme="dark">
  <App />
</TamaguiProvider>
```

Keep the existing title-bar event registration. Set the dataset to `"true"` before registering Windows handlers and to `"false"` before removing the title bar on other platforms.

- [ ] **Step 6: Replace global Tailwind CSS with the minimal browser/title-bar CSS**

Rewrite `src/style.css` to include only:

```css
* {
  box-sizing: border-box;
  scrollbar-width: thin;
  scrollbar-color: #344458 transparent;
}

html,
body,
#root {
  width: 100%;
  height: 100%;
  margin: 0;
  overflow: hidden;
  background: #0b0f14;
}

body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

*::-webkit-scrollbar { width: 6px; height: 6px; }
*::-webkit-scrollbar-track { background: transparent; }
*::-webkit-scrollbar-thumb { background: #344458; border-radius: 999px; }

.titlebar {
  position: fixed;
  inset: 0 0 auto;
  z-index: 1000;
  display: flex;
  justify-content: flex-end;
  height: 30px;
  background: #111823;
  border-bottom: 1px solid #263241;
  user-select: none;
}

.titlebar-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 42px;
  height: 30px;
  padding: 0;
  border: 0;
  background: transparent;
}

.titlebar-button:hover { background: #263241; }
#titlebar-close:hover { background: #b4232f; }
.titlebar-button img { width: 14px; height: 14px; }
html[data-custom-decorations="true"] #root { padding-top: 30px; }
```

- [ ] **Step 7: Correct document metadata and title-bar semantics**

In `index.html`, set `<title>Trayasen</title>`, use `/carrot.png` as the icon, remove inline body styles, and change each `.titlebar-button` wrapper from a `div` to `<button type="button" aria-label="Minimize window">`, `Maximize window`, and `Close window` while preserving IDs and image assets. Keep the loading fallback but style it with a small inline dark-safe declaration because it renders before Tamagui mounts.

- [ ] **Step 8: Run foundation verification**

Run:

```bash
npm test -- src/test/tamagui-config.test.ts
npm run build
```

Expected: theme test PASS and production build PASS.

- [ ] **Step 9: Commit the foundation**

```bash
git add package.json package-lock.json tamagui.config.ts vitest.config.ts src/test src/main.tsx src/style.css index.html
git commit -m "feat: establish Tamagui UI foundation"
```

---

### Task 2: Build the shared Tamagui UI layer

**Files:**
- Create: `src/ui/Button.tsx`
- Create: `src/ui/Card.tsx`
- Create: `src/ui/PageShell.tsx`
- Create: `src/ui/Feedback.tsx`
- Create: `src/ui/FormField.tsx`
- Create: `src/ui/ExternalLink.tsx`
- Create: `src/ui/LinkButton.tsx`
- Create: `src/ui/ui.test.tsx`

**Interfaces:**
- Produces: `AppButton`, `AppButtonProps`, `SurfaceCard`, `PageShell`, `Alert`, `StatePanel`, `CarrotSpinner`, `FormField`, `ExternalLink`, and `LinkButton`.
- `AppButtonProps`: Tamagui `ButtonProps` plus `variant?: "primary" | "secondary" | "ghost" | "destructive"`, `loading?: boolean`, and `loadingLabel?: string`.
- `Alert`: `{ tone: "error" | "success" | "info"; title: string; children?: ReactNode }`.
- `StatePanel`: `{ icon?: ReactNode; title: string; description?: string; action?: ReactNode }`.
- `FormField`: `{ id: string; label: string; helper?: string; error?: string; children: ReactElement }`.
- `LinkButton`: `{ to: string; children: ReactNode; variant?: "primary" | "secondary" }`.

- [ ] **Step 1: Write shared UI behavior tests**

Create `src/ui/ui.test.tsx` with these cases:

```tsx
import { render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { TamaguiProvider } from "tamagui";
import appConfig from "../../tamagui.config";
import { AppButton } from "./Button";
import { Alert } from "./Feedback";
import { FormField } from "./FormField";
import { Input } from "tamagui";

const renderUi = (node: React.ReactNode) =>
  render(
    <TamaguiProvider config={appConfig} defaultTheme="dark">
      {node}
    </TamaguiProvider>
  );

describe("shared UI", () => {
  it("blocks a loading button and exposes its loading label", async () => {
    const onPress = vi.fn();
    renderUi(<AppButton loading loadingLabel="Saving" onPress={onPress}>Save</AppButton>);
    await userEvent.click(screen.getByRole("button", { name: "Saving" }));
    expect(onPress).not.toHaveBeenCalled();
  });

  it("renders errors as alerts", () => {
    renderUi(<Alert tone="error" title="Connection failed">Try again.</Alert>);
    expect(screen.getByRole("alert")).toHaveTextContent("Connection failed");
    expect(screen.getByRole("alert")).toHaveTextContent("Try again.");
  });

  it("associates a form error with its input", () => {
    renderUi(
      <FormField id="name" label="Position name" error="Name cannot be empty">
        <Input id="name" />
      </FormField>
    );
    expect(screen.getByLabelText("Position name")).toHaveAccessibleDescription("Name cannot be empty");
  });
});
```

- [ ] **Step 2: Run the shared UI test and confirm missing-module failures**

Run: `npm test -- src/ui/ui.test.tsx`

Expected: FAIL because the `src/ui` components do not exist.

- [ ] **Step 3: Implement `AppButton`, `SurfaceCard`, and feedback components**

Use Tamagui `styled`, `Button`, `XStack`, `YStack`, `Text`, `Spinner`, and `Image`. `AppButton` must map variants to the approved palette, set `disabled={disabled || loading}`, preserve `onPress`, expose `aria-label={loading ? loadingLabel : ariaLabel}`, replace its content with a small spinner plus `loadingLabel` while pending, and use a visible carrot-orange focus ring. Disabled controls must combine reduced opacity with disabled semantics rather than relying on color alone.

`SurfaceCard` must be a `YStack` with `$backgroundStrong`, one-pixel `$borderColor`, radius `$6`, padding `$5`, and gap `$4`.

`Alert` must use `role="alert"` only for `tone="error"`; success and info use `role="status"`. `CarrotSpinner` must rotate `/carrot.png`, expose `aria-label`, and support `size?: "sm" | "md" | "lg"`. `StatePanel` centers an icon, title, description, and optional action without owning async behavior.

- [ ] **Step 4: Implement shell, field, and navigation components**

`PageShell` must render a full-height `ScrollView`, a centered `YStack` with `maxWidth={960}`, responsive padding (`$4` narrow, `$7` otherwise), a compact carrot mark, title, optional description, page children, and optional actions. Use Tamagui media props to collapse action rows under `$sm`.

`FormField` must clone its single form child to add `id`, `aria-invalid`, and `aria-describedby`. Render helper/error text with stable IDs `${id}-helper` and `${id}-error`; use the error ID when an error exists.

`ExternalLink` must preserve the existing behavior: call a supplied `onClick`, ignore prevented/non-HTTP events, prevent browser navigation for HTTP(S), and call `openUrl(href)` with a logged failure fallback.

`LinkButton` must render a Found `<Link>` as the only interactive element and place a styled Tamagui `XStack`/`Text` inside it. Do not nest a `<button>` inside an anchor.

- [ ] **Step 5: Run shared UI tests and build**

Run:

```bash
npm test -- src/ui/ui.test.tsx
npm run build
```

Expected: all shared UI tests PASS and production build PASS.

- [ ] **Step 6: Commit the UI layer**

```bash
git add src/ui
git commit -m "feat: add shared Tamagui components"
```

---

### Task 3: Rebuild the application shell and recovery page

**Files:**
- Create: `src/App.test.tsx`
- Modify: `src/App.tsx`

**Interfaces:**
- Consumes: `PageShell`, `SurfaceCard`, `Alert`, `AppButton`, `CarrotSpinner`; existing `connectToDesk`, `removeConfig`, `resetDesk`, and `relaunch` functions.
- Produces: unchanged route table and a consistent returning-user recovery flow.

- [ ] **Step 1: Write recovery behavior tests with native calls mocked**

In `src/App.test.tsx`, mock `found`'s browser router to render the configured `/error` component, mock all four native actions, and set `window.stateWorkaround`. Cover:

```tsx
it("retries a saved desk and relaunches only after connection succeeds", async () => {
  connectToDesk.mockResolvedValue(undefined);
  render(<App />);
  await userEvent.click(screen.getByRole("button", { name: "Try again" }));
  expect(connectToDesk).toHaveBeenCalledWith("Desk 1234");
  expect(relaunch).toHaveBeenCalledOnce();
});

it("shows retry failure and does not relaunch", async () => {
  connectToDesk.mockRejectedValue(new Error("Bluetooth unavailable"));
  render(<App />);
  await userEvent.click(screen.getByRole("button", { name: "Try again" }));
  expect(screen.getByRole("alert")).toHaveTextContent("Bluetooth unavailable");
  expect(relaunch).not.toHaveBeenCalled();
});
```

Also assert that a state without `desk_name` omits retry and labels its reset action `Reset config and restart`.

- [ ] **Step 2: Run the App test against the old recovery UI**

Run: `npm test -- src/App.test.tsx`

Expected: FAIL because the old UI lacks the new accessible structure/copy.

- [ ] **Step 3: Rebuild `ReturningUserErrorPage` with shared components**

Keep existing action logic but replace inline styles and Tailwind classes. Use `PageShell` and one `SurfaceCard`; show the summary and description in an error `Alert`; place retry first and destructive reset second. Put technical content in native `<details><summary>Technical details</summary><pre>…</pre></details>` inside the card. Keep separate retry and reset errors, and show the full-page carrot spinner only while retrying.

Use these concise labels:

- Retry: `Try again`
- Saved-desk reset: `Forget desk and restart setup`
- Config-only reset: `Reset config and restart`
- Pending reset: `Resetting`

- [ ] **Step 4: Apply `PageShell` at the application root without changing routes**

Remove the Tailwind wrapper from `App`. Keep the same six route entries and `createBrowserRouter({ routeConfig })`. The router is responsible for selecting pages; each page supplies its own `PageShell`.

- [ ] **Step 5: Run recovery tests and build**

Run:

```bash
npm test -- src/App.test.tsx
npm run build
```

Expected: tests PASS and build PASS.

- [ ] **Step 6: Commit the shell and recovery page**

```bash
git add src/App.tsx src/App.test.tsx
git commit -m "feat: refresh application recovery UI"
```

---

### Task 4: Simplify device discovery and connection

**Files:**
- Create: `src/features/devices/deviceFilters.ts`
- Create: `src/features/devices/deviceFilters.test.ts`
- Create: `src/DeskElement.test.tsx`
- Modify: `src/IntroPage.tsx`
- Modify: `src/DeskElement.tsx`

**Interfaces:**
- Produces: `filterDevices(devices: ConnectionDesk[] | undefined, showAll: boolean): ConnectionDesk[]`.
- `DeskElement` retains current props but calls `onConnect` only after `connectToDesk` resolves and always balances `onLoadStart`/`onLoadEnd` through `finally`.
- Consumes: shared UI components and existing `getAvailableDesks`, `connectToDesk`, `removeConfig`, and `relaunch` behavior.

- [ ] **Step 1: Write filtering and connection-state tests**

Create `deviceFilters.test.ts`:

```ts
import { describe, expect, it } from "vitest";
import { filterDevices } from "./deviceFilters";

const devices = [
  { name: "Desk 1234", status: "new" as const },
  { name: "Headphones", status: "new" as const },
];

describe("filterDevices", () => {
  it("shows only desk-named devices by default", () => {
    expect(filterDevices(devices, false).map(({ name }) => name)).toEqual(["Desk 1234"]);
  });

  it("shows every device when requested", () => {
    expect(filterDevices(devices, true)).toEqual(devices);
  });

  it("returns an empty list before discovery resolves", () => {
    expect(filterDevices(undefined, false)).toEqual([]);
  });
});
```

In `DeskElement.test.tsx`, mock `connectToDesk`. Assert that success calls `onConnect`, failure calls `onError` but not `onConnect`, and both outcomes call `onLoadStart` and `onLoadEnd` exactly once.

- [ ] **Step 2: Run tests and expose the current false-success bug**

Run:

```bash
npm test -- src/features/devices/deviceFilters.test.ts src/DeskElement.test.tsx
```

Expected: FAIL because the helper is absent and the current row calls `onConnect` even after rejection.

- [ ] **Step 3: Implement the pure device filter and corrected device row**

Implement:

```ts
export const filterDevices = (
  devices: ConnectionDesk[] | undefined,
  showAll: boolean
): ConnectionDesk[] =>
  showAll ? devices ?? [] : (devices ?? []).filter(({ name }) => name.includes("Desk"));
```

Rebuild `DeskElement` as a bordered `XStack` with device name/status and an `AppButton`. Its async handler must be:

```ts
setLoading(true);
onLoadStart?.();
try {
  await connectToDesk(deskName);
  onConnect?.();
} catch (error) {
  onError(String(error));
} finally {
  setLoading(false);
  onLoadEnd?.();
}
```

The button label is `Connect`, `Connecting`, or `Connected`; connected rows are disabled and use success color/status text.

- [ ] **Step 4: Rebuild the intro page states and controls**

Use `PageShell` with title `Connect your desk` and a concise description. Use a responsive Tamagui row at wider widths for compact branding/context beside discovery, collapsing to one column under `$sm`. In a `SurfaceCard`:

- Show `StatePanel` with `CarrotSpinner` while scanning.
- Show `No matching desks found` when discovery completed with an empty filtered list.
- Render filtered rows otherwise.
- Render Refresh as secondary with a `RefreshCw` icon.
- Render a reversible secondary toggle labeled `Show all devices` / `Show desks only`.
- Disable discovery controls while scanning or connecting and after successful connection.
- Show the existing alternate-name explanation as muted helper text.
- Show `LinkButton to="/new-position"` labeled `Add first position` only after success; before success, show muted guidance instead of a disabled link/tooltip.
- Convert the reset-on-error branch to the shared recovery card pattern and retain error details.

- [ ] **Step 5: Run device tests, lint, and build**

Run:

```bash
npm test -- src/features/devices/deviceFilters.test.ts src/DeskElement.test.tsx
npm run lint
npm run build
```

Expected: tests, lint, and build PASS.

- [ ] **Step 6: Commit device discovery**

```bash
git add src/IntroPage.tsx src/DeskElement.tsx src/DeskElement.test.tsx src/features/devices
git commit -m "feat: simplify desk discovery flow"
```

---

### Task 5: Rebuild position creation and shortcut capture

**Files:**
- Create: `src/features/positions/positionForm.ts`
- Create: `src/features/positions/positionForm.test.ts`
- Modify: `src/NewPositionPage.tsx`

**Interfaces:**
- Produces: `PositionFieldErrors`, `validatePosition(name: string, value: string): PositionFieldErrors`, `ShortcutCaptureState`, and `applyShortcutKey(state: ShortcutCaptureState, event: Pick<KeyboardEvent, "key">): ShortcutCaptureState`.
- Consumes: `MIN_HEIGHT`, `MAX_HEIGHT`, existing `createNewElem`, and existing window-close behavior.

- [ ] **Step 1: Write validation and shortcut-state tests**

Create `positionForm.test.ts` with exact cases:

```ts
expect(validatePosition("", "7200")).toEqual({ name: "Name cannot be empty" });
expect(validatePosition("Sit", "desk")).toEqual({ value: "Height must be a number" });
expect(validatePosition("Sit", "6199")).toEqual({ value: "Height must be between 6200 and 12700" });
expect(validatePosition("Sit", "12701")).toEqual({ value: "Height must be between 6200 and 12700" });
expect(validatePosition("Sit", "7200")).toEqual({});
```

Define shortcut assertions for:

- Idle/captured state resets when capture begins.
- `Control` maps to `CmdOrCtrl`.
- A second distinct modifier appends with `+`.
- A third modifier restarts capture from that modifier.
- Shifted `!` maps to `1`.
- A normal key completes capture.
- Repeating the same first modifier restarts rather than duplicates it.

- [ ] **Step 2: Run the pure tests and confirm missing exports**

Run: `npm test -- src/features/positions/positionForm.test.ts`

Expected: FAIL because `positionForm.ts` does not exist.

- [ ] **Step 3: Extract validation and shortcut transitions**

Move `modifierMap` and `lowercaseMap` into `positionForm.ts`. Define:

```ts
export type PositionFieldErrors = { name?: string; value?: string };

export type ShortcutCaptureState =
  | { status: "idle"; value: "" }
  | { status: "capturing"; value: string }
  | { status: "captured"; value: string };
```

`validatePosition` returns only field errors and uses the exact bounds. `applyShortcutKey` must preserve the existing maximum of two modifiers and transition to `captured` after a non-modifier key. Export `beginShortcutCapture(): ShortcutCaptureState` and `clearShortcutCapture(): ShortcutCaptureState` to avoid duplicating state literals in the page.

- [ ] **Step 4: Rebuild the new-position form**

Use `PageShell`, one `SurfaceCard`, Tamagui `Input`, `FormField`, and `AppButton`.

- Name input: label `Position name`.
- Height input: label `Position height`, numeric input mode, helper `Allowed range: 6200–12700`.
- Both inputs use an orange `focusStyle` border/ring and preserve visible keyboard focus.
- Shortcut section: label `Keyboard shortcut (optional)`; a secondary button shows `Click to record`, `Listening for keys…`, or the captured value; a ghost `Clear` action appears when a value exists.
- During capture, show an orange border and status text. Keep the document key listener active only while `status === "capturing"` and call `preventDefault()` for accepted shortcut input.
- On submit, run `validatePosition`; do not call Rust when errors exist.
- Translate the `"duplicate"` response to a name-field error.
- Show rejected native calls in a form-level error `Alert`.
- Use button labels `Add position` and `Adding position`.
- Close `appWindow` only after a `"success"` response.
- Remove diagnostic `console.log` calls from this page.

- [ ] **Step 5: Run position tests, lint, and build**

Run:

```bash
npm test -- src/features/positions/positionForm.test.ts
npm run lint
npm run build
```

Expected: tests, lint, and build PASS.

- [ ] **Step 6: Commit position creation**

```bash
git add src/NewPositionPage.tsx src/features/positions/positionForm.ts src/features/positions/positionForm.test.ts
git commit -m "feat: refresh position creation form"
```

---

### Task 6: Rebuild position management

**Files:**
- Create: `src/features/positions/PositionList.tsx`
- Create: `src/features/positions/PositionList.test.tsx`
- Modify: `src/ManagePositionsPage.tsx`
- Remove: `src/assets/cross.svg`

**Interfaces:**
- `PositionList` props: `{ positions: Config["saved_positions"] | undefined; loading: boolean; removingName?: string; onRemove(name: string): void }`.
- Consumes: `PageShell`, `SurfaceCard`, `StatePanel`, `Alert`, `LinkButton`, `AppButton`; existing `getPositions`, `removePosition`, retry callback, and window close.

- [ ] **Step 1: Write prop-driven list-state tests**

Create `PositionList.test.tsx` and cover:

```tsx
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
  await userEvent.click(screen.getByRole("button", { name: "Remove Standing" }));
  expect(onRemove).toHaveBeenCalledWith("Standing");
});
```

Also assert that the affected row labels its action `Removing Standing` and all removal buttons are disabled while `removingName` is set.

- [ ] **Step 2: Run the position-list test and confirm the missing component failure**

Run: `npm test -- src/features/positions/PositionList.test.tsx`

Expected: FAIL because `PositionList` does not exist.

- [ ] **Step 3: Implement the responsive position list**

Build `PositionList` with Tamagui stacks rather than an HTML table so columns can adapt. Use a header row with Name, Height, Shortcut, and an action spacer. Keep that header outside the rows' `ScrollView` so it remains visible while the data scrolls. Each data row uses the same grid-like flex widths, truncates long names with a `title` attribute, renders `—` for missing shortcuts, and uses Lucide `Trash2` in a ghost/destructive icon button with an explicit aria-label.

At narrow `$sm` width, allow Name to take the first row and move height/shortcut/actions into a compact second row. The scroll area must contain only list rows, not the persistent header or page actions.

- [ ] **Step 4: Rebuild `ManagePositionsPage` around `PositionList`**

Use `PageShell` with title `Manage positions`. Keep `useSimpleAsync(getPositions)`. Pass `data?.saved_positions`, `loading`, `removingName`, and a removal callback into `PositionList`.

The removal callback must clear stale action error, await `removePosition(name)`, call `retry()` only after success, expose rejection through `Alert`, and clear `removingName` in `finally`. Remove diagnostic logging.

Place `LinkButton to="/new-position"` labeled `New position` and a secondary `AppButton` labeled `Close` in `PageShell` actions so they remain outside the scrollable list.

- [ ] **Step 5: Run list tests, lint, and build**

Run:

```bash
npm test -- src/features/positions/PositionList.test.tsx
npm run lint
npm run build
```

Expected: tests, lint, and build PASS.

- [ ] **Step 6: Commit position management**

```bash
git add src/ManagePositionsPage.tsx src/features/positions/PositionList.tsx src/features/positions/PositionList.test.tsx
git rm src/assets/cross.svg
git commit -m "feat: refresh saved position management"
```

---

### Task 7: Rebuild Options/About and remove the legacy styling stack

**Files:**
- Create: `src/AboutPage.test.tsx`
- Modify: `src/AboutPage.tsx`
- Modify: `package.json`
- Modify: `package-lock.json`
- Modify: `README.md`
- Remove: `postcss.config.cjs`
- Remove: `tailwind.config.cjs`
- Remove: `src/generic/button.tsx`
- Remove: `src/generic/checkbox.tsx`
- Remove: `src/generic/Href.tsx`
- Remove: `src/generic/input.tsx`
- Remove: `src/generic/label.tsx`
- Remove: `src/generic/Spinner.tsx`
- Remove: `src/generic/tooltip.tsx`
- Remove: `src/generic/lib/utils.ts`

**Interfaces:**
- Consumes: shared Tamagui UI; existing `isEnabled`, `enable`, `disable`, `removeConfig`, `relaunch`, and Tauri external-link behavior.
- Produces: the same settings and support behavior without any legacy UI imports or CSS utility classes.

- [ ] **Step 1: Write settings behavior tests**

In `src/AboutPage.test.tsx`, mock `use-simple-async`, autostart functions, reset/relaunch functions, and `openUrl`. Assert:

- A checked switch is rendered when the upstream value is true.
- Activating a checked switch calls `disable()` and updates it to unchecked after resolution.
- Activating an unchecked switch calls `enable()` and updates it to checked after resolution.
- An autostart rejection produces an alert and re-enables the control.
- Reset calls `removeConfig()` before `relaunch()`.
- An HTTP project link calls Tauri `openUrl` rather than browser navigation.

Use Tamagui provider wrapping in the test renderer.

- [ ] **Step 2: Run the settings tests against the old page**

Run: `npm test -- src/AboutPage.test.tsx`

Expected: FAIL because the old page uses a Radix checkbox and lacks the new card/switch semantics.

- [ ] **Step 3: Rebuild the Options/About page**

Use `PageShell` and three `SurfaceCard` sections:

1. `Startup` — title, concise description, and Tamagui `Switch` labeled `Open Trayasen when the system starts`; disable while the initial read is unresolved or an update is pending.
2. `Advanced` — explanatory copy and destructive `Reset config and restart`; keep failures in the same card.
3. `About Trayasen` — concise creator/support copy using `ExternalLink` for GitHub, personal site, and issue creation.

Replace tooltip-only help with visible helper text. Preserve initial read errors and update/reset error behavior. Use `Settings`, `RotateCcw`, and `Info` Lucide icons. Do not change native calls.

- [ ] **Step 4: Run settings tests before dependency cleanup**

Run: `npm test -- src/AboutPage.test.tsx`

Expected: PASS.

- [ ] **Step 5: Remove all legacy imports, files, configuration, and packages**

First verify references:

```bash
rg -n "generic/|className=|@radix-ui|class-variance-authority|clsx|tailwind|twMerge" src package.json README.md
```

Expected before removal: references only in files scheduled for deletion or package metadata.

Remove packages and files:

```bash
npm uninstall @radix-ui/react-checkbox @radix-ui/react-label @radix-ui/react-slot @radix-ui/react-tooltip class-variance-authority clsx tailwind-merge tailwindcss-animate
npm uninstall --save-dev autoprefixer postcss tailwindcss
git rm postcss.config.cjs tailwind.config.cjs
git rm -r src/generic
```

If npm reports `postcss` as a transitive package after uninstall, leave the transitive lockfile entry; the requirement is to remove it as a direct dependency and remove its project configuration.

- [ ] **Step 6: Update frontend documentation and verify no legacy styling remains**

Change the README development note to:

```md
Frontend styling and components use [Tamagui](https://tamagui.dev/), with a custom dark theme for the desktop interface.
```

Run:

```bash
rg -n "generic/|className=|@radix-ui|class-variance-authority|clsx|tailwind|twMerge|shadcn" src package.json README.md postcss.config.cjs tailwind.config.cjs 2>/dev/null
```

Expected: no matches.

- [ ] **Step 7: Run complete frontend verification**

Run:

```bash
npm test
npm run lint
npm run build
```

Expected: all frontend tests PASS, ESLint exits zero, TypeScript/Vite production build exits zero.

- [ ] **Step 8: Commit About and legacy cleanup**

```bash
git add src/AboutPage.tsx src/AboutPage.test.tsx package.json package-lock.json README.md
git add -u
git commit -m "feat: complete Tamagui UI migration"
```

---

### Task 8: Resize the native window and perform cross-page verification

**Files:**
- Modify: `src-tauri/src/main.rs`

**Interfaces:**
- Consumes: existing `WindowInitUtils::init_trayasen` and platform-specific decoration behavior.
- Produces: exactly 800×600 default inner size on Windows, macOS, and Linux.

- [ ] **Step 1: Add a testable window-size constant and failing unit assertion**

Near `WindowInitUtils`, add constants but leave their initial values at the current size for the red test:

```rust
const DEFAULT_WINDOW_WIDTH: f64 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 720.0;

#[cfg(test)]
mod window_tests {
    use super::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

    #[test]
    fn default_window_uses_compact_utility_size() {
        assert_eq!((DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT), (800.0, 600.0));
    }
}
```

Replace both duplicated `inner_size(1280.0, 720.0)` calls with `inner_size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)`.

- [ ] **Step 2: Run the focused Rust test and confirm the expected failure**

Run:

```bash
cd src-tauri && cargo test default_window_uses_compact_utility_size
```

Expected: FAIL showing left `(1280.0, 720.0)` and right `(800.0, 600.0)`.

- [ ] **Step 3: Set the approved native window dimensions**

Change only the constants:

```rust
const DEFAULT_WINDOW_WIDTH: f64 = 800.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;
```

Keep `always_on_top`, Windows decoration/shadow behavior, initialization scripts, and all window-routing logic unchanged.

- [ ] **Step 4: Run native and full repository verification**

Run from the repository root:

```bash
npm test
npm run lint
npm run build
npm run tauri:test
```

Expected: every command exits zero, including the new 800×600 assertion and all existing Rust tests.

- [ ] **Step 5: Manually verify every route at desktop sizes**

Run `npm run tauri:dev` and inspect:

- `/intro` at 800×600 and a narrower resized width: no clipped actions; scanning, no-results, filtered/all toggle, connection pending, success, and error presentations are coherent.
- `/new-position`: keyboard-only navigation, visible focus, validation beside fields, shortcut idle/listening/captured/clear states, and duplicate/native errors.
- `/manage-positions`: loading, empty, populated, long-name truncation, row removal progress, and persistent bottom actions.
- `/about`: startup read/update state, destructive reset separation, external links, and responsive cards.
- `/error`: saved-desk retry, reset-only recovery, technical disclosure, pending states, and action failures.
- Windows when available: custom title bar is 30px tall, draggable, visually integrated, and its three buttons work.
- macOS/Linux when available: native decorations remain and no 30px content gap appears.

Record any unavailable platform check in the completion report as a residual risk rather than claiming it was verified.

- [ ] **Step 6: Confirm repository cleanliness and migration completeness**

Run:

```bash
git diff --check
git status --short
rg -n "generic/|className=|@radix-ui|class-variance-authority|clsx|tailwind|twMerge|shadcn" src package.json README.md 2>/dev/null
```

Expected: no whitespace errors, only intended native-size changes before commit, and no legacy styling matches.

- [ ] **Step 7: Commit native sizing and final verification adjustments**

```bash
git add src-tauri/src/main.rs
git commit -m "feat: use compact Trayasen window size"
```

After the commit, run `git status --short` and expect no output.
