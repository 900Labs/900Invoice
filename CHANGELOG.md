# Changelog

All notable changes to 900Invoice will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added a documented pre-merge validation baseline at `docs/QUALITY_GATE.md`.
- Added CI enforcement of the quality gate at `.github/workflows/ci.yml` for pull requests and pushes to `main`.
- Added `src-tauri/icons/icon.png` required for Tauri metadata/build compatibility.
- Added root and Rust lockfiles (`package-lock.json`, `src-tauri/Cargo.lock`) for reproducible installs.
- Added security regression unit tests for CSV export sanitization and hardened PDF logo loading.
- Added API docs parity verification script at `scripts/verify-api-doc-commands.sh`.
- Added release-gate workflow at `.github/workflows/release.yml` for tagged releases.
- Added scheduled governance-audit workflow at `.github/workflows/governance-audit.yml` for repository-policy drift detection.
- Added release runbook at `docs/RELEASE.md`.
- Added branch protection policy runbook at `docs/BRANCH_PROTECTION.md`.
- Added governance-audit runbook at `docs/GOVERNANCE_AUDIT.md`.
- Added automated governance incident issue routing on governance-audit workflow failures.
- Added governance profile presets (`solo`, `small-team`, `enterprise`) with shared env-contract resolution for policy scripts.
- Added governance profile onboarding examples for maintainers in `CONTRIBUTING.md`.
- Added governance-audit profile assertion artifact for workflow triage.
- Added `docs/MAINTAINER_CHECKLIST.md` with governance profile selection and policy verification handoff checklist.
- Added governance sprint-checklist enforcement script and CI check for governance-impacting PRs.
- Added release-gate enforcement parity for governance sprint checklist verification.
- Added release-gate governance diff trace artifact (`release-governance-diff-context-<tag>`) and workflow summary output.
- Added machine-readable JSON governance trace output (`release-governance-diff-context.json`) in release-gate artifacts.
- Added optional external governance-incident webhook notifications in `.github/workflows/governance-audit.yml`.
- Added governance sprint-doc checklist completion block enforcement in `scripts/verify-governance-sprint-checklist.sh`.
- Added governance diff trace JSON schema at `docs/schemas/governance-diff-trace.schema.json`.
- Added governance trace schema validator script at `scripts/verify-governance-trace-json.sh`.
- Added configurable governance artifact retention policy (`GOVERNANCE_ARTIFACT_RETENTION_DAYS`) for governance workflow artifacts.
- Added governance webhook retry/backoff controls (`GOVERNANCE_INCIDENT_WEBHOOK_MAX_ATTEMPTS`, `GOVERNANCE_INCIDENT_WEBHOOK_BACKOFF_SECONDS`).
- Added optional HMAC-signed governance webhook delivery using `GOVERNANCE_INCIDENT_WEBHOOK_HMAC_SECRET`.
- Added governance artifact inventory workflow (`.github/workflows/governance-artifact-inventory.yml`) with scheduled/dispatch reporting outputs.
- Added repository policy automation scripts:
  - `scripts/apply-repo-policy.sh`
  - `scripts/verify-repo-policy.sh`
- Added runtime smoke verification script (`scripts/verify-runtime-smoke.sh`) and runbook (`docs/RUNTIME_SMOKE.md`) with legacy-hardware mode for low-resource validation.
- Added `mark_invoice_sent` to complete the invoice Draft -> Finalized -> Sent lifecycle command surface.

### Changed

- Hardened business logo handling:
  - Validates logo paths as files.
  - Restricts extensions to known image formats.
  - Enforces a 2MB size limit.
  - Copies assets into app-managed storage before persisting path references.
