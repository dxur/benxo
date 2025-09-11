<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Select from "$lib/components/ui/select/index";
    import Column from "../../lib/components/layout/column.svelte";
    import Group from "../../lib/components/layout/group.svelte";
    import SectionHeader from "../../lib/components/section-header.svelte";
    import ActionButton from "../../lib/components/action-button.svelte";
    import LoadingSpinner from "../../lib/components/loading-spinner.svelte";
    import LoadingError from "../../lib/components/loading-error.svelte";
    import { Button } from "@/lib/components/ui/button";
    import {
        ArrowLeftIcon,
        TrendingUpIcon,
        ShoppingCartIcon,
        DollarSignIcon,
        PackageIcon,
        CheckCircleIcon,
        XCircleIcon,
        ClockIcon,
        DownloadIcon,
        RefreshCwIcon,
    } from "@lucide/svelte";

    import { useNavigate } from "@dvcol/svelte-simple-router/router";
    import { useOrderAnalyticsQuery } from "./service";
    import { formatCurrency, snakeToTitleCase } from "../../lib/utils/fmt";
    import { Routes } from ".";
    import type { OrderAnalytics } from "@bindings/OrderAnalytics";
    import { single } from "../../../../lib/event";

    const { replace } = useNavigate();

    let dateRange = $state("30d");

    // For now, we'll use the single analytics endpoint
    // In the future, this could be extended to support date ranges
    const analyticsQuery = useOrderAnalyticsQuery(() => ({
        date_from: new Date(
            dateRange === "7d"
                ? Date.now() - 7 * 24 * 60 * 60 * 1000
                : dateRange === "30d"
                  ? Date.now() - 30 * 24 * 60 * 60 * 1000
                  : dateRange === "90d"
                    ? Date.now() - 90 * 24 * 60 * 60 * 1000
                    : new Date(0).getTime(),
        ).toISOString(),
        date_to: new Date().toISOString(),
    }));

    function goBack() {
        replace({ path: Routes.LIST_PAGE.path });
    }

    function refreshData() {
        analyticsQuery.refetch();
    }

    function exportData() {
        // TODO: Implement analytics export
        console.log("Export analytics data");
    }

    function formatBigInt(value: bigint): string {
        return value.toString();
    }

    function calculateAverageOrderValue(analytics: OrderAnalytics): string {
        if (!analytics || analytics.total_orders === 0n) {
            return "0.00";
        }
        return analytics.average_order_value;
    }

    function calculateConversionRate(analytics: OrderAnalytics): string {
        if (!analytics || analytics.total_orders === 0n) {
            return "0.00";
        }
        const completed = Number(analytics.completed_orders);
        const total = Number(analytics.total_orders);
        return ((completed / total) * 100).toFixed(1);
    }

    function calculateCancellationRate(analytics: OrderAnalytics): string {
        if (!analytics || analytics.total_orders === 0n) {
            return "0.00";
        }
        const cancelled = Number(analytics.cancelled_orders);
        const total = Number(analytics.total_orders);
        return ((cancelled / total) * 100).toFixed(1);
    }
</script>

