<script lang="ts">
  import { onMount } from 'svelte';
  import InvoiceStatusBadge from './InvoiceStatusBadge.svelte';
  import TaxSummary from './TaxSummary.svelte';
  import InvoicePreview from './InvoicePreview.svelte';
  import ConfirmDialog from '../shared/ConfirmDialog.svelte';
  import Modal from '../shared/Modal.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import {
    getCurrentInvoice, loadInvoice, finalizeInvoice, voidInvoice, deleteInvoice,
    duplicateInvoice, recordPayment, getLoading
  } from '../../stores/invoiceStore';
  import { formatCurrency } from '../../utils/currency';
  import { formatDate } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getSettings } from '../../stores/settingsStore';
  import { getViewParams, navigateTo } from '../../stores/navigationStore';
  import { success, error as toastError } from '../../stores/toastStore';
  import { getAllCurrencies } from '../../utils/currency';

  let params = $derived(getViewParams());
  let invoice = $derived(getCurrentInvoice());
  let loading = $derived(getLoading());
  let settings = $derived(getSettings());

  let showPreview = $state(false);
  let showConfirmFinalize = $state(false);
  let showConfirmVoid = $state(false);
  let showConfirmDelete = $state(false);
  let showPaymentModal = $state(false);

  // Payment form
  let paymentAmount = $state(0);
  let paymentMethod = $state('BankTransfer');
  let paymentRef = $state('');
  let paymentDate = $state(new Date().toISOString().slice(0, 10));
  let paymentNotes = $state('');

  onMount(async () => {
    const id = params.id;
    if (id) await loadInvoice(id);
  });

  async function handleFinalize() {
    if (!invoice) return;
    const result = await finalizeInvoice(invoice.id);
    if (result) {
      success(t('common.success'));
    } else {
      toastError(t('common.error'));
    }
    showConfirmFinalize = false;
  }

  async function handleVoid() {
    if (!invoice) return;
    await voidInvoice(invoice.id);
    showConfirmVoid = false;
    success(t('common.success'));
  }

  async function handleDelete() {
    if (!invoice) return;
    const ok = await deleteInvoice(invoice.id);
    if (ok) {
      navigateTo('invoices');
    }
    showConfirmDelete = false;
  }

  async function handleDuplicate() {
    if (!invoice) return;
    const dup = await duplicateInvoice(invoice.id);
    if (dup) {
      navigateTo('invoice-detail', { id: dup.id });
      success(t('common.success'));
    }
  }

  async function handleRecordPayment() {
    if (!invoice) return;
    const ok = await recordPayment(invoice.id, {
      amountMinor: Math.round(paymentAmount * Math.pow(10, getAllCurrencies().find(c => c.code === invoice.currencyCode)?.decimals ?? 2)),
      method: paymentMethod,
      reference: paymentRef,
      paidAt: paymentDate,
      notes: paymentNotes,
    });
    if (ok) {
      showPaymentModal = false;
      success(t('common.success'));
    }
  }
</script>

