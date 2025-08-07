<script lang="ts">
    import type { Snippet } from "svelte";
    import { PackageIcon, PlusIcon } from "@lucide/svelte";
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
    import { untrack } from "svelte";
    import { useLink } from "@dvcol/svelte-simple-router";
    import { Routes } from ".";
    import { useNavigate } from "@dvcol/svelte-simple-router/router";
    import type { StoreListResponse } from "@bindings/StoreListResponse";
    import type { StoreListQuery } from "@bindings/StoreListQuery";
    import type { StoreDto } from "@bindings/StoreDto";
    import { list_stores, create_store } from "@bindings/StoreRoutes";

    const { push } = useNavigate();

    let data: StoreListResponse | undefined = $state(undefined);

    const columns = <ColumnDef<StoreDto>[]>[
        {
            accessorKey: "name",
            header: "Name",
        },
        {
            accessorKey: "description",
            header: "Description",
            cell: ({ row }) => {
                return renderSnippet(withDefault as Snippet, {
                    value: row.getValue("description"),
                });
            },
        },
        {
            accessorKey: "status",
            header: "Status",
            cell: ({ row }) => {
                return renderSnippet(
                    statusView as Snippet,
                    row.getValue("status"),
                );
            },
        },
        {
            accessorKey: "updated_at",
            header: "Last Update",
            cell: ({ row }) => {
                return renderSnippet(
                    dateView as Snippet,
                    row.getValue("updated_at"),
                );
            },
        },
    ];

    let pagination = $state(<PaginationState>{ pageIndex: 0, pageSize: 5 });
    let columnFilters = $state(<ColumnFiltersState>[]);
    let activeTab = $state("active");

    const table = createSvelteTable({
        get data() {
            return data?.stores || [];
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

    let searchQuery = $state("");
    let query = $state<StoreListQuery>({
        page: null,
        limit: null,
        status: "active",
        search: null,
    });

    $effect(() => {
        const tab = activeTab === "all" ? "" : activeTab;
        untrack(() => table.getColumn("status")?.setFilterValue(tab));
    });

    async function loadStores() {
        data = await list_stores();
    }

    async function createStore() {
        await create_store()
            .then((data) => {
                toast.success("Store has been created");
                push({ path: Routes.EDIT_PAGE.path, params: { id: data.id } });
            })
            .catch((e) => console.error(e));
    }

    async function handleRowAction() {}
</script>

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
            title="Stores"
            description="Manage your selling endpoints"
        />
        <ActionButton onclick={createStore}>
            <PlusIcon />
            Create A Store</ActionButton
        >
    </Group>

    <Group>
        <SearchBar bind:value={searchQuery} placeholder="Search stores..." />
        <Tabs.Root bind:value={activeTab}>
            <Tabs.List>
                <Tabs.Trigger value="active">Active</Tabs.Trigger>
                <Tabs.Trigger value="inactive">Inactive</Tabs.Trigger>
                <Tabs.Trigger value="draft">Draft</Tabs.Trigger>
                <Tabs.Trigger value="all">All</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>
    </Group>

    {#key query}
        {#await loadStores()}
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
    {/key}
</Column>
