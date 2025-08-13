<script>
    import IconContainer from "./icon-container.svelte";
    let { icon, title, description, children = undefined } = $props();
</script>

<div class="flex items-center gap-3">
    {#if icon}
        {@const Icon = icon}
        <IconContainer>
            <Icon />
        </IconContainer>
    {/if}
    <div>
        <div class="flex flex-row gap-2 items-center">
            <h1 class="text-3xl font-bold tracking-tight">
                {title}
            </h1>
            {@render children?.()}
        </div>
        <span class="text-muted-foreground">
            {description}
        </span>
    </div>
</div>
