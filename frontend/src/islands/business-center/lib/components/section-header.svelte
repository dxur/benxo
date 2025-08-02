<script>
    import IconContainer from "./icon-container.svelte";
    let { icon, title, description } = $props();
</script>

<div class="flex items-center gap-3">
    {#if icon}
        {@const Icon = icon}
        <IconContainer>
            <Icon />
        </IconContainer>
    {/if}
    <div>
        <h1 class="text-3xl font-bold tracking-tight">{title}</h1>
        <span class="text-muted-foreground">
            {description}
        </span>
    </div>
</div>
