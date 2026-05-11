// Currency utilities
import { resolveNumberLocale } from './locale';

export interface CurrencyConfig {
  code: string;
  symbol: string;
  decimals: number;
  name: string;
}

const CURRENCY_CONFIG: Record<string, CurrencyConfig> = {
  KES: { code: 'KES', symbol: 'KSh', decimals: 2, name: 'Kenyan Shilling' },
  NGN: { code: 'NGN', symbol: '₦', decimals: 2, name: 'Nigerian Naira' },
  ZAR: { code: 'ZAR', symbol: 'R', decimals: 2, name: 'South African Rand' },
  INR: { code: 'INR', symbol: '₹', decimals: 2, name: 'Indian Rupee' },
  TZS: { code: 'TZS', symbol: 'TSh', decimals: 2, name: 'Tanzanian Shilling' },
  UGX: { code: 'UGX', symbol: 'USh', decimals: 0, name: 'Ugandan Shilling' },
  GHS: { code: 'GHS', symbol: 'GH₵', decimals: 2, name: 'Ghanaian Cedi' },
  XOF: { code: 'XOF', symbol: 'CFA', decimals: 0, name: 'West African CFA Franc' },
  XAF: { code: 'XAF', symbol: 'FCFA', decimals: 0, name: 'Central African CFA Franc' },
  USD: { code: 'USD', symbol: '$', decimals: 2, name: 'US Dollar' },
  EUR: { code: 'EUR', symbol: '€', decimals: 2, name: 'Euro' },
};

export function getCurrencyConfig(code: string): CurrencyConfig {
  return CURRENCY_CONFIG[code] ?? { code, symbol: code, decimals: 2, name: code };
}

export function getAllCurrencies(): CurrencyConfig[] {
  return Object.values(CURRENCY_CONFIG);
}

/**
 * Format minor units (integer) into a display string.
 * e.g. formatCurrency(150000, 'KES') → 'KSh 1,500.00'
 */
export function formatCurrency(
  minorUnits: number,
  currencyCode: string,
  position: 'before' | 'after' = 'before',
  locale?: string
): string {
  const config = getCurrencyConfig(currencyCode);
  const divisor = Math.pow(10, config.decimals);
  const major = minorUnits / divisor;
  const formatted = new Intl.NumberFormat(resolveNumberLocale(locale), {
    minimumFractionDigits: config.decimals,
    maximumFractionDigits: config.decimals,
  }).format(major);
  if (position === 'after') {
    return `${formatted} ${config.symbol}`;
  }
  return `${config.symbol} ${formatted}`;
}

/**
 * Convert minor units to major (display) value.
 * e.g. minorToMajor(150000, 'KES') → 1500.00
 */
export function minorToMajor(minor: number, currencyCode: string): number {
  const config = getCurrencyConfig(currencyCode);
  return minor / Math.pow(10, config.decimals);
}

/**
 * Convert major (display) value to minor units.
 * e.g. majorToMinor(1500, 'KES') → 150000
 */
export function majorToMinor(major: number, currencyCode: string): number {
  const config = getCurrencyConfig(currencyCode);
  return Math.round(major * Math.pow(10, config.decimals));
}

/**
 * Format tax rate in basis points to percentage string.
 * e.g. formatTaxRate(1600) → '16.00%'
 */
export function formatTaxRate(bps: number): string {
  return (bps / 100).toFixed(2) + '%';
}

/**
 * Parse basis points from percentage string.
 * e.g. parseTaxRate('16') → 1600
 */
export function parseTaxRate(percent: string): number {
  return Math.round(parseFloat(percent) * 100);
}

/**
 * Calculate line item total in minor units (before tax).
 */
export function calcLineTotal(
  quantity: number,
  unitPriceMinor: number,
  discountPercent: number
): number {
  const raw = quantity * unitPriceMinor;
  const discount = Math.round(raw * (discountPercent / 100));
  return raw - discount;
}

/**
 * Calculate tax amount for a line in minor units.
 */
export function calcLineTax(
  lineTotalMinor: number,
  taxRateBps: number,
  mode: 'Exclusive' | 'Inclusive'
): number {
  if (mode === 'Exclusive') {
    return Math.round(lineTotalMinor * (taxRateBps / 10000));
  } else {
    // Inclusive: extract tax from total
    return Math.round(lineTotalMinor * taxRateBps / (10000 + taxRateBps));
  }
}
