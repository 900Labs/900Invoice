// Validation utilities

export interface ValidationResult {
  valid: boolean;
  errors: Record<string, string>;
}

export function validateEmail(email: string): boolean {
  if (!email) return false;
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
}

export function validatePhone(phone: string): boolean {
  if (!phone) return true; // phone is optional
  return /^[\+\d\s\-\(\)]{7,20}$/.test(phone);
}

export function validateRequired(value: string, fieldName: string): string | null {
  if (!value || value.trim() === '') {
    return `${fieldName} is required`;
  }
  return null;
}

export function validatePositiveAmount(value: number): string | null {
  if (isNaN(value) || value <= 0) {
    return 'Amount must be positive';
  }
  return null;
}

export interface CreateInvoiceValidation {
  clientId: string;
  lineItems: Array<{
    description: string;
    quantity: number;
    unitPriceMinor: number;
  }>;
}

export function validateInvoice(invoice: CreateInvoiceValidation): ValidationResult {
  const errors: Record<string, string> = {};

  if (!invoice.clientId) {
    errors.clientId = 'Please select a client';
  }

  if (!invoice.lineItems || invoice.lineItems.length === 0) {
    errors.lineItems = 'Add at least one line item';
  } else {
    invoice.lineItems.forEach((item, i) => {
      if (!item.description || item.description.trim() === '') {
        errors[`lineItem_${i}_description`] = 'Description is required';
      }
      if (item.quantity <= 0) {
        errors[`lineItem_${i}_quantity`] = 'Quantity must be positive';
      }
      if (item.unitPriceMinor < 0) {
        errors[`lineItem_${i}_unitPrice`] = 'Price cannot be negative';
      }
    });
  }

  return {
    valid: Object.keys(errors).length === 0,
    errors,
  };
}

export interface CreateClientValidation {
  name: string;
  email: string;
  phone: string;
}

export function validateClient(client: CreateClientValidation): ValidationResult {
  const errors: Record<string, string> = {};

  const nameError = validateRequired(client.name, 'Name');
  if (nameError) errors.name = nameError;

  if (client.email && !validateEmail(client.email)) {
    errors.email = 'Invalid email address';
  }

  if (client.phone && !validatePhone(client.phone)) {
    errors.phone = 'Invalid phone number';
  }

  return {
    valid: Object.keys(errors).length === 0,
    errors,
  };
}

export function validateTaxRate(data: { name: string; rateBps: number }): ValidationResult {
  const errors: Record<string, string> = {};

  const nameError = validateRequired(data.name, 'Name');
  if (nameError) errors.name = nameError;

  if (isNaN(data.rateBps) || data.rateBps < 0 || data.rateBps > 10000) {
    errors.rateBps = 'Tax rate must be between 0% and 100%';
  }

  return {
    valid: Object.keys(errors).length === 0,
    errors,
  };
}
