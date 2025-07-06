<script lang="ts">
    import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
    import { Button } from "@/lib/components/ui/button/index.js";
    import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index.js";
    import { link } from "@dvcol/svelte-simple-router";
    import { Routes } from "./index";
    import * as ApiRoutes from "@bindings/ApiRoutes";
    import { notifCenter } from "@/stores/notifications";

    let { id }: { id: string } = $props();

    async function archive() {
        ApiRoutes.archive_product({ id })
            .then(() => {
                notifCenter.success("Product archived successfully");
                // then reload the table somehow
            })
            .catch((err: Error) => {
                notifCenter.error(err.message);
            });
    }
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger>
        {#snippet child({ props })}
            <Button
                {...props}
                variant="ghost"
                size="icon"
                class="relative size-8 p-0"
            >
                <span class="sr-only">Open menu</span>
                <EllipsisIcon />
            </Button>
        {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        <a href={Routes.EDIT_PAGE.path} use:link={{ params: { id } }}>
            <DropdownMenu.Item>Edit</DropdownMenu.Item>
        </a>
        <DropdownMenu.Separator />
        <DropdownMenu.Item variant="destructive" onclick={archive}
            >Archive</DropdownMenu.Item
        >
    </DropdownMenu.Content>
</DropdownMenu.Root>
