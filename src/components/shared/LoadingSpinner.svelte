<script lang="ts">
  import { t } from '../../stores/i18nStore';

  interface Props {
    size?: 'sm' | 'md' | 'lg';
    color?: string;
  }

  let { size = 'md', color }: Props = $props();

  const sizes = {
    sm: '16px',
    md: '24px',
    lg: '40px',
  };

  let sz = $derived(sizes[size]);
</script>

<span
  class="spinner"
  style:width={sz}
  style:height={sz}
  style:border-top-color={color ?? 'var(--color-teal-primary)'}
  role="status"
  aria-label={t('common.loading')}
></span>
