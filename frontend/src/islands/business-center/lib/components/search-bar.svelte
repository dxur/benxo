<script>
    import { SearchIcon } from "@lucide/svelte";
    import { Input } from "$lib/components/ui/input/index";
    import Spinner from "./spinner.svelte";

    let { ...restProps } = $props();
</script>

<div class="relative flex-1 w-full md:max-w-sm">
    <SearchIcon
        class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
    />
    <Input
        placeholder="Search products..."
        disabled={false}
        class={`pl-10 ${false ? "pr-10 text-muted-foreground" : ""}`}
        {...restProps}
    />
    {#if false}
        <Spinner />
    {/if}
</div>
