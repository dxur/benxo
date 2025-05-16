<script lang="ts">
  import Sidebar from "@components/Sidebar.svelte";
  import type { RouterOptions } from "@dvcol/svelte-simple-router/models";
  import {
    RouterContext,
    RouterView,
  } from "@dvcol/svelte-simple-router/components";
  import routes from "@/routes";
  import Notifications from "@components/Notifications.svelte";

  const options: RouterOptions = {
    routes,
  } as const;
</script>

<RouterContext {options}>
  <Sidebar />
  <main>
    <header>
      <input type="text" placeholder="Search" />
    </header>
    <div data-page>
      <Notifications />
      <section data-page>
        <RouterView />
      </section>
    </div>
  </main>
</RouterContext>
