<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getAllCurrencies, formatTaxRate } from '../../utils/currency';
  import { getActiveTaxRates } from '../../stores/taxStore';
  import type { Product, CreateProduct } from '../../stores/productStore';

  interface Props {
    product?: Product;
    oncreate?: (data: CreateProduct) => void;
    onupdate?: (data: Partial<CreateProduct>) => void;
    oncancel?: () => void;
  }

  let { product, oncreate, onupdate, oncancel }: Props = $props();

  let currencies = getAllCurrencies();
  let taxRates = $derived(getActiveTaxRates());

  let name = $state('');
  let description = $state('');
  let defaultPrice = $state(0);
  let currencyCode = $state('USD');
  let taxRateId = $state<string | null>(null);
  let unit = $state('pcs');
  let isActive = $state(true);

  // Sync from prop (edit mode)
  $effect(() => {
    if (product) {
      name = product.name ?? '';
      description = product.description ?? '';
      defaultPrice = product.defaultPriceMinor / 100;
      currencyCode = product.currencyCode ?? 'USD';
      taxRateId = product.taxRateId ?? taxRates.find(rate => rate.rateBps === product.taxRateBps)?.id ?? null;
      unit = product.unit ?? 'pcs';
      isActive = product.isActive ?? true;
    }
  });

  let errors = $state<Record<string, string>>({});

  function handleSubmit() {
    errors = {};
    if (!name.trim()) {
      errors.name = t('validation.required').replace('{field}', t('products.name'));
      return;
    }

    const data: CreateProduct = {
      name: name.trim(),
      description: description.trim(),
      defaultPriceMinor: Math.round(defaultPrice * 100),
      currencyCode,
      taxRateId,
      unit: unit.trim(),
      isActive,
    };

    if (product) {
      onupdate?.(data);
    } else {
      oncreate?.(data);
    }
  }
</script>

<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <div class="form-row">
    <div class="form-group" style="grid-column: 1 / -1;">
      <label class="form-label required" for="lbl-products-name">{t('products.name')}</label>
      <input id="lbl-products-name" class="input {errors.name ? 'error' : ''}" type="text" bind:value={name} />
      {#if errors.name}<span class="form-error">{errors.name}</span>{/if}
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-products-description">{t('products.description')}</label>
    <textarea id="lbl-products-description" class="textarea" rows="2" bind:value={description}></textarea>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-products-defaultprice">{t('products.defaultPrice')}</label>
      <input id="lbl-products-defaultprice" class="input" type="number" min="0" step="0.01" bind:value={defaultPrice} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-products-currency">{t('products.currency')}</label>
      <select id="lbl-products-currency" class="select" bind:value={currencyCode}>
        {#each currencies as c}
          <option value={c.code}>{c.code} — {c.name}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-products-taxrate">{t('products.taxRate')}</label>
      <select id="lbl-products-taxrate" class="select" bind:value={taxRateId}>
        <option value={null}>{t('invoices.noTaxRate')}</option>
        {#each taxRates as rate}
          <option value={rate.id}>{rate.displayName} ({formatTaxRate(rate.rateBps)})</option>
        {/each}
      </select>
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-products-unit">{t('products.unit')}</label>
      <input id="lbl-products-unit" class="input" type="text" bind:value={unit} placeholder="pcs, hrs, kg..." />
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer;">
      <input type="checkbox" bind:checked={isActive} />
      {t('products.active')}
    </label>
  </div>

  <div class="form-actions mt-md">
    {#if oncancel}
      <button type="button" class="btn" onclick={oncancel}>{t('common.cancel')}</button>
    {/if}
    <button type="submit" class="btn btn-primary">
      {product ? t('common.save') : t('common.create')}
    </button>
  </div>
</form>

<style>
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
  }
  .mt-md { margin-block-start: var(--space-md); }
</style>
