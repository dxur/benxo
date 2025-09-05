<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index";
  import * as AlertDialog from "$lib/components/ui/alert-dialog/index";
  import { Badge } from "@/lib/components/ui/badge";
  import Column from "../../lib/components/layout/column.svelte";
  import Group from "../../lib/components/layout/group.svelte";
  import SectionHeader from "../../lib/components/section-header.svelte";
  import ActionButton from "../../lib/components/action-button.svelte";
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
    deleteStore,
    fetchStore,
    StoreSchema,
    updateStore,
  } from "./service";
  import { useNavigate } from "@dvcol/svelte-simple-router/router";
  import { useState } from "../../lib/utils/utils.svelte";
  import { createMutation, createQuery } from "@tanstack/svelte-query";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { createForm, getChangedValues } from "../../lib/utils/form";
  import { toast } from "svelte-sonner";
  import type { StoreUpdate } from "@bindings/StoreUpdate";
  import { snakeToTitleCase } from "../../lib/utils/fmt";
  import StoreFormGeneral from "./store-form-general.svelte";
  import StoreDomain from "./store-domain.svelte";
  import StoreFormTemplate from "./store-form-template.svelte";
  import StoreFormExtra from "./store-form-extra.svelte";
  import { dialog } from "../../lib/components/alert-dialog.svelte";
  import { Routes } from ".";

  const { replace } = useNavigate();

  const { location } = useRoute();
  const storeId = location?.params.id as string;

  const query = createQuery(() => ({
    queryKey: ["store", storeId],
    queryFn: () => fetchStore(storeId),
  }));

  const updateMutation = createMutation(() => ({
    mutationKey: ["store", storeId],
    mutationFn: (updateReq: StoreUpdate) => updateStore(storeId, updateReq),
    onSuccess: () => {
      toast.success("Store updated successfully");
      query.refetch();
    },
    onError: (error) => {
      toast.error("Error updating store", {
        description: error.message,
      });
    },
  }));

  const deleteMutation = createMutation(() => ({
    mutationKey: ["store-delete", storeId],
    mutationFn: () => deleteStore(storeId),
    onSuccess: () => {
      toast.success("Store deleted successfully");
      replace({
        path: Routes.EDIT_PAGE.path,
        params: { id: storeId },
      });
    },
    onError: (error) => {
      toast.error("Error deleting store", {
        description: error.message,
      });
    },
  }));

  let { form, data } = $derived(useState(createForm(StoreSchema, query.data)));

  type StoreAction =
    | "save"
    | "publish"
    | "unpublish"
    | "archive"
    | "restore"
    | "delete";

  async function handleAction(action: StoreAction) {
    try {
      let values = getChangedValues(form);

      if (action === "delete") {
        const confirmed = await dialog.confirm({
          title: "Delete this store?",
          description: "This action cannot be undone.",
          actions: [
            { label: "Cancel", value: false },
            { label: "Delete", value: true, variant: "destructive" },
          ],
        });
        if (!confirmed) return;

        deleteMutation.mutate();
        return;
      }

      if (action === "archive") {
        const confirmed = await dialog.confirm({
          title: "Archive this store?",
          description:
            "Archived stores cannot be published until restored. Continue?",
          actions: [
            { label: "Cancel", value: false },
            { label: "Archive", value: true, variant: "destructive" },
          ],
        });
        if (!confirmed) return;
      }

      if (action === "unpublish") {
        const confirmed = await dialog.confirm({
          title: "Unpublish this store?",
          description:
            "Customers will not be able to view this store until you publish it again.",
          actions: [
            { label: "Cancel", value: false },
            { label: "Unpublish", value: true, variant: "destructive" },
          ],
        });
        if (!confirmed) return;
      }

      let updateReq: Partial<StoreUpdate> = values ?? {};
      switch (action) {
        case "save":
          break;
        case "publish":
          updateReq.status = "active";
          break;
        case "unpublish":
          updateReq.status = "inactive";
          break;
        case "archive":
          updateReq.status = "archived";
          break;
        case "restore":
          updateReq.status = "inactive";
          break;
      }

      console.info("Updating store with values:", updateReq);

      if (Object.keys(updateReq).length > 0) {
        updateMutation.mutate(updateReq as StoreUpdate);
      } else {
        toast.warning("No changes have made");
      }
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
          icon={ShoppingBagIcon}
          title={form.name.initialValue}
          description="Manage store details"
        >
          <Badge>{snakeToTitleCase(form.status.initialValue)}</Badge>
        </SectionHeader>
      </div>
      <Group class="md:flex-row-reverse flex-wrap justify-start">
        {#if form.status.initialValue === "archived"}
          <ActionButton
            variant="default"
            onclick={() => handleAction("restore")}
          >
            <RefreshCwIcon />
            Restore Store
          </ActionButton>
        {:else}
          <ActionButton variant="default" onclick={() => handleAction("save")}>
            <SaveIcon />
            Save Changes
          </ActionButton>
        {/if}

        {#if form.status.initialValue === "inactive"}
          <ActionButton
            variant="secondary"
            onclick={() => handleAction("publish")}
          >
            <SendIcon />
            Publish Store
          </ActionButton>
        {:else if form.status.initialValue === "active"}
          <ActionButton
            variant="destructive"
            onclick={() => handleAction("unpublish")}
          >
            <PauseIcon />
            Unpublish Store
          </ActionButton>
        {:else if form.status.initialValue === "archived"}
          <ActionButton
            variant="destructive"
            disabled={!canBeDeleted(data)}
            onclick={() => handleAction("delete")}
          >
            <TrashIcon />
            Delete Store
          </ActionButton>
        {/if}

        {#if form.status.initialValue === "inactive"}
          <ActionButton
            variant="destructive"
            onclick={() => handleAction("archive")}
          >
            <ArchiveIcon />
            Archive Store
          </ActionButton>
        {/if}
      </Group>
    </Group>

    <Group class="max-w-4xl md:flex-col md:[&>*]:w-full lg:flex-row">
      <form class="flex-1">
        <Tabs.Root value="general">
          <Tabs.List class="w-full">
            <Tabs.Trigger value="general">General</Tabs.Trigger>
            <Tabs.Trigger value="domain">Domain & Access</Tabs.Trigger>
            <Tabs.Trigger value="template">Template</Tabs.Trigger>
            <Tabs.Trigger value="extra">Extra</Tabs.Trigger>
          </Tabs.List>
          <fieldset>
            <Tabs.Content value="general" class="tab-content">
              <StoreFormGeneral bind:form />
            </Tabs.Content>
            <Tabs.Content value="domain" class="tab-content">
              <StoreDomain {storeId} bind:store={form} />
            </Tabs.Content>
            <Tabs.Content value="template" class="tab-content">
              <StoreFormTemplate bind:form />
            </Tabs.Content>
            <Tabs.Content value="extra" class="tab-content">
              <StoreFormExtra bind:form />
            </Tabs.Content>
          </fieldset>
        </Tabs.Root>
      </form>
    </Group>
  </Column>
{/snippet}
