# PDF Rendering

900Invoice renders invoices through `src-tauri/src/services/pdf_engine.rs`.

The engine has three public surfaces:

1. `generate_invoice_html(...)` builds the rich HTML invoice used for WebView preview and browser print workflows.
2. `get_preview_data(...)` returns structured JSON for frontend preview components.
3. `generate_invoice_pdf_bytes(...)` builds a self-contained PDF document for native file export.

The current PDF export path is dependency-free: it writes PDF objects directly from the invoice data and uses built-in PDF fonts. This keeps offline export available without shipping an external browser, PDF converter, or typesetting binary.

---

## User Flow

1. The user opens an invoice preview.
2. The preview modal renders invoice data from the frontend store.
3. Clicking **Download** opens a native save dialog.
4. The frontend invokes `generate_invoice_pdf`.
5. Rust loads the invoice with client, line items, tax rows, and payments.
6. Rust returns base64-encoded PDF bytes.
7. The frontend decodes the bytes and writes the selected `.pdf` file through the Tauri filesystem plugin.

The **Print** button remains available and uses the WebView/browser print path.

---

## Rendered Data

The PDF renderer includes:

| Section | Source |
|---|---|
| Business name and contact details | `BusinessProfile` |
| Invoice number, issue date, due date, status | `InvoiceWithDetails` |
| Client name and contact details | `InvoiceWithDetails.client` |
| Line item description, quantity, unit price, tax, and amount | `InvoiceWithDetails.line_items` |
| Discounts, tax rows, total, paid amount, and balance | Invoice totals and `InvoiceWithDetails.taxes` |
| Bank and mobile money payment details | `BusinessProfile` |
| Notes, terms, and footer | Invoice text fields |

Money is formatted from integer minor units. The native PDF uses ISO currency codes for maximum compatibility with built-in PDF fonts; the HTML preview can use richer currency symbols.

---

## Customization

Invoice rendering is code-driven today. To customize the invoice design, update:

- `generate_invoice_html(...)` for the WebView preview and print styling.
- `generate_invoice_pdf_bytes(...)` and `PdfRenderer` for exported PDF layout.
- `get_preview_data(...)` when the frontend needs additional structured fields.

Common changes:

| Change | Where |
|---|---|
| Accent color | `PdfRenderer::draw_header`, `draw_table_header`, and HTML CSS variables in `generate_invoice_html` |
| Table columns | `PdfRenderer::columns`, `draw_table_header`, `draw_item_row`, and the HTML item table |
| Business/payment fields | `draw_header`, `draw_payment_details`, and `build_payment_html` |
| Notes/terms layout | `draw_text_section` and the HTML notes/terms blocks |
| Paper size behavior | `generate_invoice_pdf_bytes` and `generate_invoice_html` paper-size branches |

---

## Adding Custom Fields

For simple one-off content, use invoice notes, terms, or footer. These fields are persisted and rendered in both the preview and PDF export.

For structured fields such as purchase order number or project code:

1. Add the database column in a migration.
2. Update the relevant Rust model in `src-tauri/src/models/`.
3. Update create/update commands and frontend store adapters.
4. Render the field in `generate_invoice_html(...)`.
5. Render the field in `PdfRenderer`.
6. Add regression coverage for the new field when it affects exported output.

---

## Testing

Use the normal quality gate after changing rendering:

```bash
npm run check
npm run build
CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

The Rust PDF tests assert that native export returns a real `%PDF-1.4` document, includes invoice content, escapes PDF literal strings, and does not return the old HTML payload.

When changing layout, also test manually from the invoice preview modal by saving a PDF and opening it in the system viewer.
