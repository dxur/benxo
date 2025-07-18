<script lang="ts">
    import DataTable from "../../lib/components/DataTable.svelte";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { Button } from "@/lib/components/ui/button/index";
    import { Skeleton } from "@/lib/components/ui/skeleton/index";
    import PlusIcon from "@lucide/svelte/icons/plus";
    import PackageIcon from "@lucide/svelte/icons/package";
    import { link } from "@dvcol/svelte-simple-router";
    import { create, debounce } from "lodash";

    import type { ColumnDef } from "@tanstack/table-core";
    import type { ProductPublic } from "@bindings/ProductPublic";
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
    import CellActions from "./ProductCellActions.svelte";
    import { Input } from "lib/components/ui/input";
    import { SearchIcon } from "@lucide/svelte";

    const columns: ColumnDef<ProductPublic>[] = [
        {
            accessorKey: "slug",
            header: "Slug",
            cell: ({ row }) => {
                const slugCellSnippet = createRawSnippet<[string]>(
                    (getSlug) => {
                        const slug = getSlug();
                        return {
                            render: () =>
                                `<code class="px-2 py-1 text-xs bg-muted rounded font-mono">${slug}</code>`,
                        };
                    },
                );
                return renderSnippet(slugCellSnippet, row.getValue("slug"));
            },
        },
        {
            accessorKey: "name",
            header: "Name",
            cell: ({ row }) => {
                const nameCellSnippet = createRawSnippet<[string]>(
                    (getName) => {
                        const name = getName();
                        return {
                            render: () =>
                                `<div class="font-medium text-foreground">${name}</div>`,
                        };
                    },
                );
                return renderSnippet(nameCellSnippet, row.getValue("name"));
            },
        },
        {
            accessorKey: "category",
            header: "Category",
            cell: ({ row }) => {
                const categoryCellSnippet = createRawSnippet<[string]>(
                    (getCategory) => {
                        const category = getCategory();
                        return {
                            render: () =>
                                `<span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium bg-blue-50 text-blue-700 ring-1 ring-inset ring-blue-600/20">${category}</span>`,
                        };
                    },
                );
                return renderSnippet(
                    categoryCellSnippet,
                    row.getValue("category"),
                );
            },
        },
        {
            accessorKey: "base_price",
            header: "Price",
            cell: ({ row }) => {
                const formatter = new Intl.NumberFormat("en-US", {
                    style: "currency",
                    currency: "DZD",
                });

                const amountCellSnippet = createRawSnippet<[string]>(
                    (getAmount) => {
                        const amount = getAmount();
                        return {
                            render: () =>
                                `<div class="font-semibold text-green-700">${amount}</div>`,
                        };
                    },
                );

                return renderSnippet(
                    amountCellSnippet,
                    formatter.format(parseFloat(row.getValue("base_price"))),
                );
            },
        },
        {
            accessorKey: "base_compare_price",
            header: "Compare-at Price",
            cell: ({ row }) => {
                const comparePrice = parseFloat(
                    row.getValue("base_compare_price"),
                );
                if (comparePrice === 0) {
                    return renderSnippet(
                        createRawSnippet(() => ({
                            render: () =>
                                `<div class="font-semibold text-red-700">N/A</div>`,
                        })),
                    );
                } else {
                    const formatter = new Intl.NumberFormat("en-US", {
                        style: "currency",
                        currency: "DZD",
                    });

                    const amountCellSnippet = createRawSnippet<[string]>(
                        (getAmount) => {
                            const amount = getAmount();
                            return {
                                render: () =>
                                    `<div class="font-semibold text-red-700">${amount}</div>`,
                            };
                        },
                    );

                    return renderSnippet(
                        amountCellSnippet,
                        formatter.format(comparePrice),
                    );
                }
            },
        },
        {
            accessorKey: "featured",
            header: "Featured",
            cell: ({ row }) => {
                const featuredCellSnippet = createRawSnippet<[boolean]>(
                    (getFeatured) => {
                        const featured = getFeatured();
                        const badgeClass = featured
                            ? "bg-emerald-50 text-emerald-700 ring-emerald-600/20"
                            : "bg-gray-50 text-gray-600 ring-gray-500/20";
                        return {
                            render: () =>
                                `<span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset ${badgeClass}">${featured ? "Featured" : "Regular"}</span>`,
                        };
                    },
                );

                return renderSnippet(
                    featuredCellSnippet,
                    row.getValue("featured"),
                );
            },
        },
        {
            id: "actions",
            header: "Actions",
            cell: ({ row }) => {
                return renderComponent(CellActions, {
                    id: row.original.id,
                });
            },
        },
    ];

    let data: ProductPublic[] = [];
    let per_page: number = 10;
    let page: number = 1;
    let total_pages: number = 1;
    let status: LoadingStatus = undefined;
    let searchQuery: string = "";
    let statusQuery: string = "all";
    let isSearching: boolean = false;

    // Debounced search function
    const debouncedSearch = debounce((query?: string, status?: string) => {
        page = 1;
        fetchData({ page: 1, per_page, next_token: null }, query);
    }, 1000);

    async function fetchData(
        pagination: Pagination,
        searchQuery?: string,
        statusQuery?: string,
    ) {
        status = undefined;
        isSearching = !!searchQuery;

        try {
            // You might need to modify your API to accept search parameters
            const res = await ApiRoutes.get_all_products(pagination);
            data = res.data;
            total_pages = Math.ceil((res.total ?? 1) / (res.per_page ?? 1));
            status = null;
        } catch (err: any) {
            status = err;
        } finally {
            isSearching = false;
        }
    }

    // Watch for page changes
    $: fetchData({ page: page, per_page, next_token: null });

    // Watch for search changes
    $: if (searchQuery || statusQuery) {
        debouncedSearch(searchQuery, statusQuery);
    }

    $: document.title = "Products";

    // Stats (you might want to fetch these from your API)
    let stats = {
        total: data.length,
        featured: data.filter((p) => p.featured).length,
        categories: [...new Set(data.map((p) => p.category))].length,
    };

    $: stats = {
        total: data.length,
        featured: data.filter((p) => p.featured).length,
        categories: [...new Set(data.map((p) => p.category))].length,
    };
</script>

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
                    <h1 class="text-3xl font-bold tracking-tight">Products</h1>
                    <p class="text-muted-foreground">
                        Manage your product inventory and pricing
                    </p>
                </div>
            </div>
            <a href="/products/create" use:link>
                <Button class="gap-2">
                    <PlusIcon class="h-4 w-4" />
                    Add Product
                </Button>
            </a>
        </div>
    </div>

    <!-- Search and Filters -->

    <div class="px-6 mb-2 flex flex-row justify-between">
        <Tabs.Root bind:value={statusQuery} class="w-[400px]">
            <Tabs.List>
                <Tabs.Trigger value="all">All</Tabs.Trigger>
                <Tabs.Trigger value="active">Active</Tabs.Trigger>
                <Tabs.Trigger value="inactive">Inactive</Tabs.Trigger>
                <Tabs.Trigger value="draft">Draft</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>
        <div class="relative flex-1 max-w-sm">
            <SearchIcon
                class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
            />
            <Input
                placeholder="Search products..."
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
                                    <Skeleton class="h-4 w-20" />
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
                />
            {/if}
        </StatusBoundary>
    </div>
</div>
