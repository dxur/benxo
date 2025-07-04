<script lang="ts">
  import { active, link, type Route } from "@dvcol/svelte-simple-router";
  import { _ } from "svelte-i18n";

  export let route: {
    name: string;
    path: string;
  };
  export let target: string = "";
</script>

<a href={route.path} {target} data-active use:link use:active>
  {$_(`routes.${route.name as string}.sidebar`)}
</a>

<style>
  a {
    cursor: pointer;
    background: transparent;
    color: rgba(255, 255, 255, 0.75);
    padding: var(--spacing-sm) var(--spacing-base);
    border-radius: var(--radius-small);
    transition: var(--transition-base);
    width: 100%;
    font-weight: var(--font-weight-medium);
    font-size: var(--font-size-sm);
    align-items: center;
    gap: var(--spacing-sm);
    text-decoration: none;
  }

  a:hover {
    background-color: rgba(255, 255, 255, 0.08);
    color: white;
  }

  a:hover::before {
    opacity: 1;
  }

  a[data-active] {
    background-color: var(--accent);
    color: white;
  }

  a[data-active]::before {
    opacity: 1;
  }
</style>
