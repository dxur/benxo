import type { AppRoute } from "../..";
import type { SidebarItem } from "../../lib/components/sidebar.svelte";
import { PackageIcon } from "@lucide/svelte";

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
    export const CREATE_PAGE: AppRoute = {
        name: "Create product",
        path: "/products/create",
        meta: { parent: LIST_PAGE },
        component: () => import("./product-create.svelte"),
    }
}

export const productsRoutes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
    Routes.CREATE_PAGE,
];

export const productsSidebarItems = <SidebarItem[]>[
    {
        id: "products",
        name: "Products",
        path: Routes.LIST_PAGE.path,
        icon: PackageIcon
    },
];