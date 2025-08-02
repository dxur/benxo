<script>
    let { children } = $props();
</script>

<div
    class="flex flex-col md:flex-row items-center justify-between gap-4 w-full md:w-auto [&>*]:w-full md:[&>*]:w-auto"
>
    {@render children?.()}
</div>
