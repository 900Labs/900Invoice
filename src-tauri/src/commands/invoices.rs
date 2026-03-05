use crate::db;
use crate::models::invoice::{CreateInvoice, UpdateInvoice};
use crate::models::line_item::CreateLineItem;
use crate::services::invoice_numbering;
use crate::services::tax_calculator;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_invoices(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let invoices = db::queries::invoices::list_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(invoices).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_invoice(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let invoice = db::queries::invoices::get_with_details(&conn, &id).map_err(|e| e.to_string())?;
    match invoice {
        Some(i) => serde_json::to_value(i).map_err(|e| e.to_string()),
        None => Err(format!("Invoice not found: {}", id)),
    }
}

#[tauri::command]
pub fn create_invoice(
    db: State<'_, DbConn>,
    invoice: CreateInvoice,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let created = db::queries::invoices::insert(&conn, &invoice).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_invoice(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateInvoice,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    ensure_invoice_is_draft(&conn, &id)?;
    let updated = db::queries::invoices::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_invoice(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    ensure_invoice_is_draft(&conn, &id)?;
    db::queries::invoices::delete(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn finalize_invoice(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    // Verify invoice exists and is in draft
    let invoice = db::queries::invoices::get_by_id(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;

    if invoice.status != "draft" {
        return Err(format!(
            "Cannot finalize invoice with status '{}'",
            invoice.status
        ));
    }

    // Assign invoice number if not already set
    if invoice.invoice_number.is_none() {
        let number = invoice_numbering::generate_invoice_number(&conn, "default")?;
        db::queries::invoices::set_invoice_number(&conn, &id, &number)
            .map_err(|e| e.to_string())?;
    }

    // Recalculate totals from line items
    recalculate_invoice_totals(&conn, &id)?;

    db::queries::invoices::update_status(&conn, &id, "finalized", Some("finalized_at"))
        .map_err(|e| e.to_string())?;

    let updated = db::queries::invoices::get_with_details(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Invoice not found after finalize".to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn void_invoice(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_by_id(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;

    if invoice.status == "void" {
        return Err("Invoice is already voided".to_string());
    }

    db::queries::invoices::update_status(&conn, &id, "void", Some("voided_at"))
        .map_err(|e| e.to_string())?;

    let updated = db::queries::invoices::get_with_details(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Invoice not found after void".to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn duplicate_invoice(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let original = db::queries::invoices::get_with_details(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;

    // Create a new draft invoice from the original
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let new_invoice = CreateInvoice {
        client_id: original.client_id.clone(),
        currency_code: Some(original.currency_code.clone()),
        issue_date: Some(today),
        due_date: original.due_date.clone(),
        uses_inclusive_taxes: Some(original.uses_inclusive_taxes),
        notes: Some(original.notes.clone()),
        terms: Some(original.terms.clone()),
        footer: Some(original.footer.clone()),
        discount_minor: Some(original.discount_minor),
        exchange_rate_to_usd: original.exchange_rate_to_usd,
        exchange_rate_date: original.exchange_rate_date.clone(),
    };
    let created = db::queries::invoices::insert(&conn, &new_invoice).map_err(|e| e.to_string())?;

    // Copy line items
    for li in &original.line_items {
        let new_li = CreateLineItem {
            invoice_id: created.id.clone(),
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

    let with_details = db::queries::invoices::get_with_details(&conn, &created.id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Duplicated invoice not found".to_string())?;
    serde_json::to_value(with_details).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_invoices(db: State<'_, DbConn>, query: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let results = db::queries::invoices::search(&conn, &query).map_err(|e| e.to_string())?;
    serde_json::to_value(results).map_err(|e| e.to_string())
}

/// Internal helper: recompute and persist invoice totals from current line items.
pub fn recalculate_invoice_totals(conn: &Connection, invoice_id: &str) -> Result<(), String> {
    let invoice = db::queries::invoices::get_by_id(conn, invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let line_items =
        db::queries::line_items::list_for_invoice(conn, invoice_id).map_err(|e| e.to_string())?;

    // Get active tax rates referenced by line items (unique bps)
    let tax_rates = db::queries::taxes::list_all(conn).map_err(|e| e.to_string())?;

    let summary = tax_calculator::calculate_invoice_taxes_from_models(
        &line_items,
        &tax_rates,
        invoice.uses_inclusive_taxes,
    );

    db::queries::invoices::update_totals(
        conn,
        invoice_id,
        summary.subtotal_minor,
        invoice.discount_minor,
        summary.total_tax_minor,
        summary.total_minor - invoice.discount_minor,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn ensure_invoice_is_draft(conn: &Connection, invoice_id: &str) -> Result<(), String> {
    let invoice = db::queries::invoices::get_by_id(conn, invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    if invoice.status != "draft" {
        return Err(format!(
            "CONFLICT: invoice {} is not in DRAFT status (current: {})",
            invoice_id, invoice.status
        ));
    }

    Ok(())
}
