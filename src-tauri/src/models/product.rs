use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_price_minor: i64,
    pub default_currency: String,
    pub default_tax_rate_id: Option<String>,
    pub default_tax_rate_bps: i32,
    pub unit: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub default_price_minor: Option<i64>,
    pub default_currency: Option<String>,
    pub default_tax_rate_id: Option<String>,
    pub default_tax_rate_bps: Option<i32>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_price_minor: Option<i64>,
    pub default_currency: Option<String>,
    pub default_tax_rate_id: Option<String>,
    pub default_tax_rate_bps: Option<i32>,
    pub unit: Option<String>,
    pub is_active: Option<bool>,
}
