<script lang="ts">
  import * as Card from "$lib/components/ui/card/index";
  import * as AlertDialog from "$lib/components/ui/alert-dialog/index";
  import * as Select from "$lib/components/ui/select/index";
  import * as Table from "$lib/components/ui/table/index";
  import { Badge } from "@/lib/components/ui/badge";
  import { Button } from "@/lib/components/ui/button";
  import { Textarea } from "@/lib/components/ui/textarea";
  import Column from "../../lib/components/layout/column.svelte";
  import Group from "../../lib/components/layout/group.svelte";
  import SectionHeader from "../../lib/components/section-header.svelte";
  import ActionButton from "../../lib/components/action-button.svelte";
  import {
    ShoppingCartIcon,
    ArrowLeftIcon,
    EditIcon,
    TruckIcon,
    PackageIcon,
    CheckCircleIcon,
    XCircleIcon,
    RefreshCwIcon,
    PrinterIcon,
    DownloadIcon,
    MapPinIcon,
    CreditCardIcon,
    UserIcon,
    PhoneIcon,
    MailIcon,
  } from "@lucide/svelte";

  import {
    useOrderQuery,
    useOrderStatusUpdate,
    getStatusBadgeVariant,
    getPaymentStatusBadgeVariant,
    formatOrderNumber,
    canCancelOrder,
    // canRefundOrder,
    canArchiveOrder,
    OrderStatusUpdateSchema,
  } from "./service";
  import { useNavigate, useRoute } from "@dvcol/svelte-simple-router/router";
  import { useState } from "../../lib/utils/utils.svelte";
  import { createForm, getFormValues } from "../../lib/utils/form";
  import { toast } from "svelte-sonner";
  import {
    formatDateTime,
    formatCurrency,
    snakeToTitleCase,
  } from "../../lib/utils/fmt";
  import { Routes } from ".";
  import { dialog } from "../../lib/components/alert-dialog.svelte";
  import type { OrderStatusDto } from "@bindings/OrderStatusDto";
  import type { OrderDto } from "@bindings/OrderDto";
  import { Label } from "@/lib/components/ui/label";

  const { replace } = useNavigate();
  const { location } = useRoute();
  const orderId: string = location?.params.id as string;

  const query = useOrderQuery(orderId);

  const statusUpdateMutation = useOrderStatusUpdate(
    orderId,
    () => {
      toast.success("Order status updated successfully");
      query.refetch();
    },
    (error) => {
      toast.error("Error updating order status", {
        description: error.message,
      });
    },
  );

  let statusUpdateNote = $state("");
  let isUpdatingStatus = $state(false);

  async function handleStatusUpdate(newStatus: OrderStatusDto) {
    const confirmed = await dialog.confirm({
      title: `Update order status to ${snakeToTitleCase(newStatus)}?`,
      description:
        "This will update the order status and notify the customer if applicable.",
      actions: [
        { label: "Cancel", value: false },
        { label: "Update Status", value: true },
      ],
    });

    if (!confirmed) return;

    statusUpdateMutation.mutate({
      status: newStatus,
      note: statusUpdateNote || null,
    });

    statusUpdateNote = "";
  }

  async function handleCancelOrder() {
    const confirmed = await dialog.confirm({
      title: "Cancel this order?",
      description:
        "This action cannot be undone. The customer will be notified of the cancellation.",
      actions: [
        { label: "Keep Order", value: false },
        { label: "Cancel Order", value: true, variant: "destructive" },
      ],
    });

    if (!confirmed) return;

    statusUpdateMutation.mutate({
      status: "cancelled",
      note: "Order cancelled by admin",
    });
  }

  function calculateSubtotal(items: any[]): number {
    return items.reduce(
      (total, item) => total + parseFloat(item.total_price),
      0,
    );
  }

  function goBack() {
    replace({ path: Routes.LIST_PAGE.path });
  }

  function printOrder() {
    window.print();
  }

  function downloadInvoice() {
    // TODO: Implement invoice download
    toast.info("Invoice download feature coming soon");
  }
</script>

