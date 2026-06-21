# Sprint 072: Tauri Package Alignment And Package Test Workflow

## Context

The release-candidate macOS bundle dry run failed before packaging because the installed npm Tauri packages were behind the Rust Tauri crates in `src-tauri/Cargo.lock`.

The local checkout was healthy after Sprint 071, so this sprint focused on removing that release-build blocker and adding a manual workflow for candidate platform artifact checks before tagging.

## Goals

1. Align npm Tauri package versions with the Rust Tauri crate versions.
2. Restore `cargo tauri build` bundle generation for local macOS release candidates.
3. Add a manual GitHub Actions workflow for macOS DMG and Windows EXE package-test artifacts.
4. Document the package-test workflow in the release runbook.

## Changes

1. Updated `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, and `@tauri-apps/plugin-fs` to versions aligned with the Rust-side Tauri packages.
2. Added `.github/workflows/package-test.yml` with manual macOS DMG and Windows NSIS EXE jobs.
3. Added release runbook guidance for package-test artifacts.
4. Updated the changelog.

## Validation

Run from the clean local checkout:

```bash
./scripts/verify-local-preflight.sh
npm run check
npm run build
CARGO_TARGET_DIR=/tmp/900invoice-sprint072-target cargo check --manifest-path src-tauri/Cargo.toml
cargo tauri build
hdiutil verify src-tauri/target/release/bundle/dmg/900Invoice_1.0.0_aarch64.dmg
codesign --verify --deep --strict --verbose=2 /tmp/900invoice-rc-signed.<suffix>/900Invoice.app
hdiutil verify src-tauri/target/release/bundle/dmg/900Invoice_1.0.0_aarch64-local-signed.dmg
```

The signed local app copy launched successfully and stayed running in the launch smoke test.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
