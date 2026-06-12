# Sprint 065 — Performance Smoke and Query Indexes

Status: Completed
Date: 2026-06-12
Owner: 900 Labs

## Context

The public-release cleanup surfaced a gap in measurable performance coverage. The existing runtime smoke gate validates build, type, and Rust behavior, but it did not seed realistic local data or measure the app paths most likely to affect speed and memory on older hardware.

## Goals

1. Add a repeatable performance smoke command.
2. Keep performance checks explicit so normal `cargo test` and CI stay stable.
3. Exercise large-dataset list/detail/export/backup/PDF paths with real Rust code paths.
4. Add database indexes for common query hot paths.
5. Document when maintainers should run the performance smoke.

## Deliverables

1. Added `scripts/verify-performance-smoke.sh`.
2. Added ignored Rust performance smoke coverage in `src-tauri/src/perf_smoke.rs`.
3. Added hot-path SQLite indexes in `src-tauri/src/db/migrations.rs`.
4. Added migration test coverage that asserts the hot-path indexes exist.
5. Exposed internal CSV export and backup helpers for direct performance-smoke coverage.
6. Added `npm run perf:smoke`.
7. Updated `docs/RUNTIME_SMOKE.md`, `docs/QUALITY_GATE.md`, `README.md`, and `docs/README.md`.

## Performance Scope

The default smoke profile seeds:

1. 500 clients.
2. 300 products.
3. 1,000 invoices.
4. 3 line items per invoice.

The ignored test measures:

1. Client list query.
2. Product list query.
3. Invoice list query.
4. Invoice detail fanout.
5. Invoice CSV export.
6. JSON database backup.
7. Native PDF generation.

## Validation

Run from the repository root:

```bash
./scripts/verify-performance-smoke.sh
./scripts/verify-api-doc-commands.sh
CARGO_TARGET_DIR=/tmp/900invoice-perf-audit-target cargo test --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-perf-audit-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

Expected result: performance smoke reports timing lines and passes budgets; API docs remain in sync; Rust tests and clippy pass.

## Decisions

1. Keep the performance smoke outside required CI because timing varies by hardware and concurrent local workloads.
2. Keep default budgets generous enough to catch regressions without creating flaky release work.
3. Use ignored Rust tests instead of a separate benchmark dependency so the check works with the existing toolchain.
4. Prefer indexed queries before deeper query rewrites; pagination and batched invoice-detail loading remain future optimizations if large-dataset smoke shows pressure.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