<div>
  {#if query.isLoading}
    <div class="flex justify-center items-center h-64">
      <div class="text-center">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-2"
        ></div>
        <p>Loading order details...</p>
      </div>
    </div>
  {:else if query.isError}
    <div class="flex justify-center items-center h-64">
      <div class="text-center">
        <p class="text-destructive mb-2">
          Error: {query.error.message}
        </p>
        <Button onclick={() => query.refetch()}>Retry</Button>
      </div>
    </div>
  {:else if query.isSuccess}
    {@render body(query.data)}
  {/if}
</div>

{#snippet body(order: OrderDto)}
  <Column class="[&>*]:w-full items-center">
    <Group>
      <div class="flex items-center gap-4">
        <SectionHeader
          icon={ShoppingCartIcon}
          title={formatOrderNumber(order.id)}
          description={`Order placed on ${formatDateTime(order.created_at)}`}
        >
          <div class="flex items-center gap-2">
            <Badge variant={getStatusBadgeVariant(order.status)}>
              {snakeToTitleCase(order.status)}
            </Badge>
            <!-- <Badge
              variant={getPaymentStatusBadgeVariant(
                order.payment_status,
              )}
            >
              {snakeToTitleCase(order.payment_status)}
            </Badge> -->
          </div>
        </SectionHeader>
      </div>

      <Group class="md:flex-row-reverse flex-wrap justify-start">
        <ActionButton variant="outline" onclick={printOrder}>
          <PrinterIcon />
          Print
        </ActionButton>
        <ActionButton variant="outline" onclick={downloadInvoice}>
          <DownloadIcon />
          Invoice
        </ActionButton>
        {#if canCancelOrder(order)}
          <ActionButton variant="destructive" onclick={handleCancelOrder}>
            <XCircleIcon />
            Cancel Order
          </ActionButton>
        {/if}
      </Group>
    </Group>

    <Group class="max-w-6xl md:flex-col md:[&>*]:w-full lg:flex-row">
      <div class="flex-1">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Main Content -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Order Items -->
            {@render orderItems(order)}

            <!-- Order History -->
            {@render orderHistory(order)}
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- Status Management -->
            {@render statusManagement(order)}

            <!-- Customer Information -->
            {@render customerInfo(order)}

            <!-- Shipping Address -->
            {@render shippingAddress(order)}

            <!-- Billing Address -->
            {#if order.billing_address}
              {@render billingAddress(order.billing_address)}
            {/if}

            <!-- Order Summary -->
            {@render orderSummary(order)}
          </div>
        </div>
      </div>
    </Group>
  </Column>
{/snippet}

{#snippet orderItems(order: OrderDto)}
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2">
        <PackageIcon class="w-5 h-5" />
        Order Items ({order.items.length})
      </Card.Title>
    </Card.Header>
    <Card.Content>
      <Table.Root>
        <Table.Header>
          <Table.Row>
            <Table.Head>Product</Table.Head>
            <Table.Head>SKU</Table.Head>
            <Table.Head class="text-center">Quantity</Table.Head>
            <Table.Head class="text-right">Unit Price</Table.Head>
            <Table.Head class="text-right">Total</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {#each order.items as item}
            <Table.Row>
              <Table.Cell>
                <div class="font-medium">
                  {item.product_title}
                </div>
              </Table.Cell>
              <Table.Cell>
                <code class="text-sm bg-muted px-2 py-1 rounded"
                  >{item.variant_sku}</code
                >
              </Table.Cell>
              <Table.Cell class="text-center">{item.quantity}</Table.Cell>
              <Table.Cell class="text-right">
                {formatCurrency(parseFloat(item.unit_price), order.currency)}
              </Table.Cell>
              <Table.Cell class="text-right font-medium">
                {formatCurrency(parseFloat(item.total_price), order.currency)}
              </Table.Cell>
            </Table.Row>
          {/each}
        </Table.Body>
      </Table.Root>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet orderHistory(order: OrderDto)}
  <Card.Root>
    <Card.Header>
      <Card.Title>Order History</Card.Title>
    </Card.Header>
    <Card.Content>
      <div class="space-y-4">
        {#each order.history as entry}
          <div class="flex items-start gap-3 pb-3 border-b last:border-b-0">
            <div class="w-2 h-2 bg-primary rounded-full mt-2"></div>
            <div class="flex-1">
              <div class="flex items-center gap-2 mb-1">
                <Badge
                  variant={getStatusBadgeVariant(entry.status)}
                  class="text-xs"
                >
                  {snakeToTitleCase(entry.status)}
                </Badge>
                <span class="text-sm text-muted-foreground">
                  {formatDateTime(entry.created_at)}
                </span>
              </div>
              {#if entry.note}
                <p class="text-sm">{entry.note}</p>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet statusManagement(order: OrderDto)}
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2">
        <EditIcon class="w-5 h-5" />
        Update Status
      </Card.Title>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div class="space-y-2">
        <Label for={undefined} class="text-sm font-medium">Current Status</Label
        >
        <Badge variant={getStatusBadgeVariant(order.status)}>
          {snakeToTitleCase(order.status)}
        </Badge>
      </div>

      <div class="space-y-4">
        <Label for="status-note" class="text-sm font-medium"
          >Note (Optional)</Label
        >
        <Textarea
          id="status-note"
          bind:value={statusUpdateNote}
          placeholder="Add a note about this status change..."
          rows={3}
        />
      </div>

      <div class="flex flex-col gap-2">
        {#if order.status === "pending"}
          <Button
            onclick={() => handleStatusUpdate("confirmed")}
            disabled={statusUpdateMutation.isPending}
          >
            <CheckCircleIcon class="w-4 h-4 mr-2" />
            Confirm Order
          </Button>
        {:else if order.status === "confirmed"}
          <Button
            onclick={() => handleStatusUpdate("processing")}
            disabled={statusUpdateMutation.isPending}
          >
            <RefreshCwIcon class="w-4 h-4 mr-2" />
            Start Processing
          </Button>
        {:else if order.status === "processing"}
          <Button
            onclick={() => handleStatusUpdate("shipped")}
            disabled={statusUpdateMutation.isPending}
          >
            <TruckIcon class="w-4 h-4 mr-2" />
            Mark as Shipped
          </Button>
        {:else if order.status === "shipped"}
          <Button
            onclick={() => handleStatusUpdate("delivered")}
            disabled={statusUpdateMutation.isPending}
          >
            <CheckCircleIcon class="w-4 h-4 mr-2" />
            Mark as Delivered
          </Button>
        {/if}

        {#if canArchiveOrder(order)}
          <Button
            variant="outline"
            onclick={() => handleStatusUpdate("archived")}
            disabled={statusUpdateMutation.isPending}
          >
            Archive Order
          </Button>
        {/if}
      </div>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet customerInfo(order: OrderDto)}
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2">
        <UserIcon class="w-5 h-5" />
        Customer
      </Card.Title>
    </Card.Header>
    <Card.Content class="space-y-3">
      <div class="flex items-center gap-2">
        <UserIcon class="w-4 h-4 text-muted-foreground" />
        <span class="font-medium">{order.customer_name}</span>
      </div>
      <div class="flex items-center gap-2">
        <MailIcon class="w-4 h-4 text-muted-foreground" />
        <a
          href="mailto:{order.customer_email}"
          class="text-primary hover:underline"
        >
          {order.customer_email}
        </a>
      </div>
      {#if order.customer_phone}
        <div class="flex items-center gap-2">
          <PhoneIcon class="w-4 h-4 text-muted-foreground" />
          <a
            href="tel:{order.customer_phone}"
            class="text-primary hover:underline"
          >
            {order.customer_phone}
          </a>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet shippingAddress(order: OrderDto)}
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2">
        <MapPinIcon class="w-5 h-5" />
        Shipping Address
      </Card.Title>
    </Card.Header>
    <Card.Content>
      <div class="space-y-1 text-sm">
        <div class="font-medium">
          {order.shipping_address.full_name}
        </div>
        <div>{order.shipping_address.address_line_1}</div>
        {#if order.shipping_address.address_line_2}
          <div>{order.shipping_address.address_line_2}</div>
        {/if}
        <div>
          {order.shipping_address.city}, {order.shipping_address.state}
          {order.shipping_address.postal_code}
        </div>
        <div>{order.shipping_address.country}</div>
      </div>
      {#if order.tracking_number}
        <div class="mt-3 pt-3 border-t">
          <div class="text-sm">
            <span class="font-medium">Tracking:</span>
            {order.tracking_number}
          </div>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet billingAddress(
  billing_address: NonNullable<OrderDto["billing_address"]>,
)}
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2">
        <CreditCardIcon class="w-5 h-5" />
        Billing Address
      </Card.Title>
    </Card.Header>
    <Card.Content>
      <div class="space-y-1 text-sm">
        <div class="font-medium">
          {billing_address.full_name}
        </div>
        <div>{billing_address.address_line_1}</div>
        {#if billing_address.address_line_2}
          <div>{billing_address.address_line_2}</div>
        {/if}
        <div>
          {billing_address.city}, {billing_address.state}
          {billing_address.postal_code}
        </div>
        <div>{billing_address.country}</div>
      </div>
    </Card.Content>
  </Card.Root>
{/snippet}

{#snippet orderSummary(order: OrderDto)}
  <Card.Root>
    <Card.Header>
      <Card.Title>Order Summary</Card.Title>
    </Card.Header>
    <Card.Content>
      <div class="space-y-2 text-sm">
        <div class="flex justify-between">
          <span>Subtotal:</span>
          <span
            >{formatCurrency(parseFloat(order.subtotal), order.currency)}</span
          >
        </div>
        <div class="flex justify-between">
          <span>Shipping:</span>
          <span
            >{formatCurrency(
              parseFloat(order.shipping_cost),
              order.currency,
            )}</span
          >
        </div>
        <div class="flex justify-between">
          <span>Tax:</span>
          <span
            >{formatCurrency(
              parseFloat(order.tax_amount),
              order.currency,
            )}</span
          >
        </div>
        <hr />
        <div class="flex justify-between font-medium text-base">
          <span>Total:</span>
          <span
            >{formatCurrency(
              parseFloat(order.total_amount),
              order.currency,
            )}</span
          >
        </div>
      </div>
      {#if order.notes}
        <div class="mt-4 pt-4 border-t">
          <div class="text-sm">
            <span class="font-medium">Notes:</span>
            <p class="mt-1 text-muted-foreground">{order.notes}</p>
          </div>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
{/snippet}
