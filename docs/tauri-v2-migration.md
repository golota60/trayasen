# Tauri v2 Migration Notes

## Baseline

- Date: 2026-07-31
- Starting Tauri JS API: `@tauri-apps/api ^1.5.1`
- Starting Tauri CLI: `@tauri-apps/cli ^1.4.0`
- Starting Rust Tauri: `tauri 1.5.3`
- Baseline frontend build: PASS (`yarn build`)
- Baseline Rust tests: FAIL before compilation because Cargo is unavailable: `/bin/bash: cargo: command not found`

### Environment snapshot

Commands were run before changing project dependencies:

```text
$ git status --short
 M yarn.lock
?? .pi-subagents/
?? package-lock.json

$ node --version
v24.11.1

$ npm --version
11.6.2

$ yarn --version || true
1.22.22

$ rustc --version
/bin/bash: rustc: command not found

$ cargo --version
/bin/bash: cargo: command not found

$ cargo tauri --version || npx tauri --version || true
/bin/bash: cargo: command not found
tauri-cli 1.4.0
```

The `yarn.lock`, `.pi-subagents/`, and `package-lock.json` entries above were pre-existing and were not edited as part of this baseline task.

### Verification output

The project uses Yarn (`yarn.lock` and Yarn-based Tauri build hooks), so the frontend baseline used `yarn build`:

```text
$ yarn build
$ tsc && vite build
vite v4.0.4 building for production...
✓ 1559 modules transformed.
dist/assets/cross-71fdf7f1.svg    0.41 kB
dist/index.html                   1.30 kB
dist/assets/index-c63ae92b.css   13.33 kB │ gzip:  3.81 kB
dist/assets/index-7f73be55.js   280.58 kB │ gzip: 89.70 kB
Done in 4.04s.
```

The Rust baseline could not start because this environment does not have Cargo installed:

```text
$ cd src-tauri && cargo test
/bin/bash: cargo: command not found
```

## Package Manager

The project is standardized on npm for the Tauri v2 migration. `package-lock.json` is the sole lockfile and `yarn.lock` has been removed. Use `npm ci`, `npm run build`, and the existing npm scripts for dependency and frontend tasks. CircleCI, the GitHub release workflow, Tauri frontend hooks, and the README all use npm and therefore consume the committed lockfile.

`npm install` and clean `npm ci` runs completed without peer dependency errors. The final review verification ran `npm audit` against the committed lockfile and reported 0 vulnerabilities.

## Frontend Tooling Versions

The declared current compatible versions selected for the modern frontend toolchain are:

- `@vitejs/plugin-react ^6.0.5`
- `vite ^8.2.0`
- `typescript 5.1.6`
- `@types/node ^26.1.2`
- `@types/react ^18.3.31`
- `@types/react-dom ^18.3.7`

React remains on major version 18. The npm lockfile resolves `react` and `react-dom` to `18.3.1`.

TypeScript is pinned to 5.1.6 because the retained TypeScript-ESLint v5 stack supports TypeScript `>=3.3.1 <5.2.0`; pinning prevents npm from resolving a later incompatible TypeScript release. TypeScript 5.1 supports the `moduleResolution: Bundler` mode used by the project and Vite configuration projects. The obsolete `esModuleInterop: false` setting remains omitted.

The repository ESLint configuration is marked as a root config so lint resolution is isolated from any parent checkout. `npm run lint` completes successfully; the single legacy autostart import is explicitly deferred to the frontend API migration task rather than reintroducing its removed v1 dependency.

## Node.js Runtime

Vite 8 and `@vitejs/plugin-react` 6 require Node `^20.19.0 || >=22.12.0`. The same supported range is declared in `package.json`; active CircleCI and GitHub release jobs use Node 20.19.0. The README documents Node 20.19 as the minimum (and 22.12 as the minimum on Node 22).

## Frontend API Migration Errors

After the tooling configuration update, `npm run build` reaches application type-checking and fails only on Tauri v1 API imports that are scheduled for the frontend API migration:

