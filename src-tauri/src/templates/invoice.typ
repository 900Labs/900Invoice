// 900Invoice — Professional Invoice Template
// Version: 1.0
//
// This template is designed to be compiled via typst-bake at build time,
// embedding the compiled binary into the Rust application.
//
// Data is passed as a JSON file (invoice-data.json) whose schema matches
// the get_preview_data() output from pdf_engine.rs.
//
// Usage:
//   typst compile invoice.typ output.pdf --input invoice-data-path=invoice-data.json
//
// JSON schema expected in invoice-data.json:
// {
//   "invoice": { invoice_number, status, currency_code, issue_date, due_date,
//                subtotal, discount, discount_minor, tax_amount, total,
//                amount_paid, amount_paid_minor, balance_due },
//   "business": { name, address, city, country, phone, email, website, tax_id,
//                 bank_name, bank_account_number, bank_routing_number,
//                 mobile_money_number, mobile_money_provider },
//   "client": { name, email, phone, address, city, country, tax_id },
//   "line_items": [{ description, quantity, unit_price, tax_rate, discount,
//                    line_total, sort_order }],
//   "taxes": [{ name, rate, amount, is_withholding }],
//   "payments": [{ amount, payment_method, paid_at }]
// }

#let data = json("invoice-data.json")
#let inv = data.invoice
#let biz = data.business
#let client = data.client

// ---------------------------------------------------------------------------
// Color palette
// ---------------------------------------------------------------------------
#let teal        = rgb("#20808D")
#let teal-light  = rgb("#E8F5F6")
#let teal-mid    = rgb("#C5E5E8")
#let text-dark   = rgb("#1A1A1A")
#let text-mid    = rgb("#4A4A4A")
#let text-light  = rgb("#888888")
#let border-col  = rgb("#E0E0E0")
#let row-alt     = rgb("#F7FAFB")
#let red-col     = rgb("#DC2626")
#let white-col   = rgb("#FFFFFF")
#let badge-draft = rgb("#E5E7EB")
#let badge-paid  = rgb("#D1FAE5")
#let badge-sent  = rgb("#FEF3C7")
#let badge-void  = rgb("#FEE2E2")

// ---------------------------------------------------------------------------
// Page layout
// ---------------------------------------------------------------------------
#set page(
  paper: "a4",
  margin: (top: 22mm, bottom: 22mm, left: 20mm, right: 20mm),
  footer: context [
    #set text(size: 7.5pt, fill: text-light)
    #grid(
      columns: (1fr, auto, 1fr),
      align: (left, center, right),
      [#inv.invoice_number],
      [],
      [Page #counter(page).display() of #counter(page).final().first()]
    )
  ]
)

#set text(
  font: ("Noto Sans", "Helvetica Neue", "Arial", "Liberation Sans"),
  size: 10pt,
  fill: text-dark,
)

#set par(leading: 0.65em)

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Render a small label above a value (used in invoice meta grid)
#let meta-row(label, value) = {
  text(size: 8pt, fill: text-light)[#label]
  linebreak()
  text(size: 9.5pt, fill: text-dark, weight: "semibold")[#value]
}

/// Status badge (colored box)
#let status-badge(status) = {
  let (bg, fg) = if status == "paid" {
    (badge-paid, rgb("#065F46"))
  } else if status == "sent" {
    (badge-sent, rgb("#92400E"))
  } else if status == "void" {
    (badge-void, rgb("#991B1B"))
  } else if status == "finalized" {
    (rgb("#DBEAFE"), rgb("#1D4ED8"))
  } else {
    (badge-draft, rgb("#374151"))
  }
  box(
    fill: bg,
    radius: 10pt,
    inset: (x: 8pt, y: 2pt),
  )[
    #text(size: 7.5pt, fill: fg, weight: "bold")[#upper(status)]
  ]
}

/// Section heading (teal uppercase label with underline)
#let section-heading(title) = {
  v(8pt)
  text(size: 8pt, weight: "bold", fill: teal)[#upper(title)]
  v(2pt)
  line(length: 100%, stroke: 1pt + teal-mid)
  v(4pt)
}

