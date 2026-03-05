# API Reference

Complete reference for all Tauri commands exposed by the 900Invoice Rust backend. Commands are called from the Svelte frontend using `invoke()`:

```typescript
import { invoke } from '@tauri-apps/api/core';
const result = await invoke('command_name', { param1: value1 });
```

All commands return a `Promise<T>` that resolves with the return value on success, or rejects with an error string on failure.

---

## Error Handling

All commands return `Result<T, String>` in Rust. On the frontend, a rejected promise contains a human-readable error string. Common error patterns:

```typescript
try {
  const invoice = await invoke('get_invoice', { id });
} catch (error) {
  // error is a string like "NOT_FOUND: invoice abc-123 does not exist"
  console.error(error);
}
```

Error string prefixes:
- `NOT_FOUND:` — requested record does not exist
- `VALIDATION:` — input failed validation
- `CONFLICT:` — operation conflicts with current state (e.g., editing a finalized invoice)
- `DB_ERROR:` — database operation failed (rare; indicates a system-level problem)

---

## Module: Business Profile

### `get_business`

Returns the business profile. Returns `null` if not yet configured.

**Parameters:** none

**Returns:** `Business | null`

```typescript
interface Business {
  id: string;
  name: string;
  address: string;
  city: string;
  country_code: string;   // ISO 3166-1 alpha-2
  tax_id: string | null;
  email: string | null;
  phone: string | null;
  website: string | null;
  logo_path: string | null;
  default_currency: string;  // ISO 4217 currency code
  default_payment_terms: number;  // days
  invoice_prefix: string;   // e.g., "INV"
  created_at: string;       // ISO 8601
  updated_at: string;       // ISO 8601
}
```

**Example:**
```typescript
const business = await invoke<Business | null>('get_business');
if (!business) {
  // Show onboarding / business setup form
}
```

---

### `upsert_business`

Creates or updates the business profile. Only one business profile is supported per database.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `UpsertBusinessInput` | Yes | Business profile fields |

```typescript
interface UpsertBusinessInput {
  name: string;
  address: string;
  city: string;
  country_code: string;
  tax_id?: string;
  email?: string;
  phone?: string;
  website?: string;
  logo_path?: string;
  default_currency: string;
  default_payment_terms: number;
  invoice_prefix: string;
}
```

**Returns:** `Business`

**Errors:** `VALIDATION:` if name is empty or currency code is invalid.

---

## Module: Clients

### `list_clients`

Returns all clients, ordered by name.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `search` | `string \| null` | No | Full-text search query |

**Returns:** `Client[]`

```typescript
interface Client {
  id: string;         // UUID v4
  name: string;
  company: string | null;
  email: string | null;
  phone: string | null;
  address: string | null;
  city: string | null;
  country_code: string | null;
  tax_id: string | null;       // Client's VAT/GST number
  default_currency: string | null;
  default_payment_terms: number | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}
```

---

### `get_client`

Returns a single client by ID.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Client UUID |

**Returns:** `Client`

**Errors:** `NOT_FOUND:` if client does not exist.

---

### `create_client`

Creates a new client record.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `CreateClientInput` | Yes | Client fields |

```typescript
interface CreateClientInput {
  name: string;
  company?: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country_code?: string;
  tax_id?: string;
  default_currency?: string;
  default_payment_terms?: number;
  notes?: string;
}
```

**Returns:** `Client`

**Errors:** `VALIDATION:` if name is empty.

---

### `update_client`

Updates an existing client record.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Client UUID |
| `input` | `UpdateClientInput` | Yes | Fields to update (same shape as `CreateClientInput`) |

**Returns:** `Client`

**Errors:** `NOT_FOUND:`, `VALIDATION:`

---

### `delete_client`

