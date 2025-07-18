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
    import { Input } from "@/lib/components/ui/input/index.js";
    import { Label } from "@/lib/components/ui/label/index.js";
    import { Textarea } from "@/lib/components/ui/textarea/index.js";
    import * as Select from "@/lib/components/ui/select/index.js";
    import * as AlertDialog from "@/lib/components/ui/alert-dialog/index.js";
    import * as Card from "@/lib/components/ui/card/index.js";
    import { Separator } from "@/lib/components/ui/separator/index.js";
    import { Checkbox } from "@/lib/components/ui/checkbox/index.js";

    import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
    import SaveIcon from "@lucide/svelte/icons/save";
    import TruckIcon from "@lucide/svelte/icons/truck";
    import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
    import PackageIcon from "@lucide/svelte/icons/package";
    import PhoneIcon from "@lucide/svelte/icons/phone";
    import MapPinIcon from "@lucide/svelte/icons/map-pin";
    import XIcon from "@lucide/svelte/icons/x";
    import PrinterIcon from "@lucide/svelte/icons/printer";
    import SendIcon from "@lucide/svelte/icons/send";
    import UserIcon from "@lucide/svelte/icons/user";
    import ClockIcon from "@lucide/svelte/icons/clock";
    import CreditCardIcon from "@lucide/svelte/icons/credit-card";
    import CheckIcon from "@lucide/svelte/icons/check";
    import AlertCircleIcon from "@lucide/svelte/icons/alert-circle";

    import type { LoadingStatus } from "@/components/StatusBoundary.svelte";
    import StatusBoundary from "@/components/StatusBoundary.svelte";

    const { replace } = useNavigate();
    const { location } = useRoute();

    let params = location?.params;
    let order: OrderPublic | null = null;
    let loadingStatus: LoadingStatus = undefined;
    let isSaving = false;
    let isFulfilling = false;
    let hasChanges = false;
    let showCancelDialog = false;
    let showFulfillDialog = false;

    // Form data for order updates
    let formData = {
        status: "PENDING" as OrderStatus,
        notes: "",
    };

    // Fulfillment data
    let fulfillmentData = {
        shipping_provider: "",
        shipping_method: "",
        tracking_number: "",
        shipping_cost: "",
        fulfillment_notes: "",
        notify_customer: true,
    };

    // Order status options
    const statusOptions = [
        {
            value: "PENDING",
            label: "Pending",
            color: "bg-yellow-50 text-yellow-700 border-yellow-200",
        },
        {
            value: "CONFIRMED",
            label: "Confirmed",
            color: "bg-blue-50 text-blue-700 border-blue-200",
        },
        {
            value: "DELIVERED",
            label: "Delivered",
            color: "bg-green-50 text-green-700 border-green-200",
        },
        {
            value: "DONE",
            label: "Done",
            color: "bg-emerald-50 text-emerald-700 border-emerald-200",
        },
        {
            value: "CANCELED",
            label: "Canceled",
            color: "bg-red-50 text-red-700 border-red-200",
        },
        {
            value: "RETURNED",
            label: "Returned",
            color: "bg-gray-50 text-gray-700 border-gray-200",
        },
    ];

    // Shipping providers
    const shippingProviders = [
        { value: "yalidine", label: "Yalidine Express" },
        { value: "aramex", label: "Aramex" },
        { value: "dhl", label: "DHL Express" },
        { value: "fedex", label: "FedEx" },
        { value: "postal", label: "Algeria Post" },
        { value: "custom", label: "Custom Carrier" },
    ];

    // Delivery methods
    const deliveryMethods = [
        { value: "STOP_DESK", label: "Stop Desk" },
        { value: "DOMICIL", label: "Home Delivery" },
        { value: "OTHER", label: "Other" },
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

            // Initialize form data
            formData.status = order.status;
            formData.notes = order.note || "";

            document.title = `Order #${order.id.slice(-8)}`;
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
                status: formData.status,
                full_name: null,
                phone: null,
                email: null,
                province: null,
                address: null,
                delivery: null,
                note: formData.notes || null,
                items: null,
            };

            await ApiRoutes.update_order({
                id: order.id,
                body: updateBody,
            });

            notifCenter.success("Order updated successfully");
            hasChanges = false;
            await loadOrder();
        } catch (error) {
            notifCenter.error("Failed to update order");
            console.error("Error updating order:", error);
        } finally {
            isSaving = false;
        }
    }

    async function handleFulfillOrder() {
        if (!order) return;

        try {
            isFulfilling = true;

            // Update order status to CONFIRMED and add fulfillment data
            const updateBody: OrderUpdateBody = {
                status: "CONFIRMED",
                full_name: null,
                phone: null,
                email: null,
                province: null,
                address: null,
                delivery: null,
                note: fulfillmentData.fulfillment_notes || null,
                items: null,
            };

            await ApiRoutes.update_order({
                id: order.id,
                body: updateBody,
            });

            // Here you would typically also create a fulfillment record
            // await ApiRoutes.create_fulfillment({
            //     order_id: order.id,
            //     shipping_provider: fulfillmentData.shipping_provider,
            //     tracking_number: fulfillmentData.tracking_number,
            //     shipping_cost: parseFloat(fulfillmentData.shipping_cost) || 0,
            //     notify_customer: fulfillmentData.notify_customer,
            // });

            notifCenter.success("Order fulfilled successfully");
            showFulfillDialog = false;
            await loadOrder();
        } catch (error) {
            notifCenter.error("Failed to fulfill order");
            console.error("Error fulfilling order:", error);
        } finally {
            isFulfilling = false;
        }
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
        const statusOption = statusOptions.find((s) => s.value === status);
        return (
            statusOption?.color || "bg-gray-50 text-gray-600 border-gray-200"
        );
    }

    function calculateTotal() {
        if (!order?.items) return 0;
        return Object.values(order.items)
            .filter((item) => !!item)
            .reduce((total, item) => total + item.price * item.quantity, 0);
    }

    function formatCurrency(amount: number) {
        return new Intl.NumberFormat("en-US", {
            style: "currency",
            currency: "DZD",
        }).format(amount);
    }

    function canFulfill() {
        return (
            order &&
            (order.status === "PENDING" || order.status === "CONFIRMED")
        );
    }

    function canCancel() {
        return order && order.status !== "CANCELED" && order.status !== "DONE";
    }

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

    $: statusInfo = order
        ? {
              label: order.status,
              variant:
                  order.status === "DELIVERED" || order.status === "DONE"
                      ? ("default" as const)
                      : ("secondary" as const),
              icon: ShoppingCartIcon,
          }
        : undefined;
