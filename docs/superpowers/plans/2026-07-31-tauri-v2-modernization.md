# Tauri v2 Modernization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate Trayasen from Tauri v1 to a modern Tauri v2 app while preserving the core IKEA Idasen desk control workflow and documenting any intentional behavior changes.

**Architecture:** Use a fresh Tauri v2 React + Vite starter as the migration reference, then port Trayasen's existing frontend, Rust desk-control logic, tray behavior, autostart, and global shortcut behavior into the v2 project shape. Convert v1 allowlist/config APIs to v2 capabilities and plugin permissions, and verify each slice independently before moving on.

**Tech Stack:** Tauri v2, Rust 2021, React 18 or current stable React, Vite current stable, TypeScript current stable, `@tauri-apps/api` v2, `@tauri-apps/cli` v2, official Tauri v2 plugins for autostart, process, shell/opener as needed, existing `btleplug` desk integration.

## Global Constraints

- Preserve core user flows: first-run desk setup, reconnect by saved desk name, create/remove saved positions, tray menu movement commands, About/Options reset, quit, and relaunch.
- Small behavior changes are allowed only when documented in `docs/tauri-v2-migration.md`.
- Preserve the existing config file path/name unless changing it is explicitly documented with migration/fallback behavior.
- Prefer fresh Tauri v2 scaffold defaults over hand-converting legacy v1 config when the scaffold reduces migration risk.
- Do not rewrite Bluetooth/desk protocol logic unless required for v2 compatibility.
- Verify on macOS first in this workspace; document Windows/Linux verification steps for follow-up if not available locally.
- Do not overwrite unrelated working tree changes. Current observed unrelated state: `yarn.lock` modified and `package-lock.json` untracked.

---

## File Structure

**Reference/scaffold files**
- Create temporarily outside the repo or under ignored scratch path: fresh Tauri v2 React + Vite starter.
- Use it to compare generated `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/build.rs`, `src-tauri/src/main.rs`, capability files, package scripts, and frontend imports.

**Project files likely modified**
- `package.json`: modernize scripts and JS dependencies.
- `package-lock.json` or `yarn.lock`: keep exactly one package manager lockfile after confirming project preference.
- `vite.config.ts`: align dev server port/output with Tauri v2 config.
- `tsconfig.json`, `tsconfig.node.json`: modern TypeScript/Vite compatibility if needed.
- `src-tauri/Cargo.toml`: upgrade Tauri crates and plugins to v2.
- `src-tauri/build.rs`: update build integration for Tauri v2.
- `src-tauri/tauri.conf.json`: migrate v1 schema to v2 schema.
- `src-tauri/capabilities/default.json`: add v2 permissions for windows, process/relaunch, autostart, and any shell/opener usage.
- `src-tauri/src/main.rs`: migrate builder setup, tray/menu APIs, window APIs, run events, plugins, and global shortcuts.
- `src-tauri/src/config_utils.rs`: migrate path/menu/tray imports and menu construction.
- `src-tauri/src/tray_utils.rs`: migrate window creation APIs.
- `src-tauri/src/desk_mutex.rs`: update Tauri state imports/usages only if v2 requires it.
- `src-tauri/src/loose_idasen.rs`: avoid functional changes; compile-fix only.
- `src/rustUtils.ts`: update `invoke` import to v2 path.
- `src/main.tsx`, `src/AboutPage.tsx`, `src/IntroPage.tsx`, `src/NewPositionPage.tsx`, `src/ManagePositionsPage.tsx`: update v2 frontend API imports and plugin calls.
- `docs/tauri-v2-migration.md`: document migration steps, behavior changes, and platform verification status.

---

## Task 1: Baseline and v2 Scaffold Comparison

**Files:**
- Create: `docs/tauri-v2-migration.md`
- Create outside repo or ignored scratch dir: fresh Tauri v2 starter
- Modify: none of the app runtime files

**Interfaces:**
- Consumes: existing repository state.
- Produces: documented baseline commands and a concrete v2 scaffold comparison checklist for later tasks.

- [ ] **Step 1: Record current state without modifying dependencies**

Run:

```bash
git status --short
node --version
npm --version
yarn --version || true
rustc --version
cargo --version
cargo tauri --version || npx tauri --version || true
```

Expected: command output is captured in notes. Existing unrelated changes are not edited.

- [ ] **Step 2: Run baseline frontend verification**

Run:

```bash
npm run build
```

If the project is intentionally using Yarn instead of npm, run:

