use crate::models::invoice::{CreateInvoice, Invoice, InvoiceWithDetails, UpdateInvoice};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_invoice(row: &rusqlite::Row<'_>) -> Result<Invoice> {
    let uses_inclusive_taxes: i32 = row.get(14)?;
    Ok(Invoice {
        id: row.get(0)?,
        invoice_number: row.get(1)?,
        client_id: row.get(2)?,
        status: row.get(3)?,
        currency_code: row.get(4)?,
        subtotal_minor: row.get(5)?,
        discount_minor: row.get(6)?,
        tax_amount_minor: row.get(7)?,
        total_minor: row.get(8)?,
        amount_paid_minor: row.get(9)?,
        exchange_rate_to_usd: row.get(10)?,
        exchange_rate_date: row.get(11)?,
        issue_date: row.get(12)?,
        due_date: row.get(13)?,
        uses_inclusive_taxes: uses_inclusive_taxes != 0,
        notes: row.get(15)?,
        terms: row.get(16)?,
        footer: row.get(17)?,
        created_at: row.get(18)?,
        updated_at: row.get(19)?,
        finalized_at: row.get(20)?,
        sent_at: row.get(21)?,
        paid_at: row.get(22)?,
        voided_at: row.get(23)?,
    })
}

const SELECT_COLS: &str = "id, invoice_number, client_id, status, currency_code,
     subtotal_minor, discount_minor, tax_amount_minor, total_minor, amount_paid_minor,
     exchange_rate_to_usd, exchange_rate_date, issue_date, due_date,
     uses_inclusive_taxes, notes, terms, footer,
     created_at, updated_at, finalized_at, sent_at, paid_at, voided_at";

