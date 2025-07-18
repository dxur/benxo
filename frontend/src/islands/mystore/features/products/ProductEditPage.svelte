<script lang="ts">
    import { onMount } from "svelte";
    import { link, useNavigate, useRoute } from "@dvcol/svelte-simple-router";
    import { Routes } from "./index";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { notifCenter } from "@/stores/notifications";

    import type { ProductPublic } from "@bindings/ProductPublic";
    import type { ProductUpdateBody } from "@bindings/ProductUpdateBody";
    import type { ProductVariant } from "@bindings/ProductVariant";

    import EditPageLayout from "../../lib/components/EditPageLayout.svelte";
    import ProductForm from "./ProductForm.svelte";
    import { Button } from "@/lib/components/ui/button/index.js";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import { Skeleton } from "@/lib/components/ui/skeleton/index.js";
    import * as AlertDialog from "@/lib/components/ui/alert-dialog/index.js";

    import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
    import SaveIcon from "@lucide/svelte/icons/save";
    import TrashIcon from "@lucide/svelte/icons/trash";
    import PackageIcon from "@lucide/svelte/icons/package";
    import EyeIcon from "@lucide/svelte/icons/eye";
    import type { LoadingStatus } from "@/components/StatusBoundary.svelte";
    import StatusBoundary from "@/components/StatusBoundary.svelte";
    import { sidebar } from "../orders";

    const { replace } = useNavigate();
    const { location } = useRoute();

    let params = location?.params;

    let product: ProductPublic | null = null;
    let loadingStatus: LoadingStatus = undefined;
    let isSaving = false;
    let isDeleting = false;
    let hasChanges = false;
    let showDeleteDialog = false;
    let currentTab = "general" as const;

    // Form data
    let formData: ProductUpdateBody = {
        name: null,
        description: null,
        featured: null,
        category: null,
        base_price: null,
        base_compare_price: null,
        base_images: null,
        options: null,
        variants: null,
        slug: null,
    };

    onMount(async () => {
        await loadProduct();
    });

    async function loadProduct() {
        try {
            console.log(currentTab);
            loadingStatus = undefined;
            if (!params?.id || typeof params.id !== "string") {
                throw new Error("Missing product ID");
            }
            const response = await ApiRoutes.get_one_product({
                id: params.id,
            });
            product = response;

            // Initialize form with product data
            formData = {
                name: product.name,
                description: product.description,
                featured: product.featured,
                category: product.category,
                base_price: product.base_price,
                base_compare_price: product.base_compare_price,
                base_images: [...product.base_images],
                options: { ...product.options },
                variants: product.variants.map((v) => ({ ...v })),
                slug: product.slug,
            };

            document.title = `Edit ${product.name}`;
            loadingStatus = null;
            console.log(currentTab);
        } catch (error) {
            notifCenter.error("Failed to load product");
            loadingStatus = error ?? new Error("Failed to load product");
        }
    }

    async function handleSave() {
        if (!product) return;

        try {
            isSaving = true;
            await ApiRoutes.update_product({
                id: product.id,
                body: formData,
            });

            notifCenter.success("Product updated successfully");
            hasChanges = false;

            // Reload product to get updated data
            await loadProduct();
        } catch (error) {
            notifCenter.error("Failed to update product");
            console.error("Error updating product:", error);
        } finally {
            isSaving = false;
        }
    }

    async function handleDelete() {
        if (!product) return;

        try {
            isDeleting = true;
            await ApiRoutes.delete_product({ id: product.id });

            notifCenter.success("Product archived successfully");

            // Navigate back to products list
            // You'll need to implement navigation based on your router
            window.location.href = Routes.LIST_PAGE.path;
        } catch (error) {
            notifCenter.error("Failed to archive product");
            console.error("Error archiving product:", error);
        } finally {
            isDeleting = false;
            showDeleteDialog = false;
        }
    }

    function handleFormChange() {
        hasChanges = true;
    }

    // Breadcrumbs
    $: breadcrumbs = [
        { label: "Products", href: Routes.LIST_PAGE.path },
        { label: product?.name || "Loading...", href: "#" },
    ];

    // Header actions
    $: headerActions = [
        {
            label: "Save Changes",
            icon: SaveIcon,
            variant: "default" as const,
            onclick: handleSave,
            disabled: !hasChanges || isSaving,
            loading: isSaving,
        },
        {
            label: "Archive Product",
            icon: TrashIcon,
            variant: "destructive" as const,
            onclick: () => {
                showDeleteDialog = true;
            },
            disabled: isDeleting,
            loading: isDeleting,
        },
    ];

    // Status info
    $: statusInfo = product
        ? {
              label: product.featured ? "Featured" : "Regular",
              variant: product.featured
                  ? ("default" as const)
                  : ("secondary" as const),
              icon: PackageIcon,
          }
        : undefined;
