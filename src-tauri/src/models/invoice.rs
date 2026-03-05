#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::models::client::Client;
use crate::models::line_item::LineItem;
use crate::models::payment::Payment;
use crate::models::tax::InvoiceTax;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Finalized,
    Sent,
    Paid,
    Void,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Finalized => "finalized",
            InvoiceStatus::Sent => "sent",
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Void => "void",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "finalized" => InvoiceStatus::Finalized,
            "sent" => InvoiceStatus::Sent,
            "paid" => InvoiceStatus::Paid,
            "void" => InvoiceStatus::Void,
            _ => InvoiceStatus::Draft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: Option<String>,
    pub client_id: String,
    pub status: String,
    pub currency_code: String,
    pub subtotal_minor: i64,
    pub discount_minor: i64,
    pub tax_amount_minor: i64,
    pub total_minor: i64,
    pub amount_paid_minor: i64,
    pub exchange_rate_to_usd: Option<f64>,
    pub exchange_rate_date: Option<String>,
    pub issue_date: String,
    pub due_date: String,
    pub uses_inclusive_taxes: bool,
    pub notes: String,
    pub terms: String,
    pub footer: String,
    pub created_at: String,
    pub updated_at: String,
    pub finalized_at: Option<String>,
    pub sent_at: Option<String>,
    pub paid_at: Option<String>,
    pub voided_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceWithDetails {
    pub id: String,
    pub invoice_number: Option<String>,
    pub client_id: String,
    pub client: Option<Client>,
    pub status: String,
    pub currency_code: String,
    pub subtotal_minor: i64,
    pub discount_minor: i64,
    pub tax_amount_minor: i64,
    pub total_minor: i64,
    pub amount_paid_minor: i64,
    pub exchange_rate_to_usd: Option<f64>,
    pub exchange_rate_date: Option<String>,
    pub issue_date: String,
    pub due_date: String,
    pub uses_inclusive_taxes: bool,
    pub notes: String,
    pub terms: String,
    pub footer: String,
    pub created_at: String,
    pub updated_at: String,
    pub finalized_at: Option<String>,
    pub sent_at: Option<String>,
    pub paid_at: Option<String>,
    pub voided_at: Option<String>,
    pub line_items: Vec<LineItem>,
    pub taxes: Vec<InvoiceTax>,
    pub payments: Vec<Payment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoice {
    pub client_id: String,
    pub currency_code: Option<String>,
    pub issue_date: Option<String>,
    pub due_date: String,
    pub uses_inclusive_taxes: Option<bool>,
    pub notes: Option<String>,
    pub terms: Option<String>,
    pub footer: Option<String>,
    pub discount_minor: Option<i64>,
    pub exchange_rate_to_usd: Option<f64>,
    pub exchange_rate_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoice {
    pub client_id: Option<String>,
    pub currency_code: Option<String>,
    pub issue_date: Option<String>,
    pub due_date: Option<String>,
    pub uses_inclusive_taxes: Option<bool>,
    pub notes: Option<String>,
    pub terms: Option<String>,
    pub footer: Option<String>,
    pub discount_minor: Option<i64>,
    pub exchange_rate_to_usd: Option<f64>,
    pub exchange_rate_date: Option<String>,
}
