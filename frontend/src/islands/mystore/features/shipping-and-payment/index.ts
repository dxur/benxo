import TruckIcon from "@lucide/svelte/icons/truck";
import type { Route } from "../../routes";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "shipping-profiles",
        path: "/shipping/list",
        component: () => import("./ShippingProfilesPage.svelte"),
    };
    export const CREATE_PAGE: Route = {
        name: "shipping-create",
        path: "/shipping/create",
        component: () => import("./ShippingProfilesPage.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "shipping-edit",
        path: "/shipping/edit/:id",
        component: () => import("./ShippingProfilesPage.svelte"),
    };
    export const STOCKS_PAGE: Route = {
        name: "payments",
        path: "/shipping/stocks",
        component: () => import("./ShippingProfilesPage.svelte"),
    }
}

export const routes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
    Routes.CREATE_PAGE,
    Routes.STOCKS_PAGE,
];

export const sidebar = {
    title: "Shipping & Payment",
    path: Routes.LIST_PAGE.path,
    icon: TruckIcon,
    items: [
        {
            title: "Products",
            path: Routes.LIST_PAGE.path,
        },
        {
            title: "Stocks",
            path: Routes.STOCKS_PAGE.path,
        },
    ],
};