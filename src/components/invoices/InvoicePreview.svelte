<script lang="ts">
  import Modal from '../shared/Modal.svelte';
  import TaxSummary from './TaxSummary.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';
  import { formatCurrency } from '../../utils/currency';
  import { formatDate } from '../../utils/date';
  import { t } from '../../stores/i18nStore';
  import { getSettings, getBusinessProfile } from '../../stores/settingsStore';
  import { success, error as toastError } from '../../stores/toastStore';
  import type { Invoice } from '../../stores/invoiceStore';

  interface Props {
    invoice: Invoice;
    onclose: () => void;
  }

  let { invoice, onclose }: Props = $props();

  let settings = $derived(getSettings());
  let profile = $derived(getBusinessProfile());

  function base64ToBytes(base64: string) {
    const binary = atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i += 1) {
      bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
  }

  function invoicePdfName() {
    const stem = invoice.invoiceNumber || `invoice-${invoice.id.slice(0, 8)}`;
    return `${stem.replace(/[^a-zA-Z0-9._-]+/g, '-')}.pdf`;
  }

  async function handleDownloadPdf() {
    try {
      const path = await save({
        defaultPath: invoicePdfName(),
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
      });
      if (!path) return;
      const pdfBase64 = await invoke<string>('generate_invoice_pdf', { invoiceId: invoice.id });
      await writeFile(path, base64ToBytes(pdfBase64));
      success(t('common.success'));
    } catch (e) {
      toastError(String(e));
    }
  }
</script>

