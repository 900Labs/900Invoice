use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub id: String,
    pub invoice_id: String,
    pub product_id: Option<String>,
    pub description: String,
    /// Quantity stored as integer * 100 (e.g. 100 = 1.00 unit, 150 = 1.50 units)
    pub quantity: i64,
    pub unit_price_minor: i64,
    pub tax_rate_bps: i32,
    pub discount_bps: i32,
    pub line_total_minor: i64,
    pub sort_order: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLineItem {
    pub invoice_id: String,
    pub product_id: Option<String>,
    pub description: String,
    pub quantity: Option<i64>,
    pub unit_price_minor: i64,
    pub tax_rate_bps: Option<i32>,
    pub discount_bps: Option<i32>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLineItem {
    pub product_id: Option<String>,
    pub description: Option<String>,
    pub quantity: Option<i64>,
    pub unit_price_minor: Option<i64>,
    pub tax_rate_bps: Option<i32>,
    pub discount_bps: Option<i32>,
    pub sort_order: Option<i32>,
}

/// Helper: compute the line total from quantity (×100), price, discount_bps
/// Returns minor units.
pub fn compute_line_total(quantity_x100: i64, unit_price_minor: i64, discount_bps: i32) -> i64 {
    // quantity is stored ×100, so real qty = quantity_x100 / 100
    let gross = quantity_x100 * unit_price_minor / 100;
    let discount = gross * discount_bps as i64 / 10_000;
    gross - discount
}
