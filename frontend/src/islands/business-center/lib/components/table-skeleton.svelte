<script>
    import { Skeleton } from "$lib/components/ui/skeleton/index";
</script>

<div class="space-y-4">
    <div class="flex items-center justify-between gap-4">
        <Skeleton class="h-10 w-64" />
        <Skeleton class="h-10 w-24" />
    </div>
    <div class="rounded-md border">
        <div class="p-4">
            {#each Array(10) as _}
                <div class="flex items-center space-x-4 py-3">
                    <Skeleton class="h-4 w-20" />
                    <Skeleton class="h-4 w-32" />
                    <Skeleton class="h-4 w-full" />
                    <Skeleton class="h-4 w-24" />
                    <Skeleton class="h-4 w-full" />
                </div>
            {/each}
        </div>
    </div>
</div>
