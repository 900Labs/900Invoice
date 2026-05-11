# API Reference

This document is the canonical reference for Tauri commands exposed by the Rust backend.

Source of truth:

1. Command registration: `src-tauri/src/lib.rs`
2. Command implementations: `src-tauri/src/commands/*.rs`
3. Data contracts: `src-tauri/src/models/*.rs`

Frontend calls commands via `invoke()`:

```ts
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('get_invoice', { id: invoiceId });
```

Tauri command argument names use the macro default `camelCase` at the outer
`invoke()` boundary. Nested model payloads retain the Rust/serde field names
shown in `src-tauri/src/models/*.rs` unless a model explicitly declares a serde
rename rule.

All commands return `Result<T, String>` in Rust. On the frontend, failures reject with an error string.

---

## Canonical Command Catalog

The list below is machine-validated against `src-tauri/src/lib.rs` by `./scripts/verify-api-doc-commands.sh`.

<!-- COMMAND_CATALOG_START -->
- `get_business_profile`
- `update_business_profile`
- `list_clients`
- `get_client`
- `create_client`
- `update_client`
- `delete_client`
- `search_clients`
- `list_invoices`
- `get_invoice`
- `create_invoice`
- `update_invoice`
- `delete_invoice`
- `finalize_invoice`
- `mark_invoice_sent`
- `void_invoice`
- `duplicate_invoice`
- `search_invoices`
- `add_line_item`
- `update_line_item`
- `remove_line_item`
- `reorder_line_items`
- `list_tax_rates`
- `create_tax_rate`
- `update_tax_rate`
- `delete_tax_rate`
- `get_tax_rates_for_country`
- `calculate_invoice_taxes`
- `generate_invoice_pdf`
- `get_pdf_preview_data`
- `list_payments`
- `record_payment`
- `delete_payment`
- `get_invoice_payment_summary`
- `list_recurring`
- `create_recurring`
- `update_recurring`
- `delete_recurring`
- `generate_due_recurring`
- `list_products`
- `get_product`
- `create_product`
- `update_product`
- `delete_product`
- `search_products`
- `get_exchange_rates`
- `get_cached_rate`
- `convert_currency`
- `upsert_exchange_rates`
- `import_clients_csv`
- `export_clients_csv`
- `import_products_csv`
- `export_products_csv`
- `export_invoices_csv`
- `backup_database`
- `restore_database`
- `get_settings`
- `get_setting`
- `update_setting`
- `get_invoice_sequence`
- `update_invoice_sequence`
- `get_changelog`
- `get_changes_since`
<!-- COMMAND_CATALOG_END -->

---

## Module Reference

### Business

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `get_business_profile` | none | `BusinessProfile \| null` (JSON) | Returns `null` when not configured. |
| `update_business_profile` | `{ update: UpdateBusinessProfile }` | `BusinessProfile` (JSON) | `logo_path` is validated and copied into app-managed storage. |

### Clients

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `list_clients` | none | `Client[]` (JSON) | Ordered by query layer defaults. |
| `get_client` | `{ id: string }` | `Client` (JSON) | Errors when missing. |
| `create_client` | `{ client: CreateClient }` | `Client` (JSON) | |
| `update_client` | `{ id: string, update: UpdateClient }` | `Client` (JSON) | |
| `delete_client` | `{ id: string }` | `void` | |
| `search_clients` | `{ query: string }` | `Client[]` (JSON) | |

### Invoices

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `list_invoices` | none | `Invoice[]` (JSON) | |
| `get_invoice` | `{ id: string }` | `InvoiceWithDetails` (JSON) | Includes line items, taxes, and payments. |
| `create_invoice` | `{ invoice: CreateInvoice }` | `Invoice` (JSON) | Creates draft invoice and snapshots the cached rate to USD. |
| `update_invoice` | `{ id: string, update: UpdateInvoice }` | `Invoice` (JSON) | Draft-only mutation enforced; currency/date changes refresh the cached rate snapshot. |
| `delete_invoice` | `{ id: string }` | `void` | Draft-only mutation enforced. |
| `finalize_invoice` | `{ id: string }` | `InvoiceWithDetails` (JSON) | Requires draft status; assigns number when missing and backfills a missing exchange-rate snapshot. |
| `mark_invoice_sent` | `{ id: string }` | `InvoiceWithDetails` (JSON) | Requires finalized status; stamps `sent_at`. |
| `void_invoice` | `{ id: string }` | `InvoiceWithDetails` (JSON) | Rejects already-void invoices. |
| `duplicate_invoice` | `{ id: string }` | `InvoiceWithDetails` (JSON) | Copies line items into a new draft invoice with a fresh exchange-rate snapshot. |
| `search_invoices` | `{ query: string }` | `Invoice[]` (JSON) | |

