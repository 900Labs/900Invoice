use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub invoice_id: String,
    pub amount_minor: i64,
    pub currency_code: String,
    pub payment_method: String,
    pub payment_reference: String,
    pub notes: String,
    pub paid_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayment {
    pub invoice_id: String,
    pub amount_minor: i64,
    pub currency_code: String,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub notes: Option<String>,
    pub paid_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSummary {
    pub invoice_id: String,
    pub total_invoiced_minor: i64,
    pub total_paid_minor: i64,
    pub balance_due_minor: i64,
    pub currency_code: String,
    pub is_fully_paid: bool,
}
