import { routes as ProductRoutes } from "./features/products/index"
import { routes as OrderRoutes } from "./features/orders/index"
import { routes as StoreRoutes } from "./features/online-stores/index"
import { routes as ShippingRoutes } from "./features/shipping-and-payment/index"

export type Route = {
  name: string;
  path: string;
  props?: { [key: string]: any };
  component: any;
}

export namespace AppRoutes {

  export const HOME: Route = {
    name: "home",
    path: "/home",
    component: () => import("./features/analytics/AnalyticsHomePage.svelte"),
  };
  // export const ORDERS: Route = {
  //   name: "orders",
  //   path: "/orders",
  //   component: () => import("./pages/orders/index.svelte"),
  // };
  // export const ORDER: Route = {
  //   name: "edit-order",
  //   path: "/orders/:oid",
  //   component: () => import("./features/orders/OrderEditPage.svelte"),
  // };
  // export const PRODUCTS: Route = {
  //   name: "products",
  //   path: "/products",
  //   component: () => import("./pages/products/index.svelte"),
  // };
  // export const PRODUCT: Route = {
  //   name: "edit-product",
  //   path: "/products/:oid",
  //   component: () => import("./features/products/ProductListPage.svelte"),
  // };
  // export const CATEGORIES: Route = {
  //   name: "categories",
  //   path: "/categories",
  //   component: () => import("./features/categories/CategoriesPage.svelte"),
  // };
  // export const STORES: Route = {
  //   name: "stores",
  //   path: "/stores",
  //   component: () => import("./features/shops/ShopListPage.svelte"),
  // };
  // export const STORE: Route = {
  //   name: "edit-store",
  //   path: "/stores/:oid",
  //   component: () => import("./features/shops/ShopEditPage.svelte"),
  // };
  // export const CONVERSIONS: Route = {
  //   name: "conversions",
  //   path: "/conversions",
  //   component: () => import("./features/shops/ShopListPage.svelte"),
  // };
  // export const CONVERSION: Route = {
  //   name: "edit-conversion",
  //   path: "/conversions/:oid",
  //   component: () => import("./features/shops/ShopEditPage.svelte"),
  // };
  // export const SETTINGS: Route = {
  //   name: "settings",
  //   path: "/settings",
  //   component: () => import("./features/settings/index.svelte"),
  // };

}

export default <Route[]>[
  {
    path: '/',
    redirect: {
      path: AppRoutes.HOME.path,
    },
  },
  AppRoutes.HOME,
  ...ProductRoutes,
  ...OrderRoutes,
  ...StoreRoutes,
  ...ShippingRoutes,
  // AppRoutes.ORDERS,
  // AppRoutes.ORDER,
  // AppRoutes.PRODUCTS,
  // AppRoutes.PRODUCT,
  // AppRoutes.CATEGORIES,
  // AppRoutes.STORE,
  // AppRoutes.DELIVERY,
  // AppRoutes.USERS,
  // AppRoutes.STORES,
  // AppRoutes.STORE,
  // AppRoutes.CONVERSIONS,
  // AppRoutes.CONVERSION,
  // AppRoutes.SETTINGS
];
