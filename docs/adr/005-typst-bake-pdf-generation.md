# ADR 005: typst-bake for PDF Generation

## Status: Superseded

## Date: 2026-03-01

## Superseded By

Sprint 031 replaced the unimplemented `typst-bake` path with the current dependency-free Rust PDF/HTML renderer in `src-tauri/src/services/pdf_engine.rs`.

The historical decision below is retained for context. It should not be read as the current implementation.

## Context

900Invoice must generate professional PDF invoices. PDF generation is a critical user-facing feature — the invoice PDF is what gets sent to clients, and its quality directly reflects on the user's business.

We evaluated several approaches:

### Option 1: HTML-to-PDF (headless Chromium / wkhtmltopdf)

Generate an HTML/CSS invoice template and convert to PDF using a headless browser.

**Pros:**
- Familiar web technologies (HTML/CSS)
- Easy to customize with existing web skills

**Cons:**
- Headless Chromium adds ~100+ MB to the binary — unacceptable given our size constraints
- wkhtmltopdf uses a deprecated Qt WebKit fork with known rendering bugs
- Significant startup overhead (seconds) for each PDF generation
- CSS rendering for print is notoriously unreliable

### Option 2: LaTeX

Generate LaTeX source and compile to PDF using pdflatex.

**Pros:**
- Typographically excellent output
- Mature ecosystem

**Cons:**
- Full LaTeX installation is 3–6 GB
- Cannot be embedded in the binary
- Complex installation for users
- LaTeX syntax is arcane and difficult to customize

### Option 3: reportlab / fpdf (Python PDF libraries)

**Cons:**
- Requires Python runtime
- Procedural PDF generation is tedious to maintain
- No template system

### Option 4: Typst via typst-bake

[Typst](https://typst.app) is a modern typesetting system designed as a LaTeX alternative. `typst-bake` is a Rust crate that embeds the Typst compiler directly, enabling compile-time template processing.

**Pros:**
- Entire PDF engine embedded in the binary — no external dependencies
- Typst templates are readable and easy to customize (simpler than LaTeX)
- Compilation is extremely fast (milliseconds for a single invoice)
- Excellent typographic quality
- Native support for RTL text, international scripts, custom fonts
- Pure Rust implementation — no FFI, no subprocess spawning

**Cons:**
- Typst is newer than LaTeX (less ecosystem), but sufficient for invoice templates
- Contributors need to learn Typst syntax for template modifications (though it is simpler than LaTeX)
- typst-bake compiles templates at binary build time — template changes require a rebuild (this is acceptable for our use case; users who need custom templates rebuild from source or we provide a community template repository)

## Decision

Use **typst-bake** for PDF generation.

- The invoice template is at `src-tauri/src/templates/invoice.typ`
- Template variables are substituted at runtime before Typst compilation
- The Typst compiler is embedded via typst-bake (no installation required)
- PDF generation takes < 100 ms on typical hardware

**Integration in Rust:**
```rust
// src-tauri/src/services/pdf.rs
use typst_bake::TypstCompiler;

pub fn render_invoice(invoice: &FullInvoice) -> Result<Vec<u8>, PdfError> {
    let template = include_str!("../templates/invoice.typ");
    let rendered = substitute_variables(template, invoice);
    TypstCompiler::new()
        .compile(&rendered)
        .map_err(PdfError::TypstError)
}
```

## Consequences

### Positive
- Zero additional installation requirements for users — PDF works out of the box
- Binary size overhead of typst-bake is ~5 MB (acceptable given we started at 2.5 MB vs Electron's 150+ MB)
- Millisecond PDF generation — users experience it as instant
- Professional typographic output that reflects well on users' businesses
- The Typst template is in `src-tauri/src/templates/` and is modifiable before building
- RTL and international script support is native to Typst

### Negative / Trade-offs
- Template changes require rebuilding the binary (this is the right trade-off for an embedded binary application)
- Contributors who want to modify the invoice design need to learn Typst syntax (simpler than LaTeX; documentation at typst.app)
- The typst-bake crate may lag slightly behind the latest Typst language version

### Template Customization

For documentation on customizing the invoice template, see [TEMPLATES.md](../TEMPLATES.md).

### Community Templates

We encourage contributors to build and share Typst invoice templates for specific regional requirements. For example:
- Nigerian FIRS-compliant invoice format
- South African SARS VAT invoice requirements
- Indian GST invoice format (GSTIN, HSN/SAC codes)
- Kenya KRA ETR-compatible format

These can be contributed as additional template files in `src-tauri/src/templates/` with an in-app template selector (planned for a future release).
