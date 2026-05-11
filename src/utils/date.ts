// Date utilities
import { resolveDateLocale } from './locale';

/**
 * Format an ISO date string to a human-readable format.
 */
export function formatDate(isoDate: string, format: string = 'YYYY-MM-DD', locale?: string): string {
  if (!isoDate) return '';
  try {
    const date = new Date(isoDate);
    if (isNaN(date.getTime())) return isoDate;

    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, '0');
    const d = String(date.getDate()).padStart(2, '0');

    switch (format) {
      case 'YYYY-MM-DD':
        return `${y}-${m}-${d}`;
      case 'DD/MM/YYYY':
        return `${d}/${m}/${y}`;
      case 'MM/DD/YYYY':
        return `${m}/${d}/${y}`;
      case 'DD.MM.YYYY':
        return `${d}.${m}.${y}`;
      case 'MMM D, YYYY':
        return new Intl.DateTimeFormat(resolveDateLocale(locale), {
          month: 'short',
          day: 'numeric',
          year: 'numeric',
        }).format(date);
      default:
        return `${y}-${m}-${d}`;
    }
  } catch {
    return isoDate;
  }
}

/**
 * Parse a date string to ISO format (YYYY-MM-DD).
 */
export function parseDate(dateStr: string): string {
  if (!dateStr) return '';
  try {
    const date = new Date(dateStr);
    if (isNaN(date.getTime())) return '';
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, '0');
    const d = String(date.getDate()).padStart(2, '0');
    return `${y}-${m}-${d}`;
  } catch {
    return '';
  }
}

/**
 * Add days to an ISO date string.
 */
export function addDays(isoDate: string, days: number): string {
  if (!isoDate) return '';
  const date = new Date(isoDate);
  date.setDate(date.getDate() + days);
  return parseDate(date.toISOString());
}

/**
 * Check if a due date is overdue.
 */
export function isOverdue(dueDate: string): boolean {
  if (!dueDate) return false;
  const due = new Date(dueDate);
  const now = new Date();
  now.setHours(0, 0, 0, 0);
  return due < now;
}

/**
 * Get number of days until a due date (negative if overdue).
 */
export function daysUntilDue(dueDate: string): number {
  if (!dueDate) return 0;
  const due = new Date(dueDate);
  const now = new Date();
  now.setHours(0, 0, 0, 0);
  due.setHours(0, 0, 0, 0);
  return Math.round((due.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
}

/**
 * Get a date range for a given period.
 */
export function getDateRange(period: 'week' | 'month' | 'quarter' | 'year'): { start: string; end: string } {
  const now = new Date();
  let start: Date;
  let end: Date = new Date(now);

  switch (period) {
    case 'week': {
      start = new Date(now);
      start.setDate(now.getDate() - now.getDay());
      break;
    }
    case 'month': {
      start = new Date(now.getFullYear(), now.getMonth(), 1);
      break;
    }
    case 'quarter': {
      const quarterStart = Math.floor(now.getMonth() / 3) * 3;
      start = new Date(now.getFullYear(), quarterStart, 1);
      break;
    }
    case 'year': {
      start = new Date(now.getFullYear(), 0, 1);
      break;
    }
  }

  return {
    start: parseDate(start.toISOString()),
    end: parseDate(end.toISOString()),
  };
}

/**
 * Get today's date as ISO string.
 */
export function today(): string {
  return parseDate(new Date().toISOString());
}

/**
 * Format relative time (e.g., "2 days ago").
 */
export function timeAgo(isoDate: string): string {
  const date = new Date(isoDate);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  const rtf = new Intl.RelativeTimeFormat(resolveDateLocale(), { numeric: 'auto' });
  if (days > 7) return formatDate(isoDate, 'MMM D, YYYY');
  if (days > 0) return rtf.format(-days, 'day');
  if (hours > 0) return rtf.format(-hours, 'hour');
  if (minutes > 0) return rtf.format(-minutes, 'minute');
  return rtf.format(0, 'second');
}
