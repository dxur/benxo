mod home;
mod not_found;
mod products;
mod settings;
mod users;
mod orders;

pub use home::*;
pub use orders::*;
pub use not_found::*;
pub use products::*;
pub use settings::*;
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
