use crate::models::product::{CreateProduct, Product, UpdateProduct};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_product(row: &rusqlite::Row<'_>) -> Result<Product> {
    let is_active: i32 = row.get(7)?;
    Ok(Product {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        default_price_minor: row.get(3)?,
        default_currency: row.get(4)?,
        default_tax_rate_bps: row.get(5)?,
        unit: row.get(6)?,
        is_active: is_active != 0,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
    })
}

const SELECT_COLS: &str = "id, name, description, default_price_minor, default_currency,
     default_tax_rate_bps, unit, is_active, created_at, updated_at";

pub fn list_all(conn: &Connection) -> Result<Vec<Product>> {
    let sql = format!(
        "SELECT {} FROM products WHERE is_active=1 ORDER BY name ASC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_product)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Product>> {
    let sql = format!("SELECT {} FROM products WHERE id=?1", SELECT_COLS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_product)?;
    rows.next().transpose()
}

pub fn insert(conn: &Connection, c: &CreateProduct) -> Result<Product> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO products (id, name, description, default_price_minor, default_currency,
                               default_tax_rate_bps, unit)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            c.name,
            c.description.as_deref().unwrap_or(""),
            c.default_price_minor.unwrap_or(0),
            c.default_currency.as_deref().unwrap_or("USD"),
            c.default_tax_rate_bps.unwrap_or(0),
            c.unit.as_deref().unwrap_or("unit"),
        ],
    )?;
    super::changelog::insert_entry(conn, "products", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn update(conn: &Connection, id: &str, u: &UpdateProduct) -> Result<Product> {
    if let Some(v) = &u.name {
        conn.execute(
            "UPDATE products SET name=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.description {
        conn.execute(
            "UPDATE products SET description=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.default_price_minor {
        conn.execute(
            "UPDATE products SET default_price_minor=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.default_currency {
        conn.execute(
            "UPDATE products SET default_currency=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.default_tax_rate_bps {
        conn.execute(
            "UPDATE products SET default_tax_rate_bps=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.unit {
        conn.execute(
            "UPDATE products SET unit=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &u.is_active {
        let flag = if *v { 1i32 } else { 0i32 };
        conn.execute(
            "UPDATE products SET is_active=?1, updated_at=datetime('now') WHERE id=?2",
            params![flag, id],
        )?;
    }
    super::changelog::insert_entry(conn, "products", id, "UPDATE", "{}")?;
    get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "products", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM products WHERE id=?1", params![id])?;
    Ok(())
}

pub fn search(conn: &Connection, query: &str) -> Result<Vec<Product>> {
    let pattern = format!("%{}%", query);
    let sql = format!(
        "SELECT {} FROM products WHERE is_active=1 AND (name LIKE ?1 OR description LIKE ?1)
         ORDER BY name ASC LIMIT 50",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![pattern], row_to_product)?;
    rows.collect()
}
