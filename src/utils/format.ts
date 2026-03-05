// Format utilities

/**
 * Truncate a string to a maximum length with ellipsis.
 */
export function truncate(str: string, maxLength: number): string {
  if (!str) return '';
  if (str.length <= maxLength) return str;
  return str.slice(0, maxLength - 3) + '...';
}

/**
 * Format an invoice number for display.
 */
export function formatInvoiceNumber(num: string): string {
  if (!num) return '';
  return num;
}

/**
 * Format a phone number for display.
 */
export function formatPhoneNumber(phone: string, countryCode: string = ''): string {
  if (!phone) return '';
  const cleaned = phone.replace(/\D/g, '');
  if (countryCode) {
    return `+${countryCode} ${cleaned}`;
  }
  return phone;
}

/**
 * Pluralize a word based on count.
 */
export function pluralize(count: number, singular: string, plural: string): string {
  return count === 1 ? `${count} ${singular}` : `${count} ${plural}`;
}

/**
 * Format bytes to human-readable size.
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

/**
 * Format a percentage.
 */
export function formatPercent(value: number, decimals: number = 1): string {
  return `${value.toFixed(decimals)}%`;
}

/**
 * Get initials from a name (up to 2 characters).
 */
export function getInitials(name: string): string {
  if (!name) return '?';
  const parts = name.trim().split(/\s+/);
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase();
  return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
}

/**
 * Capitalize first letter of a string.
 */
export function capitalize(str: string): string {
  if (!str) return '';
  return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Convert snake_case or kebab-case to Title Case.
 */
export function toTitleCase(str: string): string {
  return str
    .split(/[_\-\s]+/)
    .map(word => capitalize(word))
    .join(' ');
}
