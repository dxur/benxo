<script lang="ts">
  import Item from "./SidebarItem.svelte";
  import { AppRoutes } from "../routes";
</script>

<aside>
  <h1>Dashboard</h1>
  <nav>
    <Item path={AppRoutes.HOME.path} name={AppRoutes.HOME.name} />
    <Item path={AppRoutes.ORDERS.path} name={AppRoutes.ORDERS.name} />
    <Item path={AppRoutes.PRODUCTS.path} name={AppRoutes.PRODUCTS.name} />
    <Item path={AppRoutes.STORE.path} name={AppRoutes.STORE.name} />
    <Item path={AppRoutes.STORE.path} name="Conversion" />
    <!-- <Item path={AppRoutes.DELIVERY.path} name={AppRoutes.DELIVERY.name} /> -->
    <!-- <Item path={AppRoutes.CHANNELS.path} name={AppRoutes.CHANNELS.name} /> -->
    <!-- <Item path={AppRoutes.USERS.path} name={AppRoutes.USERS.name} /> -->
    <Item path={AppRoutes.SETTINGS.path} name={AppRoutes.SETTINGS.name} />
  </nav>
</aside>
