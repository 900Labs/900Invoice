use crate::db;
use crate::models::client::CreateClient;
use crate::models::exchange_rate::ExchangeRate;
use crate::models::invoice::Invoice;
use crate::models::line_item::LineItem;
use crate::models::payment::Payment;
use crate::models::product::{CreateProduct, Product, UpdateProduct};
use crate::models::recurring::RecurringInvoice;
use crate::models::tax::{InvoiceTax, TaxRate};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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

fn currency_decimals(currency_code: &str) -> u32 {
    match currency_code.to_ascii_uppercase().as_str() {
        "UGX" | "XOF" | "XAF" => 0,
        _ => 2,
    }
}

fn currency_multiplier(currency_code: &str) -> i64 {
    10_i64.pow(currency_decimals(currency_code))
}

fn parse_price_minor(value: &str, currency_code: &str) -> Result<i64, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(0);
    }

    let parsed = trimmed
        .parse::<f64>()
        .map_err(|_| format!("invalid default_price '{}'", value))?;
    if !parsed.is_finite() || parsed < 0.0 {
        return Err(format!("invalid default_price '{}'", value));
    }

    Ok((parsed * currency_multiplier(currency_code) as f64).round() as i64)
}

fn format_price_major(amount_minor: i64, currency_code: &str) -> String {
    let decimals = currency_decimals(currency_code);
    let divisor = currency_multiplier(currency_code);
    if decimals == 0 {
        amount_minor.to_string()
    } else {
        format!(
            "{:.*}",
            decimals as usize,
            amount_minor as f64 / divisor as f64
        )
    }
}

