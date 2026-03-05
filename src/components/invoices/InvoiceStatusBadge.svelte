<script lang="ts">
  import { t } from '../../stores/i18nStore';
  import type { InvoiceStatus } from '../../stores/invoiceStore';

  interface Props {
    status: InvoiceStatus;
  }

  let { status }: Props = $props();

  const statusMap: Record<InvoiceStatus, { labelKey: string; cls: string }> = {
    Draft: { labelKey: 'invoices.filterDraft', cls: 'badge-draft' },
    Finalized: { labelKey: 'invoices.filterFinalized', cls: 'badge-finalized' },
    Sent: { labelKey: 'invoices.filterSent', cls: 'badge-sent' },
    Paid: { labelKey: 'invoices.filterPaid', cls: 'badge-paid' },
    Void: { labelKey: 'invoices.filterVoid', cls: 'badge-void' },
  };

  let conf = $derived(statusMap[status] ?? { labelKey: status, cls: 'badge-draft' });
</script>

<span class="badge {conf.cls}">{t(conf.labelKey)}</span>
