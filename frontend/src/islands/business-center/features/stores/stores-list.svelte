<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index";
    import * as Pagination from "$lib/components/ui/pagination/index";
    import * as Table from "$lib/components/ui/table/index";
    import * as Card from "$lib/components/ui/card/index";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index";
    import { Button } from "@/lib/components/ui/button";
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
        ShoppingBagIcon,
        PlusIcon,
    } from "@lucide/svelte";

    import { useLink } from "@dvcol/svelte-simple-router";
    import { Routes } from ".";
    import { listStores } from "./service";
    import { createQuery } from "@tanstack/svelte-query";
    import { formatDateTime, snakeToTitleCase } from "../../lib/utils/fmt";
    import type { StoreDto } from "@bindings/StoreDto";
    import { debounce, single } from "@/lib/event";

    let activeTab = $state<StoreDto["status"] | "">("");
    let searchInput = $state("");
    let searchQuery = $state("");
    let paginationQuery = $state({ page: 1, limit: 10 });

    let fetchParams = $derived({
        search: searchQuery || undefined,
        status: activeTab || undefined,
        ...paginationQuery,
    });

    const debouncedSearch = debounce((value) => {
        searchQuery = value;
        paginationQuery.page = 1;
    }, 750);

    $effect(() => {
        debouncedSearch(searchInput);
    });

    const query = createQuery(() => ({
        queryKey: ["stores", fetchParams],
        queryFn: () => listStores(fetchParams),
        placeholderData: (prev) => prev,
    }));
</script>

<Column>
    <Group>
        <SectionHeader
            icon={ShoppingBagIcon}
            title="Stores"
            description="Manage your selling endpoints"
        />
        <ActionButton {@attach useLink({ path: Routes.CREATE_PAGE.path })}>
            <PlusIcon />
            Create A Store</ActionButton
        >
    </Group>

    <Group>
        <SearchBar
            bind:value={searchInput}
            searching={query.isRefetching && !query.isFetchedAfterMount}
            placeholder="Search stores..."
        />
        <Tabs.Root bind:value={activeTab}>
            <Tabs.List>
                <Tabs.Trigger value="active">Active</Tabs.Trigger>
                <Tabs.Trigger value="inactive">Inactive</Tabs.Trigger>
                <Tabs.Trigger value="archived">Archived</Tabs.Trigger>
                <Tabs.Trigger value="">All</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>
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
                    <Table.Head>Store</Table.Head>
                    <Table.Head>Status</Table.Head>
                    <Table.Head>Description</Table.Head>
                    <Table.Head>Updated At</Table.Head>
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
            <Table.Cell colspan={8} class="h-56 text-center">
                <LoadingError message={query.error.message}>
                    <Button onclick={single(() => query.refetch())}
                        >Retry</Button
                    >
                </LoadingError>
            </Table.Cell>
        </Table.Row>
    {:else if query.isLoading || (query.isFetching && query.isFetchedAfterMount)}
        <Table.Row>
            <Table.Cell colspan={8} class="h-56 text-center">
                <LoadingSpinner text="Loading..." />
            </Table.Cell>
        </Table.Row>
    {:else if query.data?.stores.length === 0}
        <Table.Row>
            <Table.Cell colspan={8} class="h-56 text-center">
                No stores found.
            </Table.Cell>
        </Table.Row>
    {:else}
        {#each query.data?.stores || [] as store}
            <Table.Row>
                <Table.Cell></Table.Cell>
                <Table.Cell
                    class="underline cursor-pointer"
                    {@attach useLink({
                        path: Routes.EDIT_PAGE.path,
                        params: { id: store.id },
                    })}>{store.name}</Table.Cell
                >
                <Table.Cell>{snakeToTitleCase(store.status)}</Table.Cell>
                <Table.Cell class="max-w-[200px] truncate"
                    >{store.description || "N/A"}</Table.Cell
                >
                <Table.Cell>{formatDateTime(store.updated_at)}</Table.Cell>
                <Table.Cell>
                    {@render actions(store.id, store.status)}
                </Table.Cell>
            </Table.Row>
        {/each}
    {/if}
{/snippet}

{#snippet actions(id: string, status: StoreDto["status"])}
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
        <DropdownMenu.Content align="end" class="w-32">
            <DropdownMenu.Item
                {@attach useLink({
                    path: Routes.EDIT_PAGE.path,
                    params: { id },
                })}>Edit</DropdownMenu.Item
            >
            <!-- <DropdownMenu.Separator />
            {#if status === "active"}
                <DropdownMenu.Item variant="destructive"
                    >Make Inactive</DropdownMenu.Item
                >
                <DropdownMenu.Item variant="destructive"
                    >Archive</DropdownMenu.Item
                >
            {:else if status === "archived"}
                <DropdownMenu.Item>Restore</DropdownMenu.Item>
                <DropdownMenu.Item>Make Active</DropdownMenu.Item>
            {:else if status === "inactive"}
                <DropdownMenu.Item>Make Active</DropdownMenu.Item>
                <DropdownMenu.Item variant="destructive"
                    >Archive</DropdownMenu.Item
                >
            {/if} -->
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
                            <Pagination.Link
                                {page}
                                isActive={currentPage === page.value}
                            >
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
