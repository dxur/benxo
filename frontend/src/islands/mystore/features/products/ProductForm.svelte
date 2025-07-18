<script lang="ts">
    import { Button } from "@/lib/components/ui/button/index.js";
    import { Input } from "@/lib/components/ui/input/index.js";
    import { Label } from "@/lib/components/ui/label/index.js";
    import { Textarea } from "@/lib/components/ui/textarea/index.js";
    import { Switch } from "@/lib/components/ui/switch/index.js";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import { Checkbox } from "@/lib/components/ui/checkbox/index.js";
    import * as Card from "@/lib/components/ui/card/index.js";
    import * as Tabs from "@/lib/components/ui/tabs/index.js";

    import type { ProductPublic } from "@bindings/ProductPublic";
    import type { ProductUpdateBody } from "@bindings/ProductUpdateBody";
    import type { ProductVariant } from "@bindings/ProductVariant";

    import ImageIcon from "@lucide/svelte/icons/image";
    import PlusIcon from "@lucide/svelte/icons/plus";
    import TrashIcon from "@lucide/svelte/icons/trash";
    import StarIcon from "@lucide/svelte/icons/star";
    import MoveUpIcon from "@lucide/svelte/icons/move-up";
    import CheckIcon from "@lucide/svelte/icons/check";
    import XIcon from "@lucide/svelte/icons/x";
    import SettingsIcon from "@lucide/svelte/icons/settings";
    import { onMount } from "svelte";

    type ProductFormProps = {
        data: ProductUpdateBody;
        onchange: () => void;
        product: ProductPublic;
        currentTab: "general" | "pricing" | "images" | "variants";
    };

    type VariantCombination = {
        id: string;
        options: Record<string, string>;
        enabled: boolean;
        variant?: ProductVariant;
        isDefault?: boolean;
    };

    let {
        data = $bindable(),
        currentTab = $bindable("general"),
        onchange,
        product,
    }: ProductFormProps = $props();

    // Variant management state
    let variantCombinations: VariantCombination[] = $state([]);
    let editingVariant: string | null = $state(null);
    let showOptionsDialog = $state(false);

    // Initialize options if not present
    if (!data.options) {
        data.options = {};
    }

    function handleInputChange() {
        onchange();
    }

    // Image management functions
    function addImage() {
        if (!data.base_images) data.base_images = [];
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

    function setAsDefaultImage(index: number) {
        if (!data.base_images || index === 0) return;

        const imageToMove = data.base_images[index];
        const newImages = [...data.base_images];
        newImages.splice(index, 1);
        newImages.unshift(imageToMove);

        data.base_images = newImages;
        handleInputChange();
    }

    // Variant option management
    function addVariantOption() {
        const name = prompt("Enter option name (e.g., Color, Size):");
        if (name && name.trim()) {
            const cleanName = name.trim();
            if (!data.options![cleanName]) {
                data.options![cleanName] = [];
                generateVariantCombinations();
                handleInputChange();
            }
        }
    }

    function removeVariantOption(optionName: string) {
        if (data.options && data.options[optionName]) {
            delete data.options[optionName];
            data = data;
            generateVariantCombinations();
            handleInputChange();
        }
    }

    function addOptionValue(optionName: string) {
        const value = prompt("Enter option value:");
        if (value && value.trim() && data.options && data.options[optionName]) {
            const cleanValue = value.trim();
            if (!data.options[optionName]!.includes(cleanValue)) {
                data.options[optionName] = [
                    ...data.options[optionName]!,
                    cleanValue,
                ];
                generateVariantCombinations();
                handleInputChange();
            }
        }
    }

    function removeOptionValue(optionName: string, valueIndex: number) {
        if (data.options && data.options[optionName]) {
            data.options[optionName] = data.options[optionName]!.filter(
                (_, i) => i !== valueIndex,
            );
            generateVariantCombinations();
            handleInputChange();
        }
    }

    // Generate all possible combinations
    function generateVariantCombinations() {
        if (!data.options || Object.keys(data.options).length === 0) {
            variantCombinations = [];
            return;
        }

        const optionEntries = Object.entries(data.options).filter(
            ([_, values]) => values && values.length > 0,
        );
        if (optionEntries.length === 0) {
            variantCombinations = [];
            return;
        }

        const combinations: VariantCombination[] = [];

        function generateCombos(
            entries: [string, string[]][],
            currentCombo: Record<string, string>,
            index: number,
        ) {
            if (index === entries.length) {
                const id = Object.entries(currentCombo)
                    .map(([key, value]) => `${key}:${value}`)
                    .join("|");

                // Check if this combination already exists
                const existing = variantCombinations.find((c) => c.id === id);
                const existingVariant = data.variants?.find(
                    (v) =>
                        v.options &&
                        Object.entries(currentCombo).every(
                            ([key, value]) => v.options![key] === value,
                        ),
                );

                combinations.push({
                    id,
                    options: { ...currentCombo },
                    enabled: existing ? existing.enabled : !!existingVariant,
                    variant: existingVariant || existing?.variant,
                    isDefault: existingVariant
                        ? data.variants?.indexOf(existingVariant) === 0
                        : false,
                });
                return;
            }

            const [optionName, values] = entries[index];
            for (const value of values) {
                generateCombos(
                    entries,
                    { ...currentCombo, [optionName]: value },
                    index + 1,
                );
            }
        }

        generateCombos(optionEntries, {}, 0);
        variantCombinations = combinations;
    }

    function toggleVariantCombination(combinationId: string) {
        const combination = variantCombinations.find(
            (c) => c.id === combinationId,
        );
        if (!combination) return;

        combination.enabled = !combination.enabled;

        if (combination.enabled && !combination.variant) {
            // Create new variant
            const optionString = Object.entries(combination.options)
                .map(
                    ([key, value]) =>
                        `${key.toLowerCase()}-${value.toLowerCase()}`,
                )
                .join("-");

            combination.variant = {
                sku: `${data.slug || product.slug}-${optionString}`,
                price: data.base_price || 0,
                compare_price: data.base_compare_price || null,
                stocks: 0,
                images: [],
                options: combination.options,
            };
        } else if (!combination.enabled) {
            // Remove variant
            combination.variant = undefined;
            combination.isDefault = false;
        }

        updateVariantsArray();
    }

    function setDefaultVariant(combinationId: string) {
        // Remove default from all combinations
        variantCombinations.forEach((c) => (c.isDefault = false));

        // Set new default
        const combination = variantCombinations.find(
            (c) => c.id === combinationId,
        );
        if (combination) {
            combination.isDefault = true;
        }

        updateVariantsArray();
    }

    function updateVariantsArray() {
        const enabledVariants = variantCombinations
            .filter((c) => c.enabled && c.variant)
            .map((c) => c.variant!);

        // Sort so default variant is first
        const defaultCombination = variantCombinations.find((c) => c.isDefault);
        if (defaultCombination && defaultCombination.variant) {
            const defaultVariant = defaultCombination.variant;
            const otherVariants = enabledVariants.filter(
                (v) => v !== defaultVariant,
            );
            data.variants = [defaultVariant, ...otherVariants];
        } else {
            data.variants = enabledVariants;
        }

        handleInputChange();
    }

    function getVariantDisplayName(combination: VariantCombination): string {
        return Object.entries(combination.options)
            .map(([key, value]) => `${key}: ${value}`)
            .join(", ");
    }

    function startEditingVariant(combinationId: string) {
        editingVariant = combinationId;
    }

    function stopEditingVariant() {
        editingVariant = null;
        updateVariantsArray();
    }

    function getVariantPrice(combination: VariantCombination): string {
        if (!combination.variant) return "0.00";
        return combination.variant.price?.toFixed(2) || "0.00";
    }

    function getVariantComparePrice(combination: VariantCombination): string {
        if (!combination.variant?.compare_price) return "";
        return combination.variant.compare_price.toFixed(2);
    }

    function getVariantStock(combination: VariantCombination): string {
        if (!combination.variant) return "0";
        return combination.variant.stocks?.toString() || "0";
    }

    function updateVariantField(
        combinationId: string,
        field: string,
        value: any,
    ) {
        const combination = variantCombinations.find(
            (c) => c.id === combinationId,
        );
        if (!combination?.variant) return;

        if (field === "price" || field === "compare_price") {
            combination.variant[field] = value
                ? parseFloat(value)
                : field === "price"
                  ? 0
                  : null;
        } else if (field === "stocks") {
            combination.variant[field] = value ? parseInt(value) : 0;
        } else {
            combination.variant[field] = value;
        }
    }

    onMount(() => {
        generateVariantCombinations();
    });
</script>

<div class="space-y-6">
    <Tabs.Root bind:value={currentTab} class="w-full">
        <Tabs.List class="w-full">
            <Tabs.Trigger value="general">General</Tabs.Trigger>
            <Tabs.Trigger value="pricing">Pricing</Tabs.Trigger>
            <Tabs.Trigger value="images">Images</Tabs.Trigger>
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
                            <Label for="name">Product Name *</Label>
                            <Input
                                id="name"
                                bind:value={data.name}
                                oninput={handleInputChange}
                                placeholder="Enter product name"
                                required
                            />
                        </div>
                        <div class="space-y-2">
                            <Label for="category">Category</Label>
                            <Input
                                id="category"
                                bind:value={data.category}
                                oninput={handleInputChange}
                                placeholder="Enter product category"
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
                        <div class="flex items-center space-x-2 pt-8">
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
                            <Label for="base_price">Base Price (DZD) *</Label>
                            <Input
                                id="base_price"
                                type="number"
                                step="0.01"
                                min="0"
                                bind:value={data.base_price}
                                oninput={handleInputChange}
                                placeholder="0.00"
                                required
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
                                min="0"
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
                        <div
                            class="rounded-lg bg-green-50 border border-green-200 p-4"
                        >
                            <div class="flex items-center gap-2">
                                <Badge
                                    variant="outline"
                                    class="bg-green-100 text-green-800 border-green-300"
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
                        Add high-quality images to showcase your product. The
                        first image will be used as the thumbnail.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                    <div
                        class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
                    >
                        {#each data.base_images || [] as image, index}
                            <div class="relative group">
                                <div
                                    class="aspect-square rounded-lg bg-muted overflow-hidden border-2 transition-colors {index ===
                                    0
                                        ? 'border-primary shadow-md'
                                        : 'border-border hover:border-primary/50'}"
                                >
                                    <img
                                        src={image}
                                        alt="Product image {index + 1}"
                                        class="w-full h-full object-cover"
                                    />
                                </div>

                                {#if index === 0}
                                    <div class="absolute top-2 left-2">
                                        <Badge
                                            variant="default"
                                            class="bg-primary text-primary-foreground"
                                        >
                                            <StarIcon class="h-3 w-3 mr-1" />
                                            Thumbnail
                                        </Badge>
                                    </div>
                                {/if}

                                <div
                                    class="absolute top-2 right-2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
                                >
                                    {#if index !== 0}
                                        <Button
                                            variant="secondary"
                                            size="icon"
                                            class="h-8 w-8 shadow-md"
                                            onclick={() =>
                                                setAsDefaultImage(index)}
                                            title="Set as thumbnail"
                                        >
                                            <MoveUpIcon class="h-4 w-4" />
                                        </Button>
                                    {/if}
                                    <Button
                                        variant="destructive"
                                        size="icon"
                                        class="h-8 w-8 shadow-md"
                                        onclick={() => removeImage(index)}
                                        title="Remove image"
                                    >
                                        <TrashIcon class="h-4 w-4" />
                                    </Button>
                                </div>
                            </div>
                        {:else}
                            <div
                                class="aspect-square rounded-lg bg-muted flex items-center justify-center text-muted-foreground border-2 border-dashed border-border"
                            >
                                <div class="text-center">
                                    <ImageIcon class="h-8 w-8 mx-auto mb-2" />
                                    <p class="text-sm">No images yet</p>
                                </div>
                            </div>
                        {/each}

                        <button
                            onclick={addImage}
                            class="aspect-square rounded-lg border-2 border-dashed border-border hover:border-primary hover:bg-accent/50 transition-colors flex items-center justify-center text-muted-foreground hover:text-primary"
                        >
                            <div class="text-center">
                                <PlusIcon class="h-6 w-6 mx-auto mb-2" />
                                <span class="text-sm font-medium"
                                    >Add Image</span
                                >
                            </div>
                        </button>
                    </div>
                </Card.Content>
            </Card.Root>
        </Tabs.Content>

        <!-- Variants Tab -->
        <Tabs.Content value="variants" class="space-y-6">
            <!-- Variant Options -->
            <Card.Root>
                <Card.Header>
                    <div class="flex items-center justify-between">
                        <div>
                            <Card.Title>Variant Options</Card.Title>
                            <Card.Description>
                                Define the options for your product variants
                                (e.g., Color, Size).
                            </Card.Description>
                        </div>
                        <Button variant="outline" onclick={addVariantOption}>
                            <PlusIcon class="h-4 w-4 mr-2" />
                            Add Option
                        </Button>
                    </div>
                </Card.Header>
                <Card.Content class="space-y-4">
                    {#if data.options && Object.keys(data.options).length > 0}
                        {#each Object.entries(data.options) as [optionName, optionValues], optionIndex}
                            <div class="border rounded-lg p-4 bg-card">
                                <div
                                    class="flex items-center justify-between mb-4"
                                >
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="w-2 h-2 rounded-full bg-primary"
                                        ></div>
                                        <h4 class="font-medium text-lg">
                                            {optionName}
                                        </h4>
                                        <Badge variant="secondary"
                                            >{optionValues?.length || 0} values</Badge
                                        >
                                    </div>
                                    <Button
                                        variant="ghost"
                                        size="sm"
                                        onclick={() =>
                                            removeVariantOption(optionName)}
                                        class="text-destructive hover:text-destructive hover:bg-destructive/10"
                                    >
                                        <TrashIcon class="h-4 w-4 mr-1" />
                                        Remove
                                    </Button>
                                </div>

                                <div class="flex flex-wrap gap-2 mb-3">
                                    {#each optionValues || [] as value, valueIndex}
                                        <div class="flex items-center">
                                            <Badge
                                                variant="outline"
                                                class="pr-1 hover:bg-accent"
                                            >
                                                <span class="mr-2">{value}</span
                                                >
                                                <Button
                                                    variant="ghost"
                                                    size="icon"
                                                    class="h-4 w-4 p-0 hover:bg-destructive hover:text-destructive-foreground rounded-full"
                                                    onclick={() =>
                                                        removeOptionValue(
                                                            optionName,
                                                            valueIndex,
                                                        )}
                                                >
                                                    <XIcon class="h-3 w-3" />
                                                </Button>
                                            </Badge>
                                        </div>
                                    {/each}
                                </div>

                                <Button
                                    variant="outline"
                                    size="sm"
                                    onclick={() => addOptionValue(optionName)}
                                    class="w-full"
                                >
                                    <PlusIcon class="h-3 w-3 mr-2" />
                                    Add Value to {optionName}
                                </Button>
                            </div>
                        {/each}
                    {:else}
                        <div class="text-center py-12 text-muted-foreground">
                            <SettingsIcon
                                class="h-12 w-12 mx-auto mb-4 opacity-50"
                            />
                            <h3 class="font-medium text-lg mb-2">
                                No variant options defined
                            </h3>
                            <p class="text-sm mb-4">
                                Add options like Color, Size, or Material to
                                create product variants.
                            </p>
                            <Button
                                variant="outline"
                                onclick={addVariantOption}
                            >
                                <PlusIcon class="h-4 w-4 mr-2" />
                                Add Your First Option
                            </Button>
                        </div>
                    {/if}
                </Card.Content>
            </Card.Root>

            <!-- Variant Combinations -->
            {#if variantCombinations.length > 0}
                <Card.Root>
                    <Card.Header>
                        <Card.Title>Variant Combinations</Card.Title>
                        <Card.Description>
                            Select which combinations you want to include and
                            manage their details.
                        </Card.Description>
                    </Card.Header>
                    <Card.Content class="space-y-4">
                        <div class="grid gap-4">
                            {#each variantCombinations as combination}
                                <div
                                    class="border rounded-lg p-4 bg-card transition-colors {combination.enabled
                                        ? 'border-primary/20 bg-primary/5'
                                        : 'border-border'}"
                                >
                                    <div class="flex items-start gap-4">
                                        <div class="flex items-center pt-6">
                                            <Checkbox
                                                checked={combination.enabled}
                                                onCheckedChange={() =>
                                                    toggleVariantCombination(
                                                        combination.id,
                                                    )}
                                                aria-label="Enable variant: {getVariantDisplayName(
                                                    combination,
                                                )}"
                                            />
                                        </div>

                                        <div class="flex-1 min-w-0">
                                            <div
                                                class="flex items-center gap-3 mb-4"
                                            >
                                                <h4 class="font-medium text-lg">
                                                    {getVariantDisplayName(
                                                        combination,
                                                    )}
                                                </h4>
                                                {#if combination.isDefault}
                                                    <Badge
                                                        variant="default"
                                                        class="bg-primary text-primary-foreground"
                                                    >
                                                        <StarIcon
                                                            class="h-3 w-3 mr-1"
                                                        />
                                                        Default
                                                    </Badge>
                                                {/if}
                                            </div>

                                            {#if combination.enabled && combination.variant}
                                                <div
                                                    class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4"
                                                >
                                                    <div class="space-y-2">
                                                        <Label
                                                            for="sku-{combination.id}"
                                                            class="text-sm font-medium"
                                                            >SKU</Label
                                                        >
                                                        <Input
                                                            id="sku-{combination.id}"
                                                            bind:value={
                                                                combination
                                                                    .variant.sku
                                                            }
                                                            onchange={() =>
                                                                updateVariantsArray()}
                                                            placeholder="Variant SKU"
                                                            class="h-9"
                                                        />
                                                    </div>
                                                    <div class="space-y-2">
                                                        <Label
                                                            for="price-{combination.id}"
                                                            class="text-sm font-medium"
                                                            >Price (DZD)</Label
                                                        >
                                                        <Input
                                                            id="price-{combination.id}"
                                                            type="number"
                                                            step="0.01"
                                                            min="0"
                                                            value={getVariantPrice(
                                                                combination,
                                                            )}
                                                            onchange={(e) =>
                                                                updateVariantField(
                                                                    combination.id,
                                                                    "price",
                                                                    e.target
                                                                        .value,
                                                                )}
                                                            placeholder="0.00"
                                                            class="h-9"
                                                        />
                                                    </div>
                                                    <div class="space-y-2">
                                                        <Label
                                                            for="compare-{combination.id}"
                                                            class="text-sm font-medium"
                                                            >Compare Price</Label
                                                        >
                                                        <Input
                                                            id="compare-{combination.id}"
                                                            type="number"
                                                            step="0.01"
                                                            min="0"
                                                            value={getVariantComparePrice(
                                                                combination,
                                                            )}
                                                            onchange={(e) =>
                                                                updateVariantField(
                                                                    combination.id,
                                                                    "compare_price",
                                                                    e.target
                                                                        .value,
                                                                )}
                                                            placeholder="0.00"
                                                            class="h-9"
                                                        />
                                                    </div>
                                                    <div class="space-y-2">
                                                        <Label
                                                            for="stock-{combination.id}"
                                                            class="text-sm font-medium"
                                                            >Stock</Label
                                                        >
                                                        <Input
                                                            id="stock-{combination.id}"
                                                            type="number"
                                                            min="0"
                                                            value={getVariantStock(
                                                                combination,
                                                            )}
                                                            onchange={(e) =>
                                                                updateVariantField(
                                                                    combination.id,
                                                                    "stocks",
                                                                    e.target
                                                                        .value,
                                                                )}
                                                            placeholder="0"
                                                            class="h-9"
                                                        />
                                                    </div>
                                                </div>
                                            {/if}
                                        </div>

                                        {#if combination.enabled}
                                            <div
                                                class="flex flex-col gap-2 pt-6"
                                            >
                                                {#if !combination.isDefault}
                                                    <Button
                                                        variant="outline"
                                                        size="sm"
                                                        onclick={() =>
                                                            setDefaultVariant(
                                                                combination.id,
                                                            )}
                                                        class="h-9 px-3"
                                                    >
                                                        <StarIcon
                                                            class="h-3 w-3 mr-1"
                                                        />
                                                        Set Default
                                                    </Button>
                                                {/if}
                                            </div>
                                        {/if}
                                    </div>
                                </div>
                            {/each}
                        </div>

                        {#if variantCombinations.filter((c) => c.enabled).length === 0}
                            <div
                                class="text-center py-12 text-muted-foreground"
                            >
                                <div
                                    class="w-16 h-16 mx-auto mb-4 rounded-full bg-muted flex items-center justify-center"
                                >
                                    <CheckIcon class="h-8 w-8" />
                                </div>
                                <h3 class="font-medium text-lg mb-2">
                                    No variants selected
                                </h3>
                                <p class="text-sm">
                                    Choose at least one combination to enable
                                    variants for this product.
                                </p>
                            </div>
                        {:else}
                            <div class="mt-6 p-4 bg-muted/50 rounded-lg">
                                <div class="flex items-center gap-2 text-sm">
                                    <CheckIcon class="h-4 w-4 text-green-600" />
                                    <span class="font-medium">
                                        {variantCombinations.filter(
                                            (c) => c.enabled,
                                        ).length} variant{variantCombinations.filter(
                                            (c) => c.enabled,
                                        ).length === 1
                                            ? ""
                                            : "s"} enabled
                                    </span>
                                    {#if variantCombinations.find((c) => c.isDefault)}
                                        <span class="text-muted-foreground"
                                            >• Default variant set</span
                                        >
                                    {:else}
                                        <span class="text-amber-600"
                                            >• No default variant set</span
                                        >
                                    {/if}
                                </div>
                            </div>
                        {/if}
                    </Card.Content>
                </Card.Root>
            {/if}
        </Tabs.Content>
    </Tabs.Root>
</div>
