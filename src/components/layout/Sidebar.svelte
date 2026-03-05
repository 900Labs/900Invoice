<script lang="ts">
  import { navigateTo, getCurrentView } from '../../stores/navigationStore';
  import { t } from '../../stores/i18nStore';

  let currentView = $derived(getCurrentView());

  interface NavItem {
    id: string;
    icon: string;
    labelKey: string;
  }

  const navItems: NavItem[] = [
    { id: 'dashboard', icon: '📊', labelKey: 'nav.dashboard' },
    { id: 'invoices', icon: '📄', labelKey: 'nav.invoices' },
    { id: 'clients', icon: '👥', labelKey: 'nav.clients' },
    { id: 'products', icon: '📦', labelKey: 'nav.products' },
    { id: 'recurring', icon: '🔄', labelKey: 'nav.recurring' },
    { id: 'reports', icon: '📈', labelKey: 'nav.reports' },
    { id: 'settings', icon: '⚙️', labelKey: 'nav.settings' },
  ];

  function isActive(id: string): boolean {
    if (id === 'invoices') {
      return ['invoices', 'invoice-detail', 'invoice-editor'].includes(currentView);
    }
    if (id === 'clients') {
      return ['clients', 'client-detail'].includes(currentView);
    }
    return currentView === id;
  }
</script>

<aside class="sidebar">
  <div class="sidebar-brand">
    <span class="brand-900">900</span><span class="brand-name">Invoice</span>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={isActive(item.id)}
        onclick={() => navigateTo(item.id)}
        aria-current={isActive(item.id) ? 'page' : undefined}
      >
        <span class="nav-icon" aria-hidden="true">{item.icon}</span>
        <span class="nav-label">{t(item.labelKey)}</span>
      </button>
    {/each}
  </nav>

  <div class="sidebar-footer">
    <p class="sidebar-version">v1.0.0</p>
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    background: var(--color-teal-dark);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .sidebar-brand {
    padding: var(--space-lg) var(--space-md);
    padding-block-end: var(--space-md);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    margin-block-end: var(--space-sm);
    font-size: 1.25rem;
    font-weight: 700;
    letter-spacing: -0.02em;
  }

  .brand-900 {
    color: var(--color-teal-primary);
    font-weight: 800;
  }

  .brand-name {
    color: #FFFFFF;
    font-weight: 400;
  }

  .sidebar-nav {
    flex: 1;
    padding: var(--space-xs) var(--space-sm);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    cursor: pointer;
    border: none;
    background: transparent;
    color: rgba(255, 255, 255, 0.65);
    font-size: var(--font-size-sm);
    font-weight: 500;
    transition: all 0.15s ease;
    text-align: start;
    width: 100%;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.08);
    color: white;
  }

  .nav-item.active {
    background: var(--color-teal-primary);
    color: white;
  }

  .nav-icon {
    font-size: 1rem;
    flex-shrink: 0;
    width: 20px;
    text-align: center;
  }

  .nav-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sidebar-footer {
    padding: var(--space-md);
    border-top: 1px solid rgba(255, 255, 255, 0.08);
  }

  .sidebar-version {
    font-size: var(--font-size-xs);
    color: rgba(255, 255, 255, 0.3);
    text-align: center;
  }
</style>
