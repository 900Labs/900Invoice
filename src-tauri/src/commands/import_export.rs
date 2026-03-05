use crate::db;
use crate::models::client::CreateClient;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

/// Parse a simple CSV line, handling basic quoted fields.
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current = String::new();
            }
            c => current.push(c),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

/// Escape a field for CSV output.
fn csv_escape(s: &str) -> String {
    let sanitized = if matches!(s.chars().next(), Some('=' | '+' | '-' | '@' | '\t' | '\r')) {
        format!("'{}", s)
    } else {
        s.to_string()
    };

    if sanitized.contains(',') || sanitized.contains('"') || sanitized.contains('\n') {
        format!("\"{}\"", sanitized.replace('"', "\"\""))
    } else {
        sanitized
    }
}

/// Import clients from a CSV string.
/// Expected header: name,email,phone,address,city,country,country_code,currency_code,payment_terms_days
#[tauri::command]
pub fn import_clients_csv(
    db: State<'_, DbConn>,
    csv_content: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut lines = csv_content.lines();
    // Skip header
    let _header = lines.next();

    let mut imported = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = parse_csv_line(line);

        let name = fields.first().cloned().unwrap_or_default();
        if name.is_empty() {
            errors.push(format!("Row {}: missing name", i + 2));
            continue;
        }

        let client = CreateClient {
            name,
            email: fields.get(1).cloned().filter(|s| !s.is_empty()),
            phone: fields.get(2).cloned().filter(|s| !s.is_empty()),
            address: fields.get(3).cloned().filter(|s| !s.is_empty()),
            city: fields.get(4).cloned().filter(|s| !s.is_empty()),
            country: fields.get(5).cloned().filter(|s| !s.is_empty()),
            country_code: fields.get(6).cloned().filter(|s| !s.is_empty()),
            tax_id: None,
            currency_code: fields.get(7).cloned().filter(|s| !s.is_empty()),
            payment_terms_days: fields.get(8).and_then(|s| s.parse::<i32>().ok()),
            notes: None,
        };

        match db::queries::clients::insert(&conn, &client) {
            Ok(_) => imported += 1,
            Err(e) => errors.push(format!("Row {}: {}", i + 2, e)),
        }
    }

    Ok(serde_json::json!({
        "imported": imported,
        "errors": errors,
    }))
}

/// Export all clients as a CSV string.
#[tauri::command]
pub fn export_clients_csv(db: State<'_, DbConn>) -> Result<String, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let clients = db::queries::clients::list_all(&conn).map_err(|e| e.to_string())?;

    let mut out =
        String::from("name,email,phone,address,city,country,country_code,tax_id,currency_code,payment_terms_days,notes\n");
    for c in &clients {
        let row = vec![
            csv_escape(&c.name),
            csv_escape(&c.email),
            csv_escape(&c.phone),
            csv_escape(&c.address),
            csv_escape(&c.city),
            csv_escape(&c.country),
            csv_escape(&c.country_code),
            csv_escape(&c.tax_id),
            csv_escape(&c.currency_code),
            c.payment_terms_days.to_string(),
            csv_escape(&c.notes),
        ];
        out.push_str(&row.join(","));
        out.push('\n');
    }
    Ok(out)
}

/// Export all invoices as a CSV string.
#[tauri::command]
pub fn export_invoices_csv(db: State<'_, DbConn>) -> Result<String, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let invoices = db::queries::invoices::list_all(&conn).map_err(|e| e.to_string())?;

    let mut out = String::from(
        "invoice_number,client_id,status,currency_code,subtotal,discount,tax_amount,total,amount_paid,issue_date,due_date,created_at\n",
    );

    for inv in &invoices {
        let fmt = |v: i64| format!("{:.2}", v as f64 / 100.0);
        let row = vec![
            csv_escape(inv.invoice_number.as_deref().unwrap_or("")),
            csv_escape(&inv.client_id),
            csv_escape(&inv.status),
            csv_escape(&inv.currency_code),
            fmt(inv.subtotal_minor),
            fmt(inv.discount_minor),
            fmt(inv.tax_amount_minor),
            fmt(inv.total_minor),
            fmt(inv.amount_paid_minor),
            csv_escape(&inv.issue_date),
            csv_escape(&inv.due_date),
            csv_escape(&inv.created_at),
        ];
        out.push_str(&row.join(","));
        out.push('\n');
    }
    Ok(out)
}

