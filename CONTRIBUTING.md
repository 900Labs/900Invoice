# Contributing to 900Invoice

Thank you for your interest in contributing to 900Invoice. This project exists because the software industry has systematically underserved the majority of the world's businesses. Your contribution — whether it is a bug fix, a new tax rate, a translation, or a line of documentation — makes the tool more useful for a freelancer in Nairobi, a consultant in Lagos, or a small business owner in Dhaka.

We welcome contributors from all backgrounds and experience levels. If this is your first open-source contribution, you are in the right place.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Money and Tax Rules](#money-and-tax-rules-read-this-first)
- [How to Add a New Language](#how-to-add-a-new-language)
- [How to Add a New Tax Rate](#how-to-add-a-new-tax-rate)
- [How to Add a New Currency](#how-to-add-a-new-currency)
- [Sprint Process](#sprint-process)
- [Documentation Requirements](#documentation-requirements)
- [Pull Request Process](#pull-request-process)
- [Commit Message Format](#commit-message-format)
- [Code Review](#code-review)
- [Getting Help](#getting-help)

---

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold it. Unacceptable behavior may be reported to [opensource@900labs.com](mailto:opensource@900labs.com).

---

## Prerequisites

Before you can build 900Invoice, you need:

### 1. Rust (1.75 or later)

```bash
# Install rustup (the Rust version manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

If you are on Windows, download the installer from [rustup.rs](https://rustup.rs).

### 2. Node.js (18 or later)

Download from [nodejs.org](https://nodejs.org). We recommend the LTS version.

```bash
# Verify installation
node --version
npm --version
```

### 3. Tauri System Dependencies

Tauri requires native system libraries. Install them for your operating system:

**Ubuntu / Debian / Linux Mint:**
```bash
sudo apt-get update
sudo apt-get install -y \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  librsvg2-dev \
  patchelf \
  libappindicator3-dev
```

**Fedora / RHEL:**
```bash
sudo dnf install -y \
  gtk3-devel \
  webkit2gtk4.1-devel \
  librsvg2-devel \
  patchelf
```

**Arch Linux:**
```bash
sudo pacman -S \
  gtk3 \
  webkit2gtk-4.1 \
  librsvg \
  patchelf
```

**macOS:** Install Xcode Command Line Tools:
```bash
xcode-select --install
```

**Windows:** Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload. WebView2 is included in Windows 10/11.

Full Tauri prerequisites documentation: [v2.tauri.app/start/prerequisites](https://v2.tauri.app/start/prerequisites/)

---

## Development Setup

```bash
# 1. Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/900invoice.git
cd 900invoice

# 2. Add the upstream remote
git remote add upstream https://github.com/900Labs/900Invoice.git

# 3. Install frontend dependencies
npm install

# 4. Run in development mode
# This starts the Vite dev server and the Tauri window with hot-reload
cargo tauri dev
```

The application will open in a native window. Changes to Svelte files reload instantly. Changes to Rust files trigger a recompile (typically 5–15 seconds).

### Running Tests

```bash
# Verify API docs command catalog parity
./scripts/verify-api-doc-commands.sh

# Run all Rust tests
CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml

# Run a specific test module
CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml -- tax::tests

# Check for lint errors (must pass before submitting a PR)
CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

# Check formatting
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check

# Run Svelte type checking
npm run check
```

For complete pre-merge validation, run the standard gate in `docs/QUALITY_GATE.md`.
The same gate is enforced in CI at `.github/workflows/ci.yml`.

---

## Project Structure

```
900invoice/
├── src/                          # Svelte 5 frontend
│   ├── components/
│   │   ├── invoices/             # Invoice form, list, PDF preview
│   │   ├── clients/              # Client management UI
│   │   ├── products/             # Product catalog UI
│   │   ├── payments/             # Payment recording UI
│   │   ├── reports/              # Revenue, tax, aging reports
│   │   └── settings/             # App preferences
│   ├── stores/                   # Svelte 5 Runes reactive state
│   ├── i18n/                     # Translation JSON files
│   │   ├── en.json               # English (base/reference)
│   │   ├── fr.json               # French
│   │   ├── es.json               # Spanish
│   │   ├── ar.json               # Arabic (RTL)
│   │   ├── sw.json               # Swahili
│   │   └── hi.json               # Hindi
│   ├── utils/                    # Pure utility functions
│   └── lib/                      # Tauri IPC wrappers (invoke calls)
├── src-tauri/
│   └── src/
│       ├── commands/             # Tauri IPC command handlers (~45 commands)
│       │   ├── business.rs       # Business profile commands
│       │   ├── clients.rs        # Client CRUD commands
│       │   ├── invoices.rs       # Invoice lifecycle commands
│       │   ├── line_items.rs     # Invoice line item commands
│       │   ├── taxes.rs          # Tax rate commands
│       │   ├── pdf.rs            # PDF generation commands
│       │   ├── payments.rs       # Payment recording commands
│       │   ├── recurring.rs      # Recurring invoice commands
│       │   ├── products.rs       # Product catalog commands
│       │   ├── exchange_rates.rs # Currency rate commands
│       │   ├── import_export.rs  # CSV import/export commands
│       │   ├── settings.rs       # App settings commands
│       │   └── sync.rs           # Changelog sync commands
│       ├── models/               # Rust data structures (Invoice, Client, etc.)
│       ├── db/                   # SQLite: schema, migrations, typed queries
│       ├── services/             # Business logic (tax engine, PDF, numbering)
│       ├── templates/            # Typst invoice template (invoice.typ)
│       └── sync/                 # Changelog-based sync infrastructure
└── docs/                         # Documentation and ADRs
```

---

## Coding Standards

### Rust

- **No `unwrap()` in production code.** Use `?` to propagate errors, or handle them explicitly. The only acceptable use of `unwrap()` is in tests.
- **No `expect()` in production code** unless the expectation is a fundamental programming invariant (e.g., mutex poisoning). Add a comment explaining why.
- **Follow all Clippy lints.** The CI pipeline runs `cargo clippy -- -D warnings`. All warnings are errors.
- **Format with `rustfmt`.** Run `cargo fmt` before committing. The CI checks formatting.
- **Use `thiserror` for error types.** All modules with fallible operations define a typed error enum using `#[derive(thiserror::Error)]`.
- **Document public functions.** All `pub` functions must have a `///` doc comment explaining what they do, their parameters, and what errors they return.

Example of correct error handling:
```rust
// CORRECT
pub fn get_invoice(conn: &Connection, id: &str) -> Result<Invoice, DbError> {
    let invoice = conn
        .query_row(QUERY, params![id], Invoice::from_row)
        .map_err(|e| DbError::NotFound { id: id.to_string(), source: e })?;
    Ok(invoice)
}

// WRONG — never do this
pub fn get_invoice(conn: &Connection, id: &str) -> Invoice {
    conn.query_row(QUERY, params![id], Invoice::from_row).unwrap()
}
```

### Svelte / TypeScript

- **Svelte 5 Runes only.** Do not use Svelte 4 reactive stores (`writable`, `readable`, `derived`). Use `$state`, `$derived`, and `$effect` instead.
- **All displayed text must go through `i18n`.** No hardcoded strings in templates. Use `$t('key.name')` for all user-visible text.
- **Type everything.** Avoid `any`. The ESLint rule for `@typescript-eslint/no-explicit-any` is set to `warn` but should be treated as an error for new code.
- **Component naming**: PascalCase for components, kebab-case for files (`InvoiceForm.svelte`).

---

## Money and Tax Rules (Read This First)

This is the most important section. Getting money handling wrong causes data corruption that is very difficult to recover from.

### Rule 1: Money is always `i64` in minor units

**Never** store or pass money as a floating-point number (`f32`, `f64`, JavaScript `number`). Always use integer minor units.

| Currency | Minor Unit | Example |
|----------|-----------|---------|
| KES | Cents (1/100) | KES 1,500.00 = `150000` |
| NGN | Kobo (1/100) | NGN 25,000.00 = `2500000` |
| INR | Paise (1/100) | INR 10,000.00 = `1000000` |
| UGX | Shillings (no minor unit) | UGX 50,000 = `50000` |
| XOF | Francs (no minor unit) | XOF 10,000 = `10000` |

In Rust: `amount: i64`
In TypeScript: `amount: number` (JavaScript integers are exact up to 2^53)
In SQLite: `INTEGER` column type

**Conversion for display only** (frontend):
```typescript
// Display only — never store or send this value back to Rust
function formatAmount(minorUnits: number, currency: string): string {
  const decimals = CURRENCY_DECIMALS[currency] ?? 2;
  return (minorUnits / Math.pow(10, decimals)).toFixed(decimals);
}
```

### Rule 2: Tax rates are always `i32` in basis points

A basis point is 1/100 of a percent. This gives us exact two-decimal precision for tax rates without floating point.

| Tax Rate | Basis Points |
|----------|-------------|
| 16.00% (Kenya VAT) | `1600` |
| 7.5% (Nigeria VAT) | `750` |
| 15% (South Africa VAT) | `1500` |
| 18% (India GST) | `1800` |

In Rust: `rate_bps: i32`

**Tax calculation formula:**
```
// Tax-exclusive (tax added on top):
tax_amount = (subtotal * rate_bps) / 10000

// Tax-inclusive (tax already in price):
tax_amount = subtotal - (subtotal * 10000) / (10000 + rate_bps)
```

Use integer division with rounding:
```rust
// CORRECT — integer arithmetic
let tax = (subtotal * rate_bps as i64 + 5000) / 10000; // +5000 rounds to nearest

// WRONG — floating point
let tax = (subtotal as f64 * rate_bps as f64 / 10000.0) as i64;
```

---

## How to Add a New Language

1. **Create the translation file:**
   ```bash
   cp src/i18n/en.json src/i18n/XX.json  # XX = ISO 639-1 language code
   ```

2. **Translate all values** in `src/i18n/XX.json`. Do not translate keys. Do not remove keys. Use the placeholder syntax `{variable}` as-is.
   ```json
   {
     "invoice.title": "Invoice",          // English
     "invoice.title": "Facture",          // French
     "invoice.title": "فاتورة"           // Arabic
   }
   ```

3. **Register the language** in `src/i18n/index.ts`:
   ```typescript
   export const SUPPORTED_LANGUAGES = [
     { code: 'en', name: 'English', dir: 'ltr' },
     { code: 'fr', name: 'Français', dir: 'ltr' },
     { code: 'XX', name: 'Your Language Name', dir: 'ltr' }, // add here
   ] as const;
   ```

4. **For RTL languages (Arabic, Hebrew, Urdu, etc.):**
   - Set `dir: 'rtl'` in the language definition
   - The app's root element adds `dir="rtl"` automatically when an RTL language is active
   - Test all layouts in RTL mode — flex/grid directions may need CSS adjustments

5. **Test your translation:**
   - Switch to the language in Settings > Language
   - Check every screen for untranslated keys (they appear as raw keys like `invoice.title`)
   - Check number and date formatting for your locale

6. **Add a test:** Add your language code to the test in `src-tauri/src/commands/settings.rs` that validates all language codes.

7. **Update documentation:** Add your language to the table in `docs/I18N.md`.

---

## How to Add a New Tax Rate

Tax rates are defined in `src-tauri/src/services/tax.rs` in the `default_tax_rates()` function.

```rust
// In src-tauri/src/services/tax.rs
pub fn default_tax_rates() -> Vec<TaxRate> {
    vec![
        // Add your country's tax rates here
        TaxRate {
            id: Uuid::new_v4().to_string(),
            name: "VAT".to_string(),
            country_code: "KE".to_string(),   // ISO 3166-1 alpha-2 country code
            rate_bps: 1600,                    // 16.00% = 1600 basis points
            tax_type: TaxType::Vat,
            is_default: true,
            applies_to_services: true,
            applies_to_goods: true,
        },
        // Your new tax rate:
        TaxRate {
            id: Uuid::new_v4().to_string(),
            name: "GST".to_string(),           // The name as it appears on invoices
            country_code: "BD".to_string(),    // Bangladesh
            rate_bps: 1500,                    // 15.00%
            tax_type: TaxType::Gst,
            is_default: true,
            applies_to_services: true,
            applies_to_goods: true,
        },
    ]
}
```

**For withholding tax** (a tax deducted at source, applied to the payment not the invoice total):
```rust
TaxRate {
    name: "WHT (Services)".to_string(),
    country_code: "NG".to_string(),
    rate_bps: 500,                       // 5.00%
    tax_type: TaxType::Withholding,
    // ...
}
```

Add a test in `src-tauri/src/services/tax.rs` that verifies your rate calculates correctly:
```rust
#[test]
fn test_bangladesh_gst() {
    let rate = TaxRate { rate_bps: 1500, tax_type: TaxType::Gst, ..Default::default() };
    // 100,000 (= 1000.00 BDT) × 15% = 15,000 (= 150.00 BDT)
    assert_eq!(calculate_tax_exclusive(100_000, &rate), 15_000);
}
```

---

## How to Add a New Currency

Currencies are defined in `src-tauri/src/models/currency.rs`.

```rust
// In src-tauri/src/models/currency.rs
pub const SUPPORTED_CURRENCIES: &[Currency] = &[
    // Existing currencies...
    Currency {
        code: "BDT",           // ISO 4217 currency code
        name: "Bangladeshi Taka",
        symbol: "৳",
        decimal_places: 2,     // 0 for currencies with no minor unit (UGX, XOF, XAF)
        country_codes: &["BD"],
    },
];
```

Also update:
1. `src/utils/currency.ts` — add the display formatter and symbol
2. `src/i18n/en.json` — add `currency.BDT: "Bangladeshi Taka"` and translate in all other language files
3. Update the currency table in `README.md` and `docs/API.md`

---

## Sprint Process

All contributions in this repository follow the sprint workflow defined in:

- `docs/SPRINT_PROCESS.md`
- `docs/MAINTAINER_CHECKLIST.md` (maintainer handoff/governance checks)

Before opening a PR, ensure your sprint includes:

1. A sprint record in `docs/sprints/sprint-XXX-<slug>.md`
2. Implementation and documentation updates in the same change set
3. Validation evidence (commands + results) captured in the PR body
4. Quality gate execution based on `docs/QUALITY_GATE.md`

All sprint PRs must be merged with **Squash and merge**.

When using automation agents, isolate each agent in a separate workspace and perform all final git operations from one integration checkout.

---

## Documentation Requirements

900Invoice is a global open-source project. Contributors must be able to understand changes without private context.

When your change touches behavior, data model, APIs, workflows, or contributor expectations, update docs in the same PR. At minimum:

1. Update one or more of:
   - `README.md`
   - `docs/API.md`
   - `docs/ARCHITECTURE.md`
   - `docs/DEPLOYMENT.md`
   - `docs/I18N.md`
   - `docs/TEMPLATES.md`
   - ADRs in `docs/adr/` for architectural decisions
2. Add or update the sprint record in `docs/sprints/`
3. Describe doc impact explicitly in the PR body

PRs that change behavior without documentation may be asked to revise before merge.

---

## Pull Request Process

1. **Fork** the repository on GitHub and clone your fork.

2. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feat/add-bangladesh-tax-rates
   ```
   Branch names: `feat/`, `fix/`, `docs/`, `refactor/`, `test/`

3. **Make your changes.** Keep changes focused — one feature or fix per PR.

4. **Write tests** for any new logic, especially tax calculations and money arithmetic.

5. **Run the full test suite:**
   ```bash
   cargo test --manifest-path src-tauri/Cargo.toml
   cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
   cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
   npm run check
   ```

6. **Commit** using the format below.

7. **Push** to your fork and open a Pull Request against `900Labs/900Invoice:main`.

8. **Fill out the PR template** completely. Incomplete PRs may be closed.
   - PR title must be descriptive and readable
   - PR body must include summary, scope, validation, risk, and documentation impact

9. **Address review feedback.** Maintainers will review within 5 business days.

10. **Merge via squash merge** and share the merged PR URL in your sprint handoff.

---

## Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

**Types:**
- `feat` — new feature
- `fix` — bug fix
- `docs` — documentation only
- `refactor` — code change that neither fixes a bug nor adds a feature
- `test` — adding or correcting tests
- `chore` — build process, dependency updates

**Scopes (match the module):**
`invoices`, `clients`, `products`, `payments`, `taxes`, `pdf`, `recurring`, `reports`, `i18n`, `settings`, `db`, `ci`

**Examples:**
```
feat(invoices): add duplicate invoice command
fix(taxes): correct WHT basis point calculation for Nigeria
docs(i18n): add step-by-step guide for RTL languages
feat(i18n): add Swahili translation
test(taxes): add unit tests for tax-inclusive calculation
chore(deps): update tauri to 2.1.0
```

---

## Code Review

Maintainer review is strongly recommended for all PRs, but branch protection currently does not require approving reviews (`required_approving_review_count = 0`) to avoid self-approval deadlocks in low-capacity maintainer workflows.

**What we look for:**
- Does the code follow money/tax rules (integers, basis points, no floats)?
- Do all tests pass?
- Is the code well-documented?
- Are error cases handled (no `unwrap()`)?
- Is all user-visible text in `i18n`?
- Is the change consistent with the rest of the codebase?

**Review turnaround:** Maintainers aim to respond within 5 business days. We are a small team distributed across time zones — thank you for your patience.

---

## Governance Profile Onboarding (Maintainers)

Reference checklist: `docs/MAINTAINER_CHECKLIST.md`

Choose a governance profile based on maintainer capacity and repository risk:

1. `solo`
   - Best for one active maintainer.
   - Defaults: `0` required approvals, no code-owner requirement, no last-push approval requirement.
2. `small-team`
   - Best for 2-5 maintainers with shared review coverage.
   - Defaults: `1` required approval, last-push approval required.
3. `enterprise`
   - Best for larger teams with strict change-control expectations.
   - Defaults: `2` required approvals, code-owner reviews + last-push approval required.

Apply and verify examples:

```bash
# solo (default)
GOVERNANCE_PROFILE=solo ./scripts/apply-repo-policy.sh 900Labs/900Invoice main
GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main

# small-team
GOVERNANCE_PROFILE=small-team ./scripts/apply-repo-policy.sh 900Labs/900Invoice main
GOVERNANCE_PROFILE=small-team ./scripts/verify-repo-policy.sh 900Labs/900Invoice main

# enterprise
GOVERNANCE_PROFILE=enterprise ./scripts/apply-repo-policy.sh 900Labs/900Invoice main
GOVERNANCE_PROFILE=enterprise ./scripts/verify-repo-policy.sh 900Labs/900Invoice main
```

Automation defaults:

1. Set repository variable `GOVERNANCE_PROFILE` to your target profile.
2. Optionally set overrides:
   - `REQUIRED_APPROVING_REVIEW_COUNT`
   - `REQUIRE_CODE_OWNER_REVIEWS`
   - `REQUIRE_LAST_PUSH_APPROVAL`
3. Governance audit and release gate workflows consume the same contract.

---

## Getting Help

- **GitHub Discussions**: [github.com/900Labs/900Invoice/discussions](https://github.com/900Labs/900Invoice/discussions) — for questions, ideas, and general discussion
- **GitHub Issues**: [github.com/900Labs/900Invoice/issues](https://github.com/900Labs/900Invoice/issues) — for confirmed bugs and feature requests
- **Email**: [opensource@900labs.com](mailto:opensource@900labs.com) — for sensitive matters not suitable for public discussion
- **Website**: [900labs.com](https://www.900labs.com)

If you are new to Rust, we recommend:
- [The Rust Book](https://doc.rust-lang.org/book/) (free, available in many languages)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) (free, hands-on)

If you are new to Tauri, we recommend:
- [Tauri v2 Documentation](https://v2.tauri.app)

---

*900Invoice is built by and for the global community. Your contribution matters.*
