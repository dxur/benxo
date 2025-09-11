<script lang="ts">
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Textarea } from "$lib/components/ui/textarea/index";
    import * as Card from "$lib/components/ui/card/index";
    import * as Select from "$lib/components/ui/select/index";
    import { Button } from "$lib/components/ui/button/index";
    import { Badge } from "@/lib/components/ui/badge";
    import { Switch } from "$lib/components/ui/switch/index";
    import {
        PlusIcon,
        TrashIcon,
        ImageIcon,
        StarIcon,
        MoveUpIcon,
        MoveDownIcon,
    } from "@lucide/svelte";
    import type { Form } from "../../lib/utils/form";
    import type { ProductSchema } from "./service";
    import { snakeToTitleCase } from "../../lib/utils/fmt";
    import { single } from "@/lib/event";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof ProductSchema>;
    } = $props();

    function addImage() {
        const imageUrl = prompt("Enter image URL:");
        if (imageUrl) {
            form.images.value.push(imageUrl);
        }
    }

    function removeImage(index: number) {
        form.images.value.splice(index, 1);
    }

    function moveImageUp(index: number) {
        if (index === 0) return;
        const imageToMove = form.images.value[index];
        form.images.value[index] = form.images.value[index - 1];
        form.images.value[index - 1] = imageToMove;
    }

    function moveImageDown(index: number) {
        if (index + 1 >= form.images.value.length) return;
        const imageToMove = form.images.value[index];
        form.images.value[index] = form.images.value[index + 1];
        form.images.value[index + 1] = imageToMove;
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Product Images</Card.Title>
        <Card.Description>
            Add high-quality images to showcase your product. The first image
            will be used as the thumbnail.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {#each form.images.value || [] as image, index}
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
                        <div class="absolute bottom-2 left-2">
                            <Badge
                                variant="default"
                                class="bg-primary text-primary-foreground"
                            >
                                <StarIcon class="h-3 w-3 mr-1" />
                                Thumbnail
                            </Badge>
                        </div>
                    {/if}

                    <div class="absolute top-2 right-2 flex gap-1">
                        {#if index !== 0}
                            <Button
                                variant="secondary"
                                size="icon"
                                class="h-8 w-8 shadow-md"
                                onclick={single(() => moveImageUp(index))}
                                title="Move up"
                            >
                                <MoveUpIcon class="h-4 w-4" />
                            </Button>
                        {/if}
                        {#if index + 1 < (form.images.value?.length || 1)}
                            <Button
                                variant="secondary"
                                size="icon"
                                class="h-8 w-8 shadow-md"
                                onclick={single(() => moveImageDown(index))}
                                title="move Down"
                            >
                                <MoveDownIcon class="h-4 w-4" />
                            </Button>
                        {/if}
                        <Button
                            variant="destructive"
                            size="icon"
                            class="h-8 w-8 shadow-md"
                            onclick={single(() => removeImage(index))}
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
                onclick={single(addImage)}
                class="aspect-square rounded-lg border-2 border-dashed border-border hover:border-primary hover:bg-accent/50 transition-colors flex items-center justify-center text-muted-foreground hover:text-primary"
            >
                <div class="text-center">
                    <PlusIcon class="h-6 w-6 mx-auto mb-2" />
                    <span class="text-sm font-medium">Add Image</span>
                </div>
            </button>
        </div>
    </Card.Content>
</Card.Root>
