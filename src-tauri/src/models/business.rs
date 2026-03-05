use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfile {
    pub id: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub country_code: String,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub tax_id: String,
    pub logo_path: Option<String>,
    pub default_currency: String,
    pub default_payment_terms_days: i32,
    pub bank_name: String,
    pub bank_account_number: String,
    pub bank_routing_number: String,
    pub mobile_money_number: String,
    pub mobile_money_provider: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBusinessProfile {
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub tax_id: Option<String>,
    pub logo_path: Option<String>,
    pub default_currency: Option<String>,
    pub default_payment_terms_days: Option<i32>,
    pub bank_name: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_routing_number: Option<String>,
    pub mobile_money_number: Option<String>,
    pub mobile_money_provider: Option<String>,
}
