import assert from 'node:assert/strict';
import test from 'node:test';

import {
  assertMatchingReleaseVersions,
  parseCargoPackageVersion,
} from './check-release-version.mjs';

test('reads the package version rather than a dependency version', () => {
  const cargoToml = `[package]
name = "trayasen"
version = "1.2.3"

[dependencies]
example = { version = "9.9.9" }
`;

  assert.equal(parseCargoPackageVersion(cargoToml), '1.2.3');
});

test('returns a matching semantic version', () => {
  assert.equal(
    assertMatchingReleaseVersions({
      'package.json': '1.2.3',
      'src-tauri/Cargo.toml': '1.2.3',
      'src-tauri/tauri.conf.json': '1.2.3',
    }),
    '1.2.3',
  );
});

test('rejects mismatched versions with every value in the message', () => {
  assert.throws(
    () =>
      assertMatchingReleaseVersions({
        'package.json': '1.2.3',
        'src-tauri/Cargo.toml': '1.2.4',
        'src-tauri/tauri.conf.json': '1.2.3',
      }),
    /package\.json: 1\.2\.3[\s\S]*Cargo\.toml: 1\.2\.4[\s\S]*tauri\.conf\.json: 1\.2\.3/,
  );
});

test('rejects a matching value that is not a semantic version', () => {
  assert.throws(
    () =>
      assertMatchingReleaseVersions({
        'package.json': 'next',
        'src-tauri/Cargo.toml': 'next',
        'src-tauri/tauri.conf.json': 'next',
      }),
    /not a semantic version: next/,
  );
});