fn parse_bool_field(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "active" => Some(true),
        "false" | "0" | "no" | "n" | "inactive" => Some(false),
        _ => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InvoiceSequenceBackup {
    sequence_name: String,
    prefix: String,
    separator: String,
    include_year: bool,
    pad_digits: i32,
    year_reset: bool,
    last_year: Option<i32>,
    last_month: Option<i32>,
    next_number: i32,
    created_at: String,
}

fn bool_to_int(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}

fn list_products_for_backup(conn: &Connection) -> Result<Vec<Product>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, description, default_price_minor, default_currency,
                    default_tax_rate_bps, unit, is_active, created_at, updated_at
             FROM products ORDER BY name ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn list_tax_rates_for_backup(conn: &Connection) -> Result<Vec<TaxRate>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, display_name, rate_bps, country_code,
                    is_default, is_withholding, is_inclusive, is_active, created_at, updated_at
             FROM tax_rates ORDER BY name ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn list_line_items_for_backup(conn: &Connection) -> Result<Vec<LineItem>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, invoice_id, product_id, description, quantity, unit_price_minor,
                    tax_rate_bps, discount_bps, line_total_minor, sort_order, created_at
             FROM invoice_line_items ORDER BY invoice_id ASC, sort_order ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(LineItem {
                id: row.get(0)?,
                invoice_id: row.get(1)?,
                product_id: row.get(2)?,
                description: row.get(3)?,
                quantity: row.get(4)?,
                unit_price_minor: row.get(5)?,
                tax_rate_bps: row.get(6)?,
                discount_bps: row.get(7)?,
                line_total_minor: row.get(8)?,
                sort_order: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn list_invoice_taxes_for_backup(conn: &Connection) -> Result<Vec<InvoiceTax>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, invoice_id, tax_rate_id, tax_name, tax_rate_bps,
                    tax_amount_minor, is_withholding, created_at
             FROM invoice_taxes ORDER BY invoice_id ASC, created_at ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn list_exchange_rates_for_backup(conn: &Connection) -> Result<Vec<ExchangeRate>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT base_currency, target_currency, rate, fetched_at, valid_date
             FROM exchange_rates ORDER BY base_currency ASC, target_currency ASC, valid_date ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ExchangeRate {
                base_currency: row.get(0)?,
                target_currency: row.get(1)?,
                rate: row.get(2)?,
                fetched_at: row.get(3)?,
                valid_date: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn list_invoice_sequences_for_backup(
    conn: &Connection,
) -> Result<Vec<InvoiceSequenceBackup>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT sequence_name, prefix, separator, include_year, pad_digits,
                    year_reset, last_year, last_month, next_number, created_at
             FROM invoice_sequences ORDER BY sequence_name ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            let include_year: i32 = row.get(3)?;
            let year_reset: i32 = row.get(5)?;
            Ok(InvoiceSequenceBackup {
                sequence_name: row.get(0)?,
                prefix: row.get(1)?,
                separator: row.get(2)?,
                include_year: include_year != 0,
                pad_digits: row.get(4)?,
                year_reset: year_reset != 0,
                last_year: row.get(6)?,
                last_month: row.get(7)?,
                next_number: row.get(8)?,
                created_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn import_products_csv_content(conn: &Connection, csv_content: &str) -> Result<Value, String> {
    let mut lines = csv_content.lines();
    let _header = lines.next();

    let mut imported = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let fields = parse_csv_line(line);
        let row = i + 2;
        let name = fields.first().cloned().unwrap_or_default();
        if name.is_empty() {
            errors.push(format!("Row {}: missing name", row));
            continue;
        }

        let default_currency = fields
            .get(3)
            .map(|s| s.trim().to_ascii_uppercase())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "USD".to_string());

        let default_price_minor = match parse_price_minor(
            fields.get(2).map(String::as_str).unwrap_or(""),
            &default_currency,
        ) {
            Ok(amount) => amount,
            Err(e) => {
                errors.push(format!("Row {}: {}", row, e));
                continue;
            }
        };

        let default_tax_rate_bps = match fields.get(4).filter(|s| !s.trim().is_empty()) {
            Some(value) => match value.trim().parse::<i32>() {
                Ok(rate) if rate >= 0 => rate,
                _ => {
                    errors.push(format!(
                        "Row {}: invalid default_tax_rate_bps '{}'",
                        row, value
                    ));
                    continue;
                }
            },
            None => 0,
        };

        let is_active = match fields.get(6).filter(|s| !s.trim().is_empty()) {
            Some(value) => match parse_bool_field(value) {
                Some(parsed) => parsed,
                None => {
                    errors.push(format!("Row {}: invalid is_active '{}'", row, value));
                    continue;
                }
            },
            None => true,
        };

        let product = CreateProduct {
            name,
            description: fields.get(1).cloned().filter(|s| !s.is_empty()),
            default_price_minor: Some(default_price_minor),
            default_currency: Some(default_currency),
            default_tax_rate_bps: Some(default_tax_rate_bps),
            unit: fields.get(5).cloned().filter(|s| !s.is_empty()),
        };

        match db::queries::products::insert(conn, &product) {
            Ok(created) => {
                if !is_active {
                    let update = UpdateProduct {
                        name: None,
                        description: None,
                        default_price_minor: None,
                        default_currency: None,
                        default_tax_rate_bps: None,
                        unit: None,
                        is_active: Some(false),
                    };
                    if let Err(e) = db::queries::products::update(conn, &created.id, &update) {
                        errors.push(format!("Row {}: {}", row, e));
                        continue;
                    }
                }
                imported += 1;
            }
            Err(e) => errors.push(format!("Row {}: {}", row, e)),
        }
    }

    Ok(serde_json::json!({
        "imported": imported,
        "errors": errors,
    }))
}

fn export_products_csv_content(conn: &Connection) -> Result<String, String> {
    let products = list_products_for_backup(conn)?;
    let mut out = String::from(
        "name,description,default_price,default_currency,default_tax_rate_bps,unit,is_active\n",
    );

    for product in &products {
        let row = [
            csv_escape(&product.name),
            csv_escape(&product.description),
            format_price_major(product.default_price_minor, &product.default_currency),
            csv_escape(&product.default_currency),
            product.default_tax_rate_bps.to_string(),
            csv_escape(&product.unit),
            product.is_active.to_string(),
        ];
        out.push_str(&row.join(","));
        out.push('\n');
    }

    Ok(out)
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

/// Import products from a CSV string.
/// Expected header: name,description,default_price,default_currency,default_tax_rate_bps,unit,is_active
#[tauri::command]
pub fn import_products_csv(
    db: State<'_, DbConn>,
    csv_content: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    import_products_csv_content(&conn, &csv_content)
}

/// Export all products as a CSV string.
#[tauri::command]
pub fn export_products_csv(db: State<'_, DbConn>) -> Result<String, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    export_products_csv_content(&conn)
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
    let products = list_products_for_backup(&conn)?;
    let tax_rates = list_tax_rates_for_backup(&conn)?;
    let line_items = list_line_items_for_backup(&conn)?;
    let invoice_taxes = list_invoice_taxes_for_backup(&conn)?;
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
    let exchange_rates = list_exchange_rates_for_backup(&conn)?;
    let invoice_sequences = list_invoice_sequences_for_backup(&conn)?;

    let backup = serde_json::json!({
        "version": "1.0",
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "business": business,
        "clients": clients,
        "invoices": invoices,
        "line_items": line_items,
        "invoice_taxes": invoice_taxes,
        "products": products,
        "tax_rates": tax_rates,
        "payments": payments,
        "settings": settings,
        "recurring": recurring,
        "exchange_rates": exchange_rates,
        "invoice_sequences": invoice_sequences,
    });

    Ok(backup)
}

fn restore_backup(conn: &Connection, backup: Value) -> Result<Value, String> {
    if !backup.is_object() {
        return Err("VALIDATION: backup payload must be a JSON object".to_string());
    }

    let mut restored_counts = HashMap::<String, usize>::new();

    if let Some(business_val) = backup.get("business").filter(|v| !v.is_null()) {
        let business: crate::models::business::BusinessProfile =
            serde_json::from_value(business_val.clone()).map_err(|e| e.to_string())?;
        let inserted = conn
            .execute(
                "INSERT OR IGNORE INTO business_profiles
                 (id, name, address, city, country, country_code, phone, email, website,
                  tax_id, logo_path, default_currency, default_payment_terms_days,
                  bank_name, bank_account_number, bank_routing_number,
                  mobile_money_number, mobile_money_provider, created_at, updated_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20)",
                rusqlite::params![
                    business.id,
                    business.name,
                    business.address,
                    business.city,
                    business.country,
                    business.country_code,
                    business.phone,
                    business.email,
                    business.website,
                    business.tax_id,
                    business.logo_path,
                    business.default_currency,
                    business.default_payment_terms_days,
                    business.bank_name,
                    business.bank_account_number,
                    business.bank_routing_number,
                    business.mobile_money_number,
                    business.mobile_money_provider,
                    business.created_at,
                    business.updated_at,
                ],
            )
            .map_err(|e| e.to_string())?;
        restored_counts.insert("business".to_string(), inserted);
    }

    if let Some(settings) = backup.get("settings").and_then(|v| v.as_object()) {
        let mut count = 0usize;
        for (key, value) in settings {
            let stored = value
                .as_str()
                .map(ToString::to_string)
                .unwrap_or_else(|| value.to_string());
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                    rusqlite::params![key, stored],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("settings".to_string(), count);
    }

    if let Some(tax_rates) = backup.get("tax_rates").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for tax_val in tax_rates {
            let tax_rate: TaxRate =
                serde_json::from_value(tax_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO tax_rates
                     (id, name, display_name, rate_bps, country_code, is_default,
                      is_withholding, is_inclusive, is_active, created_at, updated_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                    rusqlite::params![
                        tax_rate.id,
                        tax_rate.name,
                        tax_rate.display_name,
                        tax_rate.rate_bps,
                        tax_rate.country_code,
                        bool_to_int(tax_rate.is_default),
                        bool_to_int(tax_rate.is_withholding),
                        bool_to_int(tax_rate.is_inclusive),
                        bool_to_int(tax_rate.is_active),
                        tax_rate.created_at,
                        tax_rate.updated_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("tax_rates".to_string(), count);
    }

    // Restore clients
    if let Some(clients) = backup.get("clients").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for client_val in clients {
            let client: crate::models::client::Client =
                serde_json::from_value(client_val.clone()).map_err(|e| e.to_string())?;
            // Use INSERT OR IGNORE to avoid duplicates
            count += conn
                .execute(
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
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("clients".to_string(), count);
    }

    // Restore products
    if let Some(products) = backup.get("products").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for product_val in products {
            let product: Product =
                serde_json::from_value(product_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
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
                        bool_to_int(product.is_active),
                        product.created_at,
                        product.updated_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("products".to_string(), count);
    }

    if let Some(invoices) = backup.get("invoices").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for invoice_val in invoices {
            let invoice: Invoice =
                serde_json::from_value(invoice_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO invoices
                     (id, invoice_number, client_id, status, currency_code, subtotal_minor,
                      discount_minor, tax_amount_minor, total_minor, amount_paid_minor,
                      exchange_rate_to_usd, exchange_rate_date, issue_date, due_date,
                      uses_inclusive_taxes, notes, terms, footer, created_at, updated_at,
                      finalized_at, sent_at, paid_at, voided_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24)",
                    rusqlite::params![
                        invoice.id,
                        invoice.invoice_number,
                        invoice.client_id,
                        invoice.status,
                        invoice.currency_code,
                        invoice.subtotal_minor,
                        invoice.discount_minor,
                        invoice.tax_amount_minor,
                        invoice.total_minor,
                        invoice.amount_paid_minor,
                        invoice.exchange_rate_to_usd,
                        invoice.exchange_rate_date,
                        invoice.issue_date,
                        invoice.due_date,
                        bool_to_int(invoice.uses_inclusive_taxes),
                        invoice.notes,
                        invoice.terms,
                        invoice.footer,
                        invoice.created_at,
                        invoice.updated_at,
                        invoice.finalized_at,
                        invoice.sent_at,
                        invoice.paid_at,
                        invoice.voided_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("invoices".to_string(), count);
    }

    if let Some(line_items) = backup.get("line_items").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for item_val in line_items {
            let item: LineItem =
                serde_json::from_value(item_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO invoice_line_items
                     (id, invoice_id, product_id, description, quantity, unit_price_minor,
                      tax_rate_bps, discount_bps, line_total_minor, sort_order, created_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                    rusqlite::params![
                        item.id,
                        item.invoice_id,
                        item.product_id,
                        item.description,
                        item.quantity,
                        item.unit_price_minor,
                        item.tax_rate_bps,
                        item.discount_bps,
                        item.line_total_minor,
                        item.sort_order,
                        item.created_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("line_items".to_string(), count);
    }

    if let Some(invoice_taxes) = backup.get("invoice_taxes").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for tax_val in invoice_taxes {
            let tax: InvoiceTax =
                serde_json::from_value(tax_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO invoice_taxes
                     (id, invoice_id, tax_rate_id, tax_name, tax_rate_bps,
                      tax_amount_minor, is_withholding, created_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
                    rusqlite::params![
                        tax.id,
                        tax.invoice_id,
                        tax.tax_rate_id,
                        tax.tax_name,
                        tax.tax_rate_bps,
                        tax.tax_amount_minor,
                        bool_to_int(tax.is_withholding),
                        tax.created_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("invoice_taxes".to_string(), count);
    }

    if let Some(payments) = backup.get("payments").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for payment_val in payments {
            let payment: Payment =
                serde_json::from_value(payment_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO payments
                     (id, invoice_id, amount_minor, currency_code, payment_method,
                      payment_reference, notes, paid_at, created_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                    rusqlite::params![
                        payment.id,
                        payment.invoice_id,
                        payment.amount_minor,
                        payment.currency_code,
                        payment.payment_method,
                        payment.payment_reference,
                        payment.notes,
                        payment.paid_at,
                        payment.created_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("payments".to_string(), count);
    }

    if let Some(recurring) = backup.get("recurring").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for recurring_val in recurring {
            let recurring_invoice: RecurringInvoice =
                serde_json::from_value(recurring_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO recurring_invoices
                     (id, client_id, template_invoice_id, frequency, next_generation_date,
                      end_date, auto_send, last_generated, status, created_at, updated_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                    rusqlite::params![
                        recurring_invoice.id,
                        recurring_invoice.client_id,
                        recurring_invoice.template_invoice_id,
                        recurring_invoice.frequency,
                        recurring_invoice.next_generation_date,
                        recurring_invoice.end_date,
                        bool_to_int(recurring_invoice.auto_send),
                        recurring_invoice.last_generated,
                        recurring_invoice.status,
                        recurring_invoice.created_at,
                        recurring_invoice.updated_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("recurring".to_string(), count);
    }

    if let Some(exchange_rates) = backup.get("exchange_rates").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for rate_val in exchange_rates {
            let rate: ExchangeRate =
                serde_json::from_value(rate_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO exchange_rates
                     (base_currency, target_currency, rate, fetched_at, valid_date)
                     VALUES (?1,?2,?3,?4,?5)",
                    rusqlite::params![
                        rate.base_currency,
                        rate.target_currency,
                        rate.rate,
                        rate.fetched_at,
                        rate.valid_date,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("exchange_rates".to_string(), count);
    }

    if let Some(sequences) = backup.get("invoice_sequences").and_then(|v| v.as_array()) {
        let mut count = 0usize;
        for sequence_val in sequences {
            let sequence: InvoiceSequenceBackup =
                serde_json::from_value(sequence_val.clone()).map_err(|e| e.to_string())?;
            count += conn
                .execute(
                    "INSERT OR IGNORE INTO invoice_sequences
                     (sequence_name, prefix, separator, include_year, pad_digits,
                      year_reset, last_year, last_month, next_number, created_at)
                     VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                    rusqlite::params![
                        sequence.sequence_name,
                        sequence.prefix,
                        sequence.separator,
                        bool_to_int(sequence.include_year),
                        sequence.pad_digits,
                        bool_to_int(sequence.year_reset),
                        sequence.last_year,
                        sequence.last_month,
                        sequence.next_number,
                        sequence.created_at,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }
        restored_counts.insert("invoice_sequences".to_string(), count);
    }

    Ok(serde_json::json!({
        "status": "ok",
        "restored": restored_counts,
    }))
}

/// Restore from a backup JSON (as returned by backup_database).
/// WARNING: This is additive — it does not drop existing data.
#[tauri::command]
pub fn restore_database(db: State<'_, DbConn>, backup: Value) -> Result<Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    restore_backup(&conn, backup)
}

#[cfg(test)]
mod tests {
    use super::{
        csv_escape, export_products_csv_content, import_products_csv_content, parse_csv_line,
        restore_backup,
    };
    use crate::db::migrations;
    use crate::models::product::CreateProduct;
    use rusqlite::Connection;
    use serde_json::json;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        migrations::run_migrations(&conn).expect("migrations");
        conn
    }

    fn table_count(conn: &Connection, table: &str) -> i64 {
        let sql = format!("SELECT COUNT(*) FROM {}", table);
        conn.query_row(&sql, [], |row| row.get(0)).expect("count")
    }

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

    #[test]
    fn import_products_csv_content_imports_products_and_active_flag() {
        let conn = test_conn();
        let csv = concat!(
            "name,description,default_price,default_currency,default_tax_rate_bps,unit,is_active\n",
            "Consulting,\"Senior advisory\",125.50,USD,1600,hour,true\n",
            "Legacy license,,2500,UGX,0,seat,false\n",
            ",Missing name,10,USD,0,unit,true\n",
            "Broken price,,abc,USD,0,unit,true\n"
        );

        let result = import_products_csv_content(&conn, csv).expect("import products");

        assert_eq!(result["imported"], 2);
        assert_eq!(result["errors"].as_array().expect("errors").len(), 2);

        let products = super::list_products_for_backup(&conn).expect("products");
        let consulting = products
            .iter()
            .find(|p| p.name == "Consulting")
            .expect("consulting product");
        assert_eq!(consulting.default_price_minor, 12_550);
        assert_eq!(consulting.default_currency, "USD");
        assert_eq!(consulting.default_tax_rate_bps, 1600);
        assert!(consulting.is_active);

        let legacy = products
            .iter()
            .find(|p| p.name == "Legacy license")
            .expect("legacy product");
        assert_eq!(legacy.default_price_minor, 2_500);
        assert_eq!(legacy.default_currency, "UGX");
        assert!(!legacy.is_active);
    }

    #[test]
    fn export_products_csv_content_exports_all_products_safely() {
        let conn = test_conn();
        let product = CreateProduct {
            name: "=SUM(A1:A2)".to_string(),
            description: Some("Plan, quoted".to_string()),
            default_price_minor: Some(19_995),
            default_currency: Some("USD".to_string()),
            default_tax_rate_bps: Some(750),
            unit: Some("seat".to_string()),
        };
        let created =
            crate::db::queries::products::insert(&conn, &product).expect("insert product");
        crate::db::queries::products::update(
            &conn,
            &created.id,
            &crate::models::product::UpdateProduct {
                name: None,
                description: None,
                default_price_minor: None,
                default_currency: None,
                default_tax_rate_bps: None,
                unit: None,
                is_active: Some(false),
            },
        )
        .expect("deactivate product");

        let csv = export_products_csv_content(&conn).expect("export products");

        assert!(csv.starts_with(
            "name,description,default_price,default_currency,default_tax_rate_bps,unit,is_active\n"
        ));
        assert!(csv.contains("'=SUM(A1:A2)"));
        assert!(csv.contains("\"Plan, quoted\""));
        assert!(csv.contains(",199.95,USD,750,seat,false\n"));
    }

    #[test]
    fn restore_backup_restores_invoice_related_tables_once() {
        let conn = test_conn();
        let backup = json!({
            "version": "1.0",
            "settings": {
                "custom_setting": "\"enabled\""
            },
            "clients": [{
                "id": "client-1",
                "name": "Acme",
                "email": "billing@example.com",
                "phone": "",
                "address": "",
                "city": "",
                "country": "",
                "country_code": "",
                "tax_id": "",
                "currency_code": "USD",
                "payment_terms_days": 14,
                "notes": "",
                "created_at": "2026-05-11T00:00:00Z",
                "updated_at": "2026-05-11T00:00:00Z"
            }],
            "products": [{
                "id": "product-1",
                "name": "Consulting",
                "description": "",
                "default_price_minor": 10000,
                "default_currency": "USD",
                "default_tax_rate_bps": 1600,
                "unit": "hour",
                "is_active": true,
                "created_at": "2026-05-11T00:00:00Z",
                "updated_at": "2026-05-11T00:00:00Z"
            }],
            "tax_rates": [{
                "id": "tax-custom",
                "name": "VAT",
                "display_name": "VAT @ 16%",
                "rate_bps": 1600,
                "country_code": "KE",
                "is_default": false,
                "is_withholding": false,
                "is_inclusive": false,
                "is_active": true,
                "created_at": "2026-05-11T00:00:00Z",
                "updated_at": "2026-05-11T00:00:00Z"
            }],
            "invoices": [{
                "id": "invoice-1",
                "invoice_number": "INV-0001",
                "client_id": "client-1",
                "status": "sent",
                "currency_code": "USD",
                "subtotal_minor": 10000,
                "discount_minor": 0,
                "tax_amount_minor": 1600,
                "total_minor": 11600,
                "amount_paid_minor": 5000,
                "exchange_rate_to_usd": null,
                "exchange_rate_date": null,
                "issue_date": "2026-05-11",
                "due_date": "2026-05-25",
                "uses_inclusive_taxes": false,
                "notes": "",
                "terms": "",
                "footer": "",
                "created_at": "2026-05-11T00:00:00Z",
                "updated_at": "2026-05-11T00:00:00Z",
                "finalized_at": "2026-05-11T00:00:00Z",
                "sent_at": "2026-05-11T00:00:00Z",
                "paid_at": null,
                "voided_at": null
            }],
            "line_items": [{
                "id": "line-1",
                "invoice_id": "invoice-1",
                "product_id": "product-1",
                "description": "Consulting",
                "quantity": 100,
                "unit_price_minor": 10000,
                "tax_rate_bps": 1600,
                "discount_bps": 0,
                "line_total_minor": 10000,
                "sort_order": 0,
                "created_at": "2026-05-11T00:00:00Z"
            }],
            "invoice_taxes": [{
                "id": "invoice-tax-1",
                "invoice_id": "invoice-1",
                "tax_rate_id": "tax-custom",
                "tax_name": "VAT @ 16%",
                "tax_rate_bps": 1600,
                "tax_amount_minor": 1600,
                "is_withholding": false,
                "created_at": "2026-05-11T00:00:00Z"
            }],
            "payments": [{
                "id": "payment-1",
                "invoice_id": "invoice-1",
                "amount_minor": 5000,
                "currency_code": "USD",
                "payment_method": "bank",
                "payment_reference": "REF-1",
                "notes": "",
                "paid_at": "2026-05-12T00:00:00Z",
                "created_at": "2026-05-12T00:00:00Z"
            }],
            "recurring": [{
                "id": "recurring-1",
                "client_id": "client-1",
                "template_invoice_id": "invoice-1",
                "frequency": "monthly",
                "next_generation_date": "2026-06-11",
                "end_date": null,
                "auto_send": true,
                "last_generated": null,
                "status": "active",
                "created_at": "2026-05-11T00:00:00Z",
                "updated_at": "2026-05-11T00:00:00Z"
            }],
            "exchange_rates": [{
                "base_currency": "USD",
                "target_currency": "KES",
                "rate": 130.0,
                "fetched_at": "2026-05-11T00:00:00Z",
                "valid_date": "2026-05-11"
            }],
            "invoice_sequences": [{
                "sequence_name": "archive",
                "prefix": "AR",
                "separator": "-",
                "include_year": true,
                "pad_digits": 4,
                "year_reset": true,
                "last_year": 2026,
                "last_month": null,
                "next_number": 2,
                "created_at": "2026-05-11T00:00:00Z"
            }]
        });

        let restored = restore_backup(&conn, backup.clone()).expect("restore");

        assert_eq!(restored["restored"]["clients"], 1);
        assert_eq!(restored["restored"]["products"], 1);
        assert_eq!(restored["restored"]["tax_rates"], 1);
        assert_eq!(restored["restored"]["invoices"], 1);
        assert_eq!(restored["restored"]["line_items"], 1);
        assert_eq!(restored["restored"]["invoice_taxes"], 1);
        assert_eq!(restored["restored"]["payments"], 1);
        assert_eq!(restored["restored"]["recurring"], 1);
        assert_eq!(restored["restored"]["exchange_rates"], 1);
        assert_eq!(restored["restored"]["invoice_sequences"], 1);
        assert_eq!(table_count(&conn, "invoice_line_items"), 1);
        assert_eq!(table_count(&conn, "invoice_taxes"), 1);
        assert_eq!(table_count(&conn, "payments"), 1);

        let duplicate = restore_backup(&conn, backup).expect("duplicate restore");
        assert_eq!(duplicate["restored"]["clients"], 0);
        assert_eq!(duplicate["restored"]["invoices"], 0);
        assert_eq!(duplicate["restored"]["line_items"], 0);
        assert_eq!(duplicate["restored"]["payments"], 0);
    }
}
