# Task 4 Report: Simplify device discovery and connection

## Status

Complete. Desk discovery and connection now use the reviewed Tamagui shared UI, device filtering is isolated and tested, and the false-success path is fixed.

## Commit

- `2ca82e34b21b3e64a11357a8c0ee95d18cba8489` — `feat: simplify desk discovery flow`

## Implementation

- Added `filterDevices` with the requested default desk-name filtering, reversible show-all behavior, and safe handling before discovery resolves.
- Rebuilt `DeskElement` as a bordered Tamagui row with device connection status and `AppButton` states for Connect, Connecting, and Connected.
- Corrected the connection flow so `onConnect` runs only after `connectToDesk` resolves, failures are stringified and sent to `onError`, and loading callbacks/state are balanced in `finally`.
- Rebuilt `IntroPage` with `PageShell`, responsive Tamagui stacks, `SurfaceCard`, scanning and empty `StatePanel` states, `CarrotSpinner`, secondary Refresh/show-all controls, connection guidance, success `Alert`, and the existing Found route through `LinkButton`.
- Disabled discovery controls during scanning/connection and after success.
- Converted discovery/connection errors to the shared recovery-card pattern while preserving details and the existing `removeConfig` then `relaunch` behavior.
- Preserved the `getAvailableDesks`, `connectToDesk`, `removeConfig`, and `relaunch` contracts without modifying their implementations.

## Changed files

- `src/DeskElement.test.tsx`
- `src/DeskElement.tsx`
- `src/IntroPage.tsx`
- `src/features/devices/deviceFilters.test.ts`
- `src/features/devices/deviceFilters.ts`

## Tests added or updated

- `src/features/devices/deviceFilters.test.ts`
  - Default desk-name filtering.
  - Reversible show-all result.
  - Empty result before discovery resolves.
- `src/DeskElement.test.tsx`
  - Successful connection callback behavior and balanced loading callbacks.
  - Failed connection behavior: error callback, no false success, and balanced loading callbacks.

## TDD evidence

1. The first focused run failed because `deviceFilters` did not exist and the current `DeskElement` reported an error object/continued into its false-success path.
2. After implementing the helper and corrected async row, the same focused command passed all 5 tests.

## Commands run

- `npm test -- src/features/devices/deviceFilters.test.ts src/DeskElement.test.tsx`
  - Initial expected result: failed (missing helper and current failure-path behavior exposed).
  - Final result: passed, 2 files and 5 tests.
- `npx prettier --write src/IntroPage.tsx src/DeskElement.tsx src/DeskElement.test.tsx src/features/devices/deviceFilters.ts src/features/devices/deviceFilters.test.ts`
  - Passed; formatted only Task 4 files.
- `npm run lint`
  - Initial result: failed on three Prettier findings in Task 4 files.
  - Final result: passed with no ESLint findings.
- `npm run build`
  - Passed; TypeScript and Vite production build completed.
- `npm test`
  - Passed, 5 files and 22 tests.
- `git diff --cached --check`
  - Passed before commit.
- `git commit -m "feat: simplify desk discovery flow"`
  - Passed; created commit `2ca82e3`.

## Validation output

- Focused tests: `2 passed` test files, `5 passed` tests.
- Full tests: `5 passed` test files, `22 passed` tests.
- Lint: clean.
- Build: `2765 modules transformed`, production bundle emitted successfully.
- Commit diff: 5 files changed, 280 insertions, 131 deletions.
- Repository index after commit: no staged files.

## Self-review

- No blockers found.
- Scope is limited to the two requested components, the pure filter/helper tests, and the connection row tests.
- The success callback is inside the `try` after the awaited connection; error paths cannot reach it.
- Both success and failure execute loading cleanup from `finally`.
- The new-position route remains `/new-position` and is still implemented with Found via the shared `LinkButton`.
- Recovery still removes configuration before relaunching and retains raw error details.

## Residual risks / concerns

