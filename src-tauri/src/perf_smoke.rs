use crate::commands::import_export::{backup_database_value, export_invoices_csv_content};
use crate::db::{migrations, queries};
use crate::models::business::BusinessProfile;
use crate::services::pdf_engine;
use rusqlite::{params, Connection};
use std::env;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
struct PerfProfile {
    clients: usize,
    products: usize,
    invoices: usize,
    lines_per_invoice: usize,
}

impl PerfProfile {
    fn from_env() -> Self {
        Self {
            clients: env_usize("PERF_SMOKE_CLIENTS", 500),
            products: env_usize("PERF_SMOKE_PRODUCTS", 300),
            invoices: env_usize("PERF_SMOKE_INVOICES", 1_000),
            lines_per_invoice: env_usize("PERF_SMOKE_LINES_PER_INVOICE", 3),
        }
        .normalized()
    }

    fn normalized(self) -> Self {
        Self {
            clients: self.clients.max(1),
            products: self.products.max(1),
            invoices: self.invoices.max(1),
            lines_per_invoice: self.lines_per_invoice.max(1),
        }
    }

    fn expected_line_items(self) -> usize {
        self.invoices * self.lines_per_invoice
    }
}

#[test]
#[ignore = "run with scripts/verify-performance-smoke.sh"]
fn perf_smoke_large_dataset_hot_paths() {
    let profile = PerfProfile::from_env();
    let mut conn = Connection::open_in_memory().expect("in-memory db");
    migrations::run_migrations(&conn).expect("migrations");

    let (_, seed_elapsed) = timed(|| seed_scale_data(&mut conn, profile));
    report("seed_scale_data", seed_elapsed, None);

    let (clients, clients_elapsed) =
        timed(|| queries::clients::list_all(&conn).expect("list clients"));
    assert_eq!(clients.len(), profile.clients);
    assert_budget(
        "clients_list_all",
        clients_elapsed,
        env_ms("PERF_SMOKE_CLIENTS_LIST_MS", 250),
    );

    let (products, products_elapsed) =
        timed(|| queries::products::list_all(&conn).expect("list products"));
    assert_eq!(products.len(), profile.products);
    assert_budget(
        "products_list_all",
        products_elapsed,
        env_ms("PERF_SMOKE_PRODUCTS_LIST_MS", 250),
    );

    let (invoices, invoices_elapsed) =
        timed(|| queries::invoices::list_all(&conn).expect("list invoices"));
    assert_eq!(invoices.len(), profile.invoices);
    assert_budget(
        "invoices_list_all",
        invoices_elapsed,
        env_ms("PERF_SMOKE_INVOICES_LIST_MS", 350),
    );

    let (detail_count, details_elapsed) = timed(|| {
        invoices
            .iter()
            .map(|invoice| {
                queries::invoices::get_with_details(&conn, &invoice.id)
                    .expect("get invoice details")
                    .expect("invoice exists")
                    .line_items
                    .len()
            })
            .sum::<usize>()
    });
    assert_eq!(detail_count, profile.expected_line_items());
    assert_budget(
        "invoice_detail_fanout",
        details_elapsed,
        env_ms("PERF_SMOKE_DETAIL_FANOUT_MS", 3_500),
    );

    let (csv, csv_elapsed) =
        timed(|| export_invoices_csv_content(&conn).expect("export invoices csv"));
    assert_eq!(csv.lines().count(), profile.invoices + 1);
    assert_budget(
        "invoice_csv_export",
        csv_elapsed,
        env_ms("PERF_SMOKE_INVOICE_CSV_MS", 500),
    );

    let (backup, backup_elapsed) = timed(|| backup_database_value(&conn).expect("backup database"));
    assert_eq!(array_len(&backup, "clients"), profile.clients);
    assert_eq!(array_len(&backup, "invoices"), profile.invoices);
    assert_eq!(
        array_len(&backup, "line_items"),
        profile.expected_line_items()
    );
    assert_budget(
        "json_backup",
        backup_elapsed,
        env_ms("PERF_SMOKE_BACKUP_MS", 3_500),
    );

    let invoice = queries::invoices::get_with_details(&conn, "invoice-000001")
        .expect("get pdf invoice")
        .expect("pdf invoice exists");
    let business = sample_business();
    let (pdf, pdf_elapsed) = timed(|| {
        pdf_engine::generate_invoice_pdf_bytes(&invoice, &business, "a4", "en", "YYYY-MM-DD")
    });
    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.len() > 1_000);
    assert_budget(
        "native_pdf_generation",
        pdf_elapsed,
        env_ms("PERF_SMOKE_PDF_MS", 350),
    );
}

