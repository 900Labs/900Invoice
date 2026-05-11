<script lang="ts">
  import StatCard from './StatCard.svelte';
  import RecentActivity from './RecentActivity.svelte';
  import OverdueAlert from './OverdueAlert.svelte';
  import { getInvoices, getOverdueInvoices } from '../../stores/invoiceStore';
  import { getClientCount } from '../../stores/clientStore';
  import {
    currencyTotalDetails,
    currencyTotals,
    hasPositiveCurrencyTotal,
  } from '../../utils/currencyTotals';
  import { getSettings } from '../../stores/settingsStore';
  import { getDateRange } from '../../utils/date';
  import { t } from '../../stores/i18nStore';

  let invoices = $derived(getInvoices());
  let settings = $derived(getSettings());
  let overdueInvoices = $derived(getOverdueInvoices());
  let clientCount = $derived(getClientCount());

  let thisMonthRange = $derived(getDateRange('month'));

  let thisMonthRevenueTotals = $derived(
    currencyTotals(
      invoices.filter(i =>
        i.status === 'Paid' &&
        i.issueDate >= thisMonthRange.start &&
        i.issueDate <= thisMonthRange.end
      ),
      invoice => invoice.currencyCode,
      invoice => invoice.totalMinor
    )
  );

  let outstandingTotals = $derived(
    currencyTotals(
      invoices.filter(i => i.status !== 'Paid' && i.status !== 'Void' && i.status !== 'Draft'),
      invoice => invoice.currencyCode,
      invoice => invoice.balanceDueMinor
    )
  );

  let defaultCurrency = $derived(settings.defaultCurrency);
  let hasOutstanding = $derived(hasPositiveCurrencyTotal(outstandingTotals));
</script>

<div class="view-container">
  <OverdueAlert />

  <div class="stats-grid">
    <StatCard
      label={t('dashboard.totalRevenue')}
      details={currencyTotalDetails(thisMonthRevenueTotals, defaultCurrency)}
      icon="💰"
      sublabel={t('dashboard.thisMonth')}
    />
    <StatCard
      label={t('dashboard.outstanding')}
      details={currencyTotalDetails(outstandingTotals, defaultCurrency)}
      icon="📋"
      color={hasOutstanding ? 'var(--color-warning)' : 'var(--color-teal-primary)'}
    />
    <StatCard
      label={t('dashboard.overdue')}
      value={String(overdueInvoices.length)}
      icon="⏰"
      color={overdueInvoices.length > 0 ? 'var(--color-danger)' : 'var(--color-success)'}
    />
    <StatCard
      label={t('dashboard.clientCount')}
      value={String(clientCount)}
      icon="👥"
      color="var(--color-teal-primary)"
    />
  </div>

  <RecentActivity />
</div>
