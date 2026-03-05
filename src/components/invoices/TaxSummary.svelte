<script lang="ts">
  import { formatCurrency, formatTaxRate } from '../../utils/currency';
  import { t } from '../../stores/i18nStore';
  import type { TaxLine } from '../../stores/invoiceStore';

  interface Props {
    taxLines: TaxLine[];
    subtotalMinor: number;
    discountMinor: number;
    taxTotalMinor: number;
    totalMinor: number;
    amountPaidMinor?: number;
    balanceDueMinor?: number;
    currencyCode: string;
  }

  let {
    taxLines,
    subtotalMinor,
    discountMinor,
    taxTotalMinor,
    totalMinor,
    amountPaidMinor = 0,
    balanceDueMinor,
    currencyCode,
  }: Props = $props();
</script>

<div class="tax-summary">
  <table class="totals-table">
    <tbody>
      <tr>
        <td class="label">{t('invoices.subtotal')}</td>
        <td class="amount currency">{formatCurrency(subtotalMinor, currencyCode)}</td>
      </tr>
      {#if discountMinor > 0}
        <tr>
          <td class="label discount-label">
            {t('invoices.discount')}
          </td>
          <td class="amount currency discount-amount">
            −{formatCurrency(discountMinor, currencyCode)}
          </td>
        </tr>
      {/if}
      {#each taxLines as line}
        <tr class="tax-line">
          <td class="label">
            {line.taxDisplayName} ({formatTaxRate(line.rateBps)})
          </td>
          <td class="amount currency">{formatCurrency(line.taxAmountMinor, currencyCode)}</td>
        </tr>
      {/each}
      {#if taxTotalMinor > 0 && taxLines.length > 1}
        <tr>
          <td class="label"><strong>{t('invoices.taxTotal')}</strong></td>
          <td class="amount currency"><strong>{formatCurrency(taxTotalMinor, currencyCode)}</strong></td>
        </tr>
      {/if}
      <tr class="total-row">
        <td class="label">{t('invoices.total')}</td>
        <td class="amount currency">{formatCurrency(totalMinor, currencyCode)}</td>
      </tr>
      {#if amountPaidMinor > 0}
        <tr>
          <td class="label">{t('invoices.amountPaid')}</td>
          <td class="amount currency paid">{formatCurrency(amountPaidMinor, currencyCode)}</td>
        </tr>
        <tr class="balance-row">
          <td class="label">{t('invoices.balanceDue')}</td>
          <td class="amount currency balance">
            {formatCurrency(balanceDueMinor ?? (totalMinor - amountPaidMinor), currencyCode)}
          </td>
        </tr>
      {/if}
    </tbody>
  </table>
</div>

<style>
  .tax-summary {
    width: 100%;
    max-width: 320px;
    margin-inline-start: auto;
  }

  .totals-table {
    width: 100%;
    border-collapse: collapse;
  }

  .totals-table td {
    padding: 5px var(--space-sm);
    font-size: var(--font-size-sm);
  }

  .totals-table .amount {
    text-align: end;
    font-variant-numeric: tabular-nums;
  }

  .discount-label { color: var(--color-success); }
  .discount-amount { color: var(--color-success); }

  .tax-line .label { color: var(--color-text-secondary); }

  .total-row td {
    font-weight: 700;
    font-size: var(--font-size-base);
    border-top: 2px solid var(--color-border);
    padding-block-start: var(--space-sm);
  }

  .paid { color: var(--color-success); }

  .balance-row td {
    font-weight: 700;
    font-size: var(--font-size-lg);
    color: var(--color-teal-primary);
  }
</style>