Deletes a client. Fails if the client has existing invoices.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Client UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`, `CONFLICT: client has existing invoices`

---

## Module: Invoices

### `list_invoices`

Returns invoices with optional filtering.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `filter` | `InvoiceFilter` | No | Filter criteria |

```typescript
interface InvoiceFilter {
  status?: 'draft' | 'finalized' | 'sent' | 'paid' | 'void';
  client_id?: string;
  currency?: string;
  from_date?: string;   // ISO 8601 date
  to_date?: string;     // ISO 8601 date
  search?: string;      // full-text search
  limit?: number;       // default 100
  offset?: number;      // default 0
}
```

**Returns:** `InvoiceSummary[]`

```typescript
interface InvoiceSummary {
  id: string;
  number: string;           // e.g., "INV-2026-0042"
  status: InvoiceStatus;
  client_id: string;
  client_name: string;
  currency: string;
  subtotal: number;         // i64 minor units
  tax_total: number;        // i64 minor units
  total: number;            // i64 minor units
  amount_paid: number;      // i64 minor units
  amount_due: number;       // i64 minor units
  issue_date: string;       // ISO 8601 date
  due_date: string | null;
  created_at: string;
}
```

---

### `get_invoice`

Returns a full invoice with all line items and tax rates.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Invoice UUID |

**Returns:** `Invoice`

```typescript
interface Invoice extends InvoiceSummary {
  notes: string | null;
  terms: string | null;
  exchange_rate: number | null;   // rate snapshot at time of finalization
  exchange_rate_base: string | null;
  sent_at: string | null;
  paid_at: string | null;
  voided_at: string | null;
  void_reason: string | null;
  line_items: LineItem[];
  taxes: InvoiceTax[];
  payments: Payment[];
  updated_at: string;
}
```

**Errors:** `NOT_FOUND:`

---

### `create_invoice`

Creates a new invoice in DRAFT status.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `CreateInvoiceInput` | Yes | Invoice data |

```typescript
interface CreateInvoiceInput {
  client_id: string;
  currency: string;
  issue_date: string;          // ISO 8601 date
  due_date?: string;           // ISO 8601 date
  notes?: string;
  terms?: string;
  line_items: CreateLineItemInput[];
  tax_rate_ids: string[];      // tax rates to apply
}

