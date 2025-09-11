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
        PackageIcon,
        PlusIcon,
        StarIcon,
        ImageIcon,
    } from "@lucide/svelte";

    import { useLink } from "@dvcol/svelte-simple-router";
    import { Routes } from ".";
    import { useProductListQuery } from "./service";
    import { createQuery } from "@tanstack/svelte-query";
    import { formatDateTime, snakeToTitleCase } from "../../lib/utils/fmt";
    import type { ProductDto } from "@bindings/ProductDto";
    import type { ProductListQuery } from "@bindings/ProductListQuery";
    import { debounce, single } from "@/lib/event";

    let activeTab = $state<ProductDto["status"] | "">("");
    let searchInput = $state("");
    let searchQuery = $state("");
    let categoryFilter = $state("");
    let featuredFilter = $state<"all" | "featured" | "regular" | undefined>(
        "all",
    );
    let paginationQuery = $state({ page: 1, limit: 10 });

    let fetchParams = $derived<ProductListQuery>({
        search: searchQuery || undefined,
        status: activeTab || undefined,
        category: categoryFilter || undefined,
        featured:
            featuredFilter === "featured"
                ? true
                : featuredFilter === "regular"
                  ? false
                  : undefined,
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

    const query = useProductListQuery(() => fetchParams);

    // TODO : fetch categories from API
    let availableCategories = $derived.by(() => {
        const products = query.data?.products || [];
        const categories = new Set(
            products.map((p) => p.category).filter(Boolean),
        );
        return Array.from(categories);
    });

    function getVariantsPriceRange(variants: any[]) {
        if (!variants.length) return "N/A";
        const prices = variants
            .map((v) => parseFloat(v.price))
            .filter((p) => !isNaN(p));
        if (!prices.length) return "N/A";

        const min = Math.min(...prices);
        const max = Math.max(...prices);

        if (min === max) {
            return `$${min.toFixed(2)}`;
        }
        return `$${min.toFixed(2)} - $${max.toFixed(2)}`;
    }

    function getTotalStock(variants: any[]) {
        return variants.reduce(
            (total, variant) => total + (variant.stocks || 0),
            0,
        );
    }
</script>

<Column>
    <Group>
        <SectionHeader
            icon={PackageIcon}
            title="Products"
            description="Manage your product catalog"
        />
        <ActionButton {@attach useLink({ path: Routes.CREATE_PAGE.path })}>
            <PlusIcon />
            Create Product</ActionButton
        >
    </Group>

    <Group class="flex-wrap">
        <SearchBar
            class="mr-auto"
            bind:value={searchInput}
            searching={query.isRefetching && !query.isFetchedAfterMount}
            placeholder="Search products..."
        />

        <Tabs.Root bind:value={activeTab}>
            <Tabs.List>
                <Tabs.Trigger value="active">Active</Tabs.Trigger>
                <Tabs.Trigger value="inactive">Inactive</Tabs.Trigger>
                <Tabs.Trigger value="archived">Archived</Tabs.Trigger>
                <Tabs.Trigger value="">All</Tabs.Trigger>
            </Tabs.List>
        </Tabs.Root>

        <Select.Root type="single" bind:value={categoryFilter}>
            <Select.Trigger class="w-48">
                {categoryFilter || "All Categories"}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="">All Categories</Select.Item>
                {#each availableCategories as category}
                    <Select.Item value={category}>{category}</Select.Item>
                {/each}
            </Select.Content>
        </Select.Root>

        <Select.Root type="single" bind:value={featuredFilter}>
            <Select.Trigger class="w-32">
                {snakeToTitleCase(featuredFilter)}
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="all">All</Select.Item>
                <Select.Item value="featured">Featured</Select.Item>
                <Select.Item value="regular">Regular</Select.Item>
            </Select.Content>
        </Select.Root>
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
                    <!-- <Table.Head class="w-12">
                        <Checkbox />
                    </Table.Head> -->
                    <Table.Head>Preview</Table.Head>
                    <Table.Head>Product</Table.Head>
                    <Table.Head>Status</Table.Head>
                    <Table.Head>Category</Table.Head>
                    <Table.Head>Price</Table.Head>
                    <Table.Head>Stock</Table.Head>
                    <Table.Head>Variants</Table.Head>
                    <Table.Head>Updated</Table.Head>
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
            <Table.Cell colspan={10} class="h-56 text-center">
                <LoadingError message={query.error.message}>
                    <Button onclick={single(() => query.refetch())}
                        >Retry</Button
                    >
                </LoadingError>
            </Table.Cell>
        </Table.Row>
    {:else if query.isLoading || (query.isFetching && query.isFetchedAfterMount)}
        <Table.Row>
            <Table.Cell colspan={10} class="h-56 text-center">
                <LoadingSpinner text="Loading products..." />
            </Table.Cell>
        </Table.Row>
    {:else if query.data?.products.length === 0}
        <Table.Row>
            <Table.Cell colspan={10} class="h-56 text-center">
                No products found.
            </Table.Cell>
        </Table.Row>
    {:else}
        {#each query.data?.products || [] as product}
            <Table.Row>
                <Table.Cell></Table.Cell>
                <!-- <Table.Cell>
                    <Checkbox />
                </Table.Cell> -->
                <Table.Cell>
                    {#if product.images?.length > 0}
                        <img
                            loading="lazy"
                            src={product.images[0]}
                            alt={product.title}
                            class="w-10 h-10 object-cover rounded border-2 border-dashed border-border"
                        />
                    {:else}
                        <div
                            class="w-10 h-10 bg-gray-100 rounded flex items-center justify-center"
                        >
                            <ImageIcon class="w-4 h-4 text-gray-400" />
                        </div>
                    {/if}
                </Table.Cell>
                <Table.Cell>
                    <div class="flex items-start gap-2">
                        <div
                            class="font-medium underline cursor-pointer hover:text-primary"
                            {@attach useLink({
                                path: Routes.EDIT_PAGE.path,
                                params: { id: product.id },
                            })}
                        >
                            {product.title}
                        </div>
                        {#if product.featured}
                            <StarIcon class="w-4 h-4 text-yellow-500 inline" />
                        {/if}
                    </div>
                </Table.Cell>
                <Table.Cell>
                    <Badge
                        variant={product.status === "active"
                            ? "default"
                            : product.status === "inactive"
                              ? "outline"
                              : "destructive"}
                    >
                        {snakeToTitleCase(product.status)}
                    </Badge>
                </Table.Cell>
                <Table.Cell>{product.category || "—"}</Table.Cell>
                <Table.Cell
                    >{getVariantsPriceRange(product.variants)}</Table.Cell
                >
                <Table.Cell>
                    <div class="text-center">
                        {getTotalStock(product.variants)}
                    </div>
                </Table.Cell>
                <Table.Cell class="text-center"
                    >{product.variants.length}</Table.Cell
                >
                <Table.Cell class="text-sm text-muted-foreground">
                    {formatDateTime(product.updated_at)}
                </Table.Cell>
                <Table.Cell>
                    {@render actions(product.id, product.status)}
                </Table.Cell>
            </Table.Row>
        {/each}
    {/if}
{/snippet}

{#snippet actions(id: string, status: ProductDto["status"])}
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
                    path: Routes.EDIT_PAGE.path,
                    params: { id },
                })}>Edit</DropdownMenu.Item
            >
            <!-- <DropdownMenu.Item>Duplicate</DropdownMenu.Item>
            <DropdownMenu.Separator />
            {#if status === "active"}
                <DropdownMenu.Item>Make Inactive</DropdownMenu.Item>
                <DropdownMenu.Item variant="destructive"
                    >Archive</DropdownMenu.Item
                >
            {:else if status === "inactive"}
                <DropdownMenu.Item>Activate</DropdownMenu.Item>
                <DropdownMenu.Item variant="destructive"
                    >Archive</DropdownMenu.Item
                >
            {:else if status === "archived"}
                <DropdownMenu.Item>Restore</DropdownMenu.Item>
                <DropdownMenu.Item variant="destructive"
                    >Delete</DropdownMenu.Item
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
