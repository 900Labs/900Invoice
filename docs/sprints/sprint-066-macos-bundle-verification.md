# Sprint 066 — macOS Bundle Verification

Status: Completed
Date: 2026-06-12
Owner: 900 Labs

## Context

The macOS package needed a clean rebuild after the app startup fix. A stale and polluted local dependency tree caused `svelte/compiler` to hang, and the generated macOS app wrapper did not include bundle resources because the Tauri config did not declare desktop bundle icons.

## Goals

1. Rebuild frontend dependencies from the lockfile.
2. Produce fresh macOS `.app` and `.dmg` artifacts from `cargo tauri build`.
3. Ensure the packaged app includes macOS resources and a valid icon.
4. Verify the app launches from the generated bundle without the prior startup failure.
5. Keep local signing and release-signing expectations clear.

## Deliverables

1. Rebuilt `node_modules` with `npm ci --cache .npm-cache`, then removed the temporary project-local npm cache.
2. Replaced the invalid placeholder icon with a generated invoice-style desktop icon.
3. Generated the standard desktop Tauri icon set:
   - `src-tauri/icons/32x32.png`
   - `src-tauri/icons/128x128.png`
   - `src-tauri/icons/128x128@2x.png`
   - `src-tauri/icons/icon.icns`
   - `src-tauri/icons/icon.ico`
4. Enabled explicit Tauri bundle targets in `src-tauri/tauri.conf.json`.
5. Updated `README.md`, `docs/DEPLOYMENT.md`, and `CHANGELOG.md`.

## Validation

Run from the repository root:

```bash
npm run check
npm run build
cargo tauri build
hdiutil verify src-tauri/target/release/bundle/dmg/900Invoice_1.0.0_aarch64.dmg
```

Observed local validation:

1. `npm run check` completed with 0 errors and 0 warnings.
2. `npm run build` produced a fresh Vite `dist/` bundle.
3. `cargo tauri build` produced:
   - `src-tauri/target/release/bundle/macos/900Invoice.app`
   - `src-tauri/target/release/bundle/dmg/900Invoice_1.0.0_aarch64.dmg`
4. The generated DMG mounted successfully and passed `hdiutil verify`.
5. A stripped ad hoc signed copy of the `.app` passed strict `codesign` verification, was repackaged into `900Invoice_1.0.0_aarch64-local-signed.dmg`, mounted successfully, and launched with the app process remaining alive.

## Decisions

1. Keep `tauri.conf.json` free of hardcoded ad hoc signing identity so production signing can use Developer ID configuration later.
2. Treat `900Invoice_1.0.0_aarch64-local-signed.dmg` as a local smoke-test artifact only, not a public release artifact.
3. Keep generated mobile icon variants out of the repo until mobile targets are intentionally supported.
4. Document that macOS public distribution still requires Developer ID signing and notarization.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
