/// Tax calculator for 900Invoice.
///
/// All monetary values are i64 minor units (e.g. 10000 = 100.00 USD/KES/etc.).
/// Tax rates are i32 basis points (1600 = 16.00%).
/// Rounding: half-up (0.5 always rounds toward positive infinity).

use crate::models::line_item::LineItem;
use crate::models::tax::TaxRate;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Rounding
// ---------------------------------------------------------------------------

/// Integer division with half-up rounding.
/// For positive values: (numerator + denominator/2) / denominator.
#[inline]
fn div_round_half_up(numerator: i64, denominator: i64) -> i64 {
    debug_assert!(denominator > 0, "denominator must be positive");
    if numerator >= 0 {
        (numerator + denominator / 2) / denominator
    } else {
        // For negative: round toward zero for the absolute, then negate
        let abs = (-numerator + denominator / 2) / denominator;
        -abs
    }
}

// ---------------------------------------------------------------------------
// Core single-line functions
// ---------------------------------------------------------------------------

/// Calculate tax on a single amount.
///
/// Exclusive: `tax = amount_minor * rate_bps / 10000`
/// Inclusive: `tax = amount_minor * rate_bps / (10000 + rate_bps)`
///
/// Both rounded half-up.
pub fn calculate_line_tax(amount_minor: i64, rate_bps: i32, is_inclusive: bool) -> i64 {
    if rate_bps == 0 {
        return 0;
    }
    let r = rate_bps as i64;
    if is_inclusive {
        div_round_half_up(amount_minor * r, 10_000 + r)
    } else {
        div_round_half_up(amount_minor * r, 10_000)
    }
}

/// Calculate line total from quantity, unit price, and discount.
///
/// - `quantity` is stored as INTEGER × 100 (e.g. 150 = 1.50 units)
/// - `unit_price_minor` is price per 1.00 unit in minor currency units
/// - `discount_bps` is discount in basis points (500 = 5.00%)
///
/// Formula: `line_total = round(quantity * unit_price / 100)`
/// then: `discounted = line_total - round(line_total * discount_bps / 10000)`
pub fn calculate_line_total(quantity: i32, unit_price_minor: i64, discount_bps: i32) -> i64 {
    if quantity == 0 {
        return 0;
    }
    let gross = div_round_half_up(quantity as i64 * unit_price_minor, 100);
    if discount_bps == 0 {
        return gross;
    }
    let discount = div_round_half_up(gross * discount_bps as i64, 10_000);
    gross - discount
}

// ---------------------------------------------------------------------------
// Input / output types for invoice-level calculation
// ---------------------------------------------------------------------------

/// Input for a single line's tax contribution (generic, not tied to DB model).
#[derive(Debug, Clone)]
pub struct LineTaxInput {
    /// Pre-calculated line total in minor units.
    pub line_total_minor: i64,
    /// Tax rate in basis points for this line.
    pub tax_rate_bps: i32,
    /// Display name for this tax (e.g. "VAT", "NHIL", "WHT").
    pub tax_name: String,
    /// Whether this tax is withholding (deducted from total).
    pub is_withholding: bool,
}

/// A single tax line in the summary.
#[derive(Debug, Clone)]
pub struct TaxSummaryLine {
    pub tax_name: String,
    pub tax_rate_bps: i32,
    pub tax_amount_minor: i64,
    pub is_withholding: bool,
}

/// Complete invoice tax summary (generic input path).
#[derive(Debug, Clone)]
pub struct InvoiceTaxSummary {
    pub subtotal_minor: i64,
    pub discount_minor: i64,
    pub tax_lines: Vec<TaxSummaryLine>,
    pub total_tax_minor: i64,
    pub withholding_minor: i64,
    pub total_minor: i64,
}

