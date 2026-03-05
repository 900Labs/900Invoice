use crate::db;
use crate::models::invoice::CreateInvoice;
use crate::models::line_item::CreateLineItem;
use crate::models::recurring::{CreateRecurring, UpdateRecurring};
use chrono::Datelike;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_recurring(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let recurrings = db::queries::recurring::list_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(recurrings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_recurring(
    db: State<'_, DbConn>,
    recurring: CreateRecurring,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let created = db::queries::recurring::insert(&conn, &recurring).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_recurring(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateRecurring,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let updated = db::queries::recurring::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_recurring(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    db::queries::recurring::delete(&conn, &id).map_err(|e| e.to_string())
}

/// Generate actual invoices for all recurring schedules that are due today.
/// Returns the list of newly created invoices.
#[tauri::command]
pub fn generate_due_recurring(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let due = db::queries::recurring::get_due(&conn, &today).map_err(|e| e.to_string())?;
    let mut created_invoices = Vec::new();

    for recurring in &due {
        // Load the template invoice with its line items
        let template =
            match db::queries::invoices::get_with_details(&conn, &recurring.template_invoice_id)
                .map_err(|e| e.to_string())?
            {
                Some(t) => t,
                None => continue,
            };

        // Compute due date based on frequency
        let due_date = compute_next_date(&today, &recurring.frequency);

        // Create new invoice
        let new_invoice = CreateInvoice {
            client_id: recurring.client_id.clone(),
            currency_code: Some(template.currency_code.clone()),
            issue_date: Some(today.clone()),
            due_date: due_date.clone(),
            uses_inclusive_taxes: Some(template.uses_inclusive_taxes),
            notes: Some(template.notes.clone()),
            terms: Some(template.terms.clone()),
            footer: Some(template.footer.clone()),
            discount_minor: Some(template.discount_minor),
            exchange_rate_to_usd: template.exchange_rate_to_usd,
            exchange_rate_date: template.exchange_rate_date.clone(),
        };
        let invoice =
            db::queries::invoices::insert(&conn, &new_invoice).map_err(|e| e.to_string())?;

        // Copy line items from template
        for li in &template.line_items {
            let new_li = CreateLineItem {
                invoice_id: invoice.id.clone(),
                product_id: li.product_id.clone(),
                description: li.description.clone(),
                quantity: Some(li.quantity),
                unit_price_minor: li.unit_price_minor,
                tax_rate_bps: Some(li.tax_rate_bps),
                discount_bps: Some(li.discount_bps),
                sort_order: Some(li.sort_order),
            };
            db::queries::line_items::insert(&conn, &new_li).map_err(|e| e.to_string())?;
        }

        // Advance the recurring schedule
        let next = compute_next_date(&today, &recurring.frequency);
        db::queries::recurring::mark_generated(&conn, &recurring.id, &next)
            .map_err(|e| e.to_string())?;

        created_invoices.push(invoice);
    }

    serde_json::to_value(created_invoices).map_err(|e| e.to_string())
}

/// Advance a date string by one frequency period.
fn compute_next_date(from: &str, frequency: &str) -> String {
    use chrono::NaiveDate;
    let date = NaiveDate::parse_from_str(from, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Local::now().date_naive());

    let next = match frequency {
        "weekly" => date + chrono::Duration::weeks(1),
        "biweekly" => date + chrono::Duration::weeks(2),
        "quarterly" => {
            let m = date.month0() + 3;
            let year = date.year() + (m / 12) as i32;
            let month = (m % 12) + 1;
            NaiveDate::from_ymd_opt(year, month, date.day())
                .unwrap_or(date + chrono::Duration::days(91))
        }
        "annually" => NaiveDate::from_ymd_opt(date.year() + 1, date.month(), date.day())
            .unwrap_or(date + chrono::Duration::days(365)),
        // "monthly" and default
        _ => {
            let m = date.month0() + 1;
            let year = date.year() + (m / 12) as i32;
            let month = (m % 12) + 1;
            NaiveDate::from_ymd_opt(year, month, date.day())
                .unwrap_or(date + chrono::Duration::days(30))
        }
    };
    next.format("%Y-%m-%d").to_string()
}
