<script lang="ts">
    import {
        RouterContext,
        RouterView,
    } from "@dvcol/svelte-simple-router/components";
    import Sidebar from "./lib/components/sidebar.svelte";
    import { routes, sidebarItems } from "./index";
    import type { RouterOptions } from "@dvcol/svelte-simple-router";
    import AppLayout from "./lib/components/app-layout.svelte";

    const options: RouterOptions = {
        routes,
        hash: true,
        priority: () => {
            return 0;
        },
    };
</script>

<RouterContext {options}>
    <AppLayout>
        {#snippet sidebar()}
            <Sidebar items={sidebarItems} />
        {/snippet}
        <main>
            <RouterView />
        </main>
    </AppLayout>
</RouterContext>