<div>
    {#if analyticsQuery.isLoading}
        <div class="flex justify-center items-center h-64">
            <LoadingSpinner text="Loading analytics..." />
        </div>
    {:else if analyticsQuery.isError}
        <div class="flex justify-center items-center h-64">
            <LoadingError message={analyticsQuery.error.message}>
                <Button onclick={single(() => analyticsQuery.refetch())}
                    >Retry</Button
                >
            </LoadingError>
        </div>
    {:else if analyticsQuery.isSuccess}
        {@render body(analyticsQuery.data)}
    {/if}
</div>

{#snippet body(analytics: OrderAnalytics)}
    <Column class="[&>*]:w-full items-center">
        <Group>
            <div class="flex items-center gap-4">
                <!-- <Button variant="ghost" size="icon" onclick={single(goBack)}>
                    <ArrowLeftIcon />
                </Button> -->
                <SectionHeader
                    icon={TrendingUpIcon}
                    title="Order Analytics"
                    description="Overview of order performance and metrics"
                />
            </div>

            <Group class="md:flex-row-reverse flex-wrap justify-start">
                <ActionButton variant="outline" onclick={single(exportData)}>
                    <DownloadIcon />
                    Export Data
                </ActionButton>
                <ActionButton
                    variant="outline"
                    onclick={single(refreshData)}
                    disabled={analyticsQuery.isRefetching}
                >
                    <RefreshCwIcon />
                    {analyticsQuery.isRefetching ? "Refreshing..." : "Refresh"}
                </ActionButton>
            </Group>
        </Group>

        <!-- Filters -->
        <Group class="flex-wrap">
            <Select.Root type="single" bind:value={dateRange}>
                <Select.Trigger class="w-48">
                    {dateRange === "7d"
                        ? "Last 7 days"
                        : dateRange === "30d"
                          ? "Last 30 days"
                          : dateRange === "90d"
                            ? "Last 90 days"
                            : "All time"}
                </Select.Trigger>
                <Select.Content>
                    <Select.Item value="7d">Last 7 days</Select.Item>
                    <Select.Item value="30d">Last 30 days</Select.Item>
                    <Select.Item value="90d">Last 90 days</Select.Item>
                    <Select.Item value="all">All time</Select.Item>
                </Select.Content>
            </Select.Root>
        </Group>

        <Group class="max-w-4xl md:flex-col md:[&>*]:w-full lg:flex-row">
            <div class="flex-1 space-y-4">
                <!-- Key Metrics -->
                {@render keyMetrics(analytics)}

                <!-- Order Status Breakdown -->
                {@render orderStatusBreakdown(analytics)}

                <!-- Performance Metrics -->
                {@render performanceMetrics(analytics)}
            </div>
        </Group>
    </Column>
{/snippet}

{#snippet keyMetrics(analytics: OrderAnalytics)}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card.Root>
            <Card.Header
                class="flex flex-row items-center justify-between space-y-0 pb-2"
            >
                <Card.Title class="text-sm font-medium">Total Orders</Card.Title
                >
                <ShoppingCartIcon class="h-4 w-4 text-muted-foreground" />
            </Card.Header>
            <Card.Content>
                <div class="text-2xl font-bold">
                    {formatBigInt(analytics.total_orders)}
                </div>
                <p class="text-xs text-muted-foreground">All time orders</p>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header
                class="flex flex-row items-center justify-between space-y-0 pb-2"
            >
                <Card.Title class="text-sm font-medium"
                    >Total Revenue</Card.Title
                >
                <DollarSignIcon class="h-4 w-4 text-muted-foreground" />
            </Card.Header>
            <Card.Content>
                <div class="text-2xl font-bold">
                    {formatCurrency(parseFloat(analytics.total_revenue), "USD")}
                </div>
                <p class="text-xs text-muted-foreground">Gross revenue</p>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header
                class="flex flex-row items-center justify-between space-y-0 pb-2"
            >
                <Card.Title class="text-sm font-medium"
                    >Average Order</Card.Title
                >
                <TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
            </Card.Header>
            <Card.Content>
                <div class="text-2xl font-bold">
                    {formatCurrency(
                        parseFloat(calculateAverageOrderValue(analytics)),
                        "USD",
                    )}
                </div>
                <p class="text-xs text-muted-foreground">Per order value</p>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header
                class="flex flex-row items-center justify-between space-y-0 pb-2"
            >
                <Card.Title class="text-sm font-medium"
                    >Conversion Rate</Card.Title
                >
                <CheckCircleIcon class="h-4 w-4 text-muted-foreground" />
            </Card.Header>
            <Card.Content>
                <div class="text-2xl font-bold">
                    {calculateConversionRate(analytics)}%
                </div>
                <p class="text-xs text-muted-foreground">Completed vs total</p>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}

{#snippet orderStatusBreakdown(analytics: OrderAnalytics)}
    <Card.Root>
        <Card.Header>
            <Card.Title class="flex items-center gap-2">
                <PackageIcon class="w-5 h-5" />
                Order Status Breakdown
            </Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="text-center p-4 border rounded-lg">
                    <div class="flex items-center justify-center mb-2">
                        <ClockIcon class="h-5 w-5 text-yellow-500" />
                    </div>
                    <div class="text-2xl font-bold">
                        {formatBigInt(analytics.pending_orders)}
                    </div>
                    <div class="text-sm text-muted-foreground">
                        Pending Orders
                    </div>
                </div>

                <div class="text-center p-4 border rounded-lg">
                    <div class="flex items-center justify-center mb-2">
                        <CheckCircleIcon class="h-5 w-5 text-green-500" />
                    </div>
                    <div class="text-2xl font-bold">
                        {formatBigInt(analytics.completed_orders)}
                    </div>
                    <div class="text-sm text-muted-foreground">
                        Completed Orders
                    </div>
                </div>

                <div class="text-center p-4 border rounded-lg">
                    <div class="flex items-center justify-center mb-2">
                        <XCircleIcon class="h-5 w-5 text-red-500" />
                    </div>
                    <div class="text-2xl font-bold">
                        {formatBigInt(analytics.cancelled_orders)}
                    </div>
                    <div class="text-sm text-muted-foreground">
                        Cancelled Orders
                    </div>
                </div>

                <div class="text-center p-4 border rounded-lg">
                    <div class="flex items-center justify-center mb-2">
                        <TrendingUpIcon class="h-5 w-5 text-blue-500" />
                    </div>
                    <div class="text-2xl font-bold">
                        {Number(analytics.total_orders) -
                            Number(analytics.pending_orders) -
                            Number(analytics.completed_orders) -
                            Number(analytics.cancelled_orders)}
                    </div>
                    <div class="text-sm text-muted-foreground">In Progress</div>
                </div>
            </div>
        </Card.Content>
    </Card.Root>
{/snippet}

{#snippet performanceMetrics(analytics: OrderAnalytics)}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card.Root>
            <Card.Header>
                <Card.Title>Performance Indicators</Card.Title>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div
                    class="flex justify-between items-center p-3 bg-muted rounded-lg"
                >
                    <span class="font-medium">Completion Rate</span>
                    <div class="flex items-center gap-2">
                        <div class="w-24 bg-gray-200 rounded-full h-2">
                            <div
                                class="bg-green-500 h-2 rounded-full"
                                style="width: {calculateConversionRate(
                                    analytics,
                                )}%"
                            ></div>
                        </div>
                        <span class="text-sm font-medium"
                            >{calculateConversionRate(analytics)}%</span
                        >
                    </div>
                </div>

                <div
                    class="flex justify-between items-center p-3 bg-muted rounded-lg"
                >
                    <span class="font-medium">Cancellation Rate</span>
                    <div class="flex items-center gap-2">
                        <div class="w-24 bg-gray-200 rounded-full h-2">
                            <div
                                class="bg-red-500 h-2 rounded-full"
                                style="width: {calculateCancellationRate(
                                    analytics,
                                )}%"
                            ></div>
                        </div>
                        <span class="text-sm font-medium"
                            >{calculateCancellationRate(analytics)}%</span
                        >
                    </div>
                </div>

                <div
                    class="flex justify-between items-center p-3 bg-muted rounded-lg"
                >
                    <span class="font-medium">Pending Processing</span>
                    <div class="flex items-center gap-2">
                        <div class="w-24 bg-gray-200 rounded-full h-2">
                            <div
                                class="bg-yellow-500 h-2 rounded-full"
                                style="width: {analytics.total_orders > 0n
                                    ? (
                                          (Number(analytics.pending_orders) /
                                              Number(analytics.total_orders)) *
                                          100
                                      ).toFixed(1)
                                    : '0'}%"
                            ></div>
                        </div>
                        <span class="text-sm font-medium">
                            {analytics.total_orders > 0n
                                ? (
                                      (Number(analytics.pending_orders) /
                                          Number(analytics.total_orders)) *
                                      100
                                  ).toFixed(1)
                                : "0"}%
                        </span>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Quick Stats</Card.Title>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="space-y-2">
                    <div class="flex justify-between">
                        <span class="text-muted-foreground"
                            >Orders per day (avg)</span
                        >
                        <span class="font-medium">
                            {analytics.total_orders > 0n
                                ? (Number(analytics.total_orders) / 30).toFixed(
                                      1,
                                  )
                                : "0"}
                        </span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-muted-foreground"
                            >Revenue per day (avg)</span
                        >
                        <span class="font-medium">
                            {formatCurrency(
                                parseFloat(analytics.total_revenue) / 30,
                                "USD",
                            )}
                        </span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-muted-foreground"
                            >Most common status</span
                        >
                        <span class="font-medium">
                            {analytics.completed_orders >
                                analytics.pending_orders &&
                            analytics.completed_orders >
                                analytics.cancelled_orders
                                ? "Completed"
                                : analytics.pending_orders >
                                    analytics.cancelled_orders
                                  ? "Pending"
                                  : "Cancelled"}
                        </span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-muted-foreground">Success rate</span>
                        <span class="font-medium text-green-600">
                            {(
                                100 -
                                parseFloat(calculateCancellationRate(analytics))
                            ).toFixed(1)}%
                        </span>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}
