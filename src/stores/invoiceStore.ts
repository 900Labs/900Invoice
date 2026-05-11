// Invoice store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';
import { getTaxRates } from './taxStore';

export type InvoiceStatus = 'Draft' | 'Finalized' | 'Sent' | 'Paid' | 'Void';

interface BackendClient {
  name?: string;
}

interface BackendLineItem {
  id: string;
  invoice_id: string;
  product_id: string | null;
  description: string;
  quantity: number;
  unit_price_minor: number;
  tax_rate_bps: number;
  discount_bps: number;
  line_total_minor: number;
  sort_order: number;
  created_at: string;
}

interface BackendInvoiceTax {
  id: string;
  invoice_id: string;
  tax_rate_id: string | null;
  tax_name: string;
  tax_rate_bps: number;
  tax_amount_minor: number;
  is_withholding: boolean;
  created_at: string;
}

interface BackendPayment {
  id: string;
  invoice_id: string;
  amount_minor: number;
  currency_code: string;
  payment_method: string;
  payment_reference: string;
  notes: string;
  paid_at: string;
  created_at: string;
}

interface BackendInvoice {
  id: string;
  invoice_number: string | null;
  client_id: string;
  client?: BackendClient | null;
  status: string;
  currency_code: string;
  subtotal_minor: number;
  discount_minor: number;
  tax_amount_minor: number;
  total_minor: number;
  amount_paid_minor: number;
  issue_date: string;
  due_date: string;
  uses_inclusive_taxes: boolean;
  notes: string;
  terms: string;
  footer: string;
  created_at: string;
  updated_at: string;
  line_items?: BackendLineItem[];
  taxes?: BackendInvoiceTax[];
  payments?: BackendPayment[];
}

export interface LineItem {
  id: string;
  productId: string | null;
  description: string;
  quantity: number;
  unitPriceMinor: number;
  taxRateId: string | null;
  discountPercent: number;
  sortOrder: number;
}

export type InvoiceInputLineItem = Omit<LineItem, 'id'> & { id?: string };

export interface TaxLine {
  taxRateId: string;
  taxName: string;
  taxDisplayName: string;
  rateBps: number;
  baseAmountMinor: number;
  taxAmountMinor: number;
}

export interface Payment {
  id: string;
  amountMinor: number;
  method: string;
  reference: string;
  paidAt: string;
  notes: string;
}

export interface Invoice {
  id: string;
  invoiceNumber: string;
  clientId: string;
  clientName: string;
  status: InvoiceStatus;
  currencyCode: string;
  issueDate: string;
  dueDate: string;
  subtotalMinor: number;
  discountMinor: number;
  taxTotalMinor: number;
  totalMinor: number;
  amountPaidMinor: number;
  balanceDueMinor: number;
  taxMode: 'Exclusive' | 'Inclusive';
  notes: string;
  terms: string;
  footer: string;
  lineItems: LineItem[];
  taxLines: TaxLine[];
  payments: Payment[];
  createdAt: string;
  updatedAt: string;
}

export interface CreateInvoice {
  clientId: string;
  currencyCode: string;
  issueDate: string;
  dueDate: string;
  taxMode: 'Exclusive' | 'Inclusive';
  notes: string;
  terms: string;
  footer: string;
  lineItems: InvoiceInputLineItem[];
}

export interface InvoiceFilters {
  status: string;
  dateRangeStart: string;
  dateRangeEnd: string;
  clientId: string;
  search: string;
}

let invoices = $state<Invoice[]>([]);
let currentInvoice = $state<Invoice | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);
let filters = $state<InvoiceFilters>({
  status: '',
  dateRangeStart: '',
  dateRangeEnd: '',
  clientId: '',
  search: '',
});

function taxRateIdForBps(rateBps: number): string | null {
  return getTaxRates().find(rate => rate.rateBps === rateBps)?.id ?? null;
}

