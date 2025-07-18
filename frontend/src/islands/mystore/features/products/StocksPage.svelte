<script lang="ts">
    import DataTable from "../../lib/components/DataTable.svelte";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { Button } from "@/lib/components/ui/button/index";
    import { Skeleton } from "@/lib/components/ui/skeleton/index";
    import { Input } from "lib/components/ui/input";
    import { Badge } from "@/lib/components/ui/badge/index";
    import * as Dialog from "@/lib/components/ui/dialog/index";
    import * as Select from "@/lib/components/ui/select/index";
    import { Textarea } from "@/lib/components/ui/textarea/index";
    import { Checkbox } from "@/lib/components/ui/checkbox/index";
    import { Label } from "@/lib/components/ui/label/index";
    import PackageIcon from "@lucide/svelte/icons/package";
    import SearchIcon from "@lucide/svelte/icons/search";
    import PlusIcon from "@lucide/svelte/icons/plus";
    import MinusIcon from "@lucide/svelte/icons/minus";
    import EditIcon from "@lucide/svelte/icons/edit";
    import AlertTriangleIcon from "@lucide/svelte/icons/alert-triangle";
    import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
    import TrendingDownIcon from "@lucide/svelte/icons/trending-down";
    import { debounce } from "lodash";

    import type { ColumnDef } from "@tanstack/table-core";
    import type { ProductVariant } from "@bindings/ProductVariant";
    import StatusBoundary, {
        type LoadingStatus,
    } from "@/components/StatusBoundary.svelte";
    import { createRawSnippet } from "svelte";
    import {
        renderSnippet,
        renderComponent,
    } from "@/lib/components/ui/data-table/index";
    import * as Tabs from "@/lib/components/ui/tabs/index";
    import type { Pagination } from "@bindings/Pagination";

    // Extended type for stock management
    type StockItem = ProductVariant & {
        product_name: string;
        product_id: string;
        low_stock_threshold: number;
        last_updated: string;
        selected?: boolean;
    };

    type StockAdjustment = {
        variant_sku: string;
        adjustment_type: "increase" | "decrease" | "set";
        quantity: number;
        reason: string;
        notes?: string;
    };

    const columns: ColumnDef<StockItem>[] = [
        {
            id: "select",
            header: ({ table }) => {
                const selectAllSnippet = createRawSnippet<
                    [[boolean, (checked: boolean) => void]]
                >((getParam) => {
                    const allSelected = getParam()[0];

                    return {
                        render: () => `
                                <input 
                                    type="checkbox" 
                                    ${allSelected ? "checked" : ""} 
                                    onchange="this.dispatchEvent(new CustomEvent('toggle-all', { detail: { checked: this.checked } }))"
                                    class="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
                                />
                            `,
                    };
                });
                return renderSnippet(selectAllSnippet, [
                    table.getIsAllPageRowsSelected(),
                    (checked: boolean) =>
                        table.toggleAllPageRowsSelected(checked),
                ]);
            },
            cell: ({ row }) => {
                const checkboxSnippet = createRawSnippet<
                    [boolean, (checked: boolean) => void]
                >((selected, toggle) => {
                    return {
                        render: () => `
                                <input 
                                    type="checkbox" 
                                    ${selected() ? "checked" : ""} 
                                    onchange="this.dispatchEvent(new CustomEvent('toggle-row', { detail: { checked: this.checked } }))"
                                    class="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
                                />
                            `,
                    };
                });
                return renderSnippet(
                    checkboxSnippet,
                    row.getIsSelected(),
                    (checked: boolean) => row.toggleSelected(checked),
                );
            },
        },
        {
            accessorKey: "sku",
            header: "SKU",
            cell: ({ row }) => {
                const productCellSnippet = createRawSnippet<[string]>(
                    (getSku) => {
                        const sku = getSku();
                        return {
                            render: () => `
                                <div class="space-y-1">
                                    <code class="px-2 py-1 text-xs bg-muted rounded font-mono">${sku}</code>
                                </div>
                            `,
                        };
                    },
                );
                return renderSnippet(productCellSnippet, row.getValue("sku"));
            },
        },
        {
            accessorKey: "options",
            header: "Variant",
            cell: ({ row }) => {
                const variantCellSnippet = createRawSnippet<
                    [Record<string, string>]
                >((getOptions) => {
                    const options = getOptions();
                    const optionEntries = Object.entries(options);
                    if (optionEntries.length === 0) {
                        return {
                            render: () =>
                                '<span class="text-muted-foreground">Default</span>',
                        };
                    }
                    const optionTags = optionEntries
                        .map(
                            ([key, value]) =>
                                `<span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium bg-secondary text-secondary-foreground">${key}: ${value}</span>`,
                        )
                        .join(" ");
                    return {
                        render: () =>
                            `<div class="flex flex-wrap gap-1">${optionTags}</div>`,
                    };
                });
                return renderSnippet(
                    variantCellSnippet,
                    row.getValue("options"),
                );
            },
        },
        {
            accessorKey: "stocks",
            header: "Current Stock",
            cell: ({ row }) => {
                const stockCellSnippet = createRawSnippet<[number[]]>(
                    (getParam) => {
                        const [stock, threshold] = getParam();
                        const isLowStock = stock <= threshold;
                        const stockClass = isLowStock
                            ? "text-red-600 font-semibold"
                            : stock === 0
                              ? "text-gray-500"
                              : "text-green-600 font-semibold";
                        const icon = isLowStock
                            ? "⚠️"
                            : stock === 0
                              ? "❌"
                              : "✅";
                        return {
                            render: () => `
                                <div class="flex items-center gap-2">
                                    <span class="${stockClass}">${stock}</span>
                                    ${isLowStock ? '<span class="text-xs">⚠️</span>' : ""}
                                </div>
                            `,
                        };
                    },
                );
                return renderSnippet(stockCellSnippet, [
                    row.getValue("stocks") as number,
                    row.getValue("low_stock_threshold") as number,
                ]);
            },
        },
        {
            accessorKey: "price",
            header: "Price",
            cell: ({ row }) => {
                const price = row.getValue("price") as number;
                const formatter = new Intl.NumberFormat("en-US", {
                    style: "currency",
                    currency: "DZD",
                });

                const priceCellSnippet = createRawSnippet<[string]>(
                    (getFormattedPrice) => {
                        const formattedPrice = getFormattedPrice();
                        return {
                            render: () =>
                                `<div class="font-medium">${formattedPrice}</div>`,
                        };
                    },
                );

                return renderSnippet(
                    priceCellSnippet,
                    formatter.format(price || 0),
                );
            },
        },
        {
            accessorKey: "last_updated",
            header: "Last Updated",
            cell: ({ row }) => {
                const dateCellSnippet = createRawSnippet<[string]>(
                    (getDate) => {
                        const date = getDate();
                        const formatted = new Date(date).toLocaleDateString();
                        return {
                            render: () =>
                                `<div class="text-sm text-muted-foreground">${formatted}</div>`,
                        };
                    },
                );
                return renderSnippet(
                    dateCellSnippet,
                    row.getValue("last_updated"),
                );
            },
        },
        {
            id: "actions",
            header: "Actions",
            cell: ({ row }) => {
                return renderSnippet(adjust_btn, row.getValue("sku"));
            },
        },
    ];

    let data: StockItem[] = [];
    let per_page: number = 10;
    let page: number = 1;
    let total_pages: number = 1;
    let status: LoadingStatus = undefined;
    let searchQuery: string = "";
    let stockFilter: string = "all";
    let isSearching: boolean = false;
    let selectedItems: StockItem[] = [];

    // Dialog states
    let adjustStockDialog = false;
    let bulkAdjustDialog = false;
    let currentAdjustmentSku: string = "";
    let adjustmentType: "increase" | "decrease" | "set" = "increase";
    let adjustmentQuantity: number = 1;
    let adjustmentReason: string = "";
    let adjustmentNotes: string = "";

    // Debounced search function
    const debouncedSearch = debounce((query?: string, filter?: string) => {
        page = 1;
        fetchData({ page: 1, per_page, next_token: null }, query, filter);
    }, 1000);

    async function fetchData(
        pagination: Pagination,
        searchQuery?: string,
        stockFilter?: string,
    ) {
        status = undefined;
        isSearching = !!searchQuery;

        try {
            // This would be your actual API call
            // const res = await ApiRoutes.get_stock_items(pagination, searchQuery, stockFilter);

            // Mock data for demonstration
            const mockData: StockItem[] = [
                {
                    sku: "SHIRT-001-RED-M",
                    product_name: "Cotton T-Shirt",
                    product_id: "prod_001",
                    price: 2500,
                    compare_price: 3000,
                    stocks: 15,
                    low_stock_threshold: 10,
                    images: [],
                    options: { color: "Red", size: "M" },
                    last_updated: "2024-01-15T10:30:00Z",
                },
                {
                    sku: "SHIRT-001-BLUE-L",
                    product_name: "Cotton T-Shirt",
                    product_id: "prod_001",
                    price: 2500,
                    compare_price: 3000,
                    stocks: 5,
                    low_stock_threshold: 10,
                    images: [],
                    options: { color: "Blue", size: "L" },
                    last_updated: "2024-01-14T14:20:00Z",
                },
                {
                    sku: "PANTS-002-BLACK-32",
                    product_name: "Denim Jeans",
                    product_id: "prod_002",
                    price: 4500,
                    compare_price: 5000,
                    stocks: 0,
                    low_stock_threshold: 5,
                    images: [],
                    options: { color: "Black", size: "32" },
                    last_updated: "2024-01-13T09:15:00Z",
                },
            ];

            data = mockData;
            total_pages = Math.ceil(mockData.length / per_page);
            status = null;
        } catch (err: any) {
            status = err;
        } finally {
            isSearching = false;
        }
    }

    function openAdjustStock(sku: string) {
        currentAdjustmentSku = sku;
        adjustStockDialog = true;
    }

    function openBulkAdjust() {
        bulkAdjustDialog = true;
    }

    async function handleStockAdjustment() {
        // This would be your API call
        const adjustment: StockAdjustment = {
            variant_sku: currentAdjustmentSku,
            adjustment_type: adjustmentType,
            quantity: adjustmentQuantity,
            reason: adjustmentReason,
            notes: adjustmentNotes,
        };

        try {
            // await ApiRoutes.adjust_stock(adjustment);
            console.log("Stock adjustment:", adjustment);

            // Refresh data
            await fetchData(
                { page, per_page, next_token: null },
                searchQuery,
                stockFilter,
            );

            // Reset form
            adjustStockDialog = false;
            resetAdjustmentForm();
        } catch (err) {
            console.error("Failed to adjust stock:", err);
        }
    }

    function resetAdjustmentForm() {
        currentAdjustmentSku = "";
        adjustmentType = "increase";
        adjustmentQuantity = 1;
        adjustmentReason = "";
        adjustmentNotes = "";
    }

    // Watch for page changes
    $: fetchData(
        { page: page, per_page, next_token: null },
        searchQuery,
        stockFilter,
    );

    // Watch for search/filter changes
    $: if (searchQuery || stockFilter) {
        debouncedSearch(searchQuery, stockFilter);
    }

    $: document.title = "Stock Management";

    // Stats calculations
    $: stats = {
        total: data.length,
        lowStock: data.filter((item) => item.stocks <= item.low_stock_threshold)
            .length,
        outOfStock: data.filter((item) => item.stocks === 0).length,
        totalValue: data.reduce(
            (sum, item) => sum + item.stocks * (item.price || 0),
            0,
        ),
    };

    // Handle row selection
    function handleRowSelection(selectedRows: StockItem[]) {
        selectedItems = data;
    }
