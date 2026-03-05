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

  onMount(async () => {
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
  });
</script>

{#if initialized}
  <Layout />
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
</style>
