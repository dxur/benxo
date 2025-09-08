<script lang="ts">
    import Column from "../../lib/components/layout/column.svelte";
    import Group from "../../lib/components/layout/group.svelte";
    import SectionHeader from "../../lib/components/section-header.svelte";
    import ActionButton from "../../lib/components/action-button.svelte";
    import { PlusIcon, SendIcon, ShoppingBagIcon } from "@lucide/svelte";

    import {
        createMutation,
        createQuery,
        getQueryClientContext,
    } from "@tanstack/svelte-query";
    import { useNavigate } from "@dvcol/svelte-simple-router/router";
    import type { StoreDto } from "@bindings/StoreDto";
    import { useState } from "../../lib/utils/utils.svelte";
    import { createForm, getFormValues, type Form } from "../../lib/utils/form";
    import { StoreSchema, createStore } from "./service";
    import { Routes } from ".";
    import { toast } from "svelte-sonner";
    import { snakeToTitleCase } from "../../lib/utils/fmt";
    import StoreFormGeneral from "./store-form-general.svelte";
    import type { StoreCreateDto } from "@bindings/StoreCreateDto";
    import LoadingSpinner from "../../lib/components/loading-spinner.svelte";
    import LoadingError from "../../lib/components/loading-error.svelte";
    import { Button } from "@/lib/components/ui/button";

    const { replace } = useNavigate();

    const query = createQuery(() => ({
        queryKey: ["store-create"],
        queryFn: () => {
            return <Partial<StoreDto>>{
                id: "",
                name: "Amazing Store",
                description: "",
                category: null,
                status: "inactive",
                contact_email: null,
                contact_phone: null,
                address: null,
                city: null,
                zip_code: null,
                social_links: [],

                google_analytics_id: null,
                gtm_container_id: null,
                tracking_pixels: [],
                meta_title: null,
                meta_description: null,
                meta_keywords: null,
                custom_key_values: {},
                created_at: "<timestamp>",
                updated_at: "<timestamp>",
            };
        },
    }));

    let { form } = $derived(useState(createForm(StoreSchema, query.data)));

    const queryContext = getQueryClientContext();

    const creationMutation = createMutation(() => ({
        mutationFn: async (values: StoreCreateDto) => {
            return await createStore(values);
        },
        onSuccess: (data) => {
            toast.success("Store created successfully");
            queryContext.invalidateQueries({
                queryKey: ["stores"],
            });
            replace({
                path: Routes.EDIT_PAGE.path,
                params: { id: data.id },
            });
        },
        onError: (e) => {
            toast.error("Error creating product", {
                description: e.message,
            });
        },
    }));

    function handleCreate(status: typeof form.status.value) {
        form.status.value = status;
        try {
            const values = getFormValues<typeof StoreSchema>(form);
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
        <div class="h-56 flex items-center justify-center">
            <LoadingSpinner text="Loading..." />
        </div>
    {:else if query.isError}
        <LoadingError message={query.error.message}>
            <Button onclick={() => query.refetch()}>Retry</Button>
        </LoadingError>
    {:else if query.isSuccess}
        {@render body()}
    {/if}
</div>

{#snippet body()}
    <Column class="[&>*]:w-full items-center">
        <Group>
            <div class="flex items-center gap-4">
                <SectionHeader
                    icon={ShoppingBagIcon}
                    title="Create new store"
                    description="Fill in the details below to create a new store"
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
            <form class="flex-1">
                <fieldset class="tab-content">
                    <StoreFormGeneral bind:form />
                </fieldset>
            </form>
        </Group>
    </Column>
{/snippet}
