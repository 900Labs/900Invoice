# Architecture Overview

This document describes the technical architecture of 900Invoice for developers who want to understand, modify, or contribute to the codebase.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        900Invoice                                │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    Tauri WebView                         │   │
│  │                                                          │   │
│  │   ┌──────────────────────────────────────────────────┐  │   │
│  │   │              Svelte 5 Frontend                    │  │   │
│  │   │                                                    │  │   │
│  │   │  ┌────────────┐  ┌────────────┐  ┌────────────┐  │  │   │
│  │   │  │ Components │  │   Stores   │  │    i18n    │  │  │   │
│  │   │  │ (UI views) │  │  (Runes)   │  │ (6 langs)  │  │  │   │
│  │   │  └────────────┘  └────────────┘  └────────────┘  │  │   │
│  │   │  ┌────────────┐  ┌────────────────────────────┐  │  │   │
│  │   │  │   Utils    │  │   lib/ (Tauri IPC wrappers) │  │  │   │
│  │   │  │ (currency, │  │   invoke() typed wrappers   │  │  │   │
│  │   │  │  date fmt) │  └────────────────────────────┘  │  │   │
│  │   │  └────────────┘                                   │  │   │
│  │   └──────────────────────────┬───────────────────────┘  │   │
│  │                              │ Tauri IPC (JSON over IPC) │   │
│  │   ┌──────────────────────────▼───────────────────────┐  │   │
│  │   │              Rust Backend                         │  │   │
│  │   │                                                    │  │   │
│  │   │  ┌────────────┐  ┌────────────┐  ┌────────────┐  │  │   │
│  │   │  │  Commands  │  │  Services  │  │   Models   │  │  │   │
│  │   │  │ (~45 cmds) │  │ (tax, pdf, │  │ (Invoice,  │  │  │   │
│  │   │  │            │  │ numbering) │  │  Client..) │  │  │   │
│  │   │  └────────────┘  └────────────┘  └────────────┘  │  │   │
│  │   │  ┌────────────┐  ┌────────────────────────────┐  │  │   │
│  │   │  │     DB     │  │      tokio-cron-scheduler  │  │  │   │
│  │   │  │ (rusqlite) │  │  (recurring invoice jobs)  │  │  │   │
│  │   │  └─────┬──────┘  └────────────────────────────┘  │  │   │
│  │   └────────┼─────────────────────────────────────────┘  │   │
│  └────────────┼──────────────────────────────────────────── ┘   │
│               │                                                  │
│    ┌──────────▼──────────┐    ┌───────────────────────────┐    │
│    │      SQLite DB       │    │  typst-bake (PDF engine)  │    │
│    │  900invoice.db       │    │  (embedded Typst binary)  │    │
│    │  (single file, local)│    │                           │    │
│    └─────────────────────┘    └───────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

### Svelte 5 Frontend (`src/`)

The frontend is responsible exclusively for:
- Rendering the user interface
- Managing ephemeral UI state (form values, modal visibility, active tab)
- Formatting data for display (currency amounts, dates, percentages)
- Calling Rust commands via the Tauri IPC bridge
- Internationalization (language selection, text rendering)

The frontend does **not** perform any business logic calculations. Tax calculations, invoice numbering, totals, and payment status all happen in Rust.

### Tauri IPC Bridge

Tauri provides a structured message-passing system between the WebView (JavaScript/TypeScript) and the Rust backend. Each operation is a named command:

```typescript
// Frontend: typed wrapper in src/lib/
import { invoke } from '@tauri-apps/api/core';

export async function createInvoice(input: CreateInvoiceInput): Promise<Invoice> {
  return invoke('create_invoice', { input });
}
```

All data crosses the IPC bridge as JSON. Rust structs with `#[derive(Serialize, Deserialize)]` serialize automatically.

### Rust Backend (`src-tauri/src/`)

The Rust backend owns all business logic and data persistence:

- **Commands**: Thin IPC handlers. Validate inputs, call services, return results.
- **Services**: Business logic — tax calculation, invoice numbering, PDF generation, recurring scheduling.
- **Models**: Data structures shared between commands, services, and the database layer.
- **DB**: All SQLite interactions. Typed query functions. Schema and migrations.

### SQLite Database

Single file at `{APP_DATA_DIR}/900invoice.db`. All application data is stored here. No external process can read or write this file while 900Invoice is running (SQLite file locking).

---

## Data Flow

### Standard Read Operation

```
User clicks "View Invoice"
  → Svelte component calls invoiceStore.load(id)
  → Store calls invoke('get_invoice', { id })
  → Tauri routes to get_invoice command (Rust)
  → Command calls db::invoices::get_invoice(conn, &id)
  → DB returns Ok(Invoice) or Err(DbError::NotFound)
  → Command returns Ok(invoice_json) or Err("NOT_FOUND")
  → invoke() resolves with invoice object
  → Store updates $state
  → Svelte re-renders component with new data
```

