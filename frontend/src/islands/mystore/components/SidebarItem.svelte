<script lang="ts">
  import { active, link, type Route } from "@dvcol/svelte-simple-router";
  import { _ } from "svelte-i18n";

  export let route: Route;
</script>

<a href={route.path} use:link use:active>
  {$_(`routes.${route.name as string}.sidebar`)}
</a>
