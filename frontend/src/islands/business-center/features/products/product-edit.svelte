<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index";
  import * as AlertDialog from "$lib/components/ui/alert-dialog/index";
  import { Badge } from "@/lib/components/ui/badge";
  import Column from "../../lib/components/layout/column.svelte";
  import Group from "../../lib/components/layout/group.svelte";
  import SectionHeader from "../../lib/components/section-header.svelte";
  import ActionButton from "../../lib/components/action-button.svelte";
  import {
    ArchiveIcon,
    PauseIcon,
    RefreshCwIcon,
    SaveIcon,
    SendIcon,
    PackageIcon,
    TrashIcon,
    StarIcon,
    CopyIcon,
  } from "@lucide/svelte";

  import {
    canBeDeleted,
    ProductSchema,
    generateSlug,
    useProductQuery,
    useProductUpdate,
    useProductDelete,
  } from "./service";
  import { useNavigate } from "@dvcol/svelte-simple-router/router";
  import { useState } from "../../lib/utils/utils.svelte";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { createForm, getChangedValues } from "../../lib/utils/form";
  import { toast } from "svelte-sonner";
  import type { ProductUpdate } from "@bindings/ProductUpdate";
  import { snakeToTitleCase } from "../../lib/utils/fmt";
  import ProductFormGeneral from "./product-form-general.svelte";
  import ProductFormVariants from "./product-form-variants.svelte";
  import ProductFormMedia from "./product-form-media.svelte";
  import { dialog } from "../../lib/components/alert-dialog.svelte";
  import { Routes } from ".";
  import LoadingSpinner from "../../lib/components/loading-spinner.svelte";
  import LoadingError from "../../lib/components/loading-error.svelte";
  import { Button } from "@/lib/components/ui/button";

  const { replace } = useNavigate();

  const { location } = useRoute();
  const productId: string = location?.params.id as string;

  const query = useProductQuery(productId);

  const updateMutation = useProductUpdate(
    productId,
    () => {
      toast.success("Product updated successfully");
      query.refetch();
    },
    (error) => {
      toast.error("Error updating product", {
        description: error.message,
      });
    },
  );

  const deleteMutation = useProductDelete(
    productId,
    () => {
      toast.success("Product deleted successfully");
      replace({
        path: Routes.LIST_PAGE.path,
      });
    },
    (error) => {
      toast.error("Error deleting product", {
        description: error.message,
      });
    },
  );

  let { form, data } = $derived(
    useState(createForm(ProductSchema, query.data)),
  );

  type ProductAction =
    | "save"
    | "publish"
    | "unpublish"
    | "archive"
    | "restore"
    | "delete"
    | "duplicate"
    | "toggle_featured";

  async function handleAction(action: ProductAction) {
    try {
      let values = getChangedValues(form);

      if (action === "delete") {
        const confirmed = await dialog.confirm({
          title: "Delete this product?",
          description:
            "This action cannot be undone. All product data and variants will be permanently removed.",
          actions: [
            { label: "Cancel", value: false },
            { label: "Delete", value: true, variant: "destructive" },
          ],
        });
        if (!confirmed) return;

        deleteMutation.mutate();
        return;
      }

      if (action === "duplicate") {
        const confirmed = await dialog.confirm({
          title: "Duplicate this product?",
          description: "This will create a new product with the same details.",
          actions: [
            { label: "Cancel", value: false, variant: "secondary" },
            { label: "Duplicate", value: true },
          ],
        });
        if (!confirmed) return;

        // TODO: Implement duplication logic
        toast.info("Duplication feature coming soon");
        return;
      }

      if (action === "archive") {
        const confirmed = await dialog.confirm({
          title: "Archive this product?",
          description:
            "Archived products will not be visible to customers and cannot be purchased.",
          actions: [
            { label: "Cancel", value: false },
            { label: "Archive", value: true, variant: "destructive" },
          ],
        });
        if (!confirmed) return;
      }

      if (action === "unpublish") {
        const confirmed = await dialog.confirm({
          title: "Unpublish this product?",
          description:
            "Customers will not be able to view or purchase this product until you publish it again.",
          actions: [
            { label: "Cancel", value: false },
            { label: "Unpublish", value: true, variant: "destructive" },
          ],
        });
        if (!confirmed) return;
      }

      let updateReq: Partial<ProductUpdate> = values ?? {};

      switch (action) {
        case "save":
          // Auto-generate slug if title changed and slug is empty
          if (values?.title && !form.slug.value) {
            updateReq.slug = generateSlug(values.title);
          }
          break;
        case "publish":
          updateReq.status = "active";
          break;
        case "unpublish":
          updateReq.status = "inactive";
          break;
        case "archive":
          updateReq.status = "archived";
          break;
        case "restore":
          updateReq.status = "inactive";
          break;
        case "toggle_featured":
          updateReq.featured = !form.featured.value;
          break;
      }

      console.info("Updating product with values:", updateReq);

      if (Object.keys(updateReq).length > 0) {
        updateMutation.mutate(updateReq as ProductUpdate);
      } else {
        toast.warning("No changes have been made");
      }
    } catch (e) {
      console.error("Validation Error", e);
      const errors = e as [string, string[]][];
      errors.forEach(([field, errors]) => {
        toast.error(`Error in field: ${snakeToTitleCase(field)}`, {
          description: errors.join("\n"),
        });
      });
    }
  }

  function getTotalStock() {
    if (!form.variants.value) return 0;
    return form.variants.value.reduce(
      (total, variant) => total + (variant.stocks || 0),
      0,
    );
  }

  function getLowestPrice() {
    if (!form.variants.value?.length) return "0.00";
    const prices = form.variants.value
      .map((v) => parseFloat(v.price || "0"))
      .filter((p) => !isNaN(p));
    return prices.length ? Math.min(...prices).toFixed(2) : "0.00";
  }
