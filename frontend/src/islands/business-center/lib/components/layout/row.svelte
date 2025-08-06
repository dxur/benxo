<script>
    let { children } = $props();
</script>

<div class="flex flex-row gap-4">
    {@render children?.()}
</div>
