import type { AppRoute } from "../..";
import type { SidebarItem } from "../../lib/components/sidebar.svelte";
import { ShoppingBagIcon } from "@lucide/svelte";

export namespace Routes {
    export const LIST_PAGE: AppRoute = {
        name: "Stores",
        path: "/stores",
        component: () => import("./stores-list.svelte"),
    };
    export const EDIT_PAGE: AppRoute = {
        name: "Edit store",
        path: "/stores/edit/:id",
        meta: { parent: LIST_PAGE },
        component: () => import("./store-edit.svelte"),
    };
}

export const storesRoutes = [
    Routes.LIST_PAGE,
    Routes.EDIT_PAGE,
];

export const storesSidebarItems = <SidebarItem[]>[
    {
        id: "stores",
        name: "Stores",
        path: Routes.LIST_PAGE.path,
        icon: ShoppingBagIcon
    },
];
