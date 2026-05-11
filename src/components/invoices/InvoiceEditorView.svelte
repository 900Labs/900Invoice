<script lang="ts">
  import { onMount } from 'svelte';
  import CurrencyInput from '../shared/CurrencyInput.svelte';
  import LineItemRow from './LineItemRow.svelte';
  import TaxSummary from './TaxSummary.svelte';
  import ConfirmDialog from '../shared/ConfirmDialog.svelte';
  import Modal from '../shared/Modal.svelte';
  import ClientForm from '../clients/ClientForm.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import {
    getClients, loadClients, createClient, type CreateClient,
  } from '../../stores/clientStore';
  import {
    getActiveProducts, loadProducts,
  } from '../../stores/productStore';
  import {
    getActiveTaxRates, loadTaxRates,
  } from '../../stores/taxStore';
  import {
    getCurrentInvoice, loadInvoice, createInvoice, updateInvoice, finalizeInvoice,
    getLoading,
  } from '../../stores/invoiceStore';
  import { getSettings, getBusinessProfile } from '../../stores/settingsStore';
  import { getAllCurrencies, formatCurrency, calcLineTotal, calcLineTax } from '../../utils/currency';
  import { formatDate, addDays, today } from '../../utils/date';
  import { validateInvoice } from '../../utils/validation';
  import { t } from '../../stores/i18nStore';
  import { getViewParams, navigateTo } from '../../stores/navigationStore';
  import { success, error as toastError } from '../../stores/toastStore';

  // ─── State ────────────────────────────────────────────
  let params = $derived(getViewParams());
  let settings = $derived(getSettings());
  let profile = $derived(getBusinessProfile());
  let clients = $derived(getClients());
  let products = $derived(getActiveProducts());
  let taxRates = $derived(getActiveTaxRates());
  let loading = $derived(getLoading());
  let currencies = getAllCurrencies();

  let editId = $state<string | null>(null);
  let isEditing = $state(false);

  // Form state
  let clientId = $state('');
  let clientSearch = $state('');
  let issueDate = $state(today());
  let paymentTermsDays = $state(30);
  let dueDate = $state(addDays(today(), 30));
  let currencyCode = $state('USD');
  let taxMode = $state<'Exclusive' | 'Inclusive'>('Exclusive');
  let notes = $state('');
  let terms = $state('');
  let footer = $state('');

  // Line items
  interface LineItemDraft {
    tempId: string;
    productId: string | null;
    description: string;
    quantity: number;
    unitPriceMinor: number;
    taxRateId: string | null;
    discountPercent: number;
    sortOrder: number;
  }

  let lineItems = $state<LineItemDraft[]>([createEmptyLine(0)]);

  // Validation errors
  let errors = $state<Record<string, string>>({});

  // UI state
  let showAddClient = $state(false);
  let showConfirmFinalize = $state(false);
  let saving = $state(false);

  // Filtered clients for dropdown
  let filteredClients = $derived(
    clientSearch
      ? clients.filter(c =>
          c.name.toLowerCase().includes(clientSearch.toLowerCase()) ||
          c.email.toLowerCase().includes(clientSearch.toLowerCase())
        )
      : clients
  );

  // ─── Computed Totals ─────────────────────────────────
  let subtotalMinor = $derived(
    lineItems.reduce((sum, item) => sum + calcLineTotal(item.quantity, item.unitPriceMinor, item.discountPercent), 0)
  );

  let discountMinor = $derived(
    lineItems.reduce((sum, item) => {
      const raw = item.quantity * item.unitPriceMinor;
      return sum + Math.round(raw * (item.discountPercent / 100));
    }, 0)
  );

  interface TaxAccum {
    taxRateId: string;
    taxName: string;
    taxDisplayName: string;
    rateBps: number;
    baseAmountMinor: number;
    taxAmountMinor: number;
  }

  let taxLines = $derived((() => {
    const acc: Record<string, TaxAccum> = {};
    for (const item of lineItems) {
      if (!item.taxRateId) continue;
      const rate = taxRates.find(r => r.id === item.taxRateId);
      if (!rate) continue;
      const lineTotal = calcLineTotal(item.quantity, item.unitPriceMinor, item.discountPercent);
      const taxAmt = calcLineTax(lineTotal, rate.rateBps, taxMode);
      if (!acc[rate.id]) {
        acc[rate.id] = {
          taxRateId: rate.id,
          taxName: rate.name,
          taxDisplayName: rate.displayName,
          rateBps: rate.rateBps,
          baseAmountMinor: 0,
          taxAmountMinor: 0,
        };
      }
      acc[rate.id].baseAmountMinor += lineTotal;
      acc[rate.id].taxAmountMinor += taxAmt;
    }
    return Object.values(acc);
  })());

  let taxTotalMinor = $derived(taxLines.reduce((sum, tl) => sum + tl.taxAmountMinor, 0));

  let totalMinor = $derived(
    taxMode === 'Exclusive'
      ? subtotalMinor + taxTotalMinor
      : subtotalMinor
  );

  // ─── Helper functions ─────────────────────────────────
  function createEmptyLine(sortOrder: number): LineItemDraft {
    return {
      tempId: Math.random().toString(36).slice(2, 9),
      productId: null,
      description: '',
      quantity: 1,
      unitPriceMinor: 0,
      taxRateId: taxRates[0]?.id ?? null,
      discountPercent: 0,
      sortOrder,
    };
  }

  function addLineItem() {
    lineItems = [...lineItems, createEmptyLine(lineItems.length)];
  }

  function removeLineItem(tempId: string) {
    lineItems = lineItems.filter(i => i.tempId !== tempId);
  }

  function updateLineItem(tempId: string, field: string, value: unknown) {
    lineItems = lineItems.map(item =>
      item.tempId === tempId ? { ...item, [field]: value } : item
    );
  }

  function handleClientSelect(id: string) {
    clientId = id;
    const client = clients.find(c => c.id === id);
    if (client) {
      currencyCode = client.currencyCode || currencyCode;
      paymentTermsDays = client.paymentTermsDays || 30;
      dueDate = addDays(issueDate, paymentTermsDays);
      clientSearch = '';
    }
  }

  function handleIssueDateChange(e: Event) {
    issueDate = (e.target as HTMLInputElement).value;
    dueDate = addDays(issueDate, paymentTermsDays);
  }

  function handleTermsChange(e: Event) {
    paymentTermsDays = parseInt((e.target as HTMLInputElement).value) || 0;
    dueDate = addDays(issueDate, paymentTermsDays);
  }

  // ─── Save / Submit ─────────────────────────────────────
  async function handleSaveDraft() {
    errors = {};
    const validation = validateInvoice({ clientId, lineItems });
    if (!validation.valid) {
      errors = validation.errors;
      return;
    }

    saving = true;
    try {
      const data = buildPayload();
      let result;
      if (isEditing && editId) {
        result = await updateInvoice(editId, data);
      } else {
        result = await createInvoice(data);
      }
      if (result) {
        success(t('common.success'));
        navigateTo('invoice-detail', { id: result.id });
      } else {
        toastError(t('common.error'));
      }
    } finally {
      saving = false;
    }
  }

  async function handleSaveFinalize() {
    errors = {};
    const validation = validateInvoice({ clientId, lineItems });
    if (!validation.valid) {
      errors = validation.errors;
      return;
    }
    showConfirmFinalize = true;
  }

  async function doSaveAndFinalize() {
    showConfirmFinalize = false;
    saving = true;
    try {
      const data = buildPayload();
      let invoice;
      if (isEditing && editId) {
        invoice = await updateInvoice(editId, data);
      } else {
        invoice = await createInvoice(data);
      }
      if (invoice) {
        const finalized = await finalizeInvoice(invoice.id);
        if (finalized) {
          success(t('common.success'));
          navigateTo('invoice-detail', { id: finalized.id });
        }
      }
    } finally {
      saving = false;
    }
  }

  function buildPayload() {
    return {
      clientId,
      currencyCode,
      issueDate,
      dueDate,
      taxMode,
      notes,
      terms,
      footer,
      lineItems: lineItems.map((item, i) => ({
        id: isEditing ? item.tempId : undefined,
        productId: item.productId,
        description: item.description,
        quantity: item.quantity,
        unitPriceMinor: item.unitPriceMinor,
        taxRateId: item.taxRateId,
        discountPercent: item.discountPercent,
        sortOrder: i,
      })),
    };
  }

  async function handleClientCreated(clientData: CreateClient) {
    const newClient = await createClient(clientData);
    if (newClient) {
      handleClientSelect(newClient.id);
      showAddClient = false;
      success(t('common.success'));
    }
  }

  // ─── Load existing invoice if editing ─────────────────
  onMount(async () => {
    if (settings.defaultCurrency) {
      currencyCode = settings.defaultCurrency;
    }

    await Promise.all([
      loadClients(),
      loadProducts(),
      loadTaxRates(),
    ]);

    const id = params.id;
    if (id) {
      isEditing = true;
      editId = id;
      await loadInvoice(id);
      const inv = getCurrentInvoice();
      if (inv) {
        clientId = inv.clientId;
        issueDate = inv.issueDate;
        dueDate = inv.dueDate;
        currencyCode = inv.currencyCode;
        taxMode = inv.taxMode;
        notes = inv.notes || '';
        terms = inv.terms || '';
        footer = inv.footer || '';
        lineItems = inv.lineItems.map(item => ({
          tempId: item.id,
          productId: item.productId,
          description: item.description,
          quantity: item.quantity,
          unitPriceMinor: item.unitPriceMinor,
          taxRateId: item.taxRateId,
          discountPercent: item.discountPercent,
          sortOrder: item.sortOrder,
        }));
        paymentTermsDays = 30;
      }
    }
  });

  let selectedClientName = $derived(
    clients.find(c => c.id === clientId)?.name ?? ''
  );
