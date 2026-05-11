export interface LocaleConfig {
  code: string;
  dateLocale: string;
  numberLocale: string;
}

export const LOCALE_CONFIG: Record<string, LocaleConfig> = {
  en: { code: 'en', dateLocale: 'en-US', numberLocale: 'en-US' },
  fr: { code: 'fr', dateLocale: 'fr-FR', numberLocale: 'fr-FR' },
  es: { code: 'es', dateLocale: 'es-ES', numberLocale: 'es-ES' },
  ar: { code: 'ar', dateLocale: 'ar', numberLocale: 'ar' },
  sw: { code: 'sw', dateLocale: 'sw-KE', numberLocale: 'sw-KE' },
  hi: { code: 'hi', dateLocale: 'hi-IN', numberLocale: 'hi-IN' },
};

function browserLocale(): string {
  if (typeof document !== 'undefined') {
    const lang = document.documentElement.lang;
    if (lang) return lang;
  }
  if (typeof navigator !== 'undefined' && navigator.language) {
    return navigator.language;
  }
  return 'en';
}

export function getLocaleConfig(locale = browserLocale()): LocaleConfig {
  const code = locale.toLowerCase().split('-')[0];
  return LOCALE_CONFIG[code] ?? { code, dateLocale: locale, numberLocale: locale };
}

export function resolveDateLocale(locale?: string): string {
  return getLocaleConfig(locale).dateLocale;
}

export function resolveNumberLocale(locale?: string): string {
  return getLocaleConfig(locale).numberLocale;
}
