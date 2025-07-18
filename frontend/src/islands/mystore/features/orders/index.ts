import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
import type { Route } from "../../routes";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "orders",
        path: "/orders/list",
        component: () => import("./OrderListPage.svelte"),
    };
    export const CREATE_PAGE: Route = {
        name: "order-create",
        path: "/orders/create",
        component: () => import("./OrderCreatePage.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "order-edit",
        path: "/orders/edit/:id",
        component: () => import("./OrderEditPage.svelte"),
    };
    export const FULFILLMENT_PAGE: Route = {
        name: "fulfillment",
        path: "/orders/fulfillment",
        component: () => import("./OrderFulfilmentCenter.svelte"),
    };
    // export const ABANDONED_CARTS_PAGE: Route = {
    //     name: "abandoned-carts",
    //     path: "/orders/abandoned",
    //     component: () => import("./AbandonedCarts.svelte"),
    // };

}

export const routes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
    Routes.CREATE_PAGE,
    Routes.FULFILLMENT_PAGE,
    // Routes.ABANDONED_CARTS_PAGE,
];

export const sidebar = {
    title: "Orders & Fulfillment",
    path: "/orders",
    icon: ShoppingCartIcon,
    items: [
        {
            title: "Orders List",
            path: Routes.LIST_PAGE.path,
        },
        // { title: "Processing Queue", path: "/orders/processing" },
        { title: "Fulfillment Center", path: Routes.FULFILLMENT_PAGE.path },
        // { title: "Abandoned Carts", path: Routes.ABANDONED_CARTS_PAGE.path },
    ],
};