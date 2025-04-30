use leptos::prelude::*;

use crate::pages::*;
use crate::paths::*;
pub struct Route {
    pub path: &'static str,
    pub component: fn() -> AnyView,
}

pub struct MenuItem {
    pub name: &'static str,
    pub path: &'static str,
    pub subitems: &'static [MenuItem],
}

pub const ROUTES: &[Route] = &[
    Route {
        path: HOME_PATH,
        component: Home,
    },
    Route {
        path: PRODUCTS_PATH,
        component: Products,
    },
    Route {
        path: ORDER_PLACEMENT_PATH,
        component: OrderPlacement,
    },
    Route {
        path: USERS_PATH,
        component: Users,
    },
    Route {
        path: SETTINGS_PATH,
        component: Settings,
    },
];

pub const MENU: &[MenuItem] = &[
    MenuItem {
        name: "Home",
        path: HOME_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Orders",
        path: "",
        subitems: &[
            MenuItem {
                name: "Order Placement",
                path: ORDER_PLACEMENT_PATH,
                subitems: &[],
            },
            MenuItem {
                name: "Tracking",
                path: TRACKING_PATH,
                subitems: &[],
            },
        ],
    },
    MenuItem {
        name: "Products",
        path: PRODUCTS_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Delivery",
        path: DELIVERY_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Sale channels",
        path: CHANNELS_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Team",
        path: USERS_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Settings",
        path: SETTINGS_PATH,
        subitems: &[],
    },
];
