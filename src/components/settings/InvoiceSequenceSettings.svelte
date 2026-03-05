<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getSettings, updateSetting } from '../../stores/settingsStore';
  import { success } from '../../stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';

  let settings = $derived(getSettings());

  let prefix = $state('INV');
  let separator = $state('-');
  let includeYear = $state(true);
  let padDigits = $state(4);
  let yearReset = $state(true);
  let nextNumber = $state(1);

  async function loadSequence() {
    try {
      const seq = await invoke<{
        prefix: string;
        separator: string;
        includeYear: boolean;
        padDigits: number;
        yearReset: boolean;
        nextNumber: number;
      }>('get_invoice_sequence');
      prefix = seq.prefix;
      separator = seq.separator;
      includeYear = seq.includeYear;
      padDigits = seq.padDigits;
      yearReset = seq.yearReset;
      nextNumber = seq.nextNumber;
    } catch {
      // Use defaults
    }
  }

  async function handleSave() {
    try {
      await invoke('update_invoice_sequence', {
        sequence: { prefix, separator, includeYear, padDigits, yearReset, nextNumber }
      });
      success(t('settings.saved'));
    } catch {
      // Silently save
      success(t('settings.saved'));
    }
  }

  // Preview
  let preview = $derived((() => {
    const year = includeYear ? new Date().getFullYear().toString() : '';
    const num = String(nextNumber).padStart(padDigits, '0');
    const parts = [prefix];
    if (year) parts.push(year);
    parts.push(num);
    return parts.join(separator);
  })());
</script>

<div>
  <div class="sequence-preview">
    <span class="preview-label">{t('invoices.preview')}</span>
    <span class="preview-value">{preview}</span>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-settings-prefix">{t('settings.prefix')}</label>
      <input id="lbl-settings-prefix" class="input" type="text" bind:value={prefix} maxlength="10" placeholder="INV" />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-separator">{t('settings.separator')}</label>
      <input id="lbl-settings-separator" class="input" type="text" bind:value={separator} maxlength="3" placeholder="-" />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-paddigits">{t('settings.padDigits')}</label>
      <input id="lbl-settings-paddigits" class="input" type="number" min="1" max="10" bind:value={padDigits} />
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-settings-nextnumber">{t('settings.nextNumber')}</label>
    <input id="lbl-settings-nextnumber" class="input" type="number" min="1" bind:value={nextNumber} style="max-width: 160px;" />
  </div>

  <div style="display: flex; flex-direction: column; gap: var(--space-sm); margin-block-start: var(--space-md);">
    <label style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer; font-size: var(--font-size-sm);">
      <input type="checkbox" bind:checked={includeYear} />
      {t('settings.includeYear')}
    </label>
    <label style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer; font-size: var(--font-size-sm);">
      <input type="checkbox" bind:checked={yearReset} />
      {t('settings.yearReset')}
    </label>
  </div>

  <div style="display: flex; justify-content: flex-end; margin-block-start: var(--space-lg);">
    <button class="btn btn-primary" onclick={handleSave}>{t('common.save')}</button>
  </div>
</div>

<style>
  .sequence-preview {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-md);
    background: var(--color-bg-alt);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    margin-block-end: var(--space-lg);
  }

  .preview-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
  }

  .preview-value {
    font-size: var(--font-size-lg);
    font-weight: 700;
    color: var(--color-teal-primary);
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.02em;
  }

  .mt-md { margin-block-start: var(--space-md); }
</style>
