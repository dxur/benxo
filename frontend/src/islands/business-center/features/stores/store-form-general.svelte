<script lang="ts">
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Textarea } from "$lib/components/ui/textarea/index";
    import * as Card from "$lib/components/ui/card/index";
    import { Button } from "$lib/components/ui/button/index";
    import type { Form } from "../../lib/utils/form";
    import type { StoreSchema } from "./service";
    import { TrashIcon, PlusIcon } from "@lucide/svelte";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof StoreSchema>;
    } = $props();

    function addSocialLink() {
        form.social_links.value.push({
            platform: "",
            url: "",
        });
    }

    function removeSocialLink(index: number) {
        form.social_links.value.splice(index, 1);
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

<!-- Social Media Links -->
<Card.Root>
    <Card.Header class="flex flex-row items-center justify-between">
        <div>
            <Card.Title>Social Media Links</Card.Title>
            <Card.Description>
                Connect your store with social media platforms.
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

        {#each form.social_links.value as link, index (index)}
            <div class="border rounded-lg p-4 space-y-3 min-w-[280px] flex-1">
                <div
                    class="flex flex-wrap gap-2 items-center justify-between [&>*]:space-y-2"
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
        {/each}
        {#each form.social_links.errors || [] as error}
            <p class="text-red-500">{error}</p>
        {/each}
    </Card.Content>
</Card.Root>
