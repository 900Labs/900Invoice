use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub base_currency: String,
    pub target_currency: String,
    pub rate: f64,
    pub fetched_at: String,
    pub valid_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount_minor: i64,
    pub to_amount_minor: i64,
    pub rate: f64,
    pub valid_date: String,
}
