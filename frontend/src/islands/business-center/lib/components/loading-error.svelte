<script lang="ts">
    let {
        error = "Something went wrong!",
        message = "Please try again later, or contact support if the issue persists",
        children = undefined,
    } = $props();
</script>

<div class="flex flex-col items-center justify-center py-8">
    <div class="text-center">
        <h3 class="text-lg font-semibold">
            {error}
        </h3>
        <p class="text-muted-foreground mt-4 text-wrap">
            {message}
        </p>
    </div>
    <div class="mt-4">{@render children?.()}</div>
</div>
