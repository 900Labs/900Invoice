#![allow(dead_code)]
//! Recurring invoice scheduler for 900Invoice.
//!
//! Processes recurring invoice templates and generates new invoices when due.
//! Handles: weekly, monthly, quarterly, annual frequencies.
//! Recovers missed jobs gracefully (processes all overdue recurring invoices).

use chrono::{Datelike, Duration, NaiveDate};
use rusqlite::Connection;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Returns all recurring invoice IDs that are due for generation today or earlier.
/// Only returns `status = 'active'` recurring records.
pub fn get_due_recurring(conn: &Connection) -> Result<Vec<String>, String> {
    let today = today_iso();

    let mut stmt = conn
        .prepare(
            "SELECT id FROM recurring_invoices \
             WHERE status = 'active' \
             AND next_generation_date <= ?1",
        )
        .map_err(|e| format!("Failed to prepare due-recurring query: {e}"))?;

    let ids: Vec<String> = stmt
        .query_map(rusqlite::params![today], |row| row.get(0))
        .map_err(|e| format!("Failed to query due recurring: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(ids)
}

/// Generate a new invoice from a recurring template.
///
/// 1. Loads the recurring record and its template invoice with all line items.
/// 2. Creates a new invoice as a draft copy with:
///    - New UUID and new invoice number
///    - issue_date = today, due_date = today + payment_terms_days
///    - status = 'draft'
///    - All line items duplicated
/// 3. Updates the recurring record:
///    - last_generated = now
///    - next_generation_date = calculated from current next_generation_date + frequency
/// 4. If end_date is set and has passed, sets recurring status to 'completed'.
///
/// Returns the new invoice UUID.
pub fn generate_from_template(conn: &Connection, recurring_id: &str) -> Result<String, String> {
    // Load recurring record
    let rec = load_recurring(conn, recurring_id)?;

    // Load template invoice
    let template = load_invoice(conn, &rec.template_invoice_id)?;

    // Get payment terms from client
    let payment_terms_days = load_client_payment_terms(conn, &template.client_id)?;

    // Generate invoice number
    let new_invoice_number =
        crate::services::invoice_numbering::generate_invoice_number(conn, "default")
            .unwrap_or_else(|_| format!("INV-{}", Uuid::new_v4().to_string()[..8].to_uppercase()));

    let today = today_iso();
    let due_date = add_days(&today, payment_terms_days);
    let new_invoice_id = Uuid::new_v4().to_string();
    let now = now_iso();

    // Insert the new invoice
    conn.execute(
        "INSERT INTO invoices (
            id, invoice_number, client_id, status, currency_code,
            subtotal_minor, discount_minor, tax_amount_minor, total_minor,
            amount_paid_minor, issue_date, due_date, uses_inclusive_taxes,
            notes, terms, footer, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, 'draft', ?4,
            ?5, ?6, ?7, ?8,
            0, ?9, ?10, ?11,
            ?12, ?13, ?14, ?15, ?16
        )",
        rusqlite::params![
            new_invoice_id,
            new_invoice_number,
            template.client_id,
            template.currency_code,
            template.subtotal_minor,
            template.discount_minor,
            template.tax_amount_minor,
            template.total_minor,
            today,
            due_date,
            template.uses_inclusive_taxes,
            template.notes,
            template.terms,
            template.footer,
            now,
            now,
        ],
    )
    .map_err(|e| format!("Failed to insert generated invoice: {e}"))?;

    // Duplicate all line items
    let line_items = load_line_items(conn, &template.id)?;
    for item in &line_items {
        let new_item_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO invoice_line_items (
                id, invoice_id, product_id, description,
                quantity, unit_price_minor, tax_rate_bps, discount_bps,
                line_total_minor, sort_order, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                new_item_id,
                new_invoice_id,
                item.product_id,
                item.description,
                item.quantity,
                item.unit_price_minor,
                item.tax_rate_bps,
                item.discount_bps,
                item.line_total_minor,
                item.sort_order,
                now,
            ],
        )
        .map_err(|e| format!("Failed to duplicate line item: {e}"))?;
    }

    let invoice_taxes = load_invoice_taxes(conn, &template.id)?;
    for tax in &invoice_taxes {
        let new_tax_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO invoice_taxes (
                id, invoice_id, tax_rate_id, tax_name, tax_rate_bps,
                tax_amount_minor, is_withholding, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                new_tax_id,
                new_invoice_id,
                tax.tax_rate_id,
                tax.tax_name,
                tax.tax_rate_bps,
                tax.tax_amount_minor,
                tax.is_withholding,
                now,
            ],
        )
        .map_err(|e| format!("Failed to duplicate invoice tax: {e}"))?;
    }

    // Calculate the next generation date based on current next_generation_date
    // (not today, to avoid drift — we advance from the scheduled date)
    let next_date = calculate_next_date(&rec.next_generation_date, &rec.frequency);

    // Check if end_date has passed
    let new_status = if let Some(end) = &rec.end_date {
        if next_date.as_str() > end.as_str() {
            "completed"
        } else {
            "active"
        }
    } else {
        "active"
    };

    // Update recurring record
    conn.execute(
        "UPDATE recurring_invoices \
         SET last_generated = ?1, next_generation_date = ?2, status = ?3, updated_at = ?4 \
         WHERE id = ?5",
        rusqlite::params![now, next_date, new_status, now, recurring_id],
    )
    .map_err(|e| format!("Failed to update recurring record: {e}"))?;

    // Record changelog entries
    let _ = record_changelog(
        conn,
        "invoices",
        &new_invoice_id,
        "INSERT",
        &format!(
            r#"{{"source":"recurring","recurring_id":"{}"}}"#,
            recurring_id
        ),
    );

    Ok(new_invoice_id)
}

