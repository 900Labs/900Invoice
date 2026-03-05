<script lang="ts">
  import Modal from '../shared/Modal.svelte';
  import ConfirmDialog from '../shared/ConfirmDialog.svelte';
  import { getTaxRates, loadTaxRates, createTaxRate, updateTaxRate, deleteTaxRate, getLoading } from '../../stores/taxStore';
  import { formatTaxRate } from '../../utils/currency';
  import { t } from '../../stores/i18nStore';
  import { success } from '../../stores/toastStore';
  import type { TaxRate, CreateTaxRate } from '../../stores/taxStore';
  import { onMount } from 'svelte';

  let taxRates = $derived(getTaxRates());
  let loading = $derived(getLoading());

  let showAddModal = $state(false);
  let editingRate = $state<TaxRate | null>(null);
  let deletingId = $state<string | null>(null);
  let filterCountry = $state('');

  // Form state
  let formName = $state('');
  let formDisplayName = $state('');
  let formRatePercent = $state(0);
  let formCountry = $state('');
  let formIsDefault = $state(false);
  let formIsWithholding = $state(false);
  let formIsInclusive = $state(false);

  let filteredRates = $derived(
    filterCountry
      ? taxRates.filter(r => r.country === filterCountry)
      : taxRates
  );

  let countries = $derived([...new Set(taxRates.map(r => r.country).filter(Boolean))]);

  onMount(() => loadTaxRates());

  function openAdd() {
    formName = ''; formDisplayName = ''; formRatePercent = 0;
    formCountry = ''; formIsDefault = false; formIsWithholding = false; formIsInclusive = false;
    showAddModal = true;
  }

  function openEdit(rate: TaxRate) {
    editingRate = rate;
    formName = rate.name;
    formDisplayName = rate.displayName;
    formRatePercent = rate.rateBps / 100;
    formCountry = rate.country;
    formIsDefault = rate.isDefault;
    formIsWithholding = rate.isWithholding;
    formIsInclusive = rate.isInclusive;
  }

  async function handleSave() {
    const data: CreateTaxRate = {
      name: formName,
      displayName: formDisplayName,
      rateBps: Math.round(formRatePercent * 100),
      country: formCountry,
      isDefault: formIsDefault,
      isWithholding: formIsWithholding,
      isInclusive: formIsInclusive,
    };

    if (editingRate) {
      await updateTaxRate(editingRate.id, data);
      editingRate = null;
    } else {
      await createTaxRate(data);
      showAddModal = false;
    }
    success(t('common.success'));
  }

  async function handleDelete() {
    if (!deletingId) return;
    await deleteTaxRate(deletingId);
    deletingId = null;
    success(t('common.success'));
  }
</script>

<div>
  <div class="rates-toolbar">
    <select class="select" style="width: auto; min-width: 140px;" bind:value={filterCountry}>
      <option value="">{t('common.all')}</option>
      {#each countries as c}
        <option value={c}>{c}</option>
      {/each}
    </select>
    <button class="btn btn-primary" onclick={openAdd}>
      + {t('taxes.addTaxRate')}
    </button>
  </div>

  {#if filteredRates.length === 0}
    <p class="no-rates">{t('taxes.noTaxRates')}</p>
  {:else}
    <table class="table" style="margin-block-start: var(--space-md);">
      <thead>
        <tr>
          <th>{t('taxes.name')}</th>
          <th>{t('taxes.displayName')}</th>
          <th style="text-align: end;">{t('taxes.rate')}</th>
          <th>{t('taxes.country')}</th>
          <th>{t('taxes.isDefault')}</th>
          <th>{t('taxes.isWithholding')}</th>
          <th>{t('common.actions')}</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredRates as rate}
          <tr>
            <td class="font-medium">{rate.name}</td>
            <td>{rate.displayName}</td>
            <td style="text-align: end;">{formatTaxRate(rate.rateBps)}</td>
            <td>{rate.country || '—'}</td>
            <td>{rate.isDefault ? '✓' : '—'}</td>
            <td>{rate.isWithholding ? '✓' : '—'}</td>
            <td>
              <div style="display: flex; gap: var(--space-xs);">
                <button class="btn btn-ghost btn-sm" onclick={() => openEdit(rate)}>{t('common.edit')}</button>
                <button
                  class="btn btn-ghost btn-sm"
                  style="color: var(--color-danger);"
                  onclick={() => deletingId = rate.id}
                >{t('common.delete')}</button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<!-- Add/Edit Modal -->
{#if showAddModal || editingRate}
  <Modal
    title={editingRate ? t('taxes.editTaxRate') : t('taxes.addTaxRate')}
    onclose={() => { showAddModal = false; editingRate = null; }}
    maxWidth="480px"
  >
    <div class="form-group">
      <label class="form-label" for="lbl-taxes-name">{t('taxes.name')}</label>
      <input id="lbl-taxes-name" class="input" type="text" bind:value={formName} placeholder="VAT, GST, WHT..." />
    </div>
    <div class="form-group mt-md">
      <label class="form-label" for="lbl-taxes-displayname">{t('taxes.displayName')}</label>
      <input id="lbl-taxes-displayname" class="input" type="text" bind:value={formDisplayName} placeholder="VAT 16%" />
    </div>
    <div class="form-row mt-md">
      <div class="form-group">
        <label class="form-label" for="lbl-taxes-rate">{t('taxes.rate')} (%)</label>
        <input id="lbl-taxes-rate" class="input" type="number" min="0" max="100" step="0.01" bind:value={formRatePercent} />
      </div>
      <div class="form-group">
        <label class="form-label" for="lbl-taxes-country">{t('taxes.country')}</label>
        <input id="lbl-taxes-country" class="input" type="text" bind:value={formCountry} placeholder="KE, NG, ZA..." />
      </div>
    </div>
    <div style="display: flex; flex-direction: column; gap: var(--space-sm); margin-block-start: var(--space-md);">
      <label style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer; font-size: var(--font-size-sm);">
        <input type="checkbox" bind:checked={formIsDefault} />
        {t('taxes.isDefault')}
      </label>
      <label style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer; font-size: var(--font-size-sm);">
        <input type="checkbox" bind:checked={formIsWithholding} />
        {t('taxes.isWithholding')}
      </label>
      <label style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer; font-size: var(--font-size-sm);">
        <input type="checkbox" bind:checked={formIsInclusive} />
        {t('taxes.isInclusive')}
      </label>
    </div>

    {#snippet footer()}
      <button class="btn" onclick={() => { showAddModal = false; editingRate = null; }}>{t('common.cancel')}</button>
      <button class="btn btn-primary" onclick={handleSave}>{t('common.save')}</button>
    {/snippet}
  </Modal>
{/if}

{#if deletingId}
  <ConfirmDialog
    message={t('taxes.noTaxRates')}
    danger={true}
    onconfirm={handleDelete}
    oncancel={() => deletingId = null}
  />
{/if}

<style>
  .rates-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-sm);
  }

  .no-rates {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    padding: var(--space-lg);
  }

  .mt-md { margin-block-start: var(--space-md); }
</style>
