<script lang="ts">
  import SearchBar from '../shared/SearchBar.svelte';
  import EmptyState from '../shared/EmptyState.svelte';
  import Modal from '../shared/Modal.svelte';
  import ProductForm from './ProductForm.svelte';
  import ConfirmDialog from '../shared/ConfirmDialog.svelte';
  import StatusBadge from '../shared/StatusBadge.svelte';
  import LoadingSpinner from '../shared/LoadingSpinner.svelte';
  import {
    getFilteredProducts, getLoading, searchProducts, createProduct,
    updateProduct, deleteProduct
  } from '../../stores/productStore';
  import { formatCurrency, formatTaxRate } from '../../utils/currency';
  import { getActiveTaxRates } from '../../stores/taxStore';
  import { t } from '../../stores/i18nStore';
  import { success } from '../../stores/toastStore';
  import type { Product, CreateProduct } from '../../stores/productStore';

  let products = $derived(getFilteredProducts());
  let loading = $derived(getLoading());
  let taxRates = $derived(getActiveTaxRates());

  let showAddModal = $state(false);
  let editingProduct = $state<Product | null>(null);
  let deletingProduct = $state<Product | null>(null);

  function getTaxRateName(id: string | null): string {
    if (!id) return '—';
    const rate = taxRates.find(r => r.id === id);
    return rate ? `${rate.displayName} (${formatTaxRate(rate.rateBps)})` : '—';
  }

  async function handleCreate(data: CreateProduct) {
    await createProduct(data);
    showAddModal = false;
    success(t('common.success'));
  }

  async function handleUpdate(data: Partial<CreateProduct>) {
    if (!editingProduct) return;
    await updateProduct(editingProduct.id, data);
    editingProduct = null;
    success(t('common.success'));
  }

  async function handleDelete() {
    if (!deletingProduct) return;
    await deleteProduct(deletingProduct.id);
    deletingProduct = null;
    success(t('common.success'));
  }

  async function toggleActive(product: Product) {
    await updateProduct(product.id, { isActive: !product.isActive });
  }
</script>

<div class="view-container">
  <div class="view-toolbar">
    <SearchBar placeholder={t('products.search')} onsearch={searchProducts} />
    <div style="flex: 1;"></div>
    <button class="btn btn-primary" onclick={() => showAddModal = true}>
      + {t('products.new')}
    </button>
  </div>

  <div class="card" style="padding: 0; overflow: hidden;">
    {#if loading}
      <div style="display: flex; justify-content: center; padding: 48px;">
        <LoadingSpinner size="lg" />
      </div>
    {:else if products.length === 0}
      <EmptyState
        icon="📦"
        title={t('products.noProducts')}
        description={t('products.addFirst')}
      >
        {#snippet action()}
          <button class="btn btn-primary" onclick={() => showAddModal = true}>
            + {t('products.new')}
          </button>
        {/snippet}
      </EmptyState>
    {:else}
      <table class="table">
        <thead>
          <tr>
            <th>{t('products.name')}</th>
            <th>{t('products.description')}</th>
            <th style="text-align: end;">{t('products.defaultPrice')}</th>
            <th>{t('products.currency')}</th>
            <th>{t('products.taxRate')}</th>
            <th>{t('products.unit')}</th>
            <th>{t('common.status')}</th>
            <th>{t('common.actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each products as product}
            <tr>
              <td><span class="font-medium">{product.name}</span></td>
              <td class="text-muted text-sm truncate" style="max-width: 200px;">{product.description || '—'}</td>
              <td style="text-align: end;" class="currency">
                {formatCurrency(product.defaultPriceMinor, product.currencyCode)}
              </td>
              <td>{product.currencyCode}</td>
              <td>{getTaxRateName(product.taxRateId)}</td>
              <td>{product.unit}</td>
              <td>
                <button
                  class="btn btn-sm btn-ghost"
                  onclick={() => toggleActive(product)}
                >
                  <StatusBadge status={product.isActive ? 'active' : 'inactive'} />
                </button>
              </td>
              <td>
                <div style="display: flex; gap: var(--space-xs);">
                  <button
                    class="btn btn-ghost btn-sm"
                    onclick={() => editingProduct = product}
                  >
                    {t('common.edit')}
                  </button>
                  <button
                    class="btn btn-ghost btn-sm"
                    style="color: var(--color-danger);"
                    onclick={() => deletingProduct = product}
                  >
                    {t('common.delete')}
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

{#if showAddModal}
  <Modal title={t('products.new')} onclose={() => showAddModal = false} maxWidth="560px">
    <ProductForm oncreate={handleCreate} oncancel={() => showAddModal = false} />
  </Modal>
{/if}

{#if editingProduct}
  <Modal title={t('products.edit')} onclose={() => editingProduct = null} maxWidth="560px">
    <ProductForm
      product={editingProduct}
      onupdate={handleUpdate}
      oncancel={() => editingProduct = null}
    />
  </Modal>
{/if}

{#if deletingProduct}
  <ConfirmDialog
    message={t('products.confirmDelete')}
    danger={true}
    onconfirm={handleDelete}
    oncancel={() => deletingProduct = null}
  />
{/if}
