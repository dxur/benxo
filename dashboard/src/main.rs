mod components;
mod forms;
mod pages;
mod routes;
mod utils;
mod layout;
mod services;

use leptos::prelude::*;
use layout::app::App;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(App)
}
