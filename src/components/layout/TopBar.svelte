<script lang="ts">
  import { getCurrentView, canGoBack, navigateBack } from '../../stores/navigationStore';
  import { t, setLocale, getCurrentLocale, SUPPORTED_LOCALES } from '../../stores/i18nStore';
  import { updateSetting } from '../../stores/settingsStore';

  let currentView = $derived(getCurrentView());
  let locale = $derived(getCurrentLocale());
  let showLangMenu = $state(false);

  const viewTitleKeys: Record<string, string> = {
    dashboard: 'dashboard.title',
    invoices: 'invoices.title',
    'invoice-detail': 'invoices.title',
    'invoice-editor': 'invoices.new',
    clients: 'clients.title',
    'client-detail': 'clients.title',
    products: 'products.title',
    recurring: 'recurring.title',
    reports: 'reports.title',
    settings: 'settings.title',
  };

  let title = $derived(t(viewTitleKeys[currentView] ?? 'dashboard.title'));

  async function handleLocaleChange(code: string) {
    await setLocale(code);
    await updateSetting('locale', code);
    showLangMenu = false;
  }

  function getCurrentLocaleName(): string {
    return SUPPORTED_LOCALES.find(l => l.code === locale)?.nativeName ?? 'EN';
  }
</script>

<header class="topbar">
  <div class="topbar-start">
    {#if canGoBack()}
      <button class="btn btn-ghost btn-sm" onclick={navigateBack}>
        ← {t('common.back')}
      </button>
    {/if}
    <h1 class="topbar-title">{title}</h1>
  </div>

  <div class="topbar-end">
    <!-- Language selector -->
    <div class="lang-selector">
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => showLangMenu = !showLangMenu}
      >
        🌐 {getCurrentLocaleName()}
      </button>
      {#if showLangMenu}
        <div class="lang-menu">
          {#each SUPPORTED_LOCALES as loc}
            <button
              class="lang-item"
              class:active={locale === loc.code}
              onclick={() => handleLocaleChange(loc.code)}
            >
              <span class="lang-code">{loc.code.toUpperCase()}</span>
              <span class="lang-native">{loc.nativeName}</span>
            </button>
          {/each}
        </div>
        <button class="lang-overlay" onclick={() => showLangMenu = false} aria-label={t('common.close')}></button>
      {/if}
    </div>
  </div>
</header>

<style>
  .topbar {
    height: var(--topbar-height);
    background: var(--color-bg-card);
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-lg);
    flex-shrink: 0;
    gap: var(--space-md);
  }

  .topbar-start {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    min-width: 0;
  }

  .topbar-title {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--color-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .topbar-end {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    flex-shrink: 0;
  }

  .lang-selector {
    position: relative;
  }

  .lang-menu {
    position: absolute;
    inset-block-start: calc(100% + 4px);
    inset-inline-end: 0;
    background: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    z-index: 600;
    min-width: 180px;
    overflow: hidden;
  }

  .lang-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    font-size: var(--font-size-sm);
    cursor: pointer;
    width: 100%;
    background: none;
    border: none;
    text-align: start;
    color: var(--color-text);
  }

  .lang-item:hover {
    background: var(--color-bg-alt);
  }

  .lang-item.active {
    color: var(--color-teal-primary);
    font-weight: 600;
  }

  .lang-code {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--color-text-muted);
    width: 28px;
    flex-shrink: 0;
  }

  .lang-overlay {
    position: fixed;
    inset: 0;
    z-index: 599;
    background: transparent;
    border: none;
    cursor: default;
  }
</style>
