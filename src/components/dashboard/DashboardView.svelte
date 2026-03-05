<script lang="ts">
  import StatCard from './StatCard.svelte';
  import RecentActivity from './RecentActivity.svelte';
  import OverdueAlert from './OverdueAlert.svelte';
  import { getInvoices, getTotalOutstanding, getOverdueInvoices } from '../../stores/invoiceStore';
  import { getClientCount } from '../../stores/clientStore';
  import { formatCurrency } from '../../utils/currency';
  import { getSettings } from '../../stores/settingsStore';
  import { getDateRange } from '../../utils/date';
  import { t } from '../../stores/i18nStore';

  let invoices = $derived(getInvoices());
  let settings = $derived(getSettings());
  let totalOutstanding = $derived(getTotalOutstanding());
  let overdueInvoices = $derived(getOverdueInvoices());
  let clientCount = $derived(getClientCount());

  let thisMonthRange = $derived(getDateRange('month'));

  let thisMonthRevenue = $derived(
    invoices
      .filter(i =>
        i.status === 'Paid' &&
        i.issueDate >= thisMonthRange.start &&
        i.issueDate <= thisMonthRange.end
      )
      .reduce((sum, i) => sum + i.totalMinor, 0)
  );

  let defaultCurrency = $derived(settings.defaultCurrency);
</script>

<div class="view-container">
  <OverdueAlert />

  <div class="stats-grid">
    <StatCard
      label={t('dashboard.totalRevenue')}
      value={formatCurrency(thisMonthRevenue, defaultCurrency)}
      icon="💰"
      sublabel={t('dashboard.thisMonth')}
    />
    <StatCard
      label={t('dashboard.outstanding')}
      value={formatCurrency(totalOutstanding, defaultCurrency)}
      icon="📋"
      color={totalOutstanding > 0 ? 'var(--color-warning)' : 'var(--color-teal-primary)'}
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
