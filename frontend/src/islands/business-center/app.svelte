<script lang="ts">
    import {
        RouterContext,
        RouterView,
    } from "@dvcol/svelte-simple-router/components";
    import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
    import Sidebar from "./lib/components/sidebar.svelte";
    import Breadcrumb from "./lib/components/breadcrumb.svelte";
    import { routes, sidebarItems } from "./index";
    import type { RouterOptions } from "@dvcol/svelte-simple-router";
    import AppLayout from "./lib/components/layout/app-layout.svelte";

    const options: RouterOptions = {
        routes,
        hash: true,
        priority: () => {
            return 0;
        },
    };

    const client = new QueryClient();
</script>

<QueryClientProvider {client}>
    <RouterContext {options}>
        <AppLayout breadcrumb={Breadcrumb}>
            {#snippet sidebar()}
                <Sidebar items={sidebarItems} />
            {/snippet}
            <main>
                <RouterView />
            </main>
        </AppLayout>
    </RouterContext>
</QueryClientProvider>