</script>

<div>
  {#if query.isLoading}
    <div class="h-56 flex items-center justify-center">
      <LoadingSpinner text="Loading..." />
    </div>
  {:else if query.isError}
    <LoadingError message={query.error.message}>
      <Button onclick={single(() => query.refetch())}>Retry</Button>
    </LoadingError>
  {:else if query.isSuccess}
    {@render body()}
  {/if}
</div>

{#snippet body()}
  <Column class="[&>*]:w-full items-center">
    <Group>
      <div class="flex items-center gap-4">
        <SectionHeader
          icon={PackageIcon}
          title={form.title.initialValue}
          description="Manage product details and variants"
        >
          <div class="flex items-center gap-2">
            <Badge
              variant={form.status.initialValue === "active"
                ? "default"
                : form.status.initialValue === "inactive"
                  ? "outline"
                  : "destructive"}
            >
              {snakeToTitleCase(form.status.initialValue)}
            </Badge>
            {#if form.featured.initialValue}
              <Badge
                variant="outline"
                class="text-yellow-600 border-yellow-300"
              >
                <StarIcon class="w-3 h-3 mr-1" />
                Featured
              </Badge>
            {/if}
          </div>
        </SectionHeader>

        <!-- <div class="text-sm text-muted-foreground space-y-1">
          <div>Stock: {getTotalStock()} units</div>
          <div>Price: from ${getLowestPrice()}</div>
          <div>Variants: {form.variants.value?.length || 0}</div>
        </div> -->
      </div>

      <Group class="md:flex-row-reverse flex-wrap justify-start">
        {#if form.status.initialValue === "archived"}
          <ActionButton
            variant="default"
            onclick={single(() => handleAction("restore"))}
          >
            <RefreshCwIcon />
            Restore Product
          </ActionButton>
        {:else}
          <ActionButton
            variant="default"
            onclick={single(() => handleAction("save"))}
          >
            <SaveIcon />
            Save Changes
          </ActionButton>
        {/if}

        {#if form.status.initialValue === "inactive"}
          <ActionButton
            variant="secondary"
            onclick={single(() => handleAction("publish"))}
          >
            <SendIcon />
            Publish Product
          </ActionButton>
        {:else if form.status.initialValue === "active"}
          <ActionButton
            variant="destructive"
            onclick={single(() => handleAction("unpublish"))}
          >
            <PauseIcon />
            Unpublish Product
          </ActionButton>
        {:else if form.status.initialValue === "archived"}
          <ActionButton
            variant="destructive"
            disabled={!canBeDeleted(data)}
            onclick={single(() => handleAction("delete"))}
          >
            <TrashIcon />
            Delete Product
          </ActionButton>
        {/if}

        <ActionButton
          variant="outline"
          onclick={single(() => handleAction("toggle_featured"))}
        >
          <StarIcon />
          {form.featured.value ? "Remove from Featured" : "Mark as Featured"}
        </ActionButton>

        <ActionButton
          variant="outline"
          onclick={single(() => handleAction("duplicate"))}
        >
          <CopyIcon />
          Duplicate
        </ActionButton>

        {#if form.status.initialValue === "inactive"}
          <ActionButton
            variant="destructive"
            onclick={single(() => handleAction("archive"))}
          >
            <ArchiveIcon />
            Archive Product
          </ActionButton>
        {/if}
      </Group>
    </Group>

    <Group class="max-w-4xl md:flex-col md:[&>*]:w-full lg:flex-row">
      <div class="flex-1">
        <Tabs.Root value="general">
          <Tabs.List class="w-full">
            <Tabs.Trigger value="general">General</Tabs.Trigger>
            <Tabs.Trigger value="media">Media</Tabs.Trigger>
            <Tabs.Trigger value="variants">Variants</Tabs.Trigger>
          </Tabs.List>
          <fieldset>
            <Tabs.Content value="general" class="tab-content">
              <ProductFormGeneral bind:form />
            </Tabs.Content>
            <Tabs.Content value="media" class="tab-content">
              <ProductFormMedia bind:form />
            </Tabs.Content>
            <Tabs.Content value="variants" class="tab-content">
              <ProductFormVariants bind:form />
            </Tabs.Content>
          </fieldset>
        </Tabs.Root>
      </div>
    </Group>
  </Column>
{/snippet}
