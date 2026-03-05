# Invoice Template Customization

900Invoice generates professional PDF invoices using [Typst](https://typst.app) via the `typst-bake` engine. The invoice template is a Typst source file that you can customize to match your brand.

---

## Template Location

The invoice template is located at:

```
src-tauri/src/templates/invoice.typ
```

This file is embedded into the binary at compile time using `include_str!()` in Rust. To apply your customizations, you must rebuild the application:

```bash
cargo tauri build
```

For development, run `cargo tauri dev` — template changes will trigger a Rust recompile and the updated template will be used for all subsequent PDF generation.

---

## Template Variables

The Rust backend substitutes the following variables into the template before compiling it with Typst. All variable names use double curly braces: `{{variable_name}}`.

### Business Variables

| Variable | Type | Description |
|----------|------|-------------|
| `{{business_name}}` | string | Your company/business name |
| `{{business_address}}` | string | Business street address |
| `{{business_city}}` | string | City, state/province |
| `{{business_country}}` | string | Country name |
| `{{business_tax_id}}` | string | VAT/GST/tax registration number |
| `{{business_email}}` | string | Business email address |
| `{{business_phone}}` | string | Business phone number |
| `{{business_website}}` | string | Website URL |
| `{{business_logo_path}}` | string | Absolute path to logo image file |
| `{{business_has_logo}}` | bool | `true` if a logo is configured |

### Invoice Header Variables

| Variable | Type | Description |
|----------|------|-------------|
| `{{invoice_number}}` | string | Invoice number (e.g., `INV-2026-0042`) |
| `{{invoice_status}}` | string | Status label (`DRAFT`, `FINALIZED`, etc.) |
| `{{invoice_date}}` | string | Issue date (formatted for locale) |
| `{{invoice_due_date}}` | string | Due date, or empty string if not set |
| `{{invoice_currency}}` | string | ISO 4217 currency code |
| `{{invoice_notes}}` | string | Additional notes |
| `{{invoice_terms}}` | string | Payment terms text |

### Client Variables

| Variable | Type | Description |
|----------|------|-------------|
| `{{client_name}}` | string | Client name or company name |
| `{{client_address}}` | string | Client street address |
| `{{client_city}}` | string | Client city |
| `{{client_country}}` | string | Client country name |
| `{{client_tax_id}}` | string | Client's VAT/tax number |
| `{{client_email}}` | string | Client email address |

### Totals

| Variable | Type | Description |
|----------|------|-------------|
| `{{subtotal}}` | string | Formatted subtotal (e.g., `KES 10,000.00`) |
| `{{tax_lines}}` | string | Rendered tax breakdown (Typst markup) |
| `{{total}}` | string | Formatted grand total |
| `{{amount_paid}}` | string | Formatted total payments received |
| `{{amount_due}}` | string | Formatted balance due |

### Line Items Table

The line items table is rendered as a Typst `table()` block and substituted as `{{line_items_table}}`. Each row contains:
- Description
- Quantity
- Unit price (formatted)
- Line total (formatted)

If you need to change the table column layout, you will need to modify the template rendering logic in `src-tauri/src/services/pdf.rs` as well as the template.

---

## Changing Branding and Colors

The template uses Typst's variable and function system. Here is a minimal example showing how to change the accent color and header layout:

```typst
// Define your brand colors at the top of the file
#let accent-color = rgb("#1a56db")     // Change to your brand color
#let text-color = rgb("#111827")
#let muted-color = rgb("#6b7280")
#let border-color = rgb("#e5e7eb")

// Use them throughout the template
#set text(font: "Helvetica Neue", fill: text-color)

// Header with logo
#block(
  fill: accent-color,
  width: 100%,
  inset: (x: 24pt, y: 16pt),
)[
  #text(fill: white, size: 20pt, weight: "bold")[{{business_name}}]
]
```

### Supported Fonts

Typst can use fonts that are installed on your system. To use a custom font:

1. Install the font on your system (Windows: double-click the .ttf/.otf file; macOS: Font Book; Linux: copy to `~/.local/share/fonts/`)
2. Reference it in the template: `#set text(font: "Your Font Name")`
3. Rebuild the application

For distribution, be aware that users must also have the font installed, or you must bundle the font file and load it explicitly.

---

## Paper Size Configuration

Change the paper size at the top of the template:

```typst
// A4 (international standard — recommended)
#set page(paper: "a4", margin: (x: 2.5cm, y: 2cm))

// US Letter (North America)
#set page(paper: "us-letter", margin: (x: 1in, y: 0.75in))

// A5 (compact)
#set page(paper: "a5", margin: (x: 1.5cm, y: 1.5cm))
```

Typst supports all standard paper sizes. See the [Typst page documentation](https://typst.app/docs/reference/layout/page/) for the full list.

---

## Adding Custom Fields

If you need to add custom fields to the invoice (e.g., purchase order number, project code, bank account details), you have two options:

### Option 1: Use the Notes and Terms Fields

The simplest approach. The `notes` and `terms` fields on each invoice support multi-line text and are displayed in the PDF. Use them for:
- Bank account details for payment
- Purchase order references
- Project codes
- Custom disclaimers

### Option 2: Add a Database Field and Template Variable

For fields that should be structured and queryable:

1. **Add a column to the database schema** in `src-tauri/src/db/schema.rs`:
   ```sql
   ALTER TABLE invoices ADD COLUMN po_number TEXT;
   ```

2. **Add a migration** in `src-tauri/src/db/migrations.rs` with the next version number.

3. **Update the `Invoice` model** in `src-tauri/src/models/invoice.rs` to include the field.

4. **Update the `CreateInvoiceInput`** struct and the `create_invoice` command to accept the new field.

5. **Add the template variable** in `src-tauri/src/services/pdf.rs`:
   ```rust
   template = template.replace("{{po_number}}", &invoice.po_number.unwrap_or_default());
   ```

6. **Add to the template** in `invoice.typ`:
   ```typst
   #if "{{po_number}}" != "" [
     *PO Number:* {{po_number}}
   ]
   ```

7. **Update the frontend** to include the field in the invoice form.

---

## Example: Minimal Template

Here is a minimal working template as a starting point for heavy customization:

```typst
#set page(paper: "a4", margin: (x: 2.5cm, y: 2cm))
#set text(font: "Helvetica Neue", size: 10pt, fill: rgb("#111827"))

// Header
= INVOICE

*{{business_name}}*
{{business_address}}, {{business_city}}

---

#grid(
  columns: (1fr, 1fr),
  [
    *Bill To:*
    {{client_name}}
    {{client_address}}
    {{client_city}}
  ],
  [
    *Invoice:* {{invoice_number}} \
    *Date:* {{invoice_date}} \
    *Due:* {{invoice_due_date}}
  ]
)

---

{{line_items_table}}

---

#align(right)[
  *Subtotal:* {{subtotal}} \
  {{tax_lines}}
  *Total:* {{total}} \
  *Amount Due:* {{amount_due}}
]

#if "{{invoice_notes}}" != "" [
  ---
  *Notes:* {{invoice_notes}}
]
```

---

## Testing Your Template

After editing `invoice.typ`, test it by:

1. Running `cargo tauri dev`
2. Opening a finalized invoice in the application
3. Clicking "Generate PDF" / "Preview PDF"
4. The PDF will open in your system's PDF viewer

If there is a Typst syntax error, the error message will be displayed in the application's error notification and logged to the console.

---

## Contributing Templates

If you create a template suited for a specific region or industry (e.g., a template compliant with Nigerian FIRS invoice requirements, or a template with MPESA payment QR code), please consider contributing it to the project. Place additional templates in `src-tauri/src/templates/` with descriptive names (e.g., `invoice-ng-firs.typ`) and open a PR.