</script>

<div class="view-container">
  <div class="editor-header">
    <div>
      <button class="btn btn-ghost btn-sm" onclick={() => navigateTo('invoices')}>← {t('common.back')}</button>
      <h2 class="editor-title">
        {isEditing ? t('invoices.edit') : t('invoices.new')}
      </h2>
    </div>
    <div class="editor-actions">
      <button class="btn" onclick={() => navigateTo('invoices')} disabled={saving}>
        {t('common.cancel')}
      </button>
      <button class="btn" onclick={handleSaveDraft} disabled={saving}>
        {#if saving}<span class="spinner" style="width:14px;height:14px;"></span>{/if}
        {t('invoices.saveDraft')}
      </button>
      <button class="btn btn-primary" onclick={handleSaveFinalize} disabled={saving}>
        {t('invoices.saveFinalize')}
      </button>
    </div>
  </div>

  <div class="editor-grid">
    <!-- Main Editor -->
    <div class="editor-main">
      <!-- Invoice Details Card -->
      <div class="card">
        <h3 class="card-title" style="margin-block-end: var(--space-md);">{t('invoices.invoiceDetails')}</h3>
        <div class="form-row">
          <!-- Client selector -->
          <div class="form-group">
            <label class="form-label required" for="lbl-invoices-selectclient">{t('invoices.selectClient')}</label>
            <div class="client-selector">
              <input id="lbl-invoices-selectclient"
                class="input {errors.clientId ? 'error' : ''}"
                type="text"
                placeholder={t('invoices.selectClient')}
                value={selectedClientName || clientSearch}
                oninput={(e) => {
                  clientSearch = (e.target as HTMLInputElement).value;
                  if (!clientSearch) clientId = '';
                }}
                autocomplete="off"
              />
              {#if clientSearch && filteredClients.length > 0 && !clientId}
                <div class="client-dropdown">
                  {#each filteredClients.slice(0, 8) as client}
                    <button
                      class="client-option"
                      onclick={() => handleClientSelect(client.id)}
                    >
                      <span class="client-option-name">{client.name}</span>
                      <span class="client-option-email">{client.email}</span>
                    </button>
                  {/each}
                  <button
                    class="client-option add-client-option"
                    onclick={() => showAddClient = true}
                  >
                    + {t('invoices.addNewClient')}
                  </button>
                </div>
              {/if}
              {#if !clientSearch || clientId}
                <button class="btn-add-client" onclick={() => showAddClient = true}>
                  + {t('invoices.addNewClient')}
                </button>
              {/if}
            </div>
            {#if errors.clientId}
              <span class="form-error">{errors.clientId}</span>
            {/if}
          </div>

          <!-- Currency -->
          <div class="form-group">
            <label class="form-label" for="lbl-invoices-currency">{t('invoices.currency')}</label>
            <select id="lbl-invoices-currency" class="select" bind:value={currencyCode}>
              {#each currencies as c}
                <option value={c.code}>{c.code} — {c.name}</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="form-row mt-md">
          <!-- Issue date -->
          <div class="form-group">
            <label class="form-label" for="lbl-invoices-issuedate">{t('invoices.issueDate')}</label>
            <input id="lbl-invoices-issuedate"
              class="input"
              type="date"
              value={issueDate}
              onchange={handleIssueDateChange}
            />
          </div>

          <!-- Payment Terms -->
          <div class="form-group">
            <label class="form-label" for="lbl-invoices-paymentterms">{t('invoices.paymentTerms')}</label>
            <div class="input-with-suffix">
              <input id="lbl-invoices-paymentterms"
                class="input"
                type="number"
                min="0"
                step="1"
                value={paymentTermsDays}
                onchange={handleTermsChange}
              />
              <span class="input-suffix">{t('invoices.days')}</span>
            </div>
          </div>

          <!-- Due date (readonly calculated) -->
          <div class="form-group">
            <label class="form-label" for="lbl-invoices-duedate">{t('invoices.dueDate')}</label>
            <input id="lbl-invoices-duedate"
              class="input"
              type="date"
              bind:value={dueDate}
            />
          </div>
        </div>

        <!-- Tax Mode -->
        <div class="form-group mt-md">
          <label class="form-label" for="lbl-invoices-taxmode">{t('invoices.taxMode')}</label>
          <div class="tax-mode-toggle">
            <button
              class="tax-mode-btn"
              class:active={taxMode === 'Exclusive'}
              onclick={() => taxMode = 'Exclusive'}
            >
              {t('invoices.taxExclusive')}
            </button>
            <button
              class="tax-mode-btn"
              class:active={taxMode === 'Inclusive'}
              onclick={() => taxMode = 'Inclusive'}
            >
              {t('invoices.taxInclusive')}
            </button>
          </div>
        </div>
      </div>

      <!-- Line Items Card -->
      <div class="card" style="padding: 0; overflow: hidden;">
        <div class="line-items-header">
          <h3 class="card-title">{t('invoices.lineItems')}</h3>
        </div>

        {#if errors.lineItems}
          <div style="padding: var(--space-sm) var(--space-md);">
            <span class="form-error">{errors.lineItems}</span>
          </div>
        {/if}

        <div class="line-items-scroll">
          <table class="table line-items-table">
            <thead>
              <tr>
                <th style="width: 24px;"></th>
                <th style="width: 130px;">{t('invoices.selectProduct')}</th>
                <th>{t('invoices.description')}</th>
                <th style="width: 72px; text-align: end;">{t('invoices.quantity')}</th>
                <th style="width: 130px; text-align: end;">{t('invoices.unitPrice')}</th>
                <th style="width: 140px;">{t('invoices.taxRate')}</th>
                <th style="width: 80px; text-align: end;">{t('invoices.discountPercent')}</th>
                <th style="width: 110px; text-align: end;">{t('invoices.lineTotal')}</th>
                <th style="width: 32px;"></th>
              </tr>
            </thead>
            <tbody>
              {#each lineItems as item (item.tempId)}
                <LineItemRow
                  index={lineItems.indexOf(item)}
                  description={item.description}
                  quantity={item.quantity}
                  unitPriceMinor={item.unitPriceMinor}
                  taxRateId={item.taxRateId}
                  discountPercent={item.discountPercent}
                  currencyCode={currencyCode}
                  taxRates={taxRates}
                  products={products.map(p => ({
                    id: p.id,
                    name: p.name,
                    defaultPriceMinor: p.defaultPriceMinor,
                    taxRateId: p.taxRateId ?? taxRates.find(rate => rate.rateBps === p.taxRateBps)?.id ?? null,
                  }))}
                  onupdate={(field, val) => updateLineItem(item.tempId, field, val)}
                  ondelete={() => removeLineItem(item.tempId)}
                />
              {/each}
            </tbody>
          </table>
        </div>

        <div class="line-items-footer">
          <button class="btn" onclick={addLineItem}>
            + {t('invoices.addLineItem')}
          </button>
        </div>
      </div>

      <!-- Notes / Terms / Footer -->
      <div class="card">
        <div class="form-row">
          <div class="form-group">
            <label class="form-label" for="lbl-invoices-notes">{t('invoices.notes')}</label>
            <textarea id="lbl-invoices-notes" class="textarea" rows="3" bind:value={notes} placeholder={t('invoices.notes')}></textarea>
          </div>
          <div class="form-group">
            <label class="form-label" for="lbl-invoices-terms">{t('invoices.terms')}</label>
            <textarea id="lbl-invoices-terms" class="textarea" rows="3" bind:value={terms} placeholder={t('invoices.terms')}></textarea>
          </div>
        </div>
        <div class="form-group mt-md">
          <label class="form-label" for="lbl-invoices-footer">{t('invoices.footer')}</label>
          <input id="lbl-invoices-footer" class="input" type="text" bind:value={footer} placeholder={t('invoices.footer')} />
        </div>
      </div>
    </div>

    <!-- Totals Sidebar -->
    <div class="editor-sidebar">
      <div class="card" style="position: sticky; top: var(--space-md);">
        <h3 class="card-title" style="margin-block-end: var(--space-md);">{t('invoices.total')}</h3>
        <TaxSummary
          taxLines={taxLines}
          subtotalMinor={subtotalMinor}
          discountMinor={discountMinor}
          taxTotalMinor={taxTotalMinor}
          totalMinor={totalMinor}
          currencyCode={currencyCode}
        />

        <div style="margin-block-start: var(--space-lg); display: flex; flex-direction: column; gap: var(--space-sm);">
          <button class="btn w-full" onclick={handleSaveDraft} disabled={saving}>
            {t('invoices.saveDraft')}
          </button>
          <button class="btn btn-primary w-full" onclick={handleSaveFinalize} disabled={saving}>
            {t('invoices.saveFinalize')}
          </button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Add Client Modal -->
{#if showAddClient}
  <Modal title={t('clients.new')} onclose={() => showAddClient = false} maxWidth="640px">
    <ClientForm
      oncreate={handleClientCreated}
      oncancel={() => showAddClient = false}
    />
  </Modal>
{/if}

<!-- Confirm Finalize -->
{#if showConfirmFinalize}
  <ConfirmDialog
    message={t('invoices.confirmFinalize')}
    onconfirm={doSaveAndFinalize}
    oncancel={() => showConfirmFinalize = false}
  />
{/if}

<style>
  .editor-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .editor-title {
    font-size: var(--font-size-xl);
    font-weight: 700;
    margin-block-start: var(--space-xs);
  }

  .editor-actions {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .editor-grid {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: var(--space-md);
    align-items: start;
  }

  .editor-main {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  /* Client selector */
  .client-selector {
    position: relative;
  }

  .client-dropdown {
    position: absolute;
    inset-block-start: calc(100% + 2px);
    inset-inline-start: 0;
    inset-inline-end: 0;
    background: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    z-index: 400;
    overflow: hidden;
    max-height: 240px;
    overflow-y: auto;
  }

  .client-option {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: var(--space-sm) var(--space-md);
    cursor: pointer;
    width: 100%;
    background: none;
    border: none;
    border-bottom: 1px solid var(--color-border);
    text-align: start;
  }

  .client-option:hover {
    background: var(--color-bg-alt);
  }

  .client-option-name {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text);
  }

  .client-option-email {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .add-client-option {
    color: var(--color-teal-primary);
    font-weight: 500;
    font-size: var(--font-size-sm);
  }

  .btn-add-client {
    font-size: var(--font-size-xs);
    color: var(--color-teal-primary);
    background: none;
    border: none;
    cursor: pointer;
    padding: var(--space-xs) 0;
    display: block;
    margin-block-start: var(--space-xs);
    text-decoration: underline;
  }

  /* Tax mode toggle */
  .tax-mode-toggle {
    display: inline-flex;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .tax-mode-btn {
    padding: var(--space-xs) var(--space-md);
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    background: none;
    border: none;
    color: var(--color-text-muted);
    transition: all 0.15s;
  }

  .tax-mode-btn.active {
    background: var(--color-teal-primary);
    color: white;
  }

  /* Line items */
  .line-items-header {
    padding: var(--space-md);
    border-bottom: 1px solid var(--color-border);
  }

  .line-items-scroll {
    overflow-x: auto;
  }

  .line-items-table th,
  .line-items-table :global(td) {
    border-bottom: 1px solid var(--color-border);
  }

  .line-items-footer {
    padding: var(--space-sm) var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  /* Input with suffix */
  .input-with-suffix {
    display: flex;
    align-items: center;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
    background: var(--color-bg-card);
    transition: border-color 0.15s;
  }

  .input-with-suffix:focus-within {
    border-color: var(--color-teal-primary);
    box-shadow: 0 0 0 3px rgba(32, 128, 141, 0.1);
  }

  .input-with-suffix .input {
    border: none;
    box-shadow: none;
    flex: 1;
  }

  .input-suffix {
    padding: var(--space-sm);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    background: var(--color-bg-alt);
    border-inline-start: 1px solid var(--color-border);
    white-space: nowrap;
  }

  .mt-md {
    margin-block-start: var(--space-md);
  }
</style>