```text
src/ManagePositionsPage.tsx(2,10): error TS2724: '"@tauri-apps/api/window"' has no exported member named 'appWindow'. Did you mean 'Window'?
src/NewPositionPage.tsx(2,10): error TS2724: '"@tauri-apps/api/window"' has no exported member named 'appWindow'. Did you mean 'Window'?
src/main.tsx(3,10): error TS2724: '"@tauri-apps/api/window"' has no exported member named 'appWindow'. Did you mean 'Window'?
src/rustUtils.ts(5,10): error TS2305: Module '"@tauri-apps/api"' has no exported member 'invoke'.
```

No Vite, TypeScript configuration, package resolution, or peer dependency error remains in this build attempt.

## Reference Scaffold

Fresh scaffold command:

```bash
npm create tauri-app@latest trayasen-v2-reference -- --template react-ts
```

The non-interactive runner reported `error: IO error: not a terminal: not a terminal` for the exact command, so the scaffold was generated at `/tmp/trayasen-tauri-v2-reference/trayasen-v2-reference` by adding the CLI's non-interactive flag:

```bash
npm create tauri-app@latest trayasen-v2-reference -- --template react-ts --yes
```

`npm install` completed with 0 vulnerabilities. `npm run tauri info` identified the starter as React + TypeScript + Vite on Tauri v2 and reported:

```text
@tauri-apps/api: 2.11.1
@tauri-apps/cli: 2.11.4
tauri: 2
tauri-plugin-opener: 2
@tauri-apps/plugin-opener: 2.5.4
```

The scaffold information command also reported that `rustc`, Cargo, rustup, and Xcode are not installed in this environment, so it could not resolve exact Rust crate versions from a build.

Files compared:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/build.rs`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/main.rs`

Comparison checklist for later migration tasks:

- [ ] `package.json`: move the Tauri API and CLI from v1 to v2, add only the v2 plugin packages required by Trayasen behavior, and preserve the project's package-manager convention.
- [ ] `src-tauri/Cargo.toml`: move `tauri` and `tauri-build` to v2, replace v1 feature flags (`api-all`, `system-tray`, and `global-shortcut`) with scoped v2 core/plugin dependencies, and retain existing non-Tauri application dependencies.
- [ ] `src-tauri/build.rs`: confirm the existing `tauri_build::build()` entry point remains compatible; the fresh scaffold uses the same entry point.
- [ ] `src-tauri/tauri.conf.json`: migrate to the v2 schema and keys (`devUrl`, `frontendDist`, and top-level `app`/`bundle`), preserve Trayasen identity and bundle metadata, and replace the v1 allowlist with capabilities.
- [ ] `src-tauri/capabilities/default.json`: create an explicit Trayasen capability and grant only permissions needed for its windows, tray, shortcuts, autostart, and other migrated commands.
- [ ] `src-tauri/src/main.rs`: adapt v1 tray, global-shortcut, window, and run-event APIs to v2 while preserving startup, background, menu, and window behavior; account for the fresh scaffold's thin main entry point when choosing the final source layout.

## Rust API Migration Errors

Rust dependencies resolve successfully with Cargo 1.97.1, and `cargo check` accepts the migrated Tauri v2 configuration and capability permissions before reaching application compilation. The resolved foundation uses Tauri 2.11.5, `tauri-build` 2.6.3, `tauri-plugin-autostart` 2.5.1, `tauri-plugin-global-shortcut` 2.3.2, and `tauri-plugin-process` 2.3.1.

Application compilation then fails on the expected Tauri v1 runtime APIs, which are deferred to the Rust API migration task. The first reported errors are:

```text
error[E0432]: unresolved imports `tauri::GlobalShortcutManager`, `tauri::WindowBuilder`
  --> src/main.rs:11:13

error[E0433]: cannot find `api` in `tauri`
 --> src/config_utils.rs:8:5

error[E0432]: unresolved imports `tauri::SystemTray`, `tauri::SystemTrayEvent`
  --> src/main.rs:12:47

error[E0432]: unresolved imports `tauri::CustomMenuItem`, `tauri::GlobalShortcutManager`, `tauri::SystemTrayMenu`, `tauri::SystemTrayMenuItem`, `tauri::SystemTraySubmenu`
 --> src/config_utils.rs:8:26
```

Cargo subsequently reports the remaining expected v1 API incompatibilities around `WindowUrl`, `global_shortcut_manager`, and `system_tray`. No runtime Rust APIs were changed in this foundation task.

## Rust Runtime Migration

