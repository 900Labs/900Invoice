<script lang="ts">
  import { getToasts, dismissToast } from '../../stores/toastStore';

  let toasts = $derived(getToasts());
</script>

<div class="toast-container" role="region" aria-label="Notifications" aria-live="polite">
  {#each toasts as toast (toast.id)}
    <div class="toast toast-{toast.type}" role="alert">
      <span class="toast-message">{toast.message}</span>
      <button
        class="toast-close"
        onclick={() => dismissToast(toast.id)}
        aria-label="Dismiss"
      >✕</button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: var(--space-lg);
    inset-inline-end: var(--space-lg);
    z-index: 2000;
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    font-size: var(--font-size-sm);
    font-weight: 500;
    pointer-events: all;
    max-width: 380px;
    animation: toast-in 0.2s ease;
  }

  @keyframes toast-in {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .toast-success { background: var(--color-success); color: white; }
  .toast-error { background: var(--color-danger); color: white; }
  .toast-warning { background: var(--color-warning); color: white; }
  .toast-info { background: var(--color-info); color: white; }

  .toast-message {
    flex: 1;
    line-height: 1.4;
  }

  .toast-close {
    background: none;
    border: none;
    color: rgba(255,255,255,0.8);
    cursor: pointer;
    padding: 2px;
    font-size: 0.75rem;
    line-height: 1;
    flex-shrink: 0;
  }

  .toast-close:hover {
    color: white;
  }
</style>