<Modal title={t('invoices.preview')} maxWidth="720px" {onclose}>
  <div class="invoice-preview">
    <!-- Header -->
    <div class="preview-header">
      <div class="preview-from">
        {#if profile.companyName}
          <h2 class="company-name">{profile.companyName}</h2>
        {/if}
        {#if profile.address}<p>{profile.address}</p>{/if}
        {#if profile.city}<p>{profile.city}, {profile.country}</p>{/if}
        {#if profile.phone}<p>{profile.phone}</p>{/if}
        {#if profile.email}<p>{profile.email}</p>{/if}
        {#if profile.taxId}<p>{t('clients.taxId')}: {profile.taxId}</p>{/if}
      </div>
      <div class="preview-invoice-info">
        <h1 class="preview-title">{t('invoices.invoice')}</h1>
        <table class="preview-meta-table">
          <tbody>
            <tr>
              <td class="meta-label">{t('invoices.number')}</td>
              <td class="meta-value">#{invoice.invoiceNumber}</td>
            </tr>
            <tr>
              <td class="meta-label">{t('invoices.date')}</td>
              <td class="meta-value">{formatDate(invoice.issueDate, settings.dateFormat)}</td>
            </tr>
            <tr>
              <td class="meta-label">{t('invoices.dueDate')}</td>
              <td class="meta-value">{formatDate(invoice.dueDate, settings.dateFormat)}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Bill To -->
    <div class="preview-bill-to">
      <h4 class="section-label">{t('invoices.billTo')}</h4>
      <p class="client-name">{invoice.clientName}</p>
    </div>

    <!-- Line Items -->
    <table class="preview-items-table">
      <thead>
        <tr>
          <th>{t('invoices.description')}</th>
          <th class="num">{t('invoices.quantity')}</th>
          <th class="num">{t('invoices.unitPrice')}</th>
          <th class="num">{t('invoices.discountPercent')}</th>
          <th class="num">{t('invoices.lineTotal')}</th>
        </tr>
      </thead>
      <tbody>
        {#each invoice.lineItems as item}
          <tr>
            <td>{item.description}</td>
            <td class="num">{item.quantity}</td>
            <td class="num">{formatCurrency(item.unitPriceMinor, invoice.currencyCode)}</td>
            <td class="num">{item.discountPercent > 0 ? item.discountPercent + '%' : '—'}</td>
            <td class="num">{formatCurrency(
              Math.round(item.quantity * item.unitPriceMinor * (1 - item.discountPercent / 100)),
              invoice.currencyCode
            )}</td>
          </tr>
        {/each}
      </tbody>
    </table>

    <!-- Totals -->
    <div class="preview-totals">
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

    <!-- Notes / Terms -->
    {#if invoice.notes}
      <div class="preview-section">
        <h4 class="section-label">{t('invoices.notes')}</h4>
        <p class="section-text">{invoice.notes}</p>
      </div>
    {/if}
    {#if invoice.terms}
      <div class="preview-section">
        <h4 class="section-label">{t('invoices.terms')}</h4>
        <p class="section-text">{invoice.terms}</p>
      </div>
    {/if}

    <!-- Payment Info -->
    {#if profile.bankName || profile.mobileMoney}
      <div class="preview-section">
        <h4 class="section-label">{t('payments.information')}</h4>
        {#if profile.bankName}
          <p>{t('payments.bank')}: {profile.bankName}</p>
          {#if profile.bankAccount}<p>{t('payments.account')}: {profile.bankAccount}</p>{/if}
        {/if}
        {#if profile.mobileMoney}
          <p>{profile.mobileMoneyProvider ? profile.mobileMoneyProvider + ': ' : ''}{profile.mobileMoney}</p>
        {/if}
      </div>
    {/if}

    {#if invoice.footer}
      <div class="preview-footer-text">{invoice.footer}</div>
    {/if}
  </div>

  {#snippet footer()}
    <button class="btn" onclick={onclose}>{t('common.close')}</button>
    <button class="btn" onclick={handleDownloadPdf}>↓ {t('common.download')}</button>
    <button class="btn btn-primary" onclick={() => window.print()}>🖨 {t('common.print')}</button>
  {/snippet}
</Modal>

<style>
  .invoice-preview {
    font-size: 13px;
    line-height: 1.5;
    color: #1a1a1a;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--space-lg);
    margin-block-end: var(--space-lg);
    padding-block-end: var(--space-lg);
    border-bottom: 2px solid #eee;
  }

  .company-name {
    font-size: 1rem;
    font-weight: 700;
    margin-block-end: var(--space-xs);
    color: var(--color-teal-primary);
  }

  .preview-from p {
    font-size: 12px;
    color: #555;
    margin-block-end: 2px;
  }

  .preview-title {
    font-size: 1.75rem;
    font-weight: 800;
    letter-spacing: 0.05em;
    color: var(--color-teal-dark);
    text-align: end;
    margin-block-end: var(--space-sm);
  }

  .preview-meta-table td {
    padding: 2px 0;
    font-size: 12px;
  }

  .meta-label {
    color: #888;
    padding-inline-end: var(--space-md);
    white-space: nowrap;
  }

  .meta-value {
    font-weight: 500;
    text-align: end;
  }

  .preview-bill-to {
    margin-block-end: var(--space-lg);
  }

  .section-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #888;
    margin-block-end: var(--space-xs);
  }

  .client-name {
    font-size: 1rem;
    font-weight: 600;
  }

  .preview-items-table {
    width: 100%;
    border-collapse: collapse;
    margin-block-end: var(--space-lg);
    font-size: 12px;
  }

  .preview-items-table th {
    text-align: start;
    padding: var(--space-xs) var(--space-sm);
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #888;
    border-bottom: 1px solid #eee;
    background: #fafafa;
  }

  .preview-items-table td {
    padding: var(--space-xs) var(--space-sm);
    border-bottom: 1px solid #f0f0f0;
  }

  .preview-items-table .num {
    text-align: end;
  }

  .preview-totals {
    margin-block-end: var(--space-lg);
    padding-block-start: var(--space-sm);
  }

  .preview-section {
    margin-block-end: var(--space-md);
    padding-block-start: var(--space-md);
    border-top: 1px solid #eee;
  }

  .section-text {
    font-size: 12px;
    color: #555;
    white-space: pre-wrap;
  }

  .preview-footer-text {
    margin-block-start: var(--space-lg);
    padding-block-start: var(--space-md);
    border-top: 1px solid #eee;
    font-size: 11px;
    color: #888;
    text-align: center;
  }
</style>
