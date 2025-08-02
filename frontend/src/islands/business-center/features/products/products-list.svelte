<script lang="ts">
    import {
        ImageIcon,
        PackageIcon,
        PlusIcon,
        SearchIcon,
    } from "@lucide/svelte";
    import * as Tabs from "$lib/components/ui/tabs/index";
    import { Button } from "$lib/components/ui/button/index";
    import { Badge } from "$lib/components/ui/badge/index";
    import {
        createSvelteTable,
        FlexRender,
        renderSnippet,
    } from "$lib/components/ui/data-table/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import {
        getCoreRowModel,
        getPaginationRowModel,
        getFilteredRowModel,
        type ColumnDef,
        type ColumnFiltersState,
        type PaginationState,
    } from "@tanstack/table-core";
    import Column from "../../lib/components/layout/column.svelte";
    import Group from "../../lib/components/layout/group.svelte";
    import TableSkeleton from "../../lib/components/table-skeleton.svelte";
    import ActionButton from "../../lib/components/action-button.svelte";
    import SectionHeader from "../../lib/components/section-header.svelte";
    import SearchBar from "../../lib/components/search-bar.svelte";
    import { toast } from "svelte-sonner";
    import { create_product, list_products } from "@bindings/ProductRoutes";
    import type { ProductView } from "@bindings/ProductView";
    import type { ProductListResponse } from "@bindings/ProductListResponse";
    import { untrack } from "svelte";
    import { useLink } from "@dvcol/svelte-simple-router";
    import { Routes } from ".";

    let data: ProductListResponse | undefined = $state(undefined);

    const columns = <ColumnDef<ProductView>[]>[
        {
            accessorKey: "preview",
            header: "Preview",
            cell: ({ row }) => {
                return renderSnippet(
                    productPreview,
                    row.original.images[0] || null,
                );
            },
        },
        {
            accessorKey: "title",
            header: "Title",
        },
        {
            accessorKey: "slug",
            header: "Slug",
            cell: ({ row }) => {
                return renderSnippet(withDefault, {
                    value: row.getValue("slug"),
                });
            },
        },
        {
            accessorKey: "status",
            header: "Status",
            cell: ({ row }) => {
                return renderSnippet(statusView, row.getValue("status"));
            },
        },
        {
            accessorKey: "featured",
            header: "Featured",
            cell: ({ row }) => {
                return renderSnippet(booleanCell, row.getValue("featured"));
            },
        },
        {
            accessorKey: "category",
            header: "Category",
            cell: ({ row }) => {
                return renderSnippet(withDefault, {
                    value: row.getValue("category"),
                });
            },
        },
        {
            accessorKey: "updated_at",
            header: "Last Update",
            cell: ({ row }) => {
                return renderSnippet(dateView, row.getValue("updated_at"));
            },
        },
    ];

    let pagination = $state(<PaginationState>{ pageIndex: 0, pageSize: 5 });
    let columnFilters = $state(<ColumnFiltersState>[]);
    let activeTab = $state("active");

    const table = createSvelteTable({
        get data() {
            return data?.products || [];
        },
        columns,
        getCoreRowModel: getCoreRowModel(),
        getPaginationRowModel: getPaginationRowModel(),
        getFilteredRowModel: getFilteredRowModel(),
        onPaginationChange: (updater) => {
            if (typeof updater === "function") {
                pagination = updater(pagination);
            } else {
                pagination = updater;
            }
        },
        onColumnFiltersChange: (updater) => {
            if (typeof updater === "function") {
                columnFilters = updater(columnFilters);
            } else {
                columnFilters = updater;
            }
        },
        state: {
            get pagination() {
                return pagination;
            },
            get columnFilters() {
                return columnFilters;
            },
        },
    });

    $effect(() => {
        const tab = activeTab === "all" ? "" : activeTab;
        untrack(() => table.getColumn("status")?.setFilterValue(tab));
    });

    async function loadProducts() {
        data = await list_products();
    }

    async function createProduct() {
        await create_product()
            .then((_) => {
                toast.success("Product has been created");
            })
            .catch((e) => console.error(e));
    }

    async function handleRowAction() {}
</script>

{#snippet productPreview(image: string | null)}
    <div
        class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10"
    >
        {#if image}
            <img src={image} alt="preview" />
        {:else}
            <ImageIcon class="text-muted-foreground" />
        {/if}
    </div>
{/snippet}

{#snippet booleanCell(value: boolean)}
    <Badge>
        {value ? "Yes" : "No"}
    </Badge>
{/snippet}

{#snippet withDefault({ value, other = "N/A" }: any)}
    {value ? value : other}
{/snippet}

{#snippet statusView(value: string)}
    <Badge>
        {value.toUpperCase()}
    </Badge>
{/snippet}

{#snippet dateView(datetime: string)}
    <time> {datetime} </time>
{/snippet}

<Column>
    <Group>
        <SectionHeader
            icon={PackageIcon}
            title="Products"
            description="Manage your products and pricing"
        />
        <ActionButton icon={PlusIcon} onclick={createProduct}>
            Add Product</ActionButton
        >
    </Group>

    <Group>
        <SearchBar
            value={table.getColumn("title")?.getFilterValue() ?? ""}
            oninput={(e: any) => {
                table.getColumn("title")?.setFilterValue(e.currentTarget.value);
            }}
        />
        <Tabs.Root bind:value={activeTab}>
            <Tabs.List>
                <Tabs.Trigger value="active">Active</Tabs.Trigger>
                <Tabs.Trigger value="inactive">Inactive</Tabs.Trigger>
                <Tabs.Trigger value="draft">Draft</Tabs.Trigger>
                <Tabs.Trigger value="all">All</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>
    </Group>

    {#await loadProducts()}
        <TableSkeleton />
    {:then}
        <div class="rounded-md border">
            <Table.Root>
                <Table.Header>
                    {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
                        <Table.Row>
                            {#each headerGroup.headers as header (header.id)}
                                <Table.Head colspan={header.colSpan}>
                                    {#if !header.isPlaceholder}
                                        <FlexRender
                                            content={header.column.columnDef
                                                .header}
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
                            {@attach useLink({
                                path: Routes.EDIT_PAGE.path,
                                params: { id: row.original.id },
                            })}
                        >
                            {#each row.getVisibleCells() as cell (cell.id)}
                                <Table.Cell>
                                    <FlexRender
                                        content={cell.column.columnDef.cell}
                                        context={cell.getContext()}
                                    />
                                </Table.Cell>
                            {/each}
                        </Table.Row>
                    {:else}
                        <Table.Row>
                            <Table.Cell
                                colspan={columns.length}
                                class="h-24 text-center"
                            >
                                No results.
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </div>
        <div class="flex items-center justify-end space-x-2 py-4">
            <Button
                variant="outline"
                size="sm"
                onclick={() => table.previousPage()}
                disabled={!table.getCanPreviousPage()}
            >
                Previous
            </Button>
            <Button
                variant="outline"
                size="sm"
                onclick={() => table.nextPage()}
                disabled={!table.getCanNextPage()}
            >
                Next
            </Button>
        </div>
    {:catch error}
        {error}
    {/await}
</Column>
