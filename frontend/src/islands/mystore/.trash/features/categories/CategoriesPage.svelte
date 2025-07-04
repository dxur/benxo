<script lang="ts">
    import Table from "../../components/Table.svelte";
    import TablePagination from "../../components/TablePagination.svelte";
    import LoadingShow from "../../components/LoadingShow.svelte";
    import Dialog from "../../components/Dialog.svelte";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { writable } from "svelte/store";
    import { useNavigate } from "@dvcol/svelte-simple-router";
    import { AppRoutes } from "../../routes";
    import { notifCenter } from "@/stores/notifications";
    import type { Page } from "@bindings/Page";
    import type { LoadingStatus } from "../../components/LoadingShow.svelte";
    import type { CategoryPublic } from "@bindings/CategoryPublic";
    import { isNone } from "../../lib/utils";

    const { push } = useNavigate();

    let page = writable(1);
    let total = writable(1);
    let dialog: HTMLDialogElement;
    let status: LoadingStatus = undefined;
    let categories: Page<CategoryPublic>;
    let isEditMode = writable(false);

    let fields = default_fields();

    // Filtering and sorting state
    let filters = {
        archived: null as boolean | null,
        period: "all" as "all" | "today" | "week" | "month" | "custom",
        custom_start: "",
        custom_end: "",
    };

    let sorting = {
        sort_by: "created_at" as "name" | "slug" | "created_at" | "archived",
        sort_order: "desc" as "asc" | "desc",
    };

    function default_fields() {
        return {
            id: "",
            name: "",
            slug: "",
        };
    }

    function getDateRange() {
        const today = new Date();
        const startOfDay = new Date(
            today.getFullYear(),
            today.getMonth(),
            today.getDate(),
        );
        const endOfDay = new Date(
            today.getFullYear(),
            today.getMonth(),
            today.getDate(),
            23,
            59,
            59,
        );

        switch (filters.period) {
            case "today":
                return {
                    created_after: startOfDay,
                    created_before: endOfDay,
                };
            case "week":
                const weekStart = new Date(today);
                weekStart.setDate(today.getDate() - today.getDay());
                weekStart.setHours(0, 0, 0, 0);
                return {
                    created_after: weekStart,
                    created_before: endOfDay,
                };
            case "month":
                const monthStart = new Date(
                    today.getFullYear(),
                    today.getMonth(),
                    1,
                );
                return {
                    created_after: monthStart,
                    created_before: endOfDay,
                };
            case "custom":
                return {
                    created_after: filters.custom_start
                        ? new Date(filters.custom_start)
                        : null,
                    created_before: filters.custom_end
                        ? new Date(filters.custom_end)
                        : null,
                };
            default:
                return {};
        }
    }

    function pull(p: number) {
        // Build filter object, only including non-empty values
        const filterParams: any = {
            page: p,
            per_page: null,
            next_token: null,
            sort_by: sorting.sort_by,
            sort_order: sorting.sort_order,
        };

        if (filters.archived !== null) filterParams.archived = filters.archived;

        const dateRange = getDateRange();
        if (dateRange.created_after)
            filterParams.created_after = dateRange.created_after;
        if (dateRange.created_before)
            filterParams.created_before = dateRange.created_before;

        ApiRoutes.get_all_categories(filterParams)
            .then((data) => {
                categories = data;
                total.update((_) =>
                    Math.ceil((data.total ?? 0) / (data.per_page ?? 1)),
                );
                status = null;
            })
            .catch((err) => {
                notifCenter.error(err);
                status = err || "An error occurred";
            });
    }

    function clearFilters() {
        filters = {
            archived: null,
            period: "all",
            custom_start: "",
            custom_end: "",
        };
        page.set(1);
        pull(1);
    }

    function setSorting(field: typeof sorting.sort_by) {
        if (sorting.sort_by === field) {
            sorting.sort_order = sorting.sort_order === "asc" ? "desc" : "asc";
        } else {
            sorting.sort_by = field;
            sorting.sort_order = "asc";
        }
        page.set(1);
        pull(1);
    }

    function getSortIcon(field: typeof sorting.sort_by) {
        if (sorting.sort_by !== field) return "↕";
        return sorting.sort_order === "asc" ? "↑" : "↓";
    }

    function edit(category: CategoryPublic) {
        fields = { id: category.id, name: category.name, slug: category.slug };
        isEditMode.set(true);
        dialog.showModal();
    }

    function update(id: string) {
        const body = {
            archived: null,
            name:
                fields.name === categories.data.find((c) => c.id === id)?.name
                    ? null
                    : fields.name.trim(),
            slug:
                fields.slug === categories.data.find((c) => c.id === id)?.slug
                    ? null
                    : fields.slug.trim(),
        };

        if (isNone(body)) {
            notifCenter.warning("Nothing to update");
            return;
        }

        ApiRoutes.update_category({ id, body })
            .then(() => {
                notifCenter.success("Category updated successfully");
                pull($page);
            })
            .catch((err) => notifCenter.error(err));
    }

    function create() {
        ApiRoutes.create_category({
            name: fields.name.trim(),
            slug: fields.slug.trim(),
        })
            .then((p) => {
                notifCenter.success("Category created successfully");
                pull($page);
            })
            .catch((err) => notifCenter.error(err));
    }

    function archive(id: string, archived: boolean) {
        ApiRoutes.update_category({
            id,
            body: { archived, name: null, slug: null },
        })
            .then(() => {
                notifCenter.success("Category updated successfully");
                pull($page);
            })
            .catch((err) => notifCenter.error(err));
    }

    // Reactive statements
    $: pull($page);
    $: {
        // Immediate filtering when any filter changes
        filters.archived,
            filters.period,
            filters.custom_start,
            filters.custom_end;
        page.set(1);
        pull(1);
    }
    $: document.title = "Categories";
