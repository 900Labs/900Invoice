import { formatCurrency } from './currency';

export interface CurrencyTotal {
  currencyCode: string;
  amountMinor: number;
}

export interface CurrencyTotalDetail {
  label: string;
  value: string;
}

export function currencyTotals<T>(
  items: T[],
  currencyCode: (item: T) => string,
  amountMinor: (item: T) => number
): CurrencyTotal[] {
  const totals = new Map<string, number>();

  for (const item of items) {
    const code = currencyCode(item);
    totals.set(code, (totals.get(code) ?? 0) + amountMinor(item));
  }

  return Array.from(totals.entries())
    .map(([code, amount]) => ({ currencyCode: code, amountMinor: amount }))
    .sort((a, b) => a.currencyCode.localeCompare(b.currencyCode));
}

export function currencyTotalDetails(
  totals: CurrencyTotal[],
  fallbackCurrency: string
): CurrencyTotalDetail[] {
  const nonZero = totals.filter(total => total.amountMinor !== 0);
  const displayTotals = nonZero.length > 0
    ? nonZero
    : [{ currencyCode: fallbackCurrency, amountMinor: 0 }];

  return displayTotals.map(total => ({
    label: total.currencyCode,
    value: formatCurrency(total.amountMinor, total.currencyCode),
  }));
}

export function hasPositiveCurrencyTotal(totals: CurrencyTotal[]): boolean {
  return totals.some(total => total.amountMinor > 0);
}
