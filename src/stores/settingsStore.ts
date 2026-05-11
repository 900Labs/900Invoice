// Settings store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core';
import { setLocale } from './i18nStore';

interface BackendBusinessProfile {
  id: string;
  name: string;
  address: string;
  city: string;
  country: string;
  country_code: string;
  phone: string;
  email: string;
  website: string;
  tax_id: string;
  logo_path: string | null;
  default_currency: string;
  default_payment_terms_days: number;
  bank_name: string;
  bank_account_number: string;
  bank_routing_number: string;
  mobile_money_number: string;
  mobile_money_provider: string;
  created_at: string;
  updated_at: string;
}

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

const settingKeys: Record<keyof AppSettings, string> = {
  locale: 'locale',
  theme: 'theme',
  dateFormat: 'date_format',
  currencyPosition: 'currency_position',
  paperSize: 'paper_size',
  defaultCurrency: 'default_currency',
};

function parseSetting(value: unknown): unknown {
  if (typeof value !== 'string') return value;
  try {
    return JSON.parse(value);
  } catch {
    return value;
  }
}

function normalizePaperSize(value: unknown): AppSettings['paperSize'] {
  return String(value ?? defaultSettings.paperSize).toLowerCase() === 'letter' ? 'Letter' : 'A4';
}

function mapSettings(raw: Record<string, unknown>): AppSettings {
  const parsed = Object.fromEntries(
    Object.entries(raw).map(([key, value]) => [key, parseSetting(value)])
  );

  return {
    locale: String(parsed.locale ?? defaultSettings.locale),
    theme: String(parsed.theme ?? defaultSettings.theme),
    dateFormat: String(parsed.date_format ?? parsed.dateFormat ?? defaultSettings.dateFormat),
    currencyPosition: parsed.currency_position === 'after' || parsed.currencyPosition === 'after' ? 'after' : 'before',
    paperSize: normalizePaperSize(parsed.paper_size ?? parsed.paperSize),
    defaultCurrency: String(parsed.default_currency ?? parsed.defaultCurrency ?? defaultSettings.defaultCurrency),
  };
}

function mapBusinessProfile(profile: BackendBusinessProfile | null): BusinessProfile {
  if (!profile) return { ...defaultProfile };
  return {
    companyName: profile.name,
    address: profile.address,
    city: profile.city,
    country: profile.country,
    phone: profile.phone,
    email: profile.email,
    website: profile.website,
    taxId: profile.tax_id,
    logoPath: profile.logo_path,
    bankName: profile.bank_name,
    bankAccount: profile.bank_account_number,
    bankRouting: profile.bank_routing_number,
    mobileMoney: profile.mobile_money_number,
    mobileMoneyProvider: profile.mobile_money_provider,
  };
}

function toBackendBusinessProfile(profile: BusinessProfile) {
  return {
    name: profile.companyName,
    address: profile.address,
    city: profile.city,
    country: profile.country,
    phone: profile.phone,
    email: profile.email,
    website: profile.website,
    tax_id: profile.taxId,
    logo_path: profile.logoPath,
    bank_name: profile.bankName,
    bank_account_number: profile.bankAccount,
    bank_routing_number: profile.bankRouting,
    mobile_money_number: profile.mobileMoney,
    mobile_money_provider: profile.mobileMoneyProvider,
  };
}

let settings = $state<AppSettings>({ ...defaultSettings });
let businessProfile = $state<BusinessProfile>({ ...defaultProfile });
let loading = $state(false);
let error = $state<string | null>(null);

export async function loadSettings() {
  loading = true;
  error = null;
  try {
    const s = await invoke<Record<string, unknown>>('get_settings');
    settings = mapSettings(s);
    await setLocale(settings.locale);
  } catch (e) {
    // Use defaults if backend not available
    settings = { ...defaultSettings };
    await setLocale(settings.locale);
  } finally {
    loading = false;
  }
}

export async function updateSetting<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
  settings = { ...settings, [key]: value };
  try {
    await invoke('update_setting', { key: settingKeys[key], value });
  } catch (e) {
    // Silently fail - settings already updated locally
  }
}

export async function loadBusinessProfile() {
  loading = true;
  error = null;
  try {
    const p = await invoke<BackendBusinessProfile | null>('get_business_profile');
    businessProfile = mapBusinessProfile(p);
  } catch (e) {
    businessProfile = { ...defaultProfile };
  } finally {
    loading = false;
  }
}

export async function updateBusinessProfile(data: Partial<BusinessProfile>) {
  businessProfile = { ...businessProfile, ...data };
  try {
    const result = await invoke<BackendBusinessProfile>('update_business_profile', {
      update: toBackendBusinessProfile(businessProfile),
    });
    businessProfile = mapBusinessProfile(result);
  } catch (e) {
    error = String(e);
  }
}

export function getSettings() { return settings; }
export function getBusinessProfile() { return businessProfile; }
export function getLoading() { return loading; }
export function getError() { return error; }
