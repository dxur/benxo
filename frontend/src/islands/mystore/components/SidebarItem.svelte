<script lang="ts">
  import { active, link, type Route } from "@dvcol/svelte-simple-router";

  export let route: Route;
</script>

<a href={route.path} use:link use:active> {route.name} </a>
