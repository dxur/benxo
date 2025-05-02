use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::{routes::{AppRoutes, RouteExt}, utils::is_subpath};

#[component]
fn Item(path: &'static str, name: &'static str) -> impl IntoView {
    view! {
        <A href=path>
            <button
                disabled=move || is_subpath(path, use_location().pathname.get().as_str())
            >
                {name}
            </button>
        </A>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside>
            <h1> Dashboard </h1>
            <nav>
                <Item path=AppRoutes::HOME.path() name=AppRoutes::HOME.page().title />
                <Item path=AppRoutes::PRODUCTS.path() name=AppRoutes::PRODUCTS.page().title />
                // <Item path=AppRoutes::ORDERS.path() name=AppRoutes::ORDERS.page().title />
                // <Item path=AppRoutes::DELIVERY.path() name=AppRoutes::DELIVERY.page().title />
                // <Item path=AppRoutes::CHANNELS.path() name=AppRoutes::CHANNELS.page().title />
                // <Item path=AppRoutes::USERS.path() name=AppRoutes::USERS.page().title />
                <Item path=AppRoutes::SETTINGS.path() name=AppRoutes::SETTINGS.page().title />
            </nav>
        </aside>
    }
}
