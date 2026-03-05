<script lang="ts">
  interface Props {
    label: string;
    value: string;
    icon?: string;
    color?: string;
    sublabel?: string;
  }

  let { label, value, icon = '📊', color = 'var(--color-teal-primary)', sublabel }: Props = $props();
</script>

<div class="stat-card card">
  <div class="stat-header">
    <span class="stat-label">{label}</span>
    <span class="stat-icon" aria-hidden="true">{icon}</span>
  </div>
  <p class="stat-value" style:color={color}>{value}</p>
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

  .stat-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-block-start: var(--space-xs);
  }
</style>
