<script lang="ts">
  import { t } from '../../stores/i18nStore';

  interface Props {
    value?: string;
    placeholder?: string;
    onsearch?: (value: string) => void;
    class?: string;
  }

  let { value = $bindable(''), placeholder, onsearch, class: klass = '' }: Props = $props();

  let inputEl: HTMLInputElement | undefined = $state();

  function handleInput(e: Event) {
    value = (e.target as HTMLInputElement).value;
    onsearch?.(value);
  }

  function handleClear() {
    value = '';
    onsearch?.('');
    inputEl?.focus();
  }
</script>

<div class="search-bar {klass}">
  <span class="search-icon" aria-hidden="true">🔍</span>
  <input
    bind:this={inputEl}
    class="search-input"
    type="search"
    {value}
    placeholder={placeholder ?? t('common.search')}
    oninput={handleInput}
    aria-label={placeholder ?? t('common.search')}
  />
  {#if value}
    <button class="search-clear" onclick={handleClear} aria-label={t('common.clearSearch')}>✕</button>
  {/if}
</div>

<style>
  .search-bar {
    position: relative;
    display: flex;
    align-items: center;
    min-width: 200px;
  }

  .search-icon {
    position: absolute;
    inset-inline-start: var(--space-sm);
    font-size: 0.875rem;
    pointer-events: none;
    opacity: 0.5;
  }

  .search-input {
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    padding-inline-start: calc(var(--space-md) + 20px);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    background: var(--color-bg-card);
    color: var(--color-text);
    transition: border-color 0.15s ease;
    appearance: none;
  }

  .search-input::-webkit-search-cancel-button {
    display: none;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--color-teal-primary);
    box-shadow: 0 0 0 3px rgba(32, 128, 141, 0.1);
  }

  .search-clear {
    position: absolute;
    inset-inline-end: var(--space-sm);
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.75rem;
    color: var(--color-text-muted);
    padding: 2px;
    line-height: 1;
  }

  .search-clear:hover {
    color: var(--color-text);
  }
</style>
