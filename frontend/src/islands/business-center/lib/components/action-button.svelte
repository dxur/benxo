<script>
    import { Button } from "$lib/components/ui/button/index";

    let { icon, children, ...restProps } = $props();
</script>

<Button class="gap-2" {...restProps}>
    {#if icon}
        {@const Icon = icon}
        <Icon class="h-4 w-4" />
    {/if}
    {@render children?.()}
</Button>
