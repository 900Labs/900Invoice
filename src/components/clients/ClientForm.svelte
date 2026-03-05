<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { validateClient } from '../../utils/validation';
  import { getAllCurrencies } from '../../utils/currency';
  import type { Client, CreateClient } from '../../stores/clientStore';

  interface Props {
    client?: Client;
    oncreate?: (data: CreateClient) => void;
    onupdate?: (data: Partial<CreateClient>) => void;
    oncancel?: () => void;
  }

  let { client, oncreate, onupdate, oncancel }: Props = $props();

  let currencies = getAllCurrencies();

  const COUNTRIES = [
    { code: 'KE', name: 'Kenya' }, { code: 'NG', name: 'Nigeria' },
    { code: 'ZA', name: 'South Africa' }, { code: 'IN', name: 'India' },
    { code: 'TZ', name: 'Tanzania' }, { code: 'UG', name: 'Uganda' },
    { code: 'GH', name: 'Ghana' }, { code: 'SN', name: 'Senegal' },
    { code: 'CM', name: 'Cameroon' }, { code: 'US', name: 'United States' },
    { code: 'GB', name: 'United Kingdom' }, { code: 'FR', name: 'France' },
    { code: 'DE', name: 'Germany' }, { code: 'MA', name: 'Morocco' },
    { code: 'EG', name: 'Egypt' }, { code: 'ET', name: 'Ethiopia' },
    { code: 'OTHER', name: 'Other' },
  ];

  // Form fields
  let name = $state('');
  let email = $state('');
  let phone = $state('');
  let address = $state('');
  let city = $state('');
  let country = $state('');
  let countryCode = $state('');
  let taxId = $state('');
  let currencyCode = $state('USD');
  let paymentTermsDays = $state(30);
  let notes = $state('');

  // Sync from prop (edit mode)
  $effect(() => {
    if (client) {
      name = client.name ?? '';
      email = client.email ?? '';
      phone = client.phone ?? '';
      address = client.address ?? '';
      city = client.city ?? '';
      country = client.country ?? '';
      countryCode = client.countryCode ?? '';
      taxId = client.taxId ?? '';
      currencyCode = client.currencyCode ?? 'USD';
      paymentTermsDays = client.paymentTermsDays ?? 30;
      notes = client.notes ?? '';
    }
  });

  let errors = $state<Record<string, string>>({});

  function handleSubmit() {
    errors = {};
    const result = validateClient({ name, email, phone });
    if (!result.valid) {
      errors = result.errors;
      return;
    }

    const data: CreateClient = {
      name, email, phone, address, city, country, countryCode,
      taxId, currencyCode, paymentTermsDays, notes,
    };

    if (client) {
      onupdate?.(data);
    } else {
      oncreate?.(data);
    }
  }
</script>

<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <div class="form-row">
    <div class="form-group">
      <label class="form-label required" for="lbl-clients-name">{t('clients.name')}</label>
      <input id="lbl-clients-name" class="input {errors.name ? 'error' : ''}" type="text" bind:value={name} />
      {#if errors.name}<span class="form-error">{errors.name}</span>{/if}
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-clients-email">{t('clients.email')}</label>
      <input id="lbl-clients-email" class="input {errors.email ? 'error' : ''}" type="email" bind:value={email} />
      {#if errors.email}<span class="form-error">{errors.email}</span>{/if}
    </div>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-clients-phone">{t('clients.phone')}</label>
      <input id="lbl-clients-phone" class="input {errors.phone ? 'error' : ''}" type="tel" bind:value={phone} />
      {#if errors.phone}<span class="form-error">{errors.phone}</span>{/if}
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-clients-taxid">{t('clients.taxId')}</label>
      <input id="lbl-clients-taxid" class="input" type="text" bind:value={taxId} />
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-clients-address">{t('clients.address')}</label>
    <input id="lbl-clients-address" class="input" type="text" bind:value={address} />
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-clients-city">{t('clients.city')}</label>
      <input id="lbl-clients-city" class="input" type="text" bind:value={city} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-clients-country">{t('clients.country')}</label>
      <select id="lbl-clients-country" class="select" bind:value={country}>
        <option value="">{t('common.select')}</option>
        {#each COUNTRIES as c}
          <option value={c.name}>{c.name}</option>
        {/each}
      </select>
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-clients-countrycode">{t('clients.countryCode')}</label>
      <input id="lbl-clients-countrycode" class="input" type="text" bind:value={countryCode} placeholder="KE" maxlength="3" />
    </div>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-clients-currency">{t('clients.currency')}</label>
      <select id="lbl-clients-currency" class="select" bind:value={currencyCode}>
        {#each currencies as c}
          <option value={c.code}>{c.code} — {c.name}</option>
        {/each}
      </select>
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-clients-paymentterms">{t('clients.paymentTerms')}</label>
      <input id="lbl-clients-paymentterms" class="input" type="number" min="0" step="1" bind:value={paymentTermsDays} />
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-clients-notes">{t('clients.notes')}</label>
    <textarea id="lbl-clients-notes" class="textarea" rows="2" bind:value={notes}></textarea>
  </div>

  <div class="form-actions mt-md">
    {#if oncancel}
      <button type="button" class="btn" onclick={oncancel}>{t('common.cancel')}</button>
    {/if}
    <button type="submit" class="btn btn-primary">
      {client ? t('common.save') : t('common.create')}
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
