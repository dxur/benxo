use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_navigate;

use super::sidebar::Sidebar;
use crate::pages::NotFound;
use crate::routes::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Sidebar/>
            <main>
                <header>
                    <input type="text" placeholder="Search"/>
                    // <button> Account </button>
                </header>
                <section data-page>
                    <Routes fallback=*NotFound>
                        <RoutesBuilder />
                    </Routes>
                </section>
            </main>
        </Router>
    }
}