```bash
yarn build
```

Expected: either PASS, or a documented pre-existing failure with exact error text in `docs/tauri-v2-migration.md`.

- [ ] **Step 3: Run baseline Rust verification**

Run:

```bash
cd src-tauri && cargo test
```

Expected: either PASS, or a documented pre-existing failure with exact error text in `docs/tauri-v2-migration.md`.

- [ ] **Step 4: Create a fresh Tauri v2 React + TypeScript + Vite starter outside the repo**

Run from a temp directory, not inside the project root:

```bash
mkdir -p /tmp/trayasen-tauri-v2-reference
cd /tmp/trayasen-tauri-v2-reference
npm create tauri-app@latest trayasen-v2-reference -- --template react-ts
cd trayasen-v2-reference
npm install
npm run tauri info
```

Expected: starter installs and reports Tauri v2 versions.

- [ ] **Step 5: Write the migration baseline document**

Create `docs/tauri-v2-migration.md` with this initial structure:

```markdown
# Tauri v2 Migration Notes

## Baseline

- Date: 2026-07-31
- Starting Tauri JS API: `@tauri-apps/api ^1.5.1`
- Starting Tauri CLI: `@tauri-apps/cli ^1.4.0`
- Starting Rust Tauri: `tauri 1.5.3`
- Baseline frontend build: PASS or documented failure
- Baseline Rust tests: PASS or documented failure

## Reference Scaffold

Fresh scaffold command:

```bash
npm create tauri-app@latest trayasen-v2-reference -- --template react-ts
```

Files compared:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/build.rs`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/main.rs`

## Intentional Behavior Changes

None yet.

## Platform Verification Status

- macOS: not yet verified on Tauri v2
- Windows: not yet verified on Tauri v2
- Linux: not yet verified on Tauri v2
```

Expected: document exists and contains actual baseline results.

- [ ] **Step 6: Commit baseline documentation only**

Run:

```bash
git add docs/tauri-v2-migration.md
git commit -m "docs: record tauri v2 migration baseline"
```

Expected: commit succeeds and does not include dependency/runtime changes.

---

## Task 2: Modernize JavaScript Tooling and Tauri JS Packages

**Files:**
- Modify: `package.json`
- Modify: chosen lockfile, either `package-lock.json` or `yarn.lock`
- Modify: `vite.config.ts` if required by modern Vite
- Modify: `tsconfig.json`, `tsconfig.node.json` if required by modern TypeScript/Vite
- Modify: `docs/tauri-v2-migration.md`

**Interfaces:**
- Consumes: baseline notes from Task 1.
- Produces: package scripts and frontend dependency set compatible with Tauri v2.

- [ ] **Step 1: Choose one package manager and document it**

Decision rule: if the team prefers npm, keep `package-lock.json` and remove `yarn.lock`; if the team prefers Yarn, keep `yarn.lock` and remove `package-lock.json`. Document the choice in `docs/tauri-v2-migration.md` under `## Package Manager`.

Expected: exactly one lockfile remains tracked after this task.

- [ ] **Step 2: Update `package.json` Tauri dependencies**

Change:

```json
"@tauri-apps/api": "^1.5.1",
"@tauri-apps/cli": "^1.4.0",
"tauri-plugin-autostart-api": "https://github.com/tauri-apps/tauri-plugin-autostart#v1"
```

to v2-compatible packages:

```json
"@tauri-apps/api": "^2",
"@tauri-apps/plugin-autostart": "^2",
"@tauri-apps/plugin-process": "^2"
```

and dev dependency:

```json
"@tauri-apps/cli": "^2"
```

Expected: no v1 Tauri JS packages remain.

- [ ] **Step 3: Modernize frontend build dependencies**

Update these dev dependencies to current compatible major versions:

```json
"@vitejs/plugin-react": "^latest-compatible-major",
"vite": "^latest-compatible-major",
"typescript": "^latest-compatible-major",
"@types/node": "^latest-compatible-major",
"@types/react": "^latest-compatible-major",
"@types/react-dom": "^latest-compatible-major"
```

Keep React 18 unless the starter uses React 19 and the app compiles cleanly with it. Document the final chosen versions in `docs/tauri-v2-migration.md`.

Expected: package install resolves without peer dependency errors.

- [ ] **Step 4: Install dependencies with the chosen package manager**

For npm:

```bash
npm install
```

For Yarn:

```bash
yarn install
```

Expected: lockfile updates successfully.

