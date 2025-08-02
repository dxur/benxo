<script>
    let { children } = $props();
</script>

<div
    class="flex min-h-10 min-w-10 items-center justify-center rounded-lg bg-primary/10"
>
    {@render children?.()}
</div>
