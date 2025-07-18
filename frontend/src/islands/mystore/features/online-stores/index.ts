import PackageIcon from "@lucide/svelte/icons/package";
import type { Route } from "../../routes";
import { StoreIcon } from "@lucide/svelte";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "stores",
        path: "/stores",
        component: () => import("./StoreListPage.svelte"),
    };
    export const CREATE_PAGE: Route = {
        name: "store-create",
        path: "/stores/create",
        component: () => import("./StoreCreatePage.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "store-edit",
        path: "/stores/edit/:id",
        component: () => import("./StoreEditPage.svelte"),
    };
    export const CONVERIONS_PAGE: Route = {
        name: "store-conversions",
        path: "/stores/conversions",
        component: () => import("./StoreListPage.svelte"),
    };
}

export const routes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
    Routes.CREATE_PAGE,
    Routes.CONVERIONS_PAGE,
];

export const sidebar =
{
    title: "Stores & Conversions",
    path: "/stores",
    icon: StoreIcon,
    items: [
        {
            title: "Stores",
            path: Routes.LIST_PAGE.path,
        },
        {
            title: "Store Templates",
            path: "/stores/templates",
        },
        {
            title: "Regional Management",
            path: "/stores/regional",
        },
        {
            title: "Store Performance",
            path: "/stores/analytics",
        },
        {
            title: "Conversions",
            path: Routes.CONVERIONS_PAGE.path,
        }

    ],
};