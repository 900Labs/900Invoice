#![allow(dead_code)]
//! Gap-free invoice numbering service for 900Invoice.
//!
//! Format: `{PREFIX}{SEP}{YEAR}{SEP}{PADDED_NUMBER}` e.g. `INV-2026-0001`
//!
//! The default public generator wraps the counter increment in a
//! BEGIN IMMEDIATE transaction. Callers that already own a write transaction
//! can use the transaction-scoped helper so invoice status changes and
//! sequence advancement commit or roll back together.

use chrono::Datelike;
use rusqlite::Connection;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate the next invoice number for the given sequence.
///
/// Delegates to the db::queries::sequences layer which uses an exclusive
/// SQLite transaction to ensure gap-free numbering.
///
/// Returns the formatted invoice number string (e.g. "INV-2026-0001").
pub fn generate_invoice_number(conn: &Connection, sequence_name: &str) -> Result<String, String> {
    inner_generate(conn, sequence_name)
}

/// Generate the next invoice number inside an existing write transaction.
///
/// This advances the sequence counter without opening or committing a nested
/// transaction, allowing callers to roll the sequence back with related writes.
pub fn generate_invoice_number_in_transaction(
    conn: &Connection,
    sequence_name: &str,
) -> Result<String, String> {
    try_generate_and_advance(conn, sequence_name)
}

/// Preview the next invoice number without consuming it.
/// Does not advance the counter.
pub fn preview_next_number(conn: &Connection, sequence_name: &str) -> Result<String, String> {
    let seq = load_sequence(conn, sequence_name)?;
    let current_year = chrono::Local::now().year();

    let (effective_number, effective_year) =
        if seq.year_reset && seq.last_year.map(|y| y != current_year).unwrap_or(false) {
            (1i64, current_year)
        } else {
            (seq.next_number, current_year)
        };

    Ok(format_number(
        &seq.prefix,
        &seq.separator,
        seq.include_year,
        effective_year,
        effective_number,
        seq.pad_digits,
    ))
}

// ---------------------------------------------------------------------------
// Internal implementation
// ---------------------------------------------------------------------------

struct SequenceRow {
    prefix: String,
    separator: String,
    include_year: bool,
    pad_digits: u32,
    year_reset: bool,
    last_year: Option<i32>,
    next_number: i64,
}

fn load_sequence(conn: &Connection, sequence_name: &str) -> Result<SequenceRow, String> {
    conn.query_row(
        "SELECT prefix, separator, include_year, pad_digits, year_reset, last_year, next_number \
         FROM invoice_sequences WHERE sequence_name = ?1",
        rusqlite::params![sequence_name],
        |row| {
            Ok(SequenceRow {
                prefix: row.get(0)?,
                separator: row.get(1)?,
                include_year: row.get::<_, i32>(2)? != 0,
                pad_digits: row.get::<_, i32>(3)? as u32,
                year_reset: row.get::<_, i32>(4)? != 0,
                last_year: row.get(5)?,
                next_number: row.get(6)?,
            })
        },
    )
    .map_err(|e| format!("Sequence '{}' not found: {}", sequence_name, e))
}

fn format_number(
    prefix: &str,
    separator: &str,
    include_year: bool,
    year: i32,
    number: i64,
    pad_digits: u32,
) -> String {
    let padded = format!("{:0>width$}", number, width = pad_digits as usize);
    if include_year {
        format!("{}{}{}{}{}", prefix, separator, year, separator, padded)
    } else {
        format!("{}{}{}", prefix, separator, padded)
    }
}

/// Internal implementation: BEGIN IMMEDIATE transaction, read, increment, commit.
fn inner_generate(conn: &Connection, sequence_name: &str) -> Result<String, String> {
    conn.execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| format!("Failed to begin immediate transaction: {e}"))?;

    let result = try_generate_and_advance(conn, sequence_name);

    match &result {
        Ok(_) => {
            conn.execute_batch("COMMIT").map_err(|e| {
                let _ = conn.execute_batch("ROLLBACK");
                format!("Failed to commit sequence transaction: {e}")
            })?;
        }
        Err(_) => {
            let _ = conn.execute_batch("ROLLBACK");
        }
    }

    result
}