pub fn list_all(conn: &Connection) -> Result<Vec<Invoice>> {
    let sql = format!(
        "SELECT {} FROM invoices ORDER BY created_at DESC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_invoice)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Invoice>> {
    let sql = format!("SELECT {} FROM invoices WHERE id=?1", SELECT_COLS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_invoice)?;
    rows.next().transpose()
}

pub fn get_with_details(conn: &Connection, id: &str) -> Result<Option<InvoiceWithDetails>> {
    let invoice = match get_by_id(conn, id)? {
        Some(i) => i,
        None => return Ok(None),
    };

    let line_items = super::line_items::list_for_invoice(conn, id)?;
    let taxes = super::taxes::list_for_invoice(conn, id)?;
    let payments = super::payments::list_for_invoice(conn, id)?;
    let client = super::clients::get_by_id(conn, &invoice.client_id)?;

    Ok(Some(InvoiceWithDetails {
        id: invoice.id,
        invoice_number: invoice.invoice_number,
        client_id: invoice.client_id,
        client,
        status: invoice.status,
        currency_code: invoice.currency_code,
        subtotal_minor: invoice.subtotal_minor,
        discount_minor: invoice.discount_minor,
        tax_amount_minor: invoice.tax_amount_minor,
        total_minor: invoice.total_minor,
        amount_paid_minor: invoice.amount_paid_minor,
        exchange_rate_to_usd: invoice.exchange_rate_to_usd,
        exchange_rate_date: invoice.exchange_rate_date,
        issue_date: invoice.issue_date,
        due_date: invoice.due_date,
        uses_inclusive_taxes: invoice.uses_inclusive_taxes,
        notes: invoice.notes,
        terms: invoice.terms,
        footer: invoice.footer,
        created_at: invoice.created_at,
        updated_at: invoice.updated_at,
        finalized_at: invoice.finalized_at,
        sent_at: invoice.sent_at,
        paid_at: invoice.paid_at,
        voided_at: invoice.voided_at,
        line_items,
        taxes,
        payments,
    }))
}

pub fn insert(conn: &Connection, c: &CreateInvoice) -> Result<Invoice> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO invoices (id, client_id, currency_code, issue_date, due_date,
                               uses_inclusive_taxes, notes, terms, footer,
                               discount_minor, exchange_rate_to_usd, exchange_rate_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            id,
            c.client_id,
            c.currency_code.as_deref().unwrap_or("USD"),
            c.issue_date.as_deref().unwrap_or(""),
            c.due_date,
            if c.uses_inclusive_taxes.unwrap_or(false) {
                1i32
            } else {
                0i32
            },
            c.notes.as_deref().unwrap_or(""),
            c.terms.as_deref().unwrap_or(""),
            c.footer.as_deref().unwrap_or(""),
            c.discount_minor.unwrap_or(0),
            c.exchange_rate_to_usd,
            c.exchange_rate_date,
        ],
    )?;
    super::changelog::insert_entry(conn, "invoices", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn update(conn: &Connection, id: &str, u: &UpdateInvoice) -> Result<Invoice> {
    if let Some(v) = &u.client_id {
        conn.execute(
            "UPDATE invoices SET client_id=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.currency_code {
        conn.execute(
            "UPDATE invoices SET currency_code=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.issue_date {
        conn.execute(
            "UPDATE invoices SET issue_date=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.due_date {
        conn.execute(
            "UPDATE invoices SET due_date=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.uses_inclusive_taxes {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE invoices SET uses_inclusive_taxes=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    if let Some(v) = &u.notes {
        conn.execute(
            "UPDATE invoices SET notes=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.terms {
        conn.execute(
            "UPDATE invoices SET terms=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.footer {
        conn.execute(
            "UPDATE invoices SET footer=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.discount_minor {
        conn.execute(
            "UPDATE invoices SET discount_minor=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.exchange_rate_to_usd {
        conn.execute(
            "UPDATE invoices SET exchange_rate_to_usd=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.exchange_rate_date {
        conn.execute(
            "UPDATE invoices SET exchange_rate_date=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    super::changelog::insert_entry(conn, "invoices", id, "UPDATE", "{}")?;
    get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "invoices", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM invoices WHERE id=?1", params![id])?;
    Ok(())
}

pub fn update_status(
    conn: &Connection,
    id: &str,
    status: &str,
    timestamp_col: Option<&str>,
) -> Result<()> {
    conn.execute(
        "UPDATE invoices SET status=?1, updated_at=datetime('now') WHERE id=?2",
        params![status, id],
    )?;
    if let Some(col) = timestamp_col {
        let sql = format!(
            "UPDATE invoices SET {}=datetime('now'), updated_at=datetime('now') WHERE id=?1",
            col
        );
        conn.execute(&sql, params![id])?;
    }
    super::changelog::insert_entry(conn, "invoices", id, "UPDATE", "{}")?;
    Ok(())
}

pub fn update_totals(
    conn: &Connection,
    id: &str,
    subtotal_minor: i64,
    discount_minor: i64,
    tax_amount_minor: i64,
    total_minor: i64,
) -> Result<()> {
    conn.execute(
        "UPDATE invoices SET subtotal_minor=?1, discount_minor=?2, tax_amount_minor=?3,
                              total_minor=?4, updated_at=datetime('now')
         WHERE id=?5",
        params![
            subtotal_minor,
            discount_minor,
            tax_amount_minor,
            total_minor,
            id
        ],
    )?;
    Ok(())
}

pub fn update_amount_paid(conn: &Connection, id: &str, amount_paid_minor: i64) -> Result<()> {
    conn.execute(
        "UPDATE invoices SET amount_paid_minor=?1, updated_at=datetime('now') WHERE id=?2",
        params![amount_paid_minor, id],
    )?;
    Ok(())
}

pub fn set_invoice_number(conn: &Connection, id: &str, number: &str) -> Result<()> {
    conn.execute(
        "UPDATE invoices SET invoice_number=?1, updated_at=datetime('now') WHERE id=?2",
        params![number, id],
    )?;
    Ok(())
}

pub fn search(conn: &Connection, query: &str) -> Result<Vec<Invoice>> {
    let pattern = format!("%{}%", query);
    let sql = format!(
        "SELECT {} FROM invoices
         WHERE invoice_number LIKE ?1 OR notes LIKE ?1
         ORDER BY created_at DESC LIMIT 50",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![pattern], row_to_invoice)?;
    rows.collect()
}
