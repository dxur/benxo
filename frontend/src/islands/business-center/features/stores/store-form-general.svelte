<script lang="ts">
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Textarea } from "$lib/components/ui/textarea/index";
    import * as Card from "$lib/components/ui/card/index";
    import { Button } from "$lib/components/ui/button/index";
    import type { Form } from "../../lib/utils/form";
    import type { StoreSchema } from "./service";
    import { TrashIcon, PlusIcon, GripVerticalIcon } from "@lucide/svelte";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof StoreSchema>;
    } = $props();

    // Drag and drop state
    let draggedItemIndex = $state<number | null>(null);
    let draggedItemType = $state<
        "social" | "menu" | "collection" | "footer" | null
    >(null);

    // 1. ADD these new state variables after the existing drag state variables (around line 20):
    let draggedSubItemIndex = $state<number | null>(null);
    let draggedSubItemListIndex = $state<number | null>(null);

    // 2. ADD these new functions after the existing handleDrop function (around line 150):
    function handleSubItemDragStart(
        event: DragEvent,
        listIndex: number,
        itemIndex: number,
    ) {
        if (!event.dataTransfer) return;

        draggedSubItemIndex = itemIndex;
        draggedSubItemListIndex = listIndex;
        event.dataTransfer.effectAllowed = "move";

        // Add visual feedback
        const target = event.target as HTMLElement;
        target.style.opacity = "0.5";
    }

    function handleSubItemDragEnd(event: DragEvent) {
        const target = event.target as HTMLElement;
        target.style.opacity = "1";
        draggedSubItemIndex = null;
        draggedSubItemListIndex = null;
    }

    function handleSubItemDrop(
        event: DragEvent,
        listIndex: number,
        dropIndex: number,
    ) {
        event.preventDefault();

        if (
            draggedSubItemIndex === null ||
            draggedSubItemListIndex === null ||
            draggedSubItemListIndex !== listIndex ||
            draggedSubItemIndex === dropIndex
        ) {
            return;
        }

        // Reorder the sub-items array
        const items = form.footer_lists.value[listIndex].items;
        const draggedItem = items[draggedSubItemIndex];
        items.splice(draggedSubItemIndex, 1);
        items.splice(dropIndex, 0, draggedItem);

        // Update the form
        form.footer_lists.value[listIndex].items = items;
    }

    function addSocialLink() {
        form.social_links.value.push({
            platform: "",
            url: "",
        });
    }

    function removeSocialLink(index: number) {
        form.social_links.value.splice(index, 1);
    }

    function addMenuItem() {
        form.menu_items.value.push({
            label: "",
            url: "",
        });
    }

    function removeMenuItem(index: number) {
        form.menu_items.value.splice(index, 1);
    }

    function addFeaturedCollection() {
        form.featured_collections.value.push({ label: "", img: null });
    }

    function removeFeaturedCollection(index: number) {
        form.featured_collections.value.splice(index, 1);
    }

    function addFooterList() {
        form.footer_lists.value.push({ title: "", items: [] });
    }

    function removeFooterList(index: number) {
        form.footer_lists.value.splice(index, 1);
    }

    function addFooterItem(listIndex: number) {
        form.footer_lists.value[listIndex].items.push({ label: "", url: "" });
    }

    function removeFooterItem(listIndex: number, itemIndex: number) {
        form.footer_lists.value[listIndex].items.splice(itemIndex, 1);
    }

    // Drag and drop handlers
    function handleDragStart(
        event: DragEvent,
        index: number,
        type: "social" | "menu" | "collection" | "footer",
    ) {
        if (!event.dataTransfer) return;

        draggedItemIndex = index;
        draggedItemType = type;
        event.dataTransfer.effectAllowed = "move";

        // Add visual feedback
        const target = event.target as HTMLElement;
        target.style.opacity = "0.5";
    }

    function handleDragEnd(event: DragEvent) {
        const target = event.target as HTMLElement;
        target.style.opacity = "1";
        draggedItemIndex = null;
        draggedItemType = null;
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
        if (event.dataTransfer) {
            event.dataTransfer.dropEffect = "move";
        }
    }

    function handleDrop(
        event: DragEvent,
        dropIndex: number,
        type: "social" | "menu" | "collection" | "footer",
    ) {
        event.preventDefault();

        if (
            draggedItemIndex === null ||
            draggedItemType !== type ||
            draggedItemIndex === dropIndex
        ) {
            return;
        }

        // Reorder the array
        let array: any[];
        if (type === "social") {
            array = form.social_links.value;
        } else if (type === "menu") {
            array = form.menu_items.value;
        } else if (type === "footer") {
            array = form.footer_lists.value;
        } else {
            array = form.featured_collections.value;
        }

        const draggedItem = array[draggedItemIndex];
        array.splice(draggedItemIndex, 1);
        array.splice(dropIndex, 0, draggedItem);

        // Update the form
        if (type === "social") {
            form.social_links.value = array;
        } else if (type === "menu") {
            form.menu_items.value = array;
        } else if (type === "footer") {
            form.footer_lists.value = array;
        } else {
            form.featured_collections.value = array;
        }
    }
