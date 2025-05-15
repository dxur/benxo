<script lang="ts">
    export let dialog;
</script>

<dialog bind:this={dialog}>
    <section data-dialog>
        <slot />
    </section>
</dialog>
