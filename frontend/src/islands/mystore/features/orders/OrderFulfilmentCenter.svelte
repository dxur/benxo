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
    import * as Dialog from "@/lib/components/ui/dialog/index";
    import { Checkbox } from "@/lib/components/ui/checkbox/index";
    import { Textarea } from "@/lib/components/ui/textarea/index";
    import * as Tooltip from "@/lib/components/ui/tooltip/index";
    import { Alert, AlertDescription } from "@/lib/components/ui/alert/index";

    import PackageIcon from "@lucide/svelte/icons/package";
    import TruckIcon from "@lucide/svelte/icons/truck";
    import SearchIcon from "@lucide/svelte/icons/search";
    import FilterIcon from "@lucide/svelte/icons/filter";
    import RefreshCcwIcon from "@lucide/svelte/icons/refresh-ccw";
    import DownloadIcon from "@lucide/svelte/icons/download";
    import XIcon from "@lucide/svelte/icons/x";
    import CheckIcon from "@lucide/svelte/icons/check";
    import PackageCheckIcon from "@lucide/svelte/icons/package-check";
    import ClockIcon from "@lucide/svelte/icons/clock";
    import AlertCircleIcon from "@lucide/svelte/icons/alert-circle";
    import BarcodeIcon from "@lucide/svelte/icons/barcode";
    import PrinterIcon from "@lucide/svelte/icons/printer";
    import SendIcon from "@lucide/svelte/icons/send";
    import CalendarIcon from "@lucide/svelte/icons/calendar";
    import MapPinIcon from "@lucide/svelte/icons/map-pin";

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

    // Types
    interface FulfillmentOrder extends OrderPublic {
        fulfillment_status:
            | "pending"
            | "processing"
            | "ready"
            | "shipped"
            | "delivered";
        tracking_number?: string;
        shipping_method?: string;
        estimated_delivery?: string;
        warehouse_location?: string;
        picker_assigned?: string;
        priority: "low" | "normal" | "high" | "urgent";
        fulfillment_notes?: string;
        packed_at?: string;
        shipped_at?: string;
    }

    interface BulkFulfillmentData {
        shipping_method: string;
        tracking_numbers: string;
        estimated_delivery: string;
        fulfillment_notes: string;
        notify_customer: boolean;
    }

    // Data and state
    let data: FulfillmentOrder[] = [];
    let per_page: number = 10;
    let page: number = 1;
    let total_pages: number = 1;
    let total_count: number = 0;
    let status: LoadingStatus = undefined;

    // Search and filtering
    let searchQuery: string = "";
    let searchInput: HTMLInputElement | null = null;
    let isSearching: boolean = false;
    let currentTab: string = "pending";
    let selectedProvinces: string[] = [];
    let selectedWarehouse: string = "";
    let selectedShippingMethod: string = "";
    let selectedPriority: string = "";
    let dateRange: { from: string; to: string } = { from: "", to: "" };
    let showAdvancedFilters: boolean = false;

    // Bulk operations
    let selectedOrders: string[] = [];
    let showBulkDialog: boolean = false;
    let bulkOperation: string = "";
    let isProcessingBulk: boolean = false;

    // Fulfillment data
    let fulfillmentData: BulkFulfillmentData = {
        shipping_method: "",
        tracking_numbers: "",
        estimated_delivery: "",
        fulfillment_notes: "",
        notify_customer: true,
    };

    // Constants
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
        "Béni Abbès",
        "In Salah",
        "In Guezzam",
        "Touggourt",
        "Djanet",
        "El M'Ghair",
        "El Meniaa",
    ];

    const warehouses = [
        { value: "WH001", label: "Algiers Main Warehouse" },
        { value: "WH002", label: "Oran Distribution Center" },
        { value: "WH003", label: "Constantine Hub" },
        { value: "WH004", label: "Annaba Facility" },
    ];

    const shippingMethods = [
        { value: "standard", label: "Standard Delivery (3-5 days)" },
        { value: "express", label: "Express Delivery (1-2 days)" },
        { value: "overnight", label: "Overnight Delivery" },
        { value: "pickup", label: "Pickup Point" },
    ];

    const priorityLevels = [
        { value: "low", label: "Low Priority" },
        { value: "normal", label: "Normal Priority" },
        { value: "high", label: "High Priority" },
        { value: "urgent", label: "Urgent" },
    ];

    const bulkOperations = [
        { value: "mark_ready", label: "Mark as Ready to Ship" },
        { value: "assign_shipping", label: "Assign Shipping Method" },
        { value: "generate_labels", label: "Generate Shipping Labels" },
        { value: "mark_shipped", label: "Mark as Shipped" },
        { value: "update_priority", label: "Update Priority" },
    ];

    // Table columns
    const columns: ColumnDef<FulfillmentOrder>[] = [
        {
            id: "select",
            header: ({ table }) => {
                const selectAllSnippet = createRawSnippet<[boolean, Function]>(
                    (getIsAllSelected, getToggleAllRowsSelected) => {
                        const isAllSelected = getIsAllSelected();
                        const toggleAllRowsSelected =
                            getToggleAllRowsSelected();

                        return {
                            render: () => `
                                <input 
                                    type="checkbox" 
                                    ${isAllSelected ? "checked" : ""} 
                                    class="rounded border-gray-300 text-primary focus:ring-primary"
                                    onchange="window.toggleAllRows(this.checked)"
                                />
                            `,
                        };
                    },
                );

                // Store toggle function globally for access from HTML
                (window as any).toggleAllRows = (checked: boolean) => {
                    if (checked) {
                        selectedOrders = data.map((order) => order.id);
                    } else {
                        selectedOrders = [];
                    }
                };

                return renderSnippet(
                    selectAllSnippet,
                    selectedOrders.length === data.length && data.length > 0,
                    () => {},
                );
            },
            cell: ({ row }) => {
                const selectCellSnippet = createRawSnippet<[string]>(
                    (getOrderId) => {
                        const orderId = getOrderId();
                        const isSelected = selectedOrders.includes(orderId);

                        return {
                            render: () => `
                                <input 
                                    type="checkbox" 
                                    ${isSelected ? "checked" : ""} 
                                    class="rounded border-gray-300 text-primary focus:ring-primary"
                                    onchange="window.toggleOrderSelection('${orderId}', this.checked)"
                                />
                            `,
                        };
                    },
                );

                return renderSnippet(selectCellSnippet, row.original.id);
            },
        },
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
            accessorKey: "priority",
            header: "Priority",
            cell: ({ row }) => {
                const priorityCellSnippet = createRawSnippet<[string]>(
                    (getPriority) => {
                        const priority = getPriority();
                        let badgeClass = "";
                        let icon = "";

                        switch (priority) {
                            case "urgent":
                                badgeClass =
                                    "bg-red-100 text-red-800 ring-red-200";
                                icon = "🚨";
                                break;
                            case "high":
                                badgeClass =
                                    "bg-orange-100 text-orange-800 ring-orange-200";
                                icon = "⚡";
                                break;
                            case "normal":
                                badgeClass =
                                    "bg-blue-100 text-blue-800 ring-blue-200";
                                icon = "📦";
                                break;
                            case "low":
                                badgeClass =
                                    "bg-gray-100 text-gray-800 ring-gray-200";
                                icon = "🔹";
                                break;
                            default:
                                badgeClass =
                                    "bg-gray-100 text-gray-800 ring-gray-200";
                                icon = "📦";
                        }

                        return {
                            render: () =>
                                `<span class="inline-flex items-center gap-1 rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset ${badgeClass}">
                                    ${icon} ${priority.toUpperCase()}
                                </span>`,
                        };
                    },
                );

                return renderSnippet(
                    priorityCellSnippet,
                    row.original.priority || "normal",
                );
            },
        },
        {
            accessorKey: "full_name",
            header: "Customer",
            cell: ({ row }) => {
                const customerCellSnippet = createRawSnippet<[string, string]>(
                    (getFullName, getPhone) => {
                        const fullName = getFullName();
                        const phone = getPhone();
                        return {
                            render: () =>
                                `<div>
                                    <div class="font-medium text-foreground">${fullName}</div>
                                    <div class="text-xs text-muted-foreground">${phone}</div>
                                </div>`,
                        };
                    },
                );
                return renderSnippet(
                    customerCellSnippet,
                    row.getValue("full_name"),
                    row.getValue("phone"),
                );
            },
        },
        {
            accessorKey: "fulfillment_status",
            header: "Status",
            cell: ({ row }) => {
                const statusCellSnippet = createRawSnippet<[string]>(
                    (getStatus) => {
                        const status = getStatus();
                        let badgeClass = "";
                        let icon = "";

                        switch (status) {
                            case "pending":
                                badgeClass =
                                    "bg-yellow-50 text-yellow-700 ring-yellow-600/20";
                                icon = "⏳";
                                break;
                            case "processing":
                                badgeClass =
                                    "bg-blue-50 text-blue-700 ring-blue-600/20";
                                icon = "🔄";
                                break;
                            case "ready":
                                badgeClass =
                                    "bg-green-50 text-green-700 ring-green-600/20";
                                icon = "✅";
                                break;
                            case "shipped":
                                badgeClass =
                                    "bg-purple-50 text-purple-700 ring-purple-600/20";
                                icon = "🚚";
                                break;
                            case "delivered":
                                badgeClass =
                                    "bg-emerald-50 text-emerald-700 ring-emerald-600/20";
                                icon = "📦";
                                break;
                            default:
                                badgeClass =
                                    "bg-gray-50 text-gray-600 ring-gray-500/20";
                                icon = "❓";
                        }

                        return {
                            render: () =>
                                `<span class="inline-flex items-center gap-1 rounded-full px-2 py-1 text-xs font-medium ring-1 ring-inset ${badgeClass}">
                                    ${icon} ${status.toUpperCase()}
                                </span>`,
                        };
                    },
                );

                return renderSnippet(
                    statusCellSnippet,
                    row.original.fulfillment_status || "pending",
                );
            },
        },
        {
            accessorKey: "province",
            header: "Destination",
            cell: ({ row }) => {
                const destinationCellSnippet = createRawSnippet<
                    [string, string]
                >((getProvince, getDelivery) => {
                    const province = getProvince();
                    const delivery = getDelivery();
                    const deliveryType =
                        delivery === "DOMICIL" ? "Home" : "Pickup";

                    return {
                        render: () =>
                            `<div>
                                    <div class="font-medium text-sm">${province}</div>
                                    <div class="text-xs text-muted-foreground">${deliveryType}</div>
                                </div>`,
                    };
                });
                return renderSnippet(
                    destinationCellSnippet,
                    row.getValue("province"),
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
            accessorKey: "shipping_method",
            header: "Shipping",
            cell: ({ row }) => {
                const shippingCellSnippet = createRawSnippet<[string, string]>(
                    (getShippingMethod, getTrackingNumber) => {
                        const shippingMethod = getShippingMethod();
                        const trackingNumber = getTrackingNumber();

                        return {
                            render: () => {
                                if (shippingMethod) {
                                    return `<div>
                                        <div class="font-medium text-sm">${shippingMethod}</div>
                                        ${trackingNumber ? `<div class="text-xs text-muted-foreground font-mono">${trackingNumber}</div>` : ""}
                                    </div>`;
                                }
                                return `<span class="text-xs text-muted-foreground">Not assigned</span>`;
                            },
                        };
                    },
                );
                return renderSnippet(
                    shippingCellSnippet,
                    row.original.shipping_method || "",
                    row.original.tracking_number || "",
                );
            },
        },
        {
            accessorKey: "estimated_delivery",
            header: "Est. Delivery",
            cell: ({ row }) => {
                const deliveryCellSnippet = createRawSnippet<[string]>(
                    (getEstimatedDelivery) => {
                        const estimatedDelivery = getEstimatedDelivery();

                        if (estimatedDelivery) {
                            const date = new Date(estimatedDelivery);
                            const formattedDate = date.toLocaleDateString();
                            const isOverdue = date < new Date();

                            return {
                                render: () =>
                                    `<div class="text-sm ${isOverdue ? "text-red-600" : "text-foreground"}">
                                        ${formattedDate}
                                        ${isOverdue ? '<span class="text-xs block">Overdue</span>' : ""}
                                    </div>`,
                            };
                        }
                        return {
                            render: () =>
                                `<span class="text-xs text-muted-foreground">Not set</span>`,
                        };
                    },
                );
                return renderSnippet(
                    deliveryCellSnippet,
                    row.original.estimated_delivery || "",
                );
            },
        },
        {
            id: "actions",
            header: "Actions",
            cell: ({ row }) => {
                const actionsCellSnippet = createRawSnippet<[string]>(
                    (getOrderId) => {
                        const orderId = getOrderId();

                        return {
                            render: () =>
                                `<div class="flex items-center gap-1">
                                    <button onclick="window.quickFulfill('${orderId}')" class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-8 w-8">
                                        <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M9 12l2 2 4-4"/>
                                            <path d="M21 12c-1 0-3-1-3-3s2-3 3-3 3 1 3 3-2 3-3 3"/>
                                            <path d="M3 12c1 0 3-1 3-3s-2-3-3-3-3 1-3 3 2 3 3 3"/>
                                        </svg>
                                    </button>
                                    <button onclick="window.printLabel('${orderId}')" class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-8 w-8">
                                        <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <polyline points="6,9 6,2 18,2 18,9"/>
                                            <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/>
                                            <rect x="6" y="14" width="12" height="8"/>
                                        </svg>
                                    </button>
                                </div>`,
                        };
                    },
                );

                return renderSnippet(actionsCellSnippet, row.original.id);
            },
        },
    ];

    // Global functions for table actions
    (window as any).toggleOrderSelection = (
        orderId: string,
        checked: boolean,
    ) => {
        if (checked) {
            selectedOrders = [...selectedOrders, orderId];
        } else {
            selectedOrders = selectedOrders.filter((id) => id !== orderId);
        }
    };

    (window as any).quickFulfill = (orderId: string) => {
        // TODO: Implement quick fulfillment
        console.log("Quick fulfill order:", orderId);
    };

    (window as any).printLabel = (orderId: string) => {
        // TODO: Implement label printing
        console.log("Print label for order:", orderId);
    };

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
            const filters = {
                search: searchQuery.trim() || undefined,
                fulfillment_status: getFulfillmentStatusFilter(currentTab),
                province: selectedProvinces.length
                    ? selectedProvinces
                    : undefined,
                warehouse: selectedWarehouse || undefined,
                shipping_method: selectedShippingMethod || undefined,
                priority: selectedPriority || undefined,
                date_from: dateRange.from || undefined,
                date_to: dateRange.to || undefined,
            };

            const cleanFilters = Object.fromEntries(
                Object.entries(filters).filter(
                    ([_, value]) => value !== undefined,
                ),
            );

            console.log(
                "Fetching fulfillment data with filters:",
                cleanFilters,
            );

            // TODO: Replace with actual fulfillment API call
            const res = await ApiRoutes.get_all_orders({
                page,
                per_page,
                next_token: null,
                ...cleanFilters,
            });

            data = res.data;
            total_pages = Math.ceil(50 / per_page); // Mock total
            total_count = 50; // Mock total
            status = null;
        } catch (err: any) {
            console.error("Error fetching fulfillment data:", err);
            status = err;
        } finally {
            isSearching = false;
        }
    }

    // Get fulfillment status filter
    function getFulfillmentStatusFilter(tab: string): string[] | undefined {
        switch (tab) {
            case "pending":
                return ["pending"];
            case "processing":
                return ["processing"];
            case "ready":
                return ["ready"];
            case "shipped":
                return ["shipped"];
            case "delivered":
                return ["delivered"];
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
        currentTab = "pending";
        selectedProvinces = [];
        selectedWarehouse = "";
        selectedShippingMethod = "";
        selectedPriority = "";
        dateRange = { from: "", to: "" };
        page = 1;
        fetchData();
    }

    // Bulk operations
    function handleBulkOperation(operation: string) {
        if (selectedOrders.length === 0) {
            alert("Please select orders to perform bulk operations.");
            return;
        }

        bulkOperation = operation;
        showBulkDialog = true;
    }

    async function processBulkOperation() {
        if (selectedOrders.length === 0) return;

        isProcessingBulk = true;

        try {
            switch (bulkOperation) {
                case "mark_ready":
                    // TODO: Implement mark as ready
                    console.log("Mark as ready:", selectedOrders);
                    break;
                case "assign_shipping":
                    // TODO: Implement assign shipping
                    console.log(
                        "Assign shipping:",
                        selectedOrders,
                        fulfillmentData,
                    );
                    break;
                case "generate_labels":
                    // TODO: Implement generate labels
                    console.log("Generate labels:", selectedOrders);
                    break;
                case "mark_shipped":
                    // TODO: Implement mark as shipped
                    console.log(
                        "Mark as shipped:",
                        selectedOrders,
                        fulfillmentData,
                    );
                    break;
                case "update_priority":
                    // TODO: Implement update priority
                    console.log("Update priority:", selectedOrders);
                    break;
            }

            showBulkDialog = false;
            selectedOrders = [];
            fetchData();
        } catch (error) {
            console.error("Bulk operation failed:", error);
        } finally {
            isProcessingBulk = false;
        }
    }

    // Refresh data
    function refreshData() {
        fetchData();
    }

    // Export data
    function exportData() {
        console.log("Export fulfillment data");
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
        currentTab !== "pending" ? currentTab : "",
        selectedProvinces.length,
        selectedWarehouse,
        selectedShippingMethod,
        selectedPriority,
        dateRange.from,
        dateRange.to,
    ].filter(Boolean).length;

    $: document.title = "Fulfillment Center";
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
                    <h1 class="text-3xl font-bold tracking-tight">
                        Fulfillment Center
                    </h1>
                    <p class="text-muted-foreground">
                        {#if total_count > 0}
                            {total_count} orders pending fulfillment • {selectedOrders.length}
                            selected
                        {:else}
                            Process and ship customer orders
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
                {#if selectedOrders.length > 0}
                    <Button
                        variant="default"
                        size="sm"
                        onclick={() => handleBulkOperation("mark_ready")}
                        class="gap-2"
                    >
                        <PackageCheckIcon class="h-4 w-4" />
                        Mark Ready ({selectedOrders.length})
                    </Button>
                {/if}
            </div>
        </div>

        <!-- Quick Stats -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <Card.Root>
                <Card.Content class="p-4">
                    <div class="flex items-center gap-2">
                        <ClockIcon class="h-4 w-4 text-yellow-600" />
                        <div>
                            <p class="text-sm font-medium">Pending</p>
                            <p class="text-2xl font-bold">24</p>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
            <Card.Root>
                <Card.Content class="p-4">
                    <div class="flex items-center gap-2">
                        <PackageIcon class="h-4 w-4 text-blue-600" />
                        <div>
                            <p class="text-sm font-medium">Processing</p>
                            <p class="text-2xl font-bold">12</p>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
            <Card.Root>
                <Card.Content class="p-4">
                    <div class="flex items-center gap-2">
                        <CheckIcon class="h-4 w-4 text-green-600" />
                        <div>
                            <p class="text-sm font-medium">Ready</p>
                            <p class="text-2xl font-bold">8</p>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
            <Card.Root>
                <Card.Content class="p-4">
                    <div class="flex items-center gap-2">
                        <TruckIcon class="h-4 w-4 text-purple-600" />
                        <div>
                            <p class="text-sm font-medium">Shipped</p>
                            <p class="text-2xl font-bold">156</p>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        </div>
    </div>

    <!-- Bulk Actions Bar -->
    {#if selectedOrders.length > 0}
        <div class="px-6">
            <Alert>
                <CheckIcon class="h-4 w-4" />
                <AlertDescription>
                    <div class="flex items-center justify-between">
                        <span
                            >{selectedOrders.length} order{selectedOrders.length !==
                            1
                                ? "s"
                                : ""} selected</span
                        >
                        <div class="flex items-center gap-2">
                            <Button
                                size="sm"
                                variant="outline"
                                onclick={() =>
                                    handleBulkOperation("assign_shipping")}
                                class="gap-2"
                            >
                                <TruckIcon class="h-3 w-3" />
                                Assign Shipping
                            </Button>
                            <Button
                                size="sm"
                                variant="outline"
                                onclick={() =>
                                    handleBulkOperation("generate_labels")}
                                class="gap-2"
                            >
                                <PrinterIcon class="h-3 w-3" />
                                Generate Labels
                            </Button>
                            <Button
                                size="sm"
                                variant="outline"
                                onclick={() =>
                                    handleBulkOperation("mark_shipped")}
                                class="gap-2"
                            >
                                <SendIcon class="h-3 w-3" />
                                Mark Shipped
                            </Button>
                            <Button
                                size="sm"
                                variant="ghost"
                                onclick={() => (selectedOrders = [])}
                            >
                                <XIcon class="h-3 w-3" />
                            </Button>
                        </div>
                    </div>
                </AlertDescription>
            </Alert>
        </div>
    {/if}

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
                    placeholder="Search orders, customers, tracking numbers..."
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
                        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4"
                    >
                        <!-- Provinces Filter -->
                        <Select.Root
                            type="multiple"
                            bind:value={selectedProvinces}
                        >
                            <Select.Trigger>
                                {selectedProvinces.length > 0
                                    ? `${selectedProvinces.length} provinces`
                                    : "All provinces"}
                            </Select.Trigger>
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

                        <!-- Warehouse Filter -->
                        <Select.Root
                            type="single"
                            bind:value={selectedWarehouse}
                        >
                            <Select.Trigger>
                                {selectedWarehouse
                                    ? warehouses.find(
                                          (w) => w.value === selectedWarehouse,
                                      )?.label
                                    : "All warehouses"}
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value=""
                                    >All warehouses</Select.Item
                                >
                                {#each warehouses as warehouse}
                                    <Select.Item value={warehouse.value}
                                        >{warehouse.label}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>

                        <!-- Shipping Method Filter -->
                        <Select.Root
                            type="single"
                            bind:value={selectedShippingMethod}
                        >
                            <Select.Trigger>
                                {selectedShippingMethod
                                    ? shippingMethods.find(
                                          (s) =>
                                              s.value ===
                                              selectedShippingMethod,
                                      )?.label
                                    : "All shipping methods"}
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value=""
                                    >All shipping methods</Select.Item
                                >
                                {#each shippingMethods as method}
                                    <Select.Item value={method.value}
                                        >{method.label}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>

                        <!-- Priority Filter -->
                        <Select.Root
                            type="single"
                            bind:value={selectedPriority}
                        >
                            <Select.Trigger>
                                {selectedPriority
                                    ? priorityLevels.find(
                                          (p) => p.value === selectedPriority,
                                      )?.label
                                    : "All priorities"}
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value=""
                                    >All priorities</Select.Item
                                >
                                {#each priorityLevels as priority}
                                    <Select.Item value={priority.value}
                                        >{priority.label}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>

                        <!-- Date Range -->
                        <div class="flex gap-2">
                            <Input
                                type="date"
                                bind:value={dateRange.from}
                                onchange={handleFilterChange}
                                placeholder="From Date"
                                class="flex-1"
                            />
                            <Input
                                type="date"
                                bind:value={dateRange.to}
                                onchange={handleFilterChange}
                                placeholder="To Date"
                                class="flex-1"
                            />
                        </div>
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
                <Tabs.Trigger value="pending">Pending</Tabs.Trigger>
                <Tabs.Trigger value="processing">Processing</Tabs.Trigger>
                <Tabs.Trigger value="ready">Ready to Ship</Tabs.Trigger>
                <Tabs.Trigger value="shipped">Shipped</Tabs.Trigger>
                <Tabs.Trigger value="delivered">Delivered</Tabs.Trigger>
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
                                    <Skeleton class="h-4 w-4" />
                                    <Skeleton class="h-4 w-20" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-4 w-20" />
                                    <Skeleton class="h-4 w-16" />
                                    <Skeleton class="h-8 w-16" />
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
                    <PackageIcon class="h-12 w-12 text-muted-foreground mb-4" />
                    <h3 class="text-lg font-semibold mb-2">No orders found</h3>
                    {#if searchQuery || currentTab !== "pending" || selectedProvinces.length || selectedWarehouse || selectedShippingMethod || selectedPriority || dateRange.from || dateRange.to}
                        <p class="text-muted-foreground mb-4">
                            No orders match your current filters.
                        </p>
                        <Button variant="outline" onclick={clearAllFilters}>
                            Clear filters
                        </Button>
                    {:else}
                        <p class="text-muted-foreground mb-4">
                            No orders pending fulfillment.
                        </p>
                        <a href="/orders" use:link>
                            <Button class="gap-2">
                                <PackageIcon class="h-4 w-4" />
                                View All Orders
                            </Button>
                        </a>
                    {/if}
                </div>
            {/snippet}
        </StatusBoundary>
    </div>
</div>

<!-- Bulk Operation Dialog -->
<Dialog.Root bind:open={showBulkDialog}>
    <Dialog.Content class="max-w-2xl">
        <Dialog.Header>
            <Dialog.Title>
                {#if bulkOperation === "assign_shipping"}
                    Assign Shipping Method
                {:else if bulkOperation === "mark_shipped"}
                    Mark Orders as Shipped
                {:else if bulkOperation === "generate_labels"}
                    Generate Shipping Labels
                {:else if bulkOperation === "update_priority"}
                    Update Priority
                {:else}
                    Bulk Operation
                {/if}
            </Dialog.Title>
            <Dialog.Description>
                Processing {selectedOrders.length} selected order{selectedOrders.length !==
                1
                    ? "s"
                    : ""}
            </Dialog.Description>
        </Dialog.Header>

        <div class="space-y-4">
            {#if bulkOperation === "assign_shipping" || bulkOperation === "mark_shipped"}
                <div class="space-y-4">
                    <div>
                        <label class="text-sm font-medium mb-2 block"
                            >Shipping Method</label
                        >
                        <Select.Root
                            type="single"
                            bind:value={fulfillmentData.shipping_method}
                        >
                            <Select.Trigger>
                                {fulfillmentData.shipping_method ||
                                    "Select method"}
                            </Select.Trigger>
                            <Select.Content>
                                {#each shippingMethods as method}
                                    <Select.Item value={method.value}
                                        >{method.label}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>
                    </div>

                    {#if bulkOperation === "mark_shipped"}
                        <div>
                            <label class="text-sm font-medium mb-2 block"
                                >Tracking Numbers</label
                            >
                            <Textarea
                                bind:value={fulfillmentData.tracking_numbers}
                                placeholder="Enter tracking numbers (one per line)"
                                rows="4"
                            />
                            <p class="text-xs text-muted-foreground mt-1">
                                Enter one tracking number per line in the same
                                order as the selected orders
                            </p>
                        </div>
                    {/if}

                    <div>
                        <label class="text-sm font-medium mb-2 block"
                            >Estimated Delivery Date</label
                        >
                        <Input
                            type="date"
                            bind:value={fulfillmentData.estimated_delivery}
                        />
                    </div>

                    <div>
                        <label class="text-sm font-medium mb-2 block"
                            >Fulfillment Notes</label
                        >
                        <Textarea
                            bind:value={fulfillmentData.fulfillment_notes}
                            placeholder="Optional notes about this fulfillment..."
                            rows="3"
                        />
                    </div>

                    <div class="flex items-center space-x-2">
                        <Checkbox
                            bind:checked={fulfillmentData.notify_customer}
                            id="notify_customer"
                        />
                        <label
                            for="notify_customer"
                            class="text-sm font-medium"
                        >
                            Notify customers via email/SMS
                        </label>
                    </div>
                </div>
            {:else if bulkOperation === "generate_labels"}
                <div class="space-y-4">
                    <Alert>
                        <PrinterIcon class="h-4 w-4" />
                        <AlertDescription>
                            This will generate shipping labels for all selected
                            orders. Make sure your printer is ready.
                        </AlertDescription>
                    </Alert>

                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="text-sm font-medium mb-2 block"
                                >Label Format</label
                            >
                            <Select.Root type="single" value="4x6">
                                <Select.Trigger>4x6 inches</Select.Trigger>
                                <Select.Content>
                                    <Select.Item value="4x6"
                                        >4x6 inches</Select.Item
                                    >
                                    <Select.Item value="4x8"
                                        >4x8 inches</Select.Item
                                    >
                                    <Select.Item value="a4"
                                        >A4 Paper</Select.Item
                                    >
                                </Select.Content>
                            </Select.Root>
                        </div>
                        <div>
                            <label class="text-sm font-medium mb-2 block"
                                >Include</label
                            >
                            <div class="space-y-2">
                                <div class="flex items-center space-x-2">
                                    <Checkbox checked id="include_barcode" />
                                    <label for="include_barcode" class="text-sm"
                                        >Barcode</label
                                    >
                                </div>
                                <div class="flex items-center space-x-2">
                                    <Checkbox checked id="include_return" />
                                    <label for="include_return" class="text-sm"
                                        >Return Address</label
                                    >
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            {:else if bulkOperation === "update_priority"}
                <div>
                    <label class="text-sm font-medium mb-2 block"
                        >New Priority Level</label
                    >
                    <Select.Root type="single">
                        <Select.Trigger>Select priority</Select.Trigger>
                        <Select.Content>
                            {#each priorityLevels as priority}
                                <Select.Item value={priority.value}
                                    >{priority.label}</Select.Item
                                >
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            {:else}
                <div class="space-y-4">
                    <Alert>
                        <AlertCircleIcon class="h-4 w-4" />
                        <AlertDescription>
                            This will mark all selected orders as ready to ship.
                            Are you sure?
                        </AlertDescription>
                    </Alert>
                </div>
            {/if}
        </div>

        <Dialog.Footer>
            <Button
                variant="outline"
                onclick={() => (showBulkDialog = false)}
                disabled={isProcessingBulk}
            >
                Cancel
            </Button>
            <Button
                onclick={processBulkOperation}
                disabled={isProcessingBulk}
                class="gap-2"
            >
                {#if isProcessingBulk}
                    <RefreshCcwIcon class="h-4 w-4 animate-spin" />
                    Processing...
                {:else}
                    <CheckIcon class="h-4 w-4" />
                    Confirm
                {/if}
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
