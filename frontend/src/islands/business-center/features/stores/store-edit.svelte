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

  import { canBeDeleted, fetchStore, StoreSchema } from "./service";
  import { useState } from "../../lib/utils/utils.svelte";
  import { createQuery } from "@tanstack/svelte-query";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { createForm } from "../../lib/utils/form";

  const { location } = useRoute();
  const storeId = location?.params.id;

  const query = createQuery(() => ({
    queryKey: ["store", storeId],
    queryFn: () => fetchStore(storeId),
  }));

  let form = $derived(useState(createForm(StoreSchema, query.data)));

  function handleAction(status: typeof form.status.value | "deleted") {
    return () => {
      console.log(`Store action: ${status}`);
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
          <Badge>{form.status.initialValue}</Badge>
        </SectionHeader>
      </div>
      <Group class="md:flex-row-reverse flex-wrap justify-start">
        {#if form.status.initialValue === "archived"}
          <ActionButton variant="default" onclick={handleAction("active")}>
            <RefreshCwIcon />
            Restore Store
          </ActionButton>
        {:else}
          <ActionButton
            variant="default"
            onclick={handleAction(form.status.initialValue)}
          >
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
            disabled={!canBeDeleted(form)}
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