- Hardened PDF logo loading with canonicalization, extension allowlist, and size checks.
- Restored frontend/Tauri IPC parity for client, product, tax, settings, exchange-rate, recurring, invoice, line-item, and payment flows by mapping backend snake_case contracts at the store boundary.
- Corrected model-backed invoice tax summaries to apply each line item's stored tax rate instead of applying every active tax rate to the full invoice subtotal.
- Pinned `esrap@2.2.2` because `esrap@2.2.3` was missing its exported TypeScript printer files and blocked Svelte validation/builds.
- Wired Settings import/export actions to native file open/save dialogs for client CSV import/export, invoice CSV export, JSON backups, and JSON restore.
- Expanded JSON database backups and additive restore to include invoice line items, invoice tax rows, payments, recurring schedules, exchange rates, settings, business profile, tax rates, products, clients, invoices, and invoice sequences.
- Corrected tax report base amounts by deriving each persisted tax line's taxable base from invoice line items when mapping invoice details.
- Updated report CSV export to save through a native file dialog and export rows for the active revenue, tax, or aging report.
- Aligned manual recurring invoice generation with the scheduler service so generated invoices preserve template totals, line items, and persisted tax rows.
- Replaced the placeholder invoice PDF command with native PDF byte generation and a desktop save flow from invoice preview.
- Added CSV formula-injection mitigation for exports by neutralizing spreadsheet-evaluable leading characters.
- Enforced draft-only mutation rules for invoice and line-item edit/delete operations.
- Added payment validation and lifecycle protections:
  - Rejects non-positive payment amounts.
  - Requires currency match between payment and invoice.
  - Prevents payments on `draft` and `void` invoices.
  - Reverts invoice status from `paid` when payment deletion causes outstanding balance.
- Updated sprint/contributor process docs with quality-gate and parallel-agent safety guidance.
- Updated repository links from `900-labs/900invoice` to `900Labs/900Invoice`.
- Rewrote `docs/API.md` to match the live Tauri command surface in `src-tauri/src/lib.rs`.
- Added CI step to enforce API docs command-catalog parity.
- Updated deployment documentation to align with the current automated release model.
- Enforced squash-only merge settings and auto-delete-on-merge at repository level.
- Activated `main` branch protection and strict governance verification.
- Added strict repository-policy verification step to release workflow.
- Made branch-protection approval requirements configurable in policy scripts with `REQUIRED_APPROVING_REVIEW_COUNT` and set default governance baseline to `0` required approvals for autonomous maintainer operation.
- Updated governance/release workflows to consume governance profile variables for strict policy verification across team sizes.
- Updated contributor/sprint/release/governance docs to reference the maintainer checklist workflow.
- Updated PR template to include maintainer governance checklist confirmation for governance-impacting changes.
- Updated release runbook and release workflow to enforce governance checklist verification before release builds.
- Updated governance checklist script with optional report output for release-trace generation.
- Updated governance checklist enforcement with optional strict mode (`STRICT_SPRINT_DOC_REFERENCE`) for all changed sprint docs.
- Updated governance checklist completion-block linting to fail on duplicate, nested, or unbalanced block markers.
- Updated governance audit runbook and maintainer checklist with optional external incident webhook setup guidance.
- Updated sprint and maintainer process docs with a stable `MAINTAINER_CHECKLIST_COMPLETION` sprint-doc block template.
- Updated CI and release workflows to enforce governance trace JSON schema validation.
- Updated quality/release runbooks with governance trace schema validation commands.
- Updated governance/release/maintainer docs with governance artifact retention policy guidance.
- Updated governance/maintainer docs with webhook retry/backoff and HMAC-signing guidance.
- Updated governance trace schema and validator with explicit schema-version compatibility rules (`1.0.0` legacy, `1.1.0` current).
- Updated governance workflows to support per-workflow artifact retention overrides (audit/release/inventory) with shared fallback.
- Updated quality and contributor documentation to include runtime smoke evidence as part of release-readiness validation.
- Updated frontend type-check command to use `svelte-check --workspace src --tsconfig ../tsconfig.json` for deterministic diagnostics in constrained environments.
- Updated frontend tooling dependencies by pinning `@sveltejs/vite-plugin-svelte` to `6.0.0` and `@sveltejs/vite-plugin-svelte-inspector` to `5.0.0` for reproducible smoke/build behavior.
- Added an npm override for `aria-query@5.3.2` to keep Svelte compiler imports/builds reproducible when `svelte@5.53.7` requests the broken `aria-query@5.3.1` package contents.

### Fixed

- Removed invalid `app.title` key from `src-tauri/tauri.conf.json` to satisfy strict Tauri/Rust checks.
- Removed `rg` dependency from repository-policy scripts to improve compatibility on minimal CI runners.
- Repaired `src-tauri/icons/icon.png` to valid RGBA PNG content required by `tauri::generate_context!()` during Rust compile checks.
- Ignored generated local `.tmp/` governance reports so release trace artifacts do not dirty contributor work trees.

## [1.0.0] - 2026-03-05

Initial public release of 900Invoice — enterprise-grade invoicing for developing economies.

### Added

