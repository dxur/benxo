import type { Route } from "@dvcol/svelte-simple-router";
import type { SidebarItem } from "../../lib/components/sidebar.svelte";
import { Home } from "@lucide/svelte";

export namespace Routes {
    export const HOME: Route = {
        name: "home",
        path: "/home",
        component: () => import("./home.svelte"),
    };
}

export const homeRoutes = [
    Routes.HOME,
];

export const homeSidebarItems = <SidebarItem[]>[
    { id: "home", name: "Home", path: Routes.HOME.path, icon: Home },
]
