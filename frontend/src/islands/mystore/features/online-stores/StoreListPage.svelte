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
    import StoreIcon from "@lucide/svelte/icons/store";
    import SearchIcon from "@lucide/svelte/icons/search";
    import FilterIcon from "@lucide/svelte/icons/filter";
    import RefreshCcwIcon from "@lucide/svelte/icons/refresh-ccw";
    import DownloadIcon from "@lucide/svelte/icons/download";
    import XIcon from "@lucide/svelte/icons/x";
    import GlobeIcon from "@lucide/svelte/icons/globe";
    import EyeIcon from "@lucide/svelte/icons/eye";
    import EyeOffIcon from "@lucide/svelte/icons/eye-off";

    import { link } from "@dvcol/svelte-simple-router";
    import { debounce } from "lodash";

    import type { ColumnDef } from "@tanstack/table-core";
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
    import StoreCellActions from "./StoreCellActions.svelte";

    // Store interface (you'll need to create this type)
    interface StorePublic {
        id: string;
        name: string;
        slug: string;
        description: string;
        status: "ACTIVE" | "INACTIVE" | "DRAFT" | "SUSPENDED";
        template: string;
        domain: string;
        custom_domains: string[];
        created_at: string;
        updated_at: string;
        orders_count: number;
        products_count: number;
        owner_name: string;
        owner_email: string;
    }

    // Data and state
    let data: StorePublic[] = [];
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
    let selectedTemplate: string = "";
    let dateRange: { from: string; to: string } = { from: "", to: "" };
    let showAdvancedFilters: boolean = false;

    const templates = [
        { value: "minimal", label: "Minimal" },
        { value: "classic", label: "Classic" },
        { value: "modern", label: "Modern" },
        { value: "fashion", label: "Fashion" },
        { value: "electronics", label: "Electronics" },
        { value: "food", label: "Food & Beverage" },
    ];

    // Table columns
    const columns: ColumnDef<StorePublic>[] = [
        {
            accessorKey: "name",
            header: "Store Name",
            cell: ({ row }) => {
                const nameCellSnippet = createRawSnippet<[string, string]>(
                    (getName, getSlug) => {
                        const name = getName();
                        const slug = getSlug();
                        return {
                            render: () =>
                                `<div class="flex items-center gap-3">
                                <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/10">
                                    <svg class="h-4 w-4 text-primary" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M3 21h18l-9-18-9 18Z"/>
                                        <path d="M12 3v18"/>
                                    </svg>
                                </div>
                                <div>
                                    <div class="font-medium text-foreground">${name}</div>
                                    <div class="text-xs text-muted-foreground">@${slug}</div>
                                </div>
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    nameCellSnippet,
                    row.getValue("name"),
                    row.getValue("slug"),
                );
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
                        let icon = "";

                        switch (status) {
                            case "ACTIVE":
                                badgeClass =
                                    "bg-green-50 text-green-700 ring-green-600/20";
                                icon = `<svg class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 12l2 2 4-4"/>
                                <circle cx="12" cy="12" r="10"/>
                            </svg>`;
                                break;
                            case "INACTIVE":
                                badgeClass =
                                    "bg-gray-50 text-gray-700 ring-gray-600/20";
                                icon = `<svg class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"/>
                                <path d="M8 12h8"/>
                            </svg>`;
                                break;
                            case "DRAFT":
                                badgeClass =
                                    "bg-yellow-50 text-yellow-700 ring-yellow-600/20";
                                icon = `<svg class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 2v20M2 12h20"/>
                            </svg>`;
                                break;
                            case "SUSPENDED":
                                badgeClass =
                                    "bg-red-50 text-red-700 ring-red-600/20";
                                icon = `<svg class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"/>
                                <path d="M15 9l-6 6M9 9l6 6"/>
                            </svg>`;
                                break;
                            default:
                                badgeClass =
                                    "bg-gray-50 text-gray-600 ring-gray-500/20";
                                icon = "";
                        }

                        return {
                            render: () =>
                                `<span class="inline-flex items-center gap-1 rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset ${badgeClass}">
                                ${icon}
                                ${status}
                            </span>`,
                        };
                    },
                );

                return renderSnippet(statusCellSnippet, row.getValue("status"));
            },
        },
        {
            accessorKey: "template",
            header: "Template",
            cell: ({ row }) => {
                const templateCellSnippet = createRawSnippet<[string]>(
                    (getTemplate) => {
                        const template = getTemplate();
                        const templateLabel =
                            templates.find((t) => t.value === template)
                                ?.label || template;
                        return {
                            render: () =>
                                `<div class="flex items-center gap-2">
                                <div class="h-4 w-4 rounded border bg-gradient-to-br from-blue-400 to-purple-500"></div>
                                <span class="text-sm font-medium">${templateLabel}</span>
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    templateCellSnippet,
                    row.getValue("template"),
                );
            },
        },
        {
            accessorKey: "domain",
            header: "Domain",
            cell: ({ row }) => {
                const domainCellSnippet = createRawSnippet<[string, string[]]>(
                    (getDomain, getCustomDomains) => {
                        const domain = getDomain();
                        const customDomains = getCustomDomains();
                        const hasCustomDomains =
                            customDomains && customDomains.length > 0;

                        return {
                            render: () =>
                                `<div class="space-y-1">
                                <div class="flex items-center gap-1 text-sm">
                                    <svg class="h-3 w-3 text-muted-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <circle cx="12" cy="12" r="10"/>
                                        <path d="M2 12h20"/>
                                        <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
                                    </svg>
                                    <span class="font-mono text-xs">${domain}</span>
                                </div>
                                ${hasCustomDomains ? `<div class="text-xs text-muted-foreground">+${customDomains.length} custom domain${customDomains.length !== 1 ? "s" : ""}</div>` : ""}
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    domainCellSnippet,
                    row.getValue("domain"),
                    row.getValue("custom_domains"),
                );
            },
        },
        {
            accessorKey: "owner_name",
            header: "Owner",
            cell: ({ row }) => {
                const ownerCellSnippet = createRawSnippet<[string, string]>(
                    (getOwnerName, getOwnerEmail) => {
                        const ownerName = getOwnerName();
                        const ownerEmail = getOwnerEmail();
                        return {
                            render: () =>
                                `<div>
                                <div class="font-medium text-foreground">${ownerName}</div>
                                <div class="text-xs text-muted-foreground">${ownerEmail}</div>
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    ownerCellSnippet,
                    row.getValue("owner_name"),
                    row.getValue("owner_email"),
                );
            },
        },
        {
            accessorKey: "orders_count",
            header: "Orders",
            cell: ({ row }) => {
                const ordersCellSnippet = createRawSnippet<[number]>(
                    (getOrdersCount) => {
                        const ordersCount = getOrdersCount();
                        return {
                            render: () =>
                                `<div class="text-center">
                                <div class="font-medium">${ordersCount}</div>
                                <div class="text-xs text-muted-foreground">orders</div>
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    ordersCellSnippet,
                    row.getValue("orders_count"),
                );
            },
        },
        {
            accessorKey: "products_count",
            header: "Products",
            cell: ({ row }) => {
                const productsCellSnippet = createRawSnippet<[number]>(
                    (getProductsCount) => {
                        const productsCount = getProductsCount();
                        return {
                            render: () =>
                                `<div class="text-center">
                                <div class="font-medium">${productsCount}</div>
                                <div class="text-xs text-muted-foreground">products</div>
                            </div>`,
                        };
                    },
                );
                return renderSnippet(
                    productsCellSnippet,
                    row.getValue("products_count"),
                );
            },
        },
        {
            accessorKey: "created_at",
            header: "Created",
            cell: ({ row }) => {
                const createdAtCellSnippet = createRawSnippet<[string]>(
                    (getCreatedAt) => {
                        const createdAt = getCreatedAt();
                        const date = new Date(createdAt).toLocaleDateString();
                        return {
                            render: () =>
                                `<div class="text-sm text-muted-foreground">${date}</div>`,
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
            id: "actions",
            header: "Actions",
            cell: ({ row }) => {
                return renderComponent(StoreCellActions, {
                    id: row.original.id,
                    status: row.original.status,
                    domain: row.original.domain,
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
            const filters = {
                search: searchQuery.trim() || undefined,
                status: getStatusFilter(currentTab),
                template: selectedTemplate || undefined,
                date_from: dateRange.from || undefined,
                date_to: dateRange.to || undefined,
            };

            // Remove undefined values
            const cleanFilters = Object.fromEntries(
                Object.entries(filters).filter(
                    ([_, value]) => value !== undefined,
                ),
            );

            console.log("Fetching stores with filters:", cleanFilters);
            console.log("Pagination:", { page, per_page });

            // Placeholder API call - replace with your actual implementation
            // const res = await ApiRoutes.get_all_stores({
            //     page,
            //     per_page,
            //     next_token: null,
            //     ...cleanFilters
            // });

            // Mock data for now
            const mockData: StorePublic[] = [
                {
                    id: "store-1",
                    name: "TechHub Store",
                    slug: "techhub-store",
                    description: "Electronics and gadgets store",
                    status: "ACTIVE",
                    template: "electronics",
                    domain: "techhub.kelle.store",
                    custom_domains: ["techhub.com"],
                    created_at: "2024-01-15T10:00:00Z",
                    updated_at: "2024-01-20T15:30:00Z",
                    orders_count: 156,
                    products_count: 89,
                    owner_name: "Ahmed Benali",
                    owner_email: "ahmed@techhub.com",
                },
                {
                    id: "store-2",
                    name: "Fashion Forward",
                    slug: "fashion-forward",
                    description: "Modern fashion and accessories",
                    status: "ACTIVE",
                    template: "fashion",
                    domain: "fashion.kelle.store",
                    custom_domains: [],
                    created_at: "2024-01-10T09:00:00Z",
                    updated_at: "2024-01-22T11:15:00Z",
                    orders_count: 89,
                    products_count: 234,
                    owner_name: "Fatima Zahra",
                    owner_email: "fatima@fashionforward.com",
                },
                {
                    id: "store-3",
                    name: "Coffee Corner",
                    slug: "coffee-corner",
                    description: "Premium coffee and beverages",
                    status: "DRAFT",
                    template: "food",
                    domain: "coffee.kelle.store",
                    custom_domains: [],
                    created_at: "2024-01-25T14:00:00Z",
                    updated_at: "2024-01-25T14:00:00Z",
                    orders_count: 0,
                    products_count: 15,
                    owner_name: "Omar Khoury",
                    owner_email: "omar@coffeecorner.com",
                },
            ];

            data = mockData;
            total_pages = Math.ceil(mockData.length / per_page);
            total_count = mockData.length;
            status = null;
        } catch (err: any) {
            console.error("Error fetching stores:", err);
            status = err;
        } finally {
            isSearching = false;
        }
    }

    // Get status filter for API
    function getStatusFilter(tab: string): string[] | undefined {
        switch (tab) {
            case "active":
                return ["ACTIVE"];
            case "inactive":
                return ["INACTIVE"];
            case "draft":
                return ["DRAFT"];
            case "suspended":
                return ["SUSPENDED"];
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
        selectedTemplate = "";
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
        console.log("Export stores data functionality - to be implemented");
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
        selectedTemplate,
        dateRange.from,
        dateRange.to,
    ].filter(Boolean).length;

    $: document.title = "Stores";
</script>

<div class="flex flex-1 flex-col space-y-6">
    <!-- Header -->
    <div class="flex flex-col gap-4 p-6 pb-0">
        <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
                <div
                    class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10"
                >
                    <StoreIcon class="h-5 w-5 text-primary" />
                </div>
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">Stores</h1>
                    <p class="text-muted-foreground">
                        {#if total_count > 0}
                            {total_count} total stores
                        {:else}
                            Create and manage your online stores
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
                <a href="/stores/create" use:link>
                    <Button class="gap-2">
                        <PlusIcon class="h-4 w-4" />
                        New Store
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
                    placeholder="Search stores, owners, domains..."
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
                        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
                    >
                        <!-- Template Filter -->
                        <Select.Root
                            type="single"
                            bind:value={selectedTemplate}
                        >
                            <Select.Trigger>Template...</Select.Trigger>
                            <Select.Content>
                                <Select.Item value="">All templates</Select.Item
                                >
                                {#each templates as template}
                                    <Select.Item value={template.value}
                                        >{template.label}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>

                        <!-- Date Range -->
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
                            <Button size="sm" onclick={handleFilterChange}
                                >Apply Filters</Button
                            >
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        {/if}

        <!-- Status Tabs -->
        <Tabs.Root value={currentTab} onValueChange={handleTabChange}>
            <Tabs.List>
                <Tabs.Trigger value="all">All Stores</Tabs.Trigger>
                <Tabs.Trigger value="active">Active</Tabs.Trigger>
                <Tabs.Trigger value="draft">Draft</Tabs.Trigger>
                <Tabs.Trigger value="inactive">Inactive</Tabs.Trigger>
                <Tabs.Trigger value="suspended">Suspended</Tabs.Trigger>
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
                                    <Skeleton class="h-4 w-32" />
                                    <Skeleton class="h-4 w-20" />
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-4 w-32" />
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-4 w-20" />
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
                    <StoreIcon class="h-12 w-12 text-muted-foreground mb-4" />
                    <h3 class="text-lg font-semibold mb-2">No stores found</h3>
                    {#if searchQuery || currentTab !== "all" || selectedTemplate || dateRange.from || dateRange.to}
                        <p class="text-muted-foreground mb-4">
                            No stores match your current filters.
                        </p>
                        <Button variant="outline" onclick={clearAllFilters}
                            >Clear filters</Button
                        >
                    {:else}
                        <p class="text-muted-foreground mb-4">
                            Get started by creating your first store.
                        </p>
                        <a href="/stores/create" use:link>
                            <Button class="gap-2">
                                <PlusIcon class="h-4 w-4" />
                                Create Store
                            </Button>
                        </a>
                    {/if}
                </div>
            {/snippet}
        </StatusBoundary>
    </div>
</div>
