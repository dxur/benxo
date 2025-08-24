<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index";
  import * as Pagination from "$lib/components/ui/pagination/index";
  import * as Table from "$lib/components/ui/table/index";
  import * as Card from "$lib/components/ui/card/index";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index";
  import * as Select from "$lib/components/ui/select/index";
  import { Button } from "@/lib/components/ui/button";
  import { Badge } from "@/lib/components/ui/badge";
  import { Checkbox } from "@/lib/components/ui/checkbox";
  import Column from "../../lib/components/layout/column.svelte";
  import Group from "../../lib/components/layout/group.svelte";
  import ActionButton from "../../lib/components/action-button.svelte";
  import SectionHeader from "../../lib/components/section-header.svelte";
  import LoadingSpinner from "../../lib/components/loading-spinner.svelte";
  import LoadingError from "../../lib/components/loading-error.svelte";
  import SearchBar from "../../lib/components/search-bar.svelte";
  import {
    ChevronLeftIcon,
    ChevronRightIcon,
    EllipsisVerticalIcon,
    ShoppingCartIcon,
    PlusIcon,
    DownloadIcon,
    FilterIcon,
  } from "@lucide/svelte";

  import { useLink } from "@dvcol/svelte-simple-router";
  import { Routes } from ".";
  import {
    useOrderListQuery,
    getStatusBadgeVariant,
    getPaymentStatusBadgeVariant,
    formatOrderNumber,
  } from "./service";
  import { createQuery } from "@tanstack/svelte-query";
  import {
    formatDateTime,
    formatCurrency,
    snakeToTitleCase,
  } from "../../lib/utils/fmt";
  import type { OrderDto } from "@bindings/OrderDto";
  import type { OrderListQuery } from "@bindings/OrderListQuery";
  import type { OrderStatusDto } from "@bindings/OrderStatusDto";
  import { debounce } from "../../lib/utils/event";

  let activeTab = $state<OrderStatusDto | "">("");
  let searchInput = $state("");
  let searchQuery = $state("");
  let customerEmailFilter = $state("");
  let dateFromFilter = $state("");
  let dateToFilter = $state("");
  let paginationQuery = $state({ page: 1, limit: 25 });
  let selectedOrders = $state<string[]>([]);

  let fetchParams = $derived<OrderListQuery>({
    search: searchQuery || undefined,
    status: activeTab || undefined,
    // payment_status: paymentStatusFilter || undefined,
    customer_email: customerEmailFilter || undefined,
    date_from: dateFromFilter || undefined,
    date_to: dateToFilter || undefined,
    ...paginationQuery,
  });

  const debouncedSearch = debounce((value) => {
    searchQuery = value;
    paginationQuery.page = 1;
  }, 750);

  $effect(() => {
    debouncedSearch(searchInput);
  });

  $effect(() => {
    paginationQuery.page = 1;
  });

  const query = useOrderListQuery(() => fetchParams);

  function toggleOrderSelection(orderId: string) {
    if (selectedOrders.includes(orderId)) {
      selectedOrders = selectedOrders.filter((id) => id !== orderId);
    } else {
      selectedOrders = [...selectedOrders, orderId];
    }
  }

  function toggleAllSelection() {
    if (selectedOrders.length === query.data?.orders.length) {
      selectedOrders = [];
    } else {
      selectedOrders = query.data?.orders.map((order) => order.id) || [];
    }
  }

  function getOrderTotal(order: OrderDto): number {
    return parseFloat(order.total_amount);
  }

  function exportOrders() {
    // TODO: Implement export functionality
    console.log("Export orders:", selectedOrders);
  }

  function clearFilters() {
    searchInput = "";
    searchQuery = "";
    customerEmailFilter = "";
    dateFromFilter = "";
    dateToFilter = "";
    activeTab = "";
    paginationQuery.page = 1;
  }
</script>