- [ ] **Step 5: Verify frontend-only build errors are now API migration errors, not tooling errors**

Run:

```bash
npm run build
```

or:

```bash
yarn build
```

Expected: build may fail because v1 Tauri imports have not been migrated yet. Any failure is documented in `docs/tauri-v2-migration.md` under `## Frontend API Migration Errors`.

- [ ] **Step 6: Commit JS tooling modernization**

Run:

```bash
git add package.json package-lock.json yarn.lock vite.config.ts tsconfig.json tsconfig.node.json docs/tauri-v2-migration.md
git commit -m "chore: modernize frontend tooling for tauri v2"
```

Expected: commit includes only JS tooling, lockfile, and documentation changes.

---

## Task 3: Migrate Tauri v2 Rust Dependencies and Config Schema

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/build.rs`
- Modify: `src-tauri/tauri.conf.json`
- Create: `src-tauri/capabilities/default.json`
- Modify: `docs/tauri-v2-migration.md`

**Interfaces:**
- Consumes: v2 scaffold comparison from Task 1.
- Produces: Tauri v2 Rust/config foundation; app may not compile until Rust code APIs are migrated in Task 4.

- [ ] **Step 1: Update Rust Tauri dependencies**

In `src-tauri/Cargo.toml`, change v1 crates:

```toml
tauri-build = { version = "1.5.0", features = [] }
tauri = { version = "1.5.3", features = ["api-all", "system-tray", "global-shortcut"] }
tauri-plugin-autostart = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v1" }
```

to v2 crates based on the fresh scaffold and official plugins:

```toml
tauri-build = { version = "2", features = [] }
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-autostart = "2"
tauri-plugin-process = "2"
```

If global shortcuts require the v2 plugin, add:

```toml
tauri-plugin-global-shortcut = "2"
```

Expected: no Tauri v1 Rust crates remain.

- [ ] **Step 2: Update `build.rs` from the v2 scaffold**

Ensure `src-tauri/build.rs` contains the v2-compatible build call:

```rust
fn main() {
    tauri_build::build()
}
```

Expected: build script matches v2 scaffold shape.

- [ ] **Step 3: Convert `tauri.conf.json` to v2 schema**

Use the v2 scaffold as the base. Preserve these current values:

```json
{
  "productName": "Trayasen",
  "version": "0.1.0",
  "identifier": "szywis.Trayasen-v0.1.0"
}
```

Map v1 fields as follows:

- `build.devPath` -> v2 `build.devUrl`
- `build.distDir` -> v2 `build.frontendDist`
- `package.productName` -> top-level `productName`
- `package.version` -> top-level `version`
- `tauri.bundle` -> top-level `bundle`
- `tauri.security.csp` -> `app.security.csp`
- `tauri.windows` -> `app.windows`
- remove v1 `allowlist`
- remove v1 `systemTray`; tray is built in Rust for v2

Expected: config validates against Tauri v2 once Rust API migration is complete.

- [ ] **Step 4: Add default capabilities**

Create `src-tauri/capabilities/default.json` with permissions for the app's v2 APIs. Start with scaffold defaults, then add only required plugin permissions for:

- window close/minimize/toggle maximize
- process relaunch
- autostart enable/disable/is-enabled
- shell/opener only if external links are opened via Tauri APIs

Expected: permissions are explicit and no broad v1-style `api-all` equivalent is used.

- [ ] **Step 5: Verify config/dependency resolution**

Run:

```bash
cd src-tauri
cargo update
cargo check
```

Expected: `cargo check` may fail on v1 Rust API usage. Dependency resolution itself should succeed. Document the first compile errors in `docs/tauri-v2-migration.md` under `## Rust API Migration Errors`.

- [ ] **Step 6: Commit Rust dependency/config migration**

Run:

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/build.rs src-tauri/tauri.conf.json src-tauri/capabilities/default.json docs/tauri-v2-migration.md
git commit -m "chore: migrate tauri config and rust dependencies to v2"
```

Expected: commit includes only Rust dependency/config/capability changes and documentation.

---

## Task 4: Migrate Rust Runtime APIs: Builder, Tray, Menus, Windows, Shortcuts

**Files:**
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/config_utils.rs`
- Modify: `src-tauri/src/tray_utils.rs`
- Modify: `src-tauri/src/desk_mutex.rs` if needed
- Modify: `src-tauri/src/loose_idasen.rs` only for compile compatibility
- Modify: `docs/tauri-v2-migration.md`

