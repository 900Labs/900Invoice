use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringFrequency {
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Annually,
}

impl RecurringFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecurringFrequency::Weekly => "weekly",
            RecurringFrequency::Biweekly => "biweekly",
            RecurringFrequency::Monthly => "monthly",
            RecurringFrequency::Quarterly => "quarterly",
            RecurringFrequency::Annually => "annually",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "weekly" => RecurringFrequency::Weekly,
            "biweekly" => RecurringFrequency::Biweekly,
            "quarterly" => RecurringFrequency::Quarterly,
            "annually" => RecurringFrequency::Annually,
            _ => RecurringFrequency::Monthly,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringInvoice {
    pub id: String,
    pub client_id: String,
    pub template_invoice_id: String,
    pub frequency: String,
    pub next_generation_date: String,
    pub end_date: Option<String>,
    pub auto_send: bool,
    pub last_generated: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecurring {
    pub client_id: String,
    pub template_invoice_id: String,
    pub frequency: String,
    pub next_generation_date: String,
    pub end_date: Option<String>,
    pub auto_send: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecurring {
    pub frequency: Option<String>,
    pub next_generation_date: Option<String>,
    pub end_date: Option<String>,
    pub auto_send: Option<bool>,
    pub status: Option<String>,
}
