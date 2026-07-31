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

The project is standardized on npm for the Tauri v2 migration. `package-lock.json` is the sole lockfile and `yarn.lock` has been removed. Use `npm install`, `npm run build`, and the existing npm scripts for dependency and frontend tasks.

`npm install` and a clean `npm ci` completed without peer dependency errors. npm reported 20 high-severity audit findings in the existing dependency tree; addressing unrelated dependency upgrades is outside this task.

## Frontend Tooling Versions

The declared current compatible versions selected for the modern frontend toolchain are:

- `@vitejs/plugin-react ^6.0.5`
- `vite ^8.2.0`
- `typescript ^7.0.2`
- `@types/node ^26.1.2`
- `@types/react ^18.3.31`
- `@types/react-dom ^18.3.7`

React remains on major version 18. The npm lockfile resolves `react` and `react-dom` to `18.3.1`.

TypeScript 7 no longer accepts `esModuleInterop: false` or the legacy `moduleResolution: Node` (`node10`) mode. The project and Vite configuration TypeScript projects now use `moduleResolution: Bundler`, and the removed `esModuleInterop: false` setting is omitted.

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

## Intentional Behavior Changes

None yet.

## Platform Verification Status

- macOS: not yet verified on Tauri v2
- Windows: not yet verified on Tauri v2
- Linux: not yet verified on Tauri v2
