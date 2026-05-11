<script lang="ts">
  import BusinessProfileForm from './BusinessProfileForm.svelte';
  import TaxRatesManager from './TaxRatesManager.svelte';
  import InvoiceSequenceSettings from './InvoiceSequenceSettings.svelte';
  import CurrencySettings from './CurrencySettings.svelte';
  import { t, setLocale, getCurrentLocale, SUPPORTED_LOCALES } from '../../stores/i18nStore';
  import {
    getSettings, updateSetting, loadSettings, loadBusinessProfile
  } from '../../stores/settingsStore';
  import { loadClients } from '../../stores/clientStore';
  import { loadInvoices } from '../../stores/invoiceStore';
  import { loadProducts } from '../../stores/productStore';
  import { loadTaxRates } from '../../stores/taxStore';
  import { success, error as toastError } from '../../stores/toastStore';
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';
  import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

  type Tab = 'profile' | 'taxes' | 'invoiceNumbers' | 'currency' | 'general' | 'importExport';
  type RestoreResult = { status: string; restored: Record<string, number> };
  type ImportResult = { imported: number; errors: string[] };

  let activeTab = $state<Tab>('profile');
  let settings = $derived(getSettings());
  let locale = $derived(getCurrentLocale());

  const tabs: Array<{ id: Tab; labelKey: string }> = [
    { id: 'profile', labelKey: 'settings.businessProfile' },
    { id: 'taxes', labelKey: 'settings.taxRates' },
    { id: 'invoiceNumbers', labelKey: 'settings.invoiceNumbers' },
    { id: 'currency', labelKey: 'settings.currencySettings' },
    { id: 'general', labelKey: 'settings.general' },
    { id: 'importExport', labelKey: 'settings.importExport' },
  ];

  const dateFormats = [
    { value: 'YYYY-MM-DD', label: 'YYYY-MM-DD (ISO)' },
    { value: 'DD/MM/YYYY', label: 'DD/MM/YYYY' },
    { value: 'MM/DD/YYYY', label: 'MM/DD/YYYY' },
    { value: 'DD.MM.YYYY', label: 'DD.MM.YYYY' },
    { value: 'MMM D, YYYY', label: 'Jan 5, 2025' },
  ];

  async function handleLocaleChange(code: string) {
    await setLocale(code);
    await updateSetting('locale', code);
    success(t('settings.saved'));
  }

  function exportDateStamp() {
    return new Date().toISOString().slice(0, 10);
  }

  function selectedPath(selection: string | string[] | null): string | null {
    if (Array.isArray(selection)) return selection[0] ?? null;
    return selection;
  }

  async function saveText(defaultPath: string, contents: string, extensions: string[]) {
    const path = await save({
      defaultPath,
      filters: [{ name: extensions.join('/').toUpperCase(), extensions }],
    });
    if (!path) return false;
    await writeTextFile(path, contents);
    return true;
  }

  async function reloadImportedData() {
    await Promise.all([
      loadClients(),
      loadInvoices(),
      loadProducts(),
      loadTaxRates(),
      loadSettings(),
      loadBusinessProfile(),
    ]);
  }

  async function handleImportClients() {
    try {
      const path = selectedPath(await open({
        multiple: false,
        filters: [{ name: 'CSV', extensions: ['csv'] }],
        fileAccessMode: 'scoped',
      }));
      if (!path) return;
      const csvContent = await readTextFile(path);
      const result = await invoke<ImportResult>('import_clients_csv', { csvContent });
      await loadClients();
      if (result.errors.length > 0) {
        toastError(`${result.imported} imported, ${result.errors.length} failed`);
      } else {
        success(t('common.success'));
      }
    } catch (e) {
      toastError(String(e));
    }
  }

  async function handleExportClients() {
    try {
      const csv = await invoke<string>('export_clients_csv');
      const saved = await saveText(`900invoice-clients-${exportDateStamp()}.csv`, csv, ['csv']);
      if (saved) success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }

  async function handleExportInvoices() {
    try {
      const csv = await invoke<string>('export_invoices_csv');
      const saved = await saveText(`900invoice-invoices-${exportDateStamp()}.csv`, csv, ['csv']);
      if (saved) success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }

  async function handleBackup() {
    try {
      const backup = await invoke<Record<string, unknown>>('backup_database');
      const saved = await saveText(
        `900invoice-backup-${exportDateStamp()}.json`,
        JSON.stringify(backup, null, 2),
        ['json']
      );
      if (saved) success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }

  async function handleRestore() {
    try {
      const path = selectedPath(await open({
        multiple: false,
        filters: [{ name: 'JSON', extensions: ['json'] }],
        fileAccessMode: 'scoped',
      }));
      if (!path) return;
      const backup = JSON.parse(await readTextFile(path)) as Record<string, unknown>;
      await invoke<RestoreResult>('restore_database', { backup });
      await reloadImportedData();
      success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }
</script>

<div class="view-container">
  <div class="settings-layout">
    <!-- Tab sidebar -->
    <nav class="settings-nav">
      {#each tabs as tab}
        <button
          class="settings-tab"
          class:active={activeTab === tab.id}
          onclick={() => activeTab = tab.id}
        >
          {t(tab.labelKey)}
        </button>
      {/each}
    </nav>

    <!-- Tab content -->
    <div class="settings-content card">
      {#if activeTab === 'profile'}
        <h2 class="settings-section-title">{t('settings.businessProfile')}</h2>
        <BusinessProfileForm />

      {:else if activeTab === 'taxes'}
        <h2 class="settings-section-title">{t('settings.taxRates')}</h2>
        <TaxRatesManager />

      {:else if activeTab === 'invoiceNumbers'}
        <h2 class="settings-section-title">{t('settings.invoiceNumbers')}</h2>
        <InvoiceSequenceSettings />

      {:else if activeTab === 'currency'}
        <h2 class="settings-section-title">{t('settings.currencySettings')}</h2>
        <CurrencySettings />

      {:else if activeTab === 'general'}
        <h2 class="settings-section-title">{t('settings.general')}</h2>

        <div class="form-group">
          <label class="form-label" for="lbl-settings-language">{t('settings.language')}</label>
          <select id="lbl-settings-language" class="select" style="max-width: 280px;" value={locale} onchange={(e) => handleLocaleChange((e.target as HTMLSelectElement).value)}>
            {#each SUPPORTED_LOCALES as loc}
              <option value={loc.code}>{loc.nativeName} ({loc.code.toUpperCase()})</option>
            {/each}
          </select>
        </div>

        <div class="form-group mt-md">
          <label class="form-label" for="lbl-settings-dateformat">{t('settings.dateFormat')}</label>
          <select id="lbl-settings-dateformat"
            class="select"
            style="max-width: 280px;"
            value={settings.dateFormat}
            onchange={(e) => updateSetting('dateFormat', (e.target as HTMLSelectElement).value)}
          >
            {#each dateFormats as f}
              <option value={f.value}>{f.label}</option>
            {/each}
          </select>
        </div>

        <div class="form-group mt-md">
          <label class="form-label" for="lbl-settings-papersize">{t('settings.paperSize')}</label>
          <select id="lbl-settings-papersize"
            class="select"
            style="max-width: 200px;"
            value={settings.paperSize}
            onchange={(e) => updateSetting('paperSize', (e.target as HTMLSelectElement).value as 'A4' | 'Letter')}
          >
            <option value="A4">A4</option>
            <option value="Letter">Letter</option>
          </select>
        </div>

      {:else if activeTab === 'importExport'}
        <h2 class="settings-section-title">{t('settings.importExport')}</h2>

        <div class="import-export-grid">
          <div class="import-export-item">
            <div>
              <p class="ie-title">{t('settings.importClients')}</p>
              <p class="ie-desc">Import clients from a CSV file.</p>
            </div>
            <button class="btn" onclick={handleImportClients}>
              ↑ {t('common.import')}
            </button>
          </div>
          <div class="import-export-item">
            <div>
              <p class="ie-title">{t('settings.exportClients')}</p>
              <p class="ie-desc">Export all clients to CSV.</p>
            </div>
            <button class="btn" onclick={handleExportClients}>
              ↓ {t('common.export')}
            </button>
          </div>
          <div class="import-export-item">
            <div>
              <p class="ie-title">{t('settings.exportInvoices')}</p>
              <p class="ie-desc">Export all invoices to CSV.</p>
            </div>
            <button class="btn" onclick={handleExportInvoices}>
              ↓ {t('common.export')}
            </button>
          </div>
          <hr class="divider" />
          <div class="import-export-item">
            <div>
              <p class="ie-title">{t('settings.backupDatabase')}</p>
              <p class="ie-desc">Save a copy of the database.</p>
            </div>
            <button class="btn btn-primary" onclick={handleBackup}>
              {t('common.download')}
            </button>
          </div>
          <div class="import-export-item">
            <div>
              <p class="ie-title">{t('settings.restoreDatabase')}</p>
              <p class="ie-desc">Restore from a backup file.</p>
            </div>
            <button class="btn" style="border-color: var(--color-danger); color: var(--color-danger);" onclick={handleRestore}>
              {t('common.upload')}
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .settings-layout {
    display: grid;
    grid-template-columns: 200px 1fr;
    gap: var(--space-md);
    align-items: start;
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    background: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-sm);
    box-shadow: var(--shadow-sm);
  }

  .settings-tab {
    display: block;
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    border: none;
    border-radius: var(--radius-md);
    background: none;
    text-align: start;
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }

  .settings-tab:hover {
    background: var(--color-bg-alt);
    color: var(--color-text);
  }

  .settings-tab.active {
    background: rgba(32, 128, 141, 0.1);
    color: var(--color-teal-primary);
  }

  .settings-content {
    min-height: 400px;
  }

  .settings-section-title {
    font-size: var(--font-size-lg);
    font-weight: 600;
    margin-block-end: var(--space-lg);
    padding-block-end: var(--space-md);
    border-bottom: 1px solid var(--color-border);
  }

  .import-export-grid {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .import-export-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    padding: var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .ie-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text);
  }

  .ie-desc {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-block-start: 2px;
  }

  .mt-md { margin-block-start: var(--space-md); }
</style>