// ---------------------------------------------------------------------------
// Header — Company + Invoice identity
// ---------------------------------------------------------------------------
#grid(
  columns: (1fr, 1fr),
  gutter: 16pt,
  [
    // Company branding
    #text(size: 20pt, weight: "bold", fill: teal)[#biz.name]
    #v(6pt)
    #text(size: 8.5pt, fill: text-mid)[
      #biz.address \
      #if biz.city != "" and biz.country != "" [
        #biz.city, #biz.country \
      ] else if biz.city != "" [
        #biz.city \
      ] else if biz.country != "" [
        #biz.country \
      ]
      #if biz.phone != "" [Phone: #biz.phone \ ]
      #if biz.email != "" [Email: #biz.email \ ]
      #if biz.website != "" [#biz.website \ ]
      #if biz.tax_id != "" [Tax ID: #biz.tax_id]
    ]
  ],
  align(right)[
    // Invoice identity
    #text(size: 30pt, weight: "black", fill: teal)[INVOICE]
    #v(10pt)
    #grid(
      columns: (auto, auto),
      column-gutter: 12pt,
      row-gutter: 4pt,
      align: (right, left),
      text(size: 8pt, fill: text-light)[Invoice #:],
      text(size: 9.5pt, weight: "semibold")[#inv.invoice_number],
      text(size: 8pt, fill: text-light)[Date:],
      text(size: 9.5pt)[#inv.issue_date],
      text(size: 8pt, fill: text-light)[Due:],
      text(size: 9.5pt, weight: "semibold")[#inv.due_date],
      text(size: 8pt, fill: text-light)[Status:],
      status-badge(inv.status),
    )
  ]
)

// Decorative header rule
#v(12pt)
#line(
  length: 100%,
  stroke: (paint: gradient.linear(teal, teal-mid, angle: 0deg), thickness: 2pt)
)
#v(14pt)

// ---------------------------------------------------------------------------
// Billing information
// ---------------------------------------------------------------------------
#section-heading("Bill To")

#grid(
  columns: (1fr, 1fr),
  gutter: 20pt,
  [
    // Client
    #text(size: 11pt, weight: "bold")[#client.name]
    #v(3pt)
    #text(size: 8.5pt, fill: text-mid)[
      #client.address \
      #if client.city != "" and client.country != "" [
        #client.city, #client.country \
      ] else if client.city != "" [
        #client.city \
      ] else if client.country != "" [
        #client.country \
      ]
      #if client.email != "" [#client.email \ ]
      #if client.phone != "" [#client.phone \ ]
      #if client.tax_id != "" [Tax ID: #client.tax_id]
    ]
  ],
  [],
)

#v(16pt)

// ---------------------------------------------------------------------------
// Line items table
// ---------------------------------------------------------------------------
#section-heading("Line Items")