**Interfaces:**
- Consumes: v2 config/dependencies from Task 3.
- Produces: compiling Rust Tauri v2 runtime with existing command names preserved.

- [ ] **Step 1: Preserve command interface names**

Keep these command names unchanged so frontend migration can be mechanical:

```rust
connect_to_desk_by_name
connect_to_config_desk
get_config
remove_position
create_new_elem
remove_config
reset_desk
has_custom_decorations
```

Expected: `tauri::generate_handler![]` still includes all existing commands.

- [ ] **Step 2: Replace v1 path API usage**

In `config_utils.rs`, replace `tauri::api::path::data_dir` with the v2 path approach from Tauri/AppHandle path resolver. Preserve the final filename:

```text
idasen-tray-config.json
```

Expected: existing user config remains discoverable after migration.

- [ ] **Step 3: Replace v1 menu/tray types**

Migrate these v1 types/usages:

```rust
CustomMenuItem
SystemTrayMenu
SystemTrayMenuItem
SystemTraySubmenu
SystemTray
SystemTrayEvent
.on_system_tray_event(...)
```

to v2 menu/tray equivalents from `tauri::menu` and `tauri::tray`, following the v2 scaffold/docs.

Expected: menu IDs remain stable:

```text
add_position
manage_positions
about
quit
```

and saved position names still map to movement commands.

- [ ] **Step 4: Replace v1 window construction APIs**

Migrate:

```rust
tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
```

to v2 webview/window construction. Preserve labels:

```text
main
init_window
```

Expected: setup/about/manage/new-position windows can still be opened from tray actions.

- [ ] **Step 5: Replace global shortcut registration**

If Tauri v2 core no longer exposes the v1 `GlobalShortcutManager` path used by the app, migrate shortcuts to `tauri-plugin-global-shortcut`. Preserve existing shortcut strings where supported. If a shortcut combination is unsupported in v2, document the changed shortcut in `docs/tauri-v2-migration.md` under `## Intentional Behavior Changes`.

Expected: shortcut registration failure does not crash the app without a user-visible/logged reason.

- [ ] **Step 6: Replace autostart plugin initialization**

Migrate:

```rust
tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![]))
```

to the v2 autostart plugin initialization recommended by Tauri v2 docs. Preserve macOS LaunchAgent behavior if supported.

Expected: Rust side initializes the plugin successfully.

- [ ] **Step 7: Verify Rust compile**

Run:

```bash
cd src-tauri
cargo check
cargo test
```

Expected: both pass. If tests fail because of pre-existing Bluetooth environment assumptions, document exact failures and whether `cargo check` passes.

- [ ] **Step 8: Commit Rust runtime API migration**

Run:

```bash
git add src-tauri/src src-tauri/Cargo.toml src-tauri/Cargo.lock docs/tauri-v2-migration.md
git commit -m "refactor: migrate rust runtime to tauri v2 APIs"
```

Expected: commit compiles with `cargo check`.

---

## Task 5: Migrate Frontend Tauri API and Plugin Calls

**Files:**
- Modify: `src/rustUtils.ts`
- Modify: `src/main.tsx`
- Modify: `src/AboutPage.tsx`
- Modify: `src/IntroPage.tsx`
- Modify: `src/NewPositionPage.tsx`
- Modify: `src/ManagePositionsPage.tsx`
- Modify: `docs/tauri-v2-migration.md`

**Interfaces:**
- Consumes: preserved Rust command names from Task 4.
- Produces: frontend compiled against `@tauri-apps/api` v2 and v2 plugins.

- [ ] **Step 1: Update `invoke` imports**

Change root import:

```ts
import { invoke } from "@tauri-apps/api";
```

to v2 import:

```ts
import { invoke } from "@tauri-apps/api/core";
```

Expected: all existing command wrappers in `src/rustUtils.ts` keep the same exported function names and argument shapes.

- [ ] **Step 2: Update window API imports**

Replace v1 `appWindow` imports/usages with the v2 current-window API. Preserve call behavior for:

```ts
close()
minimize()
toggleMaximize()
```

Expected: titlebar buttons and page close buttons compile and call the current window.

- [ ] **Step 3: Update process relaunch imports**

Replace:

```ts
import { relaunch } from "@tauri-apps/api/process";
```

with the v2 process plugin import from `@tauri-apps/plugin-process`.

Expected: reset/relaunch flows compile.

- [ ] **Step 4: Update autostart imports**

Replace:

```ts
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
```

with v2 autostart plugin imports from `@tauri-apps/plugin-autostart`.