/// Calculate full invoice taxes from a slice of `LineTaxInput`.
///
/// Taxes are grouped by `(tax_name, tax_rate_bps, is_withholding)`.
/// Withholding taxes are deducted from the final total.
/// The invoice-level `discount_minor` is applied proportionally across lines.
pub fn calculate_invoice_taxes(
    lines: &[LineTaxInput],
    discount_minor: i64,
    is_inclusive: bool,
) -> InvoiceTaxSummary {
    let subtotal_minor: i64 = lines.iter().map(|l| l.line_total_minor).sum();

    // Build map: (tax_name, rate_bps, is_withholding) → accumulated tax amount
    let mut tax_map: HashMap<(String, i32, bool), i64> = HashMap::new();

    for line in lines {
        // Apply proportional discount to this line's taxable base
        let line_base = if subtotal_minor > 0 && discount_minor > 0 {
            let discount_on_line =
                div_round_half_up(line.line_total_minor * discount_minor, subtotal_minor);
            line.line_total_minor - discount_on_line
        } else {
            line.line_total_minor
        };

        let tax_amount = calculate_line_tax(line_base, line.tax_rate_bps, is_inclusive);
        let key = (line.tax_name.clone(), line.tax_rate_bps, line.is_withholding);
        *tax_map.entry(key).or_insert(0) += tax_amount;
    }

    let mut tax_lines: Vec<TaxSummaryLine> = tax_map
        .into_iter()
        .map(|((name, rate, wh), amount)| TaxSummaryLine {
            tax_name: name,
            tax_rate_bps: rate,
            tax_amount_minor: amount,
            is_withholding: wh,
        })
        .collect();

    // Sort: non-withholding first, then by rate desc, then by name
    tax_lines.sort_by(|a, b| {
        a.is_withholding
            .cmp(&b.is_withholding)
            .then(b.tax_rate_bps.cmp(&a.tax_rate_bps))
            .then(a.tax_name.cmp(&b.tax_name))
    });

    let total_tax_minor: i64 = tax_lines
        .iter()
        .filter(|t| !t.is_withholding)
        .map(|t| t.tax_amount_minor)
        .sum();

    let withholding_minor: i64 = tax_lines
        .iter()
        .filter(|t| t.is_withholding)
        .map(|t| t.tax_amount_minor)
        .sum();

    let taxable_base = subtotal_minor.saturating_sub(discount_minor).max(0);
    let total_minor = taxable_base + total_tax_minor - withholding_minor;

    InvoiceTaxSummary {
        subtotal_minor,
        discount_minor,
        tax_lines,
        total_tax_minor,
        withholding_minor,
        total_minor,
    }
}

// ---------------------------------------------------------------------------
// DB-model-based invoice tax calculation (used by commands layer)
// ---------------------------------------------------------------------------

/// A tax line entry derived from DB model TaxRate (used internally).
#[derive(Debug, Clone)]
pub struct TaxLineEntry {
    pub tax_rate_id: Option<String>,
    pub tax_name: String,
    pub tax_rate_bps: i32,
    pub tax_amount_minor: i64,
    pub is_withholding: bool,
}

/// Invoice tax summary from DB models (used by commands layer).
pub struct DbInvoiceTaxSummary {
    pub subtotal_minor: i64,
    pub tax_lines: Vec<TaxLineEntry>,
    pub total_tax_minor: i64,
    /// total = subtotal + non-withholding_tax - withholding_tax
    pub total_minor: i64,
}

