# Sprint 069 - Dependency And Security Cleanup

Status: Completed
Date: 2026-06-20
Owner: 900 Labs

## Context

Sprint 068 completed the GitHub Actions Node 24 runtime cleanup. The next release-readiness gap was the dependency audit output from `npm ci`, which reported five npm advisories before the first public release tag. This sprint audited and refreshed the JavaScript and Rust lockfiles before cutting further release artifacts.

## Goals

1. Remove current npm audit vulnerabilities without risky direct dependency rewrites.
2. Refresh compatible Rust dependencies and reduce RustSec advisory warnings where possible.
3. Keep source-build dependency declarations stable unless a direct dependency change is required.
4. Document any residual security advisory output for maintainers.

## Deliverables

1. Updated `package-lock.json` with patched frontend build dependencies:
   - `svelte` from `5.53.7` to `5.56.3`
   - `vite` from `6.4.1` to `6.4.3`
   - `devalue` from `5.6.3` to `5.8.1`
   - `picomatch` from `4.0.3` to `4.0.4`
   - `postcss` from `8.5.8` to `8.5.15`
2. Updated `src-tauri/Cargo.lock` to the latest compatible Rust dependency graph:
   - `tauri` from `2.10.3` to `2.11.3`
   - `tauri-build` from `2.5.6` to `2.6.3`
   - `wry` from `0.54.2` to `0.55.1`
   - `notify-rust` from `4.12.0` to `4.18.0`
   - `uds_windows` from yanked `1.2.0` to `1.2.1`
3. Removed older transitive Rust advisory contributors from the lockfile, including `fxhash`, older `rand` versions, `proc-macro-hack`, and `kuchikiki`.
4. Removed an untracked duplicate Sprint 068 document copied with a ` 2` filename suffix.

## Audit Results

Before cleanup:

- `npm audit` reported 5 vulnerabilities: 2 moderate and 3 high.
- `cargo audit --file src-tauri/Cargo.lock` reported 22 allowed warnings.

After cleanup:

- `npm audit --json` reports 0 vulnerabilities.
- `cargo audit --file src-tauri/Cargo.lock` exits successfully and reports 17 allowed warnings.

Residual RustSec warnings are transitive through upstream Tauri platform dependencies, not direct app dependencies:

- GTK3/WebKitGTK Linux stack: `atk`, `gdk`, `gtk`, related `*-sys` crates, `glib`, and `gtk3-macros`.
- Tauri URL pattern stack: `unic-*` crates through `urlpattern` and `tauri-utils`.
- GTK macro stack: `proc-macro-error` through `glib-macros` and `gtk3-macros`.

These are accepted as upstream platform warnings for this sprint. Removing them requires upstream Tauri/Wry/Linux stack changes or a framework/backend migration, not a local lockfile refresh.

## Validation

Run from the repository root:

```bash
npm audit --json
cargo audit --file src-tauri/Cargo.lock
npm run check
CARGO_TARGET_DIR=/tmp/900invoice-sprint069-target cargo check --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-sprint069-target cargo test --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-sprint069-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
git diff --check -- package-lock.json src-tauri/Cargo.lock CHANGELOG.md docs/sprints/sprint-069-dependency-security-cleanup.md
```

Expected result: npm audit reports zero vulnerabilities, RustSec reports no failing vulnerabilities, and the local quality gate passes.

## Decisions

1. Prefer lockfile refreshes over direct dependency policy changes because the vulnerable npm packages were already covered by compatible semver ranges.
2. Keep `package.json` and `src-tauri/Cargo.toml` unchanged for this sprint.
3. Treat remaining RustSec warnings as documented upstream platform risk while Tauri continues to depend on the current GTK3/WebKitGTK stack.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
