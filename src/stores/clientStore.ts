// Client store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

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
    clients = await invoke<Client[]>('list_clients');
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
    currentClient = await invoke<Client>('get_client', { id });
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
    const client = await invoke<Client>('create_client', { data });
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
    const client = await invoke<Client>('update_client', { id, data });
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
