<script lang="ts" generics="TData, TValue">
    import {
        type PaginationState,
        type ColumnDef,
        getCoreRowModel,
        getFilteredRowModel,
        type VisibilityState,
    } from "@tanstack/table-core";
    import {
        createSvelteTable,
        FlexRender,
    } from "@/lib/components/ui/data-table/index.js";
    import { Button } from "@/lib/components/ui/button/index.js";
    import * as Table from "@/lib/components/ui/table/index.js";
    import type { Pagination } from "@bindings/Pagination";
    import { Input } from "@/lib/components/ui/input";
    import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
    import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
    import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
    import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
    import * as Select from "@/lib/components/ui/select/index";
    import { writable } from "svelte/store";
    import type { HTMLTableAttributes } from "svelte/elements";

    type DataTableProps<TData, TValue> = {
        columns: ColumnDef<TData, TValue>[];
        data: TData[];
        per_page: number;
        page: number;
        total_pages: number;
        restProps: HTMLTableAttributes;
    };

    let {
        data,
        columns,
        per_page = $bindable(),
        page = $bindable(),
        total_pages = $bindable(),
        ...restProps
    }: DataTableProps<TData, TValue> = $props();

    // Column visibility state
    const columnVisibility = writable<VisibilityState>({});

    const table = createSvelteTable({
        get data() {
            return data;
        },
        columns,
        getCoreRowModel: getCoreRowModel(),
        state: {
            get columnVisibility() {
                return $columnVisibility;
            },
        },
        onColumnVisibilityChange: (updater) => {
            if (typeof updater === "function") {
                columnVisibility.update(updater);
            } else {
                columnVisibility.set(updater);
            }
        },
    });

    let visibleColumns: number = $derived(table.getVisibleLeafColumns().length);
    let hiddenColumnsCount: number = $derived(
        table.getAllLeafColumns().length - visibleColumns,
    );
</script>

<div class="space-y-4">
    <!-- Table Controls -->
    <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
            <span class="text-sm text-muted-foreground">
                Showing {data.length} results
            </span>
            {#if total_pages > 1}
                <Badge variant="outline" class="text-xs">
                    Page {page} of {total_pages}
                </Badge>
            {/if}
        </div>
        <div class="flex items-center gap-2">
            <!-- Per page select -->
            <Select.Root
                type="single"
                onValueChange={(value) => (per_page = parseInt(value))}
            >
                <Select.Trigger class="w-[180px]">
                    Rows per page: {per_page}
                </Select.Trigger>
                <Select.Content>
                    {#each [5, 10, 20, 30, 40, 50] as pageSize (pageSize)}
                        <Select.Item value={String(pageSize)}>
                            {pageSize}
                        </Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
        </div>
    </div>

    <!-- Table Container -->
    <div class="rounded-lg border bg-card">
        <Table.Root {...restProps}>
            <Table.Header>
                {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
                    <Table.Row class="hover:bg-transparent border-b">
                        {#each headerGroup.headers as header (header.id)}
                            <Table.Head
                                colspan={header.colSpan}
                                class="h-12 font-semibold text-foreground"
                            >
                                {#if !header.isPlaceholder}
                                    <FlexRender
                                        content={header.column.columnDef.header}
                                        context={header.getContext()}
                                    />
                                {/if}
                            </Table.Head>
                        {/each}
                    </Table.Row>
                {/each}
            </Table.Header>
            <Table.Body>
                {#each table.getRowModel().rows as row (row.id)}
                    <Table.Row
                        data-state={row.getIsSelected() && "selected"}
                        class="hover:bg-muted/50 transition-colors"
                    >
                        {#each row.getVisibleCells() as cell (cell.id)}
                            <Table.Cell class="py-3">
                                <FlexRender
                                    content={cell.column.columnDef.cell}
                                    context={cell.getContext()}
                                />
                            </Table.Cell>
                        {/each}
                    </Table.Row>
                {:else}
                    <Table.Row class="hover:bg-transparent">
                        <Table.Cell
                            colspan={columns.length}
                            class="h-32 text-center"
                        >
                            <div
                                class="flex flex-col items-center gap-2 text-muted-foreground"
                            >
                                <div class="rounded-full bg-muted p-3">
                                    <svg
                                        class="h-6 w-6"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2M4 13h2m0 0V9a2 2 0 012-2h2m0 0V6a2 2 0 012-2h2m0 0v1"
                                        />
                                    </svg>
                                </div>
                                <div class="font-medium">Nothing found</div>
                                <div class="text-sm">
                                    Try adjusting your search or filters
                                </div>
                            </div>
                        </Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>

    <!-- Pagination -->
    {#if total_pages > 1}
        <div class="flex items-center justify-between">
            <div class="text-sm text-muted-foreground">
                Page {page} of {total_pages}
            </div>

            <div class="flex items-center gap-2">
                <Button
                    variant="outline"
                    size="sm"
                    onclick={() => (page = Math.max(page - 1, 1))}
                    disabled={page === 1}
                    class="gap-1"
                >
                    <ChevronLeftIcon class="h-4 w-4" />
                    Previous
                </Button>

                <!-- Page numbers for small pagination -->
                {#if total_pages <= 7}
                    {#each Array(total_pages) as _, i}
                        <Button
                            variant={page === i + 1 ? "default" : "outline"}
                            size="sm"
                            onclick={() => (page = i + 1)}
                            class="w-8"
                        >
                            {i + 1}
                        </Button>
                    {/each}
                {:else}
                    <!-- Smart pagination for larger sets -->
                    {#if page > 3}
                        <Button
                            variant="outline"
                            size="sm"
                            onclick={() => (page = 1)}
                            class="w-8"
                        >
                            1
                        </Button>
                        {#if page > 4}
                            <span class="text-muted-foreground">...</span>
                        {/if}
                    {/if}

                    {#each Array(Math.min(5, total_pages)) as _, i}
                        {@const pageNum =
                            Math.max(1, Math.min(total_pages - 4, page - 2)) +
                            i}
                        {#if pageNum <= total_pages}
                            <Button
                                variant={page === pageNum
                                    ? "default"
                                    : "outline"}
                                size="sm"
                                onclick={() => (page = pageNum)}
                                class="w-8"
                            >
                                {pageNum}
                            </Button>
                        {/if}
                    {/each}

                    {#if page < total_pages - 2}
                        {#if page < total_pages - 3}
                            <span class="text-muted-foreground">...</span>
                        {/if}
                        <Button
                            variant="outline"
                            size="sm"
                            onclick={() => (page = total_pages)}
                            class="w-8"
                        >
                            {total_pages}
                        </Button>
                    {/if}
                {/if}

                <Button
                    variant="outline"
                    size="sm"
                    onclick={() => (page = Math.min(page + 1, total_pages))}
                    disabled={page >= total_pages}
                    class="gap-1"
                >
                    Next
                    <ChevronRightIcon class="h-4 w-4" />
                </Button>
            </div>
        </div>
    {/if}
</div>
