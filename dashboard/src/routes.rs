use leptos::prelude::*;

use crate::pages::{Home, Products, Settings, Users};

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
        name: "Products",
        path: PRODUCTS_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Users",
        path: USERS_PATH,
        subitems: &[],
    },
    MenuItem {
        name: "Settings",
        path: SETTINGS_PATH,
        subitems: &[],
    },
];