### Invoice Creation Flow

```
User submits invoice form
  → validate_invoice_form() (frontend validation — UX only)
  → invoke('create_invoice', { input: CreateInvoiceInput })
  → Rust: create_invoice command
    → validate inputs (client exists, line items non-empty, etc.)
    → services::numbering::next_number(conn) → "INV-2026-0042"
    → calculate totals via services::tax::calculate_totals(&line_items, &tax_rates)
    → db::invoices::insert(conn, &invoice)
    → db::sync::log_change(conn, "invoices", "INSERT", &invoice.id)
  → Returns Ok(Invoice)
  → Store updates; router navigates to invoice detail view
```

### PDF Generation Flow

```
User clicks "Generate PDF"
  → invoke('generate_pdf', { invoice_id })
  → Rust: generate_pdf command
    → db::invoices::get_full_invoice(conn, id)  // invoice + client + line items
    → services::pdf::render(full_invoice)
      → load invoice.typ template
      → substitute template variables with invoice data
      → call typst_bake::compile(template_string)
      → returns Vec<u8> (PDF bytes)
    → write bytes to temp file
    → shell::open(temp_file_path)  // opens in system PDF viewer
  → Returns Ok(file_path)
```

---

## Money Handling Conventions

This is the most critical section for contributors working on financial logic.

### All money is stored as i64 integer minor units

Every monetary amount in the database, in Rust structs, and crossing the IPC bridge is stored as an `i64` representing the smallest currency unit (cent, kobo, paise, etc.).

```
KES 1,500.00  →  stored as  150000
NGN 25,000.00 →  stored as  2500000
UGX 50,000    →  stored as  50000    (UGX has no minor unit)
XOF 10,000    →  stored as  10000    (XOF has no minor unit)
```

**Why**: Floating-point arithmetic introduces rounding errors. `0.1 + 0.2 = 0.30000000000000004` in IEEE 754. Over thousands of invoices, these errors compound and produce incorrect tax filings.

### Display conversion happens only in the frontend

```typescript
// src/utils/currency.ts
export function formatAmount(minorUnits: number, currencyCode: string): string {
  const { decimals, symbol } = CURRENCIES[currencyCode];
  const major = minorUnits / Math.pow(10, decimals);
  return `${symbol}${major.toFixed(decimals)}`;
}

// User types "1500.00" in a form input
export function parseAmount(input: string, currencyCode: string): number {
  const { decimals } = CURRENCIES[currencyCode];
  return Math.round(parseFloat(input) * Math.pow(10, decimals));
}
```

**Critical rule**: The parsed integer is what gets sent to Rust. The formatted string is what gets displayed. Never send a float to Rust.

### Tax rates are stored as i32 basis points

A basis point = 1/100 of a percent. 100 bps = 1.00%.

```
16.00%  →  1600
7.5%    →  750
2.5%    →  250
```

### Tax calculation formulas

**Tax-exclusive** (tax added on top of the subtotal):
```
tax_amount = (subtotal * rate_bps) / 10000
total = subtotal + tax_amount
```

**Tax-inclusive** (tax is already included in the price):
```
tax_amount = subtotal - (subtotal * 10000) / (10000 + rate_bps)
net_amount = subtotal - tax_amount
```

**Rounding**: Always round to the nearest minor unit using integer arithmetic:
```rust
// Round half-up
let tax = (subtotal * rate_bps as i64 + 5000) / 10000;
```

**Multi-tax example** (Ghana: VAT 15% + NHIL 2.5% + GETFund 2.5% on a KES 10,000.00 invoice):
```
subtotal:    1_000_000  (KES 10,000.00)
VAT 15%:       150_000  (1_000_000 * 1500 / 10000)
NHIL 2.5%:      25_000  (1_000_000 * 250  / 10000)
GETFund 2.5%:   25_000  (1_000_000 * 250  / 10000)
total tax:     200_000
total:       1_200_000  (KES 12,000.00)
```

---

## Invoice Lifecycle State Machine

```
                    ┌─────────┐
               ┌──▶│  DRAFT  │◀──────────────────────┐
               │   └────┬────┘                         │
               │        │ finalize_invoice()            │ void → redraft (creates
               │        ▼                               │ new draft from void)
               │   ┌──────────┐                        │
               │   │FINALIZED │                        │
               │   └────┬─────┘                        │
               │        │ mark_sent() (optional)        │
               │        ▼                               │
               │   ┌─────────┐                         │
               │   │  SENT   │                         │
               │   └────┬────┘                         │
               │        │ record_payment() (full)       │
               │        ▼                               │
               │   ┌─────────┐                         │
               │   │  PAID   │                         │
               │   └─────────┘                         │
               │                                        │
               │   void_invoice() (from FINALIZED,     │
               └── SENT, or PAID)                      │
                         │                              │
                         ▼                              │
                    ┌─────────┐                         │
                    │  VOID   │─────────────────────────┘
                    └─────────┘
```