fn try_generate_and_advance(conn: &Connection, sequence_name: &str) -> Result<String, String> {
    let seq = load_sequence(conn, sequence_name)?;
    let current_year = chrono::Local::now().year();

    let year_changed = seq.year_reset && seq.last_year.map(|y| y != current_year).unwrap_or(false);

    let new_number = if year_changed { 1i64 } else { seq.next_number };

    let formatted = format_number(
        &seq.prefix,
        &seq.separator,
        seq.include_year,
        current_year,
        new_number,
        seq.pad_digits,
    );

    conn.execute(
        "UPDATE invoice_sequences \
         SET next_number = ?1, last_year = ?2 \
         WHERE sequence_name = ?3",
        rusqlite::params![new_number + 1, current_year, sequence_name],
    )
    .map_err(|e| format!("Failed to advance sequence '{}': {}", sequence_name, e))?;

    Ok(formatted)
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
            "CREATE TABLE IF NOT EXISTS invoice_sequences (
                sequence_name TEXT PRIMARY KEY,
                prefix TEXT NOT NULL DEFAULT 'INV',
                separator TEXT NOT NULL DEFAULT '-',
                include_year INTEGER NOT NULL DEFAULT 1,
                pad_digits INTEGER NOT NULL DEFAULT 4,
                year_reset INTEGER NOT NULL DEFAULT 1,
                last_year INTEGER,
                last_month INTEGER,
                next_number INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            INSERT OR IGNORE INTO invoice_sequences (sequence_name, prefix, next_number)
            VALUES ('default', 'INV', 1);",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_sequential_numbering() {
        let conn = setup_db();
        let y = chrono::Local::now().year();
        let n1 = generate_invoice_number(&conn, "default").unwrap();
        let n2 = generate_invoice_number(&conn, "default").unwrap();
        let n3 = generate_invoice_number(&conn, "default").unwrap();
        assert_eq!(n1, format!("INV-{y}-0001"));
        assert_eq!(n2, format!("INV-{y}-0002"));
        assert_eq!(n3, format!("INV-{y}-0003"));
    }

    #[test]
    fn test_preview_does_not_advance() {
        let conn = setup_db();
        let y = chrono::Local::now().year();
        let p1 = preview_next_number(&conn, "default").unwrap();
        let p2 = preview_next_number(&conn, "default").unwrap();
        assert_eq!(p1, format!("INV-{y}-0001"));
        assert_eq!(p2, format!("INV-{y}-0001"));
        // Now actually generate
        let real = generate_invoice_number(&conn, "default").unwrap();
        assert_eq!(real, format!("INV-{y}-0001"));
        // Preview now shows 0002
        let p3 = preview_next_number(&conn, "default").unwrap();
        assert_eq!(p3, format!("INV-{y}-0002"));
    }

    #[test]
    fn test_year_reset() {
        let conn = setup_db();
        let prev_year = chrono::Local::now().year() - 1;
        conn.execute(
            "UPDATE invoice_sequences SET last_year = ?1, next_number = 99 WHERE sequence_name = 'default'",
            rusqlite::params![prev_year],
        )
        .unwrap();
        let y = chrono::Local::now().year();
        let num = generate_invoice_number(&conn, "default").unwrap();
        assert_eq!(num, format!("INV-{y}-0001"));
        let num2 = generate_invoice_number(&conn, "default").unwrap();
        assert_eq!(num2, format!("INV-{y}-0002"));
    }

    #[test]
    fn test_not_found_returns_error() {
        let conn = setup_db();
        let result = generate_invoice_number(&conn, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn transaction_scoped_generation_rolls_back_with_caller() {
        let conn = setup_db();
        let y = chrono::Local::now().year();

        conn.execute_batch("BEGIN IMMEDIATE").unwrap();
        let generated = generate_invoice_number_in_transaction(&conn, "default").unwrap();
        assert_eq!(generated, format!("INV-{y}-0001"));
        conn.execute_batch("ROLLBACK").unwrap();

        let preview = preview_next_number(&conn, "default").unwrap();
        assert_eq!(preview, format!("INV-{y}-0001"));
    }
}
