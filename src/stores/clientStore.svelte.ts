// Client store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

interface BackendClient {
  id: string;
  name: string;
  email: string;
  phone: string;
  address: string;
  city: string;
  country: string;
  country_code: string;
  tax_id: string;
  currency_code: string;
  payment_terms_days: number;
  notes: string;
  created_at: string;
  updated_at: string;
}

export interface Client {
  id: string;
  name: string;
  email: string;
  phone: string;
  address: string;
  city: string;
  country: string;
  countryCode: string;
  taxId: string;
  currencyCode: string;
  paymentTermsDays: number;
  notes: string;
  invoiceCount: number;
  outstandingMinor: number;
  createdAt: string;
}

export interface CreateClient {
  name: string;
  email: string;
  phone: string;
  address: string;
  city: string;
  country: string;
  countryCode: string;
  taxId: string;
  currencyCode: string;
  paymentTermsDays: number;
  notes: string;
}

function mapClient(client: BackendClient): Client {
  return {
    id: client.id,
    name: client.name,
    email: client.email,
    phone: client.phone,
    address: client.address,
    city: client.city,
    country: client.country,
    countryCode: client.country_code,
    taxId: client.tax_id,
    currencyCode: client.currency_code,
    paymentTermsDays: client.payment_terms_days,
    notes: client.notes,
    invoiceCount: 0,
    outstandingMinor: 0,
    createdAt: client.created_at,
  };
}

function toBackendClient(data: Partial<CreateClient>) {
  return {
    ...(data.name !== undefined ? { name: data.name } : {}),
    ...(data.email !== undefined ? { email: data.email } : {}),
    ...(data.phone !== undefined ? { phone: data.phone } : {}),
    ...(data.address !== undefined ? { address: data.address } : {}),
    ...(data.city !== undefined ? { city: data.city } : {}),
    ...(data.country !== undefined ? { country: data.country } : {}),
    ...(data.countryCode !== undefined ? { country_code: data.countryCode } : {}),
    ...(data.taxId !== undefined ? { tax_id: data.taxId } : {}),
    ...(data.currencyCode !== undefined ? { currency_code: data.currencyCode } : {}),
    ...(data.paymentTermsDays !== undefined ? { payment_terms_days: data.paymentTermsDays } : {}),
    ...(data.notes !== undefined ? { notes: data.notes } : {}),
  };
}

let clients = $state<Client[]>([]);
let currentClient = $state<Client | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);
let searchQuery = $state('');

let filteredClients = $derived(
  searchQuery
    ? clients.filter(c =>
        c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.phone.includes(searchQuery)
      )
    : clients
);

let clientCount = $derived(clients.length);

export async function loadClients() {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendClient[]>('list_clients');
    clients = result.map(mapClient);
  } catch (e) {
    error = String(e);
    clients = [];
  } finally {
    loading = false;
  }
}

export async function loadClient(id: string) {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendClient>('get_client', { id });
    currentClient = mapClient(result);
  } catch (e) {
    error = String(e);
    currentClient = null;
  } finally {
    loading = false;
  }
}

export async function createClient(data: CreateClient): Promise<Client | null> {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendClient>('create_client', { client: toBackendClient(data) });
    const client = mapClient(result);
    clients = [...clients, client];
    return client;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function updateClient(id: string, data: Partial<CreateClient>): Promise<Client | null> {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendClient>('update_client', { id, update: toBackendClient(data) });
    const client = mapClient(result);
    clients = clients.map(c => c.id === id ? client : c);
    if (currentClient?.id === id) currentClient = client;
    return client;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function deleteClient(id: string): Promise<boolean> {
  try {
    await invoke('delete_client', { id });
    clients = clients.filter(c => c.id !== id);
    if (currentClient?.id === id) currentClient = null;
    return true;
  } catch (e) {
    error = String(e);
    return false;
  }
}

export function searchClients(query: string) {
  searchQuery = query;
}

export function getClients() { return clients; }
export function getFilteredClients() { return filteredClients; }
export function getCurrentClient() { return currentClient; }
export function getLoading() { return loading; }
export function getError() { return error; }
export function getSearchQuery() { return searchQuery; }
export function getClientCount() { return clientCount; }
