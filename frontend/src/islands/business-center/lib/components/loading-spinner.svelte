<script>
    let { text } = $props();
</script>

<div class="flex items-center justify-center">
    <div class="text-center">
        <div
            class="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent mx-auto mb-4"
        ></div>
        <p class="text-muted-foreground">{text || "Loading..."}</p>
    </div>
</div>
