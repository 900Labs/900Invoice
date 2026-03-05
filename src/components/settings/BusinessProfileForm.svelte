<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import { getBusinessProfile, updateBusinessProfile } from '../../stores/settingsStore';
  import { success, error as toastError } from '../../stores/toastStore';

  let profile = $derived(getBusinessProfile());

  let companyName = $state('');
  let address = $state('');
  let city = $state('');
  let country = $state('');
  let phone = $state('');
  let email = $state('');
  let website = $state('');
  let taxId = $state('');
  let bankName = $state('');
  let bankAccount = $state('');
  let bankRouting = $state('');
  let mobileMoney = $state('');
  let mobileMoneyProvider = $state('');
  let saving = $state(false);

  // Keep in sync with profile when it loads
  $effect(() => {
    const p = profile;
    companyName = p.companyName;
    address = p.address;
    city = p.city;
    country = p.country;
    phone = p.phone;
    email = p.email;
    website = p.website;
    taxId = p.taxId;
    bankName = p.bankName;
    bankAccount = p.bankAccount;
    bankRouting = p.bankRouting;
    mobileMoney = p.mobileMoney;
    mobileMoneyProvider = p.mobileMoneyProvider;
  });

  async function handleSave() {
    saving = true;
    try {
      await updateBusinessProfile({
        companyName, address, city, country, phone, email, website, taxId,
        bankName, bankAccount, bankRouting, mobileMoney, mobileMoneyProvider,
      });
      success(t('settings.saved'));
    } catch (e) {
      toastError(t('common.error'));
    } finally {
      saving = false;
    }
  }
</script>

<form onsubmit={(e) => { e.preventDefault(); handleSave(); }}>
  <h4 class="section-header-sm">{t('settings.companyName')}</h4>

  <div class="form-row">
    <div class="form-group">
      <label class="form-label" for="lbl-settings-companyname">{t('settings.companyName')}</label>
      <input id="lbl-settings-companyname" class="input" type="text" bind:value={companyName} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-taxid">{t('settings.taxId')}</label>
      <input id="lbl-settings-taxid" class="input" type="text" bind:value={taxId} />
    </div>
  </div>

  <div class="form-group mt-md">
    <label class="form-label" for="lbl-settings-address">{t('settings.address')}</label>
    <input id="lbl-settings-address" class="input" type="text" bind:value={address} />
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-settings-city">{t('settings.city')}</label>
      <input id="lbl-settings-city" class="input" type="text" bind:value={city} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-country">{t('settings.country')}</label>
      <input id="lbl-settings-country" class="input" type="text" bind:value={country} />
    </div>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-settings-phone">{t('settings.phone')}</label>
      <input id="lbl-settings-phone" class="input" type="tel" bind:value={phone} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-email">{t('settings.email')}</label>
      <input id="lbl-settings-email" class="input" type="email" bind:value={email} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-website">{t('settings.website')}</label>
      <input id="lbl-settings-website" class="input" type="url" bind:value={website} />
    </div>
  </div>

  <h4 class="section-header-sm" style="margin-block-start: var(--space-xl);">{t('settings.bankingInfo')}</h4>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-settings-bankname">{t('settings.bankName')}</label>
      <input id="lbl-settings-bankname" class="input" type="text" bind:value={bankName} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-bankaccount">{t('settings.bankAccount')}</label>
      <input id="lbl-settings-bankaccount" class="input" type="text" bind:value={bankAccount} />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-bankrouting">{t('settings.bankRouting')}</label>
      <input id="lbl-settings-bankrouting" class="input" type="text" bind:value={bankRouting} />
    </div>
  </div>

  <div class="form-row mt-md">
    <div class="form-group">
      <label class="form-label" for="lbl-settings-mobilemoneyprovider">{t('settings.mobileMoneyProvider')}</label>
      <input id="lbl-settings-mobilemoneyprovider" class="input" type="text" bind:value={mobileMoneyProvider} placeholder="M-Pesa, MTN, Airtel..." />
    </div>
    <div class="form-group">
      <label class="form-label" for="lbl-settings-mobilemoney">{t('settings.mobileMoney')}</label>
      <input id="lbl-settings-mobilemoney" class="input" type="text" bind:value={mobileMoney} />
    </div>
  </div>

  <div class="form-actions mt-lg">
    <button type="submit" class="btn btn-primary" disabled={saving}>
      {saving ? t('common.loading') : t('common.save')}
    </button>
  </div>
</form>

<style>
  .section-header-sm {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-block-end: var(--space-md);
    padding-block-end: var(--space-sm);
    border-bottom: 1px solid var(--color-border);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
  }

  .mt-md { margin-block-start: var(--space-md); }
  .mt-lg { margin-block-start: var(--space-lg); }
</style>
