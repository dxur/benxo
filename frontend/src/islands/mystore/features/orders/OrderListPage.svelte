<script lang="ts">
    import DataTable from "../../lib/components/DataTable.svelte";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { Button } from "@/lib/components/ui/button/index";
    import { Input } from "@/lib/components/ui/input/index";
    import { Skeleton } from "@/lib/components/ui/skeleton/index";
    import { Badge } from "@/lib/components/ui/badge/index";
    import * as Select from "@/lib/components/ui/select/index";
    import * as Card from "@/lib/components/ui/card/index";
    import { Separator } from "@/lib/components/ui/separator/index";

    import PlusIcon from "@lucide/svelte/icons/plus";
    import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
    import SearchIcon from "@lucide/svelte/icons/search";
    import FilterIcon from "@lucide/svelte/icons/filter";
    import RefreshCcwIcon from "@lucide/svelte/icons/refresh-ccw";
    import DownloadIcon from "@lucide/svelte/icons/download";
    import XIcon from "@lucide/svelte/icons/x";

    import { link } from "@dvcol/svelte-simple-router";
    import { debounce } from "lodash";

    import type { ColumnDef } from "@tanstack/table-core";
    import type { OrderPublic } from "@bindings/OrderPublic";
    import type { OrderStatus } from "@bindings/OrderStatus";
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

    // Data and state
    let data: OrderPublic[] = [];
    let per_page: number = 10;
    let page: number = 1;
    let total_pages: number = 1;
    let total_count: number = 0;
    let status: LoadingStatus = undefined;

    // Search and filtering
    let searchQuery: string = "";
    let searchInput: HTMLInputElement | null = null;
    let isSearching: boolean = false;
    let currentTab: string = "all";
    let selectedProvinces: string[] = [];
    let selectedDeliveryType: string = "";
    let dateRange: { from: string; to: string } = { from: "", to: "" };
    let showAdvancedFilters: boolean = false;

    const provinces = [
        "Algiers",
        "Oran",
        "Constantine",
        "Annaba",
        "Blida",
        "Batna",
        "Djelfa",
        "Sétif",
        "Sidi Bel Abbès",
        "Biskra",
        "Tebessa",
        "El Oued",
        "Skikda",
        "Tiaret",
        "Béjaïa",
        "Tlemcen",
        "Ouargla",
        "Béchar",
        "Mostaganem",
        "Bordj Bou Arréridj",
        "Chlef",
        "Médéa",
        "El Tarf",
        "Jijel",
        "Relizane",
        "M'Sila",
        "Saïda",
        "Mascara",
        "Ouled Djellal",
        "Bouira",
        "Boumerdès",
        "El Taref",
        "Tindouf",
        "Tissemsilt",
        "El Bayadh",
        "Khenchela",
        "Mila",
        "Aïn Defla",
        "Naâma",
        "Aïn Témouchent",
        "Ghardaïa",
        "Laghouat",
        "Oum El Bouaghi",
        "Guelma",
        "Adrar",
        "Tamanrasset",
        "Illizi",
        "Timimoun",
        "Bordj Badji Mokhtar",
        "Ouled Djellal",
        "Béni Abbès",
        "In Salah",
        "In Guezzam",
        "Touggourt",
        "Djanet",
        "El M'Ghair",
        "El Meniaa",
    ];

    const deliveryTypes = [
        { value: "DOMICIL", label: "Home Delivery" },
        { value: "STOPDESK", label: "Pickup Point" },
    ];

    // Table columns
    const columns: ColumnDef<OrderPublic>[] = [
        {
            accessorKey: "id",
            header: "Order ID",
            cell: ({ row }) => {
                const idCellSnippet = createRawSnippet<[string]>((getId) => {
                    const id = getId();
                    return {
                        render: () =>
                            `<code class="px-2 py-1 text-xs bg-muted rounded font-mono">${id.slice(-8)}...</code>`,
                    };
                });
                return renderSnippet(idCellSnippet, row.getValue("id"));
            },
        },
        {
            accessorKey: "created_at",
            header: "Created At",
            cell: ({ row }) => {
                const createdAtCellSnippet = createRawSnippet<[string]>(
                    (getCreatedAt) => {
                        const createdAt = getCreatedAt();
                        const date = new Date(createdAt).toLocaleDateString();
                        const time = new Date(createdAt).toLocaleTimeString(
                            "en-US",
                            {
                                hour: "2-digit",
                                minute: "2-digit",
                            },
                        );
                        return {
                            render: () => `<div class="text-sm">
                                <div class="font-medium">${date}</div>
                                <div class="text-muted-foreground text-xs">${time}</div>
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    createdAtCellSnippet,
                    row.getValue("created_at"),
                );
            },
        },
        {
            accessorKey: "source",
            header: "Source",
            cell: ({ row }) => {
                const sourceCellSnippet = createRawSnippet<[string]>(
                    (getSource) => {
                        return {
                            render: () =>
                                `<div class="font-medium text-foreground">Kelle.store</div>`,
                        };
                    },
                );
                return renderSnippet(sourceCellSnippet, "Kelle.store");
            },
        },
        {
            accessorKey: "full_name",
            header: "Customer",
            cell: ({ row }) => {
                const customerCellSnippet = createRawSnippet<
                    [
                        string, // full_name
                    ]
                >((getFullName) => {
                    const fullName = getFullName();
                    return {
                        render: () =>
                            `<div class="font-medium text-foreground">${fullName}</div>`,
                    };
                });
                return renderSnippet(
                    customerCellSnippet,
                    row.getValue("full_name"),
                );
            },
        },
        {
            accessorKey: "phone",
            header: "Phone",
            cell: ({ row }) => {
                return row.getValue("phone");
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
                            delivery === "DOMICIL"
                                ? "bg-blue-50 text-blue-700 ring-blue-600/20"
                                : "bg-orange-50 text-orange-700 ring-orange-600/20";
                        const displayText =
                            delivery === "DOMICIL" ? "Home" : "Pickup";

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
                    const totalQuantity = Object.values(items || {})
                        .filter((item) => !!item)
                        .reduce(
                            (sum: number, item: any) =>
                                sum + (item.quantity || 0),
                            0,
                        );

                    return {
                        render: () =>
                            `<div class="text-sm">
                                <div class="font-medium">${itemCount} item${itemCount !== 1 ? "s" : ""}</div>
                                <div class="text-xs text-muted-foreground">Qty: ${totalQuantity}</div>
                            </div>`,
                    };
                });

                return renderSnippet(itemsCellSnippet, row.getValue("items"));
            },
        },
        {
            accessorKey: "total",
            header: "Total",
            cell: ({ row }) => {
                const totalCellSnippet = createRawSnippet<
                    [Record<string, any>]
                >((getItems) => {
                    const items = getItems();
                    const total = Object.values(items || {})
                        .filter((item) => !!item)
                        .reduce(
                            (sum: number, item: any) =>
                                sum + (item.price || 0) * (item.quantity || 0),
                            0,
                        );

                    const formatted = new Intl.NumberFormat("en-US", {
                        style: "currency",
                        currency: "DZD",
                    }).format(total);

                    return {
                        render: () =>
                            `<div class="font-medium text-foreground">${formatted}</div>`,
                    };
                });

                return renderSnippet(totalCellSnippet, row.getValue("items"));
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

    // Debounced search function
    const debouncedSearch = debounce((query: string) => {
        page = 1;
        fetchData();
    }, 300);

    // Fetch data function
    async function fetchData() {
        status = undefined;
        isSearching = !!searchQuery;

        try {
            // TODO: Replace with actual API call that supports filtering
            // This is a placeholder - you'll need to implement the actual API call
            const filters = {
                search: searchQuery.trim() || undefined,
                status: getStatusFilter(currentTab),
                province: selectedProvinces || undefined,
                delivery: selectedDeliveryType || undefined,
                date_from: dateRange.from || undefined,
                date_to: dateRange.to || undefined,
            };

            // Remove undefined values
            const cleanFilters = Object.fromEntries(
                Object.entries(filters).filter(
                    ([_, value]) => value !== undefined,
                ),
            );

            console.log("Fetching with filters:", cleanFilters);
            console.log("Pagination:", { page, per_page });

            // Placeholder API call - replace with your actual implementation
            const res = await ApiRoutes.get_all_orders({
                page,
                per_page,
                next_token: null,
                // TODO: Add filter parameters here
                // ...cleanFilters
            });

            console.log("API Response:", res);

            data = res.data;
            total_pages = Math.ceil((res.total ?? 1) / (res.per_page ?? 1));
            total_count = res.total ?? 0;
            status = null;
        } catch (err: any) {
            console.error("Error fetching orders:", err);
            status = err;
        } finally {
            isSearching = false;
        }
    }

    // Get status filter for API
    function getStatusFilter(tab: string): OrderStatus[] | undefined {
        switch (tab) {
            case "pending":
                return ["PENDING"];
            case "confirmed":
                return ["CONFIRMED"];
            case "delivered":
                return ["DELIVERED", "DONE"];
            case "canceled":
                return ["CANCELED", "RETURNED"];
            default:
                return undefined;
        }
    }

    // Handle search
    function handleSearch() {
        if (searchQuery.trim()) {
            debouncedSearch(searchQuery.trim());
        } else {
            page = 1;
            fetchData();
        }
    }

    // Handle search input keydown
    function handleSearchKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            handleSearch();
        }
    }

    // Clear search
    function clearSearch() {
        searchQuery = "";
        page = 1;
        fetchData();
    }

    // Handle tab change
    function handleTabChange(newTab: string) {
        if (newTab !== currentTab) {
            currentTab = newTab;
            page = 1;
            fetchData();
        }
    }

    // Handle filter changes
    function handleFilterChange() {
        page = 1;
        fetchData();
    }

    // Clear all filters
    function clearAllFilters() {
        searchQuery = "";
        currentTab = "all";
        selectedProvinces = [];
        selectedDeliveryType = "";
        dateRange = { from: "", to: "" };
        page = 1;
        fetchData();
    }

    // Refresh data
    function refreshData() {
        fetchData();
    }

    // Export data (placeholder)
    function exportData() {
        // TODO: Implement export functionality
        console.log("Export data functionality - to be implemented");
    }

    // Reactive statements
    $: if (page > 1) {
        fetchData();
    }

    // Initial load
    $: if (typeof window !== "undefined") {
        fetchData();
    }

    // Count active filters
    $: activeFiltersCount = [
        searchQuery.trim(),
        currentTab !== "all" ? currentTab : "",
        selectedProvinces.length,
        selectedDeliveryType,
        dateRange.from,
        dateRange.to,
    ].filter(Boolean).length;

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
                        {#if total_count > 0}
                            {total_count} total orders
                        {:else}
                            Manage customer orders and track their status
                        {/if}
                    </p>
                </div>
            </div>
            <div class="flex items-center gap-2">
                <Button
                    variant="outline"
                    size="sm"
                    onclick={exportData}
                    class="gap-2"
                >
                    <DownloadIcon class="h-4 w-4" />
                    Export
                </Button>
                <Button
                    variant="outline"
                    size="sm"
                    onclick={refreshData}
                    class="gap-2"
                >
                    <RefreshCcwIcon class="h-4 w-4" />
                    Refresh
                </Button>
                <a href="/orders/create" use:link>
                    <Button class="gap-2">
                        <PlusIcon class="h-4 w-4" />
                        New Order
                    </Button>
                </a>
            </div>
        </div>
    </div>

    <!-- Search and Filters -->
    <div class="px-6 space-y-4">
        <!-- Search Bar -->
        <div class="flex items-center gap-2">
            <div class="relative flex-1 max-w-md">
                <SearchIcon
                    class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
                />
                <Input
                    bind:ref={searchInput}
                    bind:value={searchQuery}
                    placeholder="Search orders, customers, phone numbers..."
                    class="pl-10 pr-10"
                    onkeydown={handleSearchKeydown}
                />
                {#if searchQuery}
                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={clearSearch}
                        class="absolute right-1 top-1/2 h-6 w-6 -translate-y-1/2 p-0"
                    >
                        <XIcon class="h-4 w-4" />
                    </Button>
                {/if}
            </div>
            <Button onclick={handleSearch} class="gap-2" disabled={isSearching}>
                <SearchIcon class="h-4 w-4" />
                Search
            </Button>
            <Button
                variant="outline"
                onclick={() => (showAdvancedFilters = !showAdvancedFilters)}
                class="gap-2"
            >
                <FilterIcon class="h-4 w-4" />
                Filters
                {#if activeFiltersCount > 0}
                    <Badge
                        variant="secondary"
                        class="ml-1 h-5 w-5 rounded-full p-0 text-xs"
                    >
                        {activeFiltersCount}
                    </Badge>
                {/if}
            </Button>
        </div>

        <!-- Advanced Filters -->
        {#if showAdvancedFilters}
            <Card.Root>
                <Card.Header>
                    <Card.Title class="text-lg">Advanced Filters</Card.Title>
                </Card.Header>
                <Card.Content>
                    <div
                        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4"
                    >
                        <Select.Root
                            type="multiple"
                            bind:value={selectedProvinces}
                        >
                            <Select.Trigger>Provinces...</Select.Trigger>
                            <Select.Content>
                                <Select.Item value="">All provinces</Select.Item
                                >
                                {#each provinces as province}
                                    <Select.Item value={province}
                                        >{province}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>

                        <!-- Delivery Type Filter -->
                        <Select.Root
                            type="single"
                            bind:value={selectedDeliveryType}
                        >
                            <Select.Trigger>Delivery Type...</Select.Trigger>
                            <Select.Content>
                                <Select.Item value="">All types</Select.Item>
                                {#each deliveryTypes as type}
                                    <Select.Item value={type.value}
                                        >{type.label}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>
                        <Input
                            type="date"
                            bind:value={dateRange.from}
                            onchange={handleFilterChange}
                            placeholder="From Date"
                        />
                        <Input
                            type="date"
                            bind:value={dateRange.to}
                            onchange={handleFilterChange}
                            placeholder="To Date"
                        />
                    </div>

                    <div
                        class="flex items-center justify-between mt-4 pt-4 border-t"
                    >
                        <div class="text-sm text-muted-foreground">
                            {activeFiltersCount} filter{activeFiltersCount !== 1
                                ? "s"
                                : ""} applied
                        </div>
                        <div class="flex gap-2">
                            <Button
                                variant="outline"
                                size="sm"
                                onclick={clearAllFilters}
                            >
                                Clear All
                            </Button>
                            <Button size="sm" onclick={handleFilterChange}>
                                Apply Filters
                            </Button>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        {/if}

        <!-- Status Tabs -->
        <Tabs.Root value={currentTab} onValueChange={handleTabChange}>
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
                    <div class="rounded-md border">
                        <div class="p-4">
                            {#each Array(per_page) as _}
                                <div class="flex items-center space-x-4 py-3">
                                    <Skeleton class="h-4 w-20" />
                                    <Skeleton class="h-4 w-32" />
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-4 w-16" />
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
                    {data}
                    bind:per_page
                    bind:page
                    bind:total_pages
                />
            {/if}

            {#snippet error()}
                <div
                    class="flex flex-col items-center justify-center py-12 text-center"
                >
                    <ShoppingCartIcon
                        class="h-12 w-12 text-muted-foreground mb-4"
                    />
                    <h3 class="text-lg font-semibold mb-2">No orders found</h3>
                    {#if searchQuery || currentTab !== "all" || selectedProvinces || selectedDeliveryType || dateRange.from || dateRange.to}
                        <p class="text-muted-foreground mb-4">
                            No orders match your current filters.
                        </p>
                        <Button variant="outline" onclick={clearAllFilters}>
                            Clear filters
                        </Button>
                    {:else}
                        <p class="text-muted-foreground mb-4">
                            Get started by creating your first order.
                        </p>
                        <a href="/orders/create" use:link>
                            <Button class="gap-2">
                                <PlusIcon class="h-4 w-4" />
                                Create Order
                            </Button>
                        </a>
                    {/if}
                </div>
            {/snippet}
        </StatusBoundary>
    </div>
</div>
