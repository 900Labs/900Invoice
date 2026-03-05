// Invoice store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

export type InvoiceStatus = 'Draft' | 'Finalized' | 'Sent' | 'Paid' | 'Void';

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
  lineItems: Omit<LineItem, 'id'>[];
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
    invoices = await invoke<Invoice[]>('list_invoices');
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
    currentInvoice = await invoke<Invoice>('get_invoice', { id });
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
    const invoice = await invoke<Invoice>('create_invoice', { data });
    invoices = [invoice, ...invoices];
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
    const invoice = await invoke<Invoice>('update_invoice', { id, data });
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
    const invoice = await invoke<Invoice>('finalize_invoice', { id });
    invoices = invoices.map(i => i.id === id ? invoice : i);
    if (currentInvoice?.id === id) currentInvoice = invoice;
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  }
}

export async function voidInvoice(id: string): Promise<Invoice | null> {
  try {
    const invoice = await invoke<Invoice>('void_invoice', { id });
    invoices = invoices.map(i => i.id === id ? invoice : i);
    if (currentInvoice?.id === id) currentInvoice = invoice;
    return invoice;
  } catch (e) {
    error = String(e);
    return null;
  }
}

export async function duplicateInvoice(id: string): Promise<Invoice | null> {
  try {
    const invoice = await invoke<Invoice>('duplicate_invoice', { id });
    invoices = [invoice, ...invoices];
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
    const invoice = await invoke<Invoice>('record_payment', { invoiceId, payment });
    invoices = invoices.map(i => i.id === invoiceId ? invoice : i);
    if (currentInvoice?.id === invoiceId) currentInvoice = invoice;
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
