use std::{cell::OnceCell, sync::{Arc, OnceLock}};

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
    #[allow(non_upper_case_globals)]
    static Navigator: OnceCell<Box<dyn Fn(&str, NavigateOptions)>> = OnceCell::new();
}

pub fn navigate(path: &str, options: NavigateOptions) {
    Navigator.with(|cell| cell.get().expect("No Navigator on this context")(path, options));
}

pub struct AppRoutes;
#[routes_builder(as = RoutesBuilder, use = Navigator)]
impl AppRoutes {
    pub const HOME: Route = ("/", HomeIndex);
    pub const PRODUCTS: Route = ("/products", ProductsIndex);
    pub const PRODUCT_EDIT: Route = ("/products/:id", ProductEdit);
    // pub const ORDERS: Route = ("/orders", Orders);
    // pub const DELIVERY: Route = ("/delivery", Delivery);
    // pub const CHANNELS: Route = ("/channels", Channels);
    // pub const USERS: Route = ("/users", Users);
    pub const SETTINGS: Route = ("/settings", SettingsIndex);
}