### Line Items

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `add_line_item` | `{ lineItem: CreateLineItem }` | `LineItem` (JSON) | Draft-only invoice check; recalculates invoice totals and tax rows. |
| `update_line_item` | `{ id: string, update: UpdateLineItem }` | `LineItem` (JSON) | Draft-only invoice check; recalculates totals. |
| `remove_line_item` | `{ id: string }` | `void` | Draft-only invoice check; recalculates totals. |
| `reorder_line_items` | `{ orderedIds: string[] }` | `void` | Draft-only check based on first item. |

Line items preserve `tax_rate_id` when supplied. Recalculation uses that tax-rate identity first, with `tax_rate_bps` as a backward-compatible fallback for older line items.

### Taxes

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `list_tax_rates` | none | `TaxRate[]` (JSON) | |
| `create_tax_rate` | `{ taxRate: CreateTaxRate }` | `TaxRate` (JSON) | |
| `update_tax_rate` | `{ id: string, update: UpdateTaxRate }` | `TaxRate` (JSON) | |
| `delete_tax_rate` | `{ id: string }` | `void` | |
| `get_tax_rates_for_country` | `{ countryCode: string }` | `TaxRate[]` (JSON) | |
| `calculate_invoice_taxes` | `{ invoiceId: string }` | `InvoiceTax[]` (JSON) | Replaces persisted invoice tax rows and updates totals. |

### PDF

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `generate_invoice_pdf` | `{ invoiceId: string }` | `string` | Returns base64-encoded PDF bytes suitable for native file export. |
| `get_pdf_preview_data` | `{ invoiceId: string }` | JSON object | Includes `invoice`, `business`, `html`, and `preview`. |

### Payments

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `list_payments` | `{ invoiceId: string }` | `Payment[]` (JSON) | |
| `record_payment` | `{ payment: CreatePayment }` | `Payment` (JSON) | Requires `amount_minor > 0`, invoice not `draft`/`void`, and currency match. |
| `delete_payment` | `{ id: string }` | `void` | Recomputes invoice paid totals; may revert status from `paid`. |
| `get_invoice_payment_summary` | `{ invoiceId: string }` | `PaymentSummary` (JSON) | |

### Recurring

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `list_recurring` | none | `RecurringInvoice[]` (JSON) | |
| `create_recurring` | `{ recurring: CreateRecurring }` | `RecurringInvoice` (JSON) | |
| `update_recurring` | `{ id: string, update: UpdateRecurring }` | `RecurringInvoice` (JSON) | |
| `delete_recurring` | `{ id: string }` | `void` | |
| `generate_due_recurring` | none | `InvoiceWithDetails[]` (JSON) | Generates invoices for schedules due today, copies template line items and tax rows, snapshots the generated invoice exchange rate, and advances schedules. |

### Products

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `list_products` | none | `Product[]` (JSON) | |
| `get_product` | `{ id: string }` | `Product` (JSON) | Errors when missing. |
| `create_product` | `{ product: CreateProduct }` | `Product` (JSON) | |
| `update_product` | `{ id: string, update: UpdateProduct }` | `Product` (JSON) | |
| `delete_product` | `{ id: string }` | `void` | |
| `search_products` | `{ query: string }` | `Product[]` (JSON) | |

### Exchange Rates

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `get_exchange_rates` | `{ baseCurrency: string }` | `ExchangeRate[]` (JSON) | Returns latest rates for base currency. |
| `get_cached_rate` | `{ baseCurrency: string, targetCurrency: string, date?: string }` | `ExchangeRate \| null` (JSON) | Date defaults to current local date. |
| `convert_currency` | `{ fromCurrency: string, toCurrency: string, amountMinor: number, date?: string }` | `ConversionResult` (JSON) | Uses cached rates; same-currency shortcut returns 1.0 rate. |
| `upsert_exchange_rates` | `{ rates: ExchangeRate[] }` | `void` | Batch upsert. |

