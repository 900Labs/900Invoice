use crate::models::client::{Client, CreateClient, UpdateClient};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_client(row: &rusqlite::Row<'_>) -> Result<Client> {
    Ok(Client {
        id: row.get(0)?,
        name: row.get(1)?,
        email: row.get(2)?,
        phone: row.get(3)?,
        address: row.get(4)?,
        city: row.get(5)?,
        country: row.get(6)?,
        country_code: row.get(7)?,
        tax_id: row.get(8)?,
        currency_code: row.get(9)?,
        payment_terms_days: row.get(10)?,
        notes: row.get(11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

const SELECT_COLS: &str =
    "id, name, email, phone, address, city, country, country_code,
     tax_id, currency_code, payment_terms_days, notes, created_at, updated_at";

pub fn list_all(conn: &Connection) -> Result<Vec<Client>> {
    let sql = format!(
        "SELECT {} FROM clients ORDER BY name ASC",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_client)?;
    rows.collect()
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Client>> {
    let sql = format!(
        "SELECT {} FROM clients WHERE id=?1",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], row_to_client)?;
    rows.next().transpose()
}

pub fn insert(conn: &Connection, c: &CreateClient) -> Result<Client> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO clients (id, name, email, phone, address, city, country, country_code,
                              tax_id, currency_code, payment_terms_days, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            id,
            c.name,
            c.email.as_deref().unwrap_or(""),
            c.phone.as_deref().unwrap_or(""),
            c.address.as_deref().unwrap_or(""),
            c.city.as_deref().unwrap_or(""),
            c.country.as_deref().unwrap_or(""),
            c.country_code.as_deref().unwrap_or(""),
            c.tax_id.as_deref().unwrap_or(""),
            c.currency_code.as_deref().unwrap_or("USD"),
            c.payment_terms_days.unwrap_or(30),
            c.notes.as_deref().unwrap_or(""),
        ],
    )?;
    super::changelog::insert_entry(conn, "clients", &id, "INSERT", "{}")?;
    get_by_id(conn, &id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn update(conn: &Connection, id: &str, u: &UpdateClient) -> Result<Client> {
    if let Some(v) = &u.name {
        conn.execute("UPDATE clients SET name=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.email {
        conn.execute("UPDATE clients SET email=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.phone {
        conn.execute("UPDATE clients SET phone=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.address {
        conn.execute("UPDATE clients SET address=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.city {
        conn.execute("UPDATE clients SET city=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.country {
        conn.execute("UPDATE clients SET country=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.country_code {
        conn.execute("UPDATE clients SET country_code=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.tax_id {
        conn.execute("UPDATE clients SET tax_id=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.currency_code {
        conn.execute("UPDATE clients SET currency_code=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.payment_terms_days {
        conn.execute("UPDATE clients SET payment_terms_days=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &u.notes {
        conn.execute("UPDATE clients SET notes=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    super::changelog::insert_entry(conn, "clients", id, "UPDATE", "{}")?;
    get_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    super::changelog::insert_entry(conn, "clients", id, "DELETE", "{}")?;
    conn.execute("DELETE FROM clients WHERE id=?1", params![id])?;
    Ok(())
}

pub fn search(conn: &Connection, query: &str) -> Result<Vec<Client>> {
    let pattern = format!("%{}%", query);
    let sql = format!(
        "SELECT {} FROM clients
         WHERE name LIKE ?1 OR email LIKE ?1 OR phone LIKE ?1
         ORDER BY name ASC LIMIT 50",
        SELECT_COLS
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![pattern], row_to_client)?;
    rows.collect()
}
