use leptos::prelude::*;
use leptos_router::components::*;

use super::Sidebar;
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
                <div data-page>
                    <section data-page>
                        <Routes fallback=*NotFound>
                            <RoutesBuilder />
                        </Routes>
                    </section>
                </div>
            </main>
        </Router>
    }
}
