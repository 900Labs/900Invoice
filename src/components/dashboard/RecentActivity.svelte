<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getInvoices } from '../../stores/invoiceStore';
  import { formatCurrency } from '../../utils/currency';
  import { timeAgo } from '../../utils/date';
  import { navigateTo } from '../../stores/navigationStore';

  let invoices = $derived(getInvoices());

  interface Activity {
    invoiceId: string;
    invoiceNumber: string;
    clientName: string;
    status: string;
    updatedAt: string;
    totalMinor: number;
    currencyCode: string;
  }

  let recentActivity = $derived(
    [...invoices]
      .sort((a, b) => b.updatedAt.localeCompare(a.updatedAt))
      .slice(0, 10)
      .map(i => ({
        invoiceId: i.id,
        invoiceNumber: i.invoiceNumber,
        clientName: i.clientName,
        status: i.status,
        updatedAt: i.updatedAt,
        totalMinor: i.totalMinor,
        currencyCode: i.currencyCode,
      }) as Activity)
  );

  const statusIconMap: Record<string, string> = {
    Draft: '📝',
    Finalized: '✅',
    Sent: '📧',
    Paid: '💰',
    Void: '🚫',
  };
</script>

<div class="card">
  <div class="card-header">
    <h3 class="card-title">{t('dashboard.recentActivity')}</h3>
  </div>
  {#if recentActivity.length === 0}
    <p class="no-activity">{t('dashboard.noActivity')}</p>
  {:else}
    <ul class="activity-list">
      {#each recentActivity as activity}
        <li class="activity-item">
          <button
            class="activity-btn"
            onclick={() => navigateTo('invoice-detail', { id: activity.invoiceId })}
          >
            <span class="activity-icon">{statusIconMap[activity.status] ?? '📄'}</span>
            <div class="activity-info">
              <p class="activity-invoice">
                #{activity.invoiceNumber}
                <span class="activity-client">· {activity.clientName}</span>
              </p>
              <p class="activity-meta">
                <span class="badge badge-{activity.status.toLowerCase()}" style="font-size: 0.6rem; padding: 1px 5px;">{activity.status}</span>
                <span class="activity-time">{timeAgo(activity.updatedAt)}</span>
              </p>
            </div>
            <span class="activity-amount currency">
              {formatCurrency(activity.totalMinor, activity.currencyCode)}
            </span>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .no-activity {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    padding: var(--space-lg);
  }

  .activity-list {
    list-style: none;
    margin: -var(--space-md);
  }

  .activity-btn {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    width: 100%;
    padding: var(--space-sm) 0;
    border: none;
    background: none;
    cursor: pointer;
    text-align: start;
    border-bottom: 1px solid var(--color-border);
  }

  .activity-btn:last-child {
    border-bottom: none;
  }

  .activity-btn:hover .activity-invoice {
    color: var(--color-teal-primary);
  }

  .activity-icon {
    font-size: 1.1rem;
    width: 28px;
    text-align: center;
    flex-shrink: 0;
  }

  .activity-info {
    flex: 1;
    min-width: 0;
  }

  .activity-invoice {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .activity-client {
    color: var(--color-text-muted);
    font-weight: 400;
  }

  .activity-meta {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    margin-block-start: 2px;
  }

  .activity-time {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .activity-amount {
    font-size: var(--font-size-sm);
    font-weight: 500;
    flex-shrink: 0;
    color: var(--color-text);
  }
</style>