The desktop runtime now uses Tauri v2's `WebviewWindowBuilder`, `menu` and `tray` modules, app path resolver, and the v2 global-shortcut and autostart plugin builders. The config remains at the operating system data directory root with the filename `idasen-tray-config.json`, so existing user data remains discoverable.

Tray actions retain the required IDs `add_position`, `manage_positions`, `about`, and `quit`; saved-position movement items use internal `position:<name>` IDs while preserving their visible and persisted names. Global shortcut strings are passed through unchanged. Registration and unregistration errors are now written to stderr with the shortcut and position name instead of being silently ignored.

The migration inventory listed `connect_to_config_desk`, but that command does not exist in the pre-migration Rust source, frontend consumers, or repository history. No new command or unapproved behavior was invented. All actual registered commands, including `get_available_desks_to_connect`, remain in `tauri::generate_handler!`.

The incompatible `window-shadows 0.2.2` dependency (which uses raw-window-handle 0.5) was removed. Undecorated Windows windows now request shadows through Tauri v2's built-in `.shadow(true)` window option.

## Frontend API and Plugin Migration

The frontend now imports command invocation from `@tauri-apps/api/core`, obtains the active window with `getCurrentWindow`, and uses the v2 process and autostart plugin packages. Existing `rustUtils.ts` wrapper names, command names, and command argument objects are unchanged. The Rust process plugin is registered so the existing reset/relaunch flows can call the v2 plugin at runtime.

The default capability grants only the required window close/minimize/toggle-maximize/start-dragging, process restart, autostart enable/disable/status, and scoped external-URL opener permissions. The migration did not introduce a broad v1-style allowlist.

## Final Review Fixes

- Custom titlebar drag regions now have the Tauri v2 `core:window:allow-start-dragging` permission required by `data-tauri-drag-region`.
- Error-window desk names, error text, and all programmatically supplied routes are serialized as JSON before inclusion in initialization scripts. Quotes, backslashes, newlines, HTML/script-like text, and JavaScript line separators are covered by focused round-trip unit tests, and the post-build `eval` path was removed.
- Rust's default test suite is hermetic: the previous nonexistent-desk test, which initialized the platform Bluetooth manager and could contact Linux BlueZ/D-Bus, was replaced with pure protocol-decoding and error-contract tests. Hardware discovery remains runtime behavior and is not exercised by `cargo test` or CircleCI.
- macOS tray construction again sets `icon_as_template(true)` so the tray icon follows menu-bar template rendering.
- About links now use the official Tauri v2 opener plugin. The capability enables only `open_url` and scopes it to the three URLs rendered by the About page; it does not grant file/path opening or arbitrary URL access.
- `npm audit` currently reports 0 vulnerabilities for the committed npm dependency graph.

## Review-Round Follow-up Fixes

- Tray requests reuse the existing `main` webview window, safely navigate it with JSON-serialized route data, then show, unminimize, and focus it. A `main` window is created only when none exists, and window errors are logged rather than panicking.
- Config removal/reset calls complete before frontend relaunch. Restart exit requests use Tauri's `RESTART_EXIT_CODE` and bypass tray refresh and `prevent_exit`, preventing the deleted-config restart race.
- Saved-position tray IDs are namespaced as `position:<name>`, so positions named `about`, `quit`, `add_position`, or another fixed action remain independently selectable. Visible position names and the persisted config format are unchanged.
- Config creation now creates a missing data-directory parent. Missing files create defaults, while malformed configs and other I/O failures are reported without overwriting the source file. Reset, deletion, startup discovery, and config writes return recoverable errors.
- Tray/global-shortcut movement failures and unavailable desk state are logged instead of panicking. Autostart mutations complete before UI state changes, and autostart/opener/reset failures are logged or displayed by the existing error UI.
- Generated `.pi-subagents/` orchestration artifacts are ignored at the repository root.
- Malformed or unreadable startup configs are preserved byte-for-byte. Trayasen starts with an in-memory empty config, keeps the tray alive, and opens a recovery route in the single `main` window where the user can explicitly reset and relaunch.
- Startup reconnection is read-only. A desk name is persisted only after a user-initiated Bluetooth connection and protocol setup succeed; successful retry relaunches so tray shortcuts and runtime state initialize normally.
- Position create/remove/load, config reset, and autostart actions now expose failures and disable duplicate submissions while pending.

