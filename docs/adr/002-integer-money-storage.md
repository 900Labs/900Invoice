# ADR 002: Integer Minor Units for Money Storage

## Status: Accepted

## Date: 2026-03-01

## Context

Financial software must handle monetary arithmetic with exact precision. 900Invoice calculates invoice totals, tax amounts, and payment balances across 11 currencies. These calculations must be correct to the last cent — errors in tax calculations can result in incorrect tax filings with government authorities.

The naive approach is to store money as a floating-point number (JavaScript `number`, Rust `f64`). This appears to work in simple cases but fails in practice.

### The Floating-Point Problem

IEEE 754 double-precision floating-point cannot exactly represent most decimal fractions. The value `0.1` in binary floating-point is actually `0.1000000000000000055511151231257827021181583404541015625...`. This causes arithmetic errors:

```
// JavaScript
0.1 + 0.2 = 0.30000000000000004   // NOT 0.3
// If this were a tax calculation:
// Expected tax on KES 0.30: KES 0.048
// Calculated tax: KES 0.04800000000000000...640
```

In a simple 3-line invoice this is negligible. Across 10,000 invoices with multiple line items and compound tax rates, these errors accumulate and produce tax reports that do not match the sum of individual invoices.

### The Minor Unit Solution

Store all monetary amounts as integers in the smallest denomination of the currency:

| Currency | Minor Unit | Example |
|----------|-----------|---------|
| KES | Cent (1/100 shilling) | KES 1,500.00 = 150,000 |
| NGN | Kobo (1/100 naira) | NGN 25,000.00 = 2,500,000 |
| INR | Paise (1/100 rupee) | INR 10,000.00 = 1,000,000 |
| ZAR | Cent (1/100 rand) | ZAR 500.00 = 50,000 |
| GHS | Pesewa (1/100 cedi) | GHS 200.00 = 20,000 |
| UGX | No minor unit | UGX 50,000 = 50,000 |
| XOF | No minor unit | XOF 10,000 = 10,000 |
| XAF | No minor unit | XAF 10,000 = 10,000 |
| USD | Cent (1/100 dollar) | USD 99.99 = 9,999 |
| EUR | Cent (1/100 euro) | EUR 99.99 = 9,999 |

Integer arithmetic is exact. `150000 + 50000 = 200000` with no rounding errors.

### Basis Points for Tax Rates

The same principle applies to tax rates. Storing `16.00%` as the float `0.16` introduces representation error. Instead, we store rates in **basis points** (bps) where 1 bps = 0.01%:

- 16.00% → 1600 bps
- 7.5% → 750 bps
- 2.5% → 250 bps

This gives us exact two-decimal-place precision for all rates used in the countries we support.

## Decision

1. All monetary amounts are stored as `i64` in the database (SQLite `INTEGER` columns) and in Rust structs.
2. All tax rates are stored as `i32` basis points.
3. Conversion to decimal for display happens **only in the frontend**, at render time, and the result is never sent back to the backend.
4. The frontend sends amounts as integers across the Tauri IPC bridge — never as floats.

**Tax calculation (exclusive):**
```
tax_amount = (subtotal_minor_units * rate_bps + 5000) / 10000
// +5000 implements round-half-up
```

**Tax calculation (inclusive):**
```
tax_amount = subtotal - (subtotal * 10000 + (10000 + rate_bps) / 2) / (10000 + rate_bps)
```

## Consequences

### Positive
- Zero floating-point rounding errors in all financial calculations
- Tax reports always reconcile exactly with individual invoice sums
- SQLite INTEGER storage is efficient and queryable
- Consistent across all 11 supported currencies

### Negative / Trade-offs
- Frontend developers must be aware of the convention and always convert before display
- Some currencies (UGX, XOF, XAF) have no minor unit — the stored value equals the major unit value, which is unintuitive at first
- TypeScript's `number` type is sufficient (integers up to 2^53 are exact), but amounts in JavaScript must be treated carefully and never passed through floating-point operations

### Enforcement

The Rust compiler enforces this at the type level — all amount fields are `i64` and Rust will not silently convert them to floats. For the frontend, the `parseAmount()` utility in `src/utils/currency.ts` is the only approved way to convert user-entered strings to integers for sending to the backend.

### Notes for Contributors

Never do this in Rust:
```rust
// WRONG
let tax = amount as f64 * rate_bps as f64 / 10000.0;

// CORRECT
let tax = (amount * rate_bps as i64 + 5000) / 10000;
```

Never do this in TypeScript:
```typescript
// WRONG — sends a float to Rust
const amount = parseFloat(input) * 100;

// CORRECT — uses the utility function
const amount = parseAmount(input, currencyCode); // returns integer
```
