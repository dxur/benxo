<script lang="ts">
    import { Button } from "@/lib/components/ui/button/index";
    import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index";
    import * as AlertDialog from "@/lib/components/ui/alert-dialog/index";
    import { Badge } from "@/lib/components/ui/badge/index";

    import MoreHorizontalIcon from "@lucide/svelte/icons/more-horizontal";
    import EyeIcon from "@lucide/svelte/icons/eye";
    import EditIcon from "@lucide/svelte/icons/edit";
    import SettingsIcon from "@lucide/svelte/icons/settings";
    import GlobeIcon from "@lucide/svelte/icons/globe";
    import CopyIcon from "@lucide/svelte/icons/copy";
    import PlayIcon from "@lucide/svelte/icons/play";
    import PauseIcon from "@lucide/svelte/icons/pause";
    import TrashIcon from "@lucide/svelte/icons/trash";
    import DownloadIcon from "@lucide/svelte/icons/download";

    import { link } from "@dvcol/svelte-simple-router";

    export let id: string;
    export let status: "ACTIVE" | "INACTIVE" | "DRAFT" | "SUSPENDED";
    export let domain: string;

    let showDeleteDialog = false;
    let isDeleting = false;

    // Handle actions
    async function handleActivate() {
        try {
            // TODO: Implement activate store API call
            console.log("Activating store:", id);
            toast.success("Store activated successfully");
        } catch (error) {
            console.error("Error activating store:", error);
            toast.error("Failed to activate store");
        }
    }

    async function handleDeactivate() {
        try {
            // TODO: Implement deactivate store API call
            console.log("Deactivating store:", id);
            toast.success("Store deactivated successfully");
        } catch (error) {
            console.error("Error deactivating store:", error);
            toast.error("Failed to deactivate store");
        }
    }

    async function handleDelete() {
        isDeleting = true;
        try {
            // TODO: Implement delete store API call
            console.log("Deleting store:", id);
            toast.success("Store deleted successfully");
            showDeleteDialog = false;
            // Refresh the page or update the data
            window.location.reload();
        } catch (error) {
            console.error("Error deleting store:", error);
            toast.error("Failed to delete store");
        } finally {
            isDeleting = false;
        }
    }

    async function copyDomain() {
        try {
            await navigator.clipboard.writeText(`https://${domain}`);
            toast.success("Domain copied to clipboard");
        } catch (error) {
            console.error("Error copying domain:", error);
            toast.error("Failed to copy domain");
        }
    }

    function openStore() {
        window.open(`https://${domain}`, "_blank");
    }

    function exportStoreData() {
        // TODO: Implement export store data functionality
        console.log("Exporting store data:", id);
        toast.success("Store data export started");
    }
</script>

<div class="flex items-center gap-1">
    <!-- Quick Actions -->
    <Button
        variant="ghost"
        size="sm"
        onclick={openStore}
        class="h-8 w-8 p-0"
        title="View Store"
    >
        <EyeIcon class="h-4 w-4" />
    </Button>

    <a href="/stores/{id}/edit" use:link>
        <Button
            variant="ghost"
            size="sm"
            class="h-8 w-8 p-0"
            title="Edit Store"
        >
            <EditIcon class="h-4 w-4" />
        </Button>
    </a>

    <!-- More Actions Dropdown -->
    <DropdownMenu.Root>
        <DropdownMenu.Trigger asChild let:builder>
            <Button
                builders={[builder]}
                variant="ghost"
                size="sm"
                class="h-8 w-8 p-0"
                title="More Actions"
            >
                <MoreHorizontalIcon class="h-4 w-4" />
            </Button>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content align="end" class="w-56">
            <DropdownMenu.Group>
                <DropdownMenu.Item onclick={openStore} class="gap-2">
                    <GlobeIcon class="h-4 w-4" />
                    Visit Store
                </DropdownMenu.Item>
                <DropdownMenu.Item onclick={copyDomain} class="gap-2">
                    <CopyIcon class="h-4 w-4" />
                    Copy Domain
                </DropdownMenu.Item>
            </DropdownMenu.Group>

            <DropdownMenu.Separator />

            <DropdownMenu.Group>
                <DropdownMenu.Item href="/stores/{id}/edit" class="gap-2">
                    <EditIcon class="h-4 w-4" />
                    Edit Store
                </DropdownMenu.Item>
                <DropdownMenu.Item href="/stores/{id}/settings" class="gap-2">
                    <SettingsIcon class="h-4 w-4" />
                    Settings
                </DropdownMenu.Item>
                <DropdownMenu.Item href="/stores/{id}/domains" class="gap-2">
                    <GlobeIcon class="h-4 w-4" />
                    Manage Domains
                </DropdownMenu.Item>
            </DropdownMenu.Group>

            <DropdownMenu.Separator />

            <DropdownMenu.Group>
                {#if status === "ACTIVE"}
                    <DropdownMenu.Item onclick={handleDeactivate} class="gap-2">
                        <PauseIcon class="h-4 w-4" />
                        Deactivate Store
                    </DropdownMenu.Item>
                {:else if status === "INACTIVE" || status === "DRAFT"}
                    <DropdownMenu.Item onclick={handleActivate} class="gap-2">
                        <PlayIcon class="h-4 w-4" />
                        Activate Store
                    </DropdownMenu.Item>
                {/if}
                <DropdownMenu.Item onclick={exportStoreData} class="gap-2">
                    <DownloadIcon class="h-4 w-4" />
                    Export Data
                </DropdownMenu.Item>
            </DropdownMenu.Group>

            <DropdownMenu.Separator />

            <DropdownMenu.Item
                onclick={() => (showDeleteDialog = true)}
                class="gap-2 text-red-600 focus:text-red-600"
            >
                <TrashIcon class="h-4 w-4" />
                Delete Store
            </DropdownMenu.Item>
        </DropdownMenu.Content>
    </DropdownMenu.Root>
</div>

<!-- Delete Confirmation Dialog -->
<AlertDialog.Root bind:open={showDeleteDialog}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Delete Store</AlertDialog.Title>
            <AlertDialog.Description>
                Are you sure you want to delete this store? This action cannot
                be undone and will permanently remove all store data, including
                products, orders, and customer information.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action
                onclick={handleDelete}
                disabled={isDeleting}
                class="bg-red-600 hover:bg-red-700"
            >
                {isDeleting ? "Deleting..." : "Delete Store"}
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
