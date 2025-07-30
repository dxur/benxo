import type { Route } from "@dvcol/svelte-simple-router";
import type { SidebarItem } from "../../lib/components/sidebar.svelte";
import { Package } from "@lucide/svelte";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "products",
        path: "/products/list",
        component: () => import("./products-list.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "product-edit",
        path: "/products/edit/:id",
        component: () => import("./product-edit.svelte"),
    };
    export const STOCKS_PAGE: Route = {
        name: "stocks",
        path: "/products/stocks",
        component: () => import("./stocks.svelte"),
    }
}

export const productsRoutes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
    Routes.STOCKS_PAGE,
];

export const productsSidebarItems = <SidebarItem[]>[
    {
        id: "products",
        name: "Products",
        path: Routes.LIST_PAGE.path,
        icon: Package
    },
];
