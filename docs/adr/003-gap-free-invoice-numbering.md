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

### The Invoice Sequence Approach

A dedicated `invoice_sequences` table maintains configurable sequence settings and the next number to issue. Invoice numbers are assigned within a `BEGIN IMMEDIATE` SQLite transaction that:

1. `SELECT`s the sequence row by `sequence_name`
2. Determines the effective next number, resetting to `1` when yearly reset is enabled and the year changed
3. Formats the number from `prefix`, `separator`, optional year, and zero-padded counter
4. Advances `next_number` and `last_year`
5. Writes the formatted number to `invoices.invoice_number`
6. Recalculates totals, ensures an exchange-rate snapshot, and marks the invoice finalized

Because finalization now keeps sequence advancement and invoice finalization in the same SQLite write transaction, the counter increment and invoice update commit or roll back together. In SQLite's serialized concurrency model (single writer at a time), there is no possibility of duplicate numbers.

## Decision

Use a configurable **invoice sequence table** (`invoice_sequences`) to generate gap-free invoice numbers. Numbers are assigned at `finalize_invoice()` time, not at `create_invoice()` time.

**Default number format:** `{PREFIX}-{YEAR}-{COUNTER}`

Default: `INV-2026-0001` (configurable via Settings)

- `PREFIX`: configurable sequence prefix (default `INV`)
- `SEPARATOR`: configurable separator (default `-`)
- `YEAR`: optional 4-digit calendar year of finalization
- `COUNTER`: zero-padded by `pad_digits`, resets to `1` on a new year when `year_reset` is enabled

**Schema:**
```sql
CREATE TABLE invoice_sequences (
    sequence_name TEXT PRIMARY KEY,
    prefix        TEXT NOT NULL DEFAULT 'INV',
    separator     TEXT NOT NULL DEFAULT '-',
    include_year  INTEGER NOT NULL DEFAULT 1,
    pad_digits    INTEGER NOT NULL DEFAULT 4,
    year_reset    INTEGER NOT NULL DEFAULT 1,
    last_year     INTEGER,
    last_month    INTEGER,
    next_number   INTEGER NOT NULL DEFAULT 1,
    created_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Finalization procedure (pseudocode):**
```
BEGIN IMMEDIATE;
  invoice = SELECT * FROM invoices WHERE id = ?;
  IF invoice.status != 'draft':
    ROLLBACK;

  IF invoice.invoice_number IS NULL:
    sequence = SELECT * FROM invoice_sequences WHERE sequence_name = 'default';
    next_value = sequence.next_number;
    IF sequence.year_reset AND sequence.last_year != current_year:
      next_value = 1;

    formatted_number = format(prefix, separator, include_year, current_year, next_value, pad_digits);
    UPDATE invoice_sequences
      SET next_number = next_value + 1, last_year = current_year
      WHERE sequence_name = 'default';
    UPDATE invoices SET invoice_number = formatted_number WHERE id = ?;

  recalculate invoice totals and tax rows;
  ensure exchange-rate snapshot;
  UPDATE invoices SET status = 'finalized', finalized_at = datetime('now') WHERE id = ?;
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

The sequence logic lives in `src-tauri/src/services/invoice_numbering.rs`. Tests must verify:
- Concurrent finalization (even though SQLite serializes writes, tests should confirm atomicity)
- Year rollover (Dec 31 → Jan 1)
- Custom prefix configuration
- The draft-has-no-number behavior in the UI