Default offline exchange-rate rows are seeded during app startup. Invoice creation, draft currency/date updates, finalization backfill, duplication, and recurring generation store `exchange_rate_to_usd` plus `exchange_rate_date` on the invoice for audit history.

### Import / Export

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `import_clients_csv` | `{ csvContent: string }` | JSON object | Returns `{ imported, errors }`; expects `name,email,phone,address,city,country,country_code,tax_id,currency_code,payment_terms_days,notes`; legacy client CSV files without `tax_id` and `notes` are still accepted. |
| `export_clients_csv` | none | `string` | CSV export; sanitizes formula-like cells. |
| `import_products_csv` | `{ csvContent: string }` | JSON object | Returns `{ imported, errors }`; expects `name,description,default_price,default_currency,default_tax_rate_bps,default_tax_rate_id,unit,is_active`; legacy product CSV files without `default_tax_rate_id` are still accepted. |
| `export_products_csv` | none | `string` | CSV export for all products, including inactive rows; sanitizes formula-like cells. |
| `export_invoices_csv` | none | `string` | CSV export; sanitizes formula-like cells for string cells. |
| `backup_database` | none | JSON object | Exports a versioned app data snapshot including clients, invoices, line items, invoice taxes, payments, products, tax rates, settings, business profile, recurring schedules, exchange rates, and invoice sequences. |
| `restore_database` | `{ backup: object }` | JSON object | Additive restore (`INSERT OR IGNORE`); does not drop or overwrite existing rows. Returns per-table inserted counts. |

### Settings

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `get_settings` | none | `Record<string, unknown>` (JSON) | |
| `get_setting` | `{ key: string }` | JSON value or `null` | Stored values are JSON-serialized in DB. |
| `update_setting` | `{ key: string, value: unknown }` | `void` | Persists JSON-serialized value. |
| `get_invoice_sequence` | none | `InvoiceSequenceInfo` (JSON) | Uses default sequence row. |
| `update_invoice_sequence` | `{ sequence: InvoiceSequenceInfo }` | `void` | Updates default sequence row. |

### Sync

| Command | Invoke Args | Returns | Notes |
|---|---|---|---|
| `get_changelog` | none | `ChangeLogEntry[]` (JSON) | |
| `get_changes_since` | `{ since: string }` | `ChangeLogEntry[]` (JSON) | Timestamp string expected by query layer. |

---

## Core Data Contracts

Primary Rust model definitions:

- `src-tauri/src/models/business.rs`
- `src-tauri/src/models/client.rs`
- `src-tauri/src/models/invoice.rs`
- `src-tauri/src/models/line_item.rs`
- `src-tauri/src/models/payment.rs`
- `src-tauri/src/models/product.rs`
- `src-tauri/src/models/recurring.rs`
- `src-tauri/src/models/tax.rs`
- `src-tauri/src/models/exchange_rate.rs`

Money and tax conventions:

1. Money is stored as integer minor units (`i64`).
2. Tax rates are stored in basis points (`i32`).
3. Frontend formatting should only occur at display boundaries.

---

## Renamed / Removed Commands

Earlier draft documentation used outdated names. Use the canonical catalog above.

Common mappings:

| Legacy name | Current command |
|---|---|
| `get_business` | `get_business_profile` |
| `upsert_business` | `update_business_profile` |
| `delete_line_item` | `remove_line_item` |
| `generate_pdf` | `generate_invoice_pdf` |
| `list_exchange_rates` | `get_exchange_rates` |
| `upsert_exchange_rate` | `upsert_exchange_rates` |
| `list_recurring_schedules` | `list_recurring` |
| `create_recurring_schedule` | `create_recurring` |
| `delete_recurring_schedule` | `delete_recurring` |
| `set_setting` | `update_setting` |
| `get_changelog_since` | `get_changes_since` |

Legacy names removed from docs with no current command equivalent:

- `mark_invoice_sent`
- `save_pdf`
- `open_pdf`
- `import_invoices_csv`
- `get_dashboard_metrics`
- `get_revenue_report`
- `get_tax_summary_report`
- `get_aging_report`
