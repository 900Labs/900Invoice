use crate::db;
use crate::models::invoice::{CreateInvoice, InvoiceWithDetails, UpdateInvoice};
use crate::models::line_item::CreateLineItem;
use crate::models::tax::CreateInvoiceTax;
use crate::services::exchange_rate_snapshot;
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
    mut invoice: CreateInvoice,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    apply_create_exchange_rate_snapshot(&conn, &mut invoice)?;
    let created = db::queries::invoices::insert(&conn, &invoice).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_invoice(
    db: State<'_, DbConn>,
    id: String,
    mut update: UpdateInvoice,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    ensure_invoice_is_draft(&conn, &id)?;
    apply_update_exchange_rate_snapshot(&conn, &id, &mut update)?;
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

    conn.execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| format!("Failed to begin finalize transaction: {e}"))?;

    let result = finalize_invoice_in_transaction(&conn, &id);

    let updated = match result {
        Ok(updated) => {
            conn.execute_batch("COMMIT").map_err(|e| {
                let _ = conn.execute_batch("ROLLBACK");
                format!("Failed to commit finalize transaction: {e}")
            })?;
            updated
        }
        Err(err) => {
            let _ = conn.execute_batch("ROLLBACK");
            return Err(err);
        }
    };

    serde_json::to_value(updated).map_err(|e| e.to_string())
}

fn finalize_invoice_in_transaction(
    conn: &Connection,
    id: &str,
) -> Result<crate::models::invoice::InvoiceWithDetails, String> {
    // Verify invoice exists and is in draft
    let invoice = db::queries::invoices::get_by_id(conn, id)
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
        let number = invoice_numbering::generate_invoice_number_in_transaction(conn, "default")?;
        db::queries::invoices::set_invoice_number(conn, id, &number).map_err(|e| e.to_string())?;
    }

    // Recalculate totals from line items
    recalculate_invoice_totals(conn, id)?;
    ensure_invoice_exchange_rate_snapshot(conn, id)?;

    db::queries::invoices::update_status(conn, id, "finalized", Some("finalized_at"))
        .map_err(|e| e.to_string())?;

    db::queries::invoices::get_with_details(conn, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Invoice not found after finalize".to_string())
}

