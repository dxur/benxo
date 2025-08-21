<script lang="ts">
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Textarea } from "$lib/components/ui/textarea/index";
    import * as Card from "$lib/components/ui/card/index";
    import * as Select from "$lib/components/ui/select/index";
    import { Button } from "$lib/components/ui/button/index";
    import { Badge } from "@/lib/components/ui/badge";
    import { Switch } from "$lib/components/ui/switch/index";
    import { PlusIcon, TrashIcon, ImageIcon } from "@lucide/svelte";
    import type { Form } from "../../lib/utils/form";
    import type { ProductSchema } from "./service";
    import { snakeToTitleCase } from "../../lib/utils/fmt";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof ProductSchema>;
    } = $props();
</script>

<!-- Basic Information -->
<Card.Root>
    <Card.Header>
        <Card.Title>Basic Information</Card.Title>
        <Card.Description>
            Essential product details that customers will see.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div>
            <Label for="title">Product Name</Label>
            <Input
                id="title"
                errors={form.title.errors}
                bind:value={form.title.value}
                placeholder="Enter product name"
            />
        </div>
        <div>
            <Label for="description">Description</Label>
            <Textarea
                id="description"
                errors={form.description.errors}
                bind:value={form.description.value}
                placeholder="Enter product description"
                rows={4}
            />
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <Label for="category">Category</Label>
                <Input
                    id="category"
                    errors={form.category.errors}
                    bind:value={form.category.value}
                    placeholder="e.g. Electronics, Clothing, Books"
                />
            </div>
            <div>
                <Label for="slug">URL Slug</Label>
                <Input
                    id="slug"
                    errors={form.slug.errors}
                    bind:value={form.slug.value}
                    placeholder="product-url-slug"
                />
            </div>
        </div>
        <div class="flex items-center space-x-2">
            <Switch id="featured" bind:checked={form.featured.value} />
            <Label for="featured">Featured Product</Label>
        </div>
    </Card.Content>
</Card.Root>
