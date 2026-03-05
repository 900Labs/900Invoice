// Tax store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

export interface TaxRate {
  id: string;
  name: string;
  displayName: string;
  rateBps: number; // basis points: 1600 = 16%
  country: string;
  isDefault: boolean;
  isWithholding: boolean;
  isInclusive: boolean;
  isActive: boolean;
}

export interface CreateTaxRate {
  name: string;
  displayName: string;
  rateBps: number;
  country: string;
  isDefault: boolean;
  isWithholding: boolean;
  isInclusive: boolean;
}

let taxRates = $state<TaxRate[]>([]);
let loading = $state(false);
let error = $state<string | null>(null);

let activeTaxRates = $derived(taxRates.filter(t => t.isActive));

let taxRatesByCountry = $derived(
  taxRates.reduce((acc, t) => {
    if (!acc[t.country]) acc[t.country] = [];
    acc[t.country].push(t);
    return acc;
  }, {} as Record<string, TaxRate[]>)
);

export async function loadTaxRates() {
  loading = true;
  error = null;
  try {
    taxRates = await invoke<TaxRate[]>('list_tax_rates');
  } catch (e) {
    error = String(e);
    taxRates = [];
  } finally {
    loading = false;
  }
}

export async function createTaxRate(data: CreateTaxRate): Promise<TaxRate | null> {
  loading = true;
  error = null;
  try {
    const rate = await invoke<TaxRate>('create_tax_rate', { data });
    taxRates = [...taxRates, rate];
    return rate;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function updateTaxRate(id: string, data: Partial<CreateTaxRate>): Promise<TaxRate | null> {
  loading = true;
  error = null;
  try {
    const rate = await invoke<TaxRate>('update_tax_rate', { id, data });
    taxRates = taxRates.map(t => t.id === id ? rate : t);
    return rate;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function deleteTaxRate(id: string): Promise<boolean> {
  try {
    await invoke('delete_tax_rate', { id });
    taxRates = taxRates.filter(t => t.id !== id);
    return true;
  } catch (e) {
    error = String(e);
    return false;
  }
}

export function getTaxRatesForCountry(code: string): TaxRate[] {
  return taxRates.filter(t => t.country === code || t.country === '');
}

export function getTaxRates() { return taxRates; }
export function getActiveTaxRates() { return activeTaxRates; }
export function getTaxRatesByCountry() { return taxRatesByCountry; }
export function getLoading() { return loading; }
export function getError() { return error; }
