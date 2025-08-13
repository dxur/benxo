<script lang="ts">
    import Column from "../../lib/components/layout/column.svelte";
    import Group from "../../lib/components/layout/group.svelte";
    import SectionHeader from "../../lib/components/section-header.svelte";
    import ActionButton from "../../lib/components/action-button.svelte";
    import { PlusIcon, SendIcon, ShoppingBagIcon } from "@lucide/svelte";

    import { createQuery } from "@tanstack/svelte-query";
    import { useNavigate } from "@dvcol/svelte-simple-router/router";
    import type { StoreDto } from "@bindings/StoreDto";
    import { useState } from "../../lib/utils/utils.svelte";
    import { createForm, validateForm, type Form } from "../../lib/utils/form";
    import StoreForm from "./store-form.svelte";
    import { StoreSchema } from "./service";
    import { Routes } from ".";

    const { replace } = useNavigate();

    const query = createQuery(() => ({
        queryKey: ["store-create"],
        queryFn: () => {
            let now = new Date().toISOString();
            return <StoreDto>{
                id: "",
                name: "Unnamed",
                description: "",
                status: "draft",
                created_at: now,
                updated_at: now,
            };
        },
    }));

    let form = $derived(useState(createForm(StoreSchema, query.data)));

    async function handleCreate() {
        try {
            validateForm(form);
            replace({
                path: Routes.EDIT_PAGE.path,
                params: { id: form.data?.id },
            });
        } catch (error) {
            console.error("Form submission error:", error);
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
                    icon={ShoppingBagIcon}
                    title="Create new store"
                    description="Fill in the details below to create a new store"
                />
            </div>
            <Group class="md:flex-row-reverse flex-wrap justify-start">
                <ActionButton>
                    <SendIcon />
                    Publish
                </ActionButton>
                <ActionButton
                    variant="secondary"
                    onclick={() => {
                        handleCreate();
                    }}
                >
                    <PlusIcon />
                    Create
                </ActionButton>
            </Group>
        </Group>
        <StoreForm bind:form general extra />
    </Column>
{/snippet}
