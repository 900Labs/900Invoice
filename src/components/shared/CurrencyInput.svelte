<script lang="ts">
  import { minorToMajor, majorToMinor, getCurrencyConfig } from '../../utils/currency';

  interface Props {
    value: number; // minor units
    currencyCode: string;
    onchange?: (minorUnits: number) => void;
    disabled?: boolean;
    placeholder?: string;
    class?: string;
  }

  let { value, currencyCode, onchange, disabled = false, placeholder = '0.00', class: klass = '' }: Props = $props();

  let config = $derived(getCurrencyConfig(currencyCode));
  let focused = $state(false);
  let rawInput = $state('');

  let displayValue = $derived(() => {
    if (focused) return rawInput;
    const major = minorToMajor(value, currencyCode);
    return major.toFixed(config.decimals);
  });

  function handleFocus() {
    focused = true;
    const major = minorToMajor(value, currencyCode);
    rawInput = major === 0 ? '' : major.toFixed(config.decimals);
  }

  function handleBlur() {
    focused = false;
    const parsed = parseFloat(rawInput);
    if (!isNaN(parsed) && parsed >= 0) {
      const minor = majorToMinor(parsed, currencyCode);
      onchange?.(minor);
    } else if (rawInput === '' || rawInput === '0') {
      onchange?.(0);
    }
    rawInput = '';
  }

  function handleInput(e: Event) {
    rawInput = (e.target as HTMLInputElement).value;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      (e.target as HTMLInputElement).blur();
    }
  }
</script>

<div class="currency-input {klass}">
  <span class="currency-symbol">{config.symbol}</span>
  <input
    class="currency-field"
    type="text"
    inputmode="decimal"
    value={displayValue()}
    {disabled}
    {placeholder}
    onfocus={handleFocus}
    onblur={handleBlur}
    oninput={handleInput}
    onkeydown={handleKeydown}
    aria-label="Amount in {currencyCode}"
  />
</div>

<style>
  .currency-input {
    display: flex;
    align-items: center;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg-card);
    overflow: hidden;
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
  }

  .currency-input:focus-within {
    border-color: var(--color-teal-primary);
    box-shadow: 0 0 0 3px rgba(32, 128, 141, 0.1);
  }

  .currency-symbol {
    padding: var(--space-sm);
    padding-inline-end: var(--space-xs);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: 500;
    white-space: nowrap;
    user-select: none;
    background: var(--color-bg-alt);
    border-inline-end: 1px solid var(--color-border);
  }

  .currency-field {
    flex: 1;
    border: none;
    background: transparent;
    padding: var(--space-sm);
    font-size: var(--font-size-sm);
    color: var(--color-text);
    text-align: end;
    min-width: 0;
    font-variant-numeric: tabular-nums;
  }

  .currency-field:focus {
    outline: none;
  }

  .currency-field:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
