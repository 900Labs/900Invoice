<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getOverdueInvoices } from '../../stores/invoiceStore';
  import { navigateTo } from '../../stores/navigationStore';

  let overdueInvoices = $derived(getOverdueInvoices());
  let count = $derived(overdueInvoices.length);
</script>

{#if count > 0}
  <div class="alert alert-warning overdue-alert">
    <span class="overdue-icon" aria-hidden="true">⚠️</span>
    <div class="overdue-content">
      <p class="overdue-message">
        {t('dashboard.overdueAlert', { count })}
      </p>
    </div>
    <button
      class="btn btn-sm"
      style="border-color: #D97706; color: #92400e;"
      onclick={() => navigateTo('invoices')}
    >
      {t('dashboard.viewAll')}
    </button>
  </div>
{/if}

<style>
  .overdue-alert {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    background: rgba(217, 119, 6, 0.08);
    border: 1px solid rgba(217, 119, 6, 0.3);
  }

  .overdue-icon {
    font-size: 1.2rem;
    flex-shrink: 0;
  }

  .overdue-content {
    flex: 1;
  }

  .overdue-message {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: #92400e;
  }
</style>