</script>

<!-- Basic Information -->
<Card.Root>
    <Card.Header>
        <Card.Title>Basic Information</Card.Title>
        <Card.Description>
            Essential store details that customers will see.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div>
            <Label for="title">Store Name</Label>
            <Input
                id="title"
                errors={form.name.errors}
                bind:value={form.name.value}
                placeholder="Enter store name"
            />
        </div>
        <div>
            <Label for="description">Description</Label>
            <Textarea
                id="description"
                errors={form.description.errors}
                bind:value={form.description.value}
                placeholder="Enter store description"
                rows={3}
            />
        </div>
        <div>
            <Label for="category">Category</Label>
            <Input
                id="category"
                errors={form.category.errors}
                bind:value={form.category.value}
                placeholder="e.g. Electronics, Fashion, Home & Garden"
            />
        </div>
    </Card.Content>
</Card.Root>

<!-- Contact & Location -->
<Card.Root>
    <Card.Header>
        <Card.Title>Contact & Location</Card.Title>
        <Card.Description>
            How customers can reach or find your store.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <Label for="contact_email">Contact Email</Label>
                <Input
                    id="contact_email"
                    type="email"
                    errors={form.contact_email.errors}
                    bind:value={form.contact_email.value}
                    placeholder="info@store.com"
                />
            </div>
            <div>
                <Label for="contact_phone">Contact Phone</Label>
                <Input
                    id="contact_phone"
                    type="tel"
                    errors={form.contact_phone.errors}
                    bind:value={form.contact_phone.value}
                    placeholder="+1 555 123 4567"
                />
            </div>
        </div>

        <div>
            <Label for="address">Address</Label>
            <Input
                id="address"
                errors={form.address.errors}
                bind:value={form.address.value}
                placeholder="123 Main St, City, State, Country"
            />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <Label for="city">City</Label>
                <Input
                    id="city"
                    errors={form.city.errors}
                    bind:value={form.city.value}
                    placeholder="New York"
                />
            </div>
            <div>
                <Label for="zip_code">Zip Code</Label>
                <Input
                    id="zip_code"
                    errors={form.zip_code.errors}
                    bind:value={form.zip_code.value}
                    placeholder="12345"
                />
            </div>
        </div>
    </Card.Content>
</Card.Root>

<!-- Logo & Branding -->
<Card.Root>
    <Card.Header>
        <Card.Title>Logo & Branding</Card.Title>
        <Card.Description>
            Configure your store's logo and branding elements.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div>
            <Label for="logo">Logo URL</Label>
            <Input
                id="logo"
                errors={form.logo.errors}
                bind:value={form.logo.value}
                placeholder="https://example.com/logo.png"
            />
        </div>
        <div>
            <Label for="logo_alt">Logo Alt Text</Label>
            <Input
                id="logo_alt"
                errors={form.logo_alt.errors}
                bind:value={form.logo_alt.value}
                placeholder="Your Store Logo"
            />
        </div>
        <div>
            <Label for="favicon_url">Favicon URL</Label>
            <Input
                id="favicon_url"
                errors={form.favicon_url.errors}
                bind:value={form.favicon_url.value}
                placeholder="https://example.com/favicon.ico"
            />
        </div>
    </Card.Content>
</Card.Root>