#### Invoice Management
- Create, edit, duplicate, and void invoices
- Full invoice lifecycle state machine: Draft → Finalized → Sent → Paid → Void
- State transition validation (e.g., cannot edit a finalized invoice without voiding first)
- Customizable invoice number sequences with prefix, year, and zero-padded counter (e.g., `INV-2026-0001`)
- Gap-free sequential invoice numbering using database-level sequence management
- Invoice notes and terms fields
- Due date calculation with configurable payment terms (Net 7, Net 14, Net 30, Net 60, custom)

#### PDF Generation
- Professional PDF generation using the typst-bake engine
- Live invoice preview in the application
- Customizable Typst template at `src-tauri/src/templates/invoice.typ`
- Business logo embedding in PDF
- Paper size configuration (A4, US Letter)

#### Multi-Currency Support
- 11 currencies: KES (Kenyan Shilling), NGN (Nigerian Naira), ZAR (South African Rand), INR (Indian Rupee), TZS (Tanzanian Shilling), UGX (Ugandan Shilling), GHS (Ghanaian Cedi), XOF (West African CFA franc), XAF (Central African CFA franc), USD (US Dollar), EUR (Euro)
- Exchange rate caching for offline use
- Rate snapshot stored on each invoice for permanent audit trail
- Display amounts in any currency with correct decimal places (0 decimals for UGX, XOF, XAF)

#### Tax Engine
- Country-specific tax rates pre-configured for Kenya (VAT 16%), Nigeria (VAT 7.5%, WHT 5%/10%), South Africa (VAT 15%), India (GST 18%, 12%, 5%), Ghana (VAT 15%, NHIL 2.5%, GETFund 2.5%), Tanzania (VAT 18%), Uganda (VAT 18%), Senegal (TVA 18%)
- Tax-exclusive pricing mode (tax added on top of subtotal)
- Tax-inclusive pricing mode (tax extracted from price)
- Withholding tax support with separate calculation
- Multiple tax rates per invoice
- Tax type labels on PDF (VAT, GST, WHT, etc.)
- Custom tax rates via Settings

#### Client Management
- Client database with full contact information
- Tax ID / VAT number storage per client
- Default currency and payment terms per client
- Client search with full-text indexing
- CSV import for bulk client creation
- CSV export for backup or migration

#### Product & Service Catalog
- Product and service catalog for quick line-item entry
- Unit of measure per product (hours, units, kg, etc.)
- Default unit price per product
- Default tax rate association per product
- Product search

#### Recurring Invoices
- Recurring invoice schedules: weekly, monthly, quarterly, annual
- Auto-generation of invoices based on schedule
- Missed-job recovery: generates missed invoices on application startup if the app was offline
- Template-based creation from any existing invoice
- Enable/disable recurring schedules without deleting them

#### Payment Tracking
- Record partial and full payments against invoices
- Supported payment methods: cash, bank transfer, mobile money, cheque, card
- Payment history with date, amount, and method per invoice
- Automatic invoice status update to "Paid" when fully settled
- Overpayment detection

#### Reports & Analytics
- Revenue report: total invoiced and collected by time period and currency
- Tax summary report: total tax collected by type, suitable for tax filing
- Aging report: outstanding invoices bucketed by 0–30, 31–60, 61–90, and 90+ days overdue
- Dashboard with real-time metrics: total revenue, outstanding balance, overdue amount, and invoice count

#### Settings & Configuration
- Business profile: name, address, logo, tax ID, default currency, default payment terms
- Invoice number sequence configuration
- Tax rate management
- Supported currency selection
- Language selection
- Database backup to user-selected file
- Database restore from backup file

#### Internationalization
- 6 languages: English, French, Spanish, Arabic (with RTL layout), Swahili, Hindi
- ~250 translation keys covering all user-visible text
- RTL layout support activated automatically for Arabic
- Locale-aware date formatting (e.g., `5 mars 2026` in French)
- Locale-aware number formatting (e.g., `1.500,00` in European locales)

#### Data & Sync Infrastructure
- All data stored in a single SQLite file at `{APP_DATA_DIR}/900invoice.db`
- All monetary amounts stored as `i64` integer minor units — no floating point
- All tax rates stored as `i32` basis points
- UUID v4 primary keys for offline-safe record creation
- Database schema versioning with forward-only migrations
- Changelog table for future multi-device sync (infrastructure only in v1.0.0)

#### Search
- Full-text search across invoices (number, client name, notes)
- Full-text search across clients (name, email, company)
- Full-text search across products (name, description)

---

[Unreleased]: https://github.com/900Labs/900Invoice/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/900Labs/900Invoice/releases/tag/v1.0.0
