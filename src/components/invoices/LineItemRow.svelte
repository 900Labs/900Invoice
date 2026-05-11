<script lang="ts">
  import CurrencyInput from '../shared/CurrencyInput.svelte';
  import { t } from '../../stores/i18nStore';
  import { calcLineTotal } from '../../utils/currency';
  import { formatCurrency } from '../../utils/currency';
  import type { TaxRate } from '../../stores/taxStore';

  interface Props {
    index: number;
    description: string;
    quantity: number;
    unitPriceMinor: number;
    taxRateId: string | null;
    discountPercent: number;
    currencyCode: string;
    taxRates: TaxRate[];
    products: Array<{ id: string; name: string; defaultPriceMinor: number; taxRateId: string | null }>;
    onupdate: (field: string, value: unknown) => void;
    ondelete: () => void;
  }

  let {
    index,
    description,
    quantity,
    unitPriceMinor,
    taxRateId,
    discountPercent,
    currencyCode,
    taxRates,
    products,
    onupdate,
    ondelete,
  }: Props = $props();

  let lineTotal = $derived(calcLineTotal(quantity, unitPriceMinor, discountPercent));

  function handleProductSelect(e: Event) {
    const productId = (e.target as HTMLSelectElement).value;
    if (!productId) return;
    const product = products.find(p => p.id === productId);
    if (product) {
      onupdate('productId', productId);
      onupdate('description', product.name);
      onupdate('unitPriceMinor', product.defaultPriceMinor);
      onupdate('taxRateId', product.taxRateId);
    }
  }
</script>

<tr class="line-item-row">
  <td class="col-drag">
    <span class="drag-handle" title={t('common.reorder')}>⠿</span>
  </td>
  <td class="col-product">
    <select class="select select-sm" onchange={handleProductSelect}>
      <option value="">{t('invoices.selectProduct')}</option>
      {#each products as product}
        <option value={product.id}>{product.name}</option>
      {/each}
    </select>
  </td>
  <td class="col-description">
    <input
      class="input input-sm"
      type="text"
      value={description}
      placeholder={t('invoices.description')}
      oninput={(e) => onupdate('description', (e.target as HTMLInputElement).value)}
    />
  </td>
  <td class="col-qty">
    <input
      class="input input-sm text-end"
      type="number"
      min="0"
      step="1"
      value={quantity}
      oninput={(e) => onupdate('quantity', parseFloat((e.target as HTMLInputElement).value) || 0)}
    />
  </td>
  <td class="col-price">
    <CurrencyInput
      value={unitPriceMinor}
      currencyCode={currencyCode}
      onchange={(v) => onupdate('unitPriceMinor', v)}
    />
  </td>
  <td class="col-tax">
    <select
      class="select select-sm"
      value={taxRateId ?? ''}
      onchange={(e) => onupdate('taxRateId', (e.target as HTMLSelectElement).value || null)}
    >
      <option value="">{t('invoices.noTaxRate')}</option>
      {#each taxRates as rate}
        <option value={rate.id}>{rate.displayName} ({(rate.rateBps / 100).toFixed(0)}%)</option>
      {/each}
    </select>
  </td>
  <td class="col-discount">
    <input
      class="input input-sm text-end"
      type="number"
      min="0"
      max="100"
      step="0.1"
      value={discountPercent}
      oninput={(e) => onupdate('discountPercent', parseFloat((e.target as HTMLInputElement).value) || 0)}
    />
  </td>
  <td class="col-total currency">
    {formatCurrency(lineTotal, currencyCode)}
  </td>
  <td class="col-delete">
    <button class="btn btn-ghost btn-icon btn-sm delete-btn" onclick={ondelete} title={t('common.removeLine')}>✕</button>
  </td>
</tr>

<style>
  .line-item-row td {
    padding: var(--space-xs);
    vertical-align: middle;
  }

  .col-drag { width: 24px; }
  .col-product { width: 140px; }
  .col-description { min-width: 160px; }
  .col-qty { width: 72px; }
  .col-price { width: 130px; }
  .col-tax { width: 140px; }
  .col-discount { width: 80px; }
  .col-total { width: 110px; text-align: end; font-weight: 500; white-space: nowrap; }
  .col-delete { width: 32px; }

  .drag-handle {
    color: var(--color-text-muted);
    cursor: grab;
    font-size: 1.2rem;
    line-height: 1;
  }

  .input-sm {
    padding: 5px var(--space-sm);
    font-size: var(--font-size-xs);
  }

  .select-sm {
    padding: 5px var(--space-sm);
    padding-inline-end: 28px;
    font-size: var(--font-size-xs);
    background-size: 10px;
    background-position: right 8px center;
  }

  .delete-btn {
    color: var(--color-text-muted);
  }

  .delete-btn:hover {
    color: var(--color-danger);
    background: rgba(220, 38, 38, 0.08);
  }
</style>