{#if loading && !invoice}
  <div style="display: flex; justify-content: center; padding: 64px;">
    <LoadingSpinner size="lg" />
  </div>
{:else if invoice}
  <div class="view-container">
    <!-- Header -->
    <div class="detail-header">
      <div class="detail-header-left">
        <button class="btn btn-ghost btn-sm" onclick={() => navigateTo('invoices')}>← {t('common.back')}</button>
        <h2 class="detail-title">#{invoice.invoiceNumber}</h2>
        <InvoiceStatusBadge status={invoice.status} />
      </div>
      <div class="detail-actions">
        {#if invoice.status === 'Draft'}
          <button class="btn" onclick={() => navigateTo('invoice-editor', { id: invoice.id })}>
            ✏️ {t('common.edit')}
          </button>
          <button class="btn btn-primary" onclick={() => showConfirmFinalize = true}>
            {t('invoices.finalize')}
          </button>
        {/if}
        {#if invoice.status === 'Finalized' || invoice.status === 'Sent'}
          <button class="btn btn-primary" onclick={() => showPaymentModal = true}>
            💰 {t('invoices.recordPayment')}
          </button>
        {/if}
        <button class="btn" onclick={() => showPreview = true}>
          📄 {t('invoices.preview')}
        </button>
        <button class="btn" onclick={handleDuplicate}>
          {t('invoices.duplicate')}
        </button>
        {#if invoice.status !== 'Void' && invoice.status !== 'Paid'}
          <button class="btn btn-ghost" style="color: var(--color-danger);" onclick={() => showConfirmVoid = true}>
            {t('invoices.void')}
          </button>
        {/if}
        {#if invoice.status === 'Draft'}
          <button class="btn btn-ghost" style="color: var(--color-danger);" onclick={() => showConfirmDelete = true}>
            {t('invoices.delete')}
          </button>
        {/if}
      </div>
    </div>

    <div class="detail-grid">
      <!-- Main Content -->
      <div class="detail-main">
        <!-- Business Info -->
        <div class="card">
          <div class="info-two-col">
            <div>
              <h4 class="info-label">{t('invoices.businessInfo')}</h4>
              <!-- populated from business profile -->
            </div>
            <div>
              <h4 class="info-label">{t('invoices.billTo')}</h4>
              <p class="info-name">{invoice.clientName}</p>
            </div>
          </div>
          <div class="info-two-col" style="margin-block-start: var(--space-md);">
            <div>
              <span class="info-label">{t('invoices.issueDate')}:</span>
              <span> {formatDate(invoice.issueDate, settings.dateFormat)}</span>
            </div>
            <div>
              <span class="info-label">{t('invoices.dueDate')}:</span>
              <span> {formatDate(invoice.dueDate, settings.dateFormat)}</span>
            </div>
          </div>
        </div>

        <!-- Line Items -->
        <div class="card" style="padding: 0; overflow: hidden;">
          <div style="padding: var(--space-md);">
            <h3 class="card-title">{t('invoices.lineItems')}</h3>
          </div>
          <table class="table">
            <thead>
              <tr>
                <th>{t('invoices.description')}</th>
                <th style="text-align: end;">{t('invoices.quantity')}</th>
                <th style="text-align: end;">{t('invoices.unitPrice')}</th>
                <th style="text-align: end;">{t('invoices.discountPercent')}</th>
                <th style="text-align: end;">{t('invoices.lineTotal')}</th>
              </tr>
            </thead>
            <tbody>
              {#each invoice.lineItems as item}
                <tr>
                  <td>{item.description}</td>
                  <td style="text-align: end;">{item.quantity}</td>
                  <td style="text-align: end;" class="currency">
                    {formatCurrency(item.unitPriceMinor, invoice.currencyCode)}
                  </td>
                  <td style="text-align: end;">
                    {item.discountPercent > 0 ? item.discountPercent + '%' : '—'}
                  </td>
                  <td style="text-align: end;" class="currency">
                    {formatCurrency(
                      Math.round(item.quantity * item.unitPriceMinor * (1 - item.discountPercent / 100)),
                      invoice.currencyCode
                    )}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

        <!-- Notes / Terms -->
        {#if invoice.notes || invoice.terms}
          <div class="card">
            {#if invoice.notes}
              <div class="note-section">
                <h4 class="info-label">{t('invoices.notes')}</h4>
                <p class="note-text">{invoice.notes}</p>
              </div>
            {/if}
            {#if invoice.terms}
              <div class="note-section">
                <h4 class="info-label">{t('invoices.terms')}</h4>
                <p class="note-text">{invoice.terms}</p>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Payment History -->
        {#if invoice.payments && invoice.payments.length > 0}
          <div class="card">
            <h3 class="card-title" style="margin-block-end: var(--space-md);">{t('payments.title')}</h3>
            <table class="table">
              <thead>
                <tr>
                  <th>{t('payments.date')}</th>
                  <th>{t('payments.method')}</th>
                  <th>{t('payments.reference')}</th>
                  <th style="text-align: end;">{t('payments.amount')}</th>
                </tr>
              </thead>
              <tbody>
                {#each invoice.payments as p}
                  <tr>
                    <td>{formatDate(p.paidAt, settings.dateFormat)}</td>
                    <td>{p.method}</td>
                    <td>{p.reference || '—'}</td>
                    <td style="text-align: end;" class="currency">
                      {formatCurrency(p.amountMinor, invoice.currencyCode)}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <!-- Totals Sidebar -->
      <div class="detail-sidebar">
        <div class="card">
          <TaxSummary
            taxLines={invoice.taxLines}
            subtotalMinor={invoice.subtotalMinor}
            discountMinor={invoice.discountMinor}
            taxTotalMinor={invoice.taxTotalMinor}
            totalMinor={invoice.totalMinor}
            amountPaidMinor={invoice.amountPaidMinor}
            balanceDueMinor={invoice.balanceDueMinor}
            currencyCode={invoice.currencyCode}
          />
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Confirm Dialogs -->
{#if showConfirmFinalize}
  <ConfirmDialog
    message={t('invoices.confirmFinalize')}
    onconfirm={handleFinalize}
    oncancel={() => showConfirmFinalize = false}
  />
{/if}

{#if showConfirmVoid}
  <ConfirmDialog
    message={t('invoices.confirmVoid')}
    danger={true}
    onconfirm={handleVoid}
    oncancel={() => showConfirmVoid = false}
  />
{/if}

{#if showConfirmDelete}
  <ConfirmDialog
    message={t('invoices.confirmDelete')}
    danger={true}
    onconfirm={handleDelete}
    oncancel={() => showConfirmDelete = false}
  />
{/if}

<!-- Preview Modal -->
{#if showPreview && invoice}
  <InvoicePreview {invoice} onclose={() => showPreview = false} />
{/if}

<!-- Record Payment Modal -->
{#if showPaymentModal && invoice}
  <Modal title={t('payments.record')} onclose={() => showPaymentModal = false}>
    <div class="form-group">
      <label class="form-label" for="pay-amount">{t('payments.amount')}</label>
      <input
        id="pay-amount"
        class="input"
        type="number"
        min="0"
        step="0.01"
        bind:value={paymentAmount}
      />
    </div>
    <div class="form-group mt-md">
      <label class="form-label" for="pay-method">{t('payments.method')}</label>
      <select id="pay-method" class="select" bind:value={paymentMethod}>
        <option value="Cash">{t('payments.cash')}</option>
        <option value="BankTransfer">{t('payments.bankTransfer')}</option>
        <option value="MobileMoney">{t('payments.mobileMoney')}</option>
        <option value="Cheque">{t('payments.cheque')}</option>
        <option value="Other">{t('payments.other')}</option>
      </select>
    </div>
    <div class="form-group mt-md">
      <label class="form-label" for="pay-ref">{t('payments.reference')}</label>
      <input id="pay-ref" class="input" type="text" bind:value={paymentRef} />
    </div>
    <div class="form-group mt-md">
      <label class="form-label" for="pay-date">{t('payments.date')}</label>
      <input id="pay-date" class="input" type="date" bind:value={paymentDate} />
    </div>
    <div class="form-group mt-md">
      <label class="form-label" for="pay-notes">{t('payments.notes')}</label>
      <textarea id="pay-notes" class="textarea" rows="2" bind:value={paymentNotes}></textarea>
    </div>

    {#snippet footer()}
      <button class="btn" onclick={() => showPaymentModal = false}>{t('common.cancel')}</button>
      <button class="btn btn-primary" onclick={handleRecordPayment}>{t('common.save')}</button>
    {/snippet}
  </Modal>
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
    flex-wrap: wrap;
  }

  .detail-grid {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: var(--space-md);
    align-items: start;
  }

  .detail-main {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .detail-sidebar {
    position: sticky;
    top: var(--space-md);
  }

  .info-two-col {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-md);
  }

  .info-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
    margin-block-end: var(--space-xs);
  }

  .info-name {
    font-size: var(--font-size-base);
    font-weight: 600;
  }

  .note-section + .note-section {
    margin-block-start: var(--space-md);
    padding-block-start: var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  .note-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    white-space: pre-wrap;
    line-height: 1.6;
  }
</style>