interface CreateLineItemInput {
  description: string;
  quantity: number;            // integer (e.g., 5 for 5 hours)
  quantity_decimal?: number;   // fractional part in thousandths (e.g., 500 for 0.5)
  unit_price: number;          // i64 minor units
  product_id?: string;         // optional link to product catalog
}
```

**Returns:** `Invoice`

**Errors:** `VALIDATION:` if client_id is invalid, currency is unsupported, or line_items is empty.

---

### `update_invoice`

Updates a DRAFT invoice. Finalized, sent, paid, and void invoices cannot be updated.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Invoice UUID |
| `input` | `UpdateInvoiceInput` | Yes | Fields to update |

**Returns:** `Invoice`

**Errors:** `NOT_FOUND:`, `CONFLICT: invoice is not in DRAFT status`

---

### `finalize_invoice`

Transitions a DRAFT invoice to FINALIZED. Assigns the next sequential invoice number.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Invoice UUID |

**Returns:** `Invoice` (with number assigned)

**Errors:** `NOT_FOUND:`, `CONFLICT: invoice is not in DRAFT status`, `VALIDATION:` if invoice has no line items.

---

### `mark_invoice_sent`

Records that an invoice has been sent to the client (FINALIZED → SENT).

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Invoice UUID |

**Returns:** `Invoice`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if invoice is not FINALIZED.

---

### `void_invoice`

Voids an invoice. A void reason is required. Voided invoices are permanent records — they cannot be deleted.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Invoice UUID |
| `reason` | `string` | Yes | Reason for voiding |

**Returns:** `Invoice`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if invoice is already DRAFT or VOID, `VALIDATION:` if reason is empty.

---

### `duplicate_invoice`

Creates a new DRAFT invoice copied from an existing invoice. The new invoice has no number, no payments, and a new issue date (today).

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Invoice UUID to copy |

**Returns:** `Invoice` (new draft)

**Errors:** `NOT_FOUND:`

---

## Module: Line Items

### `add_line_item`

Adds a line item to a DRAFT invoice.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `invoice_id` | `string` | Yes | Invoice UUID |
| `input` | `CreateLineItemInput` | Yes | Line item data |

**Returns:** `LineItem`

```typescript
interface LineItem {
  id: string;
  invoice_id: string;
  description: string;
  quantity: number;
  quantity_decimal: number;
  unit_price: number;     // i64 minor units
  line_total: number;     // i64 minor units (computed: qty * unit_price)
  product_id: string | null;
  sort_order: number;
  created_at: string;
}
```

**Errors:** `NOT_FOUND:`, `CONFLICT:` if invoice is not DRAFT.

---

### `update_line_item`

Updates a line item on a DRAFT invoice.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Line item UUID |
| `input` | `UpdateLineItemInput` | Yes | Fields to update |

**Returns:** `LineItem`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if parent invoice is not DRAFT.

---

### `delete_line_item`

Deletes a line item from a DRAFT invoice.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Line item UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if parent invoice is not DRAFT.

---

### `reorder_line_items`

Updates the sort order of line items on a DRAFT invoice.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `invoice_id` | `string` | Yes | Invoice UUID |
| `item_ids` | `string[]` | Yes | Line item UUIDs in desired order |

**Returns:** `void`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if invoice is not DRAFT.

---

## Module: Tax Rates

### `list_tax_rates`

Returns all tax rates, optionally filtered by country code.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `country_code` | `string \| null` | No | ISO 3166-1 alpha-2 country code |

**Returns:** `TaxRate[]`

```typescript
interface TaxRate {
  id: string;
  name: string;                // e.g., "VAT", "GST", "WHT"
  country_code: string;
  rate_bps: number;            // i32 basis points (1600 = 16.00%)
  tax_type: 'vat' | 'gst' | 'withholding' | 'other';
  is_default: boolean;
  applies_to_services: boolean;
  applies_to_goods: boolean;
  created_at: string;
}
```

---

### `create_tax_rate`

Creates a custom tax rate.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `CreateTaxRateInput` | Yes | Tax rate definition |

```typescript
interface CreateTaxRateInput {
  name: string;
  country_code: string;
  rate_bps: number;
  tax_type: 'vat' | 'gst' | 'withholding' | 'other';
  applies_to_services: boolean;
  applies_to_goods: boolean;
}
```

**Returns:** `TaxRate`

**Errors:** `VALIDATION:` if rate_bps is negative or exceeds 10000 (100%).

---

### `delete_tax_rate`

Deletes a tax rate. Cannot delete tax rates that are referenced by existing invoices.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Tax rate UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if tax rate is in use.

---

## Module: PDF

### `generate_pdf`

Generates a PDF for an invoice and returns the path to the temporary file.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `invoice_id` | `string` | Yes | Invoice UUID |

**Returns:** `string` (absolute path to the generated PDF file)

**Errors:** `NOT_FOUND:`, `DB_ERROR:`, render errors from the Typst engine.

---

### `save_pdf`

Generates a PDF and saves it to a user-chosen location (opens a save dialog).

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `invoice_id` | `string` | Yes | Invoice UUID |

**Returns:** `string | null` — path where file was saved, or null if user cancelled.

**Errors:** `NOT_FOUND:`, render errors.

---

### `open_pdf`

Generates a PDF and opens it in the system default PDF viewer.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `invoice_id` | `string` | Yes | Invoice UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`, render errors, system errors if no PDF viewer is installed.

---

## Module: Payments

### `list_payments`

Returns all payments for a given invoice.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `invoice_id` | `string` | Yes | Invoice UUID |

**Returns:** `Payment[]`

```typescript
interface Payment {
  id: string;
  invoice_id: string;
  amount: number;       // i64 minor units
  currency: string;
  payment_date: string; // ISO 8601 date
  method: 'cash' | 'bank_transfer' | 'mobile_money' | 'cheque' | 'card' | 'other';
  reference: string | null;  // cheque number, bank ref, M-Pesa code, etc.
  notes: string | null;
  created_at: string;
}
```