- Vitest passes but jsdom logs Tamagui generated-CSS parsing noise because its CSS parser does not understand all generated CSS constructs.
- Vite reports the existing warning that the main minified chunk exceeds 500 kB; the build succeeds and bundle splitting is outside Task 4 scope.
- No manual Tauri/Bluetooth hardware connection was performed in this environment; the Rust command contracts are covered by preservation and mocked component tests.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Commit 2ca82e3 changes only the five Task 4 implementation/test files; focused tests, full tests, lint, build, and diff checks passed, and the connection failure test proves onConnect is not called on rejection."
    }
  ],
  "changedFiles": [
    "src/DeskElement.test.tsx",
    "src/DeskElement.tsx",
    "src/IntroPage.tsx",
    "src/features/devices/deviceFilters.test.ts",
    "src/features/devices/deviceFilters.ts"
  ],
  "testsAddedOrUpdated": [
    "src/DeskElement.test.tsx",
    "src/features/devices/deviceFilters.test.ts"
  ],
  "commandsRun": [
    {
      "command": "npm test -- src/features/devices/deviceFilters.test.ts src/DeskElement.test.tsx (initial red run)",
      "result": "failed",
      "summary": "Expected TDD failure: deviceFilters was absent and existing DeskElement failure behavior did not meet the new contract."
    },
    {
      "command": "npm test -- src/features/devices/deviceFilters.test.ts src/DeskElement.test.tsx (final run)",
      "result": "passed",
      "summary": "2 test files and 5 tests passed."
    },
    {
      "command": "npm run lint (initial run)",
      "result": "failed",
      "summary": "Three Prettier findings in Task 4 files were identified and corrected."
    },
    {
      "command": "npm run lint (final run)",
      "result": "passed",
      "summary": "ESLint completed with no findings."
    },
    {
      "command": "npm run build",
      "result": "passed",
      "summary": "TypeScript and Vite production build completed successfully."
    },
    {
      "command": "npm test",
      "result": "passed",
      "summary": "Full suite passed: 5 test files and 22 tests."
    },
    {
      "command": "git diff --cached --check",
      "result": "passed",
      "summary": "No whitespace errors before commit."
    },
    {
      "command": "git commit -m \"feat: simplify desk discovery flow\"",
      "result": "passed",
      "summary": "Created commit 2ca82e3."
    }
  ],
  "validationOutput": [
    "Focused Vitest: 2 files passed, 5 tests passed.",
    "Full Vitest: 5 files passed, 22 tests passed.",
    "ESLint: passed with no findings.",
    "Build: 2765 modules transformed and production assets emitted.",
    "git diff --cached --check: passed before commit.",
    "Commit diff: 5 files changed, 280 insertions, 131 deletions."
  ],
  "residualRisks": [
    "Vitest emits non-failing jsdom CSS parser noise for Tamagui generated CSS.",
    "Vite emits the existing non-failing main-chunk size warning.",
    "No manual Tauri/Bluetooth hardware validation was performed."
  ],
  "noStagedFiles": true,
  "diffSummary": "Added the pure device filter and callback tests, rebuilt DeskElement with balanced async state and no false success, and migrated IntroPage discovery/recovery/success states to shared Tamagui UI.",
  "reviewFindings": [
    "no blockers"
  ],
  "manualNotes": "Full report artifact is stored at .superpowers/sdd/2026-08-01-tamagui-ui-refresh/task-4-report.md; implementation is committed as 2ca82e3."
}
```

## Fix round 1

### Status

Complete. Both review findings are addressed without changing discovery or connection behavior outside the requested accessibility and test coverage.

### Changed files

- `src/IntroPage.tsx`
- `src/IntroPage.test.tsx`
- `src/DeskElement.test.tsx`
- `src/ui/Feedback.tsx`
- `src/ui/ui.test.tsx`
- `.superpowers/sdd/2026-08-01-tamagui-ui-refresh/task-4-report.md`

### Implementation and covering tests

- Wrapped the scanning and completed no-results `StatePanel` messages in explicit `role="status"`, `aria-live="polite"` regions, with `aria-busy="true"` exposed during scanning.
- Added a typed `decorative` mode to `CarrotSpinner`; scanning uses it so the status text is announced without a duplicate spinner label, while the spinner's default `role="img"` and `Loading` accessible name remain unchanged.
- Added `src/IntroPage.test.tsx` coverage proving both messages are reachable by `role="status"`, both are polite, scanning is busy, and its spinner is hidden from the accessibility tree.
- Added `src/ui/ui.test.tsx` coverage proving the default spinner remains accessible.
- Strengthened `src/DeskElement.test.tsx` success coverage with a manually controlled deferred promise. Before resolution it proves `onConnect` and `onLoadEnd` have not fired; after resolution it proves completion callbacks fire exactly once. Existing rejection coverage remains.

### Commands and results

- `npm test -- src/IntroPage.test.tsx src/DeskElement.test.tsx src/features/devices/deviceFilters.test.ts`
  - Passed: 3 test files, 7 tests.
- `npm run lint`
  - Passed with no ESLint findings.
- `npm run build` (first run)
  - Failed: TypeScript required Tamagui's `aria-busy` prop to be boolean rather than a string. Changed the value to the typed boolean form.
- `npm run build` (after the type correction)
  - Passed: TypeScript and Vite build completed; 2765 modules transformed.
- `npm test -- src/IntroPage.test.tsx src/DeskElement.test.tsx src/features/devices/deviceFilters.test.ts src/ui/ui.test.tsx` (final focused run; affected files named explicitly)
  - Passed: 4 test files, 18 tests.
- `npm run lint` (final run)
  - Passed with no ESLint findings.
- `npm run build` (final run)
  - Passed: TypeScript and Vite production build completed; 2765 modules transformed.
- `git diff --check`
  - Passed with no whitespace errors.

### Risks / concerns

- Vitest continues to emit the existing non-failing jsdom/Tamagui generated-CSS parser noise.
- Vite continues to emit the existing non-failing main-chunk size warning.
- No manual screen-reader pass was performed; the requested semantics are covered by DOM accessibility-role assertions.
