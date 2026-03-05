<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import TopBar from './TopBar.svelte';
  import { getCurrentView } from '../../stores/navigationStore';

  // View components
  import DashboardView from '../dashboard/DashboardView.svelte';
  import InvoicesListView from '../invoices/InvoicesListView.svelte';
  import InvoiceDetailView from '../invoices/InvoiceDetailView.svelte';
  import InvoiceEditorView from '../invoices/InvoiceEditorView.svelte';
  import ClientsListView from '../clients/ClientsListView.svelte';
  import ClientDetailView from '../clients/ClientDetailView.svelte';
  import ProductsView from '../products/ProductsView.svelte';
  import RecurringView from '../recurring/RecurringView.svelte';
  import ReportsView from '../reports/ReportsView.svelte';
  import SettingsView from '../settings/SettingsView.svelte';

  let currentView = $derived(getCurrentView());
</script>

<div class="layout">
  <Sidebar />
  <div class="layout-main">
    <TopBar />
    <main class="layout-content">
      {#if currentView === 'dashboard'}
        <DashboardView />
      {:else if currentView === 'invoices'}
        <InvoicesListView />
      {:else if currentView === 'invoice-detail'}
        <InvoiceDetailView />
      {:else if currentView === 'invoice-editor'}
        <InvoiceEditorView />
      {:else if currentView === 'clients'}
        <ClientsListView />
      {:else if currentView === 'client-detail'}
        <ClientDetailView />
      {:else if currentView === 'products'}
        <ProductsView />
      {:else if currentView === 'recurring'}
        <RecurringView />
      {:else if currentView === 'reports'}
        <ReportsView />
      {:else if currentView === 'settings'}
        <SettingsView />
      {:else}
        <DashboardView />
      {/if}
    </main>
  </div>
</div>

<style>
  .layout {
    display: flex;
    height: 100vh;
    overflow: hidden;
    background: var(--color-bg);
  }

  .layout-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    overflow: hidden;
  }

  .layout-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }
</style>
