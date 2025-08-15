<script lang="ts">
  import { Badge } from "@/lib/components/ui/badge";
  import Column from "../../lib/components/layout/column.svelte";
  import Group from "../../lib/components/layout/group.svelte";
  import SectionHeader from "../../lib/components/section-header.svelte";
  import ActionButton from "../../lib/components/action-button.svelte";
  import StoreForm from "./store-form.svelte";
  import {
    ArchiveIcon,
    PauseIcon,
    RefreshCwIcon,
    SaveIcon,
    SendIcon,
    ShoppingBagIcon,
    TrashIcon,
  } from "@lucide/svelte";

  import {
    canBeDeleted,
    fetchStore,
    StoreSchema,
    updateStore,
  } from "./service";
  import { useState } from "../../lib/utils/utils.svelte";
  import { createMutation, createQuery } from "@tanstack/svelte-query";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { createForm, getChangedValues } from "../../lib/utils/form";
  import { toast } from "svelte-sonner";
  import type { StoreUpdate } from "@bindings/StoreUpdate";
  import { snakeToTitleCase } from "../../lib/utils/fmt";

  const { location } = useRoute();
  const storeId = location?.params.id;

  const query = createQuery(() => ({
    queryKey: ["store", storeId],
    queryFn: () => fetchStore(storeId),
  }));

  const mutation = createMutation(() => ({
    mutationKey: ["store", storeId],
    mutationFn: (updateReq: StoreUpdate) => updateStore(storeId, updateReq),
    onSuccess: () => {
      toast.success("Product updated successfully");
      query.refetch();
    },
    onError: (error) => {
      toast.error("Error updating product", {
        description: error.message,
      });
    },
  }));

  let { form, data } = $derived(useState(createForm(StoreSchema, query.data)));

  function handleAction(status?: typeof form.status.value | "deleted") {
    return async () => {
      try {
        let values = getChangedValues(form);
        const shouldSave = new Promise<boolean | undefined>((resolve) => {
          if (!status) {
            resolve(true);
          } else {
            if (values) {
              // TODO use a real dialog to prompt the user
              resolve(
                confirm("You have unsaved changes. Do you want to save them?"),
              );
            } else {
              resolve(false);
            }
          }
        });

        const shouldSaveRes = await shouldSave;
        console.log("shouldSaveRes", shouldSaveRes);
        if (shouldSaveRes) {
          return;
        }

        let updateReq = null;
        if (shouldSaveRes && values) {
          updateReq = values;
          updateReq.status = status;
        } else {
          updateReq = { status };
        }

        mutation.mutate(updateReq as any);
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
          title={form.name.initialValue}
          description="Manage store details"
        >
          <Badge>{snakeToTitleCase(form.status.initialValue)}</Badge>
        </SectionHeader>
      </div>
      <Group class="md:flex-row-reverse flex-wrap justify-start">
        {#if form.status.initialValue === "archived"}
          <ActionButton variant="default" onclick={handleAction("active")}>
            <RefreshCwIcon />
            Restore Store
          </ActionButton>
        {:else}
          <ActionButton variant="default" onclick={handleAction()}>
            <SaveIcon />
            Save Changes
          </ActionButton>
        {/if}

        {#if form.status.initialValue === "inactive"}
          <ActionButton variant="secondary" onclick={handleAction("active")}>
            <SendIcon />
            Publish Store
          </ActionButton>
        {:else if form.status.initialValue === "active"}
          <ActionButton
            variant="destructive"
            onclick={handleAction("inactive")}
          >
            <PauseIcon />
            Unpublish Store
          </ActionButton>
        {:else if form.status.initialValue === "archived"}
          <ActionButton
            variant="destructive"
            disabled={!canBeDeleted(data)}
            onclick={handleAction("deleted")}
          >
            <TrashIcon />
            Delete Store
          </ActionButton>
        {/if}

        {#if form.status.initialValue === "inactive"}
          <ActionButton
            variant="destructive"
            onclick={handleAction("archived")}
          >
            <ArchiveIcon />
            Archive Store
          </ActionButton>
        {/if}
      </Group>
    </Group>
    <StoreForm
      bind:form
      all
      disabled={form.status.initialValue === "archived"}
    />
  </Column>
{/snippet}
