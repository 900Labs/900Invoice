use crate::models::line_item::{compute_line_total, CreateLineItem, LineItem, UpdateLineItem};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_line_item(row: &rusqlite::Row<'_>) -> Result<LineItem> {
    Ok(LineItem {
        id: row.get(0)?,
        invoice_id: row.get(1)?,
        product_id: row.get(2)?,
        tax_rate_id: row.get(3)?,
        description: row.get(4)?,
        quantity: row.get(5)?,
        unit_price_minor: row.get(6)?,
        tax_rate_bps: row.get(7)?,
        discount_bps: row.get(8)?,
        line_total_minor: row.get(9)?,
        sort_order: row.get(10)?,
        created_at: row.get(11)?,
    })
}

const SELECT_COLS: &str =
    "id, invoice_id, product_id, tax_rate_id, description, quantity, unit_price_minor,
     tax_rate_bps, discount_bps, line_total_minor, sort_order, created_at";

pub fn list_for_invoice(conn: &Connection, invoice_id: &str) -> Result<Vec<LineItem>> {
    let sql = format!(
        "SELECT {} FROM invoice_line_items WHERE invoice_id=?1 ORDER BY sort_order ASC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![invoice_id], row_to_line_item)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<LineItem>> {
    let sql = format!("SELECT {} FROM invoice_line_items WHERE id=?1", SELECT_COLS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_line_item)?;
    rows.next().transpose()
}

pub fn insert(conn: &Connection, c: &CreateLineItem) -> Result<LineItem> {
    let id = Uuid::new_v4().to_string();
    let quantity = c.quantity.unwrap_or(100);
    let tax_rate_bps = c.tax_rate_bps.unwrap_or(0);
    let discount_bps = c.discount_bps.unwrap_or(0);
    let sort_order = c.sort_order.unwrap_or(0);
    let line_total = compute_line_total(quantity, c.unit_price_minor, discount_bps);

    conn.execute(
        "INSERT INTO invoice_line_items
             (id, invoice_id, product_id, tax_rate_id, description, quantity, unit_price_minor,
              tax_rate_bps, discount_bps, line_total_minor, sort_order)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            id,
            c.invoice_id,
            c.product_id,
            c.tax_rate_id,
            c.description,
            quantity,
            c.unit_price_minor,
            tax_rate_bps,
            discount_bps,
            line_total,
            sort_order,
        ],
    )?;
    super::changelog::insert_entry(conn, "invoice_line_items", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn update(conn: &Connection, id: &str, u: &UpdateLineItem) -> Result<LineItem> {
    if let Some(v) = &u.product_id {
        conn.execute(
            "UPDATE invoice_line_items SET product_id=?1 WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.tax_rate_id {
        conn.execute(
            "UPDATE invoice_line_items SET tax_rate_id=?1 WHERE id=?2",
            params![if v.is_empty() { None } else { Some(v.as_str()) }, id],
        )?;
    }
    if let Some(v) = &u.description {
        conn.execute(
            "UPDATE invoice_line_items SET description=?1 WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.quantity {
        conn.execute(
            "UPDATE invoice_line_items SET quantity=?1 WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.unit_price_minor {
        conn.execute(
            "UPDATE invoice_line_items SET unit_price_minor=?1 WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.tax_rate_bps {
        conn.execute(
            "UPDATE invoice_line_items SET tax_rate_bps=?1 WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.discount_bps {
        conn.execute(
            "UPDATE invoice_line_items SET discount_bps=?1 WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.sort_order {
        conn.execute(
            "UPDATE invoice_line_items SET sort_order=?1 WHERE id=?2",
            params![v, id],
        )?;
    }

    // Recompute line_total after any update
    let item = get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;
    let new_total = compute_line_total(item.quantity, item.unit_price_minor, item.discount_bps);
    conn.execute(
        "UPDATE invoice_line_items SET line_total_minor=?1 WHERE id=?2",
        params![new_total, id],
    )?;

    super::changelog::insert_entry(conn, "invoice_line_items", id, "UPDATE", "{}")?;
    get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "invoice_line_items", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM invoice_line_items WHERE id=?1", params![id])?;
    Ok(())
}

pub fn reorder(conn: &Connection, ordered_ids: &[String]) -> Result<()> {
    for (index, item_id) in ordered_ids.iter().enumerate() {
        conn.execute(
            "UPDATE invoice_line_items SET sort_order=?1 WHERE id=?2",
            params![index as i32, item_id],
        )?;
    }
    Ok(())
}