</script>

<svelte:head>
    <title>{product ? `Edit ${product.name}` : "Loading..."}</title>
</svelte:head>

<StatusBoundary status={loadingStatus}>
    <EditPageLayout
        {headerActions}
        {statusInfo}
        title={product?.name || "Loading..."}
        subtitle="Manage product details, pricing, and inventory"
        icon={PackageIcon}
    >
        <!-- Back Button -->
        {#snippet back_button()}
            <a href={Routes.LIST_PAGE.path} use:link>
                <Button variant="ghost" size="sm" class="gap-2">
                    <ArrowLeftIcon class="h-4 w-4" />
                    Back to Products
                </Button>
            </a>
        {/snippet}

        <!-- Additional Actions -->

        {#if product}
            <ProductForm
                bind:data={formData}
                bind:currentTab
                onchange={handleFormChange}
                {product}
            />
        {/if}

        {#snippet sidebar()}
            {#if product}
                <div class="space-y-6">
                    <!-- Quick Stats -->
                    <div class="rounded-lg border bg-card p-4">
                        <h3 class="font-semibold mb-3">Quick Stats</h3>
                        <div class="space-y-3">
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground"
                                    >Variants</span
                                >
                                <span class="font-medium"
                                    >{product.variants.length}</span
                                >
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground"
                                    >Base Price</span
                                >
                                <span class="font-medium">
                                    {new Intl.NumberFormat("en-US", {
                                        style: "currency",
                                        currency: "DZD",
                                    }).format(product.base_price)}
                                </span>
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground"
                                    >Category</span
                                >
                                <Badge variant="outline"
                                    >{product.category}</Badge
                                >
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground"
                                    >Featured</span
                                >
                                <Badge
                                    variant={product.featured
                                        ? "default"
                                        : "secondary"}
                                >
                                    {product.featured ? "Yes" : "No"}
                                </Badge>
                            </div>
                        </div>
                    </div>

                    <!-- Product Images -->
                    {#if product.base_images.length > 0}
                        <div class="rounded-lg border bg-card p-4">
                            <h3 class="font-semibold mb-3">Product Images</h3>
                            <div class="grid grid-cols-2 gap-2">
                                {#each product.base_images.slice(0, 4) as image}
                                    <div
                                        class="aspect-square rounded-md bg-muted overflow-hidden"
                                    >
                                        <img
                                            src={image}
                                            alt="Product"
                                            class="w-full h-full object-cover"
                                        />
                                    </div>
                                {/each}
                            </div>
                            {#if product.base_images.length > 4}
                                <p class="text-sm text-muted-foreground mt-2">
                                    +{product.base_images.length - 4} more images
                                </p>
                            {/if}
                        </div>
                    {/if}
                </div>
            {/if}
        {/snippet}
    </EditPageLayout>

    <!-- Delete Confirmation Dialog -->
    <AlertDialog.Root bind:open={showDeleteDialog}>
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title>Are you sure?</AlertDialog.Title>
                <AlertDialog.Description>
                    This action will archive the product "{product?.name}".
                    Archived products can be restored later, but they won't be
                    visible to customers.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
                <AlertDialog.Action
                    onclick={handleDelete}
                    disabled={isDeleting}
                    class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                >
                    {#if isDeleting}
                        <div class="flex items-center gap-2">
                            <div
                                class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
                            ></div>
                            Archiving...
                        </div>
                    {:else}
                        Archive Product
                    {/if}
                </AlertDialog.Action>
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>
    {#snippet error()}
        <div class="flex flex-col items-center justify-center py-12">
            <div class="text-center">
                <h3 class="text-lg font-semibold">Product not found</h3>
                <p class="text-muted-foreground mt-2">
                    The product you're looking for doesn't exist or has been
                    removed.
                </p>
                <a href={Routes.LIST_PAGE.path} use:link>
                    <Button class="mt-4">Back to Products</Button>
                </a>
            </div>
        </div>
    {/snippet}
</StatusBoundary>
