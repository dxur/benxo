<script lang="ts">
    import { Button } from "@/lib/components/ui/button/index.js";
    import { Input } from "@/lib/components/ui/input/index.js";
    import { Label } from "@/lib/components/ui/label/index.js";
    import { Textarea } from "@/lib/components/ui/textarea/index.js";
    import { Switch } from "@/lib/components/ui/switch/index.js";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import * as Card from "@/lib/components/ui/card/index.js";
    import * as Select from "@/lib/components/ui/select/index.js";
    import * as Tabs from "@/lib/components/ui/tabs/index.js";
    import { Separator } from "@/lib/components/ui/separator/index.js";

    import type { ProductPublic } from "@bindings/ProductPublic";
    import type { ProductUpdateBody } from "@bindings/ProductUpdateBody";
    import type { ProductVariant } from "@bindings/ProductVariant";

    import ImageIcon from "@lucide/svelte/icons/image";
    import PlusIcon from "@lucide/svelte/icons/plus";
    import TrashIcon from "@lucide/svelte/icons/trash";
    import GripVerticalIcon from "@lucide/svelte/icons/grip-vertical";

    type ProductFormProps = {
        data: ProductUpdateBody;
        onchange: () => void;
        product: ProductPublic;
    };

    let { data = $bindable(), onchange, product }: ProductFormProps = $props();

    // Categories - you might want to fetch these from your API
    const categories = [
        "Electronics",
        "Clothing",
        "Books",
        "Home & Garden",
        "Sports",
        "Beauty",
        "Toys",
        "Food & Beverages",
    ];

    function handleInputChange() {
        onchange();
    }

    function addVariant() {
        if (!data.variants) data.variants = [];

        const newVariant: ProductVariant = {
            sku: `${data.slug || product.slug}-${Date.now()}`,
            price: data.base_price,
            compare_price: data.base_compare_price,
            stocks: 0,
            images: [],
            options: {},
        };

        data.variants = [...data.variants, newVariant];
        handleInputChange();
    }

    function removeVariant(index: number) {
        if (!data.variants) return;
        data.variants = data.variants.filter((_, i) => i !== index);
        handleInputChange();
    }

    function addImage() {
        if (!data.base_images) data.base_images = [];
        // In a real app, you'd open a file picker or image upload dialog
        const imageUrl = prompt("Enter image URL:");
        if (imageUrl) {
            data.base_images = [...data.base_images, imageUrl];
            handleInputChange();
        }
    }

    function removeImage(index: number) {
        if (!data.base_images) return;
        data.base_images = data.base_images.filter((_, i) => i !== index);
        handleInputChange();
    }

    function generateSlug(name: string): string {
        return name
            .toLowerCase()
            .replace(/[^a-z0-9]+/g, "-")
            .replace(/^-+|-+$/g, "");
    }

    // Auto-generate slug when name changes
    // $: if (data.name && data.name !== product.name) {
    //     data.slug = generateSlug(data.name);
    // }
</script>

