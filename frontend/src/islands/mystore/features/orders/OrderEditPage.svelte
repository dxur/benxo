<script lang="ts">
    import { onMount } from "svelte";
    import { link, useNavigate, useRoute } from "@dvcol/svelte-simple-router";
    import { Routes } from "./index";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { notifCenter } from "@/stores/notifications";

    import type { OrderPublic } from "@bindings/OrderPublic";
    import type { OrderUpdateBody } from "@bindings/OrderUpdateBody";
    import type { OrderStatus } from "@bindings/OrderStatus";

    import EditPageLayout from "../../lib/components/EditPageLayout.svelte";
    import { Button } from "@/lib/components/ui/button/index.js";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import { Skeleton } from "@/lib/components/ui/skeleton/index.js";
    import { Input } from "@/lib/components/ui/input/index.js";
    import { Label } from "@/lib/components/ui/label/index.js";
    import { Textarea } from "@/lib/components/ui/textarea/index.js";
    import * as Select from "@/lib/components/ui/select/index.js";
    import * as AlertDialog from "@/lib/components/ui/alert-dialog/index.js";
    import * as Card from "@/lib/components/ui/card/index.js";
    import { Separator } from "@/lib/components/ui/separator/index.js";

    import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
    import SaveIcon from "@lucide/svelte/icons/save";
    import TruckIcon from "@lucide/svelte/icons/truck";
    import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
    import PackageIcon from "@lucide/svelte/icons/package";
    import PhoneIcon from "@lucide/svelte/icons/phone";
    import MapPinIcon from "@lucide/svelte/icons/map-pin";
    import ClockIcon from "@lucide/svelte/icons/clock";
    import CheckIcon from "@lucide/svelte/icons/check";
    import XIcon from "@lucide/svelte/icons/x";
    import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
    import PrinterIcon from "@lucide/svelte/icons/printer";
    import SendIcon from "@lucide/svelte/icons/send";

    import type { LoadingStatus } from "@/components/StatusBoundary.svelte";
    import StatusBoundary from "@/components/StatusBoundary.svelte";

    const { replace } = useNavigate();
    const { location } = useRoute();

    let params = location?.params;

    let order: OrderPublic | null = null;
    let loadingStatus: LoadingStatus = undefined;
    let isSaving = false;
    let hasChanges = false;
    let showCancelDialog = false;

    // Fulfillment data
    let fulfillmentData = {
        status: null as OrderStatus | null,
        shipping_provider: "",
        tracking_number: "",
        shipping_cost: "",
        notes: "",
        notify_customer: true,
    };

    // Shipping providers (placeholder data)
    const shippingProviders = [
        { value: "yalidine", label: "Yalidine Express" },
        { value: "aramex", label: "Aramex" },
        { value: "dhl", label: "DHL Express" },
        { value: "fedex", label: "FedEx" },
        { value: "postal", label: "Algeria Post" },
        { value: "custom", label: "Custom Carrier" },
    ];

    onMount(async () => {
        await loadOrder();
    });

    async function loadOrder() {
        try {
            loadingStatus = undefined;
            if (!params?.id || typeof params.id !== "string") {
                throw new Error("Missing order ID");
            }
            const response = await ApiRoutes.get_one_order({
                id: params.id,
            });
            order = response;

            // Initialize fulfillment data
            fulfillmentData.status = order.status;

            document.title = `Order #${order.id.slice(0, 8)}`;
            loadingStatus = null;
        } catch (error) {
            notifCenter.error("Failed to load order");
            loadingStatus = error ?? new Error("Failed to load order");
        }
    }

    async function handleSave() {
        if (!order) return;

        try {
            isSaving = true;

            const updateBody: OrderUpdateBody = {
                status: fulfillmentData.status,
                full_name: null,
                phone: null,
                email: null,
                province: null,
                address: null,
                delivery: null,
                note: fulfillmentData.notes || null,
                items: null,
            };

            await ApiRoutes.update_order({
                id: order.id,
                body: updateBody,
            });

            notifCenter.success("Order updated successfully");
            hasChanges = false;

            // Reload order to get updated data
            await loadOrder();
        } catch (error) {
            notifCenter.error("Failed to update order");
            console.error("Error updating order:", error);
        } finally {
            isSaving = false;
        }
    }

    async function handleStatusChange(newStatus: OrderStatus) {
        fulfillmentData.status = newStatus;
        hasChanges = true;
    }

    async function handleCancelOrder() {
        if (!order) return;

        try {
            const updateBody: OrderUpdateBody = {
                status: "CANCELED",
                full_name: null,
                phone: null,
                email: null,
                province: null,
                address: null,
                delivery: null,
                note: "Order canceled",
                items: null,
            };

            await ApiRoutes.update_order({
                id: order.id,
                body: updateBody,
            });

            notifCenter.success("Order canceled successfully");
            await loadOrder();
        } catch (error) {
            notifCenter.error("Failed to cancel order");
        } finally {
            showCancelDialog = false;
        }
    }

    function handleFormChange() {
        hasChanges = true;
    }

    function getStatusColor(status: OrderStatus) {
        switch (status) {
            case "PENDING":
                return "bg-yellow-50 text-yellow-700 border-yellow-200";
            case "CONFIRMED":
                return "bg-blue-50 text-blue-700 border-blue-200";
            case "DELIVERED":
                return "bg-green-50 text-green-700 border-green-200";
            case "DONE":
                return "bg-emerald-50 text-emerald-700 border-emerald-200";
            case "CANCELED":
                return "bg-red-50 text-red-700 border-red-200";
            case "RETURNED":
                return "bg-gray-50 text-gray-700 border-gray-200";
            default:
                return "bg-gray-50 text-gray-600 border-gray-200";
        }
    }

    function calculateTotal() {
        if (!order?.items) return 0;
        return Object.values(order.items).reduce((total, item) => {
            return total + item.price * item.quantity;
        }, 0);
    }

    // Breadcrumbs
    $: breadcrumbs = [
        { label: "Orders", href: Routes.LIST_PAGE.path },
        { label: order ? `#${order.id.slice(0, 8)}` : "Loading...", href: "#" },
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
    ];

    // Status info
    $: statusInfo = order
        ? {
              label: order.status,
              variant:
                  order.status === "DELIVERED" || order.status === "DONE"
                      ? "default"
                      : ("secondary" as const),
              icon: ShoppingCartIcon,
          }
        : null;
