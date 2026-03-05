<script lang="ts">
  import { onMount } from 'svelte';
  import Modal from '../shared/Modal.svelte';
  import RecurringForm from './RecurringForm.svelte';
  import EmptyState from '../shared/EmptyState.svelte';
  import ConfirmDialog from '../shared/ConfirmDialog.svelte';
  import StatusBadge from '../shared/StatusBadge.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getClients } from '../../stores/clientStore';
  import { formatDate } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getSettings } from '../../stores/settingsStore';
  import { success, error as toastError } from '../../stores/toastStore';
  import type { RecurringFormData } from './RecurringForm.svelte';

  interface RecurringSchedule {
    id: string;
    clientId: string;
    clientName: string;
    templateInvoiceId: string;
    frequency: string;
    nextDate: string;
    endDate: string;
    autoSend: boolean;
    status: 'active' | 'paused';
  }

  let settings = $derived(getSettings());
  let schedules = $state<RecurringSchedule[]>([]);
  let loading = $state(false);
  let showAddModal = $state(false);
  let deletingId = $state<string | null>(null);

  onMount(async () => {
    await loadSchedules();
  });

  async function loadSchedules() {
    loading = true;
    try {
      schedules = await invoke<RecurringSchedule[]>('list_recurring');
    } catch {
      schedules = [];
    } finally {
      loading = false;
    }
  }

  async function handleCreate(data: RecurringFormData) {
    try {
      await invoke('create_recurring', { data });
      await loadSchedules();
      showAddModal = false;
      success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }

  async function handleGenerateNow() {
    try {
      await invoke('generate_due_recurring');
      success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }

  async function handleDelete() {
    if (!deletingId) return;
    try {
      await invoke('delete_recurring', { id: deletingId });
      schedules = schedules.filter(s => s.id !== deletingId);
      deletingId = null;
      success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }

  function getFrequencyLabel(freq: string): string {
    const map: Record<string, string> = {
      weekly: t('recurring.weekly'),
      monthly: t('recurring.monthly'),
      quarterly: t('recurring.quarterly'),
      annual: t('recurring.annual'),
    };
    return map[freq] ?? freq;
  }
</script>

<div class="view-container">
  <div class="view-toolbar">
    <button class="btn" onclick={handleGenerateNow}>
      ⚡ {t('recurring.generateNow')}
    </button>
    <div style="flex: 1;"></div>
    <button class="btn btn-primary" onclick={() => showAddModal = true}>
      + {t('recurring.new')}
    </button>
  </div>

  <div class="card" style="padding: 0; overflow: hidden;">
    {#if loading}
      <div style="display: flex; justify-content: center; padding: 48px;">
        <LoadingSpinner size="lg" />
      </div>
    {:else if schedules.length === 0}
      <EmptyState
        icon="🔄"
        title={t('recurring.noRecurring')}
      >
        {#snippet action()}
          <button class="btn btn-primary" onclick={() => showAddModal = true}>
            + {t('recurring.new')}
          </button>
        {/snippet}
      </EmptyState>
    {:else}
      <table class="table">
        <thead>
          <tr>
            <th>{t('recurring.client')}</th>
            <th>{t('recurring.frequency')}</th>
            <th>{t('recurring.nextDate')}</th>
            <th>{t('recurring.endDate')}</th>
            <th>{t('recurring.autoSend')}</th>
            <th>{t('recurring.status')}</th>
            <th>{t('common.actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each schedules as s}
            <tr>
              <td class="font-medium">{s.clientName}</td>
              <td>{getFrequencyLabel(s.frequency)}</td>
              <td>{formatDate(s.nextDate, settings.dateFormat)}</td>
              <td>{s.endDate ? formatDate(s.endDate, settings.dateFormat) : '—'}</td>
              <td>{s.autoSend ? '✓' : '—'}</td>
              <td><StatusBadge status={s.status} /></td>
              <td>
                <button
                  class="btn btn-ghost btn-sm"
                  style="color: var(--color-danger);"
                  onclick={() => deletingId = s.id}
                >
                  {t('common.delete')}
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

{#if showAddModal}
  <Modal title={t('recurring.new')} onclose={() => showAddModal = false} maxWidth="500px">
    <RecurringForm oncreate={handleCreate} oncancel={() => showAddModal = false} />
  </Modal>
{/if}

{#if deletingId}
  <ConfirmDialog
    message={t('recurring.confirmDelete')}
    danger={true}
    onconfirm={handleDelete}
    oncancel={() => deletingId = null}
  />
{/if}
