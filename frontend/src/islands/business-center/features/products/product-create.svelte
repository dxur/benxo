<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index";
    import Column from "../../lib/components/layout/column.svelte";
    import Group from "../../lib/components/layout/group.svelte";
    import SectionHeader from "../../lib/components/section-header.svelte";
    import ActionButton from "../../lib/components/action-button.svelte";
    import { PlusIcon, SendIcon, PackageIcon } from "@lucide/svelte";

    import {
        createMutation,
        createQuery,
        getQueryClientContext,
    } from "@tanstack/svelte-query";
    import { useNavigate } from "@dvcol/svelte-simple-router/router";
    import { useState } from "../../lib/utils/utils.svelte";
    import { createForm, getFormValues, type Form } from "../../lib/utils/form";
    import { ProductSchema, useProductCreate } from "./service";
    import { Routes } from ".";
    import { toast } from "svelte-sonner";
    import { snakeToTitleCase } from "../../lib/utils/fmt";
    import ProductFormGeneral from "./product-form-general.svelte";
    import type { ProductCreateDto } from "@bindings/ProductCreateDto";
    import ProductFormVariants from "./product-form-variants.svelte";
    import ProductFormMedia from "./product-form-media.svelte";

    const { replace } = useNavigate();

    const query = createQuery(() => ({
        queryKey: ["product-create"],
        queryFn: () => {
            return <ProductCreateDto>{
                id: "",
                title: "Amazing Product",
                description: "",
                status: "inactive",
                featured: false,
                category: "",
                images: [],
                options: {},
                variants: [],
                slug: "",
            };
        },
    }));

    let { form } = $derived(useState(createForm(ProductSchema, query.data)));

    const queryContext = getQueryClientContext();

    const creationMutation = useProductCreate(
        (data) => {
            toast.success("Product created successfully");
            queryContext.invalidateQueries({
                queryKey: ["products"],
            });
            replace({
                path: Routes.EDIT_PAGE.path,
                params: { id: data.id },
            });
        },
        (e) => {
            toast.error("Error creating product", {
                description: e.message,
            });
        },
    );

    function handleCreate(status: typeof form.status.value) {
        form.status.value = status;
        try {
            const values = getFormValues<typeof ProductSchema>(form);
            creationMutation.mutate(values);
        } catch (e) {
            console.error("Validation Error", e);
            const errors = e as [string, string[]][];
            errors.forEach(([field, errors]) => {
                toast.error(`Error in field: ${snakeToTitleCase(field)}`, {
                    description: errors.join("\n"),
                });
            });
        }
    }
</script>

<div>
    {#if query.isLoading}
        <p>Loading...</p>
    {:else if query.isError}
        <p>Error: {query.error.message}</p>
    {:else if query.isSuccess}
        {@render body()}
    {/if}
</div>

{#snippet body()}
    <Column class="[&>*]:w-full items-center">
        <Group>
            <div class="flex items-center gap-4">
                <SectionHeader
                    icon={PackageIcon}
                    title="Create new product"
                    description="Fill in the details below to create a new product"
                />
            </div>
            <Group class="md:flex-row-reverse flex-wrap justify-start">
                <ActionButton onclick={() => handleCreate("active")}>
                    <SendIcon />
                    Publish
                </ActionButton>
                <ActionButton
                    variant="secondary"
                    onclick={() => handleCreate("inactive")}
                >
                    <PlusIcon />
                    Create
                </ActionButton>
            </Group>
        </Group>

        <Group class="max-w-4xl md:flex-col md:[&>*]:w-full lg:flex-row">
            <div class="flex-1">
                <Tabs.Root value="general">
                    <Tabs.List class="w-full">
                        <Tabs.Trigger value="general">General</Tabs.Trigger>
                        <Tabs.Trigger value="media">Media</Tabs.Trigger>
                        <Tabs.Trigger value="variants">Variants</Tabs.Trigger>
                    </Tabs.List>
                    <fieldset>
                        <Tabs.Content value="general" class="tab-content">
                            <ProductFormGeneral bind:form />
                        </Tabs.Content>
                        <Tabs.Content value="media" class="tab-content">
                            <ProductFormMedia bind:form />
                        </Tabs.Content>
                        <Tabs.Content value="variants" class="tab-content">
                            <ProductFormVariants bind:form />
                        </Tabs.Content>
                    </fieldset>
                </Tabs.Root>
            </div>
        </Group>
    </Column>
{/snippet}
