use leptos::prelude::*;

use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const HomeIndex: Page = Page {
    title: "Home",
    view: HomeView,
};

#[component]
pub fn HomeView() -> AnyView {
    view! {
        <div class="flex flex-col">
            <span> Home </span>
            <hr/>
            <span> Here you should see some analytics and varios stuff about your store </span>
        </div>
    }
    .into_any()
}
