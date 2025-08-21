<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Dialog from "$lib/components/ui/dialog/index";
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import { Checkbox } from "$lib/components/ui/checkbox/index";
    import { PlusIcon, TrashIcon, ImageIcon, XIcon } from "@lucide/svelte";
    import { Badge } from "@/lib/components/ui/badge/index";
    import { ProductSchema } from "./service";
    import { onMount, untrack } from "svelte";
    import { cloneDeep, kebabCase, snakeCase } from "lodash";
    import type { Form } from "../../lib/utils/form";

    interface ProductOption {
        name: string;
        values: string[];
    }

    interface ProductVariant {
        enabled: boolean;
        images: string[];
        options: Record<string, string>;
        sku: string;
        price: string;
        compare_at: string | null;
        stocks: number;
    }

    let {
        form = $bindable(),
    }: {
        form: Form<typeof ProductSchema>;
    } = $props();

    let variants: ProductVariant[] = $state(
        form.variants.value.map((item) => ({
            ...cloneDeep(item),
            enabled: true,
        })),
    );

    let options: ProductOption[] = $state(
        form.variants.value.reduce((acc: ProductOption[], variant) => {
            Object.entries(variant.options).forEach(([name, value]) => {
                let option = acc.find((opt) => opt.name === name);
                if (!option) {
                    option = { name, values: [] };
                    acc.push(option);
                }
                if (!option.values.includes(value as string)) {
                    option.values.push(value as string);
                }
            });
            return acc;
        }, []),
    );

    let selectedVariantIndex = $state<number | null>(null);
    let showImageSelector = $state(false);
    let newImageUrl = $state("");

    function generateVariantCombinations(
        opts: ProductOption[],
    ): Record<string, string>[] {
        if (opts.length === 0) return [{}];

        const [first, ...rest] = opts;
        const restCombinations = generateVariantCombinations(rest);

        const combinations: Record<string, string>[] = [];
        for (const value of first.values) {
            for (const restCombination of restCombinations) {
                combinations.push({
                    [first.name]: value,
                    ...restCombination,
                });
            }
        }

        return combinations;
    }

    function generateSku(
        name: string,
        optionCombination: Record<string, string>,
        index: number,
    ): string {
        const values = Object.values(optionCombination);
        if (values.length === 0) return "";

        const shortValues = values.map((v) => v.substring(0, 3).toUpperCase());
        return `SKU-${kebabCase(name).toUpperCase()}-${shortValues.join("-")}`;
    }

    function combinationKey(combo: Record<string, string>): string {
        const entries = Object.entries(combo)
            .filter(([, v]) => v != null && String(v).trim() !== "")
            .sort(([a], [b]) => a.localeCompare(b)); // stable regardless of object key order

        if (entries.length === 0) return "default";

        return entries
            .map(([, v]) => String(v).toLowerCase().trim().replace(/\s+/g, "-"))
            .join("|");
    }

    function updateVariants() {
        const cleanedOptions = options
            .map((opt) => ({
                name: opt.name,
                values: opt.values
                    .map((v) => v.trim())
                    .filter((v) => v.length > 0),
            }))
            .filter((opt) => opt.values.length > 0);

        const combinations = generateVariantCombinations(cleanedOptions);

        const existingByCombo = new Map(
            variants.map((v) => [combinationKey(v.options), v]),
        );

        variants = combinations.map((combination, index) => {
            const key = combinationKey(combination);
            const existing = existingByCombo.get(key);
            if (existing) {
                existing.options = combination;
                return existing;
            }
            return {
                enabled: true,
                sku: generateSku(form.title.value, combination, index),
                price: "",
                compare_at: null,
                stocks: 0,
                options: combination,
                images: [],
            } as ProductVariant;
        });
    }

    function addOption() {
        options.push({
            name: `Option ${options.length + 1}`,
            values: [""],
        });
    }

    function removeOption(index: number) {
        options.splice(index, 1);
        updateVariants();
    }

    function addOptionValue(optionIndex: number) {
        options[optionIndex].values.push("");
    }

    function removeOptionValue(optionIndex: number, valueIndex: number) {
        options[optionIndex].values.splice(valueIndex, 1);
        updateVariants();
    }

    function openImageSelector(variantIndex: number) {
        selectedVariantIndex = variantIndex;
        showImageSelector = true;
        newImageUrl = "";
    }

    function addImageFromUrl() {
        if (newImageUrl.trim() && selectedVariantIndex !== null) {
            variants[selectedVariantIndex].images.push(newImageUrl.trim());
            newImageUrl = "";
        }
    }

    function addImageFromProduct(imageUrl: string) {
        if (
            selectedVariantIndex !== null &&
            !variants[selectedVariantIndex].images.includes(imageUrl)
        ) {
            variants[selectedVariantIndex].images.push(imageUrl);
        }
    }

    function removeVariantImage(variantIndex: number, imageIndex: number) {
        variants[variantIndex].images.splice(imageIndex, 1);
    }

    function closeImageSelector() {
        showImageSelector = false;
        selectedVariantIndex = null;
        newImageUrl = "";
    }

    // Format variant display name
    function getVariantDisplayName(variant: ProductVariant): string {
        const optionValues = Object.values(variant.options);
        return optionValues.length > 0
            ? optionValues.join(" / ")
            : "Default Variant";
    }

    $effect(() => {
        const filtered = $state
            .snapshot(variants)
            .filter((item) => item.enabled)
            .map((item) => ({ ...item, enabled: undefined }));

        untrack(() => {
            form.variants.value = filtered;
        });
    });

    onMount(() => {
        updateVariants();
    });
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Product Options</Card.Title>
        <Card.Description>
            Define options like Size, Color, etc. Variants will be automatically
            generated from these options.
        </Card.Description>
        <Card.Action>
            <Button variant="outline" size="sm" onclick={addOption}>
                <PlusIcon class="w-4 h-4" />
                Add Option
            </Button>
        </Card.Action>
    </Card.Header>
    <Card.Content class="space-y-4">
        {#if options.length === 0}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                No options added yet.
                <span class="block text-sm text-gray-400 mt-1">
                    Add options like Size, Color, etc. or leave empty for a
                    single default variant.
                </span>
            </div>
        {/if}

        {#each options as option, optionIndex}
            <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                    <Label class="text-sm font-medium">Option Name</Label>
                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => removeOption(optionIndex)}
                    >
                        <TrashIcon class="w-4 h-4" />
                    </Button>
                </div>

                <Input
                    bind:value={option.name}
                    placeholder="e.g. Size, Color, Material"
                />

                <div class="space-y-2">
                    <div class="flex items-center justify-between">
                        <Label class="text-sm font-medium">Values</Label>
                        <Button
                            variant="ghost"
                            size="sm"
                            onclick={() => addOptionValue(optionIndex)}
                        >
                            <PlusIcon />
                        </Button>
                    </div>

                    {#each option.values as value, valueIndex}
                        <div class="flex items-center gap-2">
                            <Input
                                bind:value={option.values[valueIndex]}
                                placeholder="e.g. Small, Medium, Large"
                                class="w-full"
                                onblur={updateVariants}
                            />
                            <Button
                                variant="ghost"
                                size="sm"
                                onclick={() =>
                                    removeOptionValue(optionIndex, valueIndex)}
                            >
                                <TrashIcon />
                            </Button>
                        </div>
                    {/each}
                </div>
            </div>
        {/each}
    </Card.Content>
</Card.Root>

<!-- Product Variants -->
<Card.Root>
    <Card.Header>
        <Card.Title
            >Product Variants ({variants.filter((v) => v.enabled).length} enabled)</Card.Title
        >
        <Card.Description>
            {variants.length > 0
                ? `${variants.length} variant${variants.length === 1 ? "" : "s"} generated from your options. Select which ones to include.`
                : "Variants will appear here once you add options above."}
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        {#if variants.length === 0}
            <div
                class="p-6 text-center text-gray-500 border border-dashed rounded-lg"
            >
                {options.length === 0
                    ? "Add options above to generate variants, or a default variant will be created."
                    : "Add values to your options to generate variants."}
            </div>
        {:else}
            {#each variants as variant, index}
                <div
                    class="border rounded-lg p-4 space-y-4 {variant.enabled
                        ? ''
                        : 'opacity-60'}"
                >
                    <div class="flex items-center gap-3">
                        <Checkbox
                            bind:checked={variant.enabled}
                            id="variant-{index}"
                        />
                        <Label
                            for="variant-{index}"
                            class="text-base font-medium cursor-pointer"
                        >
                            {getVariantDisplayName(variant)}
                        </Label>
                    </div>

                    {#if variant.enabled}
                        <div
                            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4"
                        >
                            <div>
                                <Label class="text-sm">SKU</Label>
                                <Input
                                    bind:value={variant.sku}
                                    placeholder="SKU-001"
                                    class="mt-1"
                                />
                            </div>
                            <div>
                                <Label class="text-sm">Price ($)</Label>
                                <Input
                                    bind:value={variant.price}
                                    placeholder="19.99"
                                    type="number"
                                    step="0.01"
                                    min="0"
                                    class="mt-1"
                                />
                            </div>
                            <div>
                                <Label class="text-sm"
                                    >Compare At Price ($)</Label
                                >
                                <Input
                                    bind:value={variant.compare_at}
                                    placeholder="29.99"
                                    type="number"
                                    step="0.01"
                                    min="0"
                                    class="mt-1"
                                />
                            </div>
                            <div>
                                <Label class="text-sm">Stock Quantity</Label>
                                <Input
                                    bind:value={variant.stocks}
                                    placeholder="100"
                                    type="number"
                                    min="0"
                                    class="mt-1"
                                />
                            </div>
                        </div>

                        <!-- Variant Images -->
                        <div class="border-t pt-4 mt-4">
                            <div class="flex items-center justify-between mb-3">
                                <Label class="text-sm font-medium"
                                    >Variant Images</Label
                                >
                                <Button
                                    variant="outline"
                                    size="sm"
                                    onclick={() => openImageSelector(index)}
                                >
                                    <ImageIcon class="w-4 h-4 mr-2" />
                                    Manage Images
                                </Button>
                            </div>

                            <div class="flex flex-wrap gap-2">
                                {#if variant.images.length === 0}
                                    <div
                                        class="text-sm text-muted-foreground italic"
                                    >
                                        No images assigned to this variant
                                    </div>
                                {:else}
                                    {#each variant.images as image, imageIndex}
                                        <div class="relative group">
                                            <img
                                                src={image}
                                                alt="Variant image {imageIndex +
                                                    1}"
                                                class="w-16 h-16 object-cover rounded border-2 border-border hover:border-primary/50 transition-colors"
                                            />
                                            <button
                                                class="absolute -top-1 -right-1 bg-destructive text-destructive-foreground rounded-full w-5 h-5 flex items-center justify-center text-xs opacity-0 group-hover:opacity-100 transition-opacity"
                                                onclick={() =>
                                                    removeVariantImage(
                                                        index,
                                                        imageIndex,
                                                    )}
                                                title="Remove image"
                                            >
                                                <XIcon class="w-3 h-3" />
                                            </button>
                                        </div>
                                    {/each}
                                {/if}
                            </div>
                        </div>

                        <!-- Display variant options (read-only) -->
                        {#if Object.keys(variant.options).length > 0}
                            <div class="border-t pt-3 mt-3">
                                <Label class="text-sm font-medium mb-2 block"
                                    >Variant Options</Label
                                >
                                <div class="flex flex-wrap gap-2">
                                    {#each Object.entries(variant.options) as [optionName, optionValue]}
                                        <Badge>
                                            {optionName + ": " + optionValue}
                                        </Badge>
                                    {/each}
                                </div>
                            </div>
                        {/if}
                    {/if}
                </div>
            {/each}
        {/if}

        {#if form.variants.errors}
            <div>
                {#each form.variants.errors as error}
                    <p class="text-sm text-destructive">{error}</p>
                {/each}
            </div>
        {/if}
    </Card.Content>
</Card.Root>

<!-- Image Selector Dialog -->
<Dialog.Root bind:open={showImageSelector}>
    <Dialog.Content class="max-w-4xl max-h-[80vh] overflow-y-auto">
        <Dialog.Header>
            <Dialog.Title>
                Select Images for {selectedVariantIndex !== null
                    ? getVariantDisplayName(variants[selectedVariantIndex])
                    : ""}
            </Dialog.Title>
            <Dialog.Description>
                Choose images from your product gallery or add new ones for this
                specific variant.
            </Dialog.Description>
        </Dialog.Header>

        <div class="space-y-6">
            <!-- Add new image by URL -->
            <div class="space-y-3">
                <Label class="text-sm font-medium">Add New Image</Label>
                <div class="flex gap-2">
                    <Input
                        bind:value={newImageUrl}
                        placeholder="Enter image URL..."
                        class="flex-1"
                        onkeydown={(e) => {
                            if (e.key === "Enter") {
                                e.preventDefault();
                                addImageFromUrl();
                            }
                        }}
                    />
                    <Button
                        onclick={addImageFromUrl}
                        disabled={!newImageUrl.trim()}
                    >
                        <PlusIcon class="w-4 h-4" />
                        Add
                    </Button>
                </div>
            </div>

            <!-- Select from product images -->
            {#if form.images.value && form.images.value.length > 0}
                <div class="space-y-3">
                    <Label class="text-sm font-medium"
                        >Select from Product Images</Label
                    >
                    <div
                        class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3"
                    >
                        {#each form.images.value as image, imageIndex}
                            <div class="relative">
                                <button
                                    class="w-full aspect-square rounded-lg overflow-hidden border-2 transition-colors {selectedVariantIndex !==
                                        null &&
                                    variants[
                                        selectedVariantIndex
                                    ].images.includes(image)
                                        ? 'border-primary bg-primary/10'
                                        : 'border-border hover:border-primary/50'}"
                                    onclick={() => addImageFromProduct(image)}
                                >
                                    <img
                                        src={image}
                                        alt="Product image {imageIndex + 1}"
                                        class="w-full h-full object-cover"
                                    />
                                </button>
                                {#if selectedVariantIndex !== null && variants[selectedVariantIndex].images.includes(image)}
                                    <div class="absolute top-2 right-2">
                                        <Badge
                                            variant="default"
                                            class="text-xs"
                                        >
                                            Selected
                                        </Badge>
                                    </div>
                                {/if}
                            </div>
                        {/each}
                    </div>
                </div>
            {:else}
                <div class="text-center py-8 text-muted-foreground">
                    <ImageIcon class="w-12 h-12 mx-auto mb-3 opacity-50" />
                    <p>No product images available.</p>
                    <p class="text-sm">
                        Add images in the Media tab first, or use the URL input
                        above.
                    </p>
                </div>
            {/if}

            <!-- Currently selected images for this variant -->
            {#if selectedVariantIndex !== null && variants[selectedVariantIndex].images.length > 0}
                <div class="space-y-3">
                    <Label class="text-sm font-medium"
                        >Currently Selected ({variants[selectedVariantIndex]
                            .images.length})</Label
                    >
                    <div
                        class="grid grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-2"
                    >
                        {#each variants[selectedVariantIndex].images as image, imageIndex}
                            <div class="relative group">
                                <img
                                    src={image}
                                    alt="Selected image {imageIndex + 1}"
                                    class="w-full aspect-square object-cover rounded border-2 border-primary"
                                />
                                <button
                                    class="absolute -top-1 -right-1 bg-destructive text-destructive-foreground rounded-full w-5 h-5 flex items-center justify-center text-xs opacity-0 group-hover:opacity-100 transition-opacity"
                                    onclick={() =>
                                        selectedVariantIndex !== null &&
                                        removeVariantImage(
                                            selectedVariantIndex,
                                            imageIndex,
                                        )}
                                    title="Remove image"
                                >
                                    <XIcon class="w-3 h-3" />
                                </button>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>

        <Dialog.Footer>
            <Button variant="outline" onclick={closeImageSelector}>
                Close
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
