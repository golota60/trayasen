# Tamagui UI Refresh Design

## Summary

Trayasen will receive a complete visual refresh built with Tamagui. The new interface will use a calm, minimal, dark-only design suited to a focused desktop utility. Tamagui will fully replace the existing Tailwind, shadcn-style, and Radix styling layer.

The refresh will preserve Trayasen's routes, saved data, Tauri commands, and desk-control behavior while simplifying awkward interactions and improving feedback. The default application window will change from 1280×720 to 800×600.

## Goals

- Give every page a consistent, polished dark interface.
- Establish Tamagui as the only component and styling system.
- Make scanning, connecting, form entry, shortcut capture, deletion, settings, and recovery states clear.
- Keep the application responsive when its desktop window is resized.
- Preserve existing native integrations and persisted-data behavior.
- Improve keyboard focus, labels, status messaging, and color contrast.

## Non-goals

- Adding light mode.
- Changing desk-control protocols, saved configuration formats, routes, or Tauri command contracts.
- Adding new product features unrelated to the current workflows.
- Performing broad Rust or application-architecture refactors.

## Visual Direction

The interface will use a calm, minimal dark theme:

- Deep charcoal and navy surfaces.
- Slightly elevated cards separated by restrained borders.
- Off-white primary text and muted secondary text.
- Carrot orange used selectively for branding, focus, and primary actions.
- Red reserved for destructive actions and errors.
- Moderate corner radii and compact spacing appropriate for a desktop utility.
- System-font typography with clear title, section, body, label, and helper-text levels.

The Windows custom title bar will remain and adopt the same palette. Native decorations on macOS and Linux remain unchanged.

## Architecture

### Tamagui foundation

The frontend will add a Tamagui configuration containing tokens for color, spacing, size, radius, and typography, plus one dark application theme. `TamaguiProvider` will wrap the router at the application root.

Tailwind, shadcn-style local primitives, Radix UI dependencies, class-variance-authority, clsx, tailwind-merge, and Tailwind-specific build configuration will be removed when no remaining code uses them. Lucide React will remain for icons.

### Shared UI layer

A small shared component layer will provide consistent page construction without creating a large bespoke design system:

- `PageShell`: viewport, scrolling, centered content width, and page padding.
- `PageHeader`: page title, optional description, and optional branding.
- `Card`: grouped content with consistent surface, border, radius, and spacing.
- `Button`: primary, secondary, ghost, and destructive treatments with disabled/loading behavior.
- `FormField`: label, control, helper text, and field-level error placement.
- `Alert`: error, warning, informational, and success feedback.
- Loading, empty, and status-state presentations shared across pages.

Complex interaction components will only be extracted when they have a clear independent responsibility. Shortcut capture and device-result rows are expected candidates.

### Layout behavior

The default Tauri window will be 800×600. Tamagui stacks and scroll containers will let content adapt to smaller resized windows without clipping controls. Each page will use the shared shell rather than implementing its own viewport centering and spacing.

## Page Designs

### Connect / Intro

The intro will use a welcoming layout with compact branding and a prominent device-discovery card. At wide sizes, branding/context and discovery content may sit in two columns; narrow windows collapse them into one column.

The discovery card will explicitly represent:

- Initial scanning and refresh progress.
- No matching desks found.
- Filtered desk results.
- All Bluetooth device results.
- Per-device connection progress.
- Successful connection.
- Discovery or connection failure.

Each device will appear as a row with its name, status, and connect action. Refresh and the filtered/all-results toggle will be secondary controls. “Show all devices” will work as a reversible toggle rather than a one-way action. The add-position action becomes prominent only after a successful connection.

A desk must only enter the connected state after `connectToDesk` resolves successfully. Failures remain visible and do not unlock the next step.

### New Position

The page will use a focused form card with three sections:

1. Position name.
2. Height within the existing minimum and maximum values.
3. Optional keyboard shortcut.

Validation feedback will be attached to the relevant field instead of occupying an unrelated fixed error row. Submission failures will appear as a form-level alert.

Shortcut capture will have explicit idle, listening, and captured states. The control will show when keyboard input is being recorded and provide a clear reset action. Existing accelerator mapping and supported shortcut behavior remain unchanged.

The submit action will show progress and prevent duplicate submission. Successful creation will retain the current window-closing behavior.

### Manage Positions

Saved positions will appear in a structured, scrollable list with a persistent header and columns for name, height, shortcut, and actions. The layout will preserve readable information at the 800×600 default size and degrade gracefully if narrowed.

The page will include dedicated loading, empty, load-error, and removal-error states. Removing a position will show progress on the affected row and temporarily prevent conflicting removals. “New position” remains the primary page action; “Close” remains secondary and visible outside the scrolling list.

### Options / About

The page will separate unrelated concerns into distinct cards:

- Startup behavior, including current read/update status.
- Advanced/destructive reset controls.
- Project and support information.

The reset action will be isolated, use destructive styling, state its effect clearly, and expose reset failures nearby. Autostart updates will prevent duplicate toggles and report failures without obscuring the setting.

### Returning-user and Intro Errors

Both recovery experiences will use the shared alert and card patterns. They will present:

- A plain-language summary.
- A concise description.
- Primary retry behavior when a saved desk can be retried.
- Secondary destructive reset behavior.
- Technical error details in a subdued, expandable disclosure.

Progress and failures from retry/reset actions remain visible. Recovery command behavior remains unchanged.

## State and Data Flow

Existing hooks, Tauri APIs, routes, and Rust utility functions remain the source of application behavior. Page components continue to own their local asynchronous state. Shared visual components receive state through explicit props and do not invoke Tauri commands themselves.

Every asynchronous interaction will follow the same presentation contract:

1. Clear stale action-specific errors when a new attempt begins.
2. Expose a visible pending state.
3. Disable duplicate or conflicting actions.
4. Update success state only after the operation resolves.
5. Preserve actionable failure information if it rejects.

No persisted-data schema or native command signature will change.

## Accessibility

- Inputs and controls will have explicit labels.
- Interactive elements will have visible keyboard focus styles.
- Status and error messages will use appropriate accessible semantics where supported.
- Icon-only actions will retain descriptive accessible labels.
- Text and controls will maintain practical contrast against dark surfaces.
- Disabled state will not be communicated by color alone.
- Scrollable areas will not hide primary page actions.

## Testing and Verification

Implementation verification will include:

- Frontend linting.
- TypeScript and Vite production build.
- Existing Rust tests.
- Focused frontend tests for extracted stateful interactions if the existing toolchain can support them without introducing a disproportionate test framework migration.
- Manual review of every route at the default 800×600 size and at a narrower resized width.
- Manual checks of loading, empty, success, disabled, validation, and error states that can be exercised in development.

The implementation is complete when all pages use Tamagui, no Tailwind/shadcn/Radix styling code remains, the native window defaults to 800×600, current workflows still function, and the verification commands pass.
