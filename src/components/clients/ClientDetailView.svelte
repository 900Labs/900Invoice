<script lang="ts">
  import { onMount } from 'svelte';
  import ClientForm from './ClientForm.svelte';
  import ConfirmDialog from '../shared/ConfirmDialog.svelte';
  import InvoiceStatusBadge from '../invoices/InvoiceStatusBadge.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import {
    getCurrentClient, loadClient, updateClient, deleteClient, getLoading
  } from '../../stores/clientStore';
  import { getInvoices } from '../../stores/invoiceStore';
  import { formatCurrency } from '../../utils/currency';
  import { formatDate } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getSettings } from '../../stores/settingsStore';
  import { getViewParams, navigateTo } from '../../stores/navigationStore';
  import { success } from '../../stores/toastStore';
  import type { CreateClient } from '../../stores/clientStore';

  let params = $derived(getViewParams());
  let client = $derived(getCurrentClient());
  let loading = $derived(getLoading());
  let settings = $derived(getSettings());
  let allInvoices = $derived(getInvoices());

  let isEditing = $state(false);
  let showConfirmDelete = $state(false);

  let clientInvoices = $derived(
    client ? allInvoices.filter(i => i.clientId === client.id) : []
  );

  onMount(async () => {
    const id = params.id;
    if (id) await loadClient(id);
  });

  async function handleUpdate(data: Partial<CreateClient>) {
    if (!client) return;
    await updateClient(client.id, data);
    isEditing = false;
    success(t('common.success'));
  }

  async function handleDelete() {
    if (!client) return;
    await deleteClient(client.id);
    navigateTo('clients');
    showConfirmDelete = false;
  }
</script>

{#if loading && !client}
  <div style="display: flex; justify-content: center; padding: 64px;">
    <LoadingSpinner size="lg" />
  </div>
{:else if client}
  <div class="view-container">
    <div class="detail-header">
      <div class="detail-header-left">
        <button class="btn btn-ghost btn-sm" onclick={() => navigateTo('clients')}>← {t('common.back')}</button>
        <h2 class="detail-title">{client.name}</h2>
      </div>
      <div class="detail-actions">
        <button class="btn" onclick={() => isEditing = !isEditing}>
          {isEditing ? t('common.cancel') : t('common.edit')}
        </button>
        <button
          class="btn btn-primary"
          onclick={() => navigateTo('invoice-editor', { clientId: client.id })}
        >
          + {t('invoices.new')}
        </button>
        <button
          class="btn btn-ghost"
          style="color: var(--color-danger);"
          onclick={() => showConfirmDelete = true}
        >
          {t('common.delete')}
        </button>
      </div>
    </div>

    {#if isEditing}
      <div class="card">
        <ClientForm
          {client}
          onupdate={handleUpdate}
          oncancel={() => isEditing = false}
        />
      </div>
    {:else}
      <div class="detail-grid">
        <div class="card">
          <h3 class="card-title" style="margin-block-end: var(--space-md);">{t('common.name')}</h3>
          <dl class="info-list">
            <dt>{t('clients.email')}</dt><dd>{client.email || '—'}</dd>
            <dt>{t('clients.phone')}</dt><dd>{client.phone || '—'}</dd>
            <dt>{t('clients.address')}</dt><dd>{[client.address, client.city, client.country].filter(Boolean).join(', ') || '—'}</dd>
            <dt>{t('clients.taxId')}</dt><dd>{client.taxId || '—'}</dd>
            <dt>{t('clients.currency')}</dt><dd>{client.currencyCode}</dd>
            <dt>{t('clients.paymentTerms')}</dt><dd>{client.paymentTermsDays} {t('invoices.days')}</dd>
          </dl>
          {#if client.notes}
            <div style="margin-block-start: var(--space-md); padding-block-start: var(--space-md); border-top: 1px solid var(--color-border);">
              <p class="form-label">{t('clients.notes')}</p>
              <p style="font-size: var(--font-size-sm); color: var(--color-text-secondary); margin-block-start: var(--space-xs);">{client.notes}</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Invoice History -->
    <div class="card" style="padding: 0; overflow: hidden;">
      <div style="padding: var(--space-md);">
        <h3 class="card-title">{t('clients.invoiceHistory')}</h3>
      </div>
      {#if clientInvoices.length === 0}
        <p style="text-align: center; color: var(--color-text-muted); padding: var(--space-lg); font-size: var(--font-size-sm);">
          {t('invoices.noInvoices')}
        </p>
      {:else}
        <table class="table">
          <thead>
            <tr>
              <th>{t('invoices.number')}</th>
              <th>{t('invoices.date')}</th>
              <th>{t('invoices.dueDate')}</th>
              <th style="text-align: end;">{t('invoices.amount')}</th>
              <th>{t('invoices.status')}</th>
            </tr>
          </thead>
          <tbody>
            {#each clientInvoices as inv}
              <tr
                style="cursor: pointer;"
                onclick={() => navigateTo('invoice-detail', { id: inv.id })}
              >
                <td>#{inv.invoiceNumber}</td>
                <td>{formatDate(inv.issueDate, settings.dateFormat)}</td>
                <td>{formatDate(inv.dueDate, settings.dateFormat)}</td>
                <td style="text-align: end;" class="currency">
                  {formatCurrency(inv.totalMinor, inv.currencyCode)}
                </td>
                <td><InvoiceStatusBadge status={inv.status} /></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if showConfirmDelete}
  <ConfirmDialog
    message={t('clients.confirmDelete')}
    danger={true}
    onconfirm={handleDelete}
    oncancel={() => showConfirmDelete = false}
  />
{/if}

<style>
  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .detail-header-left {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .detail-title {
    font-size: var(--font-size-xl);
    font-weight: 700;
  }

  .detail-actions {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-md);
  }

  .info-list {
    display: grid;
    grid-template-columns: 140px 1fr;
    gap: var(--space-xs) var(--space-md);
    font-size: var(--font-size-sm);
  }

  .info-list dt {
    color: var(--color-text-muted);
    font-weight: 500;
  }

  .info-list dd {
    color: var(--color-text);
  }
</style>