function taxRateBpsForId(id: string | null | undefined): number {
  if (!id) return 0;
  return getTaxRates().find(rate => rate.id === id)?.rateBps ?? 0;
}

function statusFromBackend(status: string): InvoiceStatus {
  const normalized = status.toLowerCase();
  if (normalized === 'finalized') return 'Finalized';
  if (normalized === 'sent') return 'Sent';
  if (normalized === 'paid') return 'Paid';
  if (normalized === 'void') return 'Void';
  return 'Draft';
}

function mapLineItem(item: BackendLineItem): LineItem {
  return {
    id: item.id,
    productId: item.product_id,
    description: item.description,
    quantity: item.quantity / 100,
    unitPriceMinor: item.unit_price_minor,
    taxRateId: taxRateIdForBps(item.tax_rate_bps),
    discountPercent: item.discount_bps / 100,
    sortOrder: item.sort_order,
  };
}

function mapTaxLine(line: BackendInvoiceTax): TaxLine {
  const matchedRate = line.tax_rate_id
    ? getTaxRates().find(rate => rate.id === line.tax_rate_id)
    : getTaxRates().find(rate => rate.rateBps === line.tax_rate_bps);

  return {
    taxRateId: line.tax_rate_id ?? matchedRate?.id ?? '',
    taxName: line.tax_name,
    taxDisplayName: matchedRate?.displayName ?? line.tax_name,
    rateBps: line.tax_rate_bps,
    baseAmountMinor: 0,
    taxAmountMinor: line.tax_amount_minor,
  };
}

function mapPayment(payment: BackendPayment): Payment {
  return {
    id: payment.id,
    amountMinor: payment.amount_minor,
    method: payment.payment_method,
    reference: payment.payment_reference,
    paidAt: payment.paid_at,
    notes: payment.notes,
  };
}

function mapInvoice(invoice: BackendInvoice): Invoice {
  return {
    id: invoice.id,
    invoiceNumber: invoice.invoice_number ?? 'Draft',
    clientId: invoice.client_id,
    clientName: invoice.client?.name ?? '',
    status: statusFromBackend(invoice.status),
    currencyCode: invoice.currency_code,
    issueDate: invoice.issue_date,
    dueDate: invoice.due_date,
    subtotalMinor: invoice.subtotal_minor,
    discountMinor: invoice.discount_minor,
    taxTotalMinor: invoice.tax_amount_minor,
    totalMinor: invoice.total_minor,
    amountPaidMinor: invoice.amount_paid_minor,
    balanceDueMinor: invoice.total_minor - invoice.amount_paid_minor,
    taxMode: invoice.uses_inclusive_taxes ? 'Inclusive' : 'Exclusive',
    notes: invoice.notes,
    terms: invoice.terms,
    footer: invoice.footer,
    lineItems: (invoice.line_items ?? []).map(mapLineItem),
    taxLines: (invoice.taxes ?? []).map(mapTaxLine),
    payments: (invoice.payments ?? []).map(mapPayment),
    createdAt: invoice.created_at,
    updatedAt: invoice.updated_at,
  };
}

function toBackendInvoice(data: Partial<CreateInvoice>) {
  return {
    ...(data.clientId !== undefined ? { client_id: data.clientId } : {}),
    ...(data.currencyCode !== undefined ? { currency_code: data.currencyCode } : {}),
    ...(data.issueDate !== undefined ? { issue_date: data.issueDate } : {}),
    ...(data.dueDate !== undefined ? { due_date: data.dueDate } : {}),
    ...(data.taxMode !== undefined ? { uses_inclusive_taxes: data.taxMode === 'Inclusive' } : {}),
    ...(data.notes !== undefined ? { notes: data.notes } : {}),
    ...(data.terms !== undefined ? { terms: data.terms } : {}),
    ...(data.footer !== undefined ? { footer: data.footer } : {}),
  };
}

