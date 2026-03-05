use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub rate_bps: i32,
    pub country_code: Option<String>,
    pub is_default: bool,
    pub is_withholding: bool,
    pub is_inclusive: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaxRate {
    pub name: String,
    pub display_name: String,
    pub rate_bps: i32,
    pub country_code: Option<String>,
    pub is_default: Option<bool>,
    pub is_withholding: Option<bool>,
    pub is_inclusive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaxRate {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub rate_bps: Option<i32>,
    pub country_code: Option<String>,
    pub is_default: Option<bool>,
    pub is_withholding: Option<bool>,
    pub is_inclusive: Option<bool>,
    pub is_active: Option<bool>,
}

/// A tax line on a specific invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceTax {
    pub id: String,
    pub invoice_id: String,
    pub tax_rate_id: Option<String>,
    pub tax_name: String,
    pub tax_rate_bps: i32,
    pub tax_amount_minor: i64,
    pub is_withholding: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceTax {
    pub invoice_id: String,
    pub tax_rate_id: Option<String>,
    pub tax_name: String,
    pub tax_rate_bps: i32,
    pub tax_amount_minor: i64,
    pub is_withholding: bool,
}
