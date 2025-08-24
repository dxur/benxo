import type { AppRoute } from "../..";
import type { SidebarItem } from "../../lib/components/sidebar.svelte";
import { ShoppingCartIcon } from "@lucide/svelte";

export namespace Routes {
    export const LIST_PAGE: AppRoute = {
        name: "Orders",
        path: "/orders",
        component: () => import("./orders-list.svelte"),
    };
    export const DETAIL_PAGE: AppRoute = {
        name: "Order Details",
        path: "/orders/:id",
        meta: { parent: LIST_PAGE },
        component: () => import("./order-detail.svelte"),
    };
    export const CREATE_PAGE: AppRoute = {
        name: "Create Order",
        path: "/orders/create",
        meta: { parent: LIST_PAGE },
        component: () => import("./order-create.svelte"),
    };
    export const ANALYTICS_PAGE: AppRoute = {
        name: "Order Analytics",
        path: "/orders/analytics",
        meta: { parent: LIST_PAGE },
        component: () => import("./order-analytics.svelte"),
    };
}

export const ordersRoutes = [
    Routes.LIST_PAGE,
    Routes.DETAIL_PAGE,
    Routes.CREATE_PAGE,
    Routes.ANALYTICS_PAGE,
];

export const ordersSidebarItems = <SidebarItem[]>[
    {
        id: "orders",
        name: "Orders",
        path: Routes.LIST_PAGE.path,
        icon: ShoppingCartIcon
    },
];