<!-- Menu Items -->
<Card.Root>
    <Card.Header>
        <Card.Title>Menu Items</Card.Title>
        <Card.Description>
            Configure your store's navigation menu items. Drag to reorder.
        </Card.Description>
        <Card.Action>
            <Button variant="outline" size="sm" onclick={addMenuItem}>
                <PlusIcon />
                Add Menu Item
            </Button>
        </Card.Action>
    </Card.Header>
    <Card.Content>
        {#if !form.menu_items.value.length}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No menu items added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add menu items to see them here.
                </span>
            </div>
        {/if}

        <div class="space-y-2">
            {#each form.menu_items.value as item, index (index)}
                <div
                    class="border rounded-lg p-4 space-y-3 min-w-[280px] flex-1 cursor-move transition-all duration-200 hover:shadow-md"
                    role="listitem"
                    draggable="true"
                    ondragstart={(e) => handleDragStart(e, index, "menu")}
                    ondragend={handleDragEnd}
                    ondragover={handleDragOver}
                    ondrop={(e) => handleDrop(e, index, "menu")}
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600"
                        >
                            <GripVerticalIcon class="w-4 h-4" />
                        </div>
                        <div
                            class="flex flex-wrap gap-2 items-center justify-between flex-1 [&>*]:space-y-2"
                        >
                            <div class="flex-1">
                                <Label>Label</Label>
                                <Input
                                    bind:value={item.label}
                                    placeholder="Home"
                                />
                            </div>
                            <div class="flex-1">
                                <Label>URL</Label>
                                <Input bind:value={item.url} placeholder="/" />
                            </div>
                            <Button
                                variant="ghost"
                                size="sm"
                                onclick={() => removeMenuItem(index)}
                            >
                                <TrashIcon />
                            </Button>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
        {#each form.menu_items.errors || [] as error}
            <p class="text-red-500">{error}</p>
        {/each}
    </Card.Content>
</Card.Root>

<!-- Featured Collections -->
<Card.Root>
    <Card.Header>
        <Card.Title>Featured Collections</Card.Title>
        <Card.Description>
            Select collections to feature on your homepage. Drag to reorder.
        </Card.Description>
        <Card.Action>
            <Button variant="outline" size="sm" onclick={addFeaturedCollection}>
                <PlusIcon />
                Add Collection
            </Button>
        </Card.Action>
    </Card.Header>
    <Card.Content>
        {#if !form.featured_collections.value.length}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No featured collections added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add collections to see them here.
                </span>
            </div>
        {/if}

        <div class="space-y-2">
            {#each form.featured_collections.value as collection, index (index)}
                <div
                    class="border rounded-lg p-4 space-y-3 min-w-[280px] flex-1 cursor-move transition-all duration-200 hover:shadow-md"
                    role="listitem"
                    draggable="true"
                    ondragstart={(e) => handleDragStart(e, index, "collection")}
                    ondragend={handleDragEnd}
                    ondragover={handleDragOver}
                    ondrop={(e) => handleDrop(e, index, "collection")}
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600"
                        >
                            <GripVerticalIcon class="w-4 h-4" />
                        </div>
                        <div
                            class="flex flex-wrap gap-2 items-center justify-between flex-1 [&>*]:space-y-2"
                        >
                            <div class="flex-1">
                                <Label>Collection</Label>
                                <Input
                                    bind:value={
                                        form.featured_collections.value[index]
                                            .label
                                    }
                                    placeholder="Summer Collection"
                                />
                            </div>
                            <div class="flex-1">
                                <Label>Image URL</Label>
                                <Input
                                    bind:value={
                                        form.featured_collections.value[index]
                                            .img
                                    }
                                    placeholder="https://example.com/image.jpg"
                                />
                            </div>
                            <Button
                                variant="ghost"
                                size="sm"
                                onclick={() => removeFeaturedCollection(index)}
                            >
                                <TrashIcon />
                            </Button>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
        {#each form.featured_collections.errors || [] as error}
            <p class="text-red-500">{error}</p>
        {/each}
    </Card.Content>
</Card.Root>

<!-- Social Media Links -->
<Card.Root>
    <Card.Header class="flex flex-row items-center justify-between">
        <div>
            <Card.Title>Social Media Links</Card.Title>
            <Card.Description>
                Connect your store with social media platforms. Drag to reorder.
            </Card.Description>
        </div>
        <Button variant="outline" size="sm" onclick={addSocialLink}>
            <PlusIcon class="w-4 h-4 mr-2" />
            Add Platform
        </Button>
    </Card.Header>
    <Card.Content class="space-y-4">
        {#if !form.social_links.value.length}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No social media links added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add links to see them here.
                </span>
            </div>
        {/if}

        <div class="space-y-2">
            {#each form.social_links.value as link, index (index)}
                <div
                    class="border rounded-lg p-4 space-y-3 min-w-[280px] flex-1 cursor-move transition-all duration-200 hover:shadow-md"
                    role="listitem"
                    draggable="true"
                    ondragstart={(e) => handleDragStart(e, index, "social")}
                    ondragend={handleDragEnd}
                    ondragover={handleDragOver}
                    ondrop={(e) => handleDrop(e, index, "social")}
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600"
                        >
                            <GripVerticalIcon class="w-4 h-4" />
                        </div>
                        <div
                            class="flex flex-wrap gap-2 items-center justify-between flex-1 [&>*]:space-y-2"
                        >
                            <div class="flex-1">
                                <Label>Platform</Label>
                                <Input
                                    bind:value={link.platform}
                                    placeholder="e.g. TikTok"
                                />
                            </div>
                            <div class="flex-1">
                                <Label>URL</Label>
                                <Input
                                    bind:value={link.url}
                                    placeholder="e.g. https://www.tiktok.com/@username"
                                />
                            </div>
                            <Button
                                variant="ghost"
                                size="sm"
                                onclick={() => removeSocialLink(index)}
                            >
                                <TrashIcon />
                            </Button>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
        {#each form.social_links.errors || [] as error}
            <p class="text-red-500">{error}</p>
        {/each}
    </Card.Content>
</Card.Root>

<!-- Footer Lists -->
<Card.Root>
    <Card.Header class="flex flex-row items-center justify-between">
        <div>
            <Card.Title>Footer Lists</Card.Title>
            <Card.Description>
                Create footer navigation lists with multiple items. Drag to
                reorder lists and items.
            </Card.Description>
        </div>
        <Button variant="outline" size="sm" onclick={addFooterList}>
            <PlusIcon class="w-4 h-4 mr-2" />
            Add List
        </Button>
    </Card.Header>

    <Card.Content class="space-y-4">
        {#if !form.footer_lists.value.length}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No footer lists added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add lists to see them here.
                </span>
            </div>
        {/if}

        <div class="space-y-4">
            {#each form.footer_lists.value as list, listIndex (listIndex)}
                <div
                    class="border rounded-lg p-4 space-y-4 cursor-move hover:shadow-md transition-all"
                    role="listitem"
                    draggable="true"
                    ondragstart={(e) => handleDragStart(e, listIndex, "footer")}
                    ondragend={handleDragEnd}
                    ondragover={handleDragOver}
                    ondrop={(e) => handleDrop(e, listIndex, "footer")}
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600"
                        >
                            <GripVerticalIcon class="w-4 h-4" />
                        </div>
                        <!-- List Header -->
                        <div class="flex flex-col gap-4 flex-1 [&>*]:space-y-2">
                            <div
                                class="flex flex-wrap gap-2 items-center justify-between flex-1 [&>*]:space-y-2"
                            >
                                <div class="flex-1">
                                    <Label>List Title</Label>
                                    <Input
                                        bind:value={list.title}
                                        placeholder="e.g. Company"
                                    />
                                </div>
                                <Button
                                    variant="ghost"
                                    size="sm"
                                    onclick={() => removeFooterList(listIndex)}
                                >
                                    <TrashIcon />
                                </Button>
                            </div>
                            <!-- Footer List Items -->
                            <div class="space-y-3">
                                {#each list.items as item, itemIndex (itemIndex)}
                                    <div
                                        class="border rounded-lg p-4 space-y-3 flex flex-wrap gap-2 items-center justify-between flex-1 [&>*]:space-y-2 cursor-move transition-all duration-200 hover:shadow-sm"
                                        role="listitem"
                                        draggable="true"
                                        ondragstart={(e) =>
                                            handleSubItemDragStart(
                                                e,
                                                listIndex,
                                                itemIndex,
                                            )}
                                        ondragend={handleSubItemDragEnd}
                                        ondragover={handleDragOver}
                                        ondrop={(e) =>
                                            handleSubItemDrop(
                                                e,
                                                listIndex,
                                                itemIndex,
                                            )}
                                    >
                                        <div
                                            class="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600 self-start mt-6"
                                        >
                                            <GripVerticalIcon class="w-4 h-4" />
                                        </div>
                                        <div class="flex-1">
                                            <Label>Label</Label>
                                            <Input
                                                bind:value={item.label}
                                                placeholder="e.g. About Us"
                                            />
                                        </div>
                                        <div class="flex-1">
                                            <Label>URL</Label>
                                            <Input
                                                bind:value={item.url}
                                                placeholder="https://example.com/about"
                                            />
                                        </div>
                                        <Button
                                            variant="ghost"
                                            size="sm"
                                            onclick={() =>
                                                removeFooterItem(
                                                    listIndex,
                                                    itemIndex,
                                                )}
                                        >
                                            <TrashIcon />
                                        </Button>
                                    </div>
                                {/each}

                                <Button
                                    variant="outline"
                                    size="sm"
                                    onclick={() => addFooterItem(listIndex)}
                                >
                                    <PlusIcon class="w-4 h-4 mr-2" />
                                    Add Item
                                </Button>
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>

        {#each form.footer_lists.errors || [] as error}
            <p class="text-red-500">{error}</p>
        {/each}
    </Card.Content>
</Card.Root>
