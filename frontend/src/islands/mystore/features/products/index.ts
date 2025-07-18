import PackageIcon from "@lucide/svelte/icons/package";
import type { Route } from "../../routes";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "products",
        path: "/products/list",
        component: () => import("./ProductListPage.svelte"),
    };
    export const CREATE_PAGE: Route = {
        name: "product-create",
        path: "/products/create",
        component: () => import("./ProductCreatePage.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "product-edit",
        path: "/products/edit/:id",
        component: () => import("./ProductEditPage.svelte"),
    };
    export const STOCKS_PAGE: Route = {
        name: "stocks",
        path: "/products/stocks",
        component: () => import("./StocksPage.svelte"),
    }
}

export const routes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
    Routes.CREATE_PAGE,
    Routes.STOCKS_PAGE,
];

export const sidebar = {
    title: "Products & Inventory",
    path: Routes.LIST_PAGE.path,
    icon: PackageIcon,
    items: [
        {
            title: "Products List",
            path: Routes.LIST_PAGE.path,
        },
        {
            title: "Stocks Management",
            path: Routes.STOCKS_PAGE.path,
        },
    ],
};