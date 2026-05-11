<script lang="ts">
  interface Detail {
    label: string;
    value: string;
  }

  interface Props {
    label: string;
    value?: string;
    icon?: string;
    color?: string;
    sublabel?: string;
    details?: Detail[];
  }

  let {
    label,
    value = '',
    icon = '📊',
    color = 'var(--color-teal-primary)',
    sublabel,
    details = [],
  }: Props = $props();
</script>

<div class="stat-card card">
  <div class="stat-header">
    <span class="stat-label">{label}</span>
    <span class="stat-icon" aria-hidden="true">{icon}</span>
  </div>
  {#if details.length > 0}
    <div class="stat-values">
      {#each details as detail}
        <div class="stat-value-row">
          <span class="stat-value stat-value-compact" style:color={color}>{detail.value}</span>
          <span class="stat-currency">{detail.label}</span>
        </div>
      {/each}
    </div>
  {:else}
    <p class="stat-value" style:color={color}>{value}</p>
  {/if}
  {#if sublabel}
    <p class="stat-sub">{sublabel}</p>
  {/if}
</div>

<style>
  .stat-card {
    padding: var(--space-md);
  }

  .stat-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-block-end: var(--space-sm);
  }

  .stat-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
  }

  .stat-icon {
    font-size: 1.1rem;
    opacity: 0.6;
  }

  .stat-value {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    line-height: 1.2;
  }

  .stat-values {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .stat-value-row {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-sm);
    min-width: 0;
  }

  .stat-value-compact {
    font-size: var(--font-size-lg);
    overflow-wrap: anywhere;
  }

  .stat-currency {
    flex: 0 0 auto;
    font-size: var(--font-size-xs);
    font-weight: 700;
    color: var(--color-text-muted);
    font-variant-numeric: tabular-nums;
  }

  .stat-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-block-start: var(--space-xs);
  }
</style>
