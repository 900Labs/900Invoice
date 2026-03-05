<script lang="ts">
  import RevenueChart from './RevenueChart.svelte';
  import AgingReport from './AgingReport.svelte';
  import { getDateRange } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getInvoices } from '../../stores/invoiceStore';
  import { formatCurrency, formatTaxRate } from '../../utils/currency';
  import { getSettings } from '../../stores/settingsStore';

  let activeTab = $state<'revenue' | 'tax' | 'aging'>('revenue');
  let period = $state<'week' | 'month' | 'quarter' | 'year'>('month');
  let settings = $derived(getSettings());
  let invoices = $derived(getInvoices());

  let dateRange = $derived(getDateRange(period));

  // Tax summary: aggregate tax by type
  interface TaxSummaryRow {
    name: string;
    displayName: string;
    rateBps: number;
    totalBaseMinor: number;
    totalTaxMinor: number;
    currencyCode: string;
  }

  let taxSummary = $derived((() => {
    const acc: Record<string, TaxSummaryRow> = {};
    const periodInvoices = invoices.filter(i =>
      i.status === 'Paid' &&
      i.issueDate >= dateRange.start &&
      i.issueDate <= dateRange.end
    );
    for (const inv of periodInvoices) {
      for (const tl of inv.taxLines) {
        const key = `${tl.taxRateId}_${inv.currencyCode}`;
        if (!acc[key]) {
          acc[key] = {
            name: tl.taxName,
            displayName: tl.taxDisplayName,
            rateBps: tl.rateBps,
            totalBaseMinor: 0,
            totalTaxMinor: 0,
            currencyCode: inv.currencyCode,
          };
        }
        acc[key].totalBaseMinor += tl.baseAmountMinor;
        acc[key].totalTaxMinor += tl.taxAmountMinor;
      }
    }
    return Object.values(acc);
  })());

  const tabs = [
    { id: 'revenue', label: t('reports.revenue') },
    { id: 'tax', label: t('reports.taxSummary') },
    { id: 'aging', label: t('reports.aging') },
  ] as const;

  const periods = [
    { value: 'week', label: t('reports.thisWeek') },
    { value: 'month', label: t('reports.thisMonth') },
    { value: 'quarter', label: t('reports.thisQuarter') },
    { value: 'year', label: t('reports.thisYear') },
  ] as const;

  function exportCsv() {
    // Build CSV from relevant data
    const rows: string[][] = [
      [t('invoices.number'), t('invoices.client'), t('invoices.date'), t('invoices.amount'), t('invoices.status')]
    ];
    const filtered = invoices.filter(i =>
      i.issueDate >= dateRange.start && i.issueDate <= dateRange.end
    );
    for (const inv of filtered) {
      rows.push([
        inv.invoiceNumber,
        inv.clientName,
        inv.issueDate,
        (inv.totalMinor / 100).toFixed(2),
        inv.status,
      ]);
    }
    const csv = rows.map(r => r.map(c => `"${c}"`).join(',')).join('\n');
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `invoices-${period}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="view-container">
  <div class="reports-toolbar">
    <div class="tabs" style="border-bottom: none; margin: 0;">
      {#each tabs as tab}
        <button
          class="tab"
          class:active={activeTab === tab.id}
          onclick={() => activeTab = tab.id}
        >
          {tab.label}
        </button>
      {/each}
    </div>

    {#if activeTab === 'revenue' || activeTab === 'tax'}
      <div class="period-selector">
        {#each periods as p}
          <button
            class="btn btn-sm {period === p.value ? 'btn-primary' : ''}"
            onclick={() => period = p.value}
          >
            {p.label}
          </button>
        {/each}
      </div>
    {/if}

    <button class="btn btn-sm" onclick={exportCsv}>
      ↓ {t('reports.exportCsv')}
    </button>
  </div>

  <div class="card">
    {#if activeTab === 'revenue'}
      <RevenueChart {period} dateStart={dateRange.start} dateEnd={dateRange.end} />
    {:else if activeTab === 'tax'}
      <div class="tax-summary-table">
        {#if taxSummary.length === 0}
          <p style="text-align: center; color: var(--color-text-muted); padding: var(--space-2xl);">
            {t('common.noResults')}
          </p>
        {:else}
          <table class="table">
            <thead>
              <tr>
                <th>{t('taxes.name')}</th>
                <th>{t('taxes.rate')}</th>
                <th style="text-align: end;">{t('invoices.subtotal')}</th>
                <th style="text-align: end;">{t('reports.taxCollected')}</th>
              </tr>
            </thead>
            <tbody>
              {#each taxSummary as row}
                <tr>
                  <td>{row.displayName}</td>
                  <td>{formatTaxRate(row.rateBps)}</td>
                  <td style="text-align: end;" class="currency">
                    {formatCurrency(row.totalBaseMinor, row.currencyCode)}
                  </td>
                  <td style="text-align: end;" class="currency">
                    {formatCurrency(row.totalTaxMinor, row.currencyCode)}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}
      </div>
    {:else if activeTab === 'aging'}
      <AgingReport />
    {/if}
  </div>
</div>

<style>
  .reports-toolbar {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    flex-wrap: wrap;
    margin-block-end: var(--space-sm);
  }

  .period-selector {
    display: flex;
    gap: var(--space-xs);
  }
</style>
