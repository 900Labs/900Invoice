use crate::models::tax::{CreateInvoiceTax, CreateTaxRate, InvoiceTax, TaxRate, UpdateTaxRate};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_tax_rate(row: &rusqlite::Row<'_>) -> Result<TaxRate> {
    let is_default: i32 = row.get(5)?;
    let is_withholding: i32 = row.get(6)?;
    let is_inclusive: i32 = row.get(7)?;
    let is_active: i32 = row.get(8)?;
    Ok(TaxRate {
        id: row.get(0)?,
        name: row.get(1)?,
        display_name: row.get(2)?,
        rate_bps: row.get(3)?,
        country_code: row.get(4)?,
        is_default: is_default != 0,
        is_withholding: is_withholding != 0,
        is_inclusive: is_inclusive != 0,
        is_active: is_active != 0,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}

fn row_to_invoice_tax(row: &rusqlite::Row<'_>) -> Result<InvoiceTax> {
    let is_withholding: i32 = row.get(6)?;
    Ok(InvoiceTax {
        id: row.get(0)?,
        invoice_id: row.get(1)?,
        tax_rate_id: row.get(2)?,
        tax_name: row.get(3)?,
        tax_rate_bps: row.get(4)?,
        tax_amount_minor: row.get(5)?,
        is_withholding: is_withholding != 0,
        created_at: row.get(7)?,
    })
}

const TAX_RATE_COLS: &str = "id, name, display_name, rate_bps, country_code,
     is_default, is_withholding, is_inclusive, is_active, created_at, updated_at";

pub fn list_all(conn: &Connection) -> Result<Vec<TaxRate>> {
    let sql = format!(
        "SELECT {} FROM tax_rates WHERE is_active=1 ORDER BY name ASC",
        TAX_RATE_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_tax_rate)?;
    rows.collect()
}

pub fn list_for_country(conn: &Connection, country_code: &str) -> Result<Vec<TaxRate>> {
    let sql = format!(
        "SELECT {} FROM tax_rates WHERE (country_code=?1 OR country_code IS NULL) AND is_active=1 ORDER BY name ASC",
        TAX_RATE_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![country_code], row_to_tax_rate)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<TaxRate>> {
    let sql = format!("SELECT {} FROM tax_rates WHERE id=?1", TAX_RATE_COLS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_tax_rate)?;
    rows.next().transpose()
}

pub fn insert(conn: &Connection, c: &CreateTaxRate) -> Result<TaxRate> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO tax_rates (id, name, display_name, rate_bps, country_code,
                                is_default, is_withholding, is_inclusive)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            id,
            c.name,
            c.display_name,
            c.rate_bps,
            c.country_code,
            if c.is_default.unwrap_or(false) {
                1i32
            } else {
                0i32
            },
            if c.is_withholding.unwrap_or(false) {
                1i32
            } else {
                0i32
            },
            if c.is_inclusive.unwrap_or(false) {
                1i32
            } else {
                0i32
            },
        ],
    )?;
    super::changelog::insert_entry(conn, "tax_rates", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn update(conn: &Connection, id: &str, u: &UpdateTaxRate) -> Result<TaxRate> {
    if let Some(v) = &u.name {
        conn.execute(
            "UPDATE tax_rates SET name=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.display_name {
        conn.execute(
            "UPDATE tax_rates SET display_name=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.rate_bps {
        conn.execute(
            "UPDATE tax_rates SET rate_bps=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.country_code {
        conn.execute(
            "UPDATE tax_rates SET country_code=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.is_default {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE tax_rates SET is_default=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    if let Some(v) = &u.is_withholding {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE tax_rates SET is_withholding=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    if let Some(v) = &u.is_inclusive {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE tax_rates SET is_inclusive=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    if let Some(v) = &u.is_active {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE tax_rates SET is_active=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    super::changelog::insert_entry(conn, "tax_rates", id, "UPDATE", "{}")?;
    get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "tax_rates", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM tax_rates WHERE id=?1", params![id])?;
    Ok(())
}

pub fn list_for_invoice(conn: &Connection, invoice_id: &str) -> Result<Vec<InvoiceTax>> {
    let mut stmt = conn.prepare(
        "SELECT id, invoice_id, tax_rate_id, tax_name, tax_rate_bps,
                tax_amount_minor, is_withholding, created_at
         FROM invoice_taxes WHERE invoice_id=?1 ORDER BY created_at ASC",
    )?;
    let rows = stmt.query_map(params![invoice_id], row_to_invoice_tax)?;
    rows.collect()
}

pub fn insert_invoice_tax(conn: &Connection, c: &CreateInvoiceTax) -> Result<InvoiceTax> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO invoice_taxes (id, invoice_id, tax_rate_id, tax_name, tax_rate_bps,
                                    tax_amount_minor, is_withholding)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            c.invoice_id,
            c.tax_rate_id,
            c.tax_name,
            c.tax_rate_bps,
            c.tax_amount_minor,
            if c.is_withholding { 1i32 } else { 0i32 },
        ],
    )?;
    super::changelog::insert_entry(conn, "invoice_taxes", &id, "INSERT", "{}")?;

    let mut stmt = conn.prepare(
        "SELECT id, invoice_id, tax_rate_id, tax_name, tax_rate_bps,
                tax_amount_minor, is_withholding, created_at
         FROM invoice_taxes WHERE id=?1",
    )?;
    let mut rows = stmt.query_map(params![id], row_to_invoice_tax)?;
    rows.next()
        .transpose()?
        .ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete_for_invoice(conn: &Connection, invoice_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM invoice_taxes WHERE invoice_id=?1",
        params![invoice_id],
    )?;
    Ok(())
}
