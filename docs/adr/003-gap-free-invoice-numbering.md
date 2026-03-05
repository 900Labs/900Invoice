# ADR 003: Gap-Free Sequential Invoice Numbering

## Status: Accepted

## Date: 2026-03-01

## Context

Invoice numbering is not merely a convenience feature — it is a fiscal and legal requirement in most jurisdictions. Tax authorities in Kenya, Nigeria, South Africa, India, and most other countries that 900Invoice targets require:

1. **Sequential numbering**: Invoices must be numbered in order with no gaps.
2. **No missing numbers**: A missing invoice number (e.g., INV-001, INV-002, INV-004 — where is INV-003?) is a red flag during a tax audit. It suggests that an invoice was voided and destroyed to hide income.
3. **No reuse**: An invoice number, once assigned, can never be reused, even if the invoice is voided.

This creates specific technical requirements:

- Invoice numbers must be assigned atomically (no two invoices can receive the same number even if created simultaneously — though 900Invoice is single-user, this still applies during testing)
- Numbers cannot be assigned at draft creation time (a draft that is never finalized would create a gap)
- Numbers must be assigned at finalization time, under a database transaction
- Voided invoices retain their numbers permanently

### Why UUIDs Are Not Sufficient for Invoice Numbers

900Invoice uses UUID v4 as the primary key for all records (for offline-safe creation and future sync compatibility). UUIDs are not suitable as invoice numbers for several reasons:

1. Not human-readable: `a7f3d9c1-4e2b-4f8a-b1d6-8c3e7f2a1b9d` is not what an accountant wants to see on an invoice
2. Not sequential: No audit trail of order
3. Not auditable: Cannot easily determine if there are gaps

### The Sequence Counter Approach

A dedicated `sequence_counters` table maintains the last-issued sequence number. Invoice numbers are assigned within a database transaction that:

1. `SELECT counter FROM sequence_counters WHERE prefix = ?` (with `FOR UPDATE` semantics via SQLite's exclusive transaction)
2. `UPDATE sequence_counters SET counter = counter + 1 WHERE prefix = ?`
3. Formats the number: `{prefix}-{year}-{zero_padded_counter}`
4. Inserts the invoice with the formatted number

Because this happens inside a SQLite EXCLUSIVE transaction, the counter increment and invoice insert are atomic. In SQLite's serialized concurrency model (single writer at a time), there is no possibility of duplicate or skipped numbers.

## Decision

Use a **sequence counter table** (`sequence_counters`) to generate gap-free invoice numbers. Numbers are assigned at `finalize_invoice()` time, not at `create_invoice()` time.

**Number format:** `{PREFIX}-{YEAR}-{COUNTER}`

Default: `INV-2026-0001` (configurable via Settings)

- `PREFIX`: 1–5 uppercase letters, configurable per business (default `INV`)
- `YEAR`: 4-digit calendar year of finalization
- `COUNTER`: Zero-padded to 4 digits within the year, resets to 0001 on new year

**Schema:**
```sql
CREATE TABLE sequence_counters (
    prefix      TEXT NOT NULL,
    year        INTEGER NOT NULL,
    last_value  INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (prefix, year)
);
```

**Finalization procedure (pseudocode):**
```
BEGIN EXCLUSIVE TRANSACTION;
  SELECT last_value FROM sequence_counters WHERE prefix = ? AND year = ?;
  IF NOT FOUND:
    INSERT INTO sequence_counters (prefix, year, last_value) VALUES (?, ?, 0);
    next_value = 1;
  ELSE:
    next_value = last_value + 1;
    UPDATE sequence_counters SET last_value = next_value WHERE prefix = ? AND year = ?;
  
  formatted_number = format!("{}-{}-{:04}", prefix, year, next_value);
  UPDATE invoices SET number = formatted_number, status = 'finalized' WHERE id = ?;
COMMIT;
```

## Consequences

### Positive
- Gap-free sequential numbering satisfies tax authority requirements in all target countries
- Numbers are assigned atomically — no duplicates or gaps possible
- Human-readable and auditable format
- Year-based reset keeps numbers short even after years of operation
- Voided invoice numbers are preserved (the invoice record remains, status = void)

### Negative / Trade-offs
- Draft invoices do not have a number (they show "DRAFT" until finalized) — this is a UX tradeoff but is the correct behavior
- If a user deletes the entire database and starts over, numbering restarts from 0001 for that year — this is acceptable and expected for fresh installs
- The EXCLUSIVE transaction is slightly slower than a non-exclusive insert, but the operation is infrequent (once per invoice finalization) and imperceptible to users

### Notes for Contributors

The sequence logic lives in `src-tauri/src/services/numbering.rs`. Tests must verify:
- Concurrent finalization (even though SQLite serializes writes, tests should confirm atomicity)
- Year rollover (Dec 31 → Jan 1)
- Custom prefix configuration
- The draft-has-no-number behavior in the UI
