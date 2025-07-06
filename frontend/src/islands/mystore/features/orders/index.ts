import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
import type { Route } from "../../routes";

export namespace Routes {
    export const LIST_PAGE: Route = {
        name: "orders",
        path: "/orders",
        component: () => import("./OrderListPage.svelte"),
    };
    export const CREATE_PAGE: Route = {
        name: "order-create",
        path: "/orders/create",
        component: () => import("./OrderCreatePage.svelte"),
    };
    export const EDIT_PAGE: Route = {
        name: "order-edit",
        path: "/orders/:id",
        component: () => import("./OrderEditPage.svelte"),
    };
}

export const routes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
];

export const sidebar = {
    title: "Orders & Fulfillment",
    path: "/orders",
    icon: ShoppingCartIcon,
    items: [
        {
            title: "All Orders",
            path: Routes.LIST_PAGE.path,
        },
        { title: "Orders by Store", path: "/orders/by-store" },
        { title: "Processing Queue", path: "/orders/processing" },
        { title: "Fulfillment Center", path: "/orders/fulfillment" },
        { title: "Returns & Refunds", path: "/orders/returns" },
        { title: "Abandoned Carts", path: "/orders/abandoned" },
        { title: "Test Orders", path: "/orders/testing" },
    ],
};