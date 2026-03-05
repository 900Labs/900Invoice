// Settings store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';

export interface AppSettings {
  locale: string;
  theme: string;
  dateFormat: string;
  currencyPosition: 'before' | 'after';
  paperSize: 'A4' | 'Letter';
  defaultCurrency: string;
}

export interface BusinessProfile {
  companyName: string;
  address: string;
  city: string;
  country: string;
  phone: string;
  email: string;
  website: string;
  taxId: string;
  logoPath: string | null;
  bankName: string;
  bankAccount: string;
  bankRouting: string;
  mobileMoney: string;
  mobileMoneyProvider: string;
}

const defaultSettings: AppSettings = {
  locale: 'en',
  theme: 'light',
  dateFormat: 'YYYY-MM-DD',
  currencyPosition: 'before',
  paperSize: 'A4',
  defaultCurrency: 'USD',
};

const defaultProfile: BusinessProfile = {
  companyName: '',
  address: '',
  city: '',
  country: '',
  phone: '',
  email: '',
  website: '',
  taxId: '',
  logoPath: null,
  bankName: '',
  bankAccount: '',
  bankRouting: '',
  mobileMoney: '',
  mobileMoneyProvider: '',
};

let settings = $state<AppSettings>({ ...defaultSettings });
let businessProfile = $state<BusinessProfile>({ ...defaultProfile });
let loading = $state(false);
let error = $state<string | null>(null);

export async function loadSettings() {
  loading = true;
  error = null;
  try {
    const s = await invoke<AppSettings>('get_settings');
    settings = s;
  } catch (e) {
    // Use defaults if backend not available
    settings = { ...defaultSettings };
  } finally {
    loading = false;
  }
}

export async function updateSetting<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
  settings = { ...settings, [key]: value };
  try {
    await invoke('update_setting', { key, value: String(value) });
  } catch (e) {
    // Silently fail - settings already updated locally
  }
}

export async function loadBusinessProfile() {
  loading = true;
  error = null;
  try {
    const p = await invoke<BusinessProfile>('get_business_profile');
    businessProfile = p;
  } catch (e) {
    businessProfile = { ...defaultProfile };
  } finally {
    loading = false;
  }
}

export async function updateBusinessProfile(data: Partial<BusinessProfile>) {
  businessProfile = { ...businessProfile, ...data };
  try {
    await invoke('update_business_profile', { profile: businessProfile });
  } catch (e) {
    error = String(e);
  }
}

export function getSettings() { return settings; }
export function getBusinessProfile() { return businessProfile; }
export function getLoading() { return loading; }
export function getError() { return error; }
