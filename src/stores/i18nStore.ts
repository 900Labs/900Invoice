// i18n store using Svelte 5 runes
import type enJson from '../i18n/en.json';

type TranslationKeys = typeof enJson;

let currentLocale = $state('en');
let translations = $state<Record<string, unknown>>({});
let fallback = $state<Record<string, unknown>>({});

export async function loadTranslations(locale: string) {
  try {
    let data: Record<string, unknown>;
    switch (locale) {
      case 'fr':
        data = (await import('../i18n/fr.json')).default as Record<string, unknown>;
        break;
      case 'es':
        data = (await import('../i18n/es.json')).default as Record<string, unknown>;
        break;
      case 'ar':
        data = (await import('../i18n/ar.json')).default as Record<string, unknown>;
        break;
      case 'sw':
        data = (await import('../i18n/sw.json')).default as Record<string, unknown>;
        break;
      case 'hi':
        data = (await import('../i18n/hi.json')).default as Record<string, unknown>;
        break;
      default:
        data = (await import('../i18n/en.json')).default as Record<string, unknown>;
    }
    translations = data;
  } catch (e) {
    console.error('Failed to load translations for', locale, e);
  }
}

export async function setLocale(locale: string) {
  await loadTranslations(locale);
  currentLocale = locale;

  // RTL support for Arabic
  if (locale === 'ar') {
    document.documentElement.setAttribute('dir', 'rtl');
    document.documentElement.setAttribute('lang', 'ar');
  } else {
    document.documentElement.setAttribute('dir', 'ltr');
    document.documentElement.setAttribute('lang', locale);
  }
}

export async function initI18n() {
  // Load English as fallback
  const enData = (await import('../i18n/en.json')).default as Record<string, unknown>;
  fallback = enData;
  translations = enData;
  currentLocale = 'en';
  document.documentElement.setAttribute('dir', 'ltr');
  document.documentElement.setAttribute('lang', 'en');
}

function getNestedValue(obj: Record<string, unknown>, path: string): string | null {
  const parts = path.split('.');
  let current: unknown = obj;
  for (const part of parts) {
    if (current == null || typeof current !== 'object') return null;
    current = (current as Record<string, unknown>)[part];
  }
  if (typeof current === 'string') return current;
  return null;
}

export function t(key: string, params?: Record<string, string | number>): string {
  let value = getNestedValue(translations, key) ?? getNestedValue(fallback, key) ?? key;

  if (params) {
    for (const [k, v] of Object.entries(params)) {
      value = value.replace(new RegExp(`\\{${k}\\}`, 'g'), String(v));
    }
  }
  return value;
}

export function getCurrentLocale() {
  return currentLocale;
}

export function getTranslations() {
  return translations;
}

export const SUPPORTED_LOCALES = [
  { code: 'en', name: 'English', nativeName: 'English' },
  { code: 'fr', name: 'French', nativeName: 'Français' },
  { code: 'es', name: 'Spanish', nativeName: 'Español' },
  { code: 'ar', name: 'Arabic', nativeName: 'العربية' },
  { code: 'sw', name: 'Swahili', nativeName: 'Kiswahili' },
  { code: 'hi', name: 'Hindi', nativeName: 'हिंदी' },
];
