use rusqlite::{params, Connection, Result};
use chrono::Datelike;

/// Atomically fetch-and-increment the sequence counter.
/// Returns the formatted invoice number, e.g. "INV-2026-0001".
pub fn get_next_number(conn: &Connection, sequence_name: &str) -> Result<String> {
    // Use an immediate transaction so no other write can interleave
    conn.execute_batch("BEGIN IMMEDIATE")?;

    let result: Result<String> = (|| {
        let mut stmt = conn.prepare(
            "SELECT prefix, separator, include_year, pad_digits,
                    year_reset, last_year, next_number
             FROM invoice_sequences WHERE sequence_name=?1",
        )?;

        let (prefix, separator, include_year, pad_digits, year_reset, last_year, mut next_number): (
            String, String, i32, i32, i32, Option<i32>, i32,
        ) = stmt.query_row(params![sequence_name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, Option<i32>>(5)?,
                row.get::<_, i32>(6)?,
            ))
        })?;

        let current_year = chrono::Local::now().year();

        // Reset counter if year changed and year_reset is enabled
        if year_reset != 0 {
            if let Some(ly) = last_year {
                if ly != current_year {
                    next_number = 1;
                }
            }
        }

        // Build the formatted number
        let padded = format!("{:0>width$}", next_number, width = pad_digits as usize);
        let number = if include_year != 0 {
            format!("{}{}{}{}{}", prefix, separator, current_year, separator, padded)
        } else {
            format!("{}{}{}", prefix, separator, padded)
        };

        // Increment and persist
        conn.execute(
            "UPDATE invoice_sequences
             SET next_number=?1, last_year=?2
             WHERE sequence_name=?3",
            params![next_number + 1, current_year, sequence_name],
        )?;

        Ok(number)
    })();

    match result {
        Ok(n) => {
            conn.execute_batch("COMMIT")?;
            Ok(n)
        }
        Err(e) => {
            let _ = conn.execute_batch("ROLLBACK");
            Err(e)
        }
    }
}