</script>

<svelte:head>
    <title>{order ? `Order #${order.id.slice(-8)}` : "Loading..."}</title>
</svelte:head>

<StatusBoundary status={loadingStatus}>
    <EditPageLayout
        {headerActions}
        {statusInfo}
        title={order ? `Order #${order.id.slice(-8)}` : "Loading..."}
        subtitle="Manage order fulfillment and customer communication"
        icon={ShoppingCartIcon}
    >
        <!-- Back Button -->
        {#snippet back_button()}
            <a href={Routes.LIST_PAGE.path} use:link>
                <Button variant="ghost" size="sm" class="gap-2">
                    <ArrowLeftIcon class="h-4 w-4" />
                    Back to Orders
                </Button>
            </a>
        {/snippet}

        <!-- Main Content -->
        {#if order}
            <div class="space-y-6">
                <!-- Order Status & Quick Actions -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <PackageIcon class="h-5 w-5" />
                                Order Management
                            </div>
                            <div class="flex gap-2">
                                {#if canFulfill()}
                                    <Button
                                        variant="default"
                                        size="sm"
                                        class="gap-2"
                                        onclick={() =>
                                            (showFulfillDialog = true)}
                                    >
                                        <TruckIcon class="h-4 w-4" />
                                        Fulfill Order
                                    </Button>
                                {/if}
                                <Button
                                    variant="outline"
                                    size="sm"
                                    class="gap-2"
                                >
                                    <PrinterIcon class="h-4 w-4" />
                                    Print Label
                                </Button>
                                {#if canCancel()}
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        class="gap-2 text-destructive hover:text-destructive"
                                        onclick={() =>
                                            (showCancelDialog = true)}
                                    >
                                        <XIcon class="h-4 w-4" />
                                        Cancel Order
                                    </Button>
                                {/if}
                            </div>
                        </Card.Title>
                    </Card.Header>
                    <Card.Content class="space-y-4">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="space-y-2">
                                <Label for="status">Order Status</Label>
                                <Select.Root
                                    type="single"
                                    bind:value={formData.status}
                                    onValueChange={handleFormChange}
                                >
                                    <Select.Trigger>
                                        {formData.status || "Select status"}
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each statusOptions as status}
                                            <Select.Item value={status.value}>
                                                <div
                                                    class="flex items-center gap-2"
                                                >
                                                    <Badge
                                                        class={status.color}
                                                        variant="outline"
                                                    >
                                                        {status.label}
                                                    </Badge>
                                                </div>
                                            </Select.Item>
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            </div>

                            <div class="space-y-2">
                                <Label>Current Status</Label>
                                <Badge
                                    class={getStatusColor(order.status)}
                                    variant="outline"
                                >
                                    {order.status}
                                </Badge>
                            </div>
                        </div>

                        <div class="space-y-2">
                            <Label for="notes">Order Notes</Label>
                            <Textarea
                                id="notes"
                                bind:value={formData.notes}
                                placeholder="Add notes about this order..."
                                rows={3}
                                oninput={handleFormChange}
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
                                {#if item}
                                    <div
                                        class="flex justify-between items-center p-4 border rounded-lg"
                                    >
                                        <div class="flex-1">
                                            <h4 class="font-medium">
                                                {productId}
                                            </h4>
                                            <p
                                                class="text-sm text-muted-foreground"
                                            >
                                                Quantity: {item.quantity} × {formatCurrency(
                                                    item.price,
                                                )}
                                            </p>
                                        </div>
                                        <div class="font-semibold">
                                            {formatCurrency(
                                                item.price * item.quantity,
                                            )}
                                        </div>
                                    </div>
                                {/if}
                            {/each}

                            <Separator />

                            <div
                                class="flex justify-between items-center font-semibold text-lg"
                            >
                                <span>Total:</span>
                                <span>{formatCurrency(calculateTotal())}</span>
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>
            </div>
        {/if}

        <!-- Sidebar Content -->
        {#snippet sidebar()}
            {#if order}
                <!-- Customer Information -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2 text-lg">
                            <UserIcon class="h-5 w-5" />
                            Customer Details
                        </Card.Title>
                    </Card.Header>
                    <Card.Content class="space-y-4">
                        <div class="space-y-3">
                            <div>
                                <p class="font-medium">{order.full_name}</p>
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

                        <Separator />

                        <div>
                            <p class="text-sm font-medium mb-2">Address:</p>
                            <p class="text-sm text-muted-foreground">
                                {order.address}
                            </p>
                        </div>

                        {#if order.email}
                            <div>
                                <p class="text-sm font-medium mb-1">Email:</p>
                                <p class="text-sm text-muted-foreground">
                                    {order.email}
                                </p>
                            </div>
                        {/if}
                    </Card.Content>
                </Card.Root>

                <!-- Order Timeline -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2 text-lg">
                            <ClockIcon class="h-5 w-5" />
                            Order History
                        </Card.Title>
                    </Card.Header>
                    <Card.Content>
                        <div class="space-y-4">
                            {#each order.history as entry}
                                <div class="flex items-start gap-3">
                                    <div
                                        class="w-2 h-2 rounded-full bg-primary mt-2 shrink-0"
                                    ></div>
                                    <div class="flex-1 min-w-0">
                                        <div class="flex flex-col gap-1">
                                            <Badge
                                                variant="outline"
                                                class={`${getStatusColor(entry.status)} w-fit`}
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
                        <Card.Title class="flex items-center gap-2 text-lg">
                            <CreditCardIcon class="h-5 w-5" />
                            Payment Details
                        </Card.Title>
                    </Card.Header>
                    <Card.Content class="space-y-4">
                        <div class="space-y-3">
                            <div class="flex justify-between items-center">
                                <span class="text-sm text-muted-foreground"
                                    >Method</span
                                >
                                <Badge variant="outline">Cash on Delivery</Badge
                                >
                            </div>
                            <div class="flex justify-between items-center">
                                <span class="text-sm text-muted-foreground"
                                    >Amount</span
                                >
                                <span class="font-medium"
                                    >{formatCurrency(calculateTotal())}</span
                                >
                            </div>
                            <div class="flex justify-between items-center">
                                <span class="text-sm text-muted-foreground"
                                    >Status</span
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
                        </div>
                    </Card.Content>
                </Card.Root>
            {/if}
        {/snippet}
    </EditPageLayout>

    <!-- Fulfill Order Dialog -->
    <AlertDialog.Root bind:open={showFulfillDialog}>
        <AlertDialog.Content class="max-w-2xl">
            <AlertDialog.Header>
                <AlertDialog.Title class="flex items-center gap-2">
                    <TruckIcon class="h-5 w-5" />
                    Fulfill Order #{order?.id.slice(-8)}
                </AlertDialog.Title>
                <AlertDialog.Description>
                    Configure shipping details and fulfill this order. The
                    customer will be notified if enabled.
                </AlertDialog.Description>
            </AlertDialog.Header>

            <div class="space-y-4 py-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <Label for="shipping_provider">Shipping Provider</Label>
                        <Select.Root
                            type="single"
                            bind:value={fulfillmentData.shipping_provider}
                        >
                            <Select.Trigger>
                                {fulfillmentData.shipping_provider ||
                                    "Select provider"}
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
                        <Label for="shipping_method">Delivery Method</Label>
                        <Select.Root
                            type="single"
                            bind:value={fulfillmentData.shipping_method}
                        >
                            <Select.Trigger>
                                {fulfillmentData.shipping_method ||
                                    "Select method"}
                            </Select.Trigger>
                            <Select.Content>
                                {#each deliveryMethods as method}
                                    <Select.Item value={method.value}>
                                        {method.label}
                                    </Select.Item>
                                {/each}
                            </Select.Content>
                        </Select.Root>
                    </div>

                    <div class="space-y-2">
                        <Label for="tracking_number">Tracking Number</Label>
                        <Input
                            id="tracking_number"
                            bind:value={fulfillmentData.tracking_number}
                            placeholder="Enter tracking number"
                        />
                    </div>

                    <div class="space-y-2">
                        <Label for="shipping_cost">Shipping Cost (DZD)</Label>
                        <Input
                            id="shipping_cost"
                            type="number"
                            bind:value={fulfillmentData.shipping_cost}
                            placeholder="0.00"
                        />
                    </div>
                </div>

                <div class="space-y-2">
                    <Label for="fulfillment_notes">Fulfillment Notes</Label>
                    <Textarea
                        id="fulfillment_notes"
                        bind:value={fulfillmentData.fulfillment_notes}
                        placeholder="Add notes about shipping, handling, or special instructions..."
                        rows={3}
                    />
                </div>

                <div class="flex items-center space-x-2">
                    <Checkbox
                        id="notify_customer"
                        bind:checked={fulfillmentData.notify_customer}
                    />
                    <Label for="notify_customer" class="text-sm">
                        Notify customer about fulfillment
                    </Label>
                </div>
            </div>

            <AlertDialog.Footer>
                <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
                <AlertDialog.Action
                    onclick={handleFulfillOrder}
                    disabled={isFulfilling}
                    class="bg-primary text-primary-foreground hover:bg-primary/90"
                >
                    {#if isFulfilling}
                        <div class="flex items-center gap-2">
                            <div
                                class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"
                            ></div>
                            Fulfilling...
                        </div>
                    {:else}
                        <div class="flex items-center gap-2">
                            <CheckIcon class="h-4 w-4" />
                            Fulfill Order
                        </div>
                    {/if}
                </AlertDialog.Action>
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>

    <!-- Cancel Order Dialog -->
    <AlertDialog.Root bind:open={showCancelDialog}>
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title class="flex items-center gap-2">
                    <AlertCircleIcon class="h-5 w-5 text-destructive" />
                    Cancel Order
                </AlertDialog.Title>
                <AlertDialog.Description>
                    Are you sure you want to cancel order #{order?.id.slice(
                        -8,
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
                    <div class="flex items-center gap-2">
                        <XIcon class="h-4 w-4" />
                        Cancel Order
                    </div>
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
