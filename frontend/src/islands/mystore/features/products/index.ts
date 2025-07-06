import PackageIcon from "@lucide/svelte/icons/package";
import type { Route } from "../../routes";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "products",
        path: "/products",
        component: () => import("./ProductListPage.svelte"),
    };
    export const CREATE_PAGE: Route = {
        name: "product-create",
        path: "/products/create",
        component: () => import("./ProductCreatePage.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "product-edit",
        path: "/products/:id",
        component: () => import("./ProductEditPage.svelte"),
    };
}

export const routes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE
];

export const sidebar = {
    title: "Products & Inventory",
    path: Routes.LIST_PAGE.path,
    icon: PackageIcon,
    items: [
        {
            title: "All Products",
            path: Routes.LIST_PAGE.path,
        },
    ],
};