/// Calculate the next generation date from a current date and frequency string.
///
/// - "weekly"    → +7 days
/// - "monthly"   → +1 month (same day, clamped to month end)
/// - "quarterly" → +3 months
/// - "annual"    → +1 year
///
/// If the date cannot be parsed, falls back to +30 days.
pub fn calculate_next_date(current: &str, frequency: &str) -> String {
    let date = match NaiveDate::parse_from_str(current, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return add_days(current, 30),
    };

    let next = match frequency.to_lowercase().as_str() {
        "weekly" => date + Duration::days(7),
        "biweekly" => date + Duration::days(14),
        "monthly" => advance_months(date, 1),
        "quarterly" => advance_months(date, 3),
        "annual" | "annually" | "yearly" => advance_months(date, 12),
        _ => date + Duration::days(30),
    };

    next.format("%Y-%m-%d").to_string()
}

/// Process all due recurring invoices.
///
/// Called on app startup and periodically. Recovers missed jobs:
/// if a recurring invoice was due multiple periods ago, it will be
/// processed once now and scheduled forward (not retroactively spammed).
///
/// Returns a list of newly generated invoice IDs.
pub fn process_all_due(conn: &Connection) -> Result<Vec<String>, String> {
    let due_ids = get_due_recurring(conn)?;
    let mut generated = Vec::new();

    for recurring_id in &due_ids {
        match generate_from_template(conn, recurring_id) {
            Ok(invoice_id) => {
                generated.push(invoice_id);
            }
            Err(e) => {
                // Log and continue — don't let one failure block others
                eprintln!("[recurring_scheduler] Failed to generate from {recurring_id}: {e}");
            }
        }
    }

    Ok(generated)
}

// ---------------------------------------------------------------------------
// Internal data structures
// ---------------------------------------------------------------------------

struct RecurringRecord {
    id: String,
    template_invoice_id: String,
    frequency: String,
    next_generation_date: String,
    end_date: Option<String>,
}

struct InvoiceTemplate {
    id: String,
    client_id: String,
    currency_code: String,
    subtotal_minor: i64,
    discount_minor: i64,
    tax_amount_minor: i64,
    total_minor: i64,
    uses_inclusive_taxes: i32,
    notes: String,
    terms: String,
    footer: String,
}

struct LineItemTemplate {
    product_id: Option<String>,
    description: String,
    quantity: i32,
    unit_price_minor: i64,
    tax_rate_bps: i32,
    discount_bps: i32,
    line_total_minor: i64,
    sort_order: i32,
}