---

### `record_payment`

Records a payment against an invoice. Automatically updates invoice status to PAID if fully settled.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `RecordPaymentInput` | Yes | Payment details |

```typescript
interface RecordPaymentInput {
  invoice_id: string;
  amount: number;         // i64 minor units
  currency: string;
  payment_date: string;
  method: PaymentMethod;
  reference?: string;
  notes?: string;
}
```

**Returns:** `Payment`

**Errors:** `NOT_FOUND:`, `CONFLICT:` if invoice is DRAFT or VOID, `VALIDATION:` if amount is zero or negative.

---

### `delete_payment`

Deletes a payment record. Reverts invoice status if it was PAID.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Payment UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`

---

## Module: Products

### `list_products`

Returns all products in the catalog.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `search` | `string \| null` | No | Full-text search |

**Returns:** `Product[]`

```typescript
interface Product {
  id: string;
  name: string;
  description: string | null;
  unit: string | null;         // e.g., "hours", "units", "kg"
  unit_price: number;          // i64 minor units
  currency: string;
  tax_rate_id: string | null;  // default tax rate for this product
  created_at: string;
  updated_at: string;
}
```

---

### `create_product`

Adds a product to the catalog.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `CreateProductInput` | Yes | Product data |

**Returns:** `Product`

**Errors:** `VALIDATION:` if name is empty or unit_price is negative.

---

### `update_product`

Updates an existing product.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Product UUID |
| `input` | `UpdateProductInput` | Yes | Fields to update |

**Returns:** `Product`

**Errors:** `NOT_FOUND:`, `VALIDATION:`

---

### `delete_product`

Deletes a product from the catalog. Does not affect existing invoice line items.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Product UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`

---

## Module: Exchange Rates

### `list_exchange_rates`

Returns cached exchange rates.

**Parameters:** none

**Returns:** `ExchangeRate[]`

```typescript
interface ExchangeRate {
  id: string;
  base_currency: string;
  target_currency: string;
  rate: number;          // stored as integer: rate * 1_000_000
  fetched_at: string;    // ISO 8601 timestamp
  source: string;        // e.g., "manual", "open_exchange_rates"
}
```

---

### `upsert_exchange_rate`

Creates or updates an exchange rate (manual entry, since the app is offline-first).

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `base` | `string` | Yes | Base currency code |
| `target` | `string` | Yes | Target currency code |
| `rate` | `number` | Yes | Rate as integer (rate × 1,000,000) |

**Returns:** `ExchangeRate`

**Errors:** `VALIDATION:` if rate is zero or negative.

---

## Module: Recurring Invoices

### `list_recurring_schedules`

Returns all recurring invoice schedules.

**Parameters:** none

**Returns:** `RecurringSchedule[]`

```typescript
interface RecurringSchedule {
  id: string;
  name: string;
  client_id: string;
  client_name: string;
  template_invoice_id: string;
  frequency: 'weekly' | 'monthly' | 'quarterly' | 'annual';
  next_run: string;       // ISO 8601 date
  last_run: string | null;
  is_active: boolean;
  created_at: string;
}
```

---

### `create_recurring_schedule`

Creates a new recurring invoice schedule.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `CreateRecurringScheduleInput` | Yes | Schedule configuration |

```typescript
interface CreateRecurringScheduleInput {
  name: string;
  template_invoice_id: string;
  frequency: 'weekly' | 'monthly' | 'quarterly' | 'annual';
  start_date: string;
  is_active: boolean;
}
```

**Returns:** `RecurringSchedule`

---

### `toggle_recurring_schedule`

Enables or disables a recurring schedule without deleting it.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Schedule UUID |
| `active` | `boolean` | Yes | New active state |

**Returns:** `RecurringSchedule`

**Errors:** `NOT_FOUND:`

---

### `delete_recurring_schedule`

Deletes a recurring schedule. Does not affect already-generated invoices.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | Yes | Schedule UUID |

