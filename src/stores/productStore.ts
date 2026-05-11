// Product store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';
import { getTaxRates } from './taxStore';

interface BackendProduct {
  id: string;
  name: string;
  description: string;
  default_price_minor: number;
  default_currency: string;
  default_tax_rate_bps: number;
  unit: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Product {
  id: string;
  name: string;
  description: string;
  defaultPriceMinor: number;
  currencyCode: string;
  taxRateId: string | null;
  taxRateBps: number;
  unit: string;
  isActive: boolean;
  createdAt: string;
}

export interface CreateProduct {
  name: string;
  description: string;
  defaultPriceMinor: number;
  currencyCode: string;
  taxRateId: string | null;
  unit: string;
  isActive: boolean;
}

function taxRateIdForBps(rateBps: number): string | null {
  return getTaxRates().find(rate => rate.rateBps === rateBps)?.id ?? null;
}

function taxRateBpsForId(id: string | null | undefined): number | undefined {
  if (id === undefined) return undefined;
  if (id === null || id === '') return 0;
  return getTaxRates().find(rate => rate.id === id)?.rateBps ?? 0;
}

function mapProduct(product: BackendProduct): Product {
  return {
    id: product.id,
    name: product.name,
    description: product.description,
    defaultPriceMinor: product.default_price_minor,
    currencyCode: product.default_currency,
    taxRateId: taxRateIdForBps(product.default_tax_rate_bps),
    taxRateBps: product.default_tax_rate_bps,
    unit: product.unit,
    isActive: product.is_active,
    createdAt: product.created_at,
  };
}

function toBackendProduct(data: Partial<CreateProduct>) {
  const defaultTaxRateBps = taxRateBpsForId(data.taxRateId);
  return {
    ...(data.name !== undefined ? { name: data.name } : {}),
    ...(data.description !== undefined ? { description: data.description } : {}),
    ...(data.defaultPriceMinor !== undefined ? { default_price_minor: data.defaultPriceMinor } : {}),
    ...(data.currencyCode !== undefined ? { default_currency: data.currencyCode } : {}),
    ...(defaultTaxRateBps !== undefined ? { default_tax_rate_bps: defaultTaxRateBps } : {}),
    ...(data.unit !== undefined ? { unit: data.unit } : {}),
    ...(data.isActive !== undefined ? { is_active: data.isActive } : {}),
  };
}

let products = $state<Product[]>([]);
let loading = $state(false);
let error = $state<string | null>(null);
let searchQuery = $state('');

let activeProducts = $derived(products.filter(p => p.isActive));

let filteredProducts = $derived(
  searchQuery
    ? products.filter(p =>
        p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        p.description.toLowerCase().includes(searchQuery.toLowerCase())
      )
    : products
);

export async function loadProducts() {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendProduct[]>('list_products');
    products = result.map(mapProduct);
  } catch (e) {
    error = String(e);
    products = [];
  } finally {
    loading = false;
  }
}

export async function createProduct(data: CreateProduct): Promise<Product | null> {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendProduct>('create_product', { product: toBackendProduct(data) });
    const product = mapProduct(result);
    products = [...products, product];
    return product;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function updateProduct(id: string, data: Partial<CreateProduct>): Promise<Product | null> {
  loading = true;
  error = null;
  try {
    const result = await invoke<BackendProduct>('update_product', { id, update: toBackendProduct(data) });
    const product = mapProduct(result);
    products = products.map(p => p.id === id ? product : p);
    return product;
  } catch (e) {
    error = String(e);
    return null;
  } finally {
    loading = false;
  }
}

export async function deleteProduct(id: string): Promise<boolean> {
  try {
    await invoke('delete_product', { id });
    products = products.filter(p => p.id !== id);
    return true;
  } catch (e) {
    error = String(e);
    return false;
  }
}

export function searchProducts(query: string) {
  searchQuery = query;
}

export function getProducts() { return products; }
export function getActiveProducts() { return activeProducts; }
export function getFilteredProducts() { return filteredProducts; }
export function getLoading() { return loading; }
export function getError() { return error; }
