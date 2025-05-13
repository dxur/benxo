use leptos::prelude::*;

use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const StoreIndex: Page = Page {
    title: "Online Store",
    view: View,
};

#[component]
fn View() -> AnyView {
    view! {
        <Header title=StoreIndex.title>
            <div/>
        </Header>
    }
    .into_any()
}
