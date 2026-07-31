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

`npm install` and a clean `npm ci` completed without peer dependency errors. npm reported 20 high-severity audit findings in the existing dependency tree; addressing unrelated dependency upgrades is outside this task.

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

Tray actions retain the required IDs `add_position`, `manage_positions`, `about`, and `quit`; saved position names remain the IDs for their movement items. Global shortcut strings are passed through unchanged. Registration and unregistration errors are now written to stderr with the shortcut and position name instead of being silently ignored.

The migration inventory listed `connect_to_config_desk`, but that command does not exist in the pre-migration Rust source, frontend consumers, or repository history. No new command or unapproved behavior was invented. All actual registered commands, including `get_available_desks_to_connect`, remain in `tauri::generate_handler!`.

The incompatible `window-shadows 0.2.2` dependency (which uses raw-window-handle 0.5) was removed. Undecorated Windows windows now request shadows through Tauri v2's built-in `.shadow(true)` window option.

## Intentional Behavior Changes

- The internal About/Options tray menu ID is normalized from the source's legacy `about/options` value to the migration contract's `about` value. The menu event handler uses the same ID, so no user-visible behavior change is expected.
- Windows custom-decoration shadows now use Tauri v2's built-in shadow implementation rather than `window-shadows`. Tauri documents that an undecorated window with shadows has a 1px white border and, on Windows 11, rounded corners; exact rendering can therefore differ from the v1 dependency.

## Platform Verification Status

- macOS: not yet verified on Tauri v2
- Windows: not yet verified on Tauri v2
- Linux: not yet verified on Tauri v2
