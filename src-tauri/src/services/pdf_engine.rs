//! PDF/HTML Invoice Engine for 900Invoice.
//!
//! Generates print-ready HTML invoices with embedded CSS.
//! The HTML serves as:
//!   1. Live preview in Tauri WebView
//!   2. PDF source via window.print() or headless browser
//!
//! All money in i64 minor units; currency config drives formatting.

use crate::models::business::BusinessProfile;
use crate::models::invoice::InvoiceWithDetails;
use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// Currency configuration
// ---------------------------------------------------------------------------

struct CurrencyConfig {
    symbol: &'static str,
    symbol_after: bool,
    decimals: u32,
    thousands_sep: char,
    decimal_sep: char,
}

fn currency_config(code: &str) -> CurrencyConfig {
    match code {
        "KES" => CurrencyConfig {
            symbol: "KSh",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "NGN" => CurrencyConfig {
            symbol: "\u{20A6}",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "ZAR" => CurrencyConfig {
            symbol: "R",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "INR" => CurrencyConfig {
            symbol: "\u{20B9}",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "GHS" => CurrencyConfig {
            symbol: "GH\u{20B5}",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "TZS" => CurrencyConfig {
            symbol: "TSh",
            symbol_after: false,
            decimals: 0,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "UGX" => CurrencyConfig {
            symbol: "USh",
            symbol_after: false,
            decimals: 0,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "XOF" => CurrencyConfig {
            symbol: "CFA",
            symbol_after: false,
            decimals: 0,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "XAF" => CurrencyConfig {
            symbol: "CFA",
            symbol_after: false,
            decimals: 0,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "EUR" => CurrencyConfig {
            symbol: "\u{20AC}",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        "USD" => CurrencyConfig {
            symbol: "$",
            symbol_after: false,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
        _ => CurrencyConfig {
            symbol: "",
            symbol_after: true,
            decimals: 2,
            thousands_sep: ',',
            decimal_sep: '.',
        },
    }
}

// ---------------------------------------------------------------------------
// Formatting helpers
// ---------------------------------------------------------------------------

/// Format a minor-unit amount as a human-readable currency string.
/// KES 150000 → "KSh 1,500.00", UGX 1500 → "USh 1,500", NGN 150000 → "₦1,500.00"
pub fn format_currency_html(amount_minor: i64, currency_code: &str) -> String {
    let cfg = currency_config(currency_code);
    let negative = amount_minor < 0;
    let abs_amount = amount_minor.unsigned_abs() as i64;

    let (whole, frac): (i64, i64) = if cfg.decimals == 0 {
        (abs_amount, 0)
    } else {
        let divisor = 10i64.pow(cfg.decimals);
        (abs_amount / divisor, abs_amount % divisor)
    };

    let whole_str = format_with_thousands(whole, cfg.thousands_sep);

    let number_str = if cfg.decimals > 0 {
        let frac_str = format!("{:0>width$}", frac, width = cfg.decimals as usize);
        format!("{}{}{}", whole_str, cfg.decimal_sep, frac_str)
    } else {
        whole_str
    };

    let neg_sign = if negative { "-" } else { "" };

    if cfg.symbol.is_empty() {
        format!("{}{} {}", neg_sign, number_str, currency_code)
    } else if cfg.symbol_after {
        format!("{}{} {}", neg_sign, number_str, cfg.symbol)
    } else if cfg.symbol.chars().count() <= 2 {
        format!("{}{}{}", neg_sign, cfg.symbol, number_str)
    } else {
        format!("{}{} {}", neg_sign, cfg.symbol, number_str)
    }
}

fn format_with_thousands(n: i64, sep: char) -> String {
    let s = format!("{}", n.abs());
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(sep);
        }
        result.push(*c);
    }
    result
}

/// Format quantity stored as qty*100 integer.
fn format_quantity(qty: i64) -> String {
    let whole = qty / 100;
    let frac = (qty % 100).unsigned_abs();
    if frac == 0 {
        format!("{}", whole)
    } else {
        format!("{}.{:02}", whole, frac)
    }
}

/// Format basis points as percentage string. 1600 → "16%", 750 → "7.5%"
fn format_rate_bps(rate_bps: i32) -> String {
    let whole = rate_bps / 100;
    let frac = rate_bps % 100;
    if frac == 0 {
        format!("{}%", whole)
    } else {
        let s = format!("{}.{:02}", whole, frac);
        let trimmed = s.trim_end_matches('0');
        format!("{}%", trimmed)
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn nl2br(s: &str) -> String {
    s.replace('\n', "<br/>")
}

fn status_colors(status: &str) -> (&'static str, &'static str) {
    match status.to_lowercase().as_str() {
        "draft" => ("#E5E7EB", "#374151"),
        "finalized" => ("#DBEAFE", "#1D4ED8"),
        "sent" => ("#FEF3C7", "#92400E"),
        "paid" => ("#D1FAE5", "#065F46"),
        "void" => ("#FEE2E2", "#991B1B"),
        "overdue" => ("#FEE2E2", "#991B1B"),
        _ => ("#F3F4F6", "#6B7280"),
    }
}

/// Read logo file and return base64 data URI, or None if unreadable.
fn load_logo_base64(path: &str) -> Option<String> {
    let canonical = std::fs::canonicalize(path).ok()?;
    let ext = canonical
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())?;
    if !matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp"
    ) {
        return None;
    }
    let size = std::fs::metadata(&canonical).ok()?.len();
    if size > 2 * 1024 * 1024 {
        return None;
    }

    let data = std::fs::read(&canonical).ok()?;
    let b64 = base64_encode(&data);
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => "image/png",
    };
    Some(format!("data:{};base64,{}", mime, b64))
}

fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < input.len() {
        let b0 = input[i] as u32;
        let b1 = if i + 1 < input.len() {
            input[i + 1] as u32
        } else {
            0
        };
        let b2 = if i + 2 < input.len() {
            input[i + 2] as u32
        } else {
            0
        };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        out.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        if i + 1 < input.len() {
            out.push(CHARS[((n >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < input.len() {
            out.push(CHARS[(n & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

// ---------------------------------------------------------------------------
// HTML generation
// ---------------------------------------------------------------------------

/// Generate a complete, print-ready HTML invoice.
///
/// `paper_size`: "a4" (default) or "letter"
/// `locale`: reserved for future RTL/locale support
pub fn generate_invoice_html(
    invoice: &InvoiceWithDetails,
    business: &BusinessProfile,
    paper_size: &str,
    _locale: &str,
) -> String {
    let currency = &invoice.currency_code;

    let page_css = match paper_size.to_lowercase().as_str() {
        "letter" => "@page { size: letter; margin: 0; }",
        _ => "@page { size: A4; margin: 0; }",
    };
    let page_w = if paper_size.to_lowercase() == "letter" {
        "816px"
    } else {
        "794px"
    };
    let page_h = if paper_size.to_lowercase() == "letter" {
        "1056px"
    } else {
        "1123px"
    };

    // Logo
    let logo_html = business
        .logo_path
        .as_ref()
        .and_then(|p| load_logo_base64(p))
        .map(|uri| {
            format!(
                r#"<img src="{}" alt="{}" class="logo" />"#,
                uri,
                html_escape(&business.name)
            )
        })
        .unwrap_or_default();

    // Business info
    let biz_addr = build_address_lines(&business.address, &business.city, &business.country);
    let biz_phone = opt_div("Phone: ", &business.phone);
    let biz_email = opt_div("Email: ", &business.email);
    let biz_tax = opt_div("Tax ID: ", &business.tax_id);
    let biz_web = opt_div("", &business.website);

    // Client info
    let client = invoice.client.as_ref();
    let cli_name_text = client.map(|c| c.name.as_str()).unwrap_or("Unknown Client");
    let cli_addr = build_address_lines(
        client.map(|c| c.address.as_str()).unwrap_or(""),
        client.map(|c| c.city.as_str()).unwrap_or(""),
        client.map(|c| c.country.as_str()).unwrap_or(""),
    );
    let cli_email = opt_div("", client.map(|c| c.email.as_str()).unwrap_or(""));
    let cli_phone = opt_div("", client.map(|c| c.phone.as_str()).unwrap_or(""));
    let cli_tax = opt_div("Tax ID: ", client.map(|c| c.tax_id.as_str()).unwrap_or(""));
    let inv_num_text = invoice.invoice_number.as_deref().unwrap_or("DRAFT");

    // Status badge
    let (status_bg, status_fg) = status_colors(&invoice.status);
    let status_label = invoice.status.to_uppercase();

    // Line item rows
    let mut sorted_items: Vec<_> = invoice.line_items.iter().collect();
    sorted_items.sort_by_key(|i| i.sort_order);
    let mut line_rows = String::new();
    for (idx, item) in sorted_items.iter().enumerate() {
        let disc_note = if item.discount_bps > 0 {
            format!(
                r#"<div class="item-note">Disc: {}</div>"#,
                format_rate_bps(item.discount_bps)
            )
        } else {
            String::new()
        };
        line_rows.push_str(&format!(
            r#"<tr>
              <td class="num">{}</td>
              <td><div class="item-desc">{}</div>{}</td>
              <td class="r">{}</td>
              <td class="r">{}</td>
              <td class="r">{}</td>
              <td class="r">{}</td>
            </tr>"#,
            idx + 1,
            html_escape(&item.description),
            disc_note,
            format_quantity(item.quantity),
            format_currency_html(item.unit_price_minor, currency),
            if item.tax_rate_bps > 0 {
                format_rate_bps(item.tax_rate_bps)
            } else {
                "—".into()
            },
            format_currency_html(item.line_total_minor, currency),
        ));
    }

    // Tax rows
    let mut tax_rows = String::new();
    for tax in &invoice.taxes {
        let label = format!("{} ({})", tax.tax_name, format_rate_bps(tax.tax_rate_bps));
        let amt = format_currency_html(tax.tax_amount_minor, currency);
        if tax.is_withholding {
            tax_rows.push_str(&format!(
                r#"<tr><td class="slbl">{} <em class="wht">(WHT)</em></td><td class="r red">-{}</td></tr>"#,
                html_escape(&label), amt
            ));
        } else {
            tax_rows.push_str(&format!(
                r#"<tr><td class="slbl">{}</td><td class="r">{}</td></tr>"#,
                html_escape(&label),
                amt
            ));
        }
    }

    // Discount row
    let discount_row = if invoice.discount_minor > 0 {
        format!(
            r#"<tr><td class="slbl">Discount</td><td class="r red">-{}</td></tr>"#,
            format_currency_html(invoice.discount_minor, currency)
        )
    } else {
        String::new()
    };

    // Payment row
    let balance = invoice
        .total_minor
        .saturating_sub(invoice.amount_paid_minor);
    let paid_rows = if invoice.amount_paid_minor > 0 {
        format!(
            r#"<tr class="sep"><td colspan="2"><hr/></td></tr>
            <tr><td class="slbl">Amount Paid</td><td class="r red">-{}</td></tr>
            <tr class="balance"><td class="slbl">Balance Due</td><td class="r red-bold">{}</td></tr>"#,
            format_currency_html(invoice.amount_paid_minor, currency),
            format_currency_html(balance, currency),
        )
    } else {
        String::new()
    };

    // Payment details
    let pay_details = build_payment_html(business);

    // Notes / Terms
    let notes_html = if !invoice.notes.trim().is_empty() {
        format!(
            r#"<div class="section"><div class="stitle">NOTES</div><div class="notetext">{}</div></div>"#,
            nl2br(&html_escape(&invoice.notes))
        )
    } else {
        String::new()
    };
    let terms_html = if !invoice.terms.trim().is_empty() {
        format!(
            r#"<div class="section"><div class="stitle">TERMS &amp; CONDITIONS</div><div class="notetext">{}</div></div>"#,
            nl2br(&html_escape(&invoice.terms))
        )
    } else {
        String::new()
    };

    let footer_text = if invoice.footer.trim().is_empty() {
        "Generated by 900Invoice &mdash; 900labs.com/open-source".to_string()
    } else {
        html_escape(&invoice.footer)
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>Invoice {inv_num}</title>
<style>
{page_css}
*,*::before,*::after{{box-sizing:border-box}}
:root{{
  --t:#20808D;--tl:#E8F5F6;--tm:#C5E5E8;
  --dk:#1A1A1A;--md:#4A4A4A;--lt:#888;
  --bd:#E0E0E0;--alt:#F7FAFB;--rd:#DC2626;
  --pw:{page_w};--ph:{page_h};--pm:28px;
}}
body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI','Helvetica Neue',Arial,sans-serif;
  font-size:10pt;color:var(--dk);background:#F0F0F0;margin:0;padding:20px;line-height:1.5}}
.page{{background:#fff;width:var(--pw);min-height:var(--ph);margin:0 auto;
  padding:var(--pm);box-shadow:0 2px 12px rgba(0,0,0,.12);position:relative}}
.hdr{{display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:22px}}
.hdr-l{{flex:1}}
.logo{{max-height:60px;max-width:160px;object-fit:contain;margin-bottom:8px;display:block}}
.biz-name{{font-size:17pt;font-weight:700;color:var(--t);line-height:1.2;margin:0 0 4px}}
.biz-det{{font-size:8.5pt;color:var(--md);line-height:1.65}}
.hdr-r{{text-align:right;flex-shrink:0;padding-left:24px}}
.inv-title{{font-size:26pt;font-weight:800;color:var(--t);letter-spacing:-.5px;line-height:1;margin-bottom:12px}}
.inv-meta{{font-size:9pt;line-height:1.85;color:var(--md)}}
.inv-meta .lbl{{font-weight:600;color:var(--dk)}}
.badge{{display:inline-block;padding:2px 10px;border-radius:12px;font-size:7.5pt;
  font-weight:700;letter-spacing:.5px;background:{sbg};color:{sfg};margin-top:4px}}
.divider{{height:2px;background:linear-gradient(to right,var(--t),var(--tm));
  margin:0 0 18px;border-radius:1px}}
.stitle{{font-size:7.5pt;font-weight:700;color:var(--t);letter-spacing:1px;
  text-transform:uppercase;margin-bottom:5px;padding-bottom:3px;
  border-bottom:1.5px solid var(--tm)}}
.bill{{display:flex;gap:36px;margin-bottom:22px}}
.cli-name{{font-size:11pt;font-weight:700;margin-bottom:3px}}
.cli-det{{font-size:8.5pt;color:var(--md);line-height:1.7}}
table.items{{width:100%;border-collapse:collapse;margin-bottom:18px;font-size:9pt}}
table.items thead tr{{background:var(--t);color:#fff}}
table.items thead th{{padding:7px 9px;text-align:left;font-size:8.5pt;font-weight:600}}
table.items thead th.r{{text-align:right}}
table.items tbody tr:nth-child(odd){{background:var(--alt)}}
table.items tbody tr:nth-child(even){{background:#fff}}
table.items tbody td{{padding:6px 9px;vertical-align:top;border-bottom:1px solid var(--bd)}}
td.r{{text-align:right;font-variant-numeric:tabular-nums}}
td.num{{text-align:center;color:var(--lt);font-size:8pt;width:28px}}
.item-desc{{font-weight:600}}
.item-note{{font-size:7.5pt;color:var(--lt);margin-top:2px}}
.sum-wrap{{display:flex;justify-content:flex-end;margin-bottom:22px}}
table.sum{{width:268px;border-collapse:collapse;font-size:9pt}}
table.sum td{{padding:3px 0;vertical-align:middle}}
.slbl{{color:var(--md);padding-right:14px}}
.sep td{{padding:4px 0}}.sep hr{{border:none;border-top:1px solid var(--bd);margin:0}}
.total-row td{{padding-top:8px;border-top:2px solid var(--t);font-size:12pt;font-weight:700}}
.total-row .r{{color:var(--t)}}
.red{{color:var(--rd)}}.red-bold{{color:var(--rd);font-weight:700;font-size:11pt}}
.wht{{font-size:7pt;color:var(--lt)}}
.pay-wrap{{display:flex;gap:28px;margin-bottom:4px}}
.pay-blk{{flex:1}}.pay-blk p{{margin:0 0 3px;font-size:8.5pt;color:var(--md);line-height:1.6}}
.notetext{{font-size:8.5pt;color:var(--md);line-height:1.7;background:var(--tl);
  padding:9px 11px;border-radius:4px;border-left:3px solid var(--t)}}
.section{{margin-bottom:16px}}
.qr-wrap{{text-align:center;margin:18px 0}}
.qr-box{{display:inline-flex;width:76px;height:76px;border:1.5px solid var(--bd);
  border-radius:4px;align-items:center;justify-content:center;flex-direction:column;gap:3px}}
.qr-box span{{font-size:7pt;color:var(--lt);text-align:center;line-height:1.3}}
.footer{{margin-top:28px;padding-top:10px;border-top:1px solid var(--bd);
  display:flex;justify-content:space-between;align-items:center}}
.footer-txt{{font-size:7.5pt;color:var(--lt)}}
@media print{{
  {page_css}
  body{{background:none;padding:0;margin:0}}
  .page{{box-shadow:none;margin:0;padding:16mm;width:100%;min-height:unset}}
  table.items tbody tr{{page-break-inside:avoid}}
}}
</style>
</head>
<body>
<div class="page">
  <div class="hdr">
    <div class="hdr-l">
      {logo_html}
      <div class="biz-name">{biz_name}</div>
      <div class="biz-det">{biz_addr}{biz_phone}{biz_email}{biz_tax}{biz_web}</div>
    </div>
    <div class="hdr-r">
      <div class="inv-title">INVOICE</div>
      <div class="inv-meta">
        <div><span class="lbl">Invoice #:</span> {inv_num}</div>
        <div><span class="lbl">Date:</span> {issue_date}</div>
        <div><span class="lbl">Due:</span> {due_date}</div>
        <div><span class="badge">{status_label}</span></div>
      </div>
    </div>
  </div>
  <div class="divider"></div>
  <div class="bill">
    <div>
      <div class="stitle">Bill To</div>
      <div class="cli-name">{cli_name}</div>
      <div class="cli-det">{cli_addr}{cli_email}{cli_phone}{cli_tax}</div>
    </div>
  </div>
  <div class="stitle" style="margin-bottom:6px">Line Items</div>
  <table class="items">
    <thead><tr>
      <th style="width:28px">#</th>
      <th>Description</th>
      <th class="r" style="width:54px">Qty</th>
      <th class="r" style="width:100px">Unit Price</th>
      <th class="r" style="width:54px">Tax</th>
      <th class="r" style="width:108px">Amount</th>
    </tr></thead>
    <tbody>{line_rows}</tbody>
  </table>
  <div class="sum-wrap">
    <table class="sum">
      <tbody>
        <tr><td class="slbl">Subtotal</td><td class="r">{subtotal}</td></tr>
        {discount_row}
        {tax_rows}
        <tr class="sep"><td colspan="2"><hr/></td></tr>
        <tr class="total-row"><td class="slbl">Total</td><td class="r">{total}</td></tr>
        {paid_rows}
      </tbody>
    </table>
  </div>
  {pay_details}
  {notes_html}
  {terms_html}
  <div class="qr-wrap">
    <div class="qr-box"><span>QR Code<br/>(Fiscal)</span></div>
  </div>
  <div class="footer">
    <div class="footer-txt">{inv_num}</div>
    <div class="footer-txt">{footer_text}</div>
  </div>
</div>
</body>
</html>"#,
        page_css = page_css,
        page_w = page_w,
        page_h = page_h,
        sbg = status_bg,
        sfg = status_fg,
        inv_num = html_escape(inv_num_text),
        biz_name = html_escape(&business.name),
        logo_html = logo_html,
        biz_addr = biz_addr,
        biz_phone = biz_phone,
        biz_email = biz_email,
        biz_tax = biz_tax,
        biz_web = biz_web,
        issue_date = html_escape(&invoice.issue_date),
        due_date = html_escape(&invoice.due_date),
        status_label = status_label,
        cli_name = html_escape(cli_name_text),
        cli_addr = cli_addr,
        cli_email = cli_email,
        cli_phone = cli_phone,
        cli_tax = cli_tax,
        line_rows = line_rows,
        subtotal = format_currency_html(invoice.subtotal_minor, currency),
        discount_row = discount_row,
        tax_rows = tax_rows,
        total = format_currency_html(invoice.total_minor, currency),
        paid_rows = paid_rows,
        pay_details = pay_details,
        notes_html = notes_html,
        terms_html = terms_html,
        footer_text = footer_text,
    )
}

fn build_address_lines(address: &str, city: &str, country: &str) -> String {
    let mut s = String::new();
    if !address.trim().is_empty() {
        s.push_str(&format!("<div>{}</div>", html_escape(address)));
    }
    let cc = match (city.trim().is_empty(), country.trim().is_empty()) {
        (false, false) => format!("{}, {}", city, country),
        (false, true) => city.to_string(),
        (true, false) => country.to_string(),
        (true, true) => String::new(),
    };
    if !cc.is_empty() {
        s.push_str(&format!("<div>{}</div>", html_escape(&cc)));
    }
    s
}

fn opt_div(prefix: &str, val: &str) -> String {
    if val.trim().is_empty() {
        String::new()
    } else {
        format!("<div>{}{}</div>", html_escape(prefix), html_escape(val))
    }
}

fn build_payment_html(business: &BusinessProfile) -> String {
    let has_bank = !business.bank_name.is_empty() || !business.bank_account_number.is_empty();
    let has_mobile = !business.mobile_money_number.is_empty();
    if !has_bank && !has_mobile {
        return String::new();
    }
    let bank_blk = if has_bank {
        let mut parts = String::new();
        if !business.bank_name.is_empty() {
            parts.push_str(&format!(
                "<p><strong>Bank:</strong> {}</p>",
                html_escape(&business.bank_name)
            ));
        }
        if !business.bank_account_number.is_empty() {
            parts.push_str(&format!(
                "<p><strong>Account #:</strong> {}</p>",
                html_escape(&business.bank_account_number)
            ));
        }
        if !business.bank_routing_number.is_empty() {
            parts.push_str(&format!(
                "<p><strong>Routing #:</strong> {}</p>",
                html_escape(&business.bank_routing_number)
            ));
        }
        format!(
            r#"<div class="pay-blk"><div class="stitle" style="margin-bottom:4px">Bank Transfer</div>{}</div>"#,
            parts
        )
    } else {
        String::new()
    };

    let mobile_blk = if has_mobile {
        let prov = if !business.mobile_money_provider.is_empty() {
            format!(
                "<p><strong>Provider:</strong> {}</p>",
                html_escape(&business.mobile_money_provider)
            )
        } else {
            String::new()
        };
        format!(
            r#"<div class="pay-blk"><div class="stitle" style="margin-bottom:4px">Mobile Money</div>{}<p><strong>Number:</strong> {}</p></div>"#,
            prov,
            html_escape(&business.mobile_money_number)
        )
    } else {
        String::new()
    };

    format!(
        r#"<div class="section"><div class="stitle">Payment Details</div><div class="pay-wrap">{}{}</div></div>"#,
        bank_blk, mobile_blk
    )
}

// ---------------------------------------------------------------------------
// Native PDF generation
// ---------------------------------------------------------------------------

/// Generate a self-contained PDF invoice.
///
/// The HTML renderer remains the rich preview surface; this renderer emits
/// portable PDF bytes for native file export without external binaries.
pub fn generate_invoice_pdf_bytes(
    invoice: &InvoiceWithDetails,
    business: &BusinessProfile,
    paper_size: &str,
    _locale: &str,
) -> Vec<u8> {
    let (page_width, page_height) = match paper_size.to_lowercase().as_str() {
        "letter" => (612.0, 792.0),
        _ => (595.0, 842.0),
    };

    let mut renderer = PdfRenderer::new(page_width, page_height);
    renderer.draw_invoice(invoice, business);
    renderer.finish()
}

struct PdfRenderer {
    width: f32,
    height: f32,
    margin: f32,
    y: f32,
    pages: Vec<String>,
    canvas: PdfCanvas,
}

impl PdfRenderer {
    fn new(width: f32, height: f32) -> Self {
        let margin = 40.0;
        Self {
            width,
            height,
            margin,
            y: height - margin,
            pages: Vec::new(),
            canvas: PdfCanvas::new(),
        }
    }

    fn draw_invoice(&mut self, invoice: &InvoiceWithDetails, business: &BusinessProfile) {
        self.draw_header(invoice, business);
        self.draw_bill_to(invoice);
        self.draw_line_items(invoice);
        self.draw_totals(invoice);
        self.draw_payment_details(business);
        self.draw_text_section("Notes", &invoice.notes);
        self.draw_text_section("Terms", &invoice.terms);
        if !invoice.footer.trim().is_empty() {
            self.draw_text_section("", &invoice.footer);
        }
    }

    fn finish(mut self) -> Vec<u8> {
        self.draw_footer();
        self.push_page();
        build_pdf_document(&self.pages, self.width, self.height)
    }

    fn push_page(&mut self) {
        let content = std::mem::take(&mut self.canvas.content);
        if !content.trim().is_empty() {
            self.pages.push(content);
        }
    }

    fn new_page(&mut self) {
        self.draw_footer();
        self.push_page();
        self.canvas = PdfCanvas::new();
        self.y = self.height - self.margin;
    }

    fn ensure_space(&mut self, required_height: f32) {
        if self.y - required_height < self.margin + 24.0 {
            self.new_page();
        }
    }

    fn draw_header(&mut self, invoice: &InvoiceWithDetails, business: &BusinessProfile) {
        let teal = (0.125, 0.502, 0.553);
        let dark = (0.10, 0.10, 0.10);
        let muted = (0.34, 0.34, 0.34);
        let inv_num = invoice.invoice_number.as_deref().unwrap_or("DRAFT");
        let business_name = if business.name.trim().is_empty() {
            "900Invoice"
        } else {
            business.name.as_str()
        };

        self.canvas
            .text(self.margin, self.y, 18.0, business_name, "F2", teal);

        let mut left_y = self.y - 18.0;
        for line in compact_lines(&[
            business.address.as_str(),
            business.city.as_str(),
            business.country.as_str(),
            business.phone.as_str(),
            business.email.as_str(),
            business.tax_id.as_str(),
        ]) {
            self.canvas
                .text(self.margin, left_y, 9.0, &line, "F1", muted);
            left_y -= 12.0;
        }

        let right = self.width - self.margin;
        self.canvas
            .text_right(right, self.y, 26.0, "INVOICE", "F2", teal);
        let meta_y = self.y - 22.0;
        self.canvas.text_right(
            right,
            meta_y,
            10.0,
            &format!("Invoice #: {inv_num}"),
            "F2",
            dark,
        );
        self.canvas.text_right(
            right,
            meta_y - 14.0,
            9.0,
            &format!("Date: {}", invoice.issue_date),
            "F1",
            muted,
        );
        self.canvas.text_right(
            right,
            meta_y - 28.0,
            9.0,
            &format!("Due: {}", invoice.due_date),
            "F1",
            muted,
        );
        self.canvas.text_right(
            right,
            meta_y - 42.0,
            9.0,
            &invoice.status.to_uppercase(),
            "F2",
            teal,
        );

        self.y = left_y.min(meta_y - 62.0);
        self.canvas.line(
            self.margin,
            self.y,
            self.width - self.margin,
            self.y,
            teal,
            1.6,
        );
        self.y -= 24.0;
    }

    fn draw_bill_to(&mut self, invoice: &InvoiceWithDetails) {
        self.ensure_space(86.0);
        let teal = (0.125, 0.502, 0.553);
        let dark = (0.10, 0.10, 0.10);
        let muted = (0.34, 0.34, 0.34);
        let client = invoice.client.as_ref();
        let client_name = client.map(|c| c.name.as_str()).unwrap_or("Unknown Client");

        self.canvas
            .text(self.margin, self.y, 9.0, "BILL TO", "F2", teal);
        self.y -= 15.0;
        self.canvas
            .text(self.margin, self.y, 12.0, client_name, "F2", dark);
        self.y -= 14.0;

        let lines = if let Some(client) = client {
            compact_lines(&[
                client.address.as_str(),
                client.city.as_str(),
                client.country.as_str(),
                client.email.as_str(),
                client.phone.as_str(),
                client.tax_id.as_str(),
            ])
        } else {
            Vec::new()
        };
        for line in lines {
            self.canvas
                .text(self.margin, self.y, 9.0, &line, "F1", muted);
            self.y -= 12.0;
        }
        self.y -= 12.0;
    }

    fn draw_line_items(&mut self, invoice: &InvoiceWithDetails) {
        let mut items: Vec<_> = invoice.line_items.iter().collect();
        items.sort_by_key(|item| item.sort_order);
        self.ensure_space(48.0);
        self.draw_table_header();

        let mut shaded = false;
        for (idx, item) in items.iter().enumerate() {
            let desc_width = self.description_column_width();
            let desc_lines = wrap_pdf_text(&item.description, (desc_width / 4.6) as usize);
            let row_h = (desc_lines.len() as f32 * 10.0 + 12.0).max(24.0);
            if self.y - row_h < self.margin + 90.0 {
                self.new_page();
                let inv_num = invoice.invoice_number.as_deref().unwrap_or("DRAFT");
                self.canvas.text(
                    self.margin,
                    self.y,
                    10.0,
                    &format!("Invoice {inv_num} continued"),
                    "F2",
                    (0.34, 0.34, 0.34),
                );
                self.y -= 20.0;
                self.draw_table_header();
            }
            self.draw_item_row(
                idx + 1,
                item,
                &desc_lines,
                row_h,
                &invoice.currency_code,
                shaded,
            );
            shaded = !shaded;
        }
        self.y -= 14.0;
    }

    fn draw_table_header(&mut self) {
        let x = self.margin;
        let y = self.y;
        let w = self.width - self.margin * 2.0;
        self.canvas
            .rect_fill(x, y - 18.0, w, 18.0, (0.125, 0.502, 0.553));
        self.canvas
            .text(x + 6.0, y - 12.0, 8.0, "#", "F2", (1.0, 1.0, 1.0));
        self.canvas.text(
            x + 30.0,
            y - 12.0,
            8.0,
            "Description",
            "F2",
            (1.0, 1.0, 1.0),
        );
        let cols = self.columns();
        self.canvas
            .text_right(cols.qty_right, y - 12.0, 8.0, "Qty", "F2", (1.0, 1.0, 1.0));
        self.canvas.text_right(
            cols.unit_right,
            y - 12.0,
            8.0,
            "Unit",
            "F2",
            (1.0, 1.0, 1.0),
        );
        self.canvas
            .text_right(cols.tax_right, y - 12.0, 8.0, "Tax", "F2", (1.0, 1.0, 1.0));
        self.canvas.text_right(
            cols.amount_right,
            y - 12.0,
            8.0,
            "Amount",
            "F2",
            (1.0, 1.0, 1.0),
        );
        self.y -= 18.0;
    }

    fn draw_item_row(
        &mut self,
        row_num: usize,
        item: &crate::models::line_item::LineItem,
        desc_lines: &[String],
        row_h: f32,
        currency: &str,
        shaded: bool,
    ) {
        let x = self.margin;
        let y_top = self.y;
        let w = self.width - self.margin * 2.0;
        if shaded {
            self.canvas
                .rect_fill(x, y_top - row_h, w, row_h, (0.969, 0.980, 0.984));
        }
        self.canvas.line(
            x,
            y_top - row_h,
            x + w,
            y_top - row_h,
            (0.86, 0.86, 0.86),
            0.5,
        );

        let cols = self.columns();
        let row_y = y_top - 14.0;
        self.canvas.text(
            x + 6.0,
            row_y,
            8.0,
            &row_num.to_string(),
            "F1",
            (0.45, 0.45, 0.45),
        );
        for (line_idx, line) in desc_lines.iter().enumerate() {
            self.canvas.text(
                x + 30.0,
                row_y - line_idx as f32 * 10.0,
                8.5,
                line,
                if line_idx == 0 { "F2" } else { "F1" },
                (0.10, 0.10, 0.10),
            );
        }
        self.canvas.text_right(
            cols.qty_right,
            row_y,
            8.0,
            &format_quantity(item.quantity),
            "F1",
            (0.10, 0.10, 0.10),
        );
        self.canvas.text_right(
            cols.unit_right,
            row_y,
            8.0,
            &format_currency_pdf(item.unit_price_minor, currency),
            "F1",
            (0.10, 0.10, 0.10),
        );
        self.canvas.text_right(
            cols.tax_right,
            row_y,
            8.0,
            &format_rate_bps(item.tax_rate_bps),
            "F1",
            (0.10, 0.10, 0.10),
        );
        self.canvas.text_right(
            cols.amount_right,
            row_y,
            8.0,
            &format_currency_pdf(item.line_total_minor, currency),
            "F1",
            (0.10, 0.10, 0.10),
        );
        self.y -= row_h;
    }

    fn draw_totals(&mut self, invoice: &InvoiceWithDetails) {
        let total_rows = 3 + invoice.taxes.len() + usize::from(invoice.amount_paid_minor > 0);
        self.ensure_space(total_rows as f32 * 16.0 + 38.0);
        let totals = PdfTotals {
            x: self.width - self.margin - 230.0,
            right: self.width - self.margin,
        };
        let mut y = self.y;
        let currency = invoice.currency_code.as_str();

        self.draw_total_row(
            &totals,
            y,
            "Subtotal",
            invoice.subtotal_minor,
            currency,
            false,
        );
        y -= 16.0;
        if invoice.discount_minor > 0 {
            self.draw_total_row(
                &totals,
                y,
                "Discount",
                -invoice.discount_minor,
                currency,
                false,
            );
            y -= 16.0;
        }
        for tax in &invoice.taxes {
            let label = format!("{} ({})", tax.tax_name, format_rate_bps(tax.tax_rate_bps));
            let amount = if tax.is_withholding {
                -tax.tax_amount_minor
            } else {
                tax.tax_amount_minor
            };
            self.draw_total_row(&totals, y, &label, amount, currency, false);
            y -= 16.0;
        }

        self.canvas.line(
            totals.x,
            y + 5.0,
            totals.right,
            y + 5.0,
            (0.125, 0.502, 0.553),
            1.2,
        );
        self.draw_total_row(
            &totals,
            y - 8.0,
            "Total",
            invoice.total_minor,
            currency,
            true,
        );
        y -= 28.0;
        if invoice.amount_paid_minor > 0 {
            self.draw_total_row(
                &totals,
                y,
                "Amount Paid",
                -invoice.amount_paid_minor,
                currency,
                false,
            );
            y -= 16.0;
            let balance = invoice
                .total_minor
                .saturating_sub(invoice.amount_paid_minor);
            self.draw_total_row(&totals, y, "Balance Due", balance, currency, true);
            y -= 18.0;
        }
        self.y = y - 10.0;
    }

    fn draw_total_row(
        &mut self,
        totals: &PdfTotals,
        y: f32,
        label: &str,
        amount: i64,
        currency: &str,
        bold: bool,
    ) {
        let font = if bold { "F2" } else { "F1" };
        let size = if bold { 10.5 } else { 8.5 };
        let color = if bold {
            (0.125, 0.502, 0.553)
        } else if amount < 0 {
            (0.70, 0.10, 0.10)
        } else {
            (0.18, 0.18, 0.18)
        };
        self.canvas
            .text(totals.x, y, size, label, font, (0.26, 0.26, 0.26));
        self.canvas.text_right(
            totals.right,
            y,
            size,
            &format_currency_pdf(amount, currency),
            font,
            color,
        );
    }

    fn draw_payment_details(&mut self, business: &BusinessProfile) {
        let has_bank = !business.bank_name.trim().is_empty()
            || !business.bank_account_number.trim().is_empty()
            || !business.bank_routing_number.trim().is_empty();
        let has_mobile = !business.mobile_money_number.trim().is_empty();
        if !has_bank && !has_mobile {
            return;
        }

        self.ensure_space(72.0);
        self.canvas.text(
            self.margin,
            self.y,
            9.0,
            "PAYMENT DETAILS",
            "F2",
            (0.125, 0.502, 0.553),
        );
        self.y -= 15.0;
        if has_bank {
            self.draw_small_line(&format!("Bank: {}", business.bank_name));
            self.draw_small_line(&format!("Account: {}", business.bank_account_number));
            self.draw_small_line(&format!("Routing: {}", business.bank_routing_number));
        }
        if has_mobile {
            self.draw_small_line(&format!(
                "Mobile Money: {} {}",
                business.mobile_money_provider, business.mobile_money_number
            ));
        }
        self.y -= 8.0;
    }

    fn draw_text_section(&mut self, label: &str, text: &str) {
        if text.trim().is_empty() {
            return;
        }
        let lines = wrap_pdf_text(text, 92);
        let label_height = if label.is_empty() { 0.0 } else { 15.0 };
        self.ensure_space(label_height + lines.len() as f32 * 12.0 + 16.0);
        if !label.is_empty() {
            self.canvas.text(
                self.margin,
                self.y,
                9.0,
                &label.to_uppercase(),
                "F2",
                (0.125, 0.502, 0.553),
            );
            self.y -= 15.0;
        }
        for line in lines {
            self.draw_small_line(&line);
        }
        self.y -= 8.0;
    }

    fn draw_small_line(&mut self, text: &str) {
        if text.trim().is_empty() {
            return;
        }
        self.canvas
            .text(self.margin, self.y, 8.5, text, "F1", (0.30, 0.30, 0.30));
        self.y -= 11.0;
    }

    fn draw_footer(&mut self) {
        let page_num = self.pages.len() + 1;
        self.canvas.text(
            self.margin,
            24.0,
            7.5,
            "Generated by 900Invoice",
            "F1",
            (0.55, 0.55, 0.55),
        );
        self.canvas.text_right(
            self.width - self.margin,
            24.0,
            7.5,
            &format!("Page {page_num}"),
            "F1",
            (0.55, 0.55, 0.55),
        );
    }

    fn description_column_width(&self) -> f32 {
        let cols = self.columns();
        cols.qty_right - (self.margin + 38.0)
    }

    fn columns(&self) -> PdfColumns {
        let right = self.width - self.margin;
        PdfColumns {
            qty_right: right - 200.0,
            unit_right: right - 120.0,
            tax_right: right - 72.0,
            amount_right: right - 6.0,
        }
    }
}

struct PdfColumns {
    qty_right: f32,
    unit_right: f32,
    tax_right: f32,
    amount_right: f32,
}

struct PdfTotals {
    x: f32,
    right: f32,
}

#[derive(Default)]
struct PdfCanvas {
    content: String,
}

impl PdfCanvas {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn text(&mut self, x: f32, y: f32, size: f32, text: &str, font: &str, color: (f32, f32, f32)) {
        if text.trim().is_empty() {
            return;
        }
        self.content.push_str(&format!(
            "{:.3} {:.3} {:.3} rg BT /{} {:.2} Tf 1 0 0 1 {:.2} {:.2} Tm ({}) Tj ET\n",
            color.0,
            color.1,
            color.2,
            font,
            size,
            x,
            y,
            pdf_escape_text(text),
        ));
    }

    fn text_right(
        &mut self,
        right: f32,
        y: f32,
        size: f32,
        text: &str,
        font: &str,
        color: (f32, f32, f32),
    ) {
        let width = estimate_text_width(text, size);
        self.text(right - width, y, size, text, font, color);
    }

    fn rect_fill(&mut self, x: f32, y: f32, w: f32, h: f32, color: (f32, f32, f32)) {
        self.content.push_str(&format!(
            "{:.3} {:.3} {:.3} rg {:.2} {:.2} {:.2} {:.2} re f\n",
            color.0, color.1, color.2, x, y, w, h
        ));
    }

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: (f32, f32, f32), width: f32) {
        self.content.push_str(&format!(
            "{:.3} {:.3} {:.3} RG {:.2} w {:.2} {:.2} m {:.2} {:.2} l S\n",
            color.0, color.1, color.2, width, x1, y1, x2, y2
        ));
    }
}

fn compact_lines(parts: &[&str]) -> Vec<String> {
    parts
        .iter()
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn format_currency_pdf(amount_minor: i64, currency_code: &str) -> String {
    let cfg = currency_config(currency_code);
    let negative = amount_minor < 0;
    let abs_amount = amount_minor.unsigned_abs() as i64;
    let (whole, frac): (i64, i64) = if cfg.decimals == 0 {
        (abs_amount, 0)
    } else {
        let divisor = 10i64.pow(cfg.decimals);
        (abs_amount / divisor, abs_amount % divisor)
    };
    let whole_str = format_with_thousands(whole, cfg.thousands_sep);
    let number = if cfg.decimals > 0 {
        format!(
            "{}{}{:0>width$}",
            whole_str,
            cfg.decimal_sep,
            frac,
            width = cfg.decimals as usize
        )
    } else {
        whole_str
    };
    if negative {
        format!("-{} {}", currency_code, number)
    } else {
        format!("{} {}", currency_code, number)
    }
}

fn wrap_pdf_text(input: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(8);
    let mut lines = Vec::new();
    for raw_line in sanitize_pdf_text(input).split('\n') {
        let mut current = String::new();
        for word in raw_line.split_whitespace() {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= max_chars {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current);
                current = word.to_string();
            }
            while current.len() > max_chars {
                let rest = current.split_off(max_chars);
                lines.push(current);
                current = rest;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn estimate_text_width(text: &str, size: f32) -> f32 {
    sanitize_pdf_text(text).chars().count() as f32 * size * 0.50
}

fn pdf_escape_text(input: &str) -> String {
    let mut escaped = String::new();
    for ch in sanitize_pdf_text(input).chars() {
        match ch {
            '(' => escaped.push_str("\\("),
            ')' => escaped.push_str("\\)"),
            '\\' => escaped.push_str("\\\\"),
            '\n' | '\r' | '\t' => escaped.push(' '),
            c if c.is_ascii() && !c.is_control() => escaped.push(c),
            _ => escaped.push('?'),
        }
    }
    escaped
}

fn sanitize_pdf_text(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '\r' => {}
            '\n' => out.push('\n'),
            '\t' => out.push(' '),
            '\u{2013}' | '\u{2014}' | '\u{2212}' => out.push('-'),
            '\u{2018}' | '\u{2019}' => out.push('\''),
            '\u{201C}' | '\u{201D}' => out.push('"'),
            '\u{20A6}' => out.push_str("NGN"),
            '\u{20B9}' => out.push_str("INR"),
            '\u{20B5}' => out.push_str("GHS"),
            '\u{20AC}' => out.push_str("EUR"),
            c if c.is_ascii() && !c.is_control() => out.push(c),
            c if c.is_whitespace() => out.push(' '),
            _ => out.push('?'),
        }
    }
    out
}

fn build_pdf_document(pages: &[String], width: f32, height: f32) -> Vec<u8> {
    use std::io::Write;

    let page_count = pages.len().max(1);
    let mut objects = Vec::with_capacity(4 + page_count * 2);
    let page_object_ids: Vec<usize> = (0..page_count).map(|idx| 5 + idx * 2).collect();
    let kids = page_object_ids
        .iter()
        .map(|id| format!("{id} 0 R"))
        .collect::<Vec<_>>()
        .join(" ");

    objects.push("<< /Type /Catalog /Pages 2 0 R >>".to_string());
    objects.push(format!(
        "<< /Type /Pages /Kids [{}] /Count {} >>",
        kids, page_count
    ));
    objects.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string());
    objects.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>".to_string());

    for (idx, page_id) in page_object_ids.iter().enumerate() {
        let content_id = page_id + 1;
        objects.push(format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {:.2} {:.2}] /Resources << /Font << /F1 3 0 R /F2 4 0 R >> >> /Contents {} 0 R >>",
            width, height, content_id
        ));
        let stream = pages.get(idx).map(String::as_str).unwrap_or("");
        objects.push(format!(
            "<< /Length {} >>\nstream\n{}endstream",
            stream.len(),
            stream
        ));
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");
    let mut offsets = Vec::with_capacity(objects.len() + 1);
    offsets.push(0usize);
    for (idx, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        writeln!(out, "{} 0 obj", idx + 1).expect("write pdf object header");
        writeln!(out, "{}", object).expect("write pdf object");
        writeln!(out, "endobj").expect("write pdf object footer");
    }

    let xref_offset = out.len();
    writeln!(out, "xref").expect("write xref");
    writeln!(out, "0 {}", objects.len() + 1).expect("write xref size");
    writeln!(out, "0000000000 65535 f ").expect("write xref free");
    for offset in offsets.iter().skip(1) {
        writeln!(out, "{:010} 00000 n ", offset).expect("write xref row");
    }
    writeln!(out, "trailer").expect("write trailer");
    writeln!(out, "<< /Size {} /Root 1 0 R >>", objects.len() + 1).expect("write trailer body");
    writeln!(out, "startxref").expect("write startxref");
    writeln!(out, "{}", xref_offset).expect("write xref offset");
    writeln!(out, "%%EOF").expect("write eof");
    out
}

// ---------------------------------------------------------------------------
// Preview JSON data
// ---------------------------------------------------------------------------

/// Generate JSON preview data for frontend rendering.
pub fn get_preview_data(invoice: &InvoiceWithDetails, business: &BusinessProfile) -> Value {
    let currency = &invoice.currency_code;
    let balance = invoice
        .total_minor
        .saturating_sub(invoice.amount_paid_minor);

    let line_items: Vec<Value> = invoice
        .line_items
        .iter()
        .map(|item| {
            json!({
                "id": item.id,
                "description": item.description,
                "quantity": format_quantity(item.quantity),
                "quantity_raw": item.quantity,
                "unit_price": format_currency_html(item.unit_price_minor, currency),
                "unit_price_minor": item.unit_price_minor,
                "tax_rate": format_rate_bps(item.tax_rate_bps),
                "tax_rate_bps": item.tax_rate_bps,
                "discount": format_rate_bps(item.discount_bps),
                "discount_bps": item.discount_bps,
                "line_total": format_currency_html(item.line_total_minor, currency),
                "line_total_minor": item.line_total_minor,
                "sort_order": item.sort_order,
            })
        })
        .collect();

    let taxes: Vec<Value> = invoice
        .taxes
        .iter()
        .map(|t| {
            json!({
                "name": t.tax_name,
                "rate": format_rate_bps(t.tax_rate_bps),
                "rate_bps": t.tax_rate_bps,
                "amount": format_currency_html(t.tax_amount_minor, currency),
                "amount_minor": t.tax_amount_minor,
                "is_withholding": t.is_withholding,
            })
        })
        .collect();

    let payments: Vec<Value> = invoice
        .payments
        .iter()
        .map(|p| {
            json!({
                "amount": format_currency_html(p.amount_minor, &p.currency_code),
                "amount_minor": p.amount_minor,
                "currency_code": p.currency_code,
                "payment_method": p.payment_method,
                "paid_at": p.paid_at,
            })
        })
        .collect();

    let client_name = invoice
        .client
        .as_ref()
        .map(|c| c.name.clone())
        .unwrap_or_default();
    let client_email = invoice
        .client
        .as_ref()
        .map(|c| c.email.clone())
        .unwrap_or_default();
    let client_phone = invoice
        .client
        .as_ref()
        .map(|c| c.phone.clone())
        .unwrap_or_default();
    let client_address = invoice
        .client
        .as_ref()
        .map(|c| c.address.clone())
        .unwrap_or_default();
    let client_city = invoice
        .client
        .as_ref()
        .map(|c| c.city.clone())
        .unwrap_or_default();
    let client_country = invoice
        .client
        .as_ref()
        .map(|c| c.country.clone())
        .unwrap_or_default();
    let client_tax_id = invoice
        .client
        .as_ref()
        .map(|c| c.tax_id.clone())
        .unwrap_or_default();

    json!({
        "invoice": {
            "id": invoice.id,
            "invoice_number": invoice.invoice_number,
            "status": invoice.status,
            "currency_code": currency,
            "issue_date": invoice.issue_date,
            "due_date": invoice.due_date,
            "uses_inclusive_taxes": invoice.uses_inclusive_taxes,
            "notes": invoice.notes,
            "terms": invoice.terms,
            "subtotal": format_currency_html(invoice.subtotal_minor, currency),
            "subtotal_minor": invoice.subtotal_minor,
            "discount": format_currency_html(invoice.discount_minor, currency),
            "discount_minor": invoice.discount_minor,
            "tax_amount": format_currency_html(invoice.tax_amount_minor, currency),
            "tax_amount_minor": invoice.tax_amount_minor,
            "total": format_currency_html(invoice.total_minor, currency),
            "total_minor": invoice.total_minor,
            "amount_paid": format_currency_html(invoice.amount_paid_minor, currency),
            "amount_paid_minor": invoice.amount_paid_minor,
            "balance_due": format_currency_html(balance, currency),
            "balance_due_minor": balance,
        },
        "client": {
            "name": client_name,
            "email": client_email,
            "phone": client_phone,
            "address": client_address,
            "city": client_city,
            "country": client_country,
            "tax_id": client_tax_id,
        },
        "business": {
            "name": business.name,
            "address": business.address,
            "city": business.city,
            "country": business.country,
            "phone": business.phone,
            "email": business.email,
            "website": business.website,
            "tax_id": business.tax_id,
            "default_currency": business.default_currency,
            "bank_name": business.bank_name,
            "bank_account_number": business.bank_account_number,
            "bank_routing_number": business.bank_routing_number,
            "mobile_money_number": business.mobile_money_number,
            "mobile_money_provider": business.mobile_money_provider,
        },
        "line_items": line_items,
        "taxes": taxes,
        "payments": payments,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::client::Client;
    use crate::models::invoice::InvoiceWithDetails;
    use crate::models::line_item::LineItem;
    use crate::models::tax::InvoiceTax;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file_path(suffix: &str) -> std::path::PathBuf {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("900invoice_pdf_engine_test_{}_{}", ts, suffix))
    }

    fn business_fixture() -> BusinessProfile {
        BusinessProfile {
            id: "business-1".to_string(),
            name: "Acme Studio".to_string(),
            address: "1 Market Street".to_string(),
            city: "Nairobi".to_string(),
            country: "Kenya".to_string(),
            country_code: "KE".to_string(),
            phone: "+254 700 000000".to_string(),
            email: "billing@example.com".to_string(),
            website: "example.com".to_string(),
            tax_id: "P051234567A".to_string(),
            logo_path: None,
            default_currency: "KES".to_string(),
            default_payment_terms_days: 30,
            bank_name: "Equity Bank".to_string(),
            bank_account_number: "123456789".to_string(),
            bank_routing_number: "EQBLKENA".to_string(),
            mobile_money_number: "0700000000".to_string(),
            mobile_money_provider: "M-Pesa".to_string(),
            created_at: "2026-05-11T00:00:00".to_string(),
            updated_at: "2026-05-11T00:00:00".to_string(),
        }
    }

    fn invoice_fixture() -> InvoiceWithDetails {
        InvoiceWithDetails {
            id: "invoice-1".to_string(),
            invoice_number: Some("INV-2026-0001".to_string()),
            client_id: "client-1".to_string(),
            client: Some(Client {
                id: "client-1".to_string(),
                name: "Globex Kenya".to_string(),
                email: "ap@globex.example".to_string(),
                phone: "+254 711 000000".to_string(),
                address: "2 Client Road".to_string(),
                city: "Nairobi".to_string(),
                country: "Kenya".to_string(),
                country_code: "KE".to_string(),
                tax_id: "P009876543B".to_string(),
                currency_code: "KES".to_string(),
                payment_terms_days: 30,
                notes: String::new(),
                created_at: "2026-05-11T00:00:00".to_string(),
                updated_at: "2026-05-11T00:00:00".to_string(),
            }),
            status: "sent".to_string(),
            currency_code: "KES".to_string(),
            subtotal_minor: 100_000,
            discount_minor: 0,
            tax_amount_minor: 16_000,
            total_minor: 116_000,
            amount_paid_minor: 20_000,
            exchange_rate_to_usd: None,
            exchange_rate_date: None,
            issue_date: "2026-05-11".to_string(),
            due_date: "2026-06-10".to_string(),
            uses_inclusive_taxes: false,
            notes: "Thank you for your business.".to_string(),
            terms: "Payment due within 30 days.".to_string(),
            footer: "Acme Studio".to_string(),
            created_at: "2026-05-11T00:00:00".to_string(),
            updated_at: "2026-05-11T00:00:00".to_string(),
            finalized_at: None,
            sent_at: None,
            paid_at: None,
            voided_at: None,
            line_items: vec![LineItem {
                id: "line-1".to_string(),
                invoice_id: "invoice-1".to_string(),
                product_id: None,
                tax_rate_id: Some("tax-ke-vat".to_string()),
                description: "Monthly advisory retainer".to_string(),
                quantity: 100,
                unit_price_minor: 100_000,
                tax_rate_bps: 1600,
                discount_bps: 0,
                line_total_minor: 100_000,
                sort_order: 0,
                created_at: "2026-05-11T00:00:00".to_string(),
            }],
            taxes: vec![InvoiceTax {
                id: "tax-1".to_string(),
                invoice_id: "invoice-1".to_string(),
                tax_rate_id: Some("vat-ke".to_string()),
                tax_name: "VAT".to_string(),
                tax_rate_bps: 1600,
                tax_amount_minor: 16_000,
                is_withholding: false,
                created_at: "2026-05-11T00:00:00".to_string(),
            }],
            payments: Vec::new(),
        }
    }

    #[test]
    fn test_format_currency_kes() {
        assert_eq!(format_currency_html(150_000, "KES"), "KSh 1,500.00");
    }

    #[test]
    fn test_format_currency_ngn() {
        assert_eq!(format_currency_html(150_000, "NGN"), "\u{20A6}1,500.00");
    }

    #[test]
    fn test_format_currency_ugx_no_decimals() {
        assert_eq!(format_currency_html(1_500, "UGX"), "USh 1,500");
    }

    #[test]
    fn test_format_currency_xof_no_decimals() {
        assert_eq!(format_currency_html(150_000, "XOF"), "CFA 150,000");
    }

    #[test]
    fn test_format_currency_usd() {
        assert_eq!(format_currency_html(100, "USD"), "$1.00");
    }

    #[test]
    fn test_format_quantity() {
        assert_eq!(format_quantity(100), "1");
        assert_eq!(format_quantity(150), "1.50");
        assert_eq!(format_quantity(333), "3.33");
    }

    #[test]
    fn test_format_rate_bps() {
        assert_eq!(format_rate_bps(1600), "16%");
        assert_eq!(format_rate_bps(750), "7.5%");
        assert_eq!(format_rate_bps(0), "0%");
        assert_eq!(format_rate_bps(250), "2.5%");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("AT&T <Inc>"), "AT&amp;T &lt;Inc&gt;");
    }

    #[test]
    fn test_generate_invoice_pdf_bytes_returns_real_pdf() {
        let invoice = invoice_fixture();
        let business = business_fixture();
        let pdf = generate_invoice_pdf_bytes(&invoice, &business, "a4", "en");
        let text = String::from_utf8_lossy(&pdf);

        assert!(pdf.starts_with(b"%PDF-1.4"));
        assert!(text.contains("/Type /Page"));
        assert!(text.contains("Invoice #: INV-2026-0001"));
        assert!(text.contains("Monthly advisory retainer"));
        assert!(text.contains("%%EOF"));
        assert!(!text.contains("<html"));
    }

    #[test]
    fn test_pdf_text_escapes_literal_delimiters() {
        assert_eq!(
            pdf_escape_text("Acme (Ops) \\ Billing"),
            "Acme \\(Ops\\) \\\\ Billing"
        );
    }

    #[test]
    fn test_load_logo_base64_rejects_unsupported_extension() {
        let path = temp_file_path("bad.txt");
        fs::write(&path, b"not-an-image").expect("write test file");
        let got = load_logo_base64(path.to_str().expect("utf-8 path"));
        assert!(got.is_none());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_load_logo_base64_rejects_oversized_file() {
        let path = temp_file_path("large.png");
        let oversized = vec![0u8; 2 * 1024 * 1024 + 1];
        fs::write(&path, oversized).expect("write oversized test file");
        let got = load_logo_base64(path.to_str().expect("utf-8 path"));
        assert!(got.is_none());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_load_logo_base64_accepts_allowed_extension_and_size() {
        let path = temp_file_path("ok.png");
        fs::write(&path, b"\x89PNG\r\n\x1a\n").expect("write png header");
        let got = load_logo_base64(path.to_str().expect("utf-8 path"));
        assert!(got.is_some());
        let uri = got.expect("data uri");
        assert!(uri.starts_with("data:image/png;base64,"));
        let _ = fs::remove_file(path);
    }
}
