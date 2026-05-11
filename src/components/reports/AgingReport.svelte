<script lang="ts">
  import { getInvoices, type Invoice } from '../../stores/invoiceStore';
  import { formatCurrency } from '../../utils/currency';
  import { currencyTotalDetails, currencyTotals } from '../../utils/currencyTotals';
  import { isOverdue, daysUntilDue } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getSettings } from '../../stores/settingsStore';

  interface AgingBucket {
    invoices: Invoice[];
  }

  let invoices = $derived(getInvoices());
  let settings = $derived(getSettings());
  let defaultCurrency = $derived(settings.defaultCurrency || 'USD');

  let outstanding = $derived(
    invoices.filter(i => i.status !== 'Paid' && i.status !== 'Void' && i.status !== 'Draft')
  );

  function getAgingBucket(dueDate: string): string {
    const days = daysUntilDue(dueDate);
    if (days >= 0) return 'current';
    if (days >= -30) return 'overdue30';
    if (days >= -60) return 'overdue60';
    if (days >= -90) return 'overdue90';
    return 'overdue90plus';
  }

  let agingBuckets = $derived((() => {
    const buckets: Record<string, AgingBucket> = {
      current: { invoices: [] },
      overdue30: { invoices: [] },
      overdue60: { invoices: [] },
      overdue90: { invoices: [] },
      overdue90plus: { invoices: [] },
    };

    for (const inv of outstanding) {
      const bucket = getAgingBucket(inv.dueDate);
      buckets[bucket].invoices.push(inv);
    }

    return buckets;
  })());

  const bucketLabels: Record<string, string> = {
    current: t('reports.current'),
    overdue30: t('reports.overdue30'),
    overdue60: t('reports.overdue60'),
    overdue90: t('reports.overdue90'),
    overdue90plus: t('reports.overdue90plus'),
  };

  const bucketColors: Record<string, string> = {
    current: 'var(--color-success)',
    overdue30: 'var(--color-warning)',
    overdue60: '#f97316',
    overdue90: '#ea580c',
    overdue90plus: 'var(--color-danger)',
  };
</script>

<div class="aging-report">
  <div class="aging-grid">
    {#each Object.entries(agingBuckets) as [key, bucket]}
      <div class="aging-card" style:border-color={bucketColors[key]}>
        <div class="aging-header">
          <span class="aging-label" style:color={bucketColors[key]}>
            {bucketLabels[key]}
          </span>
          <span class="aging-count">{bucket.invoices.length}</span>
        </div>
        <div class="aging-total-list">
          {#each currencyTotalDetails(
            currencyTotals(bucket.invoices, inv => inv.currencyCode, inv => inv.balanceDueMinor),
            defaultCurrency
          ) as total}
            <p class="aging-total currency">
              <span>{total.value}</span>
              <span class="aging-currency">{total.label}</span>
            </p>
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <!-- Detailed table for overdue invoices -->
  {#if outstanding.filter(i => isOverdue(i.dueDate)).length > 0}
    <div style="margin-block-start: var(--space-lg);">
      <h4 style="font-size: var(--font-size-sm); font-weight: 600; color: var(--color-text-secondary); margin-block-end: var(--space-sm);">
        {t('dashboard.overdue')}
      </h4>
      <table class="table">
        <thead>
          <tr>
            <th>{t('invoices.number')}</th>
            <th>{t('invoices.client')}</th>
            <th>{t('invoices.dueDate')}</th>
            <th style="text-align: end;">{t('invoices.balanceDue')}</th>
          </tr>
        </thead>
        <tbody>
          {#each outstanding.filter(i => isOverdue(i.dueDate)) as inv}
            <tr>
              <td>#{inv.invoiceNumber}</td>
              <td>{inv.clientName}</td>
              <td style="color: var(--color-danger);">
                {inv.dueDate} ({Math.abs(daysUntilDue(inv.dueDate))}d)
              </td>
              <td style="text-align: end;" class="currency">
                {formatCurrency(inv.balanceDueMinor, inv.currencyCode)}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .aging-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: var(--space-md);
  }

  .aging-card {
    padding: var(--space-md);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-md);
    border-inline-start-width: 4px;
    background: var(--color-bg-card);
  }

  .aging-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-block-end: var(--space-sm);
  }

  .aging-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .aging-count {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    background: var(--color-bg-alt);
    padding: 1px 6px;
    border-radius: 9999px;
  }

  .aging-total {
    font-size: var(--font-size-xl);
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--color-text);
    display: flex;
    justify-content: space-between;
    gap: var(--space-sm);
    overflow-wrap: anywhere;
  }

  .aging-total-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .aging-currency {
    flex: 0 0 auto;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