<div class="space-y-6">
    <Tabs.Root value="general" class="w-full">
        <Tabs.List class="w-full">
            <Tabs.Trigger value="general">General</Tabs.Trigger>
            <Tabs.Trigger value="pricing">Pricing</Tabs.Trigger>
            <Tabs.Trigger value="images">Images</Tabs.Trigger>
            <Tabs.Trigger value="options">Options</Tabs.Trigger>
            <Tabs.Trigger value="variants">Variants</Tabs.Trigger>
        </Tabs.List>

        <!-- General Tab -->
        <Tabs.Content value="general" class="space-y-6">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Basic Information</Card.Title>
                    <Card.Description>
                        Essential product details that customers will see.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="space-y-2">
                            <Label for="name">Product Name</Label>
                            <Input
                                id="name"
                                bind:value={data.name}
                                oninput={handleInputChange}
                                placeholder="Enter product name"
                            />
                        </div>
                        <div class="space-y-2">
                            <Label for="category">Category</Label>
                            <Input
                                id="name"
                                bind:value={data.category}
                                oninput={handleInputChange}
                                placeholder="Enter product name"
                            />
                        </div>
                    </div>

                    <div class="space-y-2">
                        <Label for="description">Description</Label>
                        <Textarea
                            id="description"
                            bind:value={data.description}
                            oninput={handleInputChange}
                            placeholder="Describe your product..."
                            rows={4}
                        />
                    </div>

                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="space-y-2">
                            <Label for="slug">URL Slug</Label>
                            <Input
                                id="slug"
                                bind:value={data.slug}
                                oninput={handleInputChange}
                                placeholder="product-url-slug"
                            />
                            <p class="text-sm text-muted-foreground">
                                URL: /products/{data.slug || product.slug}
                            </p>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Switch
                                id="featured"
                                bind:checked={data.featured}
                                onCheckedChange={handleInputChange}
                            />
                            <Label for="featured">Featured Product</Label>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        </Tabs.Content>

        <!-- Pricing Tab -->
        <Tabs.Content value="pricing" class="space-y-6">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Pricing</Card.Title>
                    <Card.Description>
                        Set your product pricing and compare prices.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="space-y-2">
                            <Label for="base_price">Base Price (DZD)</Label>
                            <Input
                                id="base_price"
                                type="number"
                                step="0.01"
                                bind:value={data.base_price}
                                oninput={handleInputChange}
                                placeholder="0.00"
                            />
                        </div>
                        <div class="space-y-2">
                            <Label for="base_compare_price"
                                >Compare Price (DZD)</Label
                            >
                            <Input
                                id="base_compare_price"
                                type="number"
                                step="0.01"
                                bind:value={data.base_compare_price}
                                oninput={handleInputChange}
                                placeholder="0.00"
                            />
                            <p class="text-sm text-muted-foreground">
                                Higher price to show savings
                            </p>
                        </div>
                    </div>

                    {#if data.base_compare_price && data.base_price && data.base_compare_price > data.base_price}
                        <div class="rounded-lg bg-green-50 p-4">
                            <div class="flex items-center gap-2">
                                <Badge
                                    variant="outline"
                                    class="bg-green-100 text-green-800"
                                >
                                    {Math.round(
                                        (1 -
                                            data.base_price /
                                                data.base_compare_price) *
                                            100,
                                    )}% OFF
                                </Badge>
                                <span class="text-sm text-green-800">
                                    Customers save {new Intl.NumberFormat(
                                        "en-US",
                                        {
                                            style: "currency",
                                            currency: "DZD",
                                        },
                                    ).format(
                                        data.base_compare_price -
                                            data.base_price,
                                    )}
                                </span>
                            </div>
                        </div>
                    {/if}
                </Card.Content>
            </Card.Root>
        </Tabs.Content>

        <!-- Images Tab -->
        <Tabs.Content value="images" class="space-y-6">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Product Images</Card.Title>
                    <Card.Description>
                        Add high-quality images to showcase your product.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <div class="grid gap-4 md:grid-cols-3">
                        {#each data.base_images || [] as image, index}
                            <div class="relative group">
                                <div
                                    class="aspect-square rounded-lg bg-muted overflow-hidden"
                                >
                                    <img
                                        src={image}
                                        alt="Product"
                                        class="w-full h-full object-cover"
                                    />
                                </div>
                                <Button
                                    variant="destructive"
                                    size="icon"
                                    class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity"
                                    onclick={() => removeImage(index)}
                                >
                                    <TrashIcon class="h-4 w-4" />
                                </Button>
                            </div>
                        {:else}
                            <div
                                class="aspect-square rounded-lg bg-muted flex items-center justify-center text-muted-foreground"
                            >
                                <ImageIcon class="h-8 w-8" />
                            </div>
                        {/each}
                        <Button variant="outline" onclick={addImage}>
                            <PlusIcon class="h-6 w-6" />
                            <span>Add Image</span>
                        </Button>
                    </div>
                </Card.Content>
            </Card.Root>
        </Tabs.Content>

        <!-- Variants Tab -->
        <Tabs.Content value="variants" class="space-y-6">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Product Variants</Card.Title>
                    <Card.Description>
                        Manage different versions of your product (e.g., sizes,
                        colors).
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    {#if data.variants && data.variants.length > 0}
                        <div class="space-y-4">
                            {#each data.variants as variant, index}
                                <div
                                    class="border rounded-lg p-4 flex items-center gap-4"
                                >
                                    <GripVerticalIcon
                                        class="h-5 w-5 text-muted-foreground cursor-grab"
                                    />
                                    <div
                                        class="flex-1 grid grid-cols-2 lg:grid-cols-4 gap-4"
                                    >
                                        <div class="space-y-2">
                                            <Label for="sku-{index}">SKU</Label>
                                            <Input
                                                id="sku-{index}"
                                                bind:value={variant.sku}
                                                oninput={handleInputChange}
                                                placeholder="Variant SKU"
                                            />
                                        </div>
                                        <div class="space-y-2">
                                            <Label for="variant-price-{index}"
                                                >Price (DZD)</Label
                                            >
                                            <Input
                                                id="variant-price-{index}"
                                                type="number"
                                                step="0.01"
                                                bind:value={variant.price}
                                                oninput={handleInputChange}
                                                placeholder="0.00"
                                            />
                                        </div>
                                        <div class="space-y-2">
                                            <Label for="variant-stocks-{index}"
                                                >Stocks</Label
                                            >
                                            <Input
                                                id="variant-stocks-{index}"
                                                type="number"
                                                bind:value={variant.stocks}
                                                oninput={handleInputChange}
                                                placeholder="0"
                                            />
                                        </div>
                                        <!-- Add more variant options here if needed -->
                                    </div>
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        class="text-destructive hover:text-destructive"
                                        onclick={() => removeVariant(index)}
                                    >
                                        <TrashIcon class="h-5 w-5" />
                                    </Button>
                                </div>
                            {/each}
                        </div>
                    {:else}
                        <p class="text-muted-foreground text-center py-4">
                            No variants added yet.
                        </p>
                    {/if}
                    <Button
                        variant="outline"
                        class="w-full"
                        onclick={addVariant}
                    >
                        <PlusIcon class="h-4 w-4 mr-2" />
                        Add Variant
                    </Button>
                </Card.Content>
            </Card.Root>
        </Tabs.Content>
    </Tabs.Root>
</div>