Expected: About/Options autostart toggle compiles and calls v2 plugin functions.

- [ ] **Step 5: Verify frontend build**

Run:

```bash
npm run build
```

or:

```bash
yarn build
```

Expected: TypeScript and Vite build pass.

- [ ] **Step 6: Verify full Tauri dev compile**

Run:

```bash
npm run tauri:dev
```

or:

```bash
yarn tauri:dev
```

Expected: app launches on macOS. If Bluetooth hardware is unavailable, app still reaches setup or a documented connection error screen.

- [ ] **Step 7: Commit frontend API migration**

Run:

```bash
git add src package.json package-lock.json yarn.lock src-tauri/capabilities/default.json docs/tauri-v2-migration.md
git commit -m "refactor: migrate frontend tauri APIs to v2"
```

Expected: commit includes frontend API/plugin/capability changes.

---

## Task 6: End-to-End Behavior Verification and Documentation

**Files:**
- Modify: `README.md` if user-facing commands or behavior changed
- Modify: `docs/tauri-v2-migration.md`
- Optional modify: `.github/workflows/*` or `.circleci/*` if CI uses old Tauri commands/dependencies

**Interfaces:**
- Consumes: compiling Tauri v2 app from Tasks 1-5.
- Produces: documented verification matrix and updated user/developer docs.

- [ ] **Step 1: Run automated verification**

Run:

```bash
npm run build
npm run tauri:test
npm run tauri:build
```

or Yarn equivalents:

```bash
yarn build
yarn tauri:test
yarn tauri:build
```

Expected: all commands pass on macOS, or failures are documented with exact error text and cause.

- [ ] **Step 2: Run macOS manual smoke test**

Verify these manually:

```text
1. App launches.
2. Tray icon appears.
3. Opening setup window works when no desk config exists.
4. About/Options window opens from tray.
5. Autostart toggle reads and writes state without throwing.
6. Reset config removes config and relaunches or prompts as expected.
7. Quit exits the app from tray.
8. If a desk is available, connect, read current height, create a saved position, move to saved position, remove saved position.
9. Config file remains at the documented platform path or migration behavior is documented.
```

Expected: all available checks pass; unavailable Bluetooth desk checks are marked `not verified` rather than guessed.

- [ ] **Step 3: Document intentional behavior changes**

In `docs/tauri-v2-migration.md`, add one bullet per change using this format:

```markdown
- Change: [what changed]
  - Reason: [why v2 migration or modernization required/preferred it]
  - User impact: [what users will notice]
  - Verification: [command or manual check]
```

Expected: every small behavior change is explicit.

- [ ] **Step 4: Update README developer commands if needed**

If install/build commands changed, update README sections with exact commands:

```bash
npm install
npm run tauri:dev
npm run tauri:build
```

or Yarn equivalents.

Expected: README matches the chosen package manager and Tauri v2 commands.

- [ ] **Step 5: Check CI configuration**

Inspect:

```bash
find .github .circleci -type f -maxdepth 3 -print
```

Update any workflow that pins old Tauri v1 CLI/setup or obsolete Node/Rust versions. Keep changes minimal.

Expected: CI commands match local verification commands.

- [ ] **Step 6: Final clean verification**

Run:

```bash
git status --short
npm run build
npm run tauri:test
```

or Yarn equivalents. Then run:

```bash
npm run tauri:build
```

or:

```bash
yarn tauri:build
```

Expected: working tree contains only intended final changes before commit; verification results are copied into `docs/tauri-v2-migration.md`.

- [ ] **Step 7: Commit final docs and verification updates**

Run:

```bash
git add README.md docs/tauri-v2-migration.md .github .circleci
git commit -m "docs: document tauri v2 migration verification"
```

Expected: final migration documentation is committed.

---

## Review Checklist

Before considering the migration complete, confirm:

- `rg '@tauri-apps/api"|@tauri-apps/api/window|@tauri-apps/api/process|tauri-plugin-autostart-api|SystemTray|SystemTrayEvent|CustomMenuItem|WindowUrl|api-all|allowlist' .` finds no remaining v1-only API usage unless documented as false positives.
- `npm run build` or `yarn build` passes.
- `cd src-tauri && cargo check && cargo test` passes.
- `npm run tauri:build` or `yarn tauri:build` passes or has a documented platform-specific packaging blocker.
- `docs/tauri-v2-migration.md` lists all intentional behavior changes.
- README and CI commands match the chosen package manager and Tauri v2.