Residual verification limits are unchanged for physical desk movement and unavailable interactive GUI automation. The About link handler, opener registration, exact URL scopes, frontend compilation, capability generation, app packaging, and bounded app launch were validated; actually clicking the links was not attempted because it would open the host browser from this non-interactive run.

## Intentional Changes

- Change: The repository package manager changed from Yarn to npm; `package-lock.json` is now the sole lockfile and developer, Tauri hook, and CI commands use npm.
  - Reason: One locked package-manager path avoids divergent dependency graphs during the v2/tooling upgrade.
  - User impact: Contributors use `npm ci`, `npm run tauri:dev`, and `npm run tauri:build` rather than Yarn commands.
  - Verification: `npm ci`, `npm run build`, and inspection of `src-tauri/tauri.conf.json`, `.circleci/config.yml`, and `.github/workflows/release.yml`.
- Change: The frontend toolchain now requires Node.js in the exact range `^20.19.0 || >=22.12.0` and Rust 1.88 or newer.
  - Reason: Vite 8 requires that Node range, and the resolved Rust dependency graph requires Rust 1.88.
  - User impact: Node 20 before 20.19.0 and Node 21 are unsupported; older Rust toolchains cannot compile the application.
  - Verification: `package.json` engines, `src-tauri/Cargo.toml` `rust-version`, README prerequisites, and CI runtime pins.
- Change: Tauri core, CLI, configuration, and permissions migrated from v1 to v2 with scoped capabilities and plugins.
  - Reason: Tauri v2 replaces the v1 allowlist and several core APIs with capabilities and official plugins.
  - User impact: No intended workflow change; users retain the same app windows, tray actions, shortcuts, autostart control, and relaunch flow.
  - Verification: `npm ls @tauri-apps/api @tauri-apps/cli @tauri-apps/plugin-autostart @tauri-apps/plugin-opener @tauri-apps/plugin-process --depth=0`, `cargo check --locked`, and `cargo test --locked`.
- Change: Frontend command invocation, current-window access, process relaunch, and autostart calls use their Tauri v2 core/plugin import paths.
  - Reason: The corresponding Tauri v1 exports and legacy autostart package are unavailable in v2.
  - User impact: No intended visible change; existing wrapper and Rust command interfaces are preserved.
  - Verification: `npm run build`, `npm run lint`, and the v1 API inventory search in the final verification evidence.
- Change: The Rust runtime uses v2 webview-window, menu, tray, global-shortcut, path, process, and autostart APIs.
  - Reason: The v1 runtime types and managers were removed or replaced in Tauri v2.
  - User impact: Startup, background operation, tray actions, saved-position actions, and shortcuts are intended to remain the same.
  - Verification: `cargo check --locked`, the eleven passing hermetic Rust tests, the bounded macOS launch, and source inspection of stable tray action IDs.
- Change: The internal About/Options tray menu ID changed from `about/options` to `about`.
  - Reason: The runtime migration contract uses a stable identifier without a slash, and the menu and handler must agree.
  - User impact: None expected; the visible label remains `About/Options`.
  - Verification: `config_utils::tests::tray_action_ids_remain_stable` and source inspection of the menu handler.
- Change: Global shortcut registration and unregistration failures are logged to stderr with shortcut and position context instead of being silently ignored.
  - Reason: Tauri v2 plugin operations return errors that should remain diagnosable without crashing the tray runtime.
  - User impact: Failed shortcut operations can produce diagnostic stderr output; successful behavior is unchanged.
  - Verification: Source inspection of shortcut registration/unregistration error branches and `cargo check`.
- Change: Windows custom-decoration shadows use Tauri v2's built-in `.shadow(true)` option instead of `window-shadows`.
  - Reason: `window-shadows 0.2.2` is incompatible with Tauri v2's raw-window-handle version.
  - User impact: Windows 11 may show Tauri's documented rounded corners and an undecorated shadow may have a 1px white border.
  - Verification: `cargo check`; Windows rendering remains not verified.
- Change: The config filename and operating-system data-directory location are intentionally preserved rather than moving into the Tauri identifier-specific app-data directory.
  - Reason: Existing installations must continue discovering `idasen-tray-config.json` without a data migration.
  - User impact: Existing desk and saved-position configuration remains at the README-documented path.
  - Verification: `config_utils::tests::config_filename_remains_compatible` and the isolated macOS launch log showing `$HOME/Library/Application Support/idasen-tray-config.json`.
