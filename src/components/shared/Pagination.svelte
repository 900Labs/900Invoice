<script lang="ts">
  import { t } from '../../stores/i18nStore';

  interface Props {
    total: number;
    page: number;
    pageSize?: number;
    onchange?: (page: number) => void;
  }

  let { total, page, pageSize = 25, onchange }: Props = $props();

  let totalPages = $derived(Math.ceil(total / pageSize));
  let start = $derived((page - 1) * pageSize + 1);
  let end = $derived(Math.min(page * pageSize, total));

  function goTo(p: number) {
    if (p >= 1 && p <= totalPages) {
      onchange?.(p);
    }
  }
</script>

{#if total > 0}
  <div class="pagination">
    <p class="pagination-info">
      {t('common.showing')} {start}–{end} {t('common.of')} {total}
    </p>
    <div class="pagination-controls">
      <button
        class="btn btn-sm"
        onclick={() => goTo(page - 1)}
        disabled={page <= 1}
        aria-label={t('common.previous')}
      >‹</button>

      {#each Array.from({ length: Math.min(totalPages, 7) }, (_, i) => {
        if (totalPages <= 7) return i + 1;
        if (page <= 4) return i + 1;
        if (page >= totalPages - 3) return totalPages - 6 + i;
        return page - 3 + i;
      }) as p}
        <button
          class="btn btn-sm"
          class:active={p === page}
          onclick={() => goTo(p)}
        >{p}</button>
      {/each}

      <button
        class="btn btn-sm"
        onclick={() => goTo(page + 1)}
        disabled={page >= totalPages}
        aria-label={t('common.next')}
      >›</button>
    </div>
  </div>
{/if}

<style>
  .pagination {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    border-top: 1px solid var(--color-border);
    background: var(--color-bg-card);
  }

  .pagination-info {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .pagination-controls {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .btn.active {
    background: var(--color-teal-primary);
    color: white;
    border-color: var(--color-teal-primary);
  }
</style>