**Returns:** `void`

**Errors:** `NOT_FOUND:`

---

## Module: Import / Export

### `import_clients_csv`

Imports clients from a CSV file. Opens a file picker dialog.

**Parameters:** none

**Returns:** `ImportResult`

```typescript
interface ImportResult {
  imported: number;
  skipped: number;
  errors: ImportError[];
}

interface ImportError {
  row: number;
  field: string;
  message: string;
}
```

---

### `export_clients_csv`

Exports all clients to a CSV file. Opens a save dialog.

**Parameters:** none

**Returns:** `string | null` — save path, or null if cancelled.

---

### `import_invoices_csv`

Imports invoices from a CSV file.

**Parameters:** none

**Returns:** `ImportResult`

---

### `export_invoices_csv`

Exports invoices to a CSV file with optional filtering.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `filter` | `InvoiceFilter \| null` | No | Applies same filter as `list_invoices` |

**Returns:** `string | null` — save path, or null if cancelled.

---

## Module: Reports

### `get_dashboard_metrics`

Returns real-time dashboard metrics.

**Parameters:** none

**Returns:** `DashboardMetrics`

```typescript
interface DashboardMetrics {
  total_invoiced_mtd: number;        // current month, i64 minor units
  total_collected_mtd: number;
  outstanding_balance: number;       // all unpaid finalized invoices
  overdue_balance: number;           // past due date
  invoice_count_mtd: number;
  paid_count_mtd: number;
  primary_currency: string;          // business default currency
}
```

---

### `get_revenue_report`

Returns revenue aggregated by period.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `from_date` | `string` | Yes | ISO 8601 date |
| `to_date` | `string` | Yes | ISO 8601 date |
| `currency` | `string \| null` | No | Filter by currency |
| `group_by` | `'day' \| 'month' \| 'quarter' \| 'year'` | Yes | Aggregation granularity |

**Returns:** `RevenueReport`

---

### `get_tax_summary_report`

Returns tax collected by tax type and country, suitable for tax filing.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `from_date` | `string` | Yes | ISO 8601 date |
| `to_date` | `string` | Yes | ISO 8601 date |

**Returns:** `TaxSummaryReport`

---

### `get_aging_report`

Returns outstanding invoices bucketed by age.

**Parameters:** none

**Returns:** `AgingReport`

```typescript
interface AgingReport {
  current: AgingBucket;       // not yet due
  days_1_30: AgingBucket;
  days_31_60: AgingBucket;
  days_61_90: AgingBucket;
  days_over_90: AgingBucket;
}

interface AgingBucket {
  count: number;
  total: number;    // i64 minor units
  invoices: InvoiceSummary[];
}
```

---

## Module: Settings

### `get_setting`

Returns a single application setting value.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `string` | Yes | Setting key |

**Returns:** `string | null`

**Common keys:** `language`, `theme`, `date_format`, `first_run`

---

### `set_setting`

Sets an application setting.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `key` | `string` | Yes | Setting key |
| `value` | `string` | Yes | Setting value |

**Returns:** `void`

---

### `backup_database`

Backs up the SQLite database to a user-chosen location.

**Parameters:** none

**Returns:** `string | null` — backup file path, or null if cancelled.

---

### `restore_database`

Restores the database from a backup file. Prompts user to confirm overwrite. Restarts the application.

**Parameters:** none

**Returns:** `void`

**Errors:** `VALIDATION:` if the selected file is not a valid 900Invoice database.

---

## Module: Sync (v1.0.0 — infrastructure only)

### `get_changelog_since`

Returns changelog entries since a given sequence number. For future sync implementation.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `since_id` | `number` | Yes | Last seen changelog ID |

**Returns:** `ChangelogEntry[]`

```typescript
interface ChangelogEntry {
  id: number;
  entity_type: string;
  entity_id: string;
  operation: 'INSERT' | 'UPDATE' | 'DELETE';
  data: string | null;   // JSON snapshot
  changed_at: string;
  device_id: string;
}
```
