use crate::models::payment::{CreatePayment, Payment};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_payment(row: &rusqlite::Row<'_>) -> Result<Payment> {
    Ok(Payment {
        id: row.get(0)?,
        invoice_id: row.get(1)?,
        amount_minor: row.get(2)?,
        currency_code: row.get(3)?,
        payment_method: row.get(4)?,
        payment_reference: row.get(5)?,
        notes: row.get(6)?,
        paid_at: row.get(7)?,
        created_at: row.get(8)?,
    })
}

const SELECT_COLS: &str =
    "id, invoice_id, amount_minor, currency_code, payment_method,
     payment_reference, notes, paid_at, created_at";

pub fn list_for_invoice(conn: &Connection, invoice_id: &str) -> Result<Vec<Payment>> {
    let sql = format!(
        "SELECT {} FROM payments WHERE invoice_id=?1 ORDER BY paid_at ASC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![invoice_id], row_to_payment)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Payment>> {
    let sql = format!("SELECT {} FROM payments WHERE id=?1", SELECT_COLS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_payment)?;
    rows.next().transpose()
}

pub fn insert(conn: &Connection, c: &CreatePayment) -> Result<Payment> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO payments (id, invoice_id, amount_minor, currency_code,
                               payment_method, payment_reference, notes, paid_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, COALESCE(?8, datetime('now')))",
        params![
            id,
            c.invoice_id,
            c.amount_minor,
            c.currency_code,
            c.payment_method.as_deref().unwrap_or("cash"),
            c.payment_reference.as_deref().unwrap_or(""),
            c.notes.as_deref().unwrap_or(""),
            c.paid_at,
        ],
    )?;
    super::changelog::insert_entry(conn, "payments", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "payments", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM payments WHERE id=?1", params![id])?;
    Ok(())
}

pub fn get_total_paid(conn: &Connection, invoice_id: &str) -> Result<i64> {
    let total: i64 = conn.query_row(
        "SELECT COALESCE(SUM(amount_minor), 0) FROM payments WHERE invoice_id=?1",
        params![invoice_id],
        |row| row.get(0),
    )?;
    Ok(total)
}
