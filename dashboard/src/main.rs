mod components;
mod forms;
mod pages;
mod routes;

use components::sidebar::Sidebar;
use leptos::prelude::*;
use leptos::tachys::view::iterators::StaticVec;
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, RouteProps, Router},
};
use routes::*;
use serde::{Deserialize, Serialize};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Sidebar/>
            <main>
                <header>
                    <input type="text" placeholder="Search"/>
                    <button> Account </button>
                </header>
                <section data-page>
                    <FlatRoutes fallback=|| "" children=ToChildren::to_children(|| {
                        StaticVec::from(
                            ROUTES
                                .iter()
                                .map(|route| {
                                    Route(
                                        RouteProps::builder()
                                            .path(StaticSegment(route.path))
                                            .view(route.component)
                                            .build(),
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    })/>
                </section>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(App)
}