**State transition rules:**
- `DRAFT` → `FINALIZED`: assigns invoice number, locks line items and tax rates
- `FINALIZED` → `SENT`: records a sent timestamp; optional but recommended
- `FINALIZED` or `SENT` → `PAID`: triggered by `record_payment()` when payments equal total
- Any non-DRAFT → `VOID`: records void reason; voids cannot be deleted
- `VOID` can generate a new `DRAFT` (for corrections), which creates a new invoice with a new number

---

## Recurring Invoice Scheduling

The scheduler uses `tokio-cron-scheduler` running in a background async task within the Tauri application process.

```
On application startup:
  1. Load all active recurring schedules from DB
  2. For each schedule, check: has the job missed any runs since last_run?
     - If yes: generate all missed invoices (up to a cap of 12)
  3. Register all schedules with the cron scheduler
  4. Scheduler fires jobs at the configured interval
  5. Each job: generate next invoice → insert to DB → update last_run timestamp
```

**Cron expressions used:**
- Weekly: `0 8 * * 1` (Monday 8 AM local time)
- Monthly: `0 8 1 * *` (1st of month, 8 AM)
- Quarterly: `0 8 1 1,4,7,10 *` (1st of Jan, Apr, Jul, Oct)
- Annual: `0 8 1 1 *` (January 1st)

---

## Changelog-Based Sync Design

900Invoice v1.0.0 is offline-only, but the database schema includes infrastructure for future multi-device sync. Every write operation logs a change record:

```sql
CREATE TABLE changelog (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type TEXT NOT NULL,    -- 'invoices', 'clients', etc.
    entity_id   TEXT NOT NULL,    -- UUID of the changed record
    operation   TEXT NOT NULL,    -- 'INSERT', 'UPDATE', 'DELETE'
    data        TEXT,             -- JSON snapshot of the record
    changed_at  TEXT NOT NULL,    -- ISO 8601 timestamp
    device_id   TEXT NOT NULL     -- UUID of the device that made the change
);
```

This table enables future CRDT-based or last-write-wins sync without schema changes. For v1.0.0, it is written but not consumed.

---

## PDF Generation Pipeline

```
Invoice data (from DB)
  │
  ▼
services::pdf::render(&FullInvoice)
  │
  ├── Load template: src-tauri/src/templates/invoice.typ
  │
  ├── Template variable substitution:
  │     {{business_name}} → "Acme Ltd"
  │     {{invoice_number}} → "INV-2026-0042"
  │     {{line_items}} → Typst table rows
  │     {{total}} → "KES 12,000.00"
  │
  ├── typst_bake::compile(rendered_template: &str) → Result<Vec<u8>>
  │     (calls the embedded Typst compiler)
  │
  └── Returns PDF bytes → written to temp file → opened in system viewer
```

The Typst template is embedded in the binary at compile time using `include_str!()`. The template can be customized before building — see [TEMPLATES.md](TEMPLATES.md).

---

## Database Schema Overview

Key tables (see `src-tauri/src/db/schema.rs` for the complete DDL):

| Table | Purpose |
|-------|---------|
| `businesses` | Business profile (name, address, logo, tax ID) |
| `clients` | Client contact information and defaults |
| `invoices` | Invoice header (number, status, dates, totals) |
| `invoice_line_items` | Line items belonging to invoices |
| `invoice_taxes` | Tax rates applied to specific invoices (snapshot) |
| `payments` | Payment records against invoices |
| `products` | Product/service catalog |
| `tax_rates` | Available tax rates by country |
| `currencies` | Supported currencies with decimal places |
| `exchange_rates` | Cached exchange rates with timestamp |
| `recurring_schedules` | Recurring invoice schedule definitions |
| `settings` | Key-value application settings |
| `sequence_counters` | Gap-free invoice number sequence state |
| `changelog` | Write-ahead log for future sync |

All primary keys are TEXT (UUID v4). All monetary columns are INTEGER (minor units). All timestamp columns are TEXT (ISO 8601).

---

## Related Documentation

- [API Reference](API.md) — complete Tauri command reference
- [ADR 001](adr/001-tauri-v2-desktop-framework.md) — why Tauri v2
- [ADR 002](adr/002-integer-money-storage.md) — why integer money storage
- [ADR 003](adr/003-gap-free-invoice-numbering.md) — gap-free invoice numbering
- [ADR 004](adr/004-offline-first-architecture.md) — offline-first design
- [ADR 005](adr/005-typst-bake-pdf-generation.md) — PDF generation with typst-bake
- [ADR 006](adr/006-apache-2-licensing.md) — Apache 2.0 licensing choice
