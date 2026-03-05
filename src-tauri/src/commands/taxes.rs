use crate::db;
use crate::models::tax::{CreateInvoiceTax, CreateTaxRate, UpdateTaxRate};
use crate::services::tax_calculator;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_tax_rates(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let rates = db::queries::taxes::list_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(rates).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_tax_rate(
    db: State<'_, DbConn>,
    tax_rate: CreateTaxRate,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let created = db::queries::taxes::insert(&conn, &tax_rate).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_tax_rate(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateTaxRate,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let updated = db::queries::taxes::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_tax_rate(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    db::queries::taxes::delete(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tax_rates_for_country(
    db: State<'_, DbConn>,
    country_code: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let rates =
        db::queries::taxes::list_for_country(&conn, &country_code).map_err(|e| e.to_string())?;
    serde_json::to_value(rates).map_err(|e| e.to_string())
}

/// Calculate taxes for a given invoice and persist the invoice_taxes rows.
/// Replaces any existing invoice_taxes entries for the invoice.
#[tauri::command]
pub fn calculate_invoice_taxes(
    db: State<'_, DbConn>,
    invoice_id: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_by_id(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let line_items =
        db::queries::line_items::list_for_invoice(&conn, &invoice_id).map_err(|e| e.to_string())?;
    let tax_rates = db::queries::taxes::list_all(&conn).map_err(|e| e.to_string())?;

    let summary = tax_calculator::calculate_invoice_taxes_from_models(
        &line_items,
        &tax_rates,
        invoice.uses_inclusive_taxes,
    );

    // Delete existing invoice taxes and re-insert
    db::queries::taxes::delete_for_invoice(&conn, &invoice_id).map_err(|e| e.to_string())?;

    let mut result_taxes = Vec::new();
    for line in &summary.tax_lines {
        let create_tax = CreateInvoiceTax {
            invoice_id: invoice_id.clone(),
            tax_rate_id: line.tax_rate_id.clone(),
            tax_name: line.tax_name.clone(),
            tax_rate_bps: line.tax_rate_bps,
            tax_amount_minor: line.tax_amount_minor,
            is_withholding: line.is_withholding,
        };
        let saved = db::queries::taxes::insert_invoice_tax(&conn, &create_tax)
            .map_err(|e| e.to_string())?;
        result_taxes.push(saved);
    }

    // Update invoice totals
    db::queries::invoices::update_totals(
        &conn,
        &invoice_id,
        summary.subtotal_minor,
        invoice.discount_minor,
        summary.total_tax_minor,
        summary.total_minor - invoice.discount_minor,
    )
    .map_err(|e| e.to_string())?;

    serde_json::to_value(result_taxes).map_err(|e| e.to_string())
}
