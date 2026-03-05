pub mod migrations;
pub mod queries;

use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_database(app_data_dir: &Path) -> Result<Connection> {
    let db_path = app_data_dir.join("900invoice.db");
    let conn = Connection::open(&db_path)?;

    // Enable WAL mode for better concurrent read performance
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;

    // Enforce foreign key constraints
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    // Run all migrations
    migrations::run_migrations(&conn)?;

    Ok(conn)
}