struct InvoiceTaxTemplate {
    tax_rate_id: Option<String>,
    tax_name: String,
    tax_rate_bps: i32,
    tax_amount_minor: i64,
    is_withholding: i32,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn load_recurring(conn: &Connection, recurring_id: &str) -> Result<RecurringRecord, String> {
    conn.query_row(
        "SELECT id, template_invoice_id, frequency, next_generation_date, end_date \
         FROM recurring_invoices WHERE id = ?1",
        rusqlite::params![recurring_id],
        |row| {
            Ok(RecurringRecord {
                id: row.get(0)?,
                template_invoice_id: row.get(1)?,
                frequency: row.get(2)?,
                next_generation_date: row.get(3)?,
                end_date: row.get(4)?,
            })
        },
    )
    .map_err(|e| format!("Recurring record '{}' not found: {}", recurring_id, e))
}

fn load_invoice(conn: &Connection, invoice_id: &str) -> Result<InvoiceTemplate, String> {
    conn.query_row(
        "SELECT id, client_id, currency_code, subtotal_minor, discount_minor, \
         tax_amount_minor, total_minor, uses_inclusive_taxes, notes, terms, footer \
         FROM invoices WHERE id = ?1",
        rusqlite::params![invoice_id],
        |row| {
            Ok(InvoiceTemplate {
                id: row.get(0)?,
                client_id: row.get(1)?,
                currency_code: row.get(2)?,
                subtotal_minor: row.get(3)?,
                discount_minor: row.get(4)?,
                tax_amount_minor: row.get(5)?,
                total_minor: row.get(6)?,
                uses_inclusive_taxes: row.get(7)?,
                notes: row.get(8)?,
                terms: row.get(9)?,
                footer: row.get(10)?,
            })
        },
    )
    .map_err(|e| format!("Template invoice '{}' not found: {}", invoice_id, e))
}

fn load_client_payment_terms(conn: &Connection, client_id: &str) -> Result<i64, String> {
    match conn.query_row(
        "SELECT payment_terms_days FROM clients WHERE id = ?1",
        rusqlite::params![client_id],
        |row| row.get::<_, i64>(0),
    ) {
        Ok(days) => Ok(days),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(30),
        Err(e) => Err(format!("Failed to load client payment terms: {e}")),
    }
}

fn load_line_items(conn: &Connection, invoice_id: &str) -> Result<Vec<LineItemTemplate>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT product_id, description, quantity, unit_price_minor, \
             tax_rate_bps, discount_bps, line_total_minor, sort_order \
             FROM invoice_line_items \
             WHERE invoice_id = ?1 \
             ORDER BY sort_order",
        )
        .map_err(|e| format!("Failed to prepare line items query: {e}"))?;

    let items = stmt
        .query_map(rusqlite::params![invoice_id], |row| {
            Ok(LineItemTemplate {
                product_id: row.get(0)?,
                description: row.get(1)?,
                quantity: row.get(2)?,
                unit_price_minor: row.get(3)?,
                tax_rate_bps: row.get(4)?,
                discount_bps: row.get(5)?,
                line_total_minor: row.get(6)?,
                sort_order: row.get(7)?,
            })
        })
        .map_err(|e| format!("Failed to query line items: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}

fn load_invoice_taxes(
    conn: &Connection,
    invoice_id: &str,
) -> Result<Vec<InvoiceTaxTemplate>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT tax_rate_id, tax_name, tax_rate_bps, tax_amount_minor, is_withholding
             FROM invoice_taxes
             WHERE invoice_id = ?1
             ORDER BY created_at",
        )
        .map_err(|e| format!("Failed to prepare invoice taxes query: {e}"))?;

    let taxes = stmt
        .query_map(rusqlite::params![invoice_id], |row| {
            Ok(InvoiceTaxTemplate {
                tax_rate_id: row.get(0)?,
                tax_name: row.get(1)?,
                tax_rate_bps: row.get(2)?,
                tax_amount_minor: row.get(3)?,
                is_withholding: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query invoice taxes: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(taxes)
}

/// Advance a NaiveDate by N months, clamping to end of month if needed.
fn advance_months(date: NaiveDate, months: u32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() + months;
    // Carry over years
    year += ((month - 1) / 12) as i32;
    month = ((month - 1) % 12) + 1;

    // Clamp day to last day of month
    let day = date.day();
    let max_day = days_in_month(year, month);
    let clamped_day = day.min(max_day);

    NaiveDate::from_ymd_opt(year, month, clamped_day)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, month, 1).unwrap())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn add_days(date_str: &str, days: i64) -> String {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(d) => (d + Duration::days(days)).format("%Y-%m-%d").to_string(),
        Err(_) => date_str.to_string(),
    }
}

fn today_iso() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn now_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn record_changelog(
    conn: &Connection,
    table_name: &str,
    row_id: &str,
    operation: &str,
    payload: &str,
) -> Result<(), String> {
    let now = now_iso();
    conn.execute(
        "INSERT INTO changelog (table_name, row_id, operation, payload, timestamp) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![table_name, row_id, operation, payload, now],
    )
    .map_err(|e| format!("Failed to record changelog: {e}"))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;

    #[test]
    fn test_calculate_next_weekly() {
        let next = calculate_next_date("2026-03-05", "weekly");
        assert_eq!(next, "2026-03-12");
    }

    #[test]
    fn test_calculate_next_monthly() {
        let next = calculate_next_date("2026-01-31", "monthly");
        // Feb has no 31st — should clamp to 28 (2026 is not leap)
        assert_eq!(next, "2026-02-28");
    }

    #[test]
    fn test_calculate_next_monthly_normal() {
        let next = calculate_next_date("2026-03-05", "monthly");
        assert_eq!(next, "2026-04-05");
    }

    #[test]
    fn test_calculate_next_quarterly() {
        let next = calculate_next_date("2026-01-15", "quarterly");
        assert_eq!(next, "2026-04-15");
    }

    #[test]
    fn test_calculate_next_annual() {
        let next = calculate_next_date("2026-02-28", "annual");
        assert_eq!(next, "2027-02-28");
    }

    #[test]
    fn test_calculate_next_annually() {
        let next = calculate_next_date("2026-02-28", "annually");
        assert_eq!(next, "2027-02-28");
    }

    #[test]
    fn test_calculate_next_annual_leap() {
        // Feb 29 on a leap year → next year Feb 28
        let next = calculate_next_date("2024-02-29", "annual");
        assert_eq!(next, "2025-02-28");
    }

    #[test]
    fn test_calculate_next_biweekly() {
        let next = calculate_next_date("2026-03-01", "biweekly");
        assert_eq!(next, "2026-03-15");
    }

    #[test]
    fn test_calculate_next_unknown_frequency() {
        // Unknown -> 30 days
        let next = calculate_next_date("2026-03-01", "semiweekly");
        assert_eq!(next, "2026-03-31");
    }

    #[test]
    fn test_add_days() {
        assert_eq!(add_days("2026-03-28", 5), "2026-04-02");
        assert_eq!(add_days("2026-12-31", 1), "2027-01-01");
    }

    #[test]
    fn test_leap_year_detection() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2026));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_advance_months_year_boundary() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 30).unwrap();
        let next = advance_months(date, 3);
        assert_eq!(next.to_string(), "2027-02-28");
    }

    #[test]
    fn test_generate_from_template_copies_totals_lines_and_taxes() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        migrations::run_migrations(&conn).expect("migrations");

        conn.execute(
            "INSERT INTO clients
             (id, name, currency_code, payment_terms_days)
             VALUES ('client-1', 'Acme', 'USD', 10)",
            [],
        )
        .expect("client");
        conn.execute(
            "INSERT INTO invoices
             (id, invoice_number, client_id, status, currency_code, subtotal_minor,
              discount_minor, tax_amount_minor, total_minor, amount_paid_minor,
              issue_date, due_date, uses_inclusive_taxes, notes, terms, footer)
             VALUES
             ('template-1', 'INV-TEMPLATE', 'client-1', 'finalized', 'USD', 10000,
              0, 1600, 11600, 0, '2026-05-01', '2026-05-11', 0, '', '', '')",
            [],
        )
        .expect("template invoice");
        conn.execute(
            "INSERT INTO invoice_line_items
             (id, invoice_id, description, quantity, unit_price_minor, tax_rate_bps,
              discount_bps, line_total_minor, sort_order)
             VALUES ('line-1', 'template-1', 'Consulting', 100, 10000, 1600, 0, 10000, 0)",
            [],
        )
        .expect("line item");
        conn.execute(
            "INSERT INTO invoice_taxes
             (id, invoice_id, tax_rate_id, tax_name, tax_rate_bps, tax_amount_minor, is_withholding)
             VALUES ('tax-line-1', 'template-1', 'tax-ke-vat', 'VAT @ 16%', 1600, 1600, 0)",
            [],
        )
        .expect("invoice tax");
        conn.execute(
            "INSERT INTO recurring_invoices
             (id, client_id, template_invoice_id, frequency, next_generation_date, auto_send, status)
             VALUES ('recurring-1', 'client-1', 'template-1', 'annually', '2026-05-11', 0, 'active')",
            [],
        )
        .expect("recurring");

        let generated_id =
            generate_from_template(&conn, "recurring-1").expect("generated invoice id");

        let (subtotal, tax, total): (i64, i64, i64) = conn
            .query_row(
                "SELECT subtotal_minor, tax_amount_minor, total_minor FROM invoices WHERE id=?1",
                rusqlite::params![generated_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .expect("generated invoice");
        assert_eq!((subtotal, tax, total), (10000, 1600, 11600));

        let line_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM invoice_line_items WHERE invoice_id=?1",
                rusqlite::params![generated_id],
                |row| row.get(0),
            )
            .expect("line count");
        let tax_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM invoice_taxes WHERE invoice_id=?1",
                rusqlite::params![generated_id],
                |row| row.get(0),
            )
            .expect("tax count");
        assert_eq!(line_count, 1);
        assert_eq!(tax_count, 1);

        let next_generation_date: String = conn
            .query_row(
                "SELECT next_generation_date FROM recurring_invoices WHERE id='recurring-1'",
                [],
                |row| row.get(0),
            )
            .expect("next date");
        assert_eq!(next_generation_date, "2027-05-11");
    }
}
