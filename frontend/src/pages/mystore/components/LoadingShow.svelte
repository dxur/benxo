<script lang="ts" context="module">
  export type LoadingStatus = undefined | null | any;
</script>

<script lang="ts">
  export let status: LoadingStatus;
</script>

{#if status}
  <span> Error: {status} </span>
{:else if status === undefined}
  <span> Loading </span>
{:else}
  <slot />
{/if}
