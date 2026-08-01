# Releasing Trayasen

Releases are built when a commit is pushed to the `release` branch. The workflow creates a draft GitHub Release and uploads unsigned installers; it does not publish the release automatically.

## 1. Choose and set the version

Choose the next semantic version without a leading `v`, such as `0.2.0`. Set exactly the same value in:

- `package.json` → `version`
- `src-tauri/Cargo.toml` → `[package].version`
- `src-tauri/tauri.conf.json` → `version`

Validate the versions:

```bash
npm run release:check
```

The command must print the intended version before continuing.

## 2. Verify and commit the release candidate

Run the project checks:

```bash
npm test
npm run lint
npm run build
npm run tauri:test
```

Commit the version changes and any other intended release changes:

```bash
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
# Add other intentional release files before committing.
git commit -m "chore: prepare v0.2.0"
```

Replace `0.2.0` with the version being released.

## 3. Trigger the release workflow

Move the `release` branch forward to the verified release commit and push it:

```bash
git switch release
git merge --ff-only master
git push origin release
```

Do not create the version tag manually. GitHub Actions creates `v<version>`, such as `v0.2.0`, when it creates the draft release. Do not force-push `release`; resolve a rejected non-fast-forward push before continuing.

Monitor the **Release** workflow in GitHub Actions. Its matrix builds:

- Linux x64
- Windows x64
- macOS Intel
- macOS Apple Silicon

A failure on one platform does not cancel the other jobs. Rerun a failed job only after diagnosing it. Do not choose a new version merely to hide a failed build.

## 4. Review the draft

Open the draft `Trayasen v<version>` under GitHub Releases and confirm that all four platform/architecture combinations have downloadable assets. A partially successful matrix can leave an incomplete draft, so never infer completeness from the draft's existence alone.

Review the generated release notes and edit them when necessary. Download and launch each installer on applicable hardware, checking that Trayasen starts and can reach its normal tray interface.

## 5. Publish manually

Publish the draft only when:

- Every matrix job has passed.
- Every expected platform and architecture has an asset.
- Smoke testing has succeeded.
- The tag, title, and release notes use the intended version.

Publishing the draft makes it visible as the final GitHub Release.

## Unsigned-build warning

Current release assets are unsigned. macOS Gatekeeper and Windows SmartScreen may warn users or require them to explicitly allow the application. Apple notarization and Windows code signing are not part of the current release process.
