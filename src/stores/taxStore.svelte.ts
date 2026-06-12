// Tax store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

interface BackendTaxRate {
  id: string;
  name: string;
  display_name: string;
  rate_bps: number;
  country_code: string | null;
  is_default: boolean;
  is_withholding: boolean;
  is_inclusive: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

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

function mapTaxRate(rate: BackendTaxRate): TaxRate {
  return {
    id: rate.id,
    name: rate.name,
    displayName: rate.display_name,
    rateBps: rate.rate_bps,
    country: rate.country_code ?? '',
    isDefault: rate.is_default,
    isWithholding: rate.is_withholding,
    isInclusive: rate.is_inclusive,
    isActive: rate.is_active,
  };
}

function toBackendTaxRate(data: Partial<CreateTaxRate>) {
  return {
    ...(data.name !== undefined ? { name: data.name } : {}),
    ...(data.displayName !== undefined ? { display_name: data.displayName } : {}),
    ...(data.rateBps !== undefined ? { rate_bps: data.rateBps } : {}),
    ...(data.country !== undefined ? { country_code: data.country || null } : {}),
    ...(data.isDefault !== undefined ? { is_default: data.isDefault } : {}),
    ...(data.isWithholding !== undefined ? { is_withholding: data.isWithholding } : {}),
    ...(data.isInclusive !== undefined ? { is_inclusive: data.isInclusive } : {}),
  };
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
    const result = await invoke<BackendTaxRate[]>('list_tax_rates');
    taxRates = result.map(mapTaxRate);
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
    const result = await invoke<BackendTaxRate>('create_tax_rate', { taxRate: toBackendTaxRate(data) });
    const rate = mapTaxRate(result);
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
    const result = await invoke<BackendTaxRate>('update_tax_rate', { id, update: toBackendTaxRate(data) });
    const rate = mapTaxRate(result);
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
