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
  export const STORE: Route = {
    name: "Store",
    path: "/store",
    component: () => import("./pages/store/index.svelte"),
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
  AppRoutes.STORE,
  // AppRoutes.DELIVERY,
  // AppRoutes.CHANNELS,
  // AppRoutes.USERS,
  AppRoutes.SETTINGS
];