/// Backup the entire database as a base64-encoded binary blob.
/// The frontend is responsible for writing this to a file.
#[tauri::command]
pub fn backup_database(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    // Export every table as JSON
    let clients = db::queries::clients::list_all(&conn).map_err(|e| e.to_string())?;
    let invoices = db::queries::invoices::list_all(&conn).map_err(|e| e.to_string())?;
    let products = db::queries::products::list_all(&conn).map_err(|e| e.to_string())?;
    let tax_rates = db::queries::taxes::list_all(&conn).map_err(|e| e.to_string())?;
    let payments: Vec<_> = {
        let mut all = Vec::new();
        for inv in &invoices {
            let p = db::queries::payments::list_for_invoice(&conn, &inv.id)
                .map_err(|e| e.to_string())?;
            all.extend(p);
        }
        all
    };
    let settings = db::queries::settings::get_all(&conn).map_err(|e| e.to_string())?;
    let business = db::queries::business::get(&conn).map_err(|e| e.to_string())?;
    let recurring = db::queries::recurring::list_all(&conn).map_err(|e| e.to_string())?;

    let backup = serde_json::json!({
        "version": "1.0",
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "business": business,
        "clients": clients,
        "invoices": invoices,
        "products": products,
        "tax_rates": tax_rates,
        "payments": payments,
        "settings": settings,
        "recurring": recurring,
    });

    Ok(backup)
}

/// Restore from a backup JSON (as returned by backup_database).
/// WARNING: This is additive — it does not drop existing data.
#[tauri::command]
pub fn restore_database(
    db: State<'_, DbConn>,
    backup: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut restored_counts = std::collections::HashMap::<String, usize>::new();

    // Restore clients
    if let Some(clients) = backup.get("clients").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for client_val in clients {
            let client: crate::models::client::Client =
                serde_json::from_value(client_val.clone()).map_err(|e| e.to_string())?;
            // Use INSERT OR IGNORE to avoid duplicates
            let result = conn.execute(
                "INSERT OR IGNORE INTO clients
                 (id, name, email, phone, address, city, country, country_code,
                  tax_id, currency_code, payment_terms_days, notes, created_at, updated_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
                rusqlite::params![
                    client.id,
                    client.name,
                    client.email,
                    client.phone,
                    client.address,
                    client.city,
                    client.country,
                    client.country_code,
                    client.tax_id,
                    client.currency_code,
                    client.payment_terms_days,
                    client.notes,
                    client.created_at,
                    client.updated_at,
                ],
            );
            if result.is_ok() {
                count += 1;
            }
        }
        restored_counts.insert("clients".to_string(), count);
    }

    // Restore products
    if let Some(products) = backup.get("products").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for product_val in products {
            let product: crate::models::product::Product =
                serde_json::from_value(product_val.clone()).map_err(|e| e.to_string())?;
            let result = conn.execute(
                "INSERT OR IGNORE INTO products
                 (id, name, description, default_price_minor, default_currency,
                  default_tax_rate_bps, unit, is_active, created_at, updated_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                rusqlite::params![
                    product.id,
                    product.name,
                    product.description,
                    product.default_price_minor,
                    product.default_currency,
                    product.default_tax_rate_bps,
                    product.unit,
                    if product.is_active { 1i32 } else { 0i32 },
                    product.created_at,
                    product.updated_at,
                ],
            );
            if result.is_ok() {
                count += 1;
            }
        }
        restored_counts.insert("products".to_string(), count);
    }

    Ok(serde_json::json!({
        "status": "ok",
        "restored": restored_counts,
    }))
}

#[cfg(test)]
mod tests {
    use super::{csv_escape, parse_csv_line};

    #[test]
    fn csv_escape_prefixes_formula_like_values() {
        assert_eq!(csv_escape("=2+2"), "'=2+2");
        assert_eq!(csv_escape("+cmd"), "'+cmd");
        assert_eq!(csv_escape("-sum"), "'-sum");
        assert_eq!(csv_escape("@risk"), "'@risk");
        assert_eq!(csv_escape("\tformula"), "'\tformula");
        assert_eq!(csv_escape("\rformula"), "'\rformula");
    }

    #[test]
    fn csv_escape_quotes_when_needed_after_sanitization() {
        assert_eq!(csv_escape("=SUM(A1,B1)"), "\"'=SUM(A1,B1)\"");
        assert_eq!(csv_escape("hello,world"), "\"hello,world\"");
        assert_eq!(csv_escape("say \"hello\""), "\"say \"\"hello\"\"\"");
    }

    #[test]
    fn csv_escape_leaves_safe_values_unchanged() {
        assert_eq!(csv_escape("normal-value"), "normal-value");
        assert_eq!(csv_escape("12345"), "12345");
    }

    #[test]
    fn parse_csv_line_handles_basic_quoted_commas() {
        let fields = parse_csv_line("name,\"addr, line\",city");
        assert_eq!(fields, vec!["name", "addr, line", "city"]);
    }
}
