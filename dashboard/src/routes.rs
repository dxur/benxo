use leptos::prelude::*;

use crate::pages::*;

pub struct Route {
    pub path: &'static str,
    pub component: fn() -> AnyView,
}

pub struct MenuItem {
    pub name: &'static str,
    pub path: &'static str,
    pub subitems: &'static [MenuItem],
}

pub const HOME_PATH: &'static str = "/";
pub const PRODUCTS_PATH: &'static str = "/products";
pub const CONFIRMATION_PATH: &'static str = "/confirmation";
pub const TRACKING_PATH: &'static str = "/tracking";
pub const DELIVERY_PATH: &'static str = "/delivery";
pub const CHANNELS_PATH: &'static str = "/channels";
pub const USERS_PATH: &'static str = "/users";
pub const SETTINGS_PATH: &'static str = "/settings";

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
        path: CONFIRMATION_PATH,
        component: Confirmation,
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
                name: "Confirmation",
                path: CONFIRMATION_PATH,
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
        name: "Stock",
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
