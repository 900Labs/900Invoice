<script lang="ts">
  import Modal from './Modal.svelte';
  import { t } from '../../stores/i18nStore';

  interface Props {
    title?: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    danger?: boolean;
    onconfirm: () => void;
    oncancel: () => void;
  }

  let {
    title,
    message,
    confirmLabel,
    cancelLabel,
    danger = false,
    onconfirm,
    oncancel,
  }: Props = $props();
</script>

<Modal title={title ?? t('common.confirm')} onclose={oncancel}>
  <p style="font-size: var(--font-size-sm); line-height: 1.6; color: var(--color-text);">
    {message}
  </p>

  {#snippet footer()}
    <button class="btn" onclick={oncancel}>{cancelLabel ?? t('common.cancel')}</button>
    <button
      class="btn {danger ? 'btn-danger' : 'btn-primary'}"
      onclick={onconfirm}
    >
      {confirmLabel ?? t('common.confirm')}
    </button>
  {/snippet}
</Modal>