function toBackendLineItem(invoiceId: string, item: InvoiceInputLineItem, sortOrder: number) {
  return {
    invoice_id: invoiceId,
    product_id: item.productId,
    description: item.description,
    quantity: Math.round(item.quantity * 100),
    unit_price_minor: item.unitPriceMinor,
    tax_rate_bps: taxRateBpsForId(item.taxRateId),
    discount_bps: Math.round(item.discountPercent * 100),
    sort_order: sortOrder,
  };
}

function toBackendLineItemUpdate(item: InvoiceInputLineItem, sortOrder: number) {
  return {
    product_id: item.productId,
    description: item.description,
    quantity: Math.round(item.quantity * 100),
    unit_price_minor: item.unitPriceMinor,
    tax_rate_bps: taxRateBpsForId(item.taxRateId),
    discount_bps: Math.round(item.discountPercent * 100),
    sort_order: sortOrder,
  };
}

async function fetchInvoice(id: string): Promise<Invoice> {
  const result = await invoke<BackendInvoice>('get_invoice', { id });
  return mapInvoice(result);
}

async function syncLineItems(invoiceId: string, existing: LineItem[], next: InvoiceInputLineItem[]) {
  const existingIds = new Set(existing.map(item => item.id));
  const retainedIds = new Set(next.map(item => item.id).filter((id): id is string => !!id && existingIds.has(id)));

  for (const item of existing) {
    if (!retainedIds.has(item.id)) {
      await invoke('remove_line_item', { id: item.id });
    }
  }

  for (const [index, item] of next.entries()) {
    if (item.id && existingIds.has(item.id)) {
      await invoke('update_line_item', {
        id: item.id,
        update: toBackendLineItemUpdate(item, index),
      });
    } else {
      await invoke('add_line_item', {
        lineItem: toBackendLineItem(invoiceId, item, index),
      });
    }
  }
}

function upsertInvoice(invoice: Invoice) {
  const exists = invoices.some(i => i.id === invoice.id);
  invoices = exists
    ? invoices.map(i => i.id === invoice.id ? invoice : i)
    : [invoice, ...invoices];
  if (currentInvoice?.id === invoice.id) currentInvoice = invoice;
}

let filteredInvoices = $derived((() => {
  let result = invoices;
  if (filters.status) result = result.filter(i => i.status === filters.status);
  if (filters.clientId) result = result.filter(i => i.clientId === filters.clientId);
  if (filters.search) {
    const q = filters.search.toLowerCase();
    result = result.filter(i =>
      i.invoiceNumber.toLowerCase().includes(q) ||
      i.clientName.toLowerCase().includes(q)
    );
  }
  if (filters.dateRangeStart) result = result.filter(i => i.issueDate >= filters.dateRangeStart);
  if (filters.dateRangeEnd) result = result.filter(i => i.issueDate <= filters.dateRangeEnd);
  return result;
})());

let overdueInvoices = $derived(
  invoices.filter(i => {
    if (i.status === 'Paid' || i.status === 'Void') return false;
    return new Date(i.dueDate) < new Date();
  })
);

let draftCount = $derived(invoices.filter(i => i.status === 'Draft').length);

let totalOutstanding = $derived(
  invoices
    .filter(i => i.status !== 'Paid' && i.status !== 'Void' && i.status !== 'Draft')
    .reduce((sum, i) => sum + i.balanceDueMinor, 0)
);

export async function loadInvoices() {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendInvoice[]>('list_invoices');
    const detailed = await Promise.all(result.map(invoice => fetchInvoice(invoice.id).catch(() => mapInvoice(invoice))));
    invoices = detailed;
  } catch (e) {
    error = String(e);
    invoices = [];
  } finally {
    loading = false;
  }
}

export async function loadInvoice(id: string) {
  loading = true;
  error = null;
  try {
    currentInvoice = await fetchInvoice(id);
  } catch (e) {
    error = String(e);
    currentInvoice = null;
  } finally {
    loading = false;
  }
}

