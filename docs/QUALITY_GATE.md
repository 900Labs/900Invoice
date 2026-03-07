# Quality Gate

This document defines the pre-merge quality gate for 900Invoice.

Run this gate before opening a pull request and again before merge when practical.

---

## Why This Exists

900Invoice is offline-first financial software. Regression risk is high when invoice lifecycle, payment state, tax math, or export/import behavior changes.

This gate standardizes how contributors validate changes and produce reviewable evidence.

---

## Required Commands

Run from repository root:

```bash
./scripts/verify-api-doc-commands.sh
npm install
npm run check
CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

Notes:

1. `CARGO_TARGET_DIR` is intentional. It prevents lock contention when contributors or automation run concurrent Rust jobs.
2. If your environment already uses a shared target dir, keep the same dedicated value for all Rust commands in that session.
3. Keep the API command catalog block in `docs/API.md` synchronized with `src-tauri/src/lib.rs`.

For governance-impacting changes, also run:

```bash
REPORT_JSON_PATH=/tmp/ci-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD
./scripts/verify-governance-trace-json.sh /tmp/ci-governance-diff-context.json
```

For runtime/release-readiness verification, run:

```bash
INSTALL_NODE_DEPS=1 SMOKE_PROFILE=baseline LEGACY_HARDWARE=1 ./scripts/verify-runtime-smoke.sh
INSTALL_NODE_DEPS=1 SMOKE_PROFILE=full ./scripts/verify-runtime-smoke.sh
```

Reference: `docs/RUNTIME_SMOKE.md`.

---

## CI Enforcement

The same command baseline is enforced in GitHub Actions at:

- `.github/workflows/ci.yml`

Workflow behavior:

1. Runs on every pull request to `main`.
2. Runs on every push to `main`.
3. Fails the workflow if any quality-gate command fails.

---

## Security-Focused Checks

When your change touches these areas, add explicit test evidence in the PR:

1. File paths from user input (canonicalization, extension allowlist, file size limits).
2. CSV import/export (formula injection and parser edge cases).
3. Backup/restore behavior (restore safety, failure handling, rollback behavior).
4. Invoice/payment lifecycle rules (state transition invariants).

---

## PR Evidence Requirements

Include a `Validation` section in the PR body with:

1. Commands executed.
2. Result summary (`passed`/`failed` with counts when available).
3. Any skipped checks and why.

Example:

```text
Validation
- npm run check: passed
- CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml: passed
- CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml: passed (52 tests)
- CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings: passed
```

---

## Parallel Automation Rules

If using multiple automation agents:

1. Never let multiple agents write to the same checkout.
2. Use isolated workspaces for all agent edits.
3. Keep one integration checkout that receives reviewed changes.
4. Serialize `git` operations (`add`, `commit`, `merge`, `push`) from the integration checkout only.

These rules reduce repo corruption risk and keep diffs attributable.
