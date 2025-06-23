<script lang="ts">
  import type { RouterOptions } from "@dvcol/svelte-simple-router/models";
  import {
    RouterContext,
    RouterView,
  } from "@dvcol/svelte-simple-router/components";
  import routes from "./routes";
  import Sidebar from "./components/Sidebar.svelte";
  import Notifications from "@/components/Notifications.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";

  function logout() {
    ApiRoutes.logout().then((_) => {
      location.replace("/login");
    });
  }

  const options: RouterOptions = {
    routes,
    hash: true,
    priority: (a, b) => {
      return 0;
    },
  } as const;
</script>

<RouterContext {options}>
  <Sidebar />
  <main>
    <header>
      <input type="text" placeholder="Search" />
      <a href={undefined} on:click={() => logout()}>Logout</a>
    </header>

    <div data-page>
      <Notifications />
      <section data-page>
        <RouterView />
      </section>
    </div>
  </main>
</RouterContext>

<style>
  .account-menu {
    position: absolute;
    right: 0;
    top: 100%;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    display: none;
  }
  .account-menu.show {
    display: block;
  }
</style>
