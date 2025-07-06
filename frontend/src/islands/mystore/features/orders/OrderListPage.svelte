<script lang="ts">
    import DataTable from "../../lib/components/DataTable.svelte";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { Button } from "@/lib/components/ui/button/index";
    import { Skeleton } from "@/lib/components/ui/skeleton/index";
    import PlusIcon from "@lucide/svelte/icons/plus";
    import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
    import { link } from "@dvcol/svelte-simple-router";
    import { create, debounce } from "lodash";

    import type { ColumnDef } from "@tanstack/table-core";
    import type { OrderPublic } from "@bindings/OrderPublic";
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
    import CellActions from "./OrderCellActions.svelte";

    const columns: ColumnDef<OrderPublic>[] = [
        {
            accessorKey: "id",
            header: "Order ID",
            cell: ({ row }) => {
                const idCellSnippet = createRawSnippet<[string]>((getId) => {
                    const id = getId();
                    return {
                        render: () =>
                            `<code class="px-2 py-1 text-xs bg-muted rounded font-mono">${id.slice(0, 8)}...</code>`,
                    };
                });
                return renderSnippet(idCellSnippet, row.getValue("id"));
            },
        },
        {
            accessorKey: "full_name",
            header: "Customer",
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
                return renderSnippet(
                    nameCellSnippet,
                    row.getValue("full_name"),
                );
            },
        },
        {
            accessorKey: "phone",
            header: "Phone",
            cell: ({ row }) => {
                const phoneCellSnippet = createRawSnippet<[string]>(
                    (getPhone) => {
                        const phone = getPhone();
                        return {
                            render: () =>
                                `<div class="text-sm text-muted-foreground">${phone}</div>`,
                        };
                    },
                );
                return renderSnippet(phoneCellSnippet, row.getValue("phone"));
            },
        },
        {
            accessorKey: "status",
            header: "Status",
            cell: ({ row }) => {
                const statusCellSnippet = createRawSnippet<[string]>(
                    (getStatus) => {
                        const status = getStatus();
                        let badgeClass = "";

                        switch (status) {
                            case "PENDING":
                                badgeClass =
                                    "bg-yellow-50 text-yellow-700 ring-yellow-600/20";
                                break;
                            case "CONFIRMED":
                                badgeClass =
                                    "bg-blue-50 text-blue-700 ring-blue-600/20";
                                break;
                            case "DELIVERED":
                                badgeClass =
                                    "bg-green-50 text-green-700 ring-green-600/20";
                                break;
                            case "DONE":
                                badgeClass =
                                    "bg-emerald-50 text-emerald-700 ring-emerald-600/20";
                                break;
                            case "CANCELED":
                                badgeClass =
                                    "bg-red-50 text-red-700 ring-red-600/20";
                                break;
                            case "RETURNED":
                                badgeClass =
                                    "bg-gray-50 text-gray-700 ring-gray-600/20";
                                break;
                            default:
                                badgeClass =
                                    "bg-gray-50 text-gray-600 ring-gray-500/20";
                        }

                        return {
                            render: () =>
                                `<span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset ${badgeClass}">${status}</span>`,
                        };
                    },
                );

                return renderSnippet(statusCellSnippet, row.getValue("status"));
            },
        },
        {
            accessorKey: "province",
            header: "Province",
            cell: ({ row }) => {
                const provinceCellSnippet = createRawSnippet<[string]>(
                    (getProvince) => {
                        const province = getProvince();
                        return {
                            render: () =>
                                `<div class="text-sm">${province}</div>`,
                        };
                    },
                );
                return renderSnippet(
                    provinceCellSnippet,
                    row.getValue("province"),
                );
            },
        },
        {
            accessorKey: "delivery",
            header: "Delivery",
            cell: ({ row }) => {
                const deliveryCellSnippet = createRawSnippet<[string]>(
                    (getDelivery) => {
                        const delivery = getDelivery();
                        const badgeClass =
                            delivery === "HOME_DELIVERY"
                                ? "bg-blue-50 text-blue-700 ring-blue-600/20"
                                : "bg-orange-50 text-orange-700 ring-orange-600/20";
                        const displayText =
                            delivery === "HOME_DELIVERY" ? "Home" : "Pickup";

                        return {
                            render: () =>
                                `<span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset ${badgeClass}">${displayText}</span>`,
                        };
                    },
                );

                return renderSnippet(
                    deliveryCellSnippet,
                    row.getValue("delivery"),
                );
            },
        },
        {
            accessorKey: "items",
            header: "Items",
            cell: ({ row }) => {
                const itemsCellSnippet = createRawSnippet<
                    [Record<string, any>]
                >((getItems) => {
                    const items = getItems();
                    const itemCount = Object.keys(items || {}).length;
                    return {
                        render: () =>
                            `<div class="text-sm font-medium">${itemCount} item${itemCount !== 1 ? "s" : ""}</div>`,
                    };
                });

                return renderSnippet(itemsCellSnippet, row.getValue("items"));
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

    let data: OrderPublic[] = [];
    let per_page: number = 10;
    let page: number = 1;
    let total_pages: number = 1;
    let status: LoadingStatus = undefined;
    let searchQuery: string = "";
    let isSearching: boolean = false;
    let currentTab: string = "all";

    // Debounced search function
    const debouncedSearch = debounce((query: string) => {
        page = 1; // Reset to first page when searching
        fetchData({ page: 1, per_page, next_token: null }, query);
    }, 300);

    async function fetchData(pagination: Pagination, search?: string) {
        status = undefined;
        isSearching = !!search;

        try {
            // You might need to modify your API to accept search parameters
            const res = await ApiRoutes.get_all_orders(pagination);
            data = res.data;
            total_pages = Math.ceil((res.total ?? 1) / (res.per_page ?? 1));
            status = null;
        } catch (err: any) {
            status = err;
        } finally {
            isSearching = false;
        }
    }

    // Filter data based on current tab
    function filterDataByTab(
        orders: OrderPublic[],
        tab: string,
    ): OrderPublic[] {
        switch (tab) {
            case "pending":
                return orders.filter((order) => order.status === "PENDING");
            case "confirmed":
                return orders.filter((order) => order.status === "CONFIRMED");
            case "delivered":
                return orders.filter(
                    (order) =>
                        order.status === "DELIVERED" || order.status === "DONE",
                );
            case "canceled":
                return orders.filter(
                    (order) =>
                        order.status === "CANCELED" ||
                        order.status === "RETURNED",
                );
            default:
                return orders;
        }
    }

    // Watch for page changes
    $: fetchData({ page: page, per_page, next_token: null });

    // Watch for search changes
    $: if (searchQuery) {
        debouncedSearch(searchQuery);
    }

    // Filter data based on current tab
    $: filteredData = filterDataByTab(data, currentTab);

    $: document.title = "Orders";
</script>

<div class="flex flex-1 flex-col space-y-6">
    <!-- Header -->
    <div class="flex flex-col gap-4 p-6 pb-0">
        <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
                <div
                    class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10"
                >
                    <ShoppingCartIcon class="h-5 w-5 text-primary" />
                </div>
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">Orders</h1>
                    <p class="text-muted-foreground">
                        Manage customer orders and track their status.
                    </p>
                </div>
            </div>
            <a href="/orders/create" use:link>
                <Button class="gap-2">
                    <PlusIcon class="h-4 w-4" />
                    New Order
                </Button>
            </a>
        </div>
    </div>

    <!-- Tabs for filtering -->
    <div class="px-6 mb-2">
        <Tabs.Root bind:value={currentTab} class="w-full">
            <Tabs.List>
                <Tabs.Trigger value="all">All Orders</Tabs.Trigger>
                <Tabs.Trigger value="pending">Pending</Tabs.Trigger>
                <Tabs.Trigger value="confirmed">Confirmed</Tabs.Trigger>
                <Tabs.Trigger value="delivered">Delivered</Tabs.Trigger>
                <Tabs.Trigger value="canceled">Canceled</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>
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
                    data={filteredData}
                    bind:per_page
                    bind:page
                    bind:total_pages
                />
            {/if}
        </StatusBoundary>
    </div>
</div>
