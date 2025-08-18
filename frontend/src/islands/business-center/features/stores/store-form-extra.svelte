<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import { Textarea } from "$lib/components/ui/textarea/index";
    import * as Select from "$lib/components/ui/select/index";
    import type { Form } from "../../lib/utils/form";
    import type { StoreSchema } from "./service";
    import { PlusIcon, TrashIcon } from "@lucide/svelte";
    import { snakeToTitleCase } from "../../lib/utils/fmt";
    import { untrack } from "svelte";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof StoreSchema>;
    } = $props();

    const conversionEvents = [
        { name: "Purchase", value: "purchase" },
        { name: "Add to Cart", value: "add_to_cart" },
        { name: "Begin Checkout", value: "begin_checkout" },
        { name: "View Content", value: "view_content" },
    ];

    const platforms = new Map([
        ["facebook", "Facebook"],
        ["google", "Google Ads"],
        ["tiktok", "TikTok"],
        ["snapchat", "Snapchat"],
        ["twitter", "Twitter"],
        ["pinterest", "Pinterest"],
    ]);

    function addPixel() {
        form.tracking_pixels.value.push({
            platform: "",
            pixel_id: "",
            events: [],
        });
    }

    function removePixel(index: number) {
        form.tracking_pixels.value.splice(index, 1);
    }

    let customKeyValues = $state<[string, string][]>(
        Object.entries(form.custom_key_values.value || {}) || [],
    );

    $effect(() => {
        let derived = Object.fromEntries(customKeyValues);
        untrack(() => {
            form.custom_key_values.value = derived;
        });
    });

    function addCustomPair() {
        customKeyValues.push(["", ""]);
    }

    function removeCustomPair(index: number) {
        customKeyValues.splice(index, 1);
    }
</script>

<!-- Analytics -->
<Card.Root>
    <Card.Header>
        <Card.Title>Analytics</Card.Title>
        <Card.Description>
            Set up analytics tracking for your store performance.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div>
            <Label for="google_analytics">Google Analytics 4 ID</Label>
            <Input
                id="google_analytics"
                errors={form.google_analytics_id.errors}
                bind:value={form.google_analytics_id.value}
                placeholder="G-XXXXXXXXXX"
            />
        </div>
        <div>
            <Label for="gtm_container">Google Tag Manager</Label>
            <Input
                id="gtm_container"
                errors={form.gtm_container_id.errors}
                bind:value={form.gtm_container_id.value}
                placeholder="GTM-XXXXXXX"
            />
        </div>
    </Card.Content>
</Card.Root>

