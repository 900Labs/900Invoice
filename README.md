<div align="center">
  <h1>900Invoice</h1>
  <p><strong>Enterprise-Grade Invoicing for Developing Economies</strong></p>
  <p>Free. Offline. Yours.</p>

  <a href="https://www.900labs.com/open-source">Website</a> •
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#contributing">Contributing</a>

  <br><br>

  <img alt="License" src="https://img.shields.io/badge/license-Apache%202.0-blue.svg">
  <img alt="Tauri" src="https://img.shields.io/badge/Tauri-v2-orange.svg">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.75+-orange.svg">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg">
  <img alt="CI" src="https://github.com/900Labs/900Invoice/actions/workflows/ci.yml/badge.svg">
</div>

---

## The Problem

A startup in Lagos pays the same $50/seat/month for invoicing software as a startup in San Francisco — but operates in an economy where the average monthly salary is $150. Every existing open-source invoicing tool requires a server, constant internet, and has zero mobile money integration. None support African fiscal compliance systems.

Freelancers in Nairobi create invoices in Word. Consultants in Accra calculate VAT on paper. Small businesses in Mumbai lose track of who owes what because a $30/month subscription is a genuine financial burden. This is not a niche problem — this is how the majority of the world's businesses operate.

## The Solution

900Invoice is a desktop invoicing application that works completely offline. It runs on the hardware you already own. No subscriptions. No cloud dependencies. No internet required after the first download.

