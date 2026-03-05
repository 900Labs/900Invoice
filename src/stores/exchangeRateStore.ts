// Exchange rate store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

export interface ExchangeRate {
  fromCurrency: string;
  toCurrency: string;
  rate: number;
  updatedAt: string;
}

let rates = $state<ExchangeRate[]>([]);
let loading = $state(false);
let error = $state<string | null>(null);

let lastUpdated = $derived(
  rates.length > 0
    ? rates.reduce((latest, r) =>
        r.updatedAt > latest ? r.updatedAt : latest,
        rates[0].updatedAt
      )
    : null
);

export async function loadRates() {
  loading = true;
  error = null;
  try {
    rates = await invoke<ExchangeRate[]>('get_exchange_rates');
  } catch (e) {
    error = String(e);
    rates = [];
  } finally {
    loading = false;
  }
}

export function convertCurrency(amount: number, from: string, to: string): number {
  if (from === to) return amount;
  const rate = rates.find(r => r.fromCurrency === from && r.toCurrency === to);
  if (rate) return Math.round(amount * rate.rate);

  // Try reverse
  const reverseRate = rates.find(r => r.fromCurrency === to && r.toCurrency === from);
  if (reverseRate) return Math.round(amount / reverseRate.rate);

  return amount; // fallback: 1:1
}

export function getRates() { return rates; }
export function getLoading() { return loading; }
export function getError() { return error; }
export function getLastUpdated() { return lastUpdated; }