#table(
  columns: (24pt, 1fr, 52pt, 90pt, 52pt, 90pt),
  align: (center, left, right, right, right, right),
  stroke: none,
  fill: (col, row) => {
    if row == 0 { teal }
    else if calc.odd(row) { row-alt }
    else { white-col }
  },
  inset: (x: 8pt, y: 6pt),

  // Header row
  table.header(
    text(fill: white-col, size: 8.5pt, weight: "bold")[#],
    text(fill: white-col, size: 8.5pt, weight: "bold")[Description],
    text(fill: white-col, size: 8.5pt, weight: "bold")[Qty],
    text(fill: white-col, size: 8.5pt, weight: "bold")[Unit Price],
    text(fill: white-col, size: 8.5pt, weight: "bold")[Tax],
    text(fill: white-col, size: 8.5pt, weight: "bold")[Amount],
  ),

  // Data rows
  ..for (i, item) in data.line_items.enumerate() {
    (
      text(size: 8.5pt, fill: text-light)[#(i + 1)],
      [
        #text(size: 9pt, weight: "semibold")[#item.description]
        #if item.discount != "0%" [
          #linebreak()
          #text(size: 7.5pt, fill: text-light)[Disc: #item.discount]
        ]
      ],
      text(size: 9pt)[#item.quantity],
      text(size: 9pt)[#item.unit_price],
      text(size: 9pt)[#item.tax_rate],
      text(size: 9pt, weight: "semibold")[#item.line_total],
    )
  }
)

#v(14pt)

// ---------------------------------------------------------------------------
// Totals summary (right-aligned)
// ---------------------------------------------------------------------------
#align(right)[
  #table(
    columns: (160pt, 110pt),
    align: (right, right),
    stroke: none,
    inset: (x: 6pt, y: 4pt),
    fill: none,

    // Subtotal
    text(size: 9pt, fill: text-mid)[Subtotal:],
    text(size: 9pt)[#inv.subtotal],

    // Discount (conditional)
    ..if inv.discount_minor != 0 {
      (
        text(size: 9pt, fill: text-mid)[Discount:],
        text(size: 9pt, fill: red-col)[-#inv.discount],
      )
    },

    // Tax lines
    ..for tax in data.taxes {
      let label = if tax.is_withholding {
        [#tax.name (#tax.rate) WHT:]
      } else {
        [#tax.name (#tax.rate):]
      }
      let value = if tax.is_withholding {
        text(size: 9pt, fill: red-col)[-#tax.amount]
      } else {
        text(size: 9pt)[#tax.amount]
      }
      (
        text(size: 9pt, fill: text-mid)[#label],
        value,
      )
    },

    // Separator
    table.hline(stroke: 1pt + border-col),
    table.hline(stroke: 1pt + border-col),

    // Total
    text(size: 12pt, weight: "bold")[Total:],
    text(size: 12pt, weight: "bold", fill: teal)[#inv.total],

    // Amount paid + balance due (conditional)
    ..if inv.amount_paid_minor != 0 {
      (
        text(size: 9pt, fill: text-mid)[Amount Paid:],
        text(size: 9pt, fill: red-col)[-#inv.amount_paid],
        text(size: 11pt, weight: "bold", fill: red-col)[Balance Due:],
        text(size: 11pt, weight: "bold", fill: red-col)[#inv.balance_due],
      )
    },
  )
]

#v(20pt)

// ---------------------------------------------------------------------------
// Payment details (conditional)
// ---------------------------------------------------------------------------
#if biz.bank_name != "" or biz.mobile_money_number != "" {
  section-heading("Payment Details")

  grid(
    columns: (1fr, 1fr),
    gutter: 24pt,
    [
      #if biz.bank_name != "" [
        #text(size: 8.5pt, fill: text-mid)[
          *Bank:* #biz.bank_name \
          #if biz.bank_account_number != "" [*Account:* #biz.bank_account_number \ ]
          #if biz.bank_routing_number != "" [*Routing:* #biz.bank_routing_number]
        ]
      ]
    ],
    [
      #if biz.mobile_money_number != "" [
        #text(size: 8.5pt, fill: text-mid)[
          #if biz.mobile_money_provider != "" [*Provider:* #biz.mobile_money_provider \ ]
          *Number:* #biz.mobile_money_number
        ]
      ]
    ]
  )
  v(12pt)
}

// ---------------------------------------------------------------------------
// Notes (conditional)
// ---------------------------------------------------------------------------
#if inv.keys().contains("notes") and inv.notes != "" {
  section-heading("Notes")
  box(
    fill: teal-light,
    radius: 3pt,
    inset: (x: 10pt, y: 8pt),
    width: 100%,
  )[
    #text(size: 8.5pt, fill: text-mid)[#inv.notes]
  ]
  v(10pt)
}

// ---------------------------------------------------------------------------
// Terms & Conditions (conditional)
// ---------------------------------------------------------------------------
#if inv.keys().contains("terms") and inv.terms != "" {
  section-heading("Terms & Conditions")
  text(size: 8.5pt, fill: text-mid)[#inv.terms]
  v(10pt)
}

// ---------------------------------------------------------------------------
// Payments history (conditional)
// ---------------------------------------------------------------------------
#if data.payments.len() > 0 {
  section-heading("Payment History")
  table(
    columns: (1fr, auto, auto),
    stroke: none,
    fill: (col, row) => if row == 0 { teal } else if calc.odd(row) { row-alt } else { white-col },
    inset: (x: 8pt, y: 5pt),
    table.header(
      text(fill: white-col, size: 8pt, weight: "bold")[Date],
      text(fill: white-col, size: 8pt, weight: "bold")[Method],
      text(fill: white-col, size: 8pt, weight: "bold")[Amount],
    ),
    ..for payment in data.payments {
      (
        text(size: 8.5pt)[#payment.paid_at],
        text(size: 8.5pt)[#payment.payment_method],
        text(size: 8.5pt, weight: "semibold")[#payment.amount],
      )
    }
  )
  v(12pt)
}

// ---------------------------------------------------------------------------
// QR code placeholder (for fiscal compliance / payment link)
// ---------------------------------------------------------------------------
#v(1fr)
#align(center)[
  #box(
    width: 72pt,
    height: 72pt,
    stroke: 0.5pt + border-col,
    radius: 4pt,
  )[
    #align(center + horizon)[
      #text(size: 7pt, fill: text-light)[
        QR Code \
        (Fiscal / Pay)
      ]
    ]
  ]
  #v(4pt)
  #text(size: 7pt, fill: text-light)[
    Generated by 900Invoice — 900labs.com/open-source
  ]
]
