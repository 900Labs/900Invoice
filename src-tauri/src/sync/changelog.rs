/// Changelog module for 900Invoice multi-device sync.
///
/// Every INSERT, UPDATE, and DELETE on key tables is recorded here.
/// Downstream sync logic (cloud, peer-to-peer) can query changes since a timestamp.

use rusqlite::Connection;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A single changelog entry representing one data mutation.
#[derive(Debug, Clone)]
pub struct ChangelogEntry {
    /// Auto-increment primary key.
    pub id: i64,
    /// The table where the change occurred (e.g. "invoices", "clients").
    pub table_name: String,
    /// The primary key (UUID) of the affected row.
    pub row_id: String,
    /// One of: "INSERT", "UPDATE", "DELETE".
    pub operation: String,
    /// JSON payload with relevant changed fields or a full row snapshot.
    pub payload: String,
    /// ISO 8601 timestamp of when the change was recorded.
    pub timestamp: String,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Record a change in the changelog.
///
/// `payload` should be a JSON string describing the change:
/// - INSERT: full row data as JSON object
/// - UPDATE: changed fields as `{"field": "old_value → new_value"}` or new values only
/// - DELETE: `{"id": "<row_id>"}` or full snapshot
///
/// Returns an error if the insert fails.
pub fn record_change(
    conn: &Connection,
    table_name: &str,
    row_id: &str,
    operation: &str,
    payload: &str,
) -> Result<(), String> {
    validate_operation(operation)?;
    let now = now_iso();
    conn.execute(
        "INSERT INTO changelog (table_name, row_id, operation, payload, timestamp) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![table_name, row_id, operation, payload, now],
    )
    .map_err(|e| format!("Failed to record changelog entry for {table_name}/{row_id}: {e}"))?;
    Ok(())
}

/// Record a change with a specific timestamp (useful for migrations/imports).
pub fn record_change_at(
    conn: &Connection,
    table_name: &str,
    row_id: &str,
    operation: &str,
    payload: &str,
    timestamp: &str,
) -> Result<(), String> {
    validate_operation(operation)?;
    conn.execute(
        "INSERT INTO changelog (table_name, row_id, operation, payload, timestamp) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![table_name, row_id, operation, payload, timestamp],
    )
    .map_err(|e| format!("Failed to record changelog entry: {e}"))?;
    Ok(())
}

/// Get all changelog entries since a given ISO 8601 timestamp.
///
/// Returns entries in chronological order (oldest first).
/// `since` is inclusive: entries with `timestamp >= since` are returned.
pub fn get_changes_since(
    conn: &Connection,
    since: &str,
) -> Result<Vec<ChangelogEntry>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, table_name, row_id, operation, payload, timestamp \
             FROM changelog \
             WHERE timestamp >= ?1 \
             ORDER BY id ASC",
        )
        .map_err(|e| format!("Failed to prepare changelog query: {e}"))?;

    let entries = stmt
        .query_map(rusqlite::params![since], |row| {
            Ok(ChangelogEntry {
                id: row.get(0)?,
                table_name: row.get(1)?,
                row_id: row.get(2)?,
                operation: row.get(3)?,
                payload: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })
        .map_err(|e| format!("Failed to query changelog: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(entries)
}

/// Get all changelog entries (for initial sync or full export).
///
/// Returns entries in chronological order (oldest first).
pub fn get_all_changes(conn: &Connection) -> Result<Vec<ChangelogEntry>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, table_name, row_id, operation, payload, timestamp \
             FROM changelog \
             ORDER BY id ASC",
        )
        .map_err(|e| format!("Failed to prepare full changelog query: {e}"))?;

    let entries = stmt
        .query_map(rusqlite::params![], |row| {
            Ok(ChangelogEntry {
                id: row.get(0)?,
                table_name: row.get(1)?,
                row_id: row.get(2)?,
                operation: row.get(3)?,
                payload: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })
        .map_err(|e| format!("Failed to query full changelog: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(entries)
}

/// Get changes for a specific table since a timestamp.
pub fn get_changes_for_table_since(
    conn: &Connection,
    table_name: &str,
    since: &str,
) -> Result<Vec<ChangelogEntry>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, table_name, row_id, operation, payload, timestamp \
             FROM changelog \
             WHERE table_name = ?1 AND timestamp >= ?2 \
             ORDER BY id ASC",
        )
        .map_err(|e| format!("Failed to prepare table changelog query: {e}"))?;

    let entries = stmt
        .query_map(rusqlite::params![table_name, since], |row| {
            Ok(ChangelogEntry {
                id: row.get(0)?,
                table_name: row.get(1)?,
                row_id: row.get(2)?,
                operation: row.get(3)?,
                payload: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })
        .map_err(|e| format!("Failed to query table changelog: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(entries)
}

/// Get the most recent timestamp in the changelog.
/// Returns None if the changelog is empty.
pub fn get_latest_timestamp(conn: &Connection) -> Result<Option<String>, String> {
    let result: rusqlite::Result<String> = conn.query_row(
        "SELECT timestamp FROM changelog ORDER BY id DESC LIMIT 1",
        rusqlite::params![],
        |row| row.get(0),
    );

    match result {
        Ok(ts) => Ok(Some(ts)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to query latest timestamp: {e}")),
    }
}

/// Get the count of changelog entries since a timestamp.
/// Useful for quick sync checks.
pub fn count_changes_since(conn: &Connection, since: &str) -> Result<i64, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM changelog WHERE timestamp >= ?1",
        rusqlite::params![since],
        |row| row.get(0),
    )
    .map_err(|e| format!("Failed to count changelog entries: {e}"))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn validate_operation(operation: &str) -> Result<(), String> {
    match operation {
        "INSERT" | "UPDATE" | "DELETE" => Ok(()),
        _ => Err(format!(
            "Invalid changelog operation '{}'. Must be INSERT, UPDATE, or DELETE.",
            operation
        )),
    }
}

fn now_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS changelog (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                row_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                payload TEXT NOT NULL DEFAULT '{}',
                timestamp TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_record_change_insert() {
        let conn = setup_db();
        record_change(&conn, "invoices", "inv-001", "INSERT", r#"{"status":"draft"}"#).unwrap();
        let all = get_all_changes(&conn).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].table_name, "invoices");
        assert_eq!(all[0].row_id, "inv-001");
        assert_eq!(all[0].operation, "INSERT");
    }

    #[test]
    fn test_record_change_update_delete() {
        let conn = setup_db();
        record_change(&conn, "clients", "cli-001", "INSERT", r#"{"name":"Acme"}"#).unwrap();
        record_change(&conn, "clients", "cli-001", "UPDATE", r#"{"name":"Acme Corp"}"#).unwrap();
        record_change(&conn, "clients", "cli-001", "DELETE", r#"{"id":"cli-001"}"#).unwrap();
        let all = get_all_changes(&conn).unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(all[2].operation, "DELETE");
    }

    #[test]
    fn test_invalid_operation() {
        let conn = setup_db();
        let result = record_change(&conn, "invoices", "inv-001", "UPSERT", "{}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid changelog operation"));
    }

    #[test]
    fn test_get_changes_since() {
        let conn = setup_db();
        // Record with explicit past timestamp
        record_change_at(&conn, "invoices", "inv-001", "INSERT", "{}", "2026-01-01T00:00:00").unwrap();
        record_change_at(&conn, "invoices", "inv-002", "INSERT", "{}", "2026-06-01T00:00:00").unwrap();
        record_change_at(&conn, "invoices", "inv-003", "UPDATE", "{}", "2026-12-01T00:00:00").unwrap();

        let since = get_changes_since(&conn, "2026-05-01T00:00:00").unwrap();
        assert_eq!(since.len(), 2);
        assert_eq!(since[0].row_id, "inv-002");
        assert_eq!(since[1].row_id, "inv-003");
    }

    #[test]
    fn test_get_changes_since_empty() {
        let conn = setup_db();
        let changes = get_changes_since(&conn, "2030-01-01T00:00:00").unwrap();
        assert!(changes.is_empty());
    }

    #[test]
    fn test_count_changes_since() {
        let conn = setup_db();
        record_change_at(&conn, "invoices", "inv-001", "INSERT", "{}", "2026-01-01T00:00:00").unwrap();
        record_change_at(&conn, "invoices", "inv-002", "INSERT", "{}", "2026-02-01T00:00:00").unwrap();
        let count = count_changes_since(&conn, "2026-01-15T00:00:00").unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_latest_timestamp() {
        let conn = setup_db();
        assert!(get_latest_timestamp(&conn).unwrap().is_none());
        record_change_at(&conn, "invoices", "inv-001", "INSERT", "{}", "2026-03-01T10:00:00").unwrap();
        record_change_at(&conn, "invoices", "inv-002", "INSERT", "{}", "2026-03-05T15:30:00").unwrap();
        let latest = get_latest_timestamp(&conn).unwrap();
        assert_eq!(latest, Some("2026-03-05T15:30:00".to_string()));
    }

    #[test]
    fn test_get_changes_for_table_since() {
        let conn = setup_db();
        record_change_at(&conn, "invoices", "inv-001", "INSERT", "{}", "2026-01-01T00:00:00").unwrap();
        record_change_at(&conn, "clients", "cli-001", "INSERT", "{}", "2026-01-02T00:00:00").unwrap();
        record_change_at(&conn, "invoices", "inv-002", "UPDATE", "{}", "2026-01-03T00:00:00").unwrap();

        let invoice_changes = get_changes_for_table_since(&conn, "invoices", "2025-01-01T00:00:00").unwrap();
        assert_eq!(invoice_changes.len(), 2);
        assert!(invoice_changes.iter().all(|c| c.table_name == "invoices"));

        let client_changes = get_changes_for_table_since(&conn, "clients", "2025-01-01T00:00:00").unwrap();
        assert_eq!(client_changes.len(), 1);
    }
}
