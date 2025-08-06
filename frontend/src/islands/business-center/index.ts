import type { Route } from "@dvcol/svelte-simple-router";
import type { SidebarItem } from "./lib/components/sidebar.svelte";
import { homeRoutes, homeSidebarItems, Routes as HomeRoutes } from "./features/home/index"
import { storesRoutes, storesSidebarItems } from "./features/stores/index"
import { productsRoutes, productsSidebarItems } from "./features/products/index"
import type { Snippet } from "svelte";
import type { ComponentOrLazy } from "@dvcol/svelte-utils/component";

export type AppRoute = Route & {
    name: string;
    path: string;
    redirect?: { path: string };
    component?: ComponentOrLazy | Snippet;
    meta?: { parent?: AppRoute };
};

export const routes = <AppRoute[]>[
    {
        path: '/',
        redirect: {
            path: HomeRoutes.HOME.path,
        },
    },
    ...homeRoutes,
    ...productsRoutes,
    ...storesRoutes,
];

export const sidebarItems = <SidebarItem[]>[
    ...homeSidebarItems,
    ...productsSidebarItems,
    ...storesSidebarItems,
];