#[tauri::command]
pub fn mark_invoice_sent(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_by_id(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;

    if invoice.status == "sent" {
        let current = db::queries::invoices::get_with_details(&conn, &id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Invoice not found after sent lookup".to_string())?;
        return serde_json::to_value(current).map_err(|e| e.to_string());
    }

    if invoice.status != "finalized" {
        return Err(format!(
            "Cannot mark invoice as sent with status '{}'",
            invoice.status
        ));
    }

    db::queries::invoices::update_status(&conn, &id, "sent", Some("sent_at"))
        .map_err(|e| e.to_string())?;

    let updated = db::queries::invoices::get_with_details(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Invoice not found after mark sent".to_string())?;
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
    let with_details = duplicate_invoice_record(&conn, &id)?;
    serde_json::to_value(with_details).map_err(|e| e.to_string())
}

fn duplicate_invoice_record(conn: &Connection, id: &str) -> Result<InvoiceWithDetails, String> {
    let original = db::queries::invoices::get_with_details(conn, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;

    // Create a new draft invoice from the original
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut new_invoice = CreateInvoice {
        client_id: original.client_id.clone(),
        currency_code: Some(original.currency_code.clone()),
        issue_date: Some(today),
        due_date: original.due_date.clone(),
        uses_inclusive_taxes: Some(original.uses_inclusive_taxes),
        notes: Some(original.notes.clone()),
        terms: Some(original.terms.clone()),
        footer: Some(original.footer.clone()),
        discount_minor: Some(original.discount_minor),
        exchange_rate_to_usd: None,
        exchange_rate_date: None,
    };
    apply_create_exchange_rate_snapshot(conn, &mut new_invoice)?;
    let created = db::queries::invoices::insert(conn, &new_invoice).map_err(|e| e.to_string())?;

    // Copy line items
    for li in &original.line_items {
        let new_li = CreateLineItem {
            invoice_id: created.id.clone(),
            product_id: li.product_id.clone(),
            tax_rate_id: li.tax_rate_id.clone(),
            description: li.description.clone(),
            quantity: Some(li.quantity),
            unit_price_minor: li.unit_price_minor,
            tax_rate_bps: Some(li.tax_rate_bps),
            discount_bps: Some(li.discount_bps),
            sort_order: Some(li.sort_order),
        };
        db::queries::line_items::insert(conn, &new_li).map_err(|e| e.to_string())?;
    }

    recalculate_invoice_totals(conn, &created.id)?;

    db::queries::invoices::get_with_details(conn, &created.id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Duplicated invoice not found".to_string())
}

#[tauri::command]
pub fn search_invoices(db: State<'_, DbConn>, query: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let results = db::queries::invoices::search(&conn, &query).map_err(|e| e.to_string())?;
    serde_json::to_value(results).map_err(|e| e.to_string())
}

fn apply_create_exchange_rate_snapshot(
    conn: &Connection,
    invoice: &mut CreateInvoice,
) -> Result<(), String> {
    if invoice.exchange_rate_to_usd.is_some() && invoice.exchange_rate_date.is_some() {
        return Ok(());
    }

    let currency = invoice.currency_code.as_deref().unwrap_or("USD");
    let issue_date = invoice
        .issue_date
        .clone()
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let snapshot = exchange_rate_snapshot::snapshot_to_usd(conn, currency, &issue_date)?;

    if invoice.exchange_rate_to_usd.is_none() {
        invoice.exchange_rate_to_usd = snapshot.rate_to_usd;
    }
    if invoice.exchange_rate_date.is_none() {
        invoice.exchange_rate_date = snapshot.valid_date;
    }
    Ok(())
}

fn apply_update_exchange_rate_snapshot(
    conn: &Connection,
    id: &str,
    update: &mut UpdateInvoice,
) -> Result<(), String> {
    if update.exchange_rate_to_usd.is_some() && update.exchange_rate_date.is_some() {
        return Ok(());
    }
    if update.currency_code.is_none() && update.issue_date.is_none() {
        return Ok(());
    }

    let existing = db::queries::invoices::get_by_id(conn, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;
    let currency = update
        .currency_code
        .as_deref()
        .unwrap_or(existing.currency_code.as_str());
    let issue_date = update
        .issue_date
        .as_deref()
        .unwrap_or(existing.issue_date.as_str());
    let snapshot = exchange_rate_snapshot::snapshot_to_usd(conn, currency, issue_date)?;

    if update.exchange_rate_to_usd.is_none() {
        update.exchange_rate_to_usd = snapshot.rate_to_usd;
    }
    if update.exchange_rate_date.is_none() {
        update.exchange_rate_date = snapshot.valid_date;
    }
    Ok(())
}

fn ensure_invoice_exchange_rate_snapshot(conn: &Connection, id: &str) -> Result<(), String> {
    let invoice = db::queries::invoices::get_by_id(conn, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", id))?;

    if invoice.exchange_rate_to_usd.is_some() && invoice.exchange_rate_date.is_some() {
        return Ok(());
    }

    let snapshot =
        exchange_rate_snapshot::snapshot_to_usd(conn, &invoice.currency_code, &invoice.issue_date)?;
    let update = UpdateInvoice {
        client_id: None,
        currency_code: None,
        issue_date: None,
        due_date: None,
        uses_inclusive_taxes: None,
        notes: None,
        terms: None,
        footer: None,
        discount_minor: None,
        exchange_rate_to_usd: if invoice.exchange_rate_to_usd.is_none() {
            snapshot.rate_to_usd
        } else {
            None
        },
        exchange_rate_date: if invoice.exchange_rate_date.is_none() {
            snapshot.valid_date
        } else {
            None
        },
    };

    if update.exchange_rate_to_usd.is_some() || update.exchange_rate_date.is_some() {
        db::queries::invoices::update(conn, id, &update).map_err(|e| e.to_string())?;
    }
    Ok(())
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

    db::queries::taxes::delete_for_invoice(conn, invoice_id).map_err(|e| e.to_string())?;
    for line in &summary.tax_lines {
        let create_tax = CreateInvoiceTax {
            invoice_id: invoice_id.to_string(),
            tax_rate_id: line.tax_rate_id.clone(),
            tax_name: line.tax_name.clone(),
            tax_rate_bps: line.tax_rate_bps,
            tax_amount_minor: line.tax_amount_minor,
            is_withholding: line.is_withholding,
        };
        db::queries::taxes::insert_invoice_tax(conn, &create_tax).map_err(|e| e.to_string())?;
    }

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

#[cfg(test)]
mod tests {
    use super::duplicate_invoice_record;
    use crate::db::migrations;
    use rusqlite::Connection;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        migrations::run_migrations(&conn).expect("migrations");
        conn
    }

    #[test]
    fn duplicate_invoice_recalculates_totals_and_tax_rows() {
        let conn = test_conn();

        conn.execute(
            "INSERT INTO clients (id, name, currency_code, payment_terms_days)
             VALUES ('client-1', 'Acme Limited', 'USD', 14)",
            [],
        )
        .expect("client");
        conn.execute(
            "INSERT INTO invoices
                (id, invoice_number, client_id, status, currency_code, subtotal_minor,
                 discount_minor, tax_amount_minor, total_minor, amount_paid_minor,
                 exchange_rate_to_usd, exchange_rate_date, issue_date, due_date,
                 uses_inclusive_taxes, notes, terms, footer)
             VALUES
                ('invoice-1', 'INV-2026-0001', 'client-1', 'finalized', 'USD',
                 10000, 0, 1600, 11600, 0, 1.0, '2026-05-11',
                 '2026-05-11', '2026-05-25', 0, 'Source notes', 'Net 14', '')",
            [],
        )
        .expect("invoice");
        conn.execute(
            "INSERT INTO invoice_line_items
                (id, invoice_id, product_id, tax_rate_id, description, quantity,
                 unit_price_minor, tax_rate_bps, discount_bps, line_total_minor,
                 sort_order)
             VALUES
                ('line-1', 'invoice-1', NULL, 'tax-ke-vat', 'Consulting',
                 100, 10000, 1600, 0, 10000, 0)",
            [],
        )
        .expect("line item");

        let duplicated = duplicate_invoice_record(&conn, "invoice-1").expect("duplicated invoice");

        assert_ne!(duplicated.id, "invoice-1");
        assert_eq!(duplicated.invoice_number, None);
        assert_eq!(duplicated.status, "draft");
        assert_eq!(duplicated.client_id, "client-1");
        assert_eq!(duplicated.subtotal_minor, 10000);
        assert_eq!(duplicated.discount_minor, 0);
        assert_eq!(duplicated.tax_amount_minor, 1600);
        assert_eq!(duplicated.total_minor, 11600);
        assert_eq!(duplicated.line_items.len(), 1);
        assert_eq!(
            duplicated.line_items[0].tax_rate_id.as_deref(),
            Some("tax-ke-vat")
        );
        assert_eq!(duplicated.taxes.len(), 1);
        assert_eq!(
            duplicated.taxes[0].tax_rate_id.as_deref(),
            Some("tax-ke-vat")
        );
        assert_eq!(duplicated.taxes[0].tax_amount_minor, 1600);
    }
}
