import type { Route } from "@dvcol/svelte-simple-router";
import type { SidebarItem } from "./lib/components/sidebar.svelte";
import { homeRoutes, homeSidebarItems, Routes as HomeRoutes } from "./features/home/index"
import { productsRoutes, productsSidebarItems } from "./features/products/index"

export const routes = <Route[]>[
    {
        path: '/',
        redirect: {
            path: HomeRoutes.HOME.path,
        },
    },
    ...homeRoutes,
    ...productsRoutes,
];

export const sidebarItems = <SidebarItem[]>[
    ...homeSidebarItems,
    ...productsSidebarItems,
];