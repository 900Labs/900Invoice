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
}
