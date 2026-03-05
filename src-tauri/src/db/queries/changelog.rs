use rusqlite::{params, Connection, Result};

pub fn insert_entry(
    conn: &Connection,
    table_name: &str,
    row_id: &str,
    operation: &str,
    payload: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO changelog (table_name, row_id, operation, payload, timestamp)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        params![table_name, row_id, operation, payload],
    )?;
    Ok(())
}

pub fn get_since(conn: &Connection, since_timestamp: &str) -> Result<Vec<serde_json::Value>> {
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, operation, payload, timestamp
         FROM changelog
         WHERE timestamp > ?1
         ORDER BY id ASC",
    )?;
    let rows = stmt.query_map(params![since_timestamp], |row| {
        let id: i64 = row.get(0)?;
        let table_name: String = row.get(1)?;
        let row_id: String = row.get(2)?;
        let operation: String = row.get(3)?;
        let payload: String = row.get(4)?;
        let timestamp: String = row.get(5)?;
        Ok((id, table_name, row_id, operation, payload, timestamp))
    })?;

    let mut entries = Vec::new();
    for row in rows {
        let (id, table_name, row_id, operation, payload, timestamp) = row?;
        let payload_val: serde_json::Value =
            serde_json::from_str(&payload).unwrap_or(serde_json::Value::Object(Default::default()));
        entries.push(serde_json::json!({
            "id": id,
            "table_name": table_name,
            "row_id": row_id,
            "operation": operation,
            "payload": payload_val,
            "timestamp": timestamp,
        }));
    }
    Ok(entries)
}

pub fn get_all(conn: &Connection) -> Result<Vec<serde_json::Value>> {
    get_since(conn, "1970-01-01T00:00:00")
}
