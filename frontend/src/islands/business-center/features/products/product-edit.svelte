<!-- ProductEditPage.svelte -->
<script lang="ts">
  import { useRoute, link, useLink } from "@dvcol/svelte-simple-router";
  import { Routes } from "./index";

  import Column from "../../lib/components/layout/column.svelte";
  import Group from "../../lib/components/layout/group.svelte";
  import SectionHeader from "../../lib/components/section-header.svelte";
  import ActionButton from "../../lib/components/action-button.svelte";
  import { Button } from "$lib/components/ui/button/index";
  import { Badge } from "$lib/components/ui/badge/index";
  import { Input } from "$lib/components/ui/input/index";
  import { Label } from "$lib/components/ui/label/index";
  import { Textarea } from "$lib/components/ui/textarea/index";
  import { Switch } from "$lib/components/ui/switch/index";
  import * as Card from "$lib/components/ui/card/index";
  import * as Tabs from "$lib/components/ui/tabs/index";
  import * as AlertDialog from "$lib/components/ui/alert-dialog/index";
  import {
    PackageIcon,
    SaveIcon,
    TrashIcon,
    ArrowLeftIcon,
    ImageIcon,
    PlusIcon,
    StarIcon,
    MoveUpIcon,
  } from "@lucide/svelte";

  import type { ProductDto } from "@bindings/ProductDto";
  import type { ProductUpdate } from "@bindings/ProductUpdate";
  import Editor from "./editor.svelte";
  import { currencyFormatter } from "../../lib/utils/fmt";
  import { MoveDownIcon } from "lucide-svelte";
  import { get_product } from "@bindings/ProductRoutes";

  const { location } = useRoute();
  const { id } = location?.params as { id: string };

  let product = $state({} as ProductDto);
  let formData = $state({} as ProductUpdate);
  let hasChanges = $state(false);
  let isSaving = $state(false);
  let isDeleting = $state(false);
  let showDeleteDialog = $state(false);

  async function loadProduct() {
    try {
      product = await get_product(id);
      formData = product;
      return product;
    } catch (error) {
      throw error;
    }
  }

  async function saveProduct() {
    try {
      isSaving = true;
      product = await productEditService.updateProduct(id, formData);
      formData = productEditService.createFormData(product);
      hasChanges = false;
    } catch (error) {
      console.error("Save failed:", error);
    } finally {
      isSaving = false;
    }
  }

  async function deleteProduct() {
    try {
      isDeleting = true;
      await productEditService.deleteProduct(id);
      window.location.href = Routes.LIST_PAGE.path;
    } catch (error) {
      console.error("Delete failed:", error);
    } finally {
      isDeleting = false;
      showDeleteDialog = false;
    }
  }

  function handleChange() {
    hasChanges = true;
  }

  // Image management
  function addImage() {
    const imageUrl = prompt("Enter image URL:");
    if (imageUrl) {
      formData.images = [...(formData.images || []), imageUrl];
      handleChange();
    }
  }

  function removeImage(index: number) {
    if (!formData.images) return;
    formData.images = formData.images.filter((_, i) => i !== index);
    handleChange();
  }

  function moveImageUp(index: number) {
    if (!formData.images || index === 0) return;
    const imageToMove = formData.images[index];
    formData.images[index] = formData.images[index - 1];
    formData.images[index - 1] = imageToMove;
    handleChange();
  }

  function moveImageDown(index: number) {
    if (!formData.images || index + 1 >= formData.images.length) return;
    const imageToMove = formData.images[index];
    formData.images[index] = formData.images[index + 1];
    formData.images[index + 1] = imageToMove;
    handleChange();
  }
</script>

