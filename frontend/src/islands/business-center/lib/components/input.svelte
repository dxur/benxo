<script lang="ts">
    import { Input } from "$lib/components/ui/input/index";

    let { errors, ...restProps } = $props();
</script>

<div>
    <Input aria-invalid={!!errors} {...restProps} />
    {#each errors || [] as error}
        <div class="error-message">
            <p>{error}</p>
        </div>
    {/each}
</div>