<Column>
  <Group>
    <SectionHeader
      icon={ShoppingCartIcon}
      title="Orders"
      description="Manage customer orders and track fulfillment"
    />
    <Group class="flex-wrap">
      <ActionButton {@attach useLink({ path: Routes.CREATE_PAGE.path })}>
        <PlusIcon />
        Create Order
      </ActionButton>
      <ActionButton
        variant="outline"
        onclick={exportOrders}
        disabled={selectedOrders.length === 0}
      >
        <DownloadIcon />
        Export ({selectedOrders.length})
      </ActionButton>
    </Group>
  </Group>

  <Group class="flex-wrap gap-4">
    <SearchBar
      class="flex-1 min-w-64"
      bind:value={searchInput}
      searching={query.isRefetching && !query.isFetchedAfterMount}
      placeholder="Search orders, customers, or order numbers..."
    />

    <Tabs.Root bind:value={activeTab}>
      <Tabs.List>
        <Tabs.Trigger value="pending">Pending</Tabs.Trigger>
        <Tabs.Trigger value="confirmed">Confirmed</Tabs.Trigger>
        <Tabs.Trigger value="processing">Processing</Tabs.Trigger>
        <Tabs.Trigger value="shipped">Shipped</Tabs.Trigger>
        <Tabs.Trigger value="delivered">Delivered</Tabs.Trigger>
        <Tabs.Trigger value="archived">Archived</Tabs.Trigger>
        <Tabs.Trigger value="">All</Tabs.Trigger>
      </Tabs.List>
    </Tabs.Root>
  </Group>

  <Group class="flex-wrap gap-4">
    <!-- <Select.Root type="single" bind:value={paymentStatusFilter}>
      <Select.Trigger class="w-48">
        {paymentStatusFilter
          ? snakeToTitleCase(paymentStatusFilter)
          : "Payment Status"}
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="">All Payment Status</Select.Item>
        <Select.Item value="pending">Pending</Select.Item>
        <Select.Item value="paid">Paid</Select.Item>
        <Select.Item value="failed">Failed</Select.Item>
        <Select.Item value="refunded">Refunded</Select.Item>
        <Select.Item value="partially_refunded">Partially Refunded</Select.Item>
      </Select.Content>
    </Select.Root> -->

    <input
      type="email"
      bind:value={customerEmailFilter}
      placeholder="Customer email"
      class="mr-auto px-3 py-2 border border-input rounded-md text-sm w-48"
    />

    <input
      type="date"
      bind:value={dateFromFilter}
      placeholder="From date"
      class="px-3 py-2 border border-input rounded-md text-sm w-36"
    />

    <input
      type="date"
      bind:value={dateToFilter}
      placeholder="To date"
      class="px-3 py-2 border border-input rounded-md text-sm w-36"
    />

    <!-- {#if searchQuery || customerEmailFilter || dateFromFilter || dateToFilter || activeTab}
      <Button variant="outline" size="sm" onclick={clearFilters}>
        Clear Filters
      </Button>
    {/if} -->
  </Group>

  <div class="space-y-4">
    {@render table()}
    {@render pagination()}
  </div>
</Column>

{#snippet table()}
  <Card.Root class="py-0">
    <Table.Root class="px-16">
      <Table.Header class="bg-muted sticky top-0 z-10">
        <Table.Row>
          <Table.Head class="w-1"></Table.Head>
          <Table.Head>
            <Checkbox
              checked={selectedOrders.length === query.data?.orders.length &&
                query.data?.orders.length > 0}
              onCheckedChange={toggleAllSelection}
            />
          </Table.Head>
          <Table.Head>Order</Table.Head>
          <Table.Head>Customer</Table.Head>
          <Table.Head>Status</Table.Head>
          <Table.Head>Items</Table.Head>
          <Table.Head>Total</Table.Head>
          <Table.Head>Date</Table.Head>
          <Table.Head>Actions</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {@render tableBody()}
      </Table.Body>
    </Table.Root>
  </Card.Root>
{/snippet}

{#snippet tableBody()}
  {#if query.isError}
    <Table.Row>
      <Table.Cell colspan={9} class="h-56 text-center">
        <LoadingError message={query.error.message}>
          <Button onclick={() => query.refetch()}>Retry</Button>
        </LoadingError>
      </Table.Cell>
    </Table.Row>
  {:else if query.isLoading || (query.isFetching && query.isFetchedAfterMount)}
    <Table.Row>
      <Table.Cell colspan={9} class="h-56 text-center">
        <LoadingSpinner text="Loading orders..." />
      </Table.Cell>
    </Table.Row>
  {:else if query.data?.orders.length === 0}
    <Table.Row>
      <Table.Cell colspan={9} class="h-56 text-center">
        No orders found.
      </Table.Cell>
    </Table.Row>
  {:else}
    {#each query.data?.orders || [] as order}
      <Table.Row>
        <Table.Cell></Table.Cell>
        <Table.Cell>
          <Checkbox
            checked={selectedOrders.includes(order.id)}
            onCheckedChange={() => toggleOrderSelection(order.id)}
          />
        </Table.Cell>
        <Table.Cell>
          <div>
            <div
              class="font-medium underline cursor-pointer hover:text-primary"
              {@attach useLink({
                path: Routes.DETAIL_PAGE.path.replace(":id", order.id),
              })}
            >
              {formatOrderNumber(order.id)}
            </div>
            {#if order.tracking_number}
              <div class="text-xs text-muted-foreground">
                Tracking: {order.tracking_number}
              </div>
            {/if}
          </div>
        </Table.Cell>
        <Table.Cell>
          <div>
            <div class="font-medium">{order.customer_name}</div>
            <div class="text-sm text-muted-foreground">
              {order.customer_email}
            </div>
            {#if order.customer_phone}
              <div class="text-xs text-muted-foreground">
                {order.customer_phone}
              </div>
            {/if}
          </div>
        </Table.Cell>
        <Table.Cell>
          <Badge variant={getStatusBadgeVariant(order.status)}>
            {snakeToTitleCase(order.status)}
          </Badge>
        </Table.Cell>
        <!-- <Table.Cell>
                    <Badge
                        variant={getPaymentStatusBadgeVariant(
                            order.payment_status,
                        )}
                    >
                        {snakeToTitleCase(order.payment_status)}
                    </Badge>
                </Table.Cell> -->
        <Table.Cell>
          {order.items.length} item{order.items.length !== 1 ? "s" : ""}
        </Table.Cell>
        <Table.Cell>
          <div class="font-medium">
            {formatCurrency(getOrderTotal(order), order.currency)}
          </div>
        </Table.Cell>
        <Table.Cell class="text-sm text-muted-foreground">
          {formatDateTime(order.created_at)}
        </Table.Cell>
        <Table.Cell>
          {@render actions(order)}
        </Table.Cell>
      </Table.Row>
    {/each}
  {/if}
{/snippet}

{#snippet actions(order: OrderDto)}
  <DropdownMenu.Root>
    <DropdownMenu.Trigger
      class="data-[state=open]:bg-muted text-muted-foreground flex size-8"
    >
      {#snippet child({ props })}
        <Button variant="ghost" size="icon" {...props}>
          <EllipsisVerticalIcon />
          <span class="sr-only">Open menu</span>
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end" class="w-40">
      <DropdownMenu.Item
        {@attach useLink({
          path: Routes.DETAIL_PAGE.path.replace(":id", order.id),
        })}>View Details</DropdownMenu.Item
      >
      <!-- <DropdownMenu.Separator /> -->
      {#if order.status === "pending"}
        <DropdownMenu.Item>Confirm Order</DropdownMenu.Item>
      {:else if order.status === "confirmed"}
        <DropdownMenu.Item>Mark Processing</DropdownMenu.Item>
      {:else if order.status === "processing"}
        <DropdownMenu.Item>Mark Shipped</DropdownMenu.Item>
      {:else if order.status === "shipped"}
        <DropdownMenu.Item>Mark Delivered</DropdownMenu.Item>
      {/if}
      {#if ["pending", "confirmed"].includes(order.status)}
        <DropdownMenu.Item variant="destructive">Cancel Order</DropdownMenu.Item
        >
      {/if}
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{/snippet}

{#snippet pagination()}
  <Pagination.Root
    bind:page={paginationQuery.page}
    count={(query.data?.total || 0) as number}
    perPage={query.data?.limit || 0}
  >
    {#snippet children({ pages, currentPage })}
      <Pagination.Content>
        <Pagination.Item>
          <Pagination.PrevButton>
            <ChevronLeftIcon class="size-4" />
            <span class="hidden sm:block">Previous</span>
          </Pagination.PrevButton>
        </Pagination.Item>
        {#each pages as page (page.key)}
          {#if page.type === "ellipsis"}
            <Pagination.Item>
              <Pagination.Ellipsis />
            </Pagination.Item>
          {:else}
            <Pagination.Item>
              <Pagination.Link {page} isActive={currentPage === page.value}>
                {page.value}
              </Pagination.Link>
            </Pagination.Item>
          {/if}
        {/each}
        <Pagination.Item>
          <Pagination.NextButton>
            <span class="hidden sm:block">Next</span>
            <ChevronRightIcon class="size-4" />
          </Pagination.NextButton>
        </Pagination.Item>
      </Pagination.Content>
    {/snippet}
  </Pagination.Root>
{/snippet}
