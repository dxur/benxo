<script module>
    import { writable } from "svelte/store";

    type DialogAction = {
        label: string;
        variant?: "default" | "secondary" | "destructive";
        value?: boolean | undefined;
    };

    type DialogRequest = {
        title: string;
        description?: string;
        actions: DialogAction[];
        resolve: (value: boolean | undefined) => void;
    };

    export const dialogStore = writable<DialogRequest | null>(null);

    export const dialog = {
        confirm(options: {
            title: string;
            description?: string;
            actions?: DialogAction[];
        }): Promise<boolean | undefined> {
            return new Promise((resolve) => {
                dialogStore.set({
                    title: options.title,
                    description: options.description,
                    actions: options.actions ?? [
                        { label: "Cancel", value: undefined },
                        {
                            label: "Discard",
                            value: false,
                            variant: "destructive",
                        },
                        { label: "Save", value: true },
                    ],
                    resolve: (value) => {
                        console.debug(
                            "[dialog.confirm] resolving with:",
                            value,
                        );
                        resolve(value);
                    },
                });
            });
        },
    };
</script>

<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import { buttonVariants } from "@/lib/components/ui/button";

    let request = $state<DialogRequest | null>(null);
    let open = $state(false);

    dialogStore.subscribe((val) => {
        request = val;
        open = !!val;
    });

    function handleAction(value: boolean | undefined) {
        if (request) {
            request.resolve(value);
        }
        open = false;
        dialogStore.set(null);
    }
</script>

<AlertDialog.Root bind:open>
    {#if request}
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title>{request.title}</AlertDialog.Title>
                {#if request.description}
                    <AlertDialog.Description>
                        {request.description}
                    </AlertDialog.Description>
                {/if}
            </AlertDialog.Header>
            <AlertDialog.Footer>
                {#each request.actions as action}
                    <button
                        class={buttonVariants({
                            variant: action.variant ?? "default",
                        })}
                        onclick={() => handleAction(action.value)}
                    >
                        {action.label}
                    </button>
                {/each}
            </AlertDialog.Footer>
        </AlertDialog.Content>
    {/if}
</AlertDialog.Root>