/// Compute full invoice tax breakdown using DB model types.
///
/// `line_items` — invoice line items (line_total_minor pre-calculated)
/// `tax_rates`  — active tax rates to apply
/// `is_inclusive` — whether prices already include tax
pub fn calculate_invoice_taxes_from_models(
    line_items: &[LineItem],
    tax_rates: &[TaxRate],
    is_inclusive: bool,
) -> DbInvoiceTaxSummary {
    let subtotal_minor: i64 = line_items.iter().map(|li| li.line_total_minor).sum();

    let mut tax_lines: Vec<TaxLineEntry> = Vec::new();
    let mut total_tax_minor: i64 = 0;

    for rate in tax_rates {
        if !rate.is_active {
            continue;
        }
        let tax_amount = calculate_line_tax(subtotal_minor, rate.rate_bps, is_inclusive);
        total_tax_minor += tax_amount;
        tax_lines.push(TaxLineEntry {
            tax_rate_id: Some(rate.id.clone()),
            tax_name: rate.display_name.clone(),
            tax_rate_bps: rate.rate_bps,
            tax_amount_minor: tax_amount,
            is_withholding: rate.is_withholding,
        });
    }

    let non_wht: i64 = tax_lines
        .iter()
        .filter(|t| !t.is_withholding)
        .map(|t| t.tax_amount_minor)
        .sum();
    let wht: i64 = tax_lines
        .iter()
        .filter(|t| t.is_withholding)
        .map(|t| t.tax_amount_minor)
        .sum();

    let total_minor = if is_inclusive {
        subtotal_minor - wht
    } else {
        subtotal_minor + non_wht - wht
    };

    DbInvoiceTaxSummary {
        subtotal_minor,
        tax_lines,
        total_tax_minor,
        total_minor,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- calculate_line_tax ---

    #[test]
    fn test_exclusive_16_percent() {
        // 10000 minor * 1600 bps = 1600 minor tax
        assert_eq!(calculate_line_tax(10_000, 1600, false), 1600);
    }

    #[test]
    fn test_exclusive_7_5_percent() {
        // 10000 * 750 / 10000 = 750
        assert_eq!(calculate_line_tax(10_000, 750, false), 750);
    }

    #[test]
    fn test_inclusive_16_percent() {
        // 11600 inclusive @ 16%: tax = 11600 * 1600 / 11600 = 1600
        assert_eq!(calculate_line_tax(11_600, 1600, true), 1600);
    }

    #[test]
    fn test_zero_rate() {
        assert_eq!(calculate_line_tax(50_000, 0, false), 0);
        assert_eq!(calculate_line_tax(50_000, 0, true), 0);
    }

    #[test]
    fn test_withholding_5_percent() {
        assert_eq!(calculate_line_tax(10_000, 500, false), 500);
    }

    #[test]
    fn test_rounding_half_up() {
        // 3 * 5000 / 10000 = 1.5 → rounds to 2
        assert_eq!(calculate_line_tax(3, 5000, false), 2);
    }

    // --- calculate_line_total ---

    #[test]
    fn test_line_total_basic() {
        // qty=100 (1 unit), price=10000, no discount → 10000
        assert_eq!(calculate_line_total(100, 10_000, 0), 10_000);
    }

    #[test]
    fn test_line_total_fractional_qty() {
        // qty=150 (1.5 units), price=10000 → 15000
        assert_eq!(calculate_line_total(150, 10_000, 0), 15_000);
    }

    #[test]
    fn test_line_total_with_discount() {
        // qty=100, price=10000, 10% disc → 10000 - 1000 = 9000
        assert_eq!(calculate_line_total(100, 10_000, 1000), 9_000);
    }

    #[test]
    fn test_line_total_zero_qty() {
        assert_eq!(calculate_line_total(0, 10_000, 0), 0);
    }

    // --- calculate_invoice_taxes ---

    #[test]
    fn test_invoice_single_tax() {
        let lines = vec![LineTaxInput {
            line_total_minor: 10_000,
            tax_rate_bps: 1600,
            tax_name: "VAT".into(),
            is_withholding: false,
        }];
        let s = calculate_invoice_taxes(&lines, 0, false);
        assert_eq!(s.subtotal_minor, 10_000);
        assert_eq!(s.total_tax_minor, 1_600);
        assert_eq!(s.withholding_minor, 0);
        assert_eq!(s.total_minor, 11_600);
    }

    #[test]
    fn test_invoice_with_discount() {
        let lines = vec![LineTaxInput {
            line_total_minor: 10_000,
            tax_rate_bps: 1600,
            tax_name: "VAT".into(),
            is_withholding: false,
        }];
        // 1000 discount → taxable = 9000, VAT = 1440
        let s = calculate_invoice_taxes(&lines, 1_000, false);
        assert_eq!(s.subtotal_minor, 10_000);
        assert_eq!(s.discount_minor, 1_000);
        assert_eq!(s.total_tax_minor, 1_440);
        assert_eq!(s.total_minor, 9_000 + 1_440);
    }

    #[test]
    fn test_ghana_triple_tax() {
        // Ghana: same goods, three tax lines
        let lines = vec![
            LineTaxInput { line_total_minor: 10_000, tax_rate_bps: 1500, tax_name: "VAT".into(), is_withholding: false },
            LineTaxInput { line_total_minor: 10_000, tax_rate_bps: 250, tax_name: "NHIL".into(), is_withholding: false },
            LineTaxInput { line_total_minor: 10_000, tax_rate_bps: 250, tax_name: "GETFund".into(), is_withholding: false },
        ];
        let s = calculate_invoice_taxes(&lines, 0, false);
        let vat = s.tax_lines.iter().find(|t| t.tax_name == "VAT").unwrap();
        assert_eq!(vat.tax_amount_minor, 1_500);
        let nhil = s.tax_lines.iter().find(|t| t.tax_name == "NHIL").unwrap();
        assert_eq!(nhil.tax_amount_minor, 250);
    }

    #[test]
    fn test_withholding_deduction() {
        let lines = vec![
            LineTaxInput { line_total_minor: 10_000, tax_rate_bps: 1600, tax_name: "VAT".into(), is_withholding: false },
            LineTaxInput { line_total_minor: 10_000, tax_rate_bps: 500, tax_name: "WHT".into(), is_withholding: true },
        ];
        let s = calculate_invoice_taxes(&lines, 0, false);
        assert_eq!(s.total_tax_minor, 1_600);
        assert_eq!(s.withholding_minor, 500);
        // total = 20000 (subtotal both lines) + 1600 - 500 = 21100
        assert_eq!(s.total_minor, 20_000 + 1_600 - 500);
    }

    #[test]
    fn test_zero_rate_line() {
        let lines = vec![LineTaxInput {
            line_total_minor: 50_000,
            tax_rate_bps: 0,
            tax_name: "No Tax".into(),
            is_withholding: false,
        }];
        let s = calculate_invoice_taxes(&lines, 0, false);
        assert_eq!(s.total_tax_minor, 0);
        assert_eq!(s.total_minor, 50_000);
    }
}
