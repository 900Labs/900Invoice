use crate::db;
use crate::models::payment::{CreatePayment, PaymentSummary};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_payments(
    db: State<'_, DbConn>,
    invoice_id: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let payments =
        db::queries::payments::list_for_invoice(&conn, &invoice_id).map_err(|e| e.to_string())?;
    serde_json::to_value(payments).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn record_payment(
    db: State<'_, DbConn>,
    payment: CreatePayment,
) -> Result<serde_json::Value, String> {
    if payment.amount_minor <= 0 {
        return Err("VALIDATION: payment amount must be greater than zero".to_string());
    }

    let conn = db.lock().map_err(|e| e.to_string())?;
    let invoice_id = payment.invoice_id.clone();

    let invoice = db::queries::invoices::get_by_id(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    if matches!(invoice.status.as_str(), "draft" | "void") {
        return Err(format!(
            "CONFLICT: cannot record payment for invoice in '{}' status",
            invoice.status
        ));
    }

    if payment.currency_code != invoice.currency_code {
        return Err("VALIDATION: payment currency must match invoice currency".to_string());
    }

    let saved = db::queries::payments::insert(&conn, &payment).map_err(|e| e.to_string())?;

    // Update amount_paid_minor on the invoice
    let total_paid =
        db::queries::payments::get_total_paid(&conn, &invoice_id).map_err(|e| e.to_string())?;
    db::queries::invoices::update_amount_paid(&conn, &invoice_id, total_paid)
        .map_err(|e| e.to_string())?;

    // Auto-mark as paid if fully paid
    if total_paid >= invoice.total_minor && invoice.status != "paid" && invoice.status != "void" {
        db::queries::invoices::update_status(&conn, &invoice_id, "paid", Some("paid_at"))
            .map_err(|e| e.to_string())?;
    }

    serde_json::to_value(saved).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_payment(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let payment = db::queries::payments::get_by_id(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Payment not found: {}", id))?;
    let invoice_id = payment.invoice_id.clone();

    db::queries::payments::delete(&conn, &id).map_err(|e| e.to_string())?;

    // Recalculate amount_paid
    let total_paid =
        db::queries::payments::get_total_paid(&conn, &invoice_id).map_err(|e| e.to_string())?;
    db::queries::invoices::update_amount_paid(&conn, &invoice_id, total_paid)
        .map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_by_id(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;
    if invoice.status == "paid" && total_paid < invoice.total_minor {
        let fallback_status = if invoice.sent_at.is_some() {
            "sent"
        } else {
            "finalized"
        };
        db::queries::invoices::update_status(&conn, &invoice_id, fallback_status, None)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_invoice_payment_summary(
    db: State<'_, DbConn>,
    invoice_id: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_by_id(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let total_paid =
        db::queries::payments::get_total_paid(&conn, &invoice_id).map_err(|e| e.to_string())?;

    let summary = PaymentSummary {
        invoice_id: invoice_id.clone(),
        total_invoiced_minor: invoice.total_minor,
        total_paid_minor: total_paid,
        balance_due_minor: invoice.total_minor - total_paid,
        currency_code: invoice.currency_code.clone(),
        is_fully_paid: total_paid >= invoice.total_minor,
    };

    serde_json::to_value(summary).map_err(|e| e.to_string())
}