</script>

<svelte:head>
    <title>{order ? `Order #${order.id.slice(0, 8)}` : "Loading..."}</title>
</svelte:head>

<StatusBoundary status={loadingStatus}>
    <EditPageLayout
        {breadcrumbs}
        {headerActions}
        {statusInfo}
        title={order ? `Order #${order.id.slice(0, 8)}` : "Loading..."}
        subtitle="Manage order fulfillment and customer communication"
        icon={ShoppingCartIcon}
    >
        <!-- Back Button -->
        <div slot="back-button">
            <a href={Routes.LIST_PAGE.path} use:link>
                <Button variant="ghost" size="sm" class="gap-2">
                    <ArrowLeftIcon class="h-4 w-4" />
                    Back to Orders
                </Button>
            </a>
        </div>

        <!-- Additional Actions -->
        <div slot="additional-actions" class="flex gap-2">
            <Button variant="outline" size="sm" class="gap-2">
                <PrinterIcon class="h-4 w-4" />
                Print Label
            </Button>
            <Button variant="outline" size="sm" class="gap-2">
                <SendIcon class="h-4 w-4" />
                Notify Customer
            </Button>
            <Button
                variant="outline"
                size="sm"
                class="gap-2 text-destructive hover:text-destructive"
                onclick={() => (showCancelDialog = true)}
                disabled={order?.status === "CANCELED"}
            >
                <XIcon class="h-4 w-4" />
                Cancel Order
            </Button>
        </div>

        <!-- Main Content -->
        <div slot="content" class="space-y-6">
            {#if order}
                <!-- Order Status & Quick Actions -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            <PackageIcon class="h-5 w-5" />
                            Order Fulfillment
                        </Card.Title>
                    </Card.Header>
                    <Card.Content class="space-y-4">
                        <!-- Status Selection -->
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="space-y-2">
                                <Label for="status">Order Status</Label>
                                <Select.Root
                                    bind:selected={fulfillmentData.status}
                                    onSelectedChange={(selected) => {
                                        if (selected) {
                                            handleStatusChange(selected.value);
                                        }
                                    }}
                                >
                                    <Select.Trigger>
                                        <!-- <Select.Item
                                            placeholder="Select status"
                                        /> -->
                                    </Select.Trigger>
                                    <Select.Content>
                                        <Select.Item value="PENDING">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <ClockIcon
                                                    class="h-4 w-4 text-yellow-500"
                                                />
                                                Pending
                                            </div>
                                        </Select.Item>
                                        <Select.Item value="CONFIRMED">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <CheckIcon
                                                    class="h-4 w-4 text-blue-500"
                                                />
                                                Confirmed
                                            </div>
                                        </Select.Item>
                                        <Select.Item value="DELIVERED">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <TruckIcon
                                                    class="h-4 w-4 text-green-500"
                                                />
                                                Delivered
                                            </div>
                                        </Select.Item>
                                        <Select.Item value="DONE">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <CheckIcon
                                                    class="h-4 w-4 text-emerald-500"
                                                />
                                                Completed
                                            </div>
                                        </Select.Item>
                                        <Select.Item value="CANCELED">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <XIcon
                                                    class="h-4 w-4 text-red-500"
                                                />
                                                Canceled
                                            </div>
                                        </Select.Item>
                                        <Select.Item value="RETURNED">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <RefreshCwIcon
                                                    class="h-4 w-4 text-gray-500"
                                                />
                                                Returned
                                            </div>
                                        </Select.Item>
                                    </Select.Content>
                                </Select.Root>
                            </div>

                            <!-- Quick Status Actions -->
                            <div class="space-y-2">
                                <Label>Quick Actions</Label>
                                <div class="flex gap-2 flex-wrap">
                                    <Button
                                        size="sm"
                                        variant="outline"
                                        onclick={() =>
                                            handleStatusChange("CONFIRMED")}
                                        disabled={order.status === "CONFIRMED"}
                                    >
                                        <CheckIcon class="h-4 w-4 mr-1" />
                                        Confirm
                                    </Button>
                                    <Button
                                        size="sm"
                                        variant="outline"
                                        onclick={() =>
                                            handleStatusChange("DELIVERED")}
                                        disabled={order.status === "DELIVERED"}
                                    >
                                        <TruckIcon class="h-4 w-4 mr-1" />
                                        Mark Delivered
                                    </Button>
                                    <Button
                                        size="sm"
                                        variant="outline"
                                        onclick={() =>
                                            handleStatusChange("DONE")}
                                        disabled={order.status === "DONE"}
                                    >
                                        <CheckIcon class="h-4 w-4 mr-1" />
                                        Complete
                                    </Button>
                                </div>
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>

                <!-- Shipping Information -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            <TruckIcon class="h-5 w-5" />
                            Shipping & Tracking
                        </Card.Title>
                    </Card.Header>
                    <Card.Content class="space-y-4">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="space-y-2">
                                <Label for="shipping_provider"
                                    >Shipping Provider</Label
                                >
                                <Select.Root
                                    bind:selected={
                                        fulfillmentData.shipping_provider
                                    }
                                >
                                    <Select.Trigger>
                                        <!-- <Select.Value
                                            placeholder="Select provider"
                                        /> -->
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each shippingProviders as provider}
                                            <Select.Item value={provider.value}>
                                                {provider.label}
                                            </Select.Item>
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            </div>

                            <div class="space-y-2">
                                <Label for="tracking_number"
                                    >Tracking Number</Label
                                >
                                <Input
                                    id="tracking_number"
                                    bind:value={fulfillmentData.tracking_number}
                                    placeholder="Enter tracking number"
                                    on:input={handleFormChange}
                                />
                            </div>

                            <div class="space-y-2">
                                <Label for="shipping_cost"
                                    >Shipping Cost (DZD)</Label
                                >
                                <Input
                                    id="shipping_cost"
                                    type="number"
                                    bind:value={fulfillmentData.shipping_cost}
                                    placeholder="0.00"
                                    on:input={handleFormChange}
                                />
                            </div>

                            <div class="space-y-2">
                                <Label>Delivery Method</Label>
                                <div class="p-3 bg-muted rounded-md">
                                    <Badge
                                        variant={order.delivery ===
                                        "HOME_DELIVERY"
                                            ? "default"
                                            : "secondary"}
                                    >
                                        {order.delivery === "HOME_DELIVERY"
                                            ? "Home Delivery"
                                            : "Pickup"}
                                    </Badge>
                                </div>
                            </div>
                        </div>

                        <div class="space-y-2">
                            <Label for="notes">Fulfillment Notes</Label>
                            <Textarea
                                id="notes"
                                bind:value={fulfillmentData.notes}
                                placeholder="Add notes about shipping, handling, or special instructions..."
                                rows="3"
                                on:input={handleFormChange}
                            />
                        </div>
                    </Card.Content>
                </Card.Root>

                <!-- Order Items -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            <PackageIcon class="h-5 w-5" />
                            Order Items
                        </Card.Title>
                    </Card.Header>
                    <Card.Content>
                        <div class="space-y-4">
                            {#each Object.entries(order.items) as [productId, item]}
                                <div
                                    class="flex justify-between items-center p-4 border rounded-lg"
                                >
                                    <div class="flex-1">
                                        <h4 class="font-medium">{item.name}</h4>
                                        <p
                                            class="text-sm text-muted-foreground"
                                        >
                                            Quantity: {item.quantity} × {new Intl.NumberFormat(
                                                "en-US",
                                                {
                                                    style: "currency",
                                                    currency: "DZD",
                                                },
                                            ).format(item.price)}
                                        </p>
                                    </div>
                                    <div class="font-semibold">
                                        {new Intl.NumberFormat("en-US", {
                                            style: "currency",
                                            currency: "DZD",
                                        }).format(item.price * item.quantity)}
                                    </div>
                                </div>
                            {/each}

                            <Separator />

                            <div
                                class="flex justify-between items-center font-semibold text-lg"
                            >
                                <span>Total:</span>
                                <span>
                                    {new Intl.NumberFormat("en-US", {
                                        style: "currency",
                                        currency: "DZD",
                                    }).format(calculateTotal())}
                                </span>
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>
            {/if}
        </div>

        <!-- Sidebar Content -->
        <div slot="sidebar">
            {#if order}
                <div class="space-y-6">
                    <!-- Customer Information -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="text-base"
                                >Customer Details</Card.Title
                            >
                        </Card.Header>
                        <Card.Content class="space-y-4">
                            <div class="space-y-2">
                                <div class="flex items-center gap-2 text-sm">
                                    <span class="font-medium"
                                        >{order.full_name}</span
                                    >
                                </div>
                                <div
                                    class="flex items-center gap-2 text-sm text-muted-foreground"
                                >
                                    <PhoneIcon class="h-4 w-4" />
                                    {order.phone}
                                </div>
                                <div
                                    class="flex items-center gap-2 text-sm text-muted-foreground"
                                >
                                    <MapPinIcon class="h-4 w-4" />
                                    {order.province}
                                </div>
                            </div>
                            <div class="pt-2 border-t">
                                <p class="text-sm text-muted-foreground">
                                    <strong>Address:</strong><br />
                                    {order.address}
                                </p>
                            </div>
                            {#if order.email}
                                <div class="text-sm text-muted-foreground">
                                    <strong>Email:</strong>
                                    {order.email}
                                </div>
                            {/if}
                        </Card.Content>
                    </Card.Root>

                    <!-- Order Timeline -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="text-base"
                                >Order History</Card.Title
                            >
                        </Card.Header>
                        <Card.Content>
                            <div class="space-y-3">
                                {#each order.history as entry}
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="w-2 h-2 rounded-full bg-primary"
                                        ></div>
                                        <div class="flex-1">
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <Badge
                                                    variant="outline"
                                                    class={getStatusColor(
                                                        entry.status,
                                                    )}
                                                >
                                                    {entry.status}
                                                </Badge>
                                                <span
                                                    class="text-xs text-muted-foreground"
                                                >
                                                    {new Date(
                                                        Number(entry.time),
                                                    ).toLocaleString()}
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </Card.Content>
                    </Card.Root>

                    <!-- Payment Information -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="text-base"
                                >Payment Details</Card.Title
                            >
                        </Card.Header>
                        <Card.Content class="space-y-3">
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground">Method</span
                                >
                                <Badge variant="outline">Cash on Delivery</Badge
                                >
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground">Amount</span
                                >
                                <span class="font-medium">
                                    {new Intl.NumberFormat("en-US", {
                                        style: "currency",
                                        currency: "DZD",
                                    }).format(calculateTotal())}
                                </span>
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-muted-foreground">Status</span
                                >
                                <Badge
                                    variant={order.status === "DONE"
                                        ? "default"
                                        : "secondary"}
                                >
                                    {order.status === "DONE"
                                        ? "Paid"
                                        : "Pending"}
                                </Badge>
                            </div>
                        </Card.Content>
                    </Card.Root>
                </div>
            {/if}
        </div>
    </EditPageLayout>

    <!-- Cancel Order Dialog -->
    <AlertDialog.Root bind:open={showCancelDialog}>
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title>Cancel Order</AlertDialog.Title>
                <AlertDialog.Description>
                    Are you sure you want to cancel order #{order?.id.slice(
                        0,
                        8,
                    )}? This action cannot be undone and the customer will be
                    notified.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel>Keep Order</AlertDialog.Cancel>
                <AlertDialog.Action
                    onclick={handleCancelOrder}
                    class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                >
                    Cancel Order
                </AlertDialog.Action>
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>

    {#snippet error()}
        <div class="flex flex-col items-center justify-center py-12">
            <div class="text-center">
                <h3 class="text-lg font-semibold">Order not found</h3>
                <p class="text-muted-foreground mt-2">
                    The order you're looking for doesn't exist or has been
                    removed.
                </p>
                <a href={Routes.LIST_PAGE.path} use:link>
                    <Button class="mt-4">Back to Orders</Button>
                </a>
            </div>
        </div>
    {/snippet}
</StatusBoundary>
