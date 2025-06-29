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
    <Item route={AppRoutes.STORES} />
    <Item route={AppRoutes.CONVERSIONS} />
    <Item route={AppRoutes.SETTINGS} />
    <a href="/docs" target="_blank"> {$_("help")} </a>
  </nav>
</aside>