</script>

<LoadingShow {status}>
    <header>
        <h1>Categories</h1>
        <button
            on:click={() => {
                fields = default_fields();
                isEditMode.set(false);
                dialog.showModal();
            }}
        >
            New Category
        </button>
    </header>

    <div class="filters-section">
        <div class="filters-row">
            <div class="filter-group">
                <label>Status</label>
                <select bind:value={filters.archived} class="filter-select">
                    <option value={null}>All Categories</option>
                    <option value={false}>Active Only</option>
                    <option value={true}>Archived Only</option>
                </select>
            </div>

            <div class="filter-group">
                <label>Created</label>
                <select bind:value={filters.period} class="filter-select">
                    <option value="all">All Time</option>
                    <option value="today">Today</option>
                    <option value="week">This Week</option>
                    <option value="month">This Month</option>
                    <option value="custom">Custom Range</option>
                </select>
            </div>

            {#if filters.period === "custom"}
                <div class="custom-date-range">
                    <div class="date-group">
                        <label>From</label>
                        <input
                            type="date"
                            bind:value={filters.custom_start}
                            class="date-input"
                        />
                    </div>
                    <div class="date-group">
                        <label>To</label>
                        <input
                            type="date"
                            bind:value={filters.custom_end}
                            class="date-input"
                        />
                    </div>
                </div>
            {/if}

            <button type="button" on:click={clearFilters} class="clear-filters">
                Clear Filters
            </button>
        </div>
    </div>

    {#if categories?.data?.length}
        <Table>
            <thead>
                <tr>
                    <th>
                        <button
                            type="button"
                            class="sort-header"
                            on:click={() => setSorting("slug")}
                        >
                            Slug <span class="sort-icon"
                                >{getSortIcon("slug")}</span
                            >
                        </button>
                    </th>
                    <th>
                        <button
                            type="button"
                            class="sort-header"
                            on:click={() => setSorting("name")}
                        >
                            Name <span class="sort-icon"
                                >{getSortIcon("name")}</span
                            >
                        </button>
                    </th>
                    <th>
                        <button
                            type="button"
                            class="sort-header"
                            on:click={() => setSorting("archived")}
                        >
                            Archived <span class="sort-icon"
                                >{getSortIcon("archived")}</span
                            >
                        </button>
                    </th>
                    <th>
                        <button
                            type="button"
                            class="sort-header"
                            on:click={() => setSorting("created_at")}
                        >
                            Created At <span class="sort-icon"
                                >{getSortIcon("created_at")}</span
                            >
                        </button>
                    </th>
                    <th>Options</th>
                </tr>
            </thead>
            <tbody>
                {#each categories.data as category}
                    <tr class={category.archived ? "archived" : ""}>
                        <td>{category.slug}</td>
                        <td>{category.name}</td>
                        <td>{category.archived ? "Yes" : "No"}</td>
                        <td>{new Date(category.created_at).toLocaleString()}</td
                        >
                        <td>
                            {#if category.archived}
                                <button
                                    type="button"
                                    on:click={() => archive(category.id, false)}
                                >
                                    Unarchive
                                </button>
                            {:else}
                                <button on:click={() => edit(category)}
                                    >Edit</button
                                >
                                <button
                                    type="reset"
                                    on:click={() => archive(category.id, true)}
                                >
                                    Archive
                                </button>
                            {/if}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </Table>
        <TablePagination {page} {total} />
    {:else}
        <div class="empty-state">
            <span>No categories found</span>
            {#if filters.archived !== null || filters.period !== "all"}
                <button type="button" on:click={clearFilters}
                    >Clear filters</button
                >
            {/if}
        </div>
    {/if}

    <Dialog bind:dialog>
        <header>
            <h2>{$isEditMode ? "Edit" : "Create"} Category</h2>
        </header>
        <form
            on:submit={(ev) => {
                ev.preventDefault();
                if ($isEditMode) {
                    update(fields.id);
                } else {
                    create();
                }
                dialog.close();
            }}
        >
            <fieldset>
                <label>
                    Slug
                    <input
                        type="text"
                        bind:value={fields.slug}
                        pattern="^[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$"
                        required
                    />
                </label>
            </fieldset>
            <fieldset>
                <label>
                    Name
                    <input type="text" bind:value={fields.name} required />
                </label>
            </fieldset>
            <button type="submit">Submit</button>
            <button type="button" on:click={() => dialog.close()}>Close</button>
        </form>
    </Dialog>
</LoadingShow>

<style>
    .archived td {
        text-decoration: line-through;
        opacity: 0.7;
    }
    .archived td:has([type="button"]) {
        text-decoration: none;
        opacity: 1;
    }
    button[type="button"] {
        color: var(--success);
    }

    /* Header */
    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: var(--spacing-base);
    }

    /* Filters Section */
    .filters-section {
        background: var(--surface);
        border: var(--border);
        border-radius: var(--radius);
        padding: var(--spacing-lg);
        margin-bottom: var(--spacing-lg);
    }

    .filters-row {
        display: flex;
        gap: var(--spacing-xl);
        align-items: flex-end;
        flex-wrap: wrap;
    }

    .filter-group {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
    }

    .filter-group label {
        font-size: var(--font-size-sm);
        font-weight: var(--font-weight-medium);
        color: var(--text-muted);
    }

    .filter-select,
    .date-input {
        padding: var(--spacing-sm) var(--spacing-base);
        border: var(--border);
        border-radius: var(--radius-small);
        background: var(--bg);
        color: var(--on-bg);
        font-size: var(--font-size-sm);
        min-width: 140px;
        transition: var(--transition-base);
    }

    .filter-select:focus,
    .date-input:focus {
        outline: none;
        border-color: var(--primary);
        box-shadow: 0 0 0 2px rgba(var(--primary-rgb), 0.2);
    }

    .custom-date-range {
        display: flex;
        gap: var(--spacing-base);
        align-items: flex-end;
    }

    .date-group {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
    }

    .date-group label {
        font-size: var(--font-size-xs);
        font-weight: var(--font-weight-medium);
        color: var(--text-muted);
    }

    .clear-filters {
        padding: var(--spacing-sm) var(--spacing-lg);
        border: var(--border);
        border-radius: var(--radius-small);
        background: var(--bg);
        color: var(--text-muted);
        cursor: pointer;
        font-size: var(--font-size-sm);
        font-weight: var(--font-weight-medium);
        transition: var(--transition-base);
        white-space: nowrap;
        height: fit-content;
    }

    .clear-filters:hover {
        background: var(--bg-hover);
        color: var(--on-bg);
        border-color: var(--border-hover);
    }

    @media (max-width: 768px) {
        .filters-row {
            flex-direction: column;
            align-items: stretch;
            gap: var(--spacing-base);
        }

        .filter-group {
            flex-direction: row;
            align-items: center;
            justify-content: space-between;
        }

        .filter-group label {
            margin-bottom: 0;
            flex-shrink: 0;
        }

        .filter-select,
        .date-input {
            min-width: 120px;
        }

        .custom-date-range {
            flex-direction: column;
            gap: var(--spacing-base);
        }
    }

    /* Sortable Headers */
    .sort-header {
        background: none !important;
        border: none !important;
        padding: 0 !important;
        margin: 0 !important;
        color: var(--text-muted) !important;
        font-weight: var(--font-weight-medium) !important;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: var(--spacing-xs);
        width: 100%;
        justify-content: flex-start;
        font-size: var(--font-size-sm);
    }

    .sort-header:hover {
        color: var(--on-bg) !important;
        background: none !important;
    }

    .sort-icon {
        font-size: var(--font-size-xs);
        opacity: 0.7;
        transition: var(--transition-base);
    }

    .sort-header:hover .sort-icon {
        opacity: 1;
    }

    /* Empty State */
    .empty-state {
        text-align: center;
        padding: var(--spacing-xl);
        color: var(--text-muted);
    }

    .empty-state button {
        margin-top: var(--spacing-base);
        padding: var(--spacing-xs) var(--spacing-sm);
        border: var(--border);
        border-radius: var(--radius-small);
        background: var(--surface);
        color: var(--text-muted);
        cursor: pointer;
        font-size: var(--font-size-sm);
        transition: var(--transition-base);
    }

    .empty-state button:hover {
        background: var(--bg-hover);
        color: var(--on-bg);
    }
</style>
