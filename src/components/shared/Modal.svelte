<script lang="ts">
  import { t } from '../../stores/i18nStore';

  interface Props {
    title: string;
    maxWidth?: string;
    onclose?: () => void;
    children?: import('svelte').Snippet;
    footer?: import('svelte').Snippet;
  }

  let { title, maxWidth = '560px', onclose, children, footer }: Props = $props();

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onclose?.();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose?.();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={handleBackdropClick} role="dialog" aria-modal="true" aria-label={title} tabindex="-1">
  <div class="modal-container" style="max-width: {maxWidth}">
    <div class="modal-header">
      <h2 class="modal-title">{title}</h2>
      {#if onclose}
        <button class="btn btn-ghost btn-icon" onclick={onclose} aria-label={t('common.close')}>✕</button>
      {/if}
    </div>
    <div class="modal-body">
      {@render children?.()}
    </div>
    {#if footer}
      <div class="modal-footer">
        {@render footer?.()}
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-title {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--color-text);
  }
</style>