export async function createInvoice(data: CreateInvoice): Promise<Invoice | null> {
  loading = true;
  error = null;
  try {
    const created = await invoke<BackendInvoice>('create_invoice', { invoice: toBackendInvoice(data) });
    await syncLineItems(created.id, [], data.lineItems);
    const invoice = await fetchInvoice(created.id);
    upsertInvoice(invoice);
    currentInvoice = invoice;
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function updateInvoice(id: string, data: Partial<CreateInvoice>): Promise<Invoice | null> {
  loading = true;
  error = null;
  try {
    const existing = currentInvoice?.id === id ? currentInvoice : await fetchInvoice(id);
    await invoke<BackendInvoice>('update_invoice', { id, update: toBackendInvoice(data) });
    if (data.lineItems) {
      await syncLineItems(id, existing.lineItems, data.lineItems);
    }
    const invoice = await fetchInvoice(id);
    invoices = invoices.map(i => i.id === id ? invoice : i);
    if (currentInvoice?.id === id) currentInvoice = invoice;
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function deleteInvoice(id: string): Promise<boolean> {
  try {
    await invoke('delete_invoice', { id });
    invoices = invoices.filter(i => i.id !== id);
    if (currentInvoice?.id === id) currentInvoice = null;
    return true;
  } catch (e) {
    error = String(e);
    return false;
  }
}

export async function finalizeInvoice(id: string): Promise<Invoice | null> {
  try {
    const result = await invoke<BackendInvoice>('finalize_invoice', { id });
    const invoice = mapInvoice(result);
    upsertInvoice(invoice);
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  }
}

export async function markInvoiceSent(id: string): Promise<Invoice | null> {
  try {
    const result = await invoke<BackendInvoice>('mark_invoice_sent', { id });
    const invoice = mapInvoice(result);
    upsertInvoice(invoice);
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  }
}

export async function voidInvoice(id: string): Promise<Invoice | null> {
  try {
    const result = await invoke<BackendInvoice>('void_invoice', { id });
    const invoice = mapInvoice(result);
    upsertInvoice(invoice);
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  }
}

export async function duplicateInvoice(id: string): Promise<Invoice | null> {
  try {
    const result = await invoke<BackendInvoice>('duplicate_invoice', { id });
    const invoice = mapInvoice(result);
    upsertInvoice(invoice);
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  }
}

export async function recordPayment(invoiceId: string, payment: {
  amountMinor: number;
  method: string;
  reference: string;
  paidAt: string;
  notes: string;
}): Promise<boolean> {
  try {
    const invoice = currentInvoice?.id === invoiceId
      ? currentInvoice
      : invoices.find(i => i.id === invoiceId);
    await invoke<BackendPayment>('record_payment', {
      payment: {
        invoice_id: invoiceId,
        amount_minor: payment.amountMinor,
        currency_code: invoice?.currencyCode ?? 'USD',
        payment_method: payment.method,
        payment_reference: payment.reference,
        paid_at: payment.paidAt,
        notes: payment.notes,
      },
    });
    const updated = await fetchInvoice(invoiceId);
    invoices = invoices.map(i => i.id === invoiceId ? updated : i);
    if (currentInvoice?.id === invoiceId) currentInvoice = updated;
    return true;
  } catch (e) {
    error = String(e);
    return false;
  }
}

export function setFilters(newFilters: Partial<InvoiceFilters>) {
  filters = { ...filters, ...newFilters };
}

export function clearFilters() {
  filters = { status: '', dateRangeStart: '', dateRangeEnd: '', clientId: '', search: '' };
}

export function getInvoices() { return invoices; }
export function getFilteredInvoices() { return filteredInvoices; }
export function getCurrentInvoice() { return currentInvoice; }
export function getLoading() { return loading; }
export function getError() { return error; }
export function getFilters() { return filters; }
export function getOverdueInvoices() { return overdueInvoices; }
export function getDraftCount() { return draftCount; }
export function getTotalOutstanding() { return totalOutstanding; }
