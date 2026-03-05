<script lang="ts">
  import { t } from '../../stores/i18nStore';

  interface Column {
    key: string;
    label: string;
    align?: 'start' | 'end' | 'center';
    width?: string;
  }

  interface Props {
    columns: Column[];
    rows: Record<string, unknown>[];
    onrowclick?: (row: Record<string, unknown>) => void;
    loading?: boolean;
    emptyMessage?: string;
    children?: import('svelte').Snippet<[{ row: Record<string, unknown>; col: Column }]>;
  }

  let { columns, rows, onrowclick, loading = false, emptyMessage, children }: Props = $props();
</script>

<div class="data-table-wrapper">
  {#if loading}
    <div class="table-loading">
      <div class="spinner"></div>
      <span>{t('common.loading')}</span>
    </div>
  {:else}
    <table class="table">
      <thead>
        <tr>
          {#each columns as col}
            <th style:width={col.width} style:text-align={col.align ?? 'start'}>
              {col.label}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each rows as row}
          <tr
            onclick={() => onrowclick?.(row)}
            class:clickable={!!onrowclick}
          >
            {#each columns as col}
              <td style:text-align={col.align ?? 'start'}>
                {#if children}
                  {@render children({ row, col })}
                {:else}
                  {String(row[col.key] ?? '')}
                {/if}
              </td>
            {/each}
          </tr>
        {:else}
          <tr>
            <td colspan={columns.length} class="table-empty">
              {emptyMessage ?? t('common.noResults')}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .data-table-wrapper {
    width: 100%;
    overflow-x: auto;
  }

  .table-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-2xl);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .table-empty {
    text-align: center;
    color: var(--color-text-muted);
    padding: var(--space-2xl) !important;
    font-size: var(--font-size-sm);
  }

  tr.clickable {
    cursor: pointer;
  }
</style>
