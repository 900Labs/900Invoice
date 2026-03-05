use crate::models::recurring::{CreateRecurring, RecurringInvoice, UpdateRecurring};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_recurring(row: &rusqlite::Row<'_>) -> Result<RecurringInvoice> {
    let auto_send: i32 = row.get(6)?;
    Ok(RecurringInvoice {
        id: row.get(0)?,
        client_id: row.get(1)?,
        template_invoice_id: row.get(2)?,
        frequency: row.get(3)?,
        next_generation_date: row.get(4)?,
        end_date: row.get(5)?,
        auto_send: auto_send != 0,
        last_generated: row.get(7)?,
        status: row.get(8)?,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}

const SELECT_COLS: &str = "id, client_id, template_invoice_id, frequency, next_generation_date,
     end_date, auto_send, last_generated, status, created_at, updated_at";

pub fn list_all(conn: &Connection) -> Result<Vec<RecurringInvoice>> {
    let sql = format!(
        "SELECT {} FROM recurring_invoices ORDER BY next_generation_date ASC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_recurring)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<RecurringInvoice>> {
    let sql = format!("SELECT {} FROM recurring_invoices WHERE id=?1", SELECT_COLS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_recurring)?;
    rows.next().transpose()
}

pub fn insert(conn: &Connection, c: &CreateRecurring) -> Result<RecurringInvoice> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO recurring_invoices (id, client_id, template_invoice_id, frequency,
                                         next_generation_date, end_date, auto_send)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            c.client_id,
            c.template_invoice_id,
            c.frequency,
            c.next_generation_date,
            c.end_date,
            if c.auto_send.unwrap_or(false) {
                1i32
            } else {
                0i32
            },
        ],
    )?;
    super::changelog::insert_entry(conn, "recurring_invoices", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn update(conn: &Connection, id: &str, u: &UpdateRecurring) -> Result<RecurringInvoice> {
    if let Some(v) = &u.frequency {
        conn.execute(
            "UPDATE recurring_invoices SET frequency=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.next_generation_date {
        conn.execute("UPDATE recurring_invoices SET next_generation_date=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.end_date {
        conn.execute(
            "UPDATE recurring_invoices SET end_date=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.auto_send {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE recurring_invoices SET auto_send=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    if let Some(v) = &u.status {
        conn.execute(
            "UPDATE recurring_invoices SET status=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    super::changelog::insert_entry(conn, "recurring_invoices", id, "UPDATE", "{}")?;
    get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "recurring_invoices", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM recurring_invoices WHERE id=?1", params![id])?;
    Ok(())
}

/// Returns recurring invoices that are active and due today or earlier
pub fn get_due(conn: &Connection, today: &str) -> Result<Vec<RecurringInvoice>> {
    let sql = format!(
        "SELECT {} FROM recurring_invoices
         WHERE status='active' AND next_generation_date <= ?1
         ORDER BY next_generation_date ASC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![today], row_to_recurring)?;
    rows.collect()
}

pub fn mark_generated(conn: &Connection, id: &str, next_date: &str) -> Result<()> {
    conn.execute(
        "UPDATE recurring_invoices
         SET last_generated=datetime('now'), next_generation_date=?1, updated_at=datetime('now')
         WHERE id=?2",
        params![next_date, id],
    )?;
    Ok(())
}