fn seed_scale_data(conn: &mut Connection, profile: PerfProfile) {
    let tx = conn.transaction().expect("transaction");
    tx.execute(
        "INSERT INTO business_profiles
            (id, name, city, country, country_code, default_currency, default_payment_terms_days)
         VALUES ('business-perf', '900Invoice Performance Smoke', 'Nairobi', 'Kenya', 'KE', 'KES', 30)",
        [],
    )
    .expect("insert business");

    for idx in 0..profile.clients {
        tx.execute(
            "INSERT INTO clients
                (id, name, email, city, country, country_code, tax_id, currency_code, payment_terms_days)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'KES', 30)",
            params![
                format!("client-{idx:06}"),
                format!("Client {idx:06}"),
                format!("client{idx:06}@example.com"),
                "Nairobi",
                "Kenya",
                "KE",
                format!("PIN{idx:06}"),
            ],
        )
        .expect("insert client");
    }

    for idx in 0..profile.products {
        tx.execute(
            "INSERT INTO products
                (id, name, description, default_price_minor, default_currency,
                 default_tax_rate_id, default_tax_rate_bps, unit, is_active)
             VALUES (?1, ?2, ?3, ?4, 'KES', 'tax-ke-vat', 1600, 'hour', 1)",
            params![
                format!("product-{idx:06}"),
                format!("Service {idx:06}"),
                format!("Seeded service {idx:06}"),
                10_000 + idx as i64,
            ],
        )
        .expect("insert product");
    }

    for idx in 0..profile.invoices {
        let invoice_id = format!("invoice-{idx:06}");
        let client_id = format!("client-{:06}", idx % profile.clients);
        let status = match idx % 5 {
            0 => "paid",
            1 => "sent",
            2 => "finalized",
            3 => "draft",
            _ => "void",
        };
        let subtotal_minor = (0..profile.lines_per_invoice)
            .map(|line_idx| line_price_minor(idx, line_idx))
            .sum::<i64>();
        let tax_minor = subtotal_minor * 1_600 / 10_000;
        let total_minor = subtotal_minor + tax_minor;
        let amount_paid_minor = if status == "paid" { total_minor } else { 0 };

        tx.execute(
            "INSERT INTO invoices
                (id, invoice_number, client_id, status, currency_code, subtotal_minor,
                 discount_minor, tax_amount_minor, total_minor, amount_paid_minor,
                 exchange_rate_to_usd, exchange_rate_date, issue_date, due_date,
                 uses_inclusive_taxes, notes, terms, footer, created_at, updated_at,
                 finalized_at, sent_at, paid_at, voided_at)
             VALUES
                (?1, ?2, ?3, ?4, 'KES', ?5, 0, ?6, ?7, ?8,
                 0.0078, '2026-05-20', '2026-05-20', '2026-06-19',
                 0, 'Performance smoke invoice', 'Net 30', '',
                 ?9, ?9, ?10, ?11, ?12, ?13)",
            params![
                invoice_id,
                format!("INV-2026-{idx:06}"),
                client_id,
                status,
                subtotal_minor,
                tax_minor,
                total_minor,
                amount_paid_minor,
                timestamp(idx),
                if status != "draft" {
                    Some(timestamp(idx))
                } else {
                    None
                },
                if matches!(status, "sent" | "paid") {
                    Some(timestamp(idx))
                } else {
                    None
                },
                if status == "paid" {
                    Some(timestamp(idx))
                } else {
                    None
                },
                if status == "void" {
                    Some(timestamp(idx))
                } else {
                    None
                },
            ],
        )
        .expect("insert invoice");

        for line_idx in 0..profile.lines_per_invoice {
            let unit_price_minor = line_price_minor(idx, line_idx);
            tx.execute(
                "INSERT INTO invoice_line_items
                    (id, invoice_id, product_id, tax_rate_id, description, quantity,
                     unit_price_minor, tax_rate_bps, discount_bps, line_total_minor,
                     sort_order, created_at)
                 VALUES (?1, ?2, ?3, 'tax-ke-vat', ?4, 100, ?5, 1600, 0, ?5, ?6, ?7)",
                params![
                    format!("line-{idx:06}-{line_idx:02}"),
                    invoice_id,
                    format!("product-{:06}", (idx + line_idx) % profile.products),
                    format!("Consulting block {line_idx}"),
                    unit_price_minor,
                    line_idx as i32,
                    timestamp(idx),
                ],
            )
            .expect("insert line item");
        }

        tx.execute(
            "INSERT INTO invoice_taxes
                (id, invoice_id, tax_rate_id, tax_name, tax_rate_bps, tax_amount_minor,
                 is_withholding, created_at)
             VALUES (?1, ?2, 'tax-ke-vat', 'VAT @ 16%', 1600, ?3, 0, ?4)",
            params![
                format!("tax-{idx:06}"),
                invoice_id,
                tax_minor,
                timestamp(idx)
            ],
        )
        .expect("insert invoice tax");

        if idx % 4 == 0 {
            tx.execute(
                "INSERT INTO payments
                    (id, invoice_id, amount_minor, currency_code, payment_method,
                     payment_reference, notes, paid_at, created_at)
                 VALUES (?1, ?2, ?3, 'KES', 'bank_transfer', ?4, '', ?5, ?5)",
                params![
                    format!("payment-{idx:06}"),
                    invoice_id,
                    if status == "paid" {
                        total_minor
                    } else {
                        total_minor / 2
                    },
                    format!("PERF-{idx:06}"),
                    timestamp(idx),
                ],
            )
            .expect("insert payment");
        }
    }

    tx.commit().expect("commit seed data");
}