{#await loadProduct()}
  <div class="flex items-center justify-center min-h-[400px]">
    <div class="text-center">
      <div
        class="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent mx-auto mb-4"
      ></div>
      <p class="text-muted-foreground">Loading product...</p>
    </div>
  </div>
{:then}
  <Column class="[&>*]:w-full items-center">
    <Group>
      <div class="flex items-center gap-4">
        <SectionHeader
          icon={PackageIcon}
          title={product.title}
          description="Manage product details, pricing, and inventory"
        />
      </div>
      <Group>
        <ActionButton
          variant="destructive"
          onclick={() => (showDeleteDialog = true)}
          disabled={isDeleting}
        >
          <TrashIcon />
          Archive
        </ActionButton>
        <ActionButton onclick={saveProduct} disabled={!hasChanges || isSaving}>
          <SaveIcon />
          {isSaving ? "Saving..." : "Save Changes"}
        </ActionButton>
      </Group>
    </Group>

    <Group class="max-w-7xl md:flex-col md:[&>*]:w-full lg:flex-row">
      <div class="flex-1">
        <Tabs.Root value="general">
          <Tabs.List class="w-full">
            <Tabs.Trigger value="general">General</Tabs.Trigger>
            <Tabs.Trigger value="description">Description</Tabs.Trigger>
            <Tabs.Trigger value="images">Images</Tabs.Trigger>
            <Tabs.Trigger value="variants">Variants</Tabs.Trigger>
          </Tabs.List>
          <Tabs.Content value="general" class="space-y-6">
            {@render general()}
          </Tabs.Content>
          <Tabs.Content value="description" class="space-y-6">
            {@render description()}
          </Tabs.Content>
          <Tabs.Content value="images" class="space-y-6">
            {@render images()}
          </Tabs.Content>
          <Tabs.Content value="variants" class="space-y-6">
            {@render variants()}
          </Tabs.Content>
        </Tabs.Root>
      </div>
      <div class="lg:max-w-sm space-y-6">
        <div class="lg:w-xs">
          {@render summary()}
        </div>
      </div>
    </Group>
  </Column>

  <!-- Delete Dialog -->
  <AlertDialog.Root bind:open={showDeleteDialog}>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>Are you sure?</AlertDialog.Title>
        <AlertDialog.Description>
          This action will archive the product "{product.title}". Archived
          products can be restored later, but they won't be visible to
          customers.
        </AlertDialog.Description>
      </AlertDialog.Header>
      <AlertDialog.Footer>
        <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
        <AlertDialog.Action
          onclick={deleteProduct}
          disabled={isDeleting}
          aria-invalid
        >
          Archive Product
        </AlertDialog.Action>
      </AlertDialog.Footer>
    </AlertDialog.Content>
  </AlertDialog.Root>
{:catch error}
  <div class="flex flex-col items-center justify-center py-12">
    <div class="text-center">
      <h3 class="text-lg font-semibold">Failed to load product</h3>
      <p class="text-muted-foreground mt-2">
        {error?.message || "Something went wrong while loading the product."}
      </p>
      <a href={Routes.LIST_PAGE.path} use:link>
        <Button class="mt-4">Back to Products</Button>
      </a>
    </div>
  </div>
{/await}

{#snippet general()}
  <Card.Root>
    <Card.Header>
      <Card.Title>Basic Information</Card.Title>
      <Card.Description
        >Essential product details that customers will see.</Card.Description
      >
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="grid gap-4 md:grid-cols-2">
        <div class="space-y-2">
          <Label for="title">Product Name *</Label>
          <Input
            id="title"
            bind:value={formData.title}
            oninput={handleChange}
            placeholder="Enter product name"
            required
          />
        </div>
        <div class="space-y-2">
          <Label for="category">Category</Label>
          <Input
            id="category"
            bind:value={formData.category}
            oninput={handleChange}
            placeholder="Enter product category"
          />
        </div>
      </div>
      <div class="flex items-center gap-2">
        <Switch
          id="featured"
          bind:checked={formData.featured as boolean}
          onCheckedChange={handleChange}
        />
        <Label for="featured">Featured Product</Label>
      </div>

      <div class="space-y-2">
        <Label for="slug">URL Slug</Label>
        <Input
          id="slug"
          bind:value={formData.slug}
          oninput={handleChange}
          placeholder="product-url-slug"
        />
        <p class="text-sm text-muted-foreground">
          URL: /products/{formData.slug || product.slug}
        </p>
      </div>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet description()}
  <Editor />
{/snippet}

<!-- 
{#snippet pricing()}
    <Card.Root>
        <Card.Header>
            <Card.Title>Pricing</Card.Title>
            <Card.Description
                >Set your product pricing and compare prices.</Card.Description
            >
        </Card.Header>
        <Card.Content class="space-y-4">
            <div class="grid gap-4 md:grid-cols-2">
                <div class="space-y-2">
                    <Label for="base_price">Base Price (DZD) *</Label>
                    <Input
                        id="base_price"
                        type="number"
                        step="0.01"
                        min="0"
                        bind:value={formData.base_price}
                        oninput={handleChange}
                        placeholder="0.00"
                        required
                    />
                </div>
                <div class="space-y-2">
                    <Label for="base_compare_price">Compare Price (DZD)</Label>
                    <Input
                        id="base_compare_price"
                        type="number"
                        step="0.01"
                        min="0"
                        bind:value={formData.base_compare_price}
                        oninput={handleChange}
                        placeholder="0.00"
                    />
                    <p class="text-sm text-muted-foreground">
                        Higher price to show savings
                    </p>
                </div>
            </div>

            {#if formData.base_compare_price && formData.base_price && formData.base_compare_price > formData.base_price}
                <div class="rounded-lg bg-green-50 border border-green-200 p-4">
                    <div class="flex items-center gap-2">
                        <Badge
                            variant="outline"
                            class="bg-green-100 text-green-800 border-green-300"
                        >
                            {Math.round(
                                (1 -
                                    formData.base_price /
                                        formData.base_compare_price) *
                                    100,
                            )}% OFF
                        </Badge>
                        <span class="text-sm text-green-800">
                            Customers save {new Intl.NumberFormat("en-US", {
                                style: "currency",
                                currency: "DZD",
                            }).format(
                                formData.base_compare_price -
                                    formData.base_price,
                            )}
                        </span>
                    </div>
                </div>
            {/if}
        </Card.Content>
    </Card.Root>
{/snippet} -->

{#snippet images()}
  <Card.Root>
    <Card.Header>
      <Card.Title>Product Images</Card.Title>
      <Card.Description>
        Add high-quality images to showcase your product. The first image will
        be used as the thumbnail.
      </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {#each formData.images || [] as image, index}
          <div class="relative group">
            <div
              class="aspect-square rounded-lg bg-muted overflow-hidden border-2 transition-colors {index ===
              0
                ? 'border-primary shadow-md'
                : 'border-border hover:border-primary/50'}"
            >
              <img
                src={image}
                alt="Product image {index + 1}"
                class="w-full h-full object-cover"
              />
            </div>

            {#if index === 0}
              <div class="absolute bottom-2 left-2">
                <Badge
                  variant="default"
                  class="bg-primary text-primary-foreground"
                >
                  <StarIcon class="h-3 w-3 mr-1" />
                  Thumbnail
                </Badge>
              </div>
            {/if}

            <div
              class="absolute top-2 right-2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
            >
              {#if index !== 0}
                <Button
                  variant="secondary"
                  size="icon"
                  class="h-8 w-8 shadow-md"
                  onclick={() => moveImageUp(index)}
                  title="Move up"
                >
                  <MoveUpIcon class="h-4 w-4" />
                </Button>
              {/if}
              {#if index + 1 < (formData.images?.length || 1)}
                <Button
                  variant="secondary"
                  size="icon"
                  class="h-8 w-8 shadow-md"
                  onclick={() => moveImageDown(index)}
                  title="move Down"
                >
                  <MoveDownIcon class="h-4 w-4" />
                </Button>
              {/if}
              <Button
                variant="destructive"
                size="icon"
                class="h-8 w-8 shadow-md"
                onclick={() => removeImage(index)}
                title="Remove image"
              >
                <TrashIcon class="h-4 w-4" />
              </Button>
            </div>
          </div>
        {:else}
          <div
            class="aspect-square rounded-lg bg-muted flex items-center justify-center text-muted-foreground border-2 border-dashed border-border"
          >
            <div class="text-center">
              <ImageIcon class="h-8 w-8 mx-auto mb-2" />
              <p class="text-sm">No images yet</p>
            </div>
          </div>
        {/each}

        <button
          onclick={addImage}
          class="aspect-square rounded-lg border-2 border-dashed border-border hover:border-primary hover:bg-accent/50 transition-colors flex items-center justify-center text-muted-foreground hover:text-primary"
        >
          <div class="text-center">
            <PlusIcon class="h-6 w-6 mx-auto mb-2" />
            <span class="text-sm font-medium">Add Image</span>
          </div>
        </button>
      </div>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet variants()}
  <Card.Root>
    <Card.Header>
      <Card.Title>Product Variants</Card.Title>
      <Card.Description
        >Manage different variations of your product (size, color, etc.)</Card.Description
      >
    </Card.Header>
    <Card.Content>
      <div class="text-center py-12 text-muted-foreground">
        <PackageIcon class="h-12 w-12 mx-auto mb-4 opacity-50" />
        <h3 class="font-medium text-lg mb-2">Variants coming soon</h3>
        <p class="text-sm">
          Advanced variant management will be available in the next update.
        </p>
      </div>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet summary()}
  <Card.Root>
    <Card.Header>
      <Card.Title>Quick Stats</Card.Title>
    </Card.Header>
    <Card.Content class="space-y-3">
      <div class="flex justify-between text-sm">
        <span class="text-muted-foreground">Variants</span>
        <span class="font-medium">{product.variants.length || 0}</span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-muted-foreground">Price</span>
        <span class="font-medium">
          {currencyFormatter.format(+(product.variants.at(0)?.price || "0"))}
        </span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-muted-foreground">Category</span>
        <Badge variant="outline">{product.category || "None"}</Badge>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-muted-foreground">Featured</span>
        <Badge variant={product.featured ? "default" : "secondary"}>
          {product.featured ? "Yes" : "No"}
        </Badge>
      </div>
    </Card.Content>
  </Card.Root>

  {#if product.images?.length > 0}
    <Card.Root>
      <Card.Header>
        <Card.Title>Product Images</Card.Title>
      </Card.Header>
      <Card.Content>
        <div class="grid grid-cols-2 gap-2">
          {#each product.images.slice(0, 4) as image}
            <div class="aspect-square rounded-md bg-muted overflow-hidden">
              <img
                src={image}
                alt="Product"
                class="w-full h-full object-cover"
              />
            </div>
          {/each}
        </div>
        {#if product.images.length > 4}
          <p class="text-sm text-muted-foreground mt-2">
            +{product.images.length - 4} more images
          </p>
        {/if}
      </Card.Content>
    </Card.Root>
  {/if}
{/snippet}
