import type { Route } from '@dvcol/svelte-simple-router/models';
import Home from "./pages/index.svelte"

export namespace AppRoutes {
  type Route = {
    name: string;
    path: string;
    component: any;
  }

  export const HOME: Route = {
    name: "Home",
    path: "/home",
    component: () => import("./pages/index.svelte"),
  };
  export const ORDERS: Route = {
    name: "Orders",
    path: "/orders",
    component: () => import("./pages/orders/index.svelte"),
  };
  export const ORDER: Route = {
    name: "Order Edit",
    path: "/orders/:oid",
    component: () => import("./pages/orders/edit.svelte"),
  };
  export const PRODUCTS: Route = {
    name: "Products",
    path: "/products",
    component: () => import("./pages/products/index.svelte"),
  };
  export const PRODUCT: Route = {
    name: "Product Edit",
    path: "/products/:oid",
    component: () => import("./pages/products/edit.svelte"),
  };
  export const CHANNELS: Route = {
    name: "Channels",
    path: "/channels",
    component: () => import("./pages/channels/index.svelte"),
  };
  export const CHANNEL_STORE: Route = {
    name: "Store",
    path: "/channels/store/:oid",
    component: () => import("./pages/channels/store.svelte"),
  };
  export const CHANNEL_API: Route = {
    name: "API",
    path: "/channels/api/:oid",
    component: () => import("./pages/channels/api.svelte"),
  };
  export const SETTINGS: Route = {
    name: "Settings",
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
  AppRoutes.CHANNELS,
  AppRoutes.CHANNEL_STORE,
  AppRoutes.CHANNEL_API,
  AppRoutes.SETTINGS
];