Built by [900 Labs](https://www.900labs.com) — building enterprise-grade open source tools for the 900 million+ people in developing economies who are priced out of the software that modern businesses depend on.

## Features

### Invoice Management
- Create, edit, duplicate, and void invoices
- Full lifecycle: Draft → Finalized → Sent → Paid → Void
- Customizable invoice number sequences (INV-2026-0001)
- Professional native PDF export
- Live invoice preview

### Multi-Currency Support
- 11 currencies: KES, NGN, ZAR, INR, TZS, UGX, GHS, XOF, XAF, USD, EUR
- Exchange rate caching for offline use
- Rate snapshot on each invoice for audit trail

### Tax Engine
- Country-specific tax rates (VAT, GST, WHT, NHIL, GETFund)
- Tax-exclusive and tax-inclusive pricing
- Withholding tax support
- Pre-configured rates for Kenya, Nigeria, South Africa, India, Ghana, Tanzania, Uganda, Senegal

### Client & Product Management
- Client database with billing details and tax IDs
- Product/service catalog for quick invoicing
- CSV import/export

### Recurring Invoices
- Weekly, monthly, quarterly, and annual schedules
- Auto-generation with missed-job recovery
- Template-based creation

### Reports & Analytics
- Revenue reports by period, client, and currency
- Tax summary reports for filing
- Aging reports (30/60/90 days overdue)
- Dashboard with real-time metrics

### Payment Tracking
- Record partial and full payments
- Multiple payment methods: cash, bank transfer, mobile money, cheque
- Payment history per invoice

### Internationalization
- 6 languages: English, French, Spanish, Arabic (RTL), Swahili, Hindi
- 300+ translation keys
- RTL layout support for Arabic
- Locale-aware date and number formatting

## Tech Stack

| Component | Technology | Why |
|-----------|-----------|-----|
| Desktop Shell | Tauri v2 | 2.5 MB binary, 30 MB RAM, instant startup |
| Frontend | Svelte 5 | Smallest bundle, reactive with Runes |
| Backend | Rust | Memory-safe, native performance |
| Database | SQLite (rusqlite) | Zero-config, single-file, offline |
| PDF Engine | Rust PDF/HTML renderer | Native PDF export plus live invoice preview |
| Scheduler | Rust due-date scheduler | Recurring invoice automation |

### Why Tauri v2?

| Metric | Tauri v2 | Electron | Advantage |
|--------|----------|----------|-----------| 
| Binary Size | 2.5–3 MB | 150+ MB | 50x smaller |
| RAM (idle) | 30–40 MB | 150–300+ MB | 5–8x less |
| Startup Time | < 500 ms | 1–3 seconds | 2–6x faster |

On a 4-year-old laptop with 4 GB of RAM running three browser tabs, this difference is everything.

## Installation

### Releases

Tagged releases are published on the [releases page](https://github.com/900Labs/900Invoice/releases/latest) when available. The current release workflow publishes source archives and checksums.
Signed Windows, macOS, and Linux installers are tracked as future release hardening. Until platform binaries are published, build from source.

### Build from Source

**Prerequisites:**

- Rust 1.75+ — install from [rustup.rs](https://rustup.rs)
- Node.js 18+ — install from [nodejs.org](https://nodejs.org)
- Tauri CLI v2 for `cargo tauri` commands:
  ```bash
  cargo install tauri-cli --version "^2"
  ```
- Tauri v2 system dependencies — see [v2.tauri.app/start/prerequisites](https://v2.tauri.app/start/prerequisites/)

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev
```

**Build and run:**
```bash
# Clone the repository
git clone https://github.com/900Labs/900Invoice.git
cd 900Invoice

# Install frontend dependencies
npm install

# Run in development mode (hot-reload)
cargo tauri dev

# Build for production
cargo tauri build
```

The production binary will be in `src-tauri/target/release/bundle/`.

## Project Structure

```
900Invoice/
├── src/                    # Svelte 5 frontend
│   ├── components/         # UI components by feature
│   │   ├── invoices/       # Invoice form, list, preview
│   │   ├── clients/        # Client management
│   │   ├── products/       # Product catalog
│   │   ├── payments/       # Payment recording
│   │   ├── reports/        # Report views
│   │   └── settings/       # App settings
│   ├── stores/             # Svelte 5 Runes state stores
│   ├── i18n/               # Translation files (6 languages)
│   ├── utils/              # Currency, date, validation utilities
│   └── lib/                # Tauri IPC wrappers
├── src-tauri/              # Rust backend
│   └── src/
│       ├── commands/       # 63 Tauri commands (IPC handlers)
│       ├── models/         # Data structures (Invoice, Client, etc.)
│       ├── db/             # SQLite schema, migrations, queries
│       ├── services/       # Tax calc, PDF engine, numbering
│       └── sync/           # Changelog for future sync
├── docs/                   # Documentation
│   └── adr/                # Architecture Decision Records
└── .github/                # CI/CD workflows and templates
```

## Data Storage

All data is stored locally in a single SQLite file. No cloud. No server.

- **Location**: `{APP_DATA_DIR}/900invoice.db`
- **Money**: Stored as integers (minor units) — no floating point
- **Tax rates**: Stored in basis points (1600 = 16.00%)
- **IDs**: UUID v4 for offline-safe creation

Your data never leaves your machine unless you explicitly export it.

## Documentation

- [Architecture Overview](docs/ARCHITECTURE.md) — system design, data flow, money conventions
- [API Documentation](docs/API.md) — complete Tauri command reference (63 commands)
- [PDF Rendering](docs/TEMPLATES.md) — invoice PDF/preview rendering notes
- [Internationalization Guide](docs/I18N.md) — add languages, RTL support
- [Deployment Guide](docs/DEPLOYMENT.md) — build and distribute for all platforms
- [Release Runbook](docs/RELEASE.md) — tagged release flow, artifacts, and checklist
- [Maintainer Checklist](docs/MAINTAINER_CHECKLIST.md) — governance profile, policy verification, and sprint/release handoff checklist
- [Branch Protection Policy](docs/BRANCH_PROTECTION.md) — required merge and branch safeguards for `main`
- [Governance Audit](docs/GOVERNANCE_AUDIT.md) — scheduled drift detection for policy and docs parity
- [Sprint Process](docs/SPRINT_PROCESS.md) — required sprint workflow and squash-merge policy
- [Quality Gate](docs/QUALITY_GATE.md) — required pre-merge validation baseline
- [Runtime Smoke Runbook](docs/RUNTIME_SMOKE.md) — release-readiness smoke verification, including legacy-hardware mode
- [Sprint Records](docs/sprints/) — delivered sprint scope, validation, and decision history

## Contributing

We welcome contributions from developers worldwide — especially those in the regions 900Invoice serves. Every line of code from a developer in Lagos, Nairobi, Accra, or Mumbai makes this tool better for the people it's built for.

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions, coding standards, sprint rules, and the PR process.

**Quick contribution ideas:**
- Add your country's default tax rates in `src-tauri/src/db/migrations.rs`
- Add a translation for your language to `src/i18n/`
- Report bugs in your operating environment
- Improve documentation clarity

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.

You are free to use, modify, and distribute this software — including commercially. You do not owe us anything.

## Security

To report a vulnerability, email [security@900labs.com](mailto:security@900labs.com). See [SECURITY.md](SECURITY.md) for the full process.

## Part of the 900 Labs Ecosystem

900Invoice is the third tool in the 900 Labs open-source portfolio:

| Tool | Description | Repository |
|------|-------------|------------|
| [900PDF](https://github.com/900-labs/900pdf) | PDF editor and toolkit | `900-labs/900pdf` |
| [900CRM](https://github.com/900-labs/900crm) | Customer relationship management | `900-labs/900crm` |
| **900Invoice** | Invoicing and billing | `900Labs/900Invoice` ← you are here |

All tools are built on the same Tauri v2 + Rust + Svelte 5 stack. They share conventions, libraries, and the same commitment: free forever, offline-first, open source.

Learn more at [900labs.com/open-source](https://www.900labs.com/open-source).

---

<div align="center">
  <strong>If we build it, you own it.</strong>
  <br>
  <a href="https://www.900labs.com">900labs.com</a>
</div>
