<script>
    import { SearchIcon } from "@lucide/svelte";
    import { Input } from "$lib/components/ui/input/index";
    import Spinner from "./spinner.svelte";
    import { fade } from "svelte/transition";

    let { value = $bindable(), searching, ...restProps } = $props();
</script>

<div class="relative flex-1 w-full md:max-w-sm">
    <SearchIcon
        class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
    />
    <Input
        bind:value
        disabled={false}
        class={`pl-10 ${false ? "pr-10 text-muted-foreground" : ""}`}
        {...restProps}
    />
    {#if searching}
        <div transition:fade>
            <Spinner />
        </div>
    {/if}
</div>
