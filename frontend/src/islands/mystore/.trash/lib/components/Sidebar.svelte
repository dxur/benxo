<script lang="ts">
  import Item from "./SidebarItem.svelte";
  import { AppRoutes } from "../routes";
  import { _ } from "svelte-i18n";
</script>

<aside>
  <h1>{$_("common.dashboard")}</h1>
  <nav>
    <Item route={AppRoutes.HOME} />
    <Item route={AppRoutes.ORDERS} />
    <Item route={AppRoutes.PRODUCTS} />
    <Item route={AppRoutes.CATEGORIES} />
    <Item route={AppRoutes.STORES} />
    <Item route={AppRoutes.CONVERSIONS} />
    <Item route={AppRoutes.SETTINGS} />
    <Item route={{ name: "help", path: "/docs" }} target="_blank" />
  </nav>
</aside>

<style>
  aside {
    background-color: var(--bg-sidebar);
    width: var(--sidebar-width);
    height: 100vh;
    overflow-y: hidden;
    transition: width var(--transition-speed) ease;
    box-shadow: var(--shadow);
    display: flex;
    flex-direction: column;
  }

  aside h1 {
    padding: var(--spacing-xl) var(--spacing-base);
    font-size: var(--font-size-xl);
    color: white;
    font-weight: var(--font-weight-bold);
    letter-spacing: -0.01em;
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
  }

  aside h1::before {
    content: "";
    display: inline-block;
    width: 24px;
    height: 24px;
    background-color: var(--accent);
    border-radius: var(--radius-small);
  }

  aside nav {
    overflow-y: auto;
    overflow-x: hidden;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    padding: var(--spacing-md) var(--spacing-md) var(--spacing-xl);
    margin-top: var(--spacing-base);
  }

  aside nav a {
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

  aside nav a:hover {
    background-color: rgba(255, 255, 255, 0.08);
    color: white;
  }

  aside nav a:hover::before {
    opacity: 1;
  }

  aside nav a[data-active] {
    background-color: var(--accent);
    color: white;
  }

  aside nav a[data-active]::before {
    opacity: 1;
  }
</style>