</script>

{#snippet adjust_btn(sku: string)}
    <div class="flex items-center gap-2">
        <button
            onclick={() => openAdjustStock(sku)}
            class="inline-flex items-center gap-1 px-2 py-1 text-xs bg-primary text-primary-foreground rounded hover:bg-primary/90"
        >
            <svg
                class="h-3 w-3"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                />
            </svg>
            Adjust
        </button>
    </div>
{/snippet}

<div class="flex flex-1 flex-col space-y-6">
    <!-- Header -->
    <div class="flex flex-col gap-4 p-6 pb-0">
        <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
                <div
                    class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10"
                >
                    <PackageIcon class="h-5 w-5 text-primary" />
                </div>
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">
                        Stock Management
                    </h1>
                    <p class="text-muted-foreground">
                        Monitor and manage your inventory levels
                    </p>
                </div>
            </div>
        </div>

        <!-- Stats Cards -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="p-4 bg-card rounded-lg border">
                <div class="flex items-center gap-2">
                    <PackageIcon class="h-4 w-4 text-muted-foreground" />
                    <span class="text-sm font-medium text-muted-foreground"
                        >Total Items</span
                    >
                </div>
                <div class="text-2xl font-bold">{stats.total}</div>
            </div>
            <div class="p-4 bg-card rounded-lg border">
                <div class="flex items-center gap-2">
                    <AlertTriangleIcon class="h-4 w-4 text-yellow-500" />
                    <span class="text-sm font-medium text-muted-foreground"
                        >Low Stock</span
                    >
                </div>
                <div class="text-2xl font-bold text-yellow-600">
                    {stats.lowStock}
                </div>
            </div>
            <div class="p-4 bg-card rounded-lg border">
                <div class="flex items-center gap-2">
                    <TrendingDownIcon class="h-4 w-4 text-red-500" />
                    <span class="text-sm font-medium text-muted-foreground"
                        >Out of Stock</span
                    >
                </div>
                <div class="text-2xl font-bold text-red-600">
                    {stats.outOfStock}
                </div>
            </div>
            <div class="p-4 bg-card rounded-lg border">
                <div class="flex items-center gap-2">
                    <TrendingUpIcon class="h-4 w-4 text-green-500" />
                    <span class="text-sm font-medium text-muted-foreground"
                        >Total Value</span
                    >
                </div>
                <div class="text-2xl font-bold text-green-600">
                    {new Intl.NumberFormat("en-US", {
                        style: "currency",
                        currency: "DZD",
                        maximumFractionDigits: 0,
                    }).format(stats.totalValue)}
                </div>
            </div>
        </div>
    </div>

    <!-- Search and Filters -->
    <div class="px-6 mb-2 flex flex-row justify-between">
        <Tabs.Root bind:value={stockFilter} class="w-[400px]">
            <Tabs.List>
                <Tabs.Trigger value="all">All Items</Tabs.Trigger>
                <Tabs.Trigger value="low">Low Stock</Tabs.Trigger>
                <Tabs.Trigger value="out">Out of Stock</Tabs.Trigger>
                <Tabs.Trigger value="good">Good Stock</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>
        <div class="flex flex-row gap-2">
            <div class="flex gap-2">
                <Button
                    variant="outline"
                    class="gap-2"
                    disabled={selectedItems.length === 0}
                    onclick={openBulkAdjust}
                >
                    <EditIcon class="h-4 w-4" />
                    Bulk Adjust ({selectedItems.length})
                </Button>
            </div>
            <div class="relative flex-1 max-w-sm">
                <SearchIcon
                    class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
                />
                <Input
                    placeholder="Search products or SKUs..."
                    bind:value={searchQuery}
                    class="pl-10"
                />
                {#if isSearching}
                    <div class="absolute right-3 top-1/2 -translate-y-1/2">
                        <div
                            class="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent"
                        ></div>
                    </div>
                {/if}
            </div>
        </div>
    </div>

    <!-- Data Table -->
    <div class="flex-1 px-6 pb-6">
        <StatusBoundary {status}>
            {#if status === undefined}
                <!-- Loading skeleton -->
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <Skeleton class="h-10 w-64" />
                        <Skeleton class="h-10 w-24" />
                    </div>
                    <div class="rounded-md border">
                        <div class="p-4">
                            {#each Array(5) as _}
                                <div class="flex items-center space-x-4 py-3">
                                    <Skeleton class="h-4 w-4" />
                                    <Skeleton class="h-4 w-32" />
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-8 w-8" />
                                </div>
                            {/each}
                        </div>
                    </div>
                </div>
            {:else}
                <DataTable
                    {columns}
                    {data}
                    bind:per_page
                    bind:page
                    bind:total_pages
                    onselectionchange={handleRowSelection}
                />
            {/if}
        </StatusBoundary>
    </div>
</div>

<!-- Adjust Stock Dialog -->
<Dialog.Root bind:open={adjustStockDialog}>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Adjust Stock</Dialog.Title>
            <Dialog.Description>
                Update stock levels for SKU: {currentAdjustmentSku}
            </Dialog.Description>
        </Dialog.Header>
        <div class="space-y-4 py-4">
            <div class="space-y-2">
                <Label for="adjustment-type">Adjustment Type</Label>
                <Select.Root type="single" bind:value={adjustmentType}>
                    <Select.Trigger
                        >{adjustmentType ||
                            "Select adjustment type"}</Select.Trigger
                    >
                    <Select.Content>
                        <Select.Item value="increase"
                            >Increase Stock</Select.Item
                        >
                        <Select.Item value="decrease"
                            >Decrease Stock</Select.Item
                        >
                        <Select.Item value="set">Set Stock Level</Select.Item>
                    </Select.Content>
                </Select.Root>
            </div>

            <div class="space-y-2">
                <Label for="quantity">Quantity</Label>
                <Input
                    id="quantity"
                    type="number"
                    bind:value={adjustmentQuantity}
                    min="1"
                    placeholder="Enter quantity"
                />
            </div>

            <div class="space-y-2">
                <Label for="reason">Reason</Label>
                <Select.Root type="single" bind:value={adjustmentReason}>
                    <Select.Trigger
                        >{adjustmentReason || "Select reason"}</Select.Trigger
                    >
                    <Select.Content>
                        <Select.Item value="restock">Restock</Select.Item>
                        <Select.Item value="sale">Sale</Select.Item>
                        <Select.Item value="damaged">Damaged</Select.Item>
                        <Select.Item value="returned">Returned</Select.Item>
                        <Select.Item value="correction">Correction</Select.Item>
                        <Select.Item value="other">Other</Select.Item>
                    </Select.Content>
                </Select.Root>
            </div>

            <div class="space-y-2">
                <Label for="notes">Notes (Optional)</Label>
                <Textarea
                    id="notes"
                    bind:value={adjustmentNotes}
                    placeholder="Add any additional notes..."
                />
            </div>
        </div>
        <Dialog.Footer>
            <Button
                variant="outline"
                on:click={() => (adjustStockDialog = false)}
            >
                Cancel
            </Button>
            <Button on:click={handleStockAdjustment}>Adjust Stock</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>

<!-- Bulk Adjust Dialog -->
<Dialog.Root bind:open={bulkAdjustDialog}>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Bulk Stock Adjustment</Dialog.Title>
            <Dialog.Description>
                Adjust stock for {selectedItems.length} selected items
            </Dialog.Description>
        </Dialog.Header>
        <div class="space-y-4 py-4">
            <div class="space-y-2">
                <Label>Selected Items</Label>
                <div class="max-h-32 overflow-y-auto border rounded p-2">
                    {#each selectedItems as item}
                        <div class="text-sm py-1">
                            {item.product_name} - {item.sku}
                        </div>
                    {/each}
                </div>
            </div>

            <!-- Same form fields as single adjustment -->
            <div class="space-y-2">
                <Label for="bulk-adjustment-type">Adjustment Type</Label>
                <Select.Root type="single" bind:value={adjustmentType}>
                    <Select.Trigger>Select adjustment type</Select.Trigger>
                    <Select.Content>
                        <Select.Item value="increase"
                            >Increase Stock</Select.Item
                        >
                        <Select.Item value="decrease"
                            >Decrease Stock</Select.Item
                        >
                        <Select.Item value="set">Set Stock Level</Select.Item>
                    </Select.Content>
                </Select.Root>
            </div>

            <div class="space-y-2">
                <Label for="bulk-quantity">Quantity</Label>
                <Input
                    id="bulk-quantity"
                    type="number"
                    bind:value={adjustmentQuantity}
                    min="1"
                    placeholder="Enter quantity"
                />
            </div>

            <div class="space-y-2">
                <Label for="bulk-reason">Reason</Label>
                <Select.Root type="single" bind:value={adjustmentReason}>
                    <Select.Trigger
                        >{adjustmentReason || "Select reason"}</Select.Trigger
                    >
                    <Select.Content>
                        <Select.Item value="restock">Restock</Select.Item>
                        <Select.Item value="sale">Sale</Select.Item>
                        <Select.Item value="damaged">Damaged</Select.Item>
                        <Select.Item value="returned">Returned</Select.Item>
                        <Select.Item value="correction">Correction</Select.Item>
                        <Select.Item value="other">Other</Select.Item>
                    </Select.Content>
                </Select.Root>
            </div>

            <div class="space-y-2">
                <Label for="bulk-notes">Notes (Optional)</Label>
                <Textarea
                    id="bulk-notes"
                    bind:value={adjustmentNotes}
                    placeholder="Add any additional notes..."
                />
            </div>
        </div>
        <Dialog.Footer>
            <Button
                variant="outline"
                on:click={() => (bulkAdjustDialog = false)}
            >
                Cancel
            </Button>
            <Button on:click={handleStockAdjustment}>
                Adjust Selected Items
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
