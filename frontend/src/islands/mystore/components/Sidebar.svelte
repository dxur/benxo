<script lang="ts">
  import Item from "./SidebarItem.svelte";
  import { AppRoutes } from "../routes";
</script>

<aside>
  <h1>Dashboard</h1>
  <nav>
    <Item route={AppRoutes.HOME} />
    <Item route={AppRoutes.ORDERS} />
    <Item route={AppRoutes.PRODUCTS} />
    <Item route={AppRoutes.CHANNELS} />
    <Item route={AppRoutes.SETTINGS} />
    <a href="/docs" target="_blank"> Help </a>
  </nav>
</aside>
