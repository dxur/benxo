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
    import StoreForm from "./store-form.svelte";
    import { StoreSchema, createStore } from "./service";
    import { Routes } from ".";
    import { toast } from "svelte-sonner";

    const { replace } = useNavigate();

    const query = createQuery(() => ({
        queryKey: ["store-create"],
        queryFn: () => {
            let now = new Date().toISOString();
            return <StoreDto>{
                id: "",
                name: "",
                description: "",
                status: "inactive",
                created_at: now,
                updated_at: now,
            };
        },
    }));

    let { form } = $derived(useState(createForm(StoreSchema, query.data)));

    function handleCreate(status: typeof form.status.value) {
        return () => {
            form.status.value = status;
            try {
                const values = getFormValues<typeof StoreSchema>(form);
                createMutation(() => ({
                    mutationFn: async () => {
                        return await createStore(values);
                    },
                    onSuccess: (data) => {
                        getQueryClientContext().setQueryData(
                            ["store", data.id],
                            data,
                        );
                        toast.success("Product created successfully");
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
            } catch (e) {
                console.error(e);
                const errors = e as [string, string[]][];
                errors.forEach(([field, errors]) => {
                    toast.error(`Error in field: ${field}`, {
                        description: errors.join("\n"),
                    });
                });
            }
        };
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
                <ActionButton onclick={handleCreate("active")}>
                    <SendIcon />
                    Publish
                </ActionButton>
                <ActionButton
                    variant="secondary"
                    onclick={handleCreate("inactive")}
                >
                    <PlusIcon />
                    Create
                </ActionButton>
            </Group>
        </Group>
        <StoreForm bind:form general extra />
    </Column>
{/snippet}
