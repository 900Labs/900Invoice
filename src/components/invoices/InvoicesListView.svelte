<script lang="ts">
  import SearchBar from '../shared/SearchBar.svelte';
  import Pagination from '../shared/Pagination.svelte';
  import EmptyState from '../shared/EmptyState.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import InvoiceStatusBadge from './InvoiceStatusBadge.svelte';
  import { getFilteredInvoices, getLoading, setFilters, getFilters } from '../../stores/invoiceStore';
  import { formatCurrency } from '../../utils/currency';
  import { formatDate } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getSettings } from '../../stores/settingsStore';
  import { navigateTo } from '../../stores/navigationStore';

  let invoices = $derived(getFilteredInvoices());
  let loading = $derived(getLoading());
  let filters = $derived(getFilters());
  let settings = $derived(getSettings());

  let page = $state(1);
  const PAGE_SIZE = 25;

  let paginated = $derived(invoices.slice((page - 1) * PAGE_SIZE, page * PAGE_SIZE));

  const statusOptions = [
    { value: '', label: t('invoices.filterAll') },
    { value: 'Draft', label: t('invoices.filterDraft') },
    { value: 'Finalized', label: t('invoices.filterFinalized') },
    { value: 'Sent', label: t('invoices.filterSent') },
    { value: 'Paid', label: t('invoices.filterPaid') },
    { value: 'Void', label: t('invoices.filterVoid') },
  ];

  function handleSearch(q: string) {
    setFilters({ search: q });
    page = 1;
  }

  function handleStatusFilter(e: Event) {
    setFilters({ status: (e.target as HTMLSelectElement).value });
    page = 1;
  }
</script>

<div class="view-container">
  <!-- Toolbar -->
  <div class="view-toolbar">
    <SearchBar
      placeholder={t('invoices.search')}
      value={filters.search}
      onsearch={handleSearch}
    />
    <select
      class="select"
      style="width: auto; min-width: 140px;"
      value={filters.status}
      onchange={handleStatusFilter}
    >
      {#each statusOptions as opt}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
    <div class="spacer"></div>
    <button
      class="btn btn-primary"
      onclick={() => navigateTo('invoice-editor')}
    >
      + {t('invoices.new')}
    </button>
  </div>

  <!-- Table -->
  <div class="card" style="padding: 0; overflow: hidden;">
    {#if loading}
      <div style="display: flex; justify-content: center; padding: 48px;">
        <LoadingSpinner size="lg" />
      </div>
    {:else if invoices.length === 0}
      <EmptyState
        icon="📄"
        title={t('invoices.noInvoices')}
        description={t('invoices.createFirst')}
      >
        {#snippet action()}
          <button class="btn btn-primary" onclick={() => navigateTo('invoice-editor')}>
            + {t('invoices.new')}
          </button>
        {/snippet}
      </EmptyState>
    {:else}
      <table class="table">
        <thead>
          <tr>
            <th>{t('invoices.number')}</th>
            <th>{t('invoices.client')}</th>
            <th>{t('invoices.date')}</th>
            <th>{t('invoices.dueDate')}</th>
            <th style="text-align: end;">{t('invoices.amount')}</th>
            <th>{t('invoices.status')}</th>
            <th>{t('invoices.actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each paginated as invoice}
            <tr
              onclick={() => navigateTo('invoice-detail', { id: invoice.id })}
              style="cursor: pointer;"
            >
              <td>
                <span class="invoice-number">#{invoice.invoiceNumber}</span>
              </td>
              <td>
                <span class="truncate" style="max-width: 180px; display: block;">{invoice.clientName}</span>
              </td>
              <td>{formatDate(invoice.issueDate, settings.dateFormat)}</td>
              <td>{formatDate(invoice.dueDate, settings.dateFormat)}</td>
              <td style="text-align: end;" class="currency">
                {formatCurrency(invoice.totalMinor, invoice.currencyCode)}
              </td>
              <td>
                <InvoiceStatusBadge status={invoice.status} />
              </td>
              <td>
                <button
                  class="btn btn-ghost btn-sm"
                  onclick={(e) => {
                    e.stopPropagation();
                    navigateTo('invoice-editor', { id: invoice.id });
                  }}
                  disabled={invoice.status !== 'Draft'}
                >
                  {t('common.edit')}
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
      <Pagination
        total={invoices.length}
        {page}
        pageSize={PAGE_SIZE}
        onchange={(p) => page = p}
      />
    {/if}
  </div>
</div>

<style>
  .spacer { flex: 1; }
  .invoice-number { font-weight: 500; font-variant-numeric: tabular-nums; }
</style>
