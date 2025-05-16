<script lang="ts">
  import { active, link } from "@dvcol/svelte-simple-router";

  export let path: string;
  export let name: string;
</script>

<a href={path} use:link use:active> {name} </a>
