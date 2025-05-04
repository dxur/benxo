use std::cell::OnceCell;
use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, path, NavigateOptions};
use macros::routes_builder;

use crate::pages::*;

pub type Route = (&'static str, Page);

pub trait RouteExt {
    fn path(&self) -> &'static str;
    fn page(&self) -> &Page;
}

impl RouteExt for Route {
    fn path(&self) -> &'static str {
        self.0
    }

    fn page(&self) -> &Page {
        &self.1
    }
}

thread_local! {
    static NAVIGATOR: OnceCell<Box<dyn Fn(&str, NavigateOptions)>> = OnceCell::new();
}

pub fn navigate(path: &str, options: NavigateOptions) {
    NAVIGATOR.with(|cell| cell.get().expect("No Navigator on this context")(path, options));
}

pub struct AppRoutes;
#[routes_builder(as = RoutesBuilder, use = NAVIGATOR)]
impl AppRoutes {
    pub const HOME: Route = ("/", HomeIndex);
    pub const PRODUCTS: Route = ("/products", ProductsIndex);
    pub const PRODUCT_EDIT: Route = ("/products/:id", ProductEdit);
    // pub const ORDERS: Route = ("/orders", Orders);
    // pub const DELIVERY: Route = ("/delivery", Delivery);
    // pub const CHANNELS: Route = ("/channels", Channels);
    pub const USERS: Route = ("/users", UsersIndex);
    pub const SETTINGS: Route = ("/settings", SettingsIndex);
}
