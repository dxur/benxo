use leptos::prelude::*;

use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const HomeIndex: Page = Page {
    title: "Home",
    view: HomeView,
};

#[component]
pub fn HomeView() -> AnyView {
    view! {
        <CodeEditor />
    }
    .into_any()
}
