// Product store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

export interface Product {
  id: string;
  name: string;
  description: string;
  defaultPriceMinor: number;
  currencyCode: string;
  taxRateId: string | null;
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
    products = await invoke<Product[]>('list_products');
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
    const product = await invoke<Product>('create_product', { data });
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
    const product = await invoke<Product>('update_product', { id, data });
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
