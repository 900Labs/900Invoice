<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getSettings, updateSetting } from '../../stores/settingsStore';
  import { getAllCurrencies } from '../../utils/currency';
  import { success } from '../../stores/toastStore';

  let settings = $derived(getSettings());
  let currencies = getAllCurrencies();

  let defaultCurrency = $state('USD');
  let currencyPosition = $state<'before' | 'after'>('before');

  $effect(() => {
    defaultCurrency = settings.defaultCurrency;
    currencyPosition = settings.currencyPosition;
  });

  async function handleSave() {
    await updateSetting('defaultCurrency', defaultCurrency);
    await updateSetting('currencyPosition', currencyPosition);
    success(t('settings.saved'));
  }
</script>

<div>
  <div class="form-group">
    <label class="form-label" for="lbl-settings-defaultcurrency">{t('settings.defaultCurrency')}</label>
    <select id="lbl-settings-defaultcurrency" class="select" bind:value={defaultCurrency} style="max-width: 320px;">
      {#each currencies as c}
        <option value={c.code}>{c.code} — {c.name}</option>
      {/each}
    </select>
    <p class="form-hint mt-xs">{t('settings.defaultCurrencyHint')}</p>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-settings-currencyposition">{t('settings.currencyPosition')}</label>
    <div class="position-toggle">
      <label class="position-option">
        <input id="lbl-settings-currencyposition"
          type="radio"
          bind:group={currencyPosition}
          value="before"
        />
        <span class="position-example">
          <span class="position-symbol">KSh</span> 1,500.00
        </span>
        <span class="position-label">{t('settings.positionBefore')}</span>
      </label>
      <label class="position-option">
        <input
          type="radio"
          bind:group={currencyPosition}
          value="after"
        />
        <span class="position-example">
          1,500.00 <span class="position-symbol">KSh</span>
        </span>
        <span class="position-label">{t('settings.positionAfter')}</span>
      </label>
    </div>
  </div>

  <div style="display: flex; justify-content: flex-end; margin-block-start: var(--space-lg);">
    <button class="btn btn-primary" onclick={handleSave}>{t('common.save')}</button>
  </div>
</div>

<style>
  .mt-md { margin-block-start: var(--space-md); }
  .mt-xs { margin-block-start: var(--space-xs); }

  .position-toggle {
    display: flex;
    gap: var(--space-md);
    flex-wrap: wrap;
  }

  .position-option {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all 0.15s;
  }

  .position-option:has(input:checked) {
    border-color: var(--color-teal-primary);
    background: rgba(32, 128, 141, 0.05);
  }

  .position-example {
    font-size: var(--font-size-sm);
    font-variant-numeric: tabular-nums;
    color: var(--color-text);
    font-weight: 500;
  }

  .position-symbol {
    font-weight: 700;
    color: var(--color-teal-primary);
  }

  .position-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