fn line_price_minor(invoice_idx: usize, line_idx: usize) -> i64 {
    10_000 + ((invoice_idx + line_idx) % 500) as i64
}

fn timestamp(idx: usize) -> String {
    let day = (idx % 28) + 1;
    let minute = idx % 60;
    format!("2026-05-{day:02} 12:{minute:02}:00")
}

fn timed<T>(work: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = work();
    (result, start.elapsed())
}

fn assert_budget(label: &str, elapsed: Duration, budget: Duration) {
    report(label, elapsed, Some(budget));
    if env_bool("PERF_SMOKE_ENFORCE_BUDGETS", true) {
        assert!(
            elapsed <= budget,
            "{label} took {}ms, over budget {}ms",
            elapsed.as_millis(),
            budget.as_millis()
        );
    }
}

fn report(label: &str, elapsed: Duration, budget: Option<Duration>) {
    match budget {
        Some(budget) => eprintln!(
            "perf_smoke {label}: {}ms (budget {}ms)",
            elapsed.as_millis(),
            budget.as_millis()
        ),
        None => eprintln!("perf_smoke {label}: {}ms", elapsed.as_millis()),
    }
}

fn array_len(value: &serde_json::Value, key: &str) -> usize {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map_or(0, Vec::len)
}

fn env_usize(name: &str, default: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
}

fn env_ms(name: &str, default: u64) -> Duration {
    Duration::from_millis(
        env::var(name)
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(default),
    )
}

fn env_bool(name: &str, default: bool) -> bool {
    env::var(name)
        .ok()
        .map(|value| !matches!(value.as_str(), "0" | "false" | "FALSE" | "no" | "NO"))
        .unwrap_or(default)
}

fn sample_business() -> BusinessProfile {
    BusinessProfile {
        id: "business-perf".to_string(),
        name: "900Invoice Performance Smoke".to_string(),
        address: "123 Test Street".to_string(),
        city: "Nairobi".to_string(),
        country: "Kenya".to_string(),
        country_code: "KE".to_string(),
        phone: "+254700000000".to_string(),
        email: "billing@example.com".to_string(),
        website: "https://example.com".to_string(),
        tax_id: "P000000000A".to_string(),
        logo_path: None,
        default_currency: "KES".to_string(),
        default_payment_terms_days: 30,
        bank_name: "Example Bank".to_string(),
        bank_account_number: "0000000000".to_string(),
        bank_routing_number: "000".to_string(),
        mobile_money_number: "+254700000000".to_string(),
        mobile_money_provider: "M-Pesa".to_string(),
        created_at: "2026-05-20 12:00:00".to_string(),
        updated_at: "2026-05-20 12:00:00".to_string(),
    }
}