<!-- Tracking Pixels -->
<Card.Root>
    <Card.Header>
        <Card.Title>Tracking Pixels</Card.Title>
        <Card.Description>
            Configure tracking pixels for advertising platforms.
        </Card.Description>
        <Card.Action>
            <Button variant="outline" size="sm" onclick={addPixel}>
                <PlusIcon />
                Add Pixel
            </Button>
        </Card.Action>
    </Card.Header>
    <Card.Content>
        {#if !form.tracking_pixels.value.length}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No pixels added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add pixels to see them here.
                </span>
            </div>
        {/if}

        {#each form.tracking_pixels.value as pixel, index (index)}
            <div class="border rounded-lg p-4 space-y-3 min-w-[280px] flex-1">
                <div
                    class="flex flex-wrap gap-2 items-center justify-between [&>*]:space-y-2"
                >
                    <div>
                        <Label>Platform</Label>
                        <Select.Root type="single" bind:value={pixel.platform}>
                            <Select.Trigger>
                                {pixel.platform
                                    ? platforms.get(pixel.platform)
                                    : "Select a Platform"}
                            </Select.Trigger>
                            <Select.Content>
                                {#each platforms as [key, title]}
                                    <Select.Item value={key}
                                        >{title}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>
                    </div>

                    <div>
                        <Label class="whitespace-nowrap">Pixel/Tag ID</Label>
                        <Input
                            bind:value={pixel.pixel_id}
                            placeholder="Enter pixel ID"
                        />
                    </div>

                    <div>
                        <Label>Events</Label>
                        <Select.Root type="multiple" bind:value={pixel.events}>
                            <Select.Trigger>
                                <span class="max-w-32 truncate text-start">
                                    {pixel.events.length
                                        ? pixel.events
                                              .map((v) => snakeToTitleCase(v))
                                              .join(", ")
                                        : "Select Events"}
                                </span>
                            </Select.Trigger>
                            <Select.Content>
                                {#each conversionEvents as event}
                                    <Select.Item value={event.value}
                                        >{event.name}</Select.Item
                                    >
                                {/each}
                            </Select.Content>
                        </Select.Root>
                    </div>

                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => removePixel(index)}
                    >
                        <TrashIcon />
                    </Button>
                </div>
            </div>
        {/each}
        {#each form.tracking_pixels.errors || [] as error}
            <div class="text-red-500 text-sm">{error}</div>
        {/each}
    </Card.Content>
</Card.Root>

<!-- <Card.Root>
    <Card.Header>
        <Card.Title>E-commerce Conversion Events</Card.Title>
        <Card.Description>
            Configure which events to track for conversions.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-3">
        {#each conversionEvents as event, index (index)}
            <div
                class="flex items-center justify-between p-3 border rounded-lg"
            >
                <div class="flex items-center space-x-3">
                    <input
                        type="checkbox"
                        bind:checked={event.enabled}
                        class="rounded"
                    />
                    <div>
                        <div class="font-medium text-sm">{event.name}</div>
                        <div class="text-xs text-muted-foreground">
                            {event.value}
                        </div>
                    </div>
                </div>
                {#if index >= 4}
                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => conversionEvents.splice(index, 1)}
                    >
                        <TrashIcon />
                    </Button>
                {/if}
            </div>
        {/each}
    </Card.Content>
</Card.Root> -->

<!-- SEO Settings -->
<Card.Root>
    <Card.Header>
        <Card.Title>SEO & Meta Tags</Card.Title>
        <Card.Description>
            Optimize your store for search engines.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div>
            <Label for="meta_title">Meta Title</Label>
            <Input
                id="meta_title"
                errors={form.meta_title.errors}
                bind:value={form.meta_title.value}
                placeholder="Your Store - Quality Products Online"
            />
        </div>
        <div>
            <Label for="meta_description">Meta Description</Label>
            <Textarea
                id="meta_description"
                errors={form.meta_description.errors}
                bind:value={form.meta_description.value}
                placeholder="Discover amazing products at unbeatable prices..."
                rows={2}
            />
        </div>
        <div>
            <Label for="meta_keywords">Meta Keywords</Label>
            <Input
                id="meta_keywords"
                errors={form.meta_keywords.errors}
                bind:value={form.meta_keywords.value}
                placeholder="store, products, shopping, online"
            />
        </div>
    </Card.Content>
</Card.Root>

<Card.Root>
    <Card.Header>
        <Card.Title>Custom Key-Value Settings</Card.Title>
        <Card.Description>
            Define custom key pairs you can reference inside your templates or
            configurations.
        </Card.Description>
        <Card.Action>
            <Button variant="outline" size="sm" onclick={addCustomPair}>
                <PlusIcon />
                Add Key Pair
            </Button>
        </Card.Action>
    </Card.Header>
    <Card.Content class="space-y-4">
        {#if !Object.keys(form.custom_key_values.value || []).length}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No custom key pairs added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add key pairs to see them here.
                </span>
            </div>
        {/if}

        {#each customKeyValues as element, index}
            <div class="border rounded-lg p-4 space-y-3 min-w-[280px] flex-1">
                <div
                    class="flex flex-wrap gap-2 items-center justify-between [&>*]:space-y-2"
                >
                    <div class="flex-1">
                        <Label>Key</Label>
                        <Input
                            bind:value={element[0]}
                            placeholder="e.g. SUPPORT_EMAIL"
                        />
                    </div>
                    <div class="flex-1">
                        <Label>Value</Label>
                        <Input
                            bind:value={element[1]}
                            placeholder="e.g. support@example.com"
                        />
                    </div>
                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => removeCustomPair(index)}
                    >
                        <TrashIcon />
                    </Button>
                </div>
            </div>
        {/each}
    </Card.Content>
</Card.Root>
