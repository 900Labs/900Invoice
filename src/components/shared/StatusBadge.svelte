<script lang="ts">
  interface Props {
    status: string;
    size?: 'sm' | 'md';
  }

  let { status, size = 'md' }: Props = $props();

  const statusConfig: Record<string, { label: string; class: string }> = {
    Draft: { label: 'Draft', class: 'badge-draft' },
    Finalized: { label: 'Finalized', class: 'badge-finalized' },
    Sent: { label: 'Sent', class: 'badge-sent' },
    Paid: { label: 'Paid', class: 'badge-paid' },
    Void: { label: 'Void', class: 'badge-void' },
    Overdue: { label: 'Overdue', class: 'badge-overdue' },
    Active: { label: 'Active', class: 'badge-active' },
    Inactive: { label: 'Inactive', class: 'badge-inactive' },
    active: { label: 'Active', class: 'badge-active' },
    inactive: { label: 'Inactive', class: 'badge-inactive' },
    paused: { label: 'Paused', class: 'badge-draft' },
  };

  let config = $derived(statusConfig[status] ?? { label: status, class: 'badge-draft' });
</script>

<span class="badge {config.class}" class:badge-sm={size === 'sm'}>
  {config.label}
</span>

<style>
  .badge-sm {
    font-size: 0.65rem;
    padding: 1px 6px;
  }
</style>
