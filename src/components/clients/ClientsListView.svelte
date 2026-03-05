<script lang="ts">
  import SearchBar from '../shared/SearchBar.svelte';
  import EmptyState from '../shared/EmptyState.svelte';
  import Modal from '../shared/Modal.svelte';
  import ClientForm from './ClientForm.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import Pagination from '../shared/Pagination.svelte';
  import {
    getFilteredClients, getLoading, searchClients, createClient
  } from '../../stores/clientStore';
  import { formatCurrency } from '../../utils/currency';
  import { t } from '../../stores/i18nStore';
  import { navigateTo } from '../../stores/navigationStore';
  import { success } from '../../stores/toastStore';
  import type { CreateClient } from '../../stores/clientStore';

  let clients = $derived(getFilteredClients());
  let loading = $derived(getLoading());
  let showAddModal = $state(false);
  let page = $state(1);
  const PAGE_SIZE = 25;
  let paginated = $derived(clients.slice((page - 1) * PAGE_SIZE, page * PAGE_SIZE));

  async function handleCreate(data: CreateClient) {
    await createClient(data);
    showAddModal = false;
    success(t('common.success'));
  }
</script>

<div class="view-container">
  <div class="view-toolbar">
    <SearchBar
      placeholder={t('clients.search')}
      onsearch={searchClients}
    />
    <div class="spacer"></div>
    <button class="btn btn-primary" onclick={() => showAddModal = true}>
      + {t('clients.new')}
    </button>
  </div>

  <div class="card" style="padding: 0; overflow: hidden;">
    {#if loading}
      <div style="display: flex; justify-content: center; padding: 48px;">
        <LoadingSpinner size="lg" />
      </div>
    {:else if clients.length === 0}
      <EmptyState
        icon="👥"
        title={t('clients.noClients')}
        description={t('clients.addFirst')}
      >
        {#snippet action()}
          <button class="btn btn-primary" onclick={() => showAddModal = true}>
            + {t('clients.new')}
          </button>
        {/snippet}
      </EmptyState>
    {:else}
      <table class="table">
        <thead>
          <tr>
            <th>{t('clients.name')}</th>
            <th>{t('clients.email')}</th>
            <th>{t('clients.phone')}</th>
            <th>{t('clients.country')}</th>
            <th>{t('clients.currency')}</th>
            <th style="text-align: end;">{t('clients.invoiceCount')}</th>
            <th style="text-align: end;">{t('clients.outstanding')}</th>
          </tr>
        </thead>
        <tbody>
          {#each paginated as client}
            <tr
              style="cursor: pointer;"
              onclick={() => navigateTo('client-detail', { id: client.id })}
            >
              <td>
                <span class="client-name">{client.name}</span>
              </td>
              <td class="text-muted">{client.email || '—'}</td>
              <td class="text-muted">{client.phone || '—'}</td>
              <td>{client.country || '—'}</td>
              <td>
                <span class="badge badge-draft" style="font-size: 0.65rem;">{client.currencyCode}</span>
              </td>
              <td style="text-align: end;">{client.invoiceCount}</td>
              <td style="text-align: end;" class="currency">
                {client.outstandingMinor > 0
                  ? formatCurrency(client.outstandingMinor, client.currencyCode)
                  : '—'}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
      <Pagination
        total={clients.length}
        {page}
        pageSize={PAGE_SIZE}
        onchange={(p) => page = p}
      />
    {/if}
  </div>
</div>

{#if showAddModal}
  <Modal title={t('clients.new')} onclose={() => showAddModal = false} maxWidth="640px">
    <ClientForm
      oncreate={handleCreate}
      oncancel={() => showAddModal = false}
    />
  </Modal>
{/if}

<style>
  .spacer { flex: 1; }
  .client-name { font-weight: 500; }
</style>
