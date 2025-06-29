import type { Route } from '@dvcol/svelte-simple-router/models';
import Home from "./pages/index.svelte"

export namespace AppRoutes {
  type Route = {
    name: string;
    path: string;
    component: any;
  }

  export const HOME: Route = {
    name: "home",
    path: "/home",
    component: () => import("./pages/index.svelte"),
  };
  export const ORDERS: Route = {
    name: "orders",
    path: "/orders",
    component: () => import("./pages/orders/index.svelte"),
  };
  export const ORDER: Route = {
    name: "edit-order",
    path: "/orders/:oid",
    component: () => import("./pages/orders/edit.svelte"),
  };
  export const PRODUCTS: Route = {
    name: "products",
    path: "/products",
    component: () => import("./pages/products/index.svelte"),
  };
  export const PRODUCT: Route = {
    name: "edit-product",
    path: "/products/:oid",
    component: () => import("./pages/products/edit.svelte"),
  };
  export const STORES: Route = {
    name: "stores",
    path: "/stores",
    component: () => import("./pages/stores/index.svelte"),
  };
  export const STORE: Route = {
    name: "edit-store",
    path: "/stores/:oid",
    component: () => import("./pages/stores/edit.svelte"),
  };
  export const CONVERSIONS: Route = {
    name: "conversions",
    path: "/conversions",
    component: () => import("./pages/stores/index.svelte"),
  };
  export const CONVERSION: Route = {
    name: "edit-conversion",
    path: "/conversions/:oid",
    component: () => import("./pages/stores/edit.svelte"),
  };
  export const SETTINGS: Route = {
    name: "settings",
    path: "/settings",
    component: () => import("./pages/settings/index.svelte"),
  };

}

export default <Route[]>[
  {
    path: '/',
    redirect: {
      path: AppRoutes.HOME.path,
    },
  },
  AppRoutes.HOME,
  AppRoutes.ORDERS,
  AppRoutes.ORDER,
  AppRoutes.PRODUCTS,
  AppRoutes.PRODUCT,
  // AppRoutes.STORE,
  // AppRoutes.DELIVERY,
  // AppRoutes.USERS,
  AppRoutes.STORES,
  AppRoutes.STORE,
  AppRoutes.CONVERSIONS,
  AppRoutes.CONVERSION,
  AppRoutes.SETTINGS
];
