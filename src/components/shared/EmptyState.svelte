<script lang="ts">
  interface Props {
    icon?: string;
    title: string;
    description?: string;
    action?: import('svelte').Snippet;
  }

  let { icon = '📋', title, description, action }: Props = $props();
</script>

<div class="empty-state">
  <span class="empty-state-icon" aria-hidden="true">{icon}</span>
  <h3 class="empty-state-title">{title}</h3>
  {#if description}
    <p class="empty-state-body">{description}</p>
  {/if}
  {#if action}
    <div class="empty-state-action">
      {@render action()}
    </div>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-2xl) var(--space-lg);
    text-align: center;
    gap: var(--space-sm);
  }

  .empty-state-icon {
    font-size: 3rem;
    opacity: 0.35;
    margin-block-end: var(--space-xs);
  }

  .empty-state-title {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--color-text-secondary);
  }

  .empty-state-body {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    max-width: 320px;
    line-height: 1.6;
  }

  .empty-state-action {
    margin-block-start: var(--space-sm);
  }
</style>
