pub mod home;
pub mod not_found;
pub mod products;
pub mod settings;
pub mod users;
pub mod delivery;
// pub mod order_placement;
pub mod sale_channels;
// pub mod order_processing;

use std::ops::Deref;

pub use home::Home;
// pub use order_placement::Orders;
pub use not_found::NotFound;
pub use products::Products;
pub use settings::Settings;
pub use users::Users;

use leptos::prelude::*;

pub struct Page {
    pub title: &'static str,
    pub view: fn() -> AnyView,
}

impl Deref for Page {
    type Target = fn() -> AnyView;
    
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}
