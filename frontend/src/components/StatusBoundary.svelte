<script lang="ts" context="module">
    export type LoadingStatus = undefined | null | any;
</script>

<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        status,
        error,
        children,
    }: {
        status: LoadingStatus;
        error?: Snippet;
        children: Snippet;
    } = $props();
</script>

{#if status === undefined}
    <span class="text-muted-foreground">Loading...</span>
{:else if status === null}
    {@render children()}
{:else if error}
    {@render error?.()}
{:else if status instanceof Error}
    <span class="text-red-500">Error: {status.message}</span>
{:else}
    <span class="text-red-500">Error: {status}</span>
{/if}
