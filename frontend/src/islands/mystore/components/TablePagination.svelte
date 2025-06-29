<script lang="ts">
  import { type Writable } from "svelte/store";
  import { _ } from "svelte-i18n";

  export let page: Writable<number>;
  export let total: Writable<number>;
</script>

<nav>
  <button
    disabled={$page === 1}
    on:click={() => page.update((n) => Math.max(n - 1, 1))}
  >
    {$_("common.navigation.previous")}
  </button>
  <span>{$_("common.navigation.page")}: {$page}</span>
  <button
    disabled={$page >= $total}
    on:click={() => page.update((n) => Math.min(n + 1, $total))}
  >
    {$_("common.navigation.next")}
  </button>
</nav>