- Change: Linux packaging CI now uses Ubuntu 22.04/Tauri v2 WebKitGTK 4.1 and Ayatana AppIndicator packages, CircleCI uses Rust 1.88, and CI runs the npm frontend build and Rust tests before packaging.
  - Reason: The old Ubuntu 20.04/WebKitGTK 4.0/AppIndicator and Rust 1.74 configuration targeted the v1 dependency stack.
  - User impact: No application runtime change; release and continuous-integration builds use supported v2 prerequisites.
  - Verification: Inspection of `.github/workflows/release.yml` and `.circleci/config.yml`.

The migration inventory mentioned `connect_to_config_desk`, but that command did not exist in the pre-migration Rust source, frontend consumers, or repository history. No new command or behavior was invented.

## Final Verification Evidence

Environment used on macOS 15.7.1 (Apple Silicon):

```text
node v24.11.1
npm 11.6.2
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
```

Automated results:

- `npm ci`: PASS; installed 466 packages from the lockfile and reported 0 vulnerabilities.
- `npm run build`: PASS; TypeScript completed and Vite 8.2.0 built 1,508 modules.
- `npm run lint`: PASS with no findings.
- `npm audit`: PASS; 0 vulnerabilities.
- `source "$HOME/.cargo/env" && cd src-tauri && cargo fmt -- --check`: PASS.
- `source "$HOME/.cargo/env" && cd src-tauri && cargo check --locked`: PASS; only the existing dead-code warning for four `ConnectedBtDevice` fields was emitted.
- `source "$HOME/.cargo/env" && cd src-tauri && cargo test --locked`: PASS; 11 passed, 0 failed, 0 ignored. The suite contains only pure tests and does not initialize Bluetooth, BlueZ, or D-Bus.
- `source "$HOME/.cargo/env" && npx tauri info`: PASS for project/plugin discovery; it reports the opener Rust and JavaScript packages at 2.5.4. The environment section still notes that full Xcode is not installed, while Xcode Command Line Tools are available.
- `source "$HOME/.cargo/env" && npm run tauri:build`: application compilation and `.app` bundling PASS, but the all-target command exited 1 while styling the DMG because Finder did not answer the bounded AppleEvent: `execution error: Finder got an error: AppleEvent timed out. (-1712)` followed by `Failed running AppleScript`. This is a packaging-environment blocker, not a compile failure.
- `source "$HOME/.cargo/env" && npm run tauri:build -- --bundles app`: PASS after the final fixes; produced `src-tauri/target/release/bundle/macos/Trayasen.app` without invoking Finder's DMG styling step. Capability/config generation accepted the scoped opener and start-dragging permissions.
- A bounded eight-second packaged-app launch with an isolated malformed config remained alive, logged configuration recovery, and left the malformed file byte-for-byte unchanged before deliberate termination.

The review inventory search found current v2 imports in `src/*.tsx` and historical references in the migration plan and this evidence document. It found no active v1-only API use in application source, configuration, or package manifests.

### Bounded macOS smoke check

After the final fix package was built, its executable was launched again for 8 seconds with an isolated temporary `HOME`, `TMPDIR`, and pre-created `Library/Application Support`. It remained alive until the deliberate `SIGTERM`, created only the isolated default config, and logged normal no-config loading and Bluetooth discovery. This recheck did not connect to or move a desk, modify real user state, click About links, or alter the residual classifications below.

The earlier packaged app executable was launched directly for 12 seconds with an isolated temporary `HOME`, `TMPDIR`, and pre-created `Library/Application Support`, then terminated with `SIGTERM`. At five seconds it was still alive and macOS Launch Services identified the process as `Trayasen` with bundle identifier `szywis.Trayasen-v0.1.0`. Core Graphics reported three on-screen app-owned surfaces: two 34-pixel-wide layer-25 menu-bar surfaces and one 1280×720 layer-5 application window. The latter and the logs provide non-interactive evidence that the no-config setup window opened; the menu-bar surfaces provide non-interactive evidence that the tray item was created.

