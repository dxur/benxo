import type { AppRoute } from "../..";
import type { SidebarItem } from "../../lib/components/sidebar.svelte";
import { Package, TrendingUp } from "@lucide/svelte";

export namespace Routes {
    export const LIST_PAGE: AppRoute = {
        name: "Products",
        path: "/products",
        component: () => import("./products-list.svelte"),
    };
    export const EDIT_PAGE: AppRoute = {
        name: "Edit product",
        path: "/products/edit/:id",
        meta: { parent: LIST_PAGE },
        component: () => import("./product-edit.svelte"),
    };
    export const STOCKS_PAGE: AppRoute = {
        name: "Stocks",
        path: "/stocks",
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
    {
        id: "stocks",
        name: "Stocks",
        path: Routes.STOCKS_PAGE.path,
        icon: TrendingUp
    },
];
