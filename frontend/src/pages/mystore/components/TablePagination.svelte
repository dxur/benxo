<script lang="ts">
  import { type Writable } from "svelte/store";

  export let page: Writable<number>;
  export let total: Writable<number>;
</script>

<nav>
  <button
    disabled={$page === 1}
    on:click={() => page.update((n) => Math.max(n - 1, 1))}
  >
    Previous
  </button>
  <span>{"Page: "}{$page}</span>
  <button
    disabled={$page >= $total}
    on:click={() => page.update((n) => Math.min(n + 1, $total))}
  >
    Next
  </button>
</nav>
