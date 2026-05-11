# Sprint 032: Recurring Startup Automation

- **Sprint ID**: `032`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Recurring generation had a shared scheduler service and a manual **Generate Now** command, but the app lifecycle did not run due processing automatically. The README promised auto-generation with missed-job recovery, so startup needed to invoke the due processor without requiring user action.

## Goals

1. Run recurring due processing automatically when the app starts.
2. Keep recurring due checks active while the app process remains open.
3. Reuse the existing scheduler service so manual and automatic generation stay identical.

## In Scope

1. Process due schedules immediately after database initialization.
2. Start a lightweight hourly background worker against the managed database connection.
3. Log startup and periodic recurring generation failures without blocking app launch.
4. Document the app lifecycle behavior in the architecture notes.

## Out of Scope

1. OS-level background agents when the desktop app is closed.
2. Email sending for `auto_send` schedules.
3. User-configurable scheduler intervals.

## Deliverables

1. Startup recurring due processing in Tauri setup.
2. Hourly in-process recurring due worker.
3. Architecture and changelog updates.
4. Sprint handoff documentation.

## Validation

1. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
2. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml services::recurring_scheduler -- --nocapture` passed.
3. `npm run check` passed.
4. `npm run build` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
7. `./scripts/verify-api-doc-commands.sh` passed.
8. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
9. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Automatic processing could duplicate invoices.
   - **Mitigation**: the shared scheduler advances `next_generation_date` after generation; startup, hourly, and manual paths all use the same idempotent due-date filter.
2. **Risk**: A scheduler failure could block app launch.
   - **Mitigation**: startup and periodic workers log errors and keep the app running.

## Decisions

1. Use an hourly in-process worker instead of adding a cron dependency because due-date filtering prevents duplicate generation and the app is offline-first.
2. Keep manual **Generate Now** as an explicit recovery action that uses the same scheduler service.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