The launch created only the isolated `idasen-tray-config.json` containing `{"local_name":null,"saved_positions":[]}`, logged the isolated config path and no-desk state, and completed Bluetooth discovery. It remained alive for the full interval, exited after the deliberate signal, and left no app process. SHA-256 snapshots taken before and after the run confirmed that the real config file and the contents of the real `~/Library/LaunchAgents` directory did not change. No desk connection or movement was requested.

A window-only `screencapture` was attempted against the Core Graphics ID for the 1280×720 setup window, but macOS returned `could not create image from window`; therefore no screenshot is claimed. A read-only accessibility query through System Events also timed out under the non-interactive runner and was terminated, so it was not used to click or inspect tray menu items. An earlier harness attempt that omitted the isolated `Library/Application Support` parent exited with code 134. Config initialization now creates missing parent directories, with focused unit coverage, so isolated or newly provisioned homes no longer rely on that directory already existing.

| # | Smoke item | Status classification | Evidence / limitation | Precise follow-up |
| --- | --- | --- | --- | --- |
| 1 | App launches | **verified** | Packaged process was alive at 5 and 12 seconds, registered with Launch Services, then exited after deliberate `SIGTERM`. | Reconfirm by launching the release candidate normally on the target macOS release. |
| 2 | Tray icon appears | **verified (non-interactive observation)** | Core Graphics reported two app-owned 34-pixel-wide layer-25 surfaces at menu-bar coordinates. No screenshot was available. | Visually confirm the carrot icon on each connected display during the interactive pass. |
| 3 | Setup window opens when no desk config exists | **verified (non-interactive observation)** | Isolated no-desk config, setup/discovery logs, and an on-screen app-owned 1280×720 layer-5 window were observed. | Visually confirm setup content and controls in a disposable macOS account or with an isolated config. |
| 4 | About/Options opens from tray | **unavailable due to lack of interactive GUI access** | The accessibility query timed out and no tray item was clicked. | In an interactive session, click `About/Options` and confirm the page renders without console/runtime errors. |
| 5 | Autostart toggle reads and writes state without throwing | **intentionally not run for safety** | The real `~/Library/LaunchAgents` snapshot was unchanged; this pass was forbidden from mutating real autostart state. | In a disposable macOS account or VM, record the initial state, open About/Options, toggle on and off, confirm state after each change, and restore the initial state. |
| 6 | Reset config removes config and relaunches or prompts | **unavailable due to lack of interactive GUI access** | No reset control was clicked; the real config was protected and remained byte-for-byte unchanged. | With an isolated/disposable config, click reset, confirm that file removal and relaunch/prompt behavior match the UI, then discard the profile. |
| 7 | Quit exits the app from tray | **unavailable due to lack of interactive GUI access** | The tray Quit item was not clicked; the harness used `SIGTERM`, which verifies cleanup only, not menu dispatch. | Click `Quit` from the tray in an interactive session and confirm the process disappears without automatic restart. |
| 8 | Connect/read/save/move/remove with a physical desk | **intentionally not run for safety** | Discovery completed, but no device was connected and no physical desk was moved or configuration changed. | With the desk area clear and an operator present, connect, read height, add a temporary position, move to it, remove it, and restore the starting height. |
| 9 | Config remains at the documented platform path | **verified** | The compatibility unit test passed and the isolated launch used `$HOME/Library/Application Support/idasen-tray-config.json`; the real file hash was unchanged. | On an upgrade test profile, confirm an existing config is discovered without migration or duplication. |

**Verification conclusion:** Task 6 automated verification and bounded launch observation are complete, but Task 6 end-to-end behavior verification is **not complete**. Checklist items 4–8 still require the precise interactive/safety-controlled follow-ups above; this migration must not be represented as fully smoke-tested until those results are recorded.

## Platform Verification Status

- macOS: frontend/Rust builds, hermetic Rust tests, `.app` packaging, isolated launch after the final fixes, tray-surface creation, setup-window creation, config creation, and Bluetooth discovery verified; DMG styling remains blocked by the previously observed Finder AppleEvent timeout. About/Options interaction (including clicking external links), autostart write/readback, reset/relaunch, tray Quit, and physical-desk operations remain unverified.
- Windows: not verified on Tauri v2.
- Linux: not verified on Tauri v2; CI prerequisites were updated but have not run in this local macOS environment.
