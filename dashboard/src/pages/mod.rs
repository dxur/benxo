mod home;
mod not_found;
mod orders;
mod products;
mod settings;
mod store;
mod users;

pub use home::*;
pub use not_found::*;
pub use orders::*;
pub use products::*;
pub use settings::*;
pub use store::*;
pub use users::*;

use leptos::prelude::*;
use std::ops::Deref;

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
