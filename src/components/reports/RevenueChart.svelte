<script lang="ts">
  import { getInvoices } from '../../stores/invoiceStore';
  import { formatCurrency } from '../../utils/currency';
  import { getSettings } from '../../stores/settingsStore';
  import { t } from '../../stores/i18nStore';

  interface Props {
    period: 'week' | 'month' | 'quarter' | 'year';
    dateStart: string;
    dateEnd: string;
  }

  let { period, dateStart, dateEnd }: Props = $props();

  let invoices = $derived(getInvoices());
  let settings = $derived(getSettings());

  // Group paid invoices by month/week for bar chart
  let periodInvoices = $derived(
    invoices.filter(i =>
      i.status === 'Paid' &&
      i.issueDate >= dateStart &&
      i.issueDate <= dateEnd
    )
  );

  // Group by currency
  let byCurrency = $derived(
    periodInvoices.reduce((acc, inv) => {
      if (!acc[inv.currencyCode]) acc[inv.currencyCode] = 0;
      acc[inv.currencyCode] += inv.totalMinor;
      return acc;
    }, {} as Record<string, number>)
  );

  interface ClientRevenue {
    clientName: string;
    currencyCode: string;
    invoiceCount: number;
    totalMinor: number;
  }

  let byClient = $derived((() => {
    const acc: Record<string, ClientRevenue> = {};
    for (const inv of periodInvoices) {
      const clientName = inv.clientName || t('invoices.client');
      const key = `${clientName}::${inv.currencyCode}`;
      if (!acc[key]) {
        acc[key] = {
          clientName,
          currencyCode: inv.currencyCode,
          invoiceCount: 0,
          totalMinor: 0,
        };
      }
      acc[key].invoiceCount += 1;
      acc[key].totalMinor += inv.totalMinor;
    }
    return Object.values(acc).sort((a, b) => {
      const currencyOrder = a.currencyCode.localeCompare(b.currencyCode);
      if (currencyOrder !== 0) return currencyOrder;
      return b.totalMinor - a.totalMinor;
    });
  })());

  // Simple bar chart: group by week or month bucket
  interface Bucket {
    label: string;
    amount: number;
  }

  let buckets = $derived((() => {
    const acc: Record<string, number> = {};
    for (const inv of periodInvoices) {
      let key = inv.issueDate.slice(0, 7); // YYYY-MM
      if (period === 'week') key = inv.issueDate.slice(0, 10);
      acc[key] = (acc[key] ?? 0) + inv.totalMinor;
    }
    return Object.entries(acc)
      .sort((a, b) => a[0].localeCompare(b[0]))
      .map(([label, amount]): Bucket => ({ label, amount }));
  })());

  let maxAmount = $derived(Math.max(...buckets.map(b => b.amount), 1));

  let totalRevenue = $derived(periodInvoices.reduce((s, i) => s + i.totalMinor, 0));
  let defaultCurrency = $derived(settings.defaultCurrency || 'USD');
</script>

<div class="revenue-chart">
  <!-- Summary stats -->
  <div class="revenue-stats">
    {#each Object.entries(byCurrency) as [code, amount]}
      <div class="revenue-stat">
        <span class="revenue-stat-label">{t('reports.totalRevenue')} ({code})</span>
        <span class="revenue-stat-value currency">{formatCurrency(amount, code)}</span>
      </div>
    {/each}
    {#if Object.keys(byCurrency).length === 0}
      <div class="revenue-stat">
        <span class="revenue-stat-label">{t('reports.totalRevenue')}</span>
        <span class="revenue-stat-value">{formatCurrency(0, defaultCurrency)}</span>
      </div>
    {/if}
  </div>

  <!-- CSS Bar Chart -->
  {#if buckets.length > 0}
    <div class="bar-chart">
      {#each buckets as bucket}
        <div class="bar-group">
          <div class="bar-wrap">
            <div
              class="bar"
              style:height="{(bucket.amount / maxAmount) * 100}%"
              title={formatCurrency(bucket.amount, defaultCurrency)}
            ></div>
          </div>
          <span class="bar-label">{bucket.label}</span>
        </div>
      {/each}
    </div>
  {:else}
    <div class="no-data">
      <p>{t('common.noResults')}</p>
    </div>
  {/if}

  {#if byClient.length > 0}
    <div class="client-breakdown">
      <h3>{t('reports.revenueByClient')}</h3>
      <table class="table">
        <thead>
          <tr>
            <th>{t('invoices.client')}</th>
            <th>{t('invoices.currency')}</th>
            <th style="text-align: end;">{t('reports.invoiceCount')}</th>
            <th style="text-align: end;">{t('reports.totalRevenue')}</th>
            <th style="text-align: end;">{t('reports.avgInvoice')}</th>
          </tr>
        </thead>
        <tbody>
          {#each byClient as row}
            <tr>
              <td>{row.clientName}</td>
              <td>{row.currencyCode}</td>
              <td style="text-align: end;">{row.invoiceCount}</td>
              <td style="text-align: end;" class="currency">
                {formatCurrency(row.totalMinor, row.currencyCode)}
              </td>
              <td style="text-align: end;" class="currency">
                {formatCurrency(Math.round(row.totalMinor / row.invoiceCount), row.currencyCode)}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .revenue-chart {
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .revenue-stats {
    display: flex;
    gap: var(--space-lg);
    flex-wrap: wrap;
  }

  .revenue-stat {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .revenue-stat-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
  }

  .revenue-stat-value {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    color: var(--color-teal-primary);
    font-variant-numeric: tabular-nums;
  }

  .bar-chart {
    display: flex;
    align-items: flex-end;
    gap: 4px;
    height: 180px;
    overflow-x: auto;
    padding-block-end: var(--space-xl);
    position: relative;
  }

  .bar-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
    min-width: 32px;
    max-width: 60px;
    height: 100%;
    gap: 4px;
  }

  .bar-wrap {
    flex: 1;
    width: 100%;
    display: flex;
    align-items: flex-end;
  }

  .bar {
    width: 100%;
    background: var(--color-teal-primary);
    border-radius: var(--radius-sm) var(--radius-sm) 0 0;
    min-height: 4px;
    transition: height 0.3s ease;
  }

  .bar:hover {
    background: var(--color-teal-deep);
  }

  .bar-label {
    font-size: 9px;
    color: var(--color-text-muted);
    transform: rotate(-45deg);
    white-space: nowrap;
    margin-block-start: var(--space-xs);
  }

  .no-data {
    display: flex;
    justify-content: center;
    padding: var(--space-2xl);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .client-breakdown {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .client-breakdown h3 {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
  }
</style>
