import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const SEMVER = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;

export function parseCargoPackageVersion(source) {
  const lines = source.split(/\r?\n/);
  const packageStart = lines.findIndex((line) => line.trim() === '[package]');

  if (packageStart === -1) {
    throw new Error('src-tauri/Cargo.toml has no [package] section');
  }

  for (const line of lines.slice(packageStart + 1)) {
    if (/^\s*\[/.test(line)) break;

    const match = line.match(/^\s*version\s*=\s*"([^"]+)"\s*(?:#.*)?$/);
    if (match) return match[1];
  }

  throw new Error('src-tauri/Cargo.toml has no package version');
}

export function assertMatchingReleaseVersions(versions) {
  const entries = Object.entries(versions);
  const uniqueVersions = new Set(entries.map(([, version]) => version));

  if (uniqueVersions.size !== 1) {
    const details = entries.map(([file, version]) => `  ${file}: ${version}`).join('\n');
    throw new Error(`Release versions must match:\n${details}`);
  }

  const version = entries[0]?.[1];
  if (!version || !SEMVER.test(version)) {
    throw new Error(`Release version is not a semantic version: ${version ?? '<missing>'}`);
  }

  return version;
}

export function readReleaseVersions(rootDir = process.cwd()) {
  const readJson = (relativePath) =>
    JSON.parse(readFileSync(path.join(rootDir, relativePath), 'utf8'));

  return {
    'package.json': readJson('package.json').version,
    'src-tauri/Cargo.toml': parseCargoPackageVersion(
      readFileSync(path.join(rootDir, 'src-tauri/Cargo.toml'), 'utf8'),
    ),
    'src-tauri/tauri.conf.json': readJson('src-tauri/tauri.conf.json').version,
  };
}

const invokedPath = process.argv[1] && path.resolve(process.argv[1]);
if (invokedPath === fileURLToPath(import.meta.url)) {
  try {
    const version = assertMatchingReleaseVersions(readReleaseVersions());
    console.log(`Release version: ${version}`);
  } catch (error) {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  }
}
