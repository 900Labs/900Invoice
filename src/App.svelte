<script lang="ts">
  import { onMount } from 'svelte';
  import Layout from './components/layout/Layout.svelte';
  import { initI18n } from './stores/i18nStore';
  import { loadSettings, loadBusinessProfile } from './stores/settingsStore';
  import { loadInvoices } from './stores/invoiceStore';
  import { loadClients } from './stores/clientStore';
  import { loadProducts } from './stores/productStore';
  import { loadTaxRates } from './stores/taxStore';
  import Toast from './components/shared/Toast.svelte';

  let initialized = $state(false);
  let startupError = $state<string | null>(null);

  onMount(async () => {
    try {
      await initI18n();
      await Promise.all([
        loadSettings(),
        loadBusinessProfile(),
        loadInvoices(),
        loadClients(),
        loadProducts(),
        loadTaxRates(),
      ]);
      initialized = true;
    } catch (error) {
      console.error('Failed to initialize 900Invoice:', error);
      startupError = error instanceof Error ? error.message : String(error);
    }
  });
</script>

{#if initialized}
  <Layout />
{:else if startupError}
  <div class="app-loading app-error" role="alert">
    <p>900Invoice could not start</p>
    <small>{startupError}</small>
  </div>
{:else}
  <div class="app-loading">
    <div class="spinner" style="width: 32px; height: 32px; border-width: 3px;"></div>
    <p>900Invoice</p>
  </div>
{/if}

<Toast />

<style>
  .app-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 16px;
    color: var(--color-teal-primary);
    font-weight: 600;
    font-size: 1.25rem;
  }

  .app-error {
    color: var(--color-danger);
    padding: var(--space-lg);
    text-align: center;
  }

  .app-error small {
    max-width: 640px;
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 400;
    line-height: 1.5;
  }
</style>
