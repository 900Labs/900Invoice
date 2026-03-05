<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getClients } from '../../stores/clientStore';
  import { getInvoices } from '../../stores/invoiceStore';
  import { today } from '../../utils/date';

  export interface RecurringFormData {
    clientId: string;
    templateInvoiceId: string;
    frequency: 'weekly' | 'monthly' | 'quarterly' | 'annual';
    startDate: string;
    endDate: string;
    autoSend: boolean;
  }

  interface Props {
    data?: Partial<RecurringFormData>;
    oncreate?: (data: RecurringFormData) => void;
    onupdate?: (data: Partial<RecurringFormData>) => void;
    oncancel?: () => void;
  }

  let { data, oncreate, onupdate, oncancel }: Props = $props();

  let clients = $derived(getClients());
  let allInvoices = $derived(getInvoices());

  let clientId = $state('');
  let templateInvoiceId = $state('');
  let frequency = $state<RecurringFormData['frequency']>('monthly');
  let startDate = $state(today());
  let endDate = $state('');
  let autoSend = $state(false);
  let errors = $state<Record<string, string>>({});

  // Sync from prop (edit mode)
  $effect(() => {
    if (data) {
      clientId = data.clientId ?? '';
      templateInvoiceId = data.templateInvoiceId ?? '';
      frequency = data.frequency ?? 'monthly';
      startDate = data.startDate ?? today();
      endDate = data.endDate ?? '';
      autoSend = data.autoSend ?? false;
    }
  });

  let clientInvoices = $derived(
    clientId ? allInvoices.filter(i => i.clientId === clientId) : allInvoices
  );

  const frequencies: Array<{ value: RecurringFormData['frequency']; label: string }> = [
    { value: 'weekly', label: t('recurring.weekly') },
    { value: 'monthly', label: t('recurring.monthly') },
    { value: 'quarterly', label: t('recurring.quarterly') },
    { value: 'annual', label: t('recurring.annual') },
  ];

  function handleSubmit() {
    errors = {};
    if (!clientId) {
      errors.clientId = t('validation.selectClient');
    }
    if (Object.keys(errors).length > 0) return;

    const formData: RecurringFormData = {
      clientId, templateInvoiceId, frequency, startDate, endDate, autoSend,
    };

    if (data) {
      onupdate?.(formData);
    } else {
      oncreate?.(formData);
    }
  }
</script>

<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <div class="form-group">
    <label class="form-label required" for="lbl-recurring-client">{t('recurring.client')}</label>
    <select id="lbl-recurring-client" class="select {errors.clientId ? 'error' : ''}" bind:value={clientId}>
      <option value="">{t('common.select')}</option>
      {#each clients as client}
        <option value={client.id}>{client.name}</option>
      {/each}
    </select>
    {#if errors.clientId}<span class="form-error">{errors.clientId}</span>{/if}
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-recurring-templateinvoice">{t('recurring.templateInvoice')}</label>
    <select id="lbl-recurring-templateinvoice" class="select" bind:value={templateInvoiceId}>
      <option value="">{t('common.none')}</option>
      {#each clientInvoices as inv}
        <option value={inv.id}>#{inv.invoiceNumber}</option>
      {/each}
    </select>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-recurring-frequency">{t('recurring.frequency')}</label>
    <select id="lbl-recurring-frequency" class="select" bind:value={frequency}>
      {#each frequencies as f}
        <option value={f.value}>{f.label}</option>
      {/each}
    </select>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-recurring-startdate">{t('recurring.startDate')}</label>
      <input id="lbl-recurring-startdate" class="input" type="date" bind:value={startDate} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-recurring-enddate">{t('recurring.endDate')} ({t('common.optional')})</label>
      <input id="lbl-recurring-enddate" class="input" type="date" bind:value={endDate} />
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" style="display: flex; align-items: center; gap: var(--space-sm); cursor: pointer;" for="lbl-field-5">
      <input id="lbl-field-5" type="checkbox" bind:checked={autoSend} />
      {t('recurring.autoSend')}
    </label>
  </div>

  <div class="form-actions mt-md">
    {#if oncancel}
      <button type="button" class="btn" onclick={oncancel}>{t('common.cancel')}</button>
    {/if}
    <button type="submit" class="btn btn-primary">
      {data ? t('common.save') : t('common.create')}
    </button>
  </div>
</form>

<style>
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
  }
  .mt-md { margin-block-start: var(--space-md); }